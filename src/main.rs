mod scanner;
mod port_info;
mod cli;
mod reporter;
mod version_detector;
mod smb_fingerprint;
mod json_output;

use scanner::PortScanner;
use cli::{CliInterface, OutputFormat};
use reporter::Reporter;
use json_output::ScanReport;
use std::time::Instant;

fn main() {
    // Get scan configuration from user
    let config = match CliInterface::build_scan_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            return;
        }
    };

    // Get output format
    let output_format = CliInterface::get_output_format();
    
    // Get JSON file path if needed
    let json_path = if output_format == OutputFormat::Json {
        CliInterface::get_json_output_path(&config.target_ip.to_string())
    } else {
        None
    };

    // Display scan information
    CliInterface::display_scan_info(&config);

    // Create scanner
    let scanner = match PortScanner::new(config.clone()) {
        Ok(scanner) => scanner,
        Err(e) => {
            eprintln!("Scanner initialization error: {}", e);
            return;
        }
    };

    // Start timing
    let start_time = Instant::now();

    // Perform the scan with progress callback
    let verbose = scanner.config().verbose;
    let show_progress = output_format == OutputFormat::Text;
    let results = scanner.scan_all(move |result| {
        if show_progress {
            Reporter::print_progress(result, verbose);
        }
    });

    // Calculate duration
    let duration = start_time.elapsed();
    let duration_seconds = duration.as_secs_f64();

    // Output results based on format
    match output_format {
        OutputFormat::Text => {
            // Display results in human-readable format
            Reporter::display_full_report(&results);
            
            // Display scan duration
            println!("\n=== SCAN DURATION ===");
            println!("Time elapsed: {:.2?}", duration);
            if duration_seconds > 0.0 {
                println!("Ports per second: {:.2}", results.len() as f64 / duration_seconds);
            }
        }
        OutputFormat::Json => {
            // Create JSON report
            let report = ScanReport::new(&config, results, duration_seconds);
            
            // Write to file if path was specified
            if let Some(path) = json_path {
                match report.write_to_file(&path) {
                    Ok(_) => {
                        println!("\n✓ JSON report written to: {}", path);
                        println!("Scan completed in {:.2?}", duration);
                    }
                    Err(e) => {
                        eprintln!("\n✗ Failed to write JSON file: {}", e);
                        eprintln!("Outputting JSON to console instead:\n");
                        if let Ok(json) = report.to_json_string() {
                            println!("{}", json);
                        }
                    }
                }
            } else {
                // Output to console
                match report.to_json_string() {
                    Ok(json) => println!("{}", json),
                    Err(e) => eprintln!("Failed to serialize JSON: {}", e),
                }
            }
        }
    }
}
