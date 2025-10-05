use std::net::{TcpListener, TcpStream, SocketAddr, IpAddr, Ipv4Addr};
use std::time::Duration;
use std::io::{self, Write};


#[derive(Debug)]
pub enum PortStatus {
    Open,
    Closed,
    Filtered,
    Error(String),
}

pub struct PortScanResult {
    pub port: u16,
    pub status: PortStatus,
}

fn scan_port(ip: IpAddr, port: u16, timeout: Duration) -> PortScanResult {
    let socket = SocketAddr::new(ip, port);
    match TcpStream::connect_timeout(&socket, timeout) {
        Ok(_) => PortScanResult { port, status: PortStatus::Open },
        Err(ref e) if e.kind() == io::ErrorKind::ConnectionRefused => PortScanResult { port, status: PortStatus::Closed },
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => PortScanResult { port, status: PortStatus::Filtered },
        Err(e) => PortScanResult { port, status: PortStatus::Error(e.to_string()) },
    }
}

fn scan_ports(ip: IpAddr, start_port: u16, end_port: u16, timeout: Duration) -> Vec<PortScanResult> {
    let mut results = Vec::new();
    for port in start_port..=end_port {
        let result = scan_port(ip, port, timeout);
        println!("Scanned port {}: {:?}", port, result.status);
        io::stdout().flush().unwrap();
        results.push(result);
    }
    println!("\nScan complete.");
    results
}

fn show_open_ports(results: &[PortScanResult]) {
    println!("Open ports:");
    let open_ports = results.iter().filter(|r| matches!(r.status, PortStatus::Open)).collect::<Vec<_>>();
    if open_ports.is_empty() {
        println!("No open ports found.");
        return;
    }
    else {
        for result in open_ports {
            println!("Port {} is open", result.port);
        }
    }
}

fn get_service_name(port: u16) -> Option<&'static str> {
    match port {
        20 => Some("FTP Data"),
        21 => Some("FTP Control"),
        22 => Some("SSH"),
        23 => Some("Telnet"),
        25 => Some("SMTP"),
        53 => Some("DNS"),
        80 => Some("HTTP"),
        110 => Some("POP3"),
        143 => Some("IMAP"),
        443 => Some("HTTPS"),
        445 => Some("SMB"),
        3306 => Some("MySQL"),
        3389 => Some("RDP"),
        5432 => Some("PostgreSQL"),
        8080 => Some("HTTP Proxy"),
        _ => None,
    }
}

/// Display open ports with service names
fn display_detailed_results(results: &[PortScanResult]) {
    println!("\n=== DETAILED RESULTS ===");
    
    for result in results.iter().filter(|r| matches!(r.status, PortStatus::Open)) {
        let service = match get_service_name(result.port) {
            Some(name) => format!(" ({})", name),
            None => String::new(),
        };
        
        println!("Port {}{}: {:?}", result.port, service, result.status);
    }
}

fn main() {
    println!("=== Rust Port Scanner ===\n");
    
    // Get target IP from user
    print!("Enter target IP address (e.g., 127.0.0.1): ");
    io::stdout().flush().unwrap();
    
    let mut ip_input = String::new();
    io::stdin().read_line(&mut ip_input).expect("Failed to read input");
    
    let ip: IpAddr = match ip_input.trim().parse() {
        Ok(addr) => addr,
        Err(_) => {
            eprintln!("Invalid IP address!");
            return;
        }
    };
    
    // Get port range
    print!("Enter start port (default 1): ");
    io::stdout().flush().unwrap();
    
    let mut start_input = String::new();
    io::stdin().read_line(&mut start_input).expect("Failed to read input");
    let start_port: u16 = start_input.trim().parse().unwrap_or(1);
    
    print!("Enter end port (default 1000): ");
    io::stdout().flush().unwrap();
    
    let mut end_input = String::new();
    io::stdin().read_line(&mut end_input).expect("Failed to read input");
    let end_port: u16 = end_input.trim().parse().unwrap_or(1000);
    
    // Validate range
    if start_port > end_port {
        eprintln!("Start port must be less than or equal to end port!");
        return;
    }
    
    println!("\nScanning {}:{}-{}", ip, start_port, end_port);
    println!("This may take a while...\n");
    
    // Perform the scan
    let timeout = Duration::from_millis(500);
    let results = scan_ports(ip, start_port, end_port, timeout);
    
    // Display results
    show_open_ports(&results);
    display_detailed_results(&results);
    
    // Summary statistics
    let total = results.len();
    let open = results.iter().filter(|r| matches!(r.status, PortStatus::Open)).count();
    let closed = results.iter().filter(|r| matches!(r.status, PortStatus::Closed)).count();
    let filtered = results.iter().filter(|r| matches!(r.status, PortStatus::Filtered)).count();
    
    println!("\n=== SUMMARY ===");
    println!("Total ports scanned: {}", total);
    println!("Open: {}", open);
    println!("Closed: {}", closed);
    println!("Filtered: {}", filtered);
}
