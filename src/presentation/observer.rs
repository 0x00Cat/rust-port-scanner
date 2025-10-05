/// Observer pattern for scan events

use crate::domain::{PortScanResult, ScanResults};

/// Trait for scan observers
pub trait ScanObserver: Send {
    fn on_port_scanned(&mut self, result: &PortScanResult);
    fn on_scan_started(&mut self, total_ports: usize);
    fn on_scan_completed(&mut self, results: &ScanResults);
}

/// Progress bar observer
pub struct ProgressObserver {
    verbose: bool,
    count: usize,
}

impl ProgressObserver {
    pub fn new(verbose: bool) -> Self {
        Self { verbose, count: 0 }
    }
}

impl ScanObserver for ProgressObserver {
    fn on_port_scanned(&mut self, result: &PortScanResult) {
        self.count += 1;
        
        if result.is_open() || self.verbose {
            let status_str = match &result.status {
                crate::domain::PortStatus::Open => "OPEN",
                crate::domain::PortStatus::Closed => "CLOSED",
                crate::domain::PortStatus::Filtered => "FILTERED",
                crate::domain::PortStatus::Error(_) => "ERROR",
            };
            
            println!("Port {}: {}", result.port, status_str);
            
            if let Some(version) = &result.service_version {
                if version.banner.is_some() {
                    println!("  └─ Service: {} {}", 
                        version.service_name,
                        version.version.as_deref().unwrap_or(""));
                }
            }
        }
    }

    fn on_scan_started(&mut self, total_ports: usize) {
        println!("Starting scan of {} ports...", total_ports);
        self.count = 0;
    }

    fn on_scan_completed(&mut self, results: &ScanResults) {
        println!("\nScan completed: {} ports scanned", self.count);
        println!("Open: {}, Closed: {}, Filtered: {}", 
            results.open_ports, 
            results.closed_ports, 
            results.filtered_ports);
    }
}

/// Metrics collector observer
pub struct MetricsCollector {
    pub start_time: std::time::Instant,
    pub ports_scanned: usize,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
            ports_scanned: 0,
        }
    }

    pub fn elapsed(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }

    pub fn ports_per_second(&self) -> f64 {
        let elapsed = self.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            self.ports_scanned as f64 / elapsed
        } else {
            0.0
        }
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl ScanObserver for MetricsCollector {
    fn on_port_scanned(&mut self, _result: &PortScanResult) {
        self.ports_scanned += 1;
    }

    fn on_scan_started(&mut self, _total_ports: usize) {
        self.start_time = std::time::Instant::now();
        self.ports_scanned = 0;
    }

    fn on_scan_completed(&mut self, _results: &ScanResults) {
        // Metrics are collected, caller can query them
    }
}
