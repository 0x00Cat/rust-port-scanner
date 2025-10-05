#  Rust Port Scanner

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/badge/version-2.0.0-brightgreen.svg)](CHANGELOG.md)

A blazing-fast, async port scanner written in Rust with clean architecture principles. Built for performance, security, and extensibility.

##  Features

###  Performance
- **Async I/O** - Built on Tokio for maximum concurrency (1000+ simultaneous connections)
- **5-10x Faster** - Async architecture delivers massive speed improvements over thread-based scanning
- **Multi-threaded** - Configurable concurrency for optimal performance
- **Smart Timeouts** - Configurable connection timeouts to balance speed and accuracy

###  Scanning Capabilities
- **Multiple Scan Modes** - Common ports, custom ranges (e.g., 1-1000), or specific port lists (e.g., 80,443,8080)
- **Service Detection** - Banner grabbing and version detection for running services
- **OS Fingerprinting** - SMB-based operating system detection (Windows/Linux/Samba)
- **Stealth Features** - Source port randomization and configurable probe delays

###  Output & Reporting
- **Multiple Formats** - JSON, CSV, and human-readable text output
- **Flexible Filtering** - `--open-only` flag to show only open ports
- **Real-time Progress** - Live scan progress with observer pattern
- **Detailed Metrics** - Scan duration, ports/second, and comprehensive statistics

###  Architecture
- **Clean Architecture** - 7-layer design with clear separation of concerns
- **Async/Await** - Modern Rust async patterns with Tokio runtime
- **Plugin System** - Extensible detector registry for custom service detectors
- **Design Patterns** - Observer, Strategy, Builder, Factory, and Repository patterns
- **Type Safety** - Comprehensive domain modeling with custom error types

###  Professional Features
- **Structured Logging** - Comprehensive tracing with `--debug` flag support
- **Robust Error Handling** - Custom error types with full context preservation
- **CLI Validation** - Mutually exclusive argument groups prevent user errors
- **Well-Documented** - Extensive inline documentation and guides

##  Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/port-scanner.git
cd port-scanner

# Build in release mode for optimal performance
cargo build --release

# Binary location
./target/release/port-scanner
```

### Quick Build

```bash
cargo build --release
```

##  Quick Start

### Basic Scans

**Scan common ports (fastest):**
```bash
port-scanner -t 192.168.1.1 --common
```

**Scan port range:**
```bash
port-scanner -t 192.168.1.1 -p 1-1000
```

**Scan specific ports:**
```bash
port-scanner -t 192.168.1.1 -p 22,80,443,3306,8080
```

### Advanced Features

**With service and OS detection:**
```bash
port-scanner -t 192.168.1.1 --common -v -o
```

**Show only open ports:**
```bash
port-scanner -t 192.168.1.1 -p 1-1000 --open-only
```

**JSON output:**
```bash
port-scanner -t 192.168.1.1 --common -f json -F results.json
```

**Stealth scan with delays:**
```bash
port-scanner -t 192.168.1.1 -p 1-1000 --randomize-port --delay 100
```

**Verbose with debug logging:**
```bash
port-scanner -t 192.168.1.1 --common --verbose --debug
```

### Performance Tuning

**Fast scan (more threads, shorter timeout):**
```bash
port-scanner -t 192.168.1.1 -p 1-10000 -T 16 --timeout 200
```

**Thorough scan (longer timeout, service detection):**
```bash
port-scanner -t 192.168.1.1 --common -v -o --timeout 2000
```

##  Documentation

- **[PERFORMANCE_OPTIMIZATION_GUIDE.md](PERFORMANCE_OPTIMIZATION_GUIDE.md)** - Performance tuning and benchmarking
- **[SERVICE-DETECTION-GUIDE.md](SERVICE-DETECTION-GUIDE.md)** - Service detection and OS fingerprinting details
- **[CHANGELOG.md](CHANGELOG.md)** - Version history and feature updates
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - How to contribute to the project
- **[SECURITY.md](SECURITY.md)** - Security policy and vulnerability reporting

##  Usage Examples

### Interactive Mode

Run without arguments for an interactive guided setup:
```bash
port-scanner
```

### Non-Interactive Mode

Skip all prompts and use defaults:
```bash
port-scanner -t 192.168.1.1 -p 1-1000 --non-interactive
```

### Real-World Scenarios

**Web server reconnaissance:**
```bash
port-scanner -t example.com -p 80,443,8080,8443 -v --verbose
```

**Database server check:**
```bash
port-scanner -t db-server.local -p 3306,5432,27017,6379 -v
```

**Network device scan:**
```bash
port-scanner -t 192.168.1.1 -p 22,23,80,443,8080 -o --open-only
```

**Full network audit:**
```bash
port-scanner -t 10.0.0.50 -p 1-65535 -v -o -f json -F audit.json
```

##  Architecture

The project follows **Clean Architecture** with async/await patterns:

```
src/
 main_new.rs              # Async main entry point (Tokio runtime)
 lib.rs                   # Library exports

 domain/                  # Business entities (Port, Service, OS, etc.)
    port.rs
    service.rs
    os.rs

 application/             # Async use cases
    scan_ports.rs        # Async port scanning orchestration
    detect_service.rs    # Async service version detection
    detect_os.rs         # Async OS fingerprinting

 scanning/                # Async scanning strategies
    strategy.rs          # Async scan strategies (Standard/Stealth)
    executor.rs          # Async execution with JoinSet & Semaphore
    config.rs            # Configuration builder
    detector.rs          # Plugin architecture

 presentation/            # Output formatting & UI
    formatter.rs         # JSON/CSV/Text formatters
    observer.rs          # Progress tracking

 infrastructure/          # External I/O
    network.rs           # (Legacy sync code - being phased out)

 constants.rs             # Application constants
 errors.rs                # Custom error types
```

### Async Architecture

The scanner uses **Tokio** for async I/O:

- **Async Scanning** - `TcpStream::connect()` with `tokio::time::timeout`
- **Concurrency Control** - `Semaphore` limits simultaneous connections
- **Task Management** - `JoinSet` manages thousands of async tasks
- **Parallel Execution** - Async tasks run concurrently (not just parallel threads)

**Performance Benefits:**
- Traditional (threads): 8-16 concurrent connections  10-15 seconds for 1000 ports
- Async (Tokio): 1000+ concurrent connections  1-2 seconds for 1000 ports
- **Result: 5-10x speed improvement** 

##  Configuration

### CLI Arguments

| Argument | Short | Description | Example |
|----------|-------|-------------|---------|
| `--target` | `-t` | Target IP address | `-t 192.168.1.1` |
| `--ports` | `-p` | Ports to scan | `-p 1-1000` or `-p 80,443` |
| `--common` | `-c` | Scan common ports | `--common` |
| `--detect-versions` | `-v` | Enable service detection | `-v` |
| `--detect-os` | `-o` | Enable OS detection | `-o` |
| `--threads` | `-T` | Number of threads | `-T 16` |
| `--timeout` | | Connection timeout (ms) | `--timeout 500` |
| `--format` | `-f` | Output format (json/csv/text) | `-f json` |
| `--output-file` | `-F` | Output file path | `-F results.json` |
| `--verbose` | | Enable verbose output | `--verbose` |
| `--debug` | `-d` | Enable debug logging | `-d` |
| `--open-only` | | Show only open ports | `--open-only` |
| `--randomize-port` | | Randomize source port | `--randomize-port` |
| `--delay` | | Delay between probes (ms) | `--delay 100` |
| `--non-interactive` | | Disable prompts | `--non-interactive` |

**Note:** `--ports` and `--common` are mutually exclusive.

### Environment

Supports standard Rust environment variables:
- `RUST_LOG` - Set logging level (e.g., `RUST_LOG=debug`)
- `RUST_BACKTRACE` - Enable backtraces on panic

##  Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_scan_port

# Run benchmarks
cargo bench
```

##  Performance

### Benchmarks

Scanning 1000 ports on localhost:

| Mode | Time | Ports/sec | Method |
|------|------|-----------|--------|
| **Async (Current)** | **1-2s** | **500-1000** | Tokio async I/O |
| Thread-based (Old) | 10-15s | 66-100 | Rayon thread pool |
| Sequential | 60-90s | 11-16 | Single-threaded |

**Speedup: 5-10x faster with async!** 

### Optimization Tips

1. **Increase concurrency** - More threads for network I/O bound operations
   ```bash
   -T 32  # 32 concurrent tasks
   ```

2. **Reduce timeout** - Faster scans, may miss slow services
   ```bash
   --timeout 200  # 200ms timeout
   ```

3. **Disable detection** - Skip service/OS detection for speed
   ```bash
   # No -v or -o flags
   ```

4. **Use common ports** - Scan fewer, more likely ports
   ```bash
   --common  # ~26 ports vs 65,535
   ```

See [PERFORMANCE_OPTIMIZATION_GUIDE.md](PERFORMANCE_OPTIMIZATION_GUIDE.md) for detailed tuning.

##  Security

- **Input Validation** - All user inputs are validated
- **Error Handling** - No panics in production code
- **Type Safety** - Strong typing prevents common bugs
- **Dependency Audits** - Regular security scans
- **No Unsafe Code** - 100% safe Rust

Report security issues to: [SECURITY.md](SECURITY.md)

##  Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
# Clone and setup
git clone https://github.com/yourusername/port-scanner.git
cd port-scanner

# Install development dependencies
cargo build

# Run tests
cargo test

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy -- -D warnings
```

##  License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

##  Acknowledgments

Built with:
- [Tokio](https://tokio.rs/) - Async runtime
- [clap](https://github.com/clap-rs/clap) - CLI parsing
- [serde](https://serde.rs/) - Serialization
- [tracing](https://github.com/tokio-rs/tracing) - Structured logging

##  Support

- **Issues**: [GitHub Issues](https://github.com/yourusername/port-scanner/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/port-scanner/discussions)
- **Documentation**: See guides in this repository

---

**Made with  and Rust** 
