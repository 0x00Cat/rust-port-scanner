/// Modernized main entry point using new architecture

use port_scanner::prelude::*;
use port_scanner::presentation::{OutputFormatter, JsonFormatter};
use std::time::Instant;
use std::io::{self, Write};
use std::path::Path;
use tracing::{info, Level};
use tracing_subscriber;

fn main() -> anyhow::Result<()> {
    // Initialize tracing with DEBUG level for verbose output
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_target(false)
        .init();

    info!("Port Scanner v2.0 - Refactored Architecture");

    // Display banner
    println!("╔════════════════════════════════════╗");
    println!("║   Rust Port Scanner v2.0          ║");
    println!("║   Clean Architecture              ║");
    println!("╚════════════════════════════════════╝\n");

    // Build config using new architecture
    let config = build_config_interactive()?;

    // Display scan info
    display_scan_info(&config);

    // Create scanner
    let scanner = PortScanner::new(config.clone())?;

    // Create progress observer for display
    let verbose = config.verbose;
    
    // Start timing
    let start_time = Instant::now();
    
    if verbose {
        println!("\nStarting scan of {} ports...", config.port_count());
    }

    // Perform scan with simple progress callback
    let results = scanner.scan_all(move |result| {
        if verbose && result.status.is_open() {
            println!("Found open port: {}", result.port);
        }
    });

    // Calculate duration
    let duration = start_time.elapsed();
    let duration_seconds = duration.as_secs_f64();

    // Get metrics from results
    let total_ports = results.total_ports;
    let open_ports = results.open_ports;
    let closed_ports = results.closed_ports;

    // Ask for output format
    print!("\nSave results to JSON file? (y/n): ");
    io::stdout().flush()?;
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line)?;
    let save_json = line.trim().to_lowercase() == "y";

    if save_json {
        let report = ScanReport::new(&config, results.clone(), duration_seconds);
        let filename = ScanReport::default_filename(&config.target_ip.to_string(), OutputFormat::Json);
        let path = Path::new(&filename);
        
        match JsonFormatter.write_to_file(&report, path) {
            Ok(_) => {
                println!("\n✓ JSON report saved to: {}", filename);
            }
            Err(e) => {
                eprintln!("\n✗ Failed to save JSON file: {}", e);
            }
        }
    }

    // Output results to console
    display_text_results(&results, duration, total_ports, open_ports, closed_ports);

    Ok(())
}

/// Build scan configuration interactively
fn build_config_interactive() -> anyhow::Result<ScanConfig> {
    use std::io::{self, BufRead};
    
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Get target IP
    print!("Enter target IP address (e.g., 127.0.0.1): ");
    io::stdout().flush()?;
    let target_input = lines.next()
        .ok_or_else(|| anyhow::anyhow!("No input provided"))??;
    let target_ip = target_input.trim().parse()
        .map_err(|e| anyhow::anyhow!("Invalid IP address format: {}", e))?;

    // Get scan mode
    println!("\nScan modes:");
    println!("  1. Common ports (21, 22, 23, 25, 53, 80, 110, 143, 443, 445, 3306, 3389, 8080)");
    println!("  2. Port range");
    println!("  3. Custom port list");
    print!("Select scan mode (1-3): ");
    io::stdout().flush()?;
    let mode_choice = lines.next()
        .ok_or_else(|| anyhow::anyhow!("No input provided"))??;

    let mode = match mode_choice.trim() {
        "1" => ScanMode::CommonPorts,
        "2" => {
            print!("Enter port range (e.g., 1-1000): ");
            io::stdout().flush()?;
            let range_input = lines.next()
                .ok_or_else(|| anyhow::anyhow!("No input provided"))??;
            let parts: Vec<&str> = range_input.trim().split('-').collect();
            if parts.len() != 2 {
                return Err(anyhow::anyhow!("Invalid range format"));
            }
            let start = parts[0].parse()?;
            let end = parts[1].parse()?;
            ScanMode::Range { start, end }
        }
        "3" => {
            print!("Enter ports (comma-separated, e.g., 80,443,8080): ");
            io::stdout().flush()?;
            let ports_input = lines.next()
                .ok_or_else(|| anyhow::anyhow!("No input provided"))??;
            let ports: Result<Vec<u16>, _> = ports_input
                .trim()
                .split(',')
                .map(|p| p.trim().parse())
                .collect();
            ScanMode::CustomList(ports?)
        }
        _ => return Err(anyhow::anyhow!("Invalid selection")),
    };

    // Ask for service version detection
    print!("\nEnable service version detection? (y/n): ");
    io::stdout().flush()?;
    let detect_versions_input = lines.next()
        .ok_or_else(|| anyhow::anyhow!("No input provided"))??;
    let detect_versions = detect_versions_input.trim().to_lowercase() == "y";

    // Ask for OS detection
    print!("Enable OS detection (SMB only)? (y/n): ");
    io::stdout().flush()?;
    let detect_os_input = lines.next()
        .ok_or_else(|| anyhow::anyhow!("No input provided"))??;
    let detect_os = detect_os_input.trim().to_lowercase() == "y";

    // Build config
    Ok(ScanConfigBuilder::new()
        .target(target_ip)
        .scan_mode(mode)
        .timeout(std::time::Duration::from_millis(500))
        .verbose(true)
        .detect_versions(detect_versions)
        .detect_os(detect_os)
        .parallel(true)
        .build()?)
}

/// Display scan configuration info
fn display_scan_info(config: &ScanConfig) {
    println!("\n=== SCAN CONFIGURATION ===");
    println!("Target:   {}", config.target_ip);
    match &config.scan_mode {
        ScanMode::Range { start, end } => {
            println!("Mode:     Port Range");
            println!("Range:    {}-{}", start, end);
        }
        ScanMode::CommonPorts => {
            println!("Mode:     Common Ports");
        }
        ScanMode::CustomList(ports) => {
            println!("Mode:     Custom List ({} ports)", ports.len());
        }
    }
    println!("Count:    {} ports", config.port_count());
    println!("Timeout:  {:?}", config.timeout);
    println!("Parallel: {}", if config.parallel {
        format!("Yes ({} threads)", config.thread_count)
    } else {
        "No".to_string()
    });
    println!("Stealth:  {}", if config.is_stealth_enabled() { "Enabled" } else { "Disabled" });
    println!("\nStarting scan...\n");
}

/// Display text results
fn display_text_results(
    results: &ScanResults,
    duration: std::time::Duration,
    total_ports: usize,
    open_ports: usize,
    closed_ports: usize
) {
    println!("\n=== SCAN RESULTS ===");
    println!("Total Ports: {}", total_ports);
    println!("Open:        {}", open_ports);
    println!("Closed:      {}", closed_ports);
    println!("Filtered:    {}", results.filtered_ports);
    println!("Errors:      {}", results.error_ports);
    
    // Display open ports with details
    if open_ports > 0 {
        println!("\n=== OPEN PORTS ===");
        for result in &results.results {
            if result.status.is_open() {
                print!("Port {}: ", result.port);
                
                // Display service version if available
                if let Some(ref version) = result.service_version {
                    print!("{}", version.service_name);
                    if let Some(ref ver) = version.version {
                        print!(" {}", ver);
                    }
                    println!();
                    if let Some(ref banner) = version.banner {
                        if !banner.is_empty() {
                            println!("  Banner: {}", banner);
                        }
                    }
                } else {
                    println!("Open");
                }
                
                // Display OS info if available
                if let Some(ref os_info) = result.os_info {
                    println!("  OS: {}", os_info.summary());
                }
            }
        }
    }
    
    println!("\n=== PERFORMANCE ===");
    println!("Duration:    {:.2?}", duration);
    if duration.as_secs_f64() > 0.0 {
        let ports_per_sec = total_ports as f64 / duration.as_secs_f64();
        println!("Speed:       {:.2} ports/sec", ports_per_sec);
    }
}