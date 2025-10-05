/// Modernized main entry point using new architecture

use port_scanner::prelude::*;
use port_scanner::presentation::OutputFormatter;
use port_scanner::cli::CliInterface; // Legacy CLI
use std::time::Instant;
use tracing::{info, Level};
use tracing_subscriber;

fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .init();

    info!("Port Scanner v2.0 - Refactored Architecture");

    // Use legacy CLI for now (can be refactored later)
    let legacy_config = match CliInterface::build_scan_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            return Ok(());
        }
    };

    // Convert legacy config to new config
    let config = convert_legacy_config(legacy_config)?;

    // Get output format (legacy way for now)
    let output_format = CliInterface::get_output_format();
    let json_path = if output_format == port_scanner::cli::OutputFormat::Json {
        CliInterface::get_json_output_path(&config.target_ip.to_string())
    } else {
        None
    };

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

    // Output results based on format
    match output_format {
        port_scanner::cli::OutputFormat::Text => {
            display_text_results(&results, duration, total_ports, open_ports, closed_ports);
        }
        port_scanner::cli::OutputFormat::Json => {
            let report = ScanReport::new(&config, results, duration_seconds);
            
            if let Some(path) = json_path {
                let formatter = port_scanner::presentation::JsonFormatter;
                match formatter.write_to_file(&report, std::path::Path::new(&path)) {
                    Ok(_) => {
                        println!("\n✓ JSON report written to: {}", path);
                        println!("Scan completed in {:.2?}", duration);
                    }
                    Err(e) => {
                        eprintln!("\n✗ Failed to write JSON file: {}", e);
                        if let Ok(json) = formatter.format(&report) {
                            println!("\nJSON output:\n{}", json);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

/// Convert legacy config to new config
fn convert_legacy_config(legacy: port_scanner::scanner::ScanConfig) -> anyhow::Result<ScanConfig> {
    let mode = match legacy.scan_mode {
        port_scanner::scanner::ScanMode::Range { start, end } => {
            ScanMode::Range { start, end }
        }
        port_scanner::scanner::ScanMode::CommonPorts => {
            ScanMode::CommonPorts
        }
        port_scanner::scanner::ScanMode::CustomList(ports) => {
            ScanMode::CustomList(ports)
        }
    };

    Ok(ScanConfigBuilder::new()
        .target(legacy.target_ip)
        .scan_mode(mode)
        .timeout(legacy.timeout)
        .verbose(legacy.verbose)
        .detect_versions(legacy.detect_versions)
        .detect_os(legacy.detect_os)
        .parallel(legacy.parallel)
        .thread_count(legacy.thread_count)
        .randomize_source_port(legacy.randomize_source_port)
        .delay_between_probes(legacy.delay_between_probes)
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
    println!("\n=== PERFORMANCE ===");
    println!("Duration:    {:.2?}", duration);
    if duration.as_secs_f64() > 0.0 {
        let ports_per_sec = total_ports as f64 / duration.as_secs_f64();
        println!("Speed:       {:.2} ports/sec", ports_per_sec);
    }
}
