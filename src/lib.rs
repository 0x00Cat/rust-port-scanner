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

// Re-exports for convenience
pub use errors::{ScanError, ConfigError, DetectionError, FormatterError};
pub use domain::{Port, PortStatus, PortScanResult, ScanResults, ServiceInfo, ServiceVersion, OSInfo};
pub use scanning::{ScanConfig, ScanConfigBuilder, ScanMode};
pub use application::{PortScanner, VersionDetector, SMBFingerprinter};
pub use presentation::{OutputFormat, OutputFormatter, OutputFormatterFactory, ScanReport};

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::domain::{Port, PortStatus, PortScanResult, ScanResults, ServiceVersion, OSInfo};
    pub use crate::scanning::{ScanConfig, ScanConfigBuilder, ScanMode};
    pub use crate::application::{PortScanner, VersionDetector, SMBFingerprinter};
    pub use crate::presentation::{OutputFormat, OutputFormatterFactory, ScanReport};
    pub use crate::errors::{ScanError, ConfigError};
    pub use anyhow;
}
