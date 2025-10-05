/// Scan strategy pattern implementation with async support

use std::net::{SocketAddr, IpAddr};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use std::sync::Arc;
use tracing::{debug, trace};

use crate::domain::{Port, PortStatus, PortScanResult};
use crate::scanning::config::ScanConfig;
use crate::application::{VersionDetector, SMBFingerprinter};

/// Trait for different scanning strategies (now async)
#[async_trait::async_trait]
pub trait ScanStrategy: Send + Sync {
    async fn scan_async(&self, port: Port, target_ip: IpAddr, config: &ScanConfig) -> PortScanResult;
    fn name(&self) -> &'static str;
}

/// Standard TCP connect scan (async)
pub struct StandardScan;

impl StandardScan {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StandardScan {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl ScanStrategy for StandardScan {
    async fn scan_async(&self, port: Port, target_ip: IpAddr, config: &ScanConfig) -> PortScanResult {
        let socket = SocketAddr::new(target_ip, port);
        
        trace!("Async scanning port {} on {}", port, target_ip);

        // Async TCP connection with timeout
        match timeout(config.timeout, TcpStream::connect(&socket)).await {
            Ok(Ok(stream)) => {
                debug!("Port {} is OPEN", port);
                let mut result = PortScanResult::new(port, PortStatus::Open);
                
                // Perform service version detection if enabled
                if config.detect_versions {
                    debug!("Service detection enabled - attempting on port {}", port);
                    let version = VersionDetector::detect_version_async(&socket, config.timeout).await;
                    if version.service_name != "Unknown" {
                        let version_str = version.version.as_deref().unwrap_or("unknown version");
                        debug!("Detected service on port {}: {} {}", port, version.service_name, version_str);
                        result = result.with_version(version);
                    } else {
                        trace!("No service detected on port {}", port);
                    }
                }
                
                // Perform OS detection if enabled and port is 445 (SMB)
                if config.detect_os && port == 445 {
                    debug!("OS detection enabled - attempting SMB fingerprinting on port {}", port);
                    let os_info = SMBFingerprinter::fingerprint_async(&socket, config.timeout).await;
                    if os_info.os_name.as_ref().map_or(false, |n| n != "Unknown") {
                        debug!("OS detected via SMB: {}", os_info.summary());
                        result = result.with_os_info(os_info);
                    } else {
                        debug!("OS detection on port {} did not yield results", port);
                    }
                }
                
                result
            }
            Ok(Err(_)) => {
                trace!("Port {} is CLOSED", port);
                PortScanResult::new(port, PortStatus::Closed)
            }
            Err(_) => {
                trace!("Port {} is FILTERED (timeout)", port);
                PortScanResult::new(port, PortStatus::Filtered)
            }
        }
    }

    fn name(&self) -> &'static str {
        "Standard TCP Connect (Async)"
    }
}

/// Stealth scan with source port randomization (async)
pub struct StealthScan;

impl StealthScan {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StealthScan {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl ScanStrategy for StealthScan {
    async fn scan_async(&self, port: Port, target_ip: IpAddr, config: &ScanConfig) -> PortScanResult {
        // Add delay if configured for stealth
        if let Some(delay) = config.delay_between_probes {
            tokio::time::sleep(delay).await;
        }

        // Use standard scan logic (async version)
        let standard = StandardScan::new();
        standard.scan_async(port, target_ip, config).await
    }

    fn name(&self) -> &'static str {
        "Stealth Scan (Async)"
    }
}

/// Factory for creating scan strategies
pub struct ScanStrategyFactory;

impl ScanStrategyFactory {
    pub fn create(config: &ScanConfig) -> Arc<dyn ScanStrategy> {
        if config.randomize_source_port || config.delay_between_probes.is_some() {
            Arc::new(StealthScan::new())
        } else {
            Arc::new(StandardScan::new())
        }
    }
}
