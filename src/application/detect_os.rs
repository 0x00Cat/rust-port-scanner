/// OS detection use case

use std::net::{SocketAddr, TcpStream};
use std::io::{Read, Write};
use std::time::Duration;
use tracing::{debug, trace, warn};

use crate::domain::{Port, OSInfo};
use crate::constants::*;
use crate::scanning::Detector;

/// SMB-based OS fingerprinter
pub struct SMBFingerprinter;

impl SMBFingerprinter {
    pub fn new() -> Self {
        Self
    }

    pub fn fingerprint(socket: &SocketAddr, timeout: Duration) -> OSInfo {
        debug!("=== Starting SMB OS Fingerprinting ===");
        debug!("Target: {}", socket);
        debug!("Timeout: {:?}", timeout);
        
        match TcpStream::connect_timeout(socket, timeout) {
            Ok(mut stream) => {
                debug!("Successfully connected to SMB port");
                let _ = stream.set_read_timeout(Some(Duration::from_millis(SMB_TIMEOUT_MS)));
                let _ = stream.set_write_timeout(Some(timeout));
                
                // Send SMB negotiate packet
                let negotiate_packet = Self::build_smb_negotiate_packet();
                
                debug!("Sending SMB negotiate packet ({} bytes)", negotiate_packet.len());
                trace!("Packet data: {:02x?}", &negotiate_packet[..std::cmp::min(32, negotiate_packet.len())]);
                
                if stream.write_all(&negotiate_packet).is_err() {
                    warn!("Failed to send SMB negotiate packet to {}", socket);
                    return OSInfo::new();
                }
                
                // Read response
                let mut buffer = vec![0u8; SMB_BUFFER_SIZE];
                match stream.read(&mut buffer) {
                    Ok(n) if n > 0 => {
                        debug!("Received SMB response ({} bytes)", n);
                        trace!("Response data: {:02x?}", &buffer[..std::cmp::min(64, n)]);
                        let os_info = Self::parse_smb_response(&buffer[..n]);
                        if os_info.is_detected() {
                            debug!("Successfully detected OS: {}", os_info.summary());
                        } else {
                            debug!("Could not determine OS from SMB response");
                        }
                        os_info
                    }
                    Ok(_) => {
                        warn!("Received empty SMB response from {}", socket);
                        OSInfo::new()
                    }
                    Err(e) => {
                        warn!("Failed to read SMB response from {}: {}", socket, e);
                        OSInfo::new()
                    }
                }
            }
            Err(e) => {
                warn!("Failed to connect for SMB fingerprinting: {}", e);
                OSInfo::new()
            }
        }
    }

    fn build_smb_negotiate_packet() -> Vec<u8> {
        // Simplified SMB negotiate packet (SMB1)
        vec![
            0x00, 0x00, 0x00, 0x85, // NetBIOS header
            0xff, 0x53, 0x4d, 0x42, // SMB header "\xffSMB"
            0x72, // Negotiate Protocol
            0x00, 0x00, 0x00, 0x00, // Status
            0x18, // Flags
            0x53, 0xc8, // Flags2
            0x00, 0x00, // PID high
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Signature
            0x00, 0x00, // Reserved
            0x00, 0x00, // TID
            0xff, 0xfe, // PID
            0x00, 0x00, // UID
            0x00, 0x00, // MID
            0x00, // Word count
            0x62, 0x00, // Byte count
            0x02, // Dialects buffer
        ]
    }

    fn parse_smb_response(data: &[u8]) -> OSInfo {
        debug!("Parsing SMB response ({} bytes)", data.len());
        
        if data.len() < 32 {
            debug!("SMB response too short (minimum 32 bytes required)");
            return OSInfo::new();
        }
        
        let mut os_info = OSInfo::new();
        
        // Check for SMB2/3
        if data.len() > 4 && &data[4..8] == b"\xfeSMB" {
            debug!("Detected SMB2/3 protocol signature");
            trace!("SMB header: {:02x?}", &data[4..8]);
            os_info = os_info.with_smb_version("SMB 2.x/3.x");
            
            // Modern Windows (7+)
            debug!("Identified as modern Windows (7 or later)");
            os_info = os_info
                .with_os_name("Windows")
                .with_os_version("7 or later");
        }
        // Check for SMB1
        else if data.len() > 4 && &data[4..8] == b"\xffSMB" {
            debug!("Detected SMB1 protocol signature");
            trace!("SMB header: {:02x?}", &data[4..8]);
            os_info = os_info.with_smb_version("SMB 1.0");
            
            // Likely older Windows or Samba
            debug!("Identified as Windows/Samba (SMB1)");
            os_info = os_info.with_os_name("Windows/Samba");
        } else {
            debug!("Unknown SMB response signature: {:02x?}", &data[4..std::cmp::min(8, data.len())]);
        }
        
        os_info
    }
}

impl Default for SMBFingerprinter {
    fn default() -> Self {
        Self::new()
    }
}

impl Detector for SMBFingerprinter {
    fn name(&self) -> &str {
        "SMBFingerprinter"
    }

    fn can_detect(&self, port: Port) -> bool {
        port == 445 // SMB port
    }

    fn detect_service(&self, _socket: &SocketAddr, _timeout: Duration) -> Option<crate::domain::ServiceVersion> {
        None // This detector only does OS detection
    }

    fn detect_os(&self, socket: &SocketAddr, timeout: Duration) -> Option<OSInfo> {
        let os_info = Self::fingerprint(socket, timeout);
        if os_info.is_detected() {
            Some(os_info)
        } else {
            None
        }
    }
}
