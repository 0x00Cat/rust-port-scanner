use crate::version_detector::ServiceVersion;
use crate::smb_fingerprint::OSInfo;
use serde::Serialize;

/// Represents the status of a scanned port
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum PortStatus {
    Open,
    Closed,
    Filtered,
    Error(String),
}

impl PortStatus {
    pub fn is_open(&self) -> bool {
        matches!(self, PortStatus::Open)
    }

    pub fn is_closed(&self) -> bool {
        matches!(self, PortStatus::Closed)
    }

    pub fn is_filtered(&self) -> bool {
        matches!(self, PortStatus::Filtered)
    }

    pub fn is_error(&self) -> bool {
        matches!(self, PortStatus::Error(_))
    }
}

/// Result of scanning a single port
#[derive(Debug, Clone, Serialize)]
pub struct PortScanResult {
    pub port: u16,
    pub status: PortStatus,
    pub service_version: Option<ServiceVersion>,
    pub os_info: Option<OSInfo>,
}

impl PortScanResult {
    pub fn new(port: u16, status: PortStatus) -> Self {
        Self { 
            port, 
            status,
            service_version: None,
            os_info: None,
        }
    }

    pub fn with_version(mut self, version: ServiceVersion) -> Self {
        self.service_version = Some(version);
        self
    }

    pub fn with_os_info(mut self, os_info: OSInfo) -> Self {
        self.os_info = Some(os_info);
        self
    }

    pub fn is_open(&self) -> bool {
        matches!(self.status, PortStatus::Open)
    }
}

/// Database of common port-to-service mappings
pub struct ServiceDatabase;

impl ServiceDatabase {
    /// Get a list of the most common ports to scan
    pub fn get_common_ports() -> Vec<u16> {
        vec![
            20, 21,    // FTP
            22,        // SSH
            23,        // Telnet
            25,        // SMTP
            53,        // DNS
            80,        // HTTP
            110,       // POP3
            143,       // IMAP
            443,       // HTTPS
            445,       // SMB
            465,       // SMTPS
            587,       // SMTP Submission
            993,       // IMAPS
            995,       // POP3S
            1433,      // MS SQL
            1521,      // Oracle
            3306,      // MySQL
            3389,      // RDP
            5432,      // PostgreSQL
            5900,      // VNC
            6379,      // Redis
            8000,      // HTTP Alt
            8080,      // HTTP Proxy
            8443,      // HTTPS Alt
            9200,      // Elasticsearch
            27017,     // MongoDB
        ]
    }

    /// Get the top N most common ports
    pub fn get_top_ports(count: usize) -> Vec<u16> {
        let common = Self::get_common_ports();
        common.into_iter().take(count).collect()
    }

    pub fn get_service_name(port: u16) -> Option<&'static str> {
        match port {
            20 => Some("FTP Data"),
            21 => Some("FTP Control"),
            22 => Some("SSH"),
            23 => Some("Telnet"),
            25 => Some("SMTP"),
            53 => Some("DNS"),
            67 => Some("DHCP Server"),
            68 => Some("DHCP Client"),
            69 => Some("TFTP"),
            80 => Some("HTTP"),
            110 => Some("POP3"),
            123 => Some("NTP"),
            143 => Some("IMAP"),
            161 => Some("SNMP"),
            194 => Some("IRC"),
            443 => Some("HTTPS"),
            445 => Some("SMB"),
            465 => Some("SMTPS"),
            514 => Some("Syslog"),
            587 => Some("SMTP (Submission)"),
            993 => Some("IMAPS"),
            995 => Some("POP3S"),
            1433 => Some("MS SQL Server"),
            1521 => Some("Oracle DB"),
            3306 => Some("MySQL"),
            3389 => Some("RDP"),
            5432 => Some("PostgreSQL"),
            5900 => Some("VNC"),
            6379 => Some("Redis"),
            8000 => Some("HTTP Alt"),
            8080 => Some("HTTP Proxy"),
            8443 => Some("HTTPS Alt"),
            9200 => Some("Elasticsearch"),
            27017 => Some("MongoDB"),
            _ => None,
        }
    }

    pub fn get_service_description(port: u16) -> String {
        match Self::get_service_name(port) {
            Some(name) => format!("{} ({})", port, name),
            None => port.to_string(),
        }
    }
}
