/// Domain model for operating system detection

use serde::Serialize;

/// Operating system information detected from network fingerprinting
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

    pub fn with_os_name(mut self, name: impl Into<String>) -> Self {
        self.os_name = Some(name.into());
        self
    }

    pub fn with_os_version(mut self, version: impl Into<String>) -> Self {
        self.os_version = Some(version.into());
        self
    }

    pub fn with_os_build(mut self, build: impl Into<String>) -> Self {
        self.os_build = Some(build.into());
        self
    }

    pub fn with_computer_name(mut self, name: impl Into<String>) -> Self {
        self.computer_name = Some(name.into());
        self
    }

    pub fn with_domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }

    pub fn with_smb_version(mut self, version: impl Into<String>) -> Self {
        self.smb_version = Some(version.into());
        self
    }

    pub fn is_detected(&self) -> bool {
        self.os_name.is_some() 
            || self.os_version.is_some() 
            || self.computer_name.is_some()
            || self.smb_version.is_some()
    }

    pub fn summary(&self) -> String {
        let mut parts = Vec::new();
        
        if let Some(name) = &self.os_name {
            parts.push(name.clone());
        }
        
        if let Some(version) = &self.os_version {
            parts.push(version.clone());
        }
        
        if let Some(build) = &self.os_build {
            parts.push(format!("(Build {})", build));
        }
        
        if parts.is_empty() {
            "Unknown OS".to_string()
        } else {
            parts.join(" ")
        }
    }
}

impl Default for OSInfo {
    fn default() -> Self {
        Self::new()
    }
}
