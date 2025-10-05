/// Output formatter factory pattern

use std::path::Path;
use std::fs::File;
use std::io::Write;
use serde::Serialize;

use crate::domain::{PortScanResult, ScanResults};
use crate::scanning::ScanConfig;
use crate::errors::FormatterResult;

/// Output format enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    Text,
    Json,
    Csv,
    Xml,
}

/// Scan report for serialization
#[derive(Debug, Serialize)]
pub struct ScanReport {
    pub scan_info: ScanInfo,
    pub results: Vec<PortScanResult>,
    pub statistics: ScanStatistics,
}

#[derive(Debug, Serialize, Clone)]
pub struct ScanInfo {
    pub target_ip: String,
    pub scan_mode: String,
    pub timeout_ms: u64,
    pub parallel_enabled: bool,
    pub thread_count: Option<usize>,
    pub version_detection: bool,
    pub os_detection: bool,
    pub stealth_enabled: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct ScanStatistics {
    pub total_ports: usize,
    pub open_ports: usize,
    pub closed_ports: usize,
    pub filtered_ports: usize,
    pub error_ports: usize,
    pub open_percentage: f32,
    pub scan_duration_seconds: f64,
    pub ports_per_second: f64,
}

impl ScanReport {
    pub fn new(config: &ScanConfig, results: ScanResults, duration_seconds: f64) -> Self {
        let total = results.total_ports;
        let open = results.open_ports;
        let closed = results.closed_ports;
        let filtered = results.filtered_ports;
        let error = results.error_ports;
        
        let open_percentage = results.open_percentage();
        
        let ports_per_second = if duration_seconds > 0.0 {
            total as f64 / duration_seconds
        } else {
            0.0
        };

        let scan_mode = match &config.scan_mode {
            crate::scanning::ScanMode::Range { start, end } => {
                format!("Range: {}-{}", start, end)
            }
            crate::scanning::ScanMode::CommonPorts => "CommonPorts".to_string(),
            crate::scanning::ScanMode::CustomList(ports) => {
                format!("Custom: {} ports", ports.len())
            }
        };

        Self {
            scan_info: ScanInfo {
                target_ip: config.target_ip.to_string(),
                scan_mode,
                timeout_ms: config.timeout.as_millis() as u64,
                parallel_enabled: config.parallel,
                thread_count: if config.parallel {
                    Some(config.thread_count)
                } else {
                    None
                },
                version_detection: config.detect_versions,
                os_detection: config.detect_os,
                stealth_enabled: config.is_stealth_enabled(),
            },
            results: results.results,
            statistics: ScanStatistics {
                total_ports: total,
                open_ports: open,
                closed_ports: closed,
                filtered_ports: filtered,
                error_ports: error,
                open_percentage,
                scan_duration_seconds: duration_seconds,
                ports_per_second,
            },
        }
    }

    pub fn default_filename(target_ip: &str, format: OutputFormat) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let safe_ip = target_ip.replace(".", "_").replace(":", "_");
        let extension = match format {
            OutputFormat::Json => "json",
            OutputFormat::Xml => "xml",
            OutputFormat::Csv => "csv",
            OutputFormat::Text => "txt",
        };
        
        format!("scan_{}_{}.{}", safe_ip, timestamp, extension)
    }
}

/// Trait for output formatters
pub trait OutputFormatter: Send + Sync {
    fn format(&self, report: &ScanReport) -> FormatterResult<String>;
    fn write_to_file(&self, report: &ScanReport, path: &Path) -> FormatterResult<()>;
    fn extension(&self) -> &'static str;
}

/// JSON formatter
pub struct JsonFormatter {
    pub open_only: bool,
}

impl JsonFormatter {
    pub fn new(open_only: bool) -> Self {
        Self { open_only }
    }
}

impl OutputFormatter for JsonFormatter {
    fn format(&self, report: &ScanReport) -> FormatterResult<String> {
        if self.open_only {
            // Create filtered report with only open ports
            let filtered_results: Vec<_> = report.results.iter()
                .filter(|r| matches!(r.status, crate::domain::PortStatus::Open))
                .cloned()
                .collect();
            
            let filtered_report = ScanReport {
                scan_info: report.scan_info.clone(),
                results: filtered_results,
                statistics: report.statistics.clone(),
            };
            
            Ok(serde_json::to_string_pretty(&filtered_report)?)
        } else {
            Ok(serde_json::to_string_pretty(report)?)
        }
    }

    fn write_to_file(&self, report: &ScanReport, path: &Path) -> FormatterResult<()> {
        let json = self.format(report)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    fn extension(&self) -> &'static str {
        "json"
    }
}

/// Text formatter
pub struct TextFormatter {
    pub open_only: bool,
}

impl TextFormatter {
    pub fn new(open_only: bool) -> Self {
        Self { open_only }
    }
}

impl OutputFormatter for TextFormatter {
    fn format(&self, report: &ScanReport) -> FormatterResult<String> {
        let mut output = String::new();

        output.push_str("╔═══════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║              PORT SCAN DETAILED REPORT                          ║\n");
        output.push_str("╚═══════════════════════════════════════════════════════════════════╝\n\n");
        
        output.push_str("=== SCAN CONFIGURATION ===\n");
        output.push_str(&format!("Target IP:          {}\n", report.scan_info.target_ip));
        output.push_str(&format!("Scan Mode:          {}\n", report.scan_info.scan_mode));
        output.push_str(&format!("Timeout:            {} ms\n", report.scan_info.timeout_ms));
        output.push_str(&format!("Parallel Scan:      {}\n", if report.scan_info.parallel_enabled { "Yes" } else { "No" }));
        if let Some(threads) = report.scan_info.thread_count {
            output.push_str(&format!("Thread Count:       {}\n", threads));
        }
        output.push_str(&format!("Version Detection:  {}\n", if report.scan_info.version_detection { "Enabled" } else { "Disabled" }));
        output.push_str(&format!("OS Detection:       {}\n", if report.scan_info.os_detection { "Enabled" } else { "Disabled" }));
        output.push_str(&format!("Stealth Mode:       {}\n", if report.scan_info.stealth_enabled { "Enabled" } else { "Disabled" }));
        
        output.push_str("\n=== SCAN STATISTICS ===\n");
        output.push_str(&format!("Total Ports Scanned: {}\n", report.statistics.total_ports));
        output.push_str(&format!("Open Ports:          {}\n", report.statistics.open_ports));
        output.push_str(&format!("Closed Ports:        {}\n", report.statistics.closed_ports));
        output.push_str(&format!("Filtered Ports:      {}\n", report.statistics.filtered_ports));
        output.push_str(&format!("Error Ports:         {}\n", report.statistics.error_ports));
        output.push_str(&format!("Open Percentage:     {:.1}%\n", report.statistics.open_percentage));
        output.push_str(&format!("Scan Duration:       {:.2} seconds\n", report.statistics.scan_duration_seconds));
        output.push_str(&format!("Scan Speed:          {:.2} ports/sec\n", report.statistics.ports_per_second));

        output.push_str("\n=== DETAILED PORT RESULTS ===\n");
        
        // Group by status
        let mut open_ports = Vec::new();
        let mut closed_ports = Vec::new();
        let mut filtered_ports = Vec::new();
        
        for result in &report.results {
            match &result.status {
                crate::domain::PortStatus::Open => open_ports.push(result),
                crate::domain::PortStatus::Closed => closed_ports.push(result),
                crate::domain::PortStatus::Filtered => filtered_ports.push(result),
                _ => {}
            }
        }
        
        if !open_ports.is_empty() {
            output.push_str("\n--- OPEN PORTS (VERBOSE) ---\n");
            for result in open_ports {
                output.push_str(&format!("\nPort {}:\n", result.port));
                output.push_str("  Status: OPEN\n");
                
                if let Some(version) = &result.service_version {
                    output.push_str(&format!("  Service Name:    {}\n", version.service_name));
                    if let Some(ver) = &version.version {
                        output.push_str(&format!("  Version:         {}\n", ver));
                    }
                    if let Some(banner) = &version.banner {
                        output.push_str(&format!("  Banner:          {}\n", banner));
                    }
                    output.push_str(&format!("  Protocol:        {}\n", version.protocol));
                } else {
                    output.push_str("  Service:         Unknown (no banner detected)\n");
                }
                
                if let Some(os_info) = &result.os_info {
                    output.push_str("  --- OS Detection ---\n");
                    if let Some(os_name) = &os_info.os_name {
                        output.push_str(&format!("  OS Name:         {}\n", os_name));
                    }
                    if let Some(os_version) = &os_info.os_version {
                        output.push_str(&format!("  OS Version:      {}\n", os_version));
                    }
                    if let Some(os_build) = &os_info.os_build {
                        output.push_str(&format!("  OS Build:        {}\n", os_build));
                    }
                    if let Some(smb_version) = &os_info.smb_version {
                        output.push_str(&format!("  SMB Version:     {}\n", smb_version));
                    }
                    if let Some(computer_name) = &os_info.computer_name {
                        output.push_str(&format!("  Computer Name:   {}\n", computer_name));
                    }
                    if let Some(domain) = &os_info.domain {
                        output.push_str(&format!("  Domain:          {}\n", domain));
                    }
                    output.push_str(&format!("  OS Summary:      {}\n", os_info.summary()));
                }
            }
        }
        
        if !filtered_ports.is_empty() {
            output.push_str(&format!("\n--- FILTERED PORTS ({}) ---\n", filtered_ports.len()));
            if self.open_only {
                output.push_str("(Summary only - use without --open-only for details)\n");
            }
            output.push_str("Ports: ");
            for (i, result) in filtered_ports.iter().enumerate() {
                if i > 0 { output.push_str(", "); }
                output.push_str(&result.port.to_string());
            }
            output.push_str("\n");
        }
        
        if !closed_ports.is_empty() {
            output.push_str(&format!("\n--- CLOSED PORTS ({}) ---\n", closed_ports.len()));
            if self.open_only {
                output.push_str("(Summary only - use without --open-only for details)\n");
            } else {
                output.push_str("(Details omitted for brevity)\n");
            }
        }

        output.push_str("\n╚═══════════════════════════════════════════════════════════════════╝\n");
        output.push_str("                    End of Report\n");
        output.push_str("╚═══════════════════════════════════════════════════════════════════╝\n");

        Ok(output)
    }    fn write_to_file(&self, report: &ScanReport, path: &Path) -> FormatterResult<()> {
        let text = self.format(report)?;
        let mut file = File::create(path)?;
        file.write_all(text.as_bytes())?;
        Ok(())
    }

    fn extension(&self) -> &'static str {
        "txt"
    }
}

/// CSV formatter
pub struct CsvFormatter {
    pub open_only: bool,
}

impl CsvFormatter {
    pub fn new(open_only: bool) -> Self {
        Self { open_only }
    }
}

impl OutputFormatter for CsvFormatter {
    fn format(&self, report: &ScanReport) -> FormatterResult<String> {
        let mut output = String::new();
        
        // Header with all columns
        output.push_str("Port,Status,Service,Version,Protocol,Banner,OS_Name,OS_Version,OS_Build,SMB_Version,Computer_Name,Domain\n");

        for result in &report.results {
            // Skip non-open ports if open_only is enabled
            if self.open_only && !matches!(result.status, crate::domain::PortStatus::Open) {
                continue;
            }
            let status = match result.status {
                crate::domain::PortStatus::Open => "OPEN",
                crate::domain::PortStatus::Closed => "CLOSED",
                crate::domain::PortStatus::Filtered => "FILTERED",
                crate::domain::PortStatus::Error(_) => "ERROR",
            };

            let service = result.service_version.as_ref()
                .map(|v| v.service_name.as_str())
                .unwrap_or("");

            let version = result.service_version.as_ref()
                .and_then(|v| v.version.as_deref())
                .unwrap_or("");
            
            let protocol = result.service_version.as_ref()
                .map(|v| v.protocol.as_str())
                .unwrap_or("");
            
            let banner = result.service_version.as_ref()
                .and_then(|v| v.banner.as_deref())
                .map(|b| b.replace(",", ";").replace("\n", " ").replace("\r", ""))
                .unwrap_or_default();
            
            let os_name = result.os_info.as_ref()
                .and_then(|os| os.os_name.as_deref())
                .unwrap_or("");
            
            let os_version = result.os_info.as_ref()
                .and_then(|os| os.os_version.as_deref())
                .unwrap_or("");
            
            let os_build = result.os_info.as_ref()
                .and_then(|os| os.os_build.as_deref())
                .unwrap_or("");
            
            let smb_version = result.os_info.as_ref()
                .and_then(|os| os.smb_version.as_deref())
                .unwrap_or("");
            
            let computer_name = result.os_info.as_ref()
                .and_then(|os| os.computer_name.as_deref())
                .unwrap_or("");
            
            let domain = result.os_info.as_ref()
                .and_then(|os| os.domain.as_deref())
                .unwrap_or("");

            output.push_str(&format!(
                "{},{},{},{},{},\"{}\",{},{},{},{},{},{}\n",
                result.port, status, service, version, protocol, banner, 
                os_name, os_version, os_build, smb_version, computer_name, domain
            ));
        }

        Ok(output)
    }    fn write_to_file(&self, report: &ScanReport, path: &Path) -> FormatterResult<()> {
        let csv = self.format(report)?;
        let mut file = File::create(path)?;
        file.write_all(csv.as_bytes())?;
        Ok(())
    }

    fn extension(&self) -> &'static str {
        "csv"
    }
}

/// Factory for creating output formatters
pub struct OutputFormatterFactory;

impl OutputFormatterFactory {
    pub fn create(format: OutputFormat, open_only: bool) -> Box<dyn OutputFormatter> {
        match format {
            OutputFormat::Json => Box::new(JsonFormatter::new(open_only)),
            OutputFormat::Text => Box::new(TextFormatter::new(open_only)),
            OutputFormat::Csv => Box::new(CsvFormatter::new(open_only)),
            OutputFormat::Xml => Box::new(TextFormatter::new(open_only)), // XML not implemented yet
        }
    }
}
