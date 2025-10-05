/// Scan strategy pattern implementation

use std::net::{SocketAddr, IpAddr};
use std::io;
use tracing::{debug, trace};

use crate::domain::{Port, PortStatus, PortScanResult};
use crate::scanning::config::ScanConfig;
use crate::infrastructure::{NetworkConnector, TcpConnector, network_utils};

/// Trait for different scanning strategies
pub trait ScanStrategy: Send + Sync {
    fn scan(&self, port: Port, target_ip: IpAddr, config: &ScanConfig) -> PortScanResult;
    fn name(&self) -> &'static str;
}

/// Standard TCP connect scan
pub struct StandardScan {
    connector: Box<dyn NetworkConnector>,
}

impl StandardScan {
    pub fn new() -> Self {
        Self {
            connector: Box::new(TcpConnector::new()),
        }
    }

    pub fn with_connector(connector: Box<dyn NetworkConnector>) -> Self {
        Self { connector }
    }
}

impl Default for StandardScan {
    fn default() -> Self {
        Self::new()
    }
}

impl ScanStrategy for StandardScan {
    fn scan(&self, port: Port, target_ip: IpAddr, config: &ScanConfig) -> PortScanResult {
        let socket = SocketAddr::new(target_ip, port);
        
        trace!("Standard scanning port {} on {}", port, target_ip);
        
        match self.connector.connect(&socket, config.timeout) {
            Ok(_) => {
                debug!("Port {} is OPEN", port);
                PortScanResult::new(port, PortStatus::Open)
            }
            Err(ref e) if network_utils::is_connection_refused(e) => {
                trace!("Port {} is CLOSED", port);
                PortScanResult::new(port, PortStatus::Closed)
            }
            Err(ref e) if network_utils::is_timeout(e) => {
                trace!("Port {} is FILTERED (timeout)", port);
                PortScanResult::new(port, PortStatus::Filtered)
            }
            Err(e) => {
                trace!("Port {} returned ERROR: {}", port, e);
                PortScanResult::new(port, PortStatus::Error(e.to_string()))
            }
        }
    }

    fn name(&self) -> &'static str {
        "Standard TCP Connect"
    }
}

/// Stealth scan with source port randomization
pub struct StealthScan {
    connector: Box<dyn NetworkConnector>,
}

impl StealthScan {
    pub fn new() -> Self {
        Self {
            connector: Box::new(TcpConnector::new()),
        }
    }

    pub fn with_connector(connector: Box<dyn NetworkConnector>) -> Self {
        Self { connector }
    }
}

impl Default for StealthScan {
    fn default() -> Self {
        Self::new()
    }
}

impl ScanStrategy for StealthScan {
    fn scan(&self, port: Port, target_ip: IpAddr, config: &ScanConfig) -> PortScanResult {
        let socket = SocketAddr::new(target_ip, port);
        
        // Add delay if configured
        if let Some(delay) = config.delay_between_probes {
            let jittered_delay = network_utils::random_delay_jitter(
                delay, 
                crate::constants::DELAY_JITTER_PERCENT
            );
            trace!("Delaying {:?} before scanning port {}", jittered_delay, port);
            std::thread::sleep(jittered_delay);
        }
        
        trace!("Stealth scanning port {} on {}", port, target_ip);
        
        // For now, fall back to standard scan
        // Full implementation would use socket2 crate for source port binding
        match self.connector.connect(&socket, config.timeout) {
            Ok(_) => {
                debug!("Port {} is OPEN (stealth)", port);
                PortScanResult::new(port, PortStatus::Open)
            }
            Err(ref e) if network_utils::is_connection_refused(e) => {
                trace!("Port {} is CLOSED", port);
                PortScanResult::new(port, PortStatus::Closed)
            }
            Err(ref e) if network_utils::is_timeout(e) => {
                trace!("Port {} is FILTERED (timeout)", port);
                PortScanResult::new(port, PortStatus::Filtered)
            }
            Err(e) => {
                trace!("Port {} returned ERROR: {}", port, e);
                PortScanResult::new(port, PortStatus::Error(e.to_string()))
            }
        }
    }

    fn name(&self) -> &'static str {
        "Stealth TCP Connect"
    }
}

/// Factory for creating scan strategies
pub struct ScanStrategyFactory;

impl ScanStrategyFactory {
    pub fn create(config: &ScanConfig) -> Box<dyn ScanStrategy> {
        if config.is_stealth_enabled() {
            Box::new(StealthScan::new())
        } else {
            Box::new(StandardScan::new())
        }
    }
}
