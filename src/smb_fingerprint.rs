use std::net::{TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::time::Duration;
use serde::Serialize;

/// Enable/disable debug tracing
const TRACE_SMB: bool = true;

macro_rules! trace {
    ($($arg:tt)*) => {
        if TRACE_SMB {
            eprintln!("[SMB TRACE] {}", format!($($arg)*));
        }
    };
}

/// Represents operating system information detected from SMB
#[derive(Debug, Clone, Serialize)]
pub struct OSInfo {
    pub os_name: Option<String>,
    pub os_version: Option<String>,
    pub os_build: Option<String>,
    pub computer_name: Option<String>,
    pub domain: Option<String>,
    pub smb_version: Option<String>,
}

impl OSInfo {
    pub fn new() -> Self {
        Self {
            os_name: None,
            os_version: None,
            os_build: None,
            computer_name: None,
            domain: None,
            smb_version: None,
        }
    }

    pub fn display_string(&self) -> String {
        let mut parts = Vec::new();

        if let Some(os) = &self.os_name {
            if let Some(version) = &self.os_version {
                parts.push(format!("{} {}", os, version));
            } else {
                parts.push(os.clone());
            }
        }

        if let Some(build) = &self.os_build {
            parts.push(format!("Build {}", build));
        }

        if let Some(name) = &self.computer_name {
            parts.push(format!("Computer: {}", name));
        }

        if parts.is_empty() {
            "Unknown OS".to_string()
        } else {
            parts.join(", ")
        }
    }

    pub fn is_detected(&self) -> bool {
        self.os_name.is_some() || self.os_version.is_some() || self.computer_name.is_some()
    }
}

/// SMB fingerprinter for OS detection
pub struct SMBFingerprinter;

impl SMBFingerprinter {
    /// Attempt to fingerprint OS via SMB
    pub fn fingerprint(socket: &SocketAddr, timeout: Duration) -> OSInfo {
        trace!("Starting SMB fingerprint for {}", socket);
        
        let mut stream = match TcpStream::connect_timeout(socket, timeout) {
            Ok(s) => {
                trace!("Successfully connected to SMB port");
                s
            }
            Err(e) => {
                trace!("Failed to connect: {}", e);
                return OSInfo::new();
            }
        };

        let _ = stream.set_read_timeout(Some(Duration::from_secs(2)));
        let _ = stream.set_write_timeout(Some(Duration::from_secs(2)));

        // Try SMB negotiation
        if let Some(os_info) = Self::smb_negotiate(&mut stream) {
            trace!("SMB negotiation successful, OS info extracted");
            return os_info;
        }

        trace!("SMB negotiation failed or no OS info found");
        OSInfo::new()
    }

    /// Perform SMB protocol negotiation to extract OS info
    fn smb_negotiate(stream: &mut TcpStream) -> Option<OSInfo> {
        trace!("Building SMB negotiate packet...");
        // SMB Negotiate Protocol Request (SMBv1)
        let negotiate_packet = Self::build_smb_negotiate_packet();
        
        trace!("Sending SMB negotiate packet ({} bytes)...", negotiate_packet.len());
        if let Err(e) = stream.write_all(&negotiate_packet) {
            trace!("Failed to write SMB packet: {}", e);
            return None;
        }

        // Read response
        let mut buffer = vec![0u8; 4096];
        trace!("Waiting for SMB response...");
        let bytes_read = match stream.read(&mut buffer) {
            Ok(n) if n > 0 => {
                trace!("Received {} bytes from SMB server", n);
                n
            }
            Ok(_n) => {
                trace!("Received 0 bytes (empty response)");
                return None;
            }
            Err(e) => {
                trace!("Failed to read SMB response: {}", e);
                return None;
            }
        };

        // Debug: Show first 64 bytes in hex
        if bytes_read > 0 {
            let preview = &buffer[..bytes_read.min(64)];
            let hex_str = preview.iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ");
            trace!("Response preview (first {} bytes): {}", preview.len(), hex_str);
        }

        // Parse SMB response
        trace!("Parsing SMB response...");
        Self::parse_smb_response(&buffer[..bytes_read])
    }

    /// Build SMB Negotiate Protocol Request packet
    fn build_smb_negotiate_packet() -> Vec<u8> {
        let mut packet = Vec::new();

        // NetBIOS Session Service header
        packet.extend_from_slice(&[0x00, 0x00, 0x00, 0x85]); // Length: 133 bytes

        // SMB Header
        packet.extend_from_slice(&[0xFF, 0x53, 0x4D, 0x42]); // Protocol: SMB
        packet.push(0x72); // Command: Negotiate Protocol
        packet.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Status
        packet.push(0x18); // Flags
        packet.extend_from_slice(&[0x01, 0x28]); // Flags2
        packet.extend_from_slice(&[0x00, 0x00]); // PID High
        packet.extend_from_slice(&[0x00; 8]); // Signature
        packet.extend_from_slice(&[0x00, 0x00]); // Reserved
        packet.extend_from_slice(&[0x00, 0x00]); // TID
        packet.extend_from_slice(&[0xFF, 0xFE]); // PID
        packet.extend_from_slice(&[0x00, 0x00]); // UID
        packet.extend_from_slice(&[0x00, 0x00]); // MID

        // Negotiate Protocol Request
        packet.push(0x00); // Word Count
        packet.extend_from_slice(&[0x62, 0x00]); // Byte Count: 98

        // Dialects
        let dialects = vec![
            "\x02NT LM 0.12",
            "\x02SMB 2.002",
            "\x02SMB 2.???",
        ];

        for dialect in dialects {
            packet.extend_from_slice(dialect.as_bytes());
            packet.push(0x00);
        }

        packet
    }

    /// Parse SMB response to extract OS information
    fn parse_smb_response(data: &[u8]) -> Option<OSInfo> {
        if data.len() < 40 {
            trace!("Response too short ({} bytes), minimum 40 required", data.len());
            return None;
        }

        trace!("Checking SMB protocol signature...");
        // Check if it's a valid SMB response
        if data.len() >= 8 && &data[4..8] == b"\xFFSMB" {
            trace!("Detected SMBv1 protocol signature");
        } else if data.len() >= 8 && &data[4..8] == b"\xFESMB" {
            trace!("Detected SMBv2/3 protocol signature");
            return Self::parse_smb2_response(data);
        } else {
            trace!("Unknown protocol signature: {:02X?}", &data[4..8.min(data.len())]);
            // Try SMB2/3 anyway
            return Self::parse_smb2_response(data);
        }

        let mut os_info = OSInfo::new();
        os_info.smb_version = Some("SMB 1.0".to_string());
        trace!("Set SMB version to 1.0");

        // Look for OS information in the response
        // The OS info is typically in the session setup response
        // For negotiate response, we can detect SMB version
        
        // Try to find native OS string (after byte offset ~47)
        if let Some(os_str) = Self::extract_string_from_offset(data, 47) {
            trace!("Found OS string at offset 47: '{}'", os_str);
            os_info.os_name = Some(os_str);
        } else {
            trace!("No OS string found at offset 47");
        }

        // Try to find native LAN Manager string
        if let Some(lanman_str) = Self::extract_string_from_offset(data, 60) {
            trace!("Found LAN Manager string at offset 60: '{}'", lanman_str);
            if os_info.os_name.is_none() {
                os_info.os_name = Some(lanman_str.clone());
            }
            // Parse version from LAN Manager string
            if lanman_str.contains("Windows") {
                if let Some(ver) = Self::extract_windows_version(&lanman_str) {
                    trace!("Extracted Windows version: {}", ver);
                    os_info.os_version = Some(ver);
                }
            }
        } else {
            trace!("No LAN Manager string found at offset 60");
        }

        // Heuristic: If we see certain patterns, make educated guesses
        if data.windows_contains(b"Windows") {
            trace!("Found 'Windows' string in response");
            if os_info.os_name.is_none() {
                os_info.os_name = Some("Windows".to_string());
            }
        }

        if data.windows_contains(b"Samba") {
            trace!("Found 'Samba' string in response");
            os_info.os_name = Some("Linux/Unix (Samba)".to_string());
            if let Some(version) = Self::extract_samba_version(data) {
                trace!("Extracted Samba version: {}", version);
                os_info.os_version = Some(version);
            }
        }

        if os_info.os_name.is_some() {
            trace!("Successfully extracted OS info: {:?}", os_info);
            Some(os_info)
        } else {
            trace!("No OS information could be extracted");
            None
        }
    }

    /// Parse SMB2/SMB3 response
    fn parse_smb2_response(data: &[u8]) -> Option<OSInfo> {
        if data.len() < 8 {
            trace!("SMB2 response too short: {} bytes", data.len());
            return None;
        }

        // Check for SMB2 magic bytes
        if &data[4..8] == b"\xFESMB" {
            trace!("Confirmed SMB2/3 magic bytes");
            let mut os_info = OSInfo::new();
            
            // Determine SMB version from dialect
            if data.len() > 72 {
                let dialect = u16::from_le_bytes([data[72], data[73]]);
                trace!("SMB dialect code: 0x{:04X}", dialect);
                os_info.smb_version = Some(match dialect {
                    0x0202 => {
                        trace!("Identified as SMB 2.0.2");
                        "SMB 2.0.2".to_string()
                    }
                    0x0210 => {
                        trace!("Identified as SMB 2.1");
                        "SMB 2.1".to_string()
                    }
                    0x0300 => {
                        trace!("Identified as SMB 3.0");
                        "SMB 3.0".to_string()
                    }
                    0x0302 => {
                        trace!("Identified as SMB 3.0.2");
                        "SMB 3.0.2".to_string()
                    }
                    0x0311 => {
                        trace!("Identified as SMB 3.1.1");
                        "SMB 3.1.1".to_string()
                    }
                    _ => {
                        trace!("Unknown SMB dialect: 0x{:04X}", dialect);
                        format!("SMB 2/3 (Dialect: 0x{:04X})", dialect)
                    }
                });
            } else {
                trace!("Response too short to determine exact SMB version, using generic SMB 2/3");
                os_info.smb_version = Some("SMB 2/3".to_string());
            }

            // SMB2/3 typically indicates modern Windows or Samba
            if data.windows_contains(b"Windows") {
                trace!("Found 'Windows' in SMB2 response");
                os_info.os_name = Some("Windows".to_string());
                os_info.os_version = Some("Vista or later".to_string());
            } else if data.windows_contains(b"Samba") {
                trace!("Found 'Samba' in SMB2 response");
                os_info.os_name = Some("Linux/Unix (Samba)".to_string());
            } else {
                // Default assumption for SMB2/3
                trace!("No OS markers found, assuming modern Windows");
                os_info.os_name = Some("Windows (Modern)".to_string());
            }

            trace!("SMB2/3 OS info: {:?}", os_info);
            return Some(os_info);
        }

        trace!("Not a valid SMB2/3 response");
        None
    }

    /// Extract null-terminated string from data starting at offset
    fn extract_string_from_offset(data: &[u8], offset: usize) -> Option<String> {
        if offset >= data.len() {
            trace!("Offset {} is beyond data length {}", offset, data.len());
            return None;
        }

        let mut end = offset;
        while end < data.len() && data[end] != 0 {
            end += 1;
            if end - offset > 100 {
                trace!("String too long (>100 bytes), truncating");
                break; // Prevent reading too much
            }
        }

        if end > offset {
            match String::from_utf8(data[offset..end].to_vec()) {
                Ok(s) => {
                    trace!("Extracted string from offset {}: '{}'", offset, s);
                    Some(s)
                }
                Err(e) => {
                    trace!("Failed to decode UTF-8 string at offset {}: {}", offset, e);
                    None
                }
            }
        } else {
            trace!("No string data at offset {}", offset);
            None
        }
    }

    /// Extract Windows version from string
    fn extract_windows_version(s: &str) -> Option<String> {
        if s.contains("10.0") || s.contains("Windows 10") {
            Some("10/11".to_string())
        } else if s.contains("6.3") || s.contains("Windows 8.1") {
            Some("8.1".to_string())
        } else if s.contains("6.2") || s.contains("Windows 8") {
            Some("8".to_string())
        } else if s.contains("6.1") || s.contains("Windows 7") {
            Some("7".to_string())
        } else if s.contains("6.0") || s.contains("Vista") {
            Some("Vista".to_string())
        } else if s.contains("5.2") || s.contains("Server 2003") {
            Some("Server 2003".to_string())
        } else if s.contains("5.1") || s.contains("XP") {
            Some("XP".to_string())
        } else if s.contains("2000") {
            Some("2000".to_string())
        } else {
            None
        }
    }

    /// Extract Samba version from response
    fn extract_samba_version(data: &[u8]) -> Option<String> {
        let data_str = String::from_utf8_lossy(data);
        
        // Look for Samba version pattern
        if let Some(start) = data_str.find("Samba ") {
            let version_part = &data_str[start + 6..];
            if let Some(end) = version_part.find(|c: char| !c.is_ascii_digit() && c != '.') {
                return Some(version_part[..end].to_string());
            }
        }
        
        None
    }
}

/// Helper trait for searching bytes in a slice
trait ByteSearch {
    fn windows_contains(&self, needle: &[u8]) -> bool;
}

impl ByteSearch for [u8] {
    fn windows_contains(&self, needle: &[u8]) -> bool {
        if needle.is_empty() || self.len() < needle.len() {
            return false;
        }
        
        self.windows(needle.len()).any(|window| window == needle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_windows_version_extraction() {
        assert_eq!(
            SMBFingerprinter::extract_windows_version("Windows 10.0"),
            Some("10/11".to_string())
        );
        assert_eq!(
            SMBFingerprinter::extract_windows_version("Windows 6.1"),
            Some("7".to_string())
        );
    }

    #[test]
    fn test_os_info_display() {
        let mut info = OSInfo::new();
        info.os_name = Some("Windows".to_string());
        info.os_version = Some("10".to_string());
        info.os_build = Some("19044".to_string());
        
        let display = info.display_string();
        assert!(display.contains("Windows 10"));
        assert!(display.contains("Build 19044"));
    }
}
