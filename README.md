# 🔍 Rust Port Scanner

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

A modern, modular, and extensible port scanner written in Rust with clean architecture principles.

## ✨ Features

### Core Capabilities
- 🚀 **High-Performance Scanning** - Multi-threaded parallel scanning with configurable thread pools
- 🎯 **Multiple Scan Modes** - Common ports, custom ranges, or specific port lists
- 🔍 **Service Detection** - Banner grabbing and version detection for running services
- 🖥️ **OS Fingerprinting** - SMB-based operating system detection
- 📊 **Multiple Output Formats** - Text, JSON, and CSV output
- 🎭 **Stealth Features** - Source port randomization and configurable delays

### Architecture
- 🏗️ **Clean Architecture** - Layered design with clear separation of concerns
- 🔌 **Plugin System** - Extensible detector registry for custom detectors
- 📈 **Observer Pattern** - Real-time progress tracking and metrics collection
- 🎨 **Strategy Pattern** - Configurable scanning strategies (Standard/Stealth)
- 🔧 **Builder Pattern** - Fluent configuration API
- 🏭 **Factory Pattern** - Flexible output formatter creation

### Professional Features
- 📝 **Structured Logging** - Comprehensive tracing with configurable levels
- ⚠️ **Robust Error Handling** - Custom error types with context preservation
- 🧪 **Well-Tested** - Unit tests, integration tests, and benchmarks
- 🐳 **Docker Support** - Ready-to-use containers (Debian and Alpine)
- 🔒 **Security-First** - Daily vulnerability scans and dependency audits
- 📚 **Comprehensive Documentation** - API docs, guides, and examples

## 🚀 Quick Start

### Installation

**From Source:**
```bash
git clone https://github.com/yourusername/port-scanner.git
cd port-scanner
cargo build --release
```

**Build locally:**
```bash
cargo build --release
./target/release/port-scanner 127.0.0.1
```

### Basic Usage

**Scan common ports:**
```bash
port-scanner 192.168.1.1 --mode common
```

**Scan custom range:**
```bash
port-scanner 192.168.1.1 --mode range --start 1 --end 1000
```

**Scan specific ports:**
```bash
port-scanner 192.168.1.1 --mode custom --ports 22,80,443,3389
```

**With service detection:**
```bash
port-scanner 192.168.1.1 --mode common --detect-services --detect-os
```

**JSON output:**
```bash
port-scanner 192.168.1.1 --mode common --format json > results.json
```

## 🏗️ Architecture

The project follows **Clean Architecture** principles with clear separation:

```
port-scanner/
├── src/
│   ├── domain/           # Business logic (entities, value objects)
│   ├── infrastructure/   # External dependencies (network, I/O)
│   ├── scanning/         # Scanning strategies and execution
│   ├── application/      # Use cases and orchestration
│   ├── presentation/     # Output formatters and observers
│   └── constants.rs      # Application-wide constants
```

### Design Patterns Used

- **Strategy Pattern** - Scanning strategies (Standard, Stealth)
- **Builder Pattern** - Configuration builder with validation
- **Observer Pattern** - Progress tracking and metrics
- **Factory Pattern** - Output formatter creation
- **Repository Pattern** - Service information lookup
- **Plugin Architecture** - Extensible detector registry

## 🛠️ Development

### Prerequisites

- Rust 1.70+ (stable, beta, or nightly)
- Cargo

### Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/port-scanner.git
cd port-scanner

# Build
cargo build

# Run tests
cargo test --all-features

# Run benchmarks
cargo bench

# Check code quality
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

### Running Locally

```bash
# Debug build
cargo run -- 127.0.0.1 --mode common

# Release build (much faster)
cargo run --release -- 127.0.0.1 --mode common

# With logging
set RUST_LOG=debug
cargo run -- 127.0.0.1 --mode common
```

## 📖 Documentation

- [Quick Start Guide](QUICKSTART.md) - Get up and running quickly
- [Refactoring Guide](REFACTORING.md) - Understanding the architecture
- [CI/CD Guide](.github/CI-CD-GUIDE.md) - Setting up automated workflows
- [Contributing](CONTRIBUTING.md) - How to contribute
- [Security Policy](SECURITY.md) - Security practices and reporting

## 🐳 Docker Usage

**Build:**
```bash
docker build -t port-scanner .
```

**Run:**
```bash
docker run --rm --network host port-scanner 127.0.0.1
```

**With Docker Compose:**
```bash
docker-compose up -d test-target
docker-compose run scanner 127.0.0.1
docker-compose down
```

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is dual-licensed under:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

You may choose either license for your use.

## ⚠️ Legal Disclaimer

**This tool is for educational and authorized testing purposes only.**

- Only scan systems you own or have explicit permission to test
- Unauthorized port scanning may be illegal in your jurisdiction
- The authors assume no liability for misuse of this tool
- Always comply with local laws and regulations
- Respect responsible disclosure practices

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [Rayon](https://github.com/rayon-rs/rayon) for parallel processing
- Logging with [Tracing](https://github.com/tokio-rs/tracing)
- Error handling with [Thiserror](https://github.com/dtolnay/thiserror) and [Anyhow](https://github.com/dtolnay/anyhow)

---

**Made with ❤️ and Rust**
- Progress reporting during scans
- Statistical analysis of results
- Multiple output formats

## Building

```powershell
# Build the project
cargo build --release

# Run the scanner
cargo run --release
```

## Usage

Run the executable and follow the interactive prompts:

```
Enter target IP address (e.g., 127.0.0.1): 127.0.0.1
Enter start port (default 1): 1
Enter end port (default 1000): 100
Enter timeout in milliseconds (default 500): 500
Enable verbose output? (y/n, default n): n
```

## Extending

### Adding New Services

Edit `src/port_info.rs` and add entries to `ServiceDatabase::get_service_name()`:

```rust
pub fn get_service_name(port: u16) -> Option<&'static str> {
    match port {
        // ... existing entries
        12345 => Some("My Custom Service"),
        _ => None,
    }
}
```

### Custom Scan Strategies

Create a new module for advanced scanning techniques:
- SYN scanning
- UDP scanning  
- Parallel scanning with threads
- Rate limiting

### Output Formats

Extend `reporter.rs` to support:
- JSON output
- XML output
- CSV export
- HTML reports

## Testing

Run unit tests:

```powershell
cargo test
```

## License

MIT License - feel free to use and modify as needed.
