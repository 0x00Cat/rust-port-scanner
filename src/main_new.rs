/// Modernized main entry point using new architecture

use port_scanner::prelude::*;
use port_scanner::presentation::{
    OutputFormatter, OutputFormatterFactory, OutputFormat,
    JsonFormatter, TextFormatter, CsvFormatter,
    ProgressObserver, MetricsCollector, ScanObserver
};
use std::time::Instant;
use std::io::{self, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tracing::{info, debug, Level};
use tracing_subscriber;
use clap::{Parser, ValueEnum, ArgGroup};

/// A fast and modular port scanner written in Rust
#[derive(Parser, Debug)]
#[command(name = "port-scanner")]
#[command(author = "Your Name")]
#[command(version = "2.0.0")]
#[command(about = "A production-grade port scanner with service detection and OS fingerprinting", long_about = None)]
#[command(group(
    ArgGroup::new("port-spec")
        .required(false)
        .args(["ports", "common"])
))]
struct Cli {
    /// Target IP address to scan
    #[arg(short, long, value_name = "IP")]
    target: Option<String>,

    /// Ports to scan (e.g., "80,443,8080" or "1-1000")
    #[arg(short, long, value_name = "PORTS", group = "port-spec")]
    ports: Option<String>,

    /// Use common ports preset
    #[arg(short, long, group = "port-spec")]
    common: bool,

    /// Enable service version detection
    #[arg(short = 'v', long)]
    detect_versions: bool,

    /// Enable OS detection (SMB)
    #[arg(short = 'o', long)]
    detect_os: bool,

    /// Enable parallel scanning
    #[arg(long, default_value = "true")]
    parallel: bool,

    /// Number of parallel threads (default: auto-detect)
    #[arg(short = 'T', long, value_name = "NUM")]
    threads: Option<usize>,

    /// Connection timeout in milliseconds
    #[arg(long, default_value = "500", value_name = "MS")]
    timeout: u64,

    /// Randomize source port (stealth)
    #[arg(long)]
    randomize_port: bool,

    /// Delay between probes in milliseconds (stealth)
    #[arg(long, value_name = "MS")]
    delay: Option<u64>,

    /// Output format
    #[arg(short = 'f', long, value_enum)]
    format: Option<OutputFormatArg>,

    /// Output file path (auto-generated if not specified)
    #[arg(short = 'F', long, value_name = "PATH")]
    output_file: Option<String>,

    /// Enable verbose output
    #[arg(long)]
    verbose: bool,

    /// Disable interactive mode (use all defaults for unspecified options)
    #[arg(long)]
    non_interactive: bool,

    /// Show detailed info only for open ports (filtered/closed ports shown as summary)
    #[arg(long)]
    open_only: bool,

    /// Enable debug logging (shows detailed trace information)
    #[arg(short = 'd', long)]
    debug: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum OutputFormatArg {
    /// JSON format
    Json,
    /// CSV format  
    Csv,
    /// Text format
    Text,
    /// All formats
    All,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse CLI args early to get debug flag
    let cli = Cli::parse();
    
    // Initialize tracing based on debug flag
    let log_level = if cli.debug {
        Level::DEBUG
    } else {
        Level::WARN  // Only show warnings and errors by default
    };
    
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_target(false)
        .init();

    info!("Port Scanner v2.0 - Refactored Architecture");

    // Display banner
    println!("╔════════════════════════════════════╗");
    println!("║   Rust Port Scanner v2.0          ║");
    println!("║   Clean Architecture              ║");
    println!("╚════════════════════════════════════╝\n");

    // Store output preferences (cli already parsed above)
    let output_format = cli.format;
    let output_file = cli.output_file.clone();
    let open_only = cli.open_only;

    // Build config from CLI args or interactive mode
    let config = if cli.target.is_some() || cli.non_interactive {
        build_config_from_cli(cli)?
    } else {
        build_config_interactive()?
    };

    // Display scan info
    display_scan_info(&config);

    // Create scanner
    let scanner = PortScanner::new(config.clone())?;

    // Create observers wrapped in Arc<Mutex<>> for thread safety
    let progress_observer = Arc::new(Mutex::new(ProgressObserver::new(config.verbose)));
    let metrics_collector = Arc::new(Mutex::new(MetricsCollector::new()));
    
    // Clone Arc references for the closure
    let progress_obs_clone = Arc::clone(&progress_observer);
    let metrics_clone = Arc::clone(&metrics_collector);
    
    // Start timing
    let start_time = Instant::now();
    
    // Notify observers scan is starting
    progress_observer.lock().unwrap().on_scan_started(config.port_count());
    
    info!("Starting parallel scan with observers enabled");

    // Perform scan with observer callbacks
    let results = scanner.scan_all(move |result| {
        if let Ok(mut obs) = progress_obs_clone.lock() {
            obs.on_port_scanned(&result);
        }
        if let Ok(mut metrics) = metrics_clone.lock() {
            metrics.on_port_scanned(&result);
        }
    }).await;

    // Calculate duration
    let duration = start_time.elapsed();
    let duration_seconds = duration.as_secs_f64();

    // Notify observers of completion
    progress_observer.lock().unwrap().on_scan_completed(&results);
    
    // Display performance metrics
    let metrics = metrics_collector.lock().unwrap();
    println!("\n=== PERFORMANCE METRICS ===");
    println!("Total time: {:.2}s", metrics.elapsed().as_secs_f64());
    println!("Ports/second: {:.2}", metrics.ports_per_second());
    println!("Ports scanned: {}", metrics.ports_scanned);
    drop(metrics); // Release lock

    // Get metrics from results
    let total_ports = results.total_ports;
    let open_ports = results.open_ports;
    let closed_ports = results.closed_ports;

    // Create report for export
    let report = ScanReport::new(&config, results.clone(), duration_seconds);

    // Handle output based on CLI args or interactive prompt
    if let Some(fmt) = output_format {
        // CLI-specified format
        match fmt {
            OutputFormatArg::Json => save_report(&report, OutputFormat::Json, &config.target_ip.to_string(), output_file.as_deref(), open_only)?,
            OutputFormatArg::Csv => save_report(&report, OutputFormat::Csv, &config.target_ip.to_string(), output_file.as_deref(), open_only)?,
            OutputFormatArg::Text => save_report(&report, OutputFormat::Text, &config.target_ip.to_string(), output_file.as_deref(), open_only)?,
            OutputFormatArg::All => {
                save_report(&report, OutputFormat::Json, &config.target_ip.to_string(), None, open_only)?;
                save_report(&report, OutputFormat::Csv, &config.target_ip.to_string(), None, open_only)?;
                save_report(&report, OutputFormat::Text, &config.target_ip.to_string(), None, open_only)?;
            }
        }
    } else {
        // Interactive format selection
        println!("\n=== OUTPUT OPTIONS ===");
        println!("Export scan results to file:");
        println!("  1. JSON format");
        println!("  2. CSV format");
        println!("  3. Text format");
        println!("  4. All formats");
        println!("  0. Skip export");
        print!("Select option (0-4): ");
        io::stdout().flush()?;
        let stdin = io::stdin();
        let mut line = String::new();
        stdin.read_line(&mut line)?;
        
        let choice = line.trim();
        
        match choice {
            "1" => save_report(&report, OutputFormat::Json, &config.target_ip.to_string(), None, open_only)?,
            "2" => save_report(&report, OutputFormat::Csv, &config.target_ip.to_string(), None, open_only)?,
            "3" => save_report(&report, OutputFormat::Text, &config.target_ip.to_string(), None, open_only)?,
            "4" => {
                save_report(&report, OutputFormat::Json, &config.target_ip.to_string(), None, open_only)?;
                save_report(&report, OutputFormat::Csv, &config.target_ip.to_string(), None, open_only)?;
                save_report(&report, OutputFormat::Text, &config.target_ip.to_string(), None, open_only)?;
            }
            "0" => debug!("Skipping file export"),
            _ => println!("Invalid option, skipping export"),
        }
    }

    // Output results to console
    display_text_results(&results, duration, total_ports, open_ports, closed_ports);

    Ok(())
}

/// Build configuration from command-line arguments
fn build_config_from_cli(cli: Cli) -> anyhow::Result<ScanConfig> {
    // Parse target IP
    let target_ip = if let Some(target) = cli.target {
        target.parse()
            .map_err(|e| anyhow::anyhow!("Invalid IP address '{}': {}", target, e))?
    } else {
        return Err(anyhow::anyhow!("Target IP is required. Use --target or run without arguments for interactive mode."));
    };

    // Parse scan mode
    let scan_mode = if cli.common {
        ScanMode::CommonPorts
    } else if let Some(ports_str) = cli.ports {
        parse_ports_string(&ports_str)?
    } else {
        // Default to common ports if nothing specified
        ScanMode::CommonPorts
    };

    // Determine thread count
    let thread_count = cli.threads
        .unwrap_or_else(|| port_scanner::infrastructure::network_utils::num_cpus())
        .max(1)
        .min(256);

    // Build delay option
    let delay_between_probes = cli.delay.map(std::time::Duration::from_millis);

    // Build configuration
    Ok(ScanConfigBuilder::new()
        .target(target_ip)
        .scan_mode(scan_mode)
        .timeout(std::time::Duration::from_millis(cli.timeout))
        .verbose(cli.verbose)
        .detect_versions(cli.detect_versions)
        .detect_os(cli.detect_os)
        .parallel(cli.parallel)
        .thread_count(thread_count)
        .randomize_source_port(cli.randomize_port)
        .delay_between_probes(delay_between_probes)
        .build()?)
}

/// Parse ports string (e.g., "80,443,8080" or "1-1000")
fn parse_ports_string(s: &str) -> anyhow::Result<ScanMode> {
    if s.contains('-') {
        // Port range
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid port range format. Use: START-END (e.g., 1-1000)"));
        }
        let start: u16 = parts[0].trim().parse()
            .map_err(|_| anyhow::anyhow!("Invalid start port: {}", parts[0]))?;
        let end: u16 = parts[1].trim().parse()
            .map_err(|_| anyhow::anyhow!("Invalid end port: {}", parts[1]))?;
        Ok(ScanMode::Range { start, end })
    } else {
        // Custom port list
        let ports: Result<Vec<u16>, _> = s.split(',')
            .map(|p| p.trim().parse())
            .collect();
        Ok(ScanMode::CustomList(ports?))
    }
}

/// Save report in specified format
fn save_report(report: &ScanReport, format: OutputFormat, target_ip: &str, custom_path: Option<&str>, open_only: bool) -> anyhow::Result<()> {
    let filename = custom_path
        .map(|p| p.to_string())
        .unwrap_or_else(|| ScanReport::default_filename(target_ip, format));
    let path = Path::new(&filename);
    let formatter = OutputFormatterFactory::create(format, open_only);
    
    match formatter.write_to_file(report, path) {
        Ok(_) => {
            println!("✓ {:?} report saved to: {}", format, filename);
            Ok(())
        }
        Err(e) => {
            eprintln!("✗ Failed to save {:?} file: {}", format, e);
            Err(e.into())
        }
    }
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
    print!("\n=== DETECTION OPTIONS ===\n");
    print!("Enable service version detection? (y/n) [y]: ");
    io::stdout().flush()?;
    let detect_versions_input = lines.next()
        .ok_or_else(|| anyhow::anyhow!("No input provided"))??;
    let detect_versions = detect_versions_input.trim().to_lowercase() != "n";

    // Ask for OS detection
    print!("Enable OS detection (SMB only)? (y/n) [n]: ");
    io::stdout().flush()?;
    let detect_os_input = lines.next()
        .ok_or_else(|| anyhow::anyhow!("No input provided"))??;
    let detect_os = detect_os_input.trim().to_lowercase() == "y";

    // Ask for parallel scanning
    print!("\n=== PERFORMANCE OPTIONS ===\n");
    print!("Enable parallel scanning? (y/n) [y]: ");
    io::stdout().flush()?;
    let parallel_input = lines.next()
        .ok_or_else(|| anyhow::anyhow!("No input provided"))??;
    let parallel = parallel_input.trim().to_lowercase() != "n";

    // Ask for thread count if parallel
    let thread_count = if parallel {
        let default_threads = port_scanner::infrastructure::network_utils::num_cpus();
        print!("Number of threads (1-256) [{}]: ", default_threads);
        io::stdout().flush()?;
        let thread_input = lines.next()
            .ok_or_else(|| anyhow::anyhow!("No input provided"))??;
        if thread_input.trim().is_empty() {
            default_threads
        } else {
            thread_input.trim().parse::<usize>()?.min(256).max(1)
        }
    } else {
        1
    };

    // Ask for timeout
    print!("Connection timeout in milliseconds [500]: ");
    io::stdout().flush()?;
    let timeout_input = lines.next()
        .ok_or_else(|| anyhow::anyhow!("No input provided"))??;
    let timeout_ms = if timeout_input.trim().is_empty() {
        500
    } else {
        timeout_input.trim().parse::<u64>()?.max(100).min(10000)
    };

    // Ask for stealth options
    print!("\n=== STEALTH OPTIONS ===\n");
    print!("Randomize source port? (y/n) [n]: ");
    io::stdout().flush()?;
    let randomize_input = lines.next()
        .ok_or_else(|| anyhow::anyhow!("No input provided"))??;
    let randomize_source_port = randomize_input.trim().to_lowercase() == "y";

    // Ask for delay between probes
    print!("Add delay between probes? (y/n) [n]: ");
    io::stdout().flush()?;
    let delay_input = lines.next()
        .ok_or_else(|| anyhow::anyhow!("No input provided"))??;
    let delay_between_probes = if delay_input.trim().to_lowercase() == "y" {
        print!("Delay in milliseconds [100]: ");
        io::stdout().flush()?;
        let delay_ms_input = lines.next()
            .ok_or_else(|| anyhow::anyhow!("No input provided"))??;
        let delay_ms = if delay_ms_input.trim().is_empty() {
            100
        } else {
            delay_ms_input.trim().parse::<u64>()?.max(10).min(5000)
        };
        Some(std::time::Duration::from_millis(delay_ms))
    } else {
        None
    };

    // Ask for verbose output
    print!("\n=== OUTPUT OPTIONS ===\n");
    print!("Enable verbose output? (y/n) [y]: ");
    io::stdout().flush()?;
    let verbose_input = lines.next()
        .ok_or_else(|| anyhow::anyhow!("No input provided"))??;
    let verbose = verbose_input.trim().to_lowercase() != "n";

    // Build config
    Ok(ScanConfigBuilder::new()
        .target(target_ip)
        .scan_mode(mode)
        .timeout(std::time::Duration::from_millis(timeout_ms))
        .verbose(verbose)
        .detect_versions(detect_versions)
        .detect_os(detect_os)
        .parallel(parallel)
        .thread_count(thread_count)
        .randomize_source_port(randomize_source_port)
        .delay_between_probes(delay_between_probes)
        .build()?)
}

/// Display scan configuration info
fn display_scan_info(config: &ScanConfig) {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║              SCAN CONFIGURATION SUMMARY                  ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    
    println!("\n=== TARGET & SCOPE ===");
    println!("Target IP:       {}", config.target_ip);
    match &config.scan_mode {
        ScanMode::Range { start, end } => {
            println!("Scan Mode:       Port Range");
            println!("Port Range:      {}-{}", start, end);
            println!("Total Ports:     {}", end - start + 1);
        }
        ScanMode::CommonPorts => {
            println!("Scan Mode:       Common Ports");
            println!("Total Ports:     {} well-known ports", config.port_count());
        }
        ScanMode::CustomList(ports) => {
            println!("Scan Mode:       Custom Port List");
            println!("Total Ports:     {}", ports.len());
            if ports.len() <= 10 {
                println!("Ports:           {:?}", ports);
            }
        }
    }
    
    println!("\n=== DETECTION SETTINGS ===");
    println!("Service Detection:    {}", if config.detect_versions { "✓ Enabled" } else { "✗ Disabled" });
    println!("OS Detection (SMB):   {}", if config.detect_os { "✓ Enabled" } else { "✗ Disabled" });
    
    println!("\n=== PERFORMANCE SETTINGS ===");
    println!("Parallel Scanning:    {}", if config.parallel { "✓ Enabled" } else { "✗ Disabled" });
    if config.parallel {
        println!("Thread Count:         {}", config.thread_count);
    }
    println!("Connection Timeout:   {:?}", config.timeout);
    
    println!("\n=== STEALTH SETTINGS ===");
    println!("Source Port Randomization: {}", if config.randomize_source_port { "✓ Enabled" } else { "✗ Disabled" });
    if let Some(delay) = config.delay_between_probes {
        println!("Probe Delay:          {:?} (Stealth mode)", delay);
    } else {
        println!("Probe Delay:          None");
    }
    if config.is_stealth_enabled() {
        println!("⚠ Stealth Mode:       ACTIVE");
    }
    
    println!("\n=== OUTPUT SETTINGS ===");
    println!("Verbose Output:       {}", if config.verbose { "✓ Enabled" } else { "✗ Disabled" });
    
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║                    Starting Scan...                      ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");
}

/// Display text results
fn display_text_results(
    results: &ScanResults,
    duration: std::time::Duration,
    total_ports: usize,
    open_ports: usize,
    closed_ports: usize
) {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║                    SCAN RESULTS                          ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    
    println!("\n=== SUMMARY STATISTICS ===");
    println!("Total Ports Scanned: {}", total_ports);
    println!("Open Ports:          {} ({:.1}%)", open_ports, results.open_percentage());
    println!("Closed Ports:        {}", closed_ports);
    println!("Filtered Ports:      {}", results.filtered_ports);
    println!("Error Ports:         {}", results.error_ports);
    
    // Display open ports with FULL details
    if open_ports > 0 {
        println!("\n╔══════════════════════════════════════════════════════════╗");
        println!("║              OPEN PORTS - DETAILED ANALYSIS              ║");
        println!("╚══════════════════════════════════════════════════════════╝");
        
        for result in &results.results {
            if result.status.is_open() {
                println!("\n┌─ Port {} ────────────────────", result.port);
                println!("│ Status: OPEN");
                
                // Display service version if available
                if let Some(ref version) = result.service_version {
                    println!("│");
                    println!("│ ┌─ Service Detection ─────");
                    println!("│ │ Service:     {}", version.service_name);
                    if let Some(ref ver) = version.version {
                        println!("│ │ Version:     {}", ver);
                    }
                    println!("│ │ Protocol:    {}", version.protocol);
                    if let Some(ref banner) = version.banner {
                        if !banner.is_empty() {
                            println!("│ │ Banner:      {}", banner.lines().next().unwrap_or(banner));
                            if banner.lines().count() > 1 {
                                for line in banner.lines().skip(1).take(2) {
                                    println!("│ │              {}", line);
                                }
                            }
                        }
                    }
                    println!("│ └─────────────────────────");
                } else {
                    println!("│ Service:     Unknown (no banner detected)");
                }
                
                // Display OS info if available
                if let Some(ref os_info) = result.os_info {
                    println!("│");
                    println!("│ ┌─ OS Detection (SMB) ────");
                    if let Some(ref os_name) = os_info.os_name {
                        println!("│ │ OS Name:     {}", os_name);
                    }
                    if let Some(ref os_version) = os_info.os_version {
                        println!("│ │ OS Version:  {}", os_version);
                    }
                    if let Some(ref os_build) = os_info.os_build {
                        println!("│ │ OS Build:    {}", os_build);
                    }
                    if let Some(ref smb_version) = os_info.smb_version {
                        println!("│ │ SMB Version: {}", smb_version);
                    }
                    if let Some(ref computer_name) = os_info.computer_name {
                        println!("│ │ Computer:    {}", computer_name);
                    }
                    if let Some(ref domain) = os_info.domain {
                        println!("│ │ Domain:      {}", domain);
                    }
                    println!("│ │ Summary:     {}", os_info.summary());
                    println!("│ └─────────────────────────");
                }
                
                println!("└────────────────────────────────");
            }
        }
    }
    
    // Display filtered ports summary
    if results.filtered_ports > 0 {
        println!("\n=== FILTERED PORTS ===");
        println!("Count: {} (possibly firewalled)", results.filtered_ports);
        let filtered: Vec<u16> = results.results.iter()
            .filter(|r| matches!(r.status, port_scanner::PortStatus::Filtered))
            .map(|r| r.port)
            .collect();
        if filtered.len() <= 10 {
            println!("Ports: {:?}", filtered);
        } else {
            println!("Ports: {:?} ... and {} more", &filtered[..10], filtered.len() - 10);
        }
    }
    
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║                  PERFORMANCE METRICS                     ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!("Scan Duration:   {:.2?}", duration);
    if duration.as_secs_f64() > 0.0 {
        let ports_per_sec = total_ports as f64 / duration.as_secs_f64();
        println!("Scan Speed:      {:.2} ports/second", ports_per_sec);
        println!("Avg Time/Port:   {:.0} ms", (duration.as_millis() as f64) / (total_ports as f64));
    }
}