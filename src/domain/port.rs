/// Domain model for ports and port status

use serde::Serialize;

/// Type alias for port numbers
pub type Port = u16;

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

impl std::fmt::Display for PortStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PortStatus::Open => write!(f, "OPEN"),
            PortStatus::Closed => write!(f, "CLOSED"),
            PortStatus::Filtered => write!(f, "FILTERED"),
            PortStatus::Error(e) => write!(f, "ERROR: {}", e),
        }
    }
}
