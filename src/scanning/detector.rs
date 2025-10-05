/// Detector plugin architecture

use std::net::SocketAddr;
use std::time::Duration;

use crate::domain::{Port, ServiceVersion, OSInfo};

/// Trait for detection plugins
pub trait Detector: Send + Sync {
    /// Name of the detector
    fn name(&self) -> &str;
    
    /// Check if this detector can run on the given port
    fn can_detect(&self, port: Port) -> bool;
    
    /// Perform detection on the given socket
    fn detect_service(&self, socket: &SocketAddr, timeout: Duration) -> Option<ServiceVersion>;
    
    /// Perform OS detection (if supported)
    fn detect_os(&self, socket: &SocketAddr, timeout: Duration) -> Option<OSInfo> {
        None // Most detectors don't do OS detection
    }
}

/// Registry for managing detectors
pub struct DetectorRegistry {
    detectors: Vec<Box<dyn Detector>>,
}

impl DetectorRegistry {
    pub fn new() -> Self {
        Self {
            detectors: Vec::new(),
        }
    }

    pub fn register(&mut self, detector: Box<dyn Detector>) {
        self.detectors.push(detector);
    }

    pub fn detect_service(&self, port: Port, socket: &SocketAddr, timeout: Duration) -> Option<ServiceVersion> {
        for detector in &self.detectors {
            if detector.can_detect(port) {
                if let Some(version) = detector.detect_service(socket, timeout) {
                    return Some(version);
                }
            }
        }
        None
    }

    pub fn detect_os(&self, port: Port, socket: &SocketAddr, timeout: Duration) -> Option<OSInfo> {
        for detector in &self.detectors {
            if detector.can_detect(port) {
                if let Some(os_info) = detector.detect_os(socket, timeout) {
                    return Some(os_info);
                }
            }
        }
        None
    }

    pub fn detector_count(&self) -> usize {
        self.detectors.len()
    }
}

impl Default for DetectorRegistry {
    fn default() -> Self {
        Self::new()
    }
}
