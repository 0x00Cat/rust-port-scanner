/// Domain model for scan results

use serde::Serialize;
use super::port::{Port, PortStatus};
use super::service::ServiceVersion;
use super::os::OSInfo;

/// Result of scanning a single port
#[derive(Debug, Clone, Serialize)]
pub struct PortScanResult {
    pub port: Port,
    pub status: PortStatus,
    pub service_version: Option<ServiceVersion>,
    pub os_info: Option<OSInfo>,
}

impl PortScanResult {
    pub fn new(port: Port, status: PortStatus) -> Self {
        Self { 
            port, 
            status,
            service_version: None,
            os_info: None,
        }
    }

    pub fn with_version(mut self, version: ServiceVersion) -> Self {
        self.service_version = Some(version);
        self
    }

    pub fn with_os_info(mut self, os_info: OSInfo) -> Self {
        self.os_info = Some(os_info);
        self
    }

    pub fn is_open(&self) -> bool {
        self.status.is_open()
    }

    pub fn has_service_info(&self) -> bool {
        self.service_version.is_some()
    }

    pub fn has_os_info(&self) -> bool {
        self.os_info.is_some()
    }
}

/// Collection of scan results with statistics
#[derive(Debug, Clone, Serialize)]
pub struct ScanResults {
    pub results: Vec<PortScanResult>,
    pub total_ports: usize,
    pub open_ports: usize,
    pub closed_ports: usize,
    pub filtered_ports: usize,
    pub error_ports: usize,
}

impl ScanResults {
    pub fn new(results: Vec<PortScanResult>) -> Self {
        let total = results.len();
        let open = results.iter().filter(|r| r.status.is_open()).count();
        let closed = results.iter().filter(|r| r.status.is_closed()).count();
        let filtered = results.iter().filter(|r| r.status.is_filtered()).count();
        let error = results.iter().filter(|r| r.status.is_error()).count();

        Self {
            results,
            total_ports: total,
            open_ports: open,
            closed_ports: closed,
            filtered_ports: filtered,
            error_ports: error,
        }
    }

    pub fn open_percentage(&self) -> f32 {
        if self.total_ports > 0 {
            (self.open_ports as f32 / self.total_ports as f32) * 100.0
        } else {
            0.0
        }
    }

    pub fn get_open_results(&self) -> Vec<&PortScanResult> {
        self.results.iter().filter(|r| r.is_open()).collect()
    }
}

impl From<Vec<PortScanResult>> for ScanResults {
    fn from(results: Vec<PortScanResult>) -> Self {
        Self::new(results)
    }
}
