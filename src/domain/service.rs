/// Domain model for services and service detection

use serde::Serialize;
use std::collections::HashMap;

/// Service information detected from a port
#[derive(Debug, Clone, Serialize)]
pub struct ServiceInfo {
    pub name: Option<String>,
    pub version: Option<String>,
    pub product: Option<String>,
    pub extra_info: Option<String>,
}

impl ServiceInfo {
    pub fn new() -> Self {
        Self {
            name: None,
            version: None,
            product: None,
            extra_info: None,
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn with_product(mut self, product: impl Into<String>) -> Self {
        self.product = Some(product.into());
        self
    }

    pub fn with_extra_info(mut self, info: impl Into<String>) -> Self {
        self.extra_info = Some(info.into());
        self
    }

    pub fn is_detected(&self) -> bool {
        self.name.is_some() || self.version.is_some() || self.product.is_some()
    }
}

/// Service version information (legacy compatibility)
#[derive(Debug, Clone, Serialize)]
pub struct ServiceVersion {
    pub service_name: String,
    pub version: Option<String>,
    pub banner: Option<String>,
    pub protocol: String,
}

impl ServiceVersion {
    pub fn unknown() -> Self {
        Self {
            service_name: "unknown".to_string(),
            version: None,
            banner: None,
            protocol: "tcp".to_string(),
        }
    }

    pub fn new(service: impl Into<String>, protocol: impl Into<String>) -> Self {
        Self {
            service_name: service.into(),
            version: None,
            banner: None,
            protocol: protocol.into(),
        }
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn with_banner(mut self, banner: impl Into<String>) -> Self {
        self.banner = Some(banner.into());
        self
    }
}

/// Repository trait for service information
pub trait ServiceRepository: Send + Sync {
    fn get_service_info(&self, port: u16) -> Option<ServiceInfo>;
    fn get_common_ports(&self) -> Vec<u16>;
    fn get_service_name(&self, port: u16) -> Option<&str>;
}

/// Static service database
pub struct StaticServiceRepository {
    services: HashMap<u16, &'static str>,
}

impl StaticServiceRepository {
    pub fn new() -> Self {
        let mut services = HashMap::new();
        
        // Common ports mapping
        services.insert(21, "FTP");
        services.insert(22, "SSH");
        services.insert(23, "Telnet");
        services.insert(25, "SMTP");
        services.insert(53, "DNS");
        services.insert(80, "HTTP");
        services.insert(110, "POP3");
        services.insert(143, "IMAP");
        services.insert(443, "HTTPS");
        services.insert(445, "SMB");
        services.insert(3306, "MySQL");
        services.insert(3389, "RDP");
        services.insert(5432, "PostgreSQL");
        services.insert(5900, "VNC");
        services.insert(6379, "Redis");
        services.insert(8080, "HTTP-Proxy");
        services.insert(8443, "HTTPS-Alt");
        services.insert(27017, "MongoDB");
        
        Self { services }
    }
}

impl ServiceRepository for StaticServiceRepository {
    fn get_service_info(&self, port: u16) -> Option<ServiceInfo> {
        self.services.get(&port).map(|&name| {
            ServiceInfo::new().with_name(name)
        })
    }

    fn get_common_ports(&self) -> Vec<u16> {
        vec![
            21, 22, 23, 25, 53, 80, 110, 111, 135, 139, 143, 443, 445, 993, 995,
            1723, 3306, 3389, 5432, 5900, 6379, 8080, 8443, 8888, 9090, 27017
        ]
    }

    fn get_service_name(&self, port: u16) -> Option<&str> {
        self.services.get(&port).copied()
    }
}

impl Default for StaticServiceRepository {
    fn default() -> Self {
        Self::new()
    }
}
