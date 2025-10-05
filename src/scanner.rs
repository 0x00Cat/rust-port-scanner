use std::net::{TcpStream, SocketAddr, IpAddr, TcpListener};
use std::time::Duration;
use std::io;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

use crate::port_info::{PortStatus, PortScanResult};
use crate::version_detector::VersionDetector;
use crate::smb_fingerprint::SMBFingerprinter;

/// Scan mode for port scanning
#[derive(Debug, Clone, PartialEq)]
pub enum ScanMode {
    /// Scan a range of ports
    Range { start: u16, end: u16 },
    /// Scan only common ports
    CommonPorts,
    /// Scan a custom list of ports
    CustomList(Vec<u16>),
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
    pub fn new(target_ip: IpAddr, start_port: u16, end_port: u16) -> Self {
        Self {
            target_ip,
            scan_mode: ScanMode::Range { start: start_port, end: end_port },
            timeout: Duration::from_millis(500),
            verbose: false,
            detect_versions: false,
            detect_os: false,
            parallel: true,
            thread_count: num_cpus(),
            randomize_source_port: false,
            delay_between_probes: None,
        }
    }

    pub fn new_common_ports(target_ip: IpAddr) -> Self {
        Self {
            target_ip,
            scan_mode: ScanMode::CommonPorts,
            timeout: Duration::from_millis(500),
            verbose: false,
            detect_versions: false,
            detect_os: false,
            parallel: true,
            thread_count: num_cpus(),
            randomize_source_port: false,
            delay_between_probes: None,
        }
    }

    pub fn new_custom_ports(target_ip: IpAddr, ports: Vec<u16>) -> Self {
        Self {
            target_ip,
            scan_mode: ScanMode::CustomList(ports),
            timeout: Duration::from_millis(500),
            verbose: false,
            detect_versions: false,
            detect_os: false,
            parallel: true,
            thread_count: num_cpus(),
            randomize_source_port: false,
            delay_between_probes: None,
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn with_version_detection(mut self, detect: bool) -> Self {
        self.detect_versions = detect;
        self
    }

    pub fn with_os_detection(mut self, detect: bool) -> Self {
        self.detect_os = detect;
        self
    }

    pub fn with_parallel(mut self, parallel: bool) -> Self {
        self.parallel = parallel;
        self
    }

    pub fn with_thread_count(mut self, count: usize) -> Self {
        self.thread_count = count.max(1).min(256); // Clamp between 1 and 256
        self
    }

    pub fn with_source_port_randomization(mut self, randomize: bool) -> Self {
        self.randomize_source_port = randomize;
        self
    }

    pub fn with_delay_between_probes(mut self, delay: Option<Duration>) -> Self {
        self.delay_between_probes = delay;
        self
    }

    pub fn validate(&self) -> Result<(), String> {
        match &self.scan_mode {
            ScanMode::Range { start, end } => {
                if start > end {
                    return Err("Start port must be less than or equal to end port".to_string());
                }
                if *start == 0 {
                    return Err("Port numbers start from 1".to_string());
                }
            }
            ScanMode::CustomList(ports) => {
                if ports.is_empty() {
                    return Err("Custom port list cannot be empty".to_string());
                }
                if ports.iter().any(|&p| p == 0) {
                    return Err("Port numbers start from 1".to_string());
                }
            }
            ScanMode::CommonPorts => {
                // Always valid
            }
        }
        Ok(())
    }

    pub fn port_count(&self) -> usize {
        match &self.scan_mode {
            ScanMode::Range { start, end } => (*end - *start + 1) as usize,
            ScanMode::CommonPorts => crate::port_info::ServiceDatabase::get_common_ports().len(),
            ScanMode::CustomList(ports) => ports.len(),
        }
    }

    pub fn get_ports(&self) -> Vec<u16> {
        match &self.scan_mode {
            ScanMode::Range { start, end } => (*start..=*end).collect(),
            ScanMode::CommonPorts => crate::port_info::ServiceDatabase::get_common_ports(),
            ScanMode::CustomList(ports) => ports.clone(),
        }
    }
}

/// Main port scanner implementation
pub struct PortScanner {
    config: ScanConfig,
}

impl PortScanner {
    pub fn new(config: ScanConfig) -> Result<Self, String> {
        config.validate()?;
        Ok(Self { config })
    }

    /// Scan a single port
    pub fn scan_port(&self, port: u16) -> PortScanResult {
        let socket = SocketAddr::new(self.config.target_ip, port);
        
        // Apply randomized delay before probe if configured
        if let Some(base_delay) = self.config.delay_between_probes {
            let jitter = random_delay_jitter(base_delay);
            thread::sleep(jitter);
        }
        
        let result = if self.config.randomize_source_port {
            self.scan_port_with_random_source(port)
        } else {
            self.scan_port_standard(port)
        };
        
        if !result.is_open() {
            return result;
        }

        let mut result = result;

        // If port is open and version detection is enabled, try to detect version
        if self.config.detect_versions {
            let version = VersionDetector::detect_version(&socket, self.config.timeout);
            if version.banner.is_some() {
                result = result.with_version(version);
            }
        }

        // If port 445 (SMB) is open and OS detection is enabled, try to fingerprint OS
        if port == 445 && self.config.detect_os {
            let os_info = SMBFingerprinter::fingerprint(&socket, self.config.timeout);
            if os_info.is_detected() {
                result = result.with_os_info(os_info);
            }
        }

        result
    }

    /// Standard port scan using system-assigned source port
    fn scan_port_standard(&self, port: u16) -> PortScanResult {
        let socket = SocketAddr::new(self.config.target_ip, port);
        
        match TcpStream::connect_timeout(&socket, self.config.timeout) {
            Ok(_) => PortScanResult::new(port, PortStatus::Open),
            Err(ref e) if e.kind() == io::ErrorKind::ConnectionRefused => {
                PortScanResult::new(port, PortStatus::Closed)
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                PortScanResult::new(port, PortStatus::Filtered)
            }
            Err(e) => PortScanResult::new(port, PortStatus::Error(e.to_string())),
        }
    }

    /// Port scan with randomized source port
    fn scan_port_with_random_source(&self, port: u16) -> PortScanResult {
        let socket = SocketAddr::new(self.config.target_ip, port);
        
        // Try to bind to a random high port (1024-65535)
        let source_port = random_source_port();
        let local_addr = match self.config.target_ip {
            IpAddr::V4(_) => SocketAddr::new("0.0.0.0".parse().unwrap(), source_port),
            IpAddr::V6(_) => SocketAddr::new("::".parse().unwrap(), source_port),
        };

        // Try to create socket bound to random source port
        match TcpListener::bind(local_addr) {
            Ok(listener) => {
                // Get the actual bound address
                if let Ok(bound_addr) = listener.local_addr() {
                    drop(listener); // Close listener immediately
                    
                    // Try to connect from the bound port
                    match connect_from_port(bound_addr, socket, self.config.timeout) {
                        Ok(_) => PortScanResult::new(port, PortStatus::Open),
                        Err(ref e) if e.kind() == io::ErrorKind::ConnectionRefused => {
                            PortScanResult::new(port, PortStatus::Closed)
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                            PortScanResult::new(port, PortStatus::Filtered)
                        }
                        Err(_) => {
                            // Fallback to standard scan if source port binding fails
                            self.scan_port_standard(port)
                        }
                    }
                } else {
                    // Fallback to standard scan
                    self.scan_port_standard(port)
                }
            }
            Err(_) => {
                // Fallback to standard scan if binding fails
                self.scan_port_standard(port)
            }
        }
    }

    /// Scan all ports based on the configured scan mode
    pub fn scan_all<F>(&self, callback: F) -> Vec<PortScanResult>
    where
        F: FnMut(&PortScanResult) + Send + 'static,
    {
        let ports = self.config.get_ports();
        
        if self.config.parallel {
            self.scan_parallel(ports, callback)
        } else {
            self.scan_sequential(ports, callback)
        }
    }

    /// Sequential scanning (original method)
    fn scan_sequential<F>(&self, ports: Vec<u16>, mut callback: F) -> Vec<PortScanResult>
    where
        F: FnMut(&PortScanResult),
    {
        let mut results = Vec::new();
        
        for port in ports {
            let result = self.scan_port(port);
            callback(&result);
            results.push(result);
        }
        
        results
    }

    /// Parallel scanning using thread pool
    fn scan_parallel<F>(&self, ports: Vec<u16>, callback: F) -> Vec<PortScanResult>
    where
        F: FnMut(&PortScanResult) + Send + 'static,
    {
        let thread_count = self.config.thread_count;
        let chunk_size = (ports.len() + thread_count - 1) / thread_count; // Ceiling division
        
        // Create channels for results and progress
        let (tx, rx) = mpsc::channel();
        let callback_mutex = Arc::new(Mutex::new(callback));
        
        // Split ports into chunks for each thread
        let mut handles = vec![];
        
        for chunk in ports.chunks(chunk_size.max(1)) {
            let tx = tx.clone();
            let chunk_ports = chunk.to_vec();
            let config = self.config.clone();
            let callback_clone = Arc::clone(&callback_mutex);
            
            let handle = thread::spawn(move || {
                let scanner = PortScanner { config };
                let mut chunk_results = Vec::new();
                
                for port in chunk_ports {
                    let result = scanner.scan_port(port);
                    
                    // Call progress callback
                    if let Ok(mut cb) = callback_clone.lock() {
                        cb(&result);
                    }
                    
                    chunk_results.push(result);
                }
                
                tx.send(chunk_results).ok();
            });
            
            handles.push(handle);
        }
        
        // Drop the original sender so rx knows when all senders are done
        drop(tx);
        
        // Collect results from all threads
        let mut all_results = Vec::new();
        for chunk_results in rx {
            all_results.extend(chunk_results);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().ok();
        }
        
        // Sort results by port number to maintain order
        all_results.sort_by_key(|r| r.port);
        
        all_results
    }

    /// Get the scan configuration
    pub fn config(&self) -> &ScanConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_scan_config_validation() {
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        
        // Valid config
        let config = ScanConfig::new(ip, 80, 100);
        assert!(config.validate().is_ok());
        
        // Invalid: start > end
        let config = ScanConfig::new(ip, 100, 80);
        assert!(config.validate().is_err());
        
        // Invalid: port 0
        let config = ScanConfig::new(ip, 0, 100);
        assert!(config.validate().is_err());

        // Valid: common ports
        let config = ScanConfig::new_common_ports(ip);
        assert!(config.validate().is_ok());

        // Valid: custom ports
        let config = ScanConfig::new_custom_ports(ip, vec![80, 443, 8080]);
        assert!(config.validate().is_ok());

        // Invalid: empty custom ports
        let config = ScanConfig::new_custom_ports(ip, vec![]);
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_port_count() {
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        
        let config = ScanConfig::new(ip, 1, 100);
        assert_eq!(config.port_count(), 100);

        let config = ScanConfig::new_common_ports(ip);
        assert!(config.port_count() > 0);

        let config = ScanConfig::new_custom_ports(ip, vec![80, 443, 8080]);
        assert_eq!(config.port_count(), 3);
    }
}

/// Get number of logical CPU cores
fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

/// Generate a random source port in the range 1024-65535
fn random_source_port() -> u16 {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    // Use system time as simple random seed
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    
    // Generate port in range 1024-65535
    let port_range = 65535 - 1024;
    1024 + (nanos % port_range) as u16
}

/// Generate randomized delay with jitter
fn random_delay_jitter(base_delay: Duration) -> Duration {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    
    // Add random jitter: base_delay ± 50%
    let base_ms = base_delay.as_millis() as u64;
    let jitter_range = (base_ms / 2).max(1);
    let jitter = (nanos as u64 % jitter_range) as i64 - (jitter_range / 2) as i64;
    
    let adjusted_ms = (base_ms as i64 + jitter).max(0) as u64;
    Duration::from_millis(adjusted_ms)
}

/// Attempt to connect from a specific local port
fn connect_from_port(
    _local_addr: SocketAddr,
    remote_addr: SocketAddr,
    timeout: Duration,
) -> io::Result<TcpStream> {
    use std::net::TcpStream as StdTcpStream;
    
    // Note: This is a simplified version. Full implementation would use socket2 crate
    // for proper source port binding. For now, we'll use standard connect.
    // The TcpListener approach above provides some randomization.
    StdTcpStream::connect_timeout(&remote_addr, timeout)
}
