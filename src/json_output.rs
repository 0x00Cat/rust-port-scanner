use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::port_info::PortScanResult;
use crate::scanner::ScanConfig;

/// Represents the complete scan results in JSON format
#[derive(Debug, Serialize)]
pub struct ScanReport {
    pub scan_info: ScanInfo,
    pub results: Vec<PortScanResult>,
    pub statistics: ScanStatistics,
}

/// Metadata about the scan
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

/// Statistical summary of the scan
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
    /// Creates a new scan report from results and configuration
    pub fn new(
        config: &ScanConfig,
        results: Vec<PortScanResult>,
        duration_seconds: f64,
    ) -> Self {
        let total = results.len();
        let open = results.iter().filter(|r| r.status.is_open()).count();
        let closed = results.iter().filter(|r| r.status.is_closed()).count();
        let filtered = results.iter().filter(|r| r.status.is_filtered()).count();
        let error = results.iter().filter(|r| r.status.is_error()).count();
        
        let open_percentage = if total > 0 {
            (open as f32 / total as f32) * 100.0
        } else {
            0.0
        };
        
        let ports_per_second = if duration_seconds > 0.0 {
            total as f64 / duration_seconds
        } else {
            0.0
        };

        let scan_mode = match &config.scan_mode {
            crate::scanner::ScanMode::Range { start, end } => {
                format!("Range: {}-{}", start, end)
            }
            crate::scanner::ScanMode::CommonPorts => "CommonPorts".to_string(),
            crate::scanner::ScanMode::CustomList(ports) => {
                format!("Custom: {} ports", ports.len())
            }
        };

        let stealth_enabled = config.randomize_source_port || config.delay_between_probes.is_some();

        ScanReport {
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
                stealth_enabled,
            },
            results,
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

    /// Serializes the report to a JSON string
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Writes the report to a JSON file
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let json = self.to_json_string()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Generates a default filename with timestamp
    pub fn default_filename(target_ip: &str) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let safe_ip = target_ip.replace(".", "_");
        format!("scan_{}_{}.json", safe_ip, timestamp)
    }
}
