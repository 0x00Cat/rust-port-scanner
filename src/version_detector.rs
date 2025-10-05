use std::net::{TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::time::Duration;
use serde::Serialize;

/// Represents service version information
#[derive(Debug, Clone, Serialize)]
pub struct ServiceVersion {
    pub banner: Option<String>,
    pub service_name: Option<String>,
    pub version: Option<String>,
}

impl ServiceVersion {
    pub fn new() -> Self {
        Self {
            banner: None,
            service_name: None,
            version: None,
        }
    }

    pub fn with_banner(mut self, banner: String) -> Self {
        // Parse the banner to extract service name and version
        let (service, version) = Self::parse_banner(&banner);
        self.banner = Some(banner);
        self.service_name = service;
        self.version = version;
        self
    }

    /// Parse banner to extract service name and version
    fn parse_banner(banner: &str) -> (Option<String>, Option<String>) {
        let banner_lower = banner.to_lowercase();
        
        // Common patterns for different services
        if banner_lower.contains("ssh") {
            return Self::parse_ssh_banner(banner);
        } else if banner_lower.contains("http") || banner_lower.contains("server:") {
            return Self::parse_http_banner(banner);
        } else if banner_lower.contains("ftp") {
            return Self::parse_ftp_banner(banner);
        } else if banner_lower.contains("smtp") {
            return Self::parse_smtp_banner(banner);
        } else if banner_lower.contains("mysql") {
            return (Some("MySQL".to_string()), None);
        } else if banner_lower.contains("postgresql") || banner_lower.contains("postgres") {
            return (Some("PostgreSQL".to_string()), None);
        }
        
        (None, None)
    }

    fn parse_ssh_banner(banner: &str) -> (Option<String>, Option<String>) {
        // SSH banner format: SSH-2.0-OpenSSH_8.2p1 Ubuntu-4ubuntu0.5
        if let Some(version_part) = banner.split("SSH-").nth(1) {
            let parts: Vec<&str> = version_part.split('-').collect();
            if parts.len() >= 2 {
                let service = parts[1].split('_').next().unwrap_or("SSH");
                let version = parts[1].split('_').nth(1);
                return (
                    Some(service.to_string()),
                    version.map(|v| v.split_whitespace().next().unwrap_or(v).to_string())
                );
            }
        }
        (Some("SSH".to_string()), None)
    }

    fn parse_http_banner(banner: &str) -> (Option<String>, Option<String>) {
        // Look for Server: header
        for line in banner.lines() {
            if line.to_lowercase().starts_with("server:") {
                let server_info = line.split(':').nth(1).unwrap_or("").trim();
                let parts: Vec<&str> = server_info.split('/').collect();
                if parts.len() >= 2 {
                    return (
                        Some(parts[0].to_string()),
                        Some(parts[1].split_whitespace().next().unwrap_or(parts[1]).to_string())
                    );
                }
                return (Some(server_info.to_string()), None);
            }
        }
        (Some("HTTP".to_string()), None)
    }

    fn parse_ftp_banner(banner: &str) -> (Option<String>, Option<String>) {
        // FTP banner format: 220 ProFTPD 1.3.5 Server
        let parts: Vec<&str> = banner.split_whitespace().collect();
        if parts.len() >= 3 {
            let service = parts[1];
            let version = parts.get(2).map(|v| v.to_string());
            return (Some(service.to_string()), version);
        }
        (Some("FTP".to_string()), None)
    }

    fn parse_smtp_banner(banner: &str) -> (Option<String>, Option<String>) {
        // SMTP banner format: 220 mail.example.com ESMTP Postfix
        if banner.contains("postfix") {
            return (Some("Postfix".to_string()), None);
        } else if banner.contains("exim") {
            return (Some("Exim".to_string()), None);
        } else if banner.contains("sendmail") {
            return (Some("Sendmail".to_string()), None);
        }
        (Some("SMTP".to_string()), None)
    }

    pub fn display_string(&self) -> String {
        match (&self.service_name, &self.version) {
            (Some(service), Some(version)) => format!("{} {}", service, version),
            (Some(service), None) => service.clone(),
            (None, Some(version)) => version.clone(),
            (None, None) => "Unknown".to_string(),
        }
    }
}

/// Service version detector using banner grabbing
pub struct VersionDetector;

impl VersionDetector {
    /// Attempt to grab banner from a service
    pub fn detect_version(socket: &SocketAddr, timeout: Duration) -> ServiceVersion {
        let mut stream = match TcpStream::connect_timeout(socket, timeout) {
            Ok(s) => s,
            Err(_) => return ServiceVersion::new(),
        };

        // Set read timeout
        let _ = stream.set_read_timeout(Some(Duration::from_millis(1000)));
        let _ = stream.set_write_timeout(Some(Duration::from_millis(1000)));

        // Try to get banner
        if let Some(banner) = Self::grab_banner(&mut stream, socket.port()) {
            return ServiceVersion::new().with_banner(banner);
        }

        ServiceVersion::new()
    }

    /// Grab banner from the service
    fn grab_banner(stream: &mut TcpStream, port: u16) -> Option<String> {
        let mut buffer = vec![0u8; 1024];

        // For some services, we need to send a probe first
        match port {
            80 | 8000 | 8080 | 8443 => {
                // HTTP probe
                let _ = stream.write_all(b"HEAD / HTTP/1.0\r\n\r\n");
            }
            25 | 587 | 465 => {
                // SMTP - just read the banner
            }
            110 | 995 => {
                // POP3 - just read the banner
            }
            143 | 993 => {
                // IMAP - just read the banner
            }
            21 => {
                // FTP - just read the banner
            }
            22 => {
                // SSH - just read the banner
            }
            _ => {
                // For unknown services, try generic probe
                let _ = stream.write_all(b"\r\n");
            }
        }

        // Try to read response
        match stream.read(&mut buffer) {
            Ok(n) if n > 0 => {
                let banner = String::from_utf8_lossy(&buffer[..n]).to_string();
                // Clean up the banner
                let cleaned = banner
                    .lines()
                    .take(5) // Take first 5 lines
                    .collect::<Vec<_>>()
                    .join(" | ");
                
                if !cleaned.trim().is_empty() {
                    Some(cleaned.trim().to_string())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Quick banner grab with shorter timeout
    pub fn quick_detect(socket: &SocketAddr) -> ServiceVersion {
        Self::detect_version(socket, Duration::from_millis(500))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssh_banner_parsing() {
        let banner = "SSH-2.0-OpenSSH_8.2p1 Ubuntu-4ubuntu0.5";
        let version = ServiceVersion::new().with_banner(banner.to_string());
        assert_eq!(version.service_name, Some("OpenSSH".to_string()));
    }

    #[test]
    fn test_http_banner_parsing() {
        let banner = "HTTP/1.1 200 OK\r\nServer: nginx/1.18.0\r\n";
        let version = ServiceVersion::new().with_banner(banner.to_string());
        assert_eq!(version.service_name, Some("nginx".to_string()));
        assert_eq!(version.version, Some("1.18.0".to_string()));
    }

    #[test]
    fn test_ftp_banner_parsing() {
        let banner = "220 ProFTPD 1.3.5 Server";
        let version = ServiceVersion::new().with_banner(banner.to_string());
        assert_eq!(version.service_name, Some("ProFTPD".to_string()));
    }
}
