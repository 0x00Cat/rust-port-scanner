/// Domain layer module exports

pub mod port;
pub mod service;
pub mod scan_result;
pub mod os;

pub use port::{Port, PortStatus};
pub use service::{ServiceInfo, ServiceVersion, ServiceRepository, StaticServiceRepository};
pub use scan_result::{PortScanResult, ScanResults};
pub use os::OSInfo;
