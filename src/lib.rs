/// Port Scanner Library
/// 
/// A modular and extensible port scanner with clean architecture.
/// 
/// # Architecture
/// 
/// The codebase follows a layered architecture:
/// - **Domain**: Core business logic and entities
/// - **Infrastructure**: External dependencies (network, IO)
/// - **Application**: Use cases and orchestration
/// - **Presentation**: User interfaces and output formatting
/// - **Scanning**: Scanning strategies and execution
/// 
/// # Example
/// 
/// ```no_run
/// use port_scanner::prelude::*;
/// use std::net::IpAddr;
/// 
/// # fn main() -> anyhow::Result<()> {
/// let config = ScanConfigBuilder::new()
///     .target("127.0.0.1".parse::<IpAddr>()?)
///     .common_ports()
///     .timeout(std::time::Duration::from_millis(500))
///     .parallel(true)
///     .build()?;
/// 
/// let scanner = PortScanner::new(config)?;
/// let results = scanner.scan_all(|result| {
///     println!("Port {}: {:?}", result.port, result.status);
/// });
/// 
/// println!("Found {} open ports", results.open_ports);
/// # Ok(())
/// # }
/// ```

// Core modules
pub mod constants;
pub mod errors;
pub mod domain;
pub mod infrastructure;
pub mod scanning;
pub mod application;
pub mod presentation;

// Legacy modules for backward compatibility
pub mod scanner;
pub mod port_info;
pub mod cli;
pub mod reporter;
pub mod version_detector;
pub mod smb_fingerprint;
pub mod json_output;

// Re-exports for convenience
pub use errors::{ScanError, ConfigError, DetectionError, FormatterError};
pub use domain::{Port, PortStatus, PortScanResult, ScanResults, ServiceInfo};
pub use domain::ServiceVersion as DomainServiceVersion;
pub use domain::OSInfo as DomainOSInfo;
pub use scanning::{ScanConfig, ScanConfigBuilder, ScanMode};
pub use application::PortScanner as NewPortScanner;
pub use application::VersionDetector as NewVersionDetector;
pub use application::SMBFingerprinter as NewSMBFingerprinter;
pub use presentation::{OutputFormat, OutputFormatter, OutputFormatterFactory, ScanReport};

// Legacy re-exports
pub use scanner::ScanConfig as LegacyScanConfig;
pub use port_info::ServiceDatabase;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::domain::{Port, PortStatus, PortScanResult, ScanResults};
    pub use crate::scanning::{ScanConfig, ScanConfigBuilder, ScanMode};
    pub use crate::application::PortScanner;
    pub use crate::presentation::{OutputFormat, OutputFormatterFactory, ScanReport};
    pub use crate::errors::{ScanError, ConfigError};
    pub use anyhow;
}

pub use cli::CliInterface;
pub use reporter::Reporter;
pub use version_detector::{VersionDetector, ServiceVersion};
pub use smb_fingerprint::{SMBFingerprinter, OSInfo};
