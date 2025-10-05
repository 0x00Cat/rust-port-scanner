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

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
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
pub struct JsonFormatter;

impl OutputFormatter for JsonFormatter {
    fn format(&self, report: &ScanReport) -> FormatterResult<String> {
        Ok(serde_json::to_string_pretty(report)?)
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
pub struct TextFormatter;

impl OutputFormatter for TextFormatter {
    fn format(&self, report: &ScanReport) -> FormatterResult<String> {
        let mut output = String::new();
        
        output.push_str(&format!("=== PORT SCAN REPORT ===\n"));
        output.push_str(&format!("Target: {}\n", report.scan_info.target_ip));
        output.push_str(&format!("Mode: {}\n", report.scan_info.scan_mode));
        output.push_str(&format!("\n=== STATISTICS ===\n"));
        output.push_str(&format!("Total Ports: {}\n", report.statistics.total_ports));
        output.push_str(&format!("Open: {}\n", report.statistics.open_ports));
        output.push_str(&format!("Closed: {}\n", report.statistics.closed_ports));
        output.push_str(&format!("Filtered: {}\n", report.statistics.filtered_ports));
        output.push_str(&format!("Duration: {:.2}s\n", report.statistics.scan_duration_seconds));
        output.push_str(&format!("Speed: {:.2} ports/sec\n", report.statistics.ports_per_second));
        
        output.push_str(&format!("\n=== OPEN PORTS ===\n"));
        for result in &report.results {
            if result.is_open() {
                output.push_str(&format!("Port {}: OPEN\n", result.port));
                if let Some(version) = &result.service_version {
                    output.push_str(&format!("  Service: {}\n", version.service_name));
                }
            }
        }
        
        Ok(output)
    }

    fn write_to_file(&self, report: &ScanReport, path: &Path) -> FormatterResult<()> {
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
pub struct CsvFormatter;

impl OutputFormatter for CsvFormatter {
    fn format(&self, report: &ScanReport) -> FormatterResult<String> {
        let mut output = String::new();
        output.push_str("Port,Status,Service,Version\n");
        
        for result in &report.results {
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
            
            output.push_str(&format!("{},{},{},{}\n", result.port, status, service, version));
        }
        
        Ok(output)
    }

    fn write_to_file(&self, report: &ScanReport, path: &Path) -> FormatterResult<()> {
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
    pub fn create(format: OutputFormat) -> Box<dyn OutputFormatter> {
        match format {
            OutputFormat::Json => Box::new(JsonFormatter),
            OutputFormat::Text => Box::new(TextFormatter),
            OutputFormat::Csv => Box::new(CsvFormatter),
            OutputFormat::Xml => Box::new(TextFormatter), // XML not implemented yet
        }
    }
}
