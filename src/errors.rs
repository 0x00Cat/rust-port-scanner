/// Custom error types for the port scanner

use thiserror::Error;
use std::io;
use std::time::Duration;

/// Main error type for scanning operations
#[derive(Error, Debug)]
pub enum ScanError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    
    #[error("Network error: {0}")]
    Network(#[from] io::Error),
    
    #[error("Timeout after {0:?}")]
    Timeout(Duration),
    
    #[error("Permission denied for port {0}")]
    PermissionDenied(u16),
    
    #[error("Invalid port number: {0}")]
    InvalidPort(u16),
    
    #[error("Invalid port range: {start}-{end}")]
    InvalidRange { start: u16, end: u16 },
    
    #[error("No ports to scan")]
    NoPorts,
    
    #[error("Thread pool error: {0}")]
    ThreadPool(String),
    
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
}

/// Configuration errors
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing required field: {0}")]
    MissingField(String),
    
    #[error("Invalid timeout: {0:?}")]
    InvalidTimeout(Duration),
    
    #[error("Invalid thread count: {0}")]
    InvalidThreadCount(usize),
    
    #[error("Invalid scan mode")]
    InvalidScanMode,
}

/// Detection errors
#[derive(Error, Debug)]
pub enum DetectionError {
    #[error("Failed to detect version: {0}")]
    VersionDetection(String),
    
    #[error("Failed to detect OS: {0}")]
    OsDetection(String),
    
    #[error("Protocol error: {0}")]
    Protocol(String),
    
    #[error("Parse error: {0}")]
    Parse(String),
}

/// Output formatting errors
#[derive(Error, Debug)]
pub enum FormatterError {
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("Unsupported format")]
    UnsupportedFormat,
}

/// Result type alias for scan operations
pub type ScanResult<T> = Result<T, ScanError>;

/// Result type alias for configuration operations
pub type ConfigResult<T> = Result<T, ConfigError>;

/// Result type alias for detection operations
pub type DetectionResult<T> = Result<T, DetectionError>;

/// Result type alias for formatting operations
pub type FormatterResult<T> = Result<T, FormatterError>;
