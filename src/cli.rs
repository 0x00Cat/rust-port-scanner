use std::io::{self, Write};
use std::net::IpAddr;
use std::time::Duration;

use crate::scanner::{ScanConfig, ScanMode};

/// Output format for scan results
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    Text,
    Json,
}

/// Command-line interface handler
pub struct CliInterface;

impl CliInterface {
    /// Display the application banner
    pub fn display_banner() {
        println!("╔════════════════════════════════════╗");
        println!("║   Rust Port Scanner v1.0          ║");
        println!("║   A modular network scanner       ║");
        println!("╚════════════════════════════════════╝");
        println!();
    }

    /// Read a line of input from the user
    fn read_input(prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input.trim().to_string()
    }

    /// Get IP address from user
    pub fn get_target_ip() -> Result<IpAddr, String> {
        let ip_input = Self::read_input("Enter target IP address (e.g., 127.0.0.1): ");
        
        ip_input.parse::<IpAddr>()
            .map_err(|_| "Invalid IP address format".to_string())
    }

    /// Get scan mode from user
    pub fn get_scan_mode() -> ScanMode {
        println!("\nScan Mode Options:");
        println!("  1. Scan common ports (fastest, ~26 ports)");
        println!("  2. Scan port range (custom range)");
        println!("  3. Scan specific ports (comma-separated list)");
        
        let mode_input = Self::read_input("Choose scan mode (1/2/3, default 1): ");
        
        match mode_input.as_str() {
            "2" => {
                let start_input = Self::read_input("Enter start port (default 1): ");
                let start_port = start_input.parse().unwrap_or(1);
                
                let end_input = Self::read_input("Enter end port (default 1000): ");
                let end_port = end_input.parse().unwrap_or(1000);
                
                ScanMode::Range { start: start_port, end: end_port }
            }
            "3" => {
                let ports_input = Self::read_input("Enter ports (comma-separated, e.g., 80,443,8080): ");
                let ports: Vec<u16> = ports_input
                    .split(',')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();
                
                if ports.is_empty() {
                    println!("No valid ports provided, using common ports instead.");
                    ScanMode::CommonPorts
                } else {
                    ScanMode::CustomList(ports)
                }
            }
            _ => ScanMode::CommonPorts,
        }
    }

    /// Get timeout from user
    pub fn get_timeout() -> Duration {
        let timeout_input = Self::read_input("Enter timeout in milliseconds (default 500): ");
        let timeout_ms: u64 = timeout_input.parse().unwrap_or(500);
        Duration::from_millis(timeout_ms)
    }

    /// Get verbose mode preference
    pub fn get_verbose_mode() -> bool {
        let verbose_input = Self::read_input("Enable verbose output? (y/n, default n): ");
        matches!(verbose_input.to_lowercase().as_str(), "y" | "yes")
    }

    /// Get version detection preference
    pub fn get_version_detection() -> bool {
        let version_input = Self::read_input("Enable service version detection? (y/n, default y): ");
        !matches!(version_input.to_lowercase().as_str(), "n" | "no")
    }

    /// Get OS detection preference
    pub fn get_os_detection() -> bool {
        let os_input = Self::read_input("Enable OS detection via SMB? (y/n, default y): ");
        !matches!(os_input.to_lowercase().as_str(), "n" | "no")
    }

    /// Get parallel scanning preference
    pub fn get_parallel_mode() -> (bool, usize) {
        let parallel_input = Self::read_input("Enable parallel scanning? (y/n, default y): ");
        let parallel = !matches!(parallel_input.to_lowercase().as_str(), "n" | "no");
        
        if parallel {
            let threads_input = Self::read_input("Number of threads (default auto): ");
            let thread_count = if threads_input.is_empty() {
                0 // 0 means auto-detect
            } else {
                threads_input.parse().unwrap_or(0)
            };
            (true, thread_count)
        } else {
            (false, 1)
        }
    }

    /// Get stealth options
    pub fn get_stealth_options() -> (bool, Option<Duration>) {
        println!("\n=== STEALTH OPTIONS ===");
        
        let randomize_input = Self::read_input("Randomize source ports? (y/n, default n): ");
        let randomize_source = matches!(randomize_input.to_lowercase().as_str(), "y" | "yes");
        
        let delay_input = Self::read_input("Delay between probes in ms (0 for none, default 0): ");
        let delay_ms: u64 = delay_input.parse().unwrap_or(0);
        let delay = if delay_ms > 0 {
            Some(Duration::from_millis(delay_ms))
        } else {
            None
        };
        
        (randomize_source, delay)
    }

    /// Build scan configuration interactively
    pub fn build_scan_config() -> Result<ScanConfig, String> {
        Self::display_banner();
        
        let ip = Self::get_target_ip()?;
        let scan_mode = Self::get_scan_mode();
        let timeout = Self::get_timeout();
        let verbose = Self::get_verbose_mode();
        let detect_versions = Self::get_version_detection();
        let detect_os = Self::get_os_detection();
        let (parallel, thread_count) = Self::get_parallel_mode();
        let (randomize_source, delay) = Self::get_stealth_options();
        
        let config = match scan_mode {
            ScanMode::Range { start, end } => {
                ScanConfig::new(ip, start, end)
            }
            ScanMode::CommonPorts => {
                ScanConfig::new_common_ports(ip)
            }
            ScanMode::CustomList(ports) => {
                ScanConfig::new_custom_ports(ip, ports)
            }
        };
        
        let mut config = config
            .with_timeout(timeout)
            .with_verbose(verbose)
            .with_version_detection(detect_versions)
            .with_os_detection(detect_os)
            .with_parallel(parallel)
            .with_source_port_randomization(randomize_source)
            .with_delay_between_probes(delay);

        if thread_count > 0 {
            config = config.with_thread_count(thread_count);
        }
        
        config.validate()?;
        
        Ok(config)
    }

    /// Get output format from user
    pub fn get_output_format() -> OutputFormat {
        println!("\nOutput Format:");
        println!("  1. Text (human-readable)");
        println!("  2. JSON (machine-readable)");
        
        let format_input = Self::read_input("Choose output format (1/2, default 1): ");
        
        match format_input.as_str() {
            "2" => OutputFormat::Json,
            _ => OutputFormat::Text,
        }
    }

    /// Get JSON output file path from user
    pub fn get_json_output_path(target_ip: &str) -> Option<String> {
        let default_filename = crate::json_output::ScanReport::default_filename(target_ip);
        
        println!("\nJSON Output File:");
        let path_input = Self::read_input(&format!("Enter file path (default '{}'): ", default_filename));
        
        if path_input.is_empty() {
            Some(default_filename)
        } else {
            Some(path_input)
        }
    }

    /// Display scan configuration before starting
    pub fn display_scan_info(config: &ScanConfig) {
        println!("\n╔════════════════════════════════════╗");
        println!("║        SCAN CONFIGURATION         ║");
        println!("╚════════════════════════════════════╝");
        println!("Target:   {}", config.target_ip);
        
        match &config.scan_mode {
            ScanMode::Range { start, end } => {
                println!("Mode:     Port Range");
                println!("Ports:    {} - {}", start, end);
            }
            ScanMode::CommonPorts => {
                println!("Mode:     Common Ports");
                println!("Ports:    Most common service ports");
            }
            ScanMode::CustomList(ports) => {
                println!("Mode:     Custom List");
                if ports.len() <= 10 {
                    println!("Ports:    {:?}", ports);
                } else {
                    println!("Ports:    {} custom ports", ports.len());
                }
            }
        }
        
        println!("Count:    {} ports", config.port_count());
        println!("Timeout:  {:?}", config.timeout);
        println!("Verbose:  {}", if config.verbose { "Yes" } else { "No" });
        println!("Version Detection: {}", if config.detect_versions { "Enabled" } else { "Disabled" });
        println!("OS Detection (SMB): {}", if config.detect_os { "Enabled" } else { "Disabled" });
        println!("Parallel Scanning: {}", if config.parallel { 
            format!("Enabled ({} threads)", config.thread_count) 
        } else { 
            "Disabled".to_string() 
        });
        println!("Source Port Randomization: {}", if config.randomize_source_port { "Enabled" } else { "Disabled" });
        if let Some(delay) = config.delay_between_probes {
            println!("Delay Between Probes: {:?}", delay);
        }
        println!("\nStarting scan...\n");
    }
}
