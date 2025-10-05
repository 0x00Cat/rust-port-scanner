/// Service detection use case

use std::net::{SocketAddr, TcpStream};
use std::io::{Read, Write};
use std::time::Duration;
use tokio::net::TcpStream as AsyncTcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::timeout as async_timeout;
use tracing::{debug, trace, warn};

use crate::domain::{Port, ServiceVersion};
use crate::constants::*;
use crate::scanning::Detector;

/// Version detector implementation
pub struct VersionDetector;

impl VersionDetector {
    pub fn new() -> Self {
        Self
    }

    /// Async version detection (NEW - for async scanning)
    pub async fn detect_version_async(socket: &SocketAddr, timeout: Duration) -> ServiceVersion {
        let port = socket.port();

        debug!("Attempting async version detection on port {}", port);

        // Try to connect and grab banner with async
        match async_timeout(timeout, AsyncTcpStream::connect(socket)).await {
            Ok(Ok(mut stream)) => {
                let mut buffer = vec![0u8; BANNER_BUFFER_SIZE];

                // Try reading banner first
                match async_timeout(
                    Duration::from_millis(BANNER_READ_TIMEOUT_MS), 
                    stream.read(&mut buffer)
                ).await {
                    Ok(Ok(n)) if n > 0 => {
                        let banner = String::from_utf8_lossy(&buffer[..n]).to_string();
                        trace!("Received banner from port {}: {}", port, banner);
                        return Self::parse_banner(port, &banner);
                    }
                    _ => {
                        // Try sending a probe
                        return Self::send_probe_and_read_async(port, &mut stream, &mut buffer).await;
                    }
                }
            }
            Ok(Err(e)) => {
                warn!("Failed to connect for async version detection on port {}: {}", port, e);
                ServiceVersion::unknown()
            }
            Err(_) => {
                warn!("Connection timeout for async version detection on port {}", port);
                ServiceVersion::unknown()
            }
        }
    }

    /// Sync version detection (kept for compatibility)
    pub fn detect_version(socket: &SocketAddr, timeout: Duration) -> ServiceVersion {
        let port = socket.port();
        
        debug!("Attempting version detection on port {}", port);
        
        // Try to connect and grab banner
        match TcpStream::connect_timeout(socket, timeout) {
            Ok(mut stream) => {
                let _ = stream.set_read_timeout(Some(Duration::from_millis(BANNER_READ_TIMEOUT_MS)));
                let _ = stream.set_write_timeout(Some(timeout));
                
                let mut buffer = vec![0u8; BANNER_BUFFER_SIZE];
                
                // Try reading banner
                match stream.read(&mut buffer) {
                    Ok(n) if n > 0 => {
                        let banner = String::from_utf8_lossy(&buffer[..n]).to_string();
                        trace!("Received banner from port {}: {}", port, banner);
                        Self::parse_banner(port, &banner)
                    }
                    _ => {
                        // Try sending a probe
                        Self::send_probe_and_read(port, &mut stream, &mut buffer)
                    }
                }
            }
            Err(e) => {
                warn!("Failed to connect for version detection on port {}: {}", port, e);
                ServiceVersion::unknown()
            }
        }
    }

    async fn send_probe_and_read_async(port: Port, stream: &mut AsyncTcpStream, buffer: &mut [u8]) -> ServiceVersion {
        let probe: &[u8] = match port {
            80 | 8080 | 8443 => b"GET / HTTP/1.0\r\n\r\n",
            21 => b"",  // FTP sends banner automatically
            22 => b"",  // SSH sends banner automatically
            25 => b"EHLO scanner\r\n",
            _ => b"",
        };

        if !probe.is_empty() {
            trace!("Sending async probe to port {}", port);
            let _ = stream.write_all(probe).await;
        }

        match async_timeout(
            Duration::from_millis(BANNER_READ_TIMEOUT_MS),
            stream.read(buffer)
        ).await {
            Ok(Ok(n)) if n > 0 => {
                let banner = String::from_utf8_lossy(&buffer[..n]).to_string();
                trace!("Received async response from port {}: {}", port, banner);
                Self::parse_banner(port, &banner)
            }
            _ => ServiceVersion::unknown(),
        }
    }

    fn send_probe_and_read(port: Port, stream: &mut TcpStream, buffer: &mut [u8]) -> ServiceVersion {
        let probe: &[u8] = match port {
            80 | 8080 | 8443 => b"GET / HTTP/1.0\r\n\r\n",
            21 => b"",  // FTP sends banner automatically
            22 => b"",  // SSH sends banner automatically
            25 => b"EHLO scanner\r\n",
            _ => b"",
        };

        if !probe.is_empty() {
            trace!("Sending probe to port {}", port);
            let _ = stream.write_all(probe);
        }

        match stream.read(buffer) {
            Ok(n) if n > 0 => {
                let banner = String::from_utf8_lossy(&buffer[..n]).to_string();
                trace!("Received response from port {}: {}", port, banner);
                Self::parse_banner(port, &banner)
            }
            _ => ServiceVersion::unknown(),
        }
    }

    fn parse_banner(port: Port, banner: &str) -> ServiceVersion {
        let banner_lower = banner.to_lowercase();
        
        // SSH detection
        if banner_lower.starts_with("ssh-") {
            let parts: Vec<&str> = banner.split_whitespace().collect();
            if parts.len() >= 2 {
                return ServiceVersion::new("SSH", "tcp")
                    .with_version(parts[0].trim_start_matches("SSH-"))
                    .with_banner(parts[1]);
            }
            return ServiceVersion::new("SSH", "tcp").with_banner(banner);
        }
        
        // HTTP detection
        if banner_lower.contains("http/") {
            if let Some(server_line) = banner.lines().find(|l| l.to_lowercase().starts_with("server:")) {
                let server = server_line.trim_start_matches("Server:").trim().to_string();
                return ServiceVersion::new("HTTP", "tcp").with_banner(server);
            }
            return ServiceVersion::new("HTTP", "tcp").with_banner("HTTP");
        }
        
        // FTP detection
        if banner_lower.contains("ftp") || banner.starts_with("220") {
            return ServiceVersion::new("FTP", "tcp").with_banner(banner);
        }
        
        // SMTP detection
        if banner.starts_with("220 ") && (banner_lower.contains("smtp") || banner_lower.contains("mail")) {
            return ServiceVersion::new("SMTP", "tcp").with_banner(banner);
        }
        
        // Default
        ServiceVersion::new("unknown", "tcp").with_banner(banner)
    }
}

impl Default for VersionDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl Detector for VersionDetector {
    fn name(&self) -> &str {
        "VersionDetector"
    }

    fn can_detect(&self, port: Port) -> bool {
        // Can attempt detection on most common ports
        matches!(port, 21 | 22 | 23 | 25 | 80 | 110 | 143 | 443 | 8080 | 8443)
    }

    fn detect_service(&self, socket: &SocketAddr, timeout: Duration) -> Option<ServiceVersion> {
        let version = Self::detect_version(socket, timeout);
        if version.service_name != "unknown" || version.banner.is_some() {
            Some(version)
        } else {
            None
        }
    }
}
