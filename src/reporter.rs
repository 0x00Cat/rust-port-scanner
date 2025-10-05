use std::io::{self, Write};
use crate::port_info::{PortScanResult, PortStatus, ServiceDatabase};

/// Handles reporting and displaying scan results
pub struct Reporter;

impl Reporter {
    /// Print progress for a scanned port
    pub fn print_progress(result: &PortScanResult, verbose: bool) {
        if verbose {
            let status_str = match &result.status {
                PortStatus::Open => "OPEN",
                PortStatus::Closed => "closed",
                PortStatus::Filtered => "filtered",
                PortStatus::Error(_) => "error",
            };
            
            print!("\rScanning port {}: {}", result.port, status_str);
            io::stdout().flush().unwrap();
        } else if result.is_open() {
            println!("Found open port: {}", result.port);
        }
    }

    /// Display summary of open ports
    pub fn display_open_ports(results: &[PortScanResult]) {
        println!("\n\n=== OPEN PORTS ===");
        
        let open_ports: Vec<_> = results
            .iter()
            .filter(|r| r.is_open())
            .collect();
        
        if open_ports.is_empty() {
            println!("No open ports found.");
            return;
        }
        
        for result in open_ports {
            let service = match ServiceDatabase::get_service_name(result.port) {
                Some(name) => format!(" - {}", name),
                None => String::new(),
            };
            
            let version_info = match &result.service_version {
                Some(v) => format!(" [{}]", v.display_string()),
                None => String::new(),
            };
            
            println!("  Port {}{}{}", result.port, service, version_info);
        }
    }

    /// Display detailed results with service names
    pub fn display_detailed_results(results: &[PortScanResult]) {
        println!("\n=== DETAILED RESULTS ===");
        
        let open_results: Vec<_> = results
            .iter()
            .filter(|r| r.is_open())
            .collect();
        
        if open_results.is_empty() {
            println!("No open ports to display.");
            return;
        }
        
        for result in open_results {
            let service = match ServiceDatabase::get_service_name(result.port) {
                Some(name) => format!(" ({})", name),
                None => String::new(),
            };
            
            print!("Port {}{}: {:?}", result.port, service, result.status);
            
            // Display version information if available
            if let Some(version) = &result.service_version {
                println!();
                println!("  └─ Detected: {}", version.display_string());
                if let Some(banner) = &version.banner {
                    // Truncate banner if too long
                    let display_banner = if banner.len() > 80 {
                        format!("{}...", &banner[..80])
                    } else {
                        banner.clone()
                    };
                    println!("  └─ Banner: {}", display_banner);
                }
            } else {
                println!();
            }

            // Display OS information if available (from SMB)
            if let Some(os_info) = &result.os_info {
                println!("  └─ OS Detected: {}", os_info.display_string());
                if let Some(smb_ver) = &os_info.smb_version {
                    println!("  └─ SMB Version: {}", smb_ver);
                }
                if let Some(domain) = &os_info.domain {
                    println!("  └─ Domain: {}", domain);
                }
            }
        }
    }

    /// Display summary statistics
    pub fn display_statistics(results: &[PortScanResult]) {
        let total = results.len();
        let open = results.iter().filter(|r| matches!(r.status, PortStatus::Open)).count();
        let closed = results.iter().filter(|r| matches!(r.status, PortStatus::Closed)).count();
        let filtered = results.iter().filter(|r| matches!(r.status, PortStatus::Filtered)).count();
        let errors = results.iter().filter(|r| matches!(r.status, PortStatus::Error(_))).count();
        
        println!("\n=== STATISTICS ===");
        println!("Total ports scanned: {}", total);
        println!("Open:     {} ({:.1}%)", open, (open as f64 / total as f64) * 100.0);
        println!("Closed:   {} ({:.1}%)", closed, (closed as f64 / total as f64) * 100.0);
        println!("Filtered: {} ({:.1}%)", filtered, (filtered as f64 / total as f64) * 100.0);
        
        if errors > 0 {
            println!("Errors:   {}", errors);
        }
    }

    /// Display a full report with all sections
    pub fn display_full_report(results: &[PortScanResult]) {
        Self::display_open_ports(results);
        Self::display_detailed_results(results);
        Self::display_os_summary(results);
        Self::display_statistics(results);
    }

    /// Display OS detection summary
    fn display_os_summary(results: &[PortScanResult]) {
        let os_detected: Vec<_> = results
            .iter()
            .filter_map(|r| r.os_info.as_ref())
            .collect();

        if !os_detected.is_empty() {
            println!("\n=== OS DETECTION SUMMARY ===");
            for os_info in os_detected {
                println!("{}", os_info.display_string());
                if let Some(smb) = &os_info.smb_version {
                    println!("  SMB: {}", smb);
                }
            }
        }
    }
}
