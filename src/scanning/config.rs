/// Scan configuration and modes

use std::net::IpAddr;
use std::time::Duration;
use serde::Serialize;

use crate::constants::*;
use crate::errors::{ConfigError, ConfigResult};
use crate::domain::Port;

/// Scan mode for port scanning
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum ScanMode {
    /// Scan a range of ports
    Range { start: Port, end: Port },
    /// Scan only common ports
    CommonPorts,
    /// Scan a custom list of ports
    CustomList(Vec<Port>),
}

impl ScanMode {
    pub fn validate(&self) -> ConfigResult<()> {
        match self {
            ScanMode::Range { start, end } => {
                if start > end {
                    return Err(ConfigError::InvalidScanMode);
                }
                if *start < MIN_PORT || *end > MAX_PORT {
                    return Err(ConfigError::InvalidScanMode);
                }
                Ok(())
            }
            ScanMode::CommonPorts => Ok(()),
            ScanMode::CustomList(ports) => {
                if ports.is_empty() {
                    return Err(ConfigError::InvalidScanMode);
                }
                for &port in ports {
                    if port < MIN_PORT || port > MAX_PORT {
                        return Err(ConfigError::InvalidScanMode);
                    }
                }
                Ok(())
            }
        }
    }

    pub fn port_count(&self) -> usize {
        match self {
            ScanMode::Range { start, end } => (end - start + 1) as usize,
            ScanMode::CommonPorts => 26, // Approximate
            ScanMode::CustomList(ports) => ports.len(),
        }
    }
}

/// Configuration for port scanning
#[derive(Debug, Clone)]
pub struct ScanConfig {
    pub target_ip: IpAddr,
    pub scan_mode: ScanMode,
    pub timeout: Duration,
    pub verbose: bool,
    pub detect_versions: bool,
    pub detect_os: bool,
    pub parallel: bool,
    pub thread_count: usize,
    pub randomize_source_port: bool,
    pub delay_between_probes: Option<Duration>,
}

impl ScanConfig {
    /// Validate the configuration
    pub fn validate(&self) -> ConfigResult<()> {
        self.scan_mode.validate()?;
        
        if self.timeout.as_millis() == 0 {
            return Err(ConfigError::InvalidTimeout(self.timeout));
        }
        
        if self.parallel && self.thread_count == 0 {
            return Err(ConfigError::InvalidThreadCount(self.thread_count));
        }
        
        Ok(())
    }

    /// Get the list of ports to scan
    pub fn get_ports(&self) -> Vec<Port> {
        match &self.scan_mode {
            ScanMode::Range { start, end } => (*start..=*end).collect(),
            ScanMode::CommonPorts => {
                vec![
                    21, 22, 23, 25, 53, 80, 110, 111, 135, 139, 143, 443, 445, 993, 995,
                    1723, 3306, 3389, 5432, 5900, 6379, 8080, 8443, 8888, 9090, 27017
                ]
            }
            ScanMode::CustomList(ports) => ports.clone(),
        }
    }

    /// Get the number of ports to scan
    pub fn port_count(&self) -> usize {
        self.scan_mode.port_count()
    }

    /// Check if stealth mode is enabled
    pub fn is_stealth_enabled(&self) -> bool {
        self.randomize_source_port || self.delay_between_probes.is_some()
    }
}

/// Builder for ScanConfig
pub struct ScanConfigBuilder {
    target_ip: Option<IpAddr>,
    scan_mode: Option<ScanMode>,
    timeout: Duration,
    verbose: bool,
    detect_versions: bool,
    detect_os: bool,
    parallel: bool,
    thread_count: usize,
    randomize_source_port: bool,
    delay_between_probes: Option<Duration>,
}

impl ScanConfigBuilder {
    pub fn new() -> Self {
        Self {
            target_ip: None,
            scan_mode: None,
            timeout: DEFAULT_TIMEOUT,
            verbose: DEFAULT_VERBOSE,
            detect_versions: DEFAULT_DETECT_VERSIONS,
            detect_os: DEFAULT_DETECT_OS,
            parallel: DEFAULT_PARALLEL,
            thread_count: crate::infrastructure::network_utils::num_cpus(),
            randomize_source_port: DEFAULT_RANDOMIZE_SOURCE,
            delay_between_probes: None,
        }
    }

    pub fn target(mut self, ip: IpAddr) -> Self {
        self.target_ip = Some(ip);
        self
    }

    pub fn scan_mode(mut self, mode: ScanMode) -> Self {
        self.scan_mode = Some(mode);
        self
    }

    pub fn range(mut self, start: Port, end: Port) -> Self {
        self.scan_mode = Some(ScanMode::Range { start, end });
        self
    }

    pub fn common_ports(mut self) -> Self {
        self.scan_mode = Some(ScanMode::CommonPorts);
        self
    }

    pub fn custom_ports(mut self, ports: Vec<Port>) -> Self {
        self.scan_mode = Some(ScanMode::CustomList(ports));
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn detect_versions(mut self, detect: bool) -> Self {
        self.detect_versions = detect;
        self
    }

    pub fn detect_os(mut self, detect: bool) -> Self {
        self.detect_os = detect;
        self
    }

    pub fn parallel(mut self, parallel: bool) -> Self {
        self.parallel = parallel;
        self
    }

    pub fn thread_count(mut self, count: usize) -> Self {
        self.thread_count = count;
        self
    }

    pub fn randomize_source_port(mut self, randomize: bool) -> Self {
        self.randomize_source_port = randomize;
        self
    }

    pub fn delay_between_probes(mut self, delay: Option<Duration>) -> Self {
        self.delay_between_probes = delay;
        self
    }

    pub fn build(self) -> ConfigResult<ScanConfig> {
        let target_ip = self.target_ip
            .ok_or_else(|| ConfigError::MissingField("target_ip".to_string()))?;
        
        let scan_mode = self.scan_mode
            .ok_or_else(|| ConfigError::MissingField("scan_mode".to_string()))?;

        let config = ScanConfig {
            target_ip,
            scan_mode,
            timeout: self.timeout,
            verbose: self.verbose,
            detect_versions: self.detect_versions,
            detect_os: self.detect_os,
            parallel: self.parallel,
            thread_count: self.thread_count,
            randomize_source_port: self.randomize_source_port,
            delay_between_probes: self.delay_between_probes,
        };

        config.validate()?;
        Ok(config)
    }
}

impl Default for ScanConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
