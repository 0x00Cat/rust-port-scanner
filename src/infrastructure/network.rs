/// Network connectivity abstractions

use std::io;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

/// Trait for network connectivity to enable testing and mocking
pub trait NetworkConnector: Send + Sync {
    fn connect(&self, addr: &SocketAddr, timeout: Duration) -> io::Result<TcpStream>;
}

/// Real TCP network connector
#[derive(Debug, Clone)]
pub struct TcpConnector;

impl TcpConnector {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TcpConnector {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkConnector for TcpConnector {
    fn connect(&self, addr: &SocketAddr, timeout: Duration) -> io::Result<TcpStream> {
        TcpStream::connect_timeout(addr, timeout)
    }
}

/// Helper functions for network operations
pub mod network_utils {
    use super::*;
    use std::net::{IpAddr, TcpListener};
    use std::io::ErrorKind;
    
    /// Generate a random high port number (1024-65535)
    pub fn random_source_port() -> u16 {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        
        let port = (timestamp % (65535 - 1024) as u128 + 1024) as u16;
        port
    }

    /// Calculate random delay with jitter
    pub fn random_delay_jitter(base_delay: Duration, jitter_percent: u64) -> Duration {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        
        let jitter_range = (base_delay.as_millis() * jitter_percent as u128) / 100;
        let jitter = (timestamp % (jitter_range * 2)) as i128 - jitter_range as i128;
        
        let new_delay_ms = base_delay.as_millis() as i128 + jitter;
        let new_delay_ms = new_delay_ms.max(0) as u64;
        
        Duration::from_millis(new_delay_ms)
    }

    /// Attempt to connect from a specific source port
    pub fn connect_from_port(
        local_addr: SocketAddr,
        remote_addr: SocketAddr,
        timeout: Duration,
    ) -> io::Result<TcpStream> {
        // This is a simplified version - full implementation would use socket2 crate
        // For now, fall back to standard connection
        TcpStream::connect_timeout(&remote_addr, timeout)
    }

    /// Get number of CPU cores for parallel processing
    pub fn num_cpus() -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(8)
    }

    /// Check if a port scan result indicates connection refused
    pub fn is_connection_refused(error: &io::Error) -> bool {
        error.kind() == ErrorKind::ConnectionRefused
    }

    /// Check if a port scan result indicates timeout
    pub fn is_timeout(error: &io::Error) -> bool {
        error.kind() == ErrorKind::TimedOut
    }
}
