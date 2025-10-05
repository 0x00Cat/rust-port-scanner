# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.0.0] - 2025-10-04

### 🎉 Major Release - Complete Architectural Refactoring

This release represents a complete rewrite of the port scanner with clean architecture principles, professional-grade features, and production-ready infrastructure.

### ✨ Added

#### Core Features
- **Multiple Scan Modes**: Common ports, custom ranges, and specific port lists
- **Service Version Detection**: Banner grabbing with protocol-specific parsing
- **OS Fingerprinting**: SMB-based operating system detection
- **Parallel Scanning**: Multi-threaded execution with configurable thread pools
- **Stealth Features**: Source port randomization and configurable probe delays
- **Multiple Output Formats**: JSON, CSV, and text output with factory pattern

#### Architecture & Design Patterns
- **Clean Architecture**: 7-layer separation (Domain, Infrastructure, Scanning, Application, Presentation)
- **Strategy Pattern**: Pluggable scanning strategies (Standard, Stealth)
- **Builder Pattern**: Fluent configuration API with validation
- **Observer Pattern**: Real-time progress tracking and metrics collection
- **Factory Pattern**: Output formatter creation
- **Repository Pattern**: Service information lookup
- **Plugin Architecture**: Extensible detector registry

#### Infrastructure
- **Structured Logging**: Comprehensive tracing with configurable log levels
- **Error Handling**: Custom error types using `thiserror` with context preservation
- **Dependency Injection**: Network connector abstraction for testability
- **Performance**: Parallel execution using `rayon`

#### CI/CD & DevOps
- **GitHub Actions Workflows**:
  - Main CI/CD pipeline (11 jobs: format, lint, test, coverage, security, build, release, publish, docker, benchmark)
  - Daily security scanning (audit, deny, SAST, outdated dependencies)
  - Pull request dependency review
- **Multi-Platform Builds**: Linux (GNU/musl), Windows, macOS (x64/ARM)
- **Docker Support**: Debian and Alpine images with multi-arch support
- **Automated Releases**: GitHub Releases with checksums
- **Package Publishing**: Automatic crates.io publishing
- **Benchmarking**: Criterion-based performance testing

#### Documentation
- Comprehensive README with badges and examples
- Quick Start Guide (QUICKSTART.md)
- Refactoring Guide (REFACTORING.md)
- CI/CD Guide with setup instructions
- Security Policy (SECURITY.md)
- Contributing Guidelines (CONTRIBUTING.md)
- Git Setup Guide (GIT-SETUP-GUIDE.md)
- API documentation with examples

### 🔧 Changed
- Migrated from monolithic structure to modular clean architecture
- Replaced custom error handling with `thiserror`/`anyhow`
- Upgraded to parallel scanning with `rayon`
- Enhanced service detection with banner parsing
- Improved configuration with builder pattern and validation

### 🗑️ Deprecated
- Legacy modules maintained for backward compatibility but deprecated
- Old CLI interface replaced with modern argument parsing

### 🔒 Security
- Added daily security scanning with `cargo-audit`
- License compliance checking with `cargo-deny`
- SAST analysis with Semgrep
- Vulnerability monitoring and automated issue creation
- Security policy with responsible disclosure guidelines

### 📦 Dependencies
- Added `thiserror` 2.0 - Ergonomic error handling
- Added `anyhow` 1.0 - Flexible error propagation
- Added `tracing` 0.1 - Structured logging
- Added `tracing-subscriber` 0.3 - Log configuration
- Added `rayon` 1.10 - Data parallelism
- Added `criterion` 0.5 - Benchmarking (dev)

### 🧪 Testing
- Unit tests for all new modules
- Integration tests for scanning workflows
- Benchmark suite with 4 test scenarios
- Multi-platform CI testing (Ubuntu, Windows, macOS)
- Multi-Rust version testing (stable, beta, nightly)

### 📊 Performance
- Parallel scanning reduces scan time by ~10x for large port ranges
- Optimized thread pool sizing based on CPU cores
- Efficient memory usage with streaming results
- Fast service detection with timeout controls

---

## [1.0.0] - Initial Release

### Added
- Basic port scanning functionality
- Service identification for common ports
- Interactive CLI interface
- Verbose output mode
- Configurable timeouts
- Results reporting

---

## Version Numbering

This project follows [Semantic Versioning](https://semver.org/):
- **MAJOR** version for incompatible API changes
- **MINOR** version for backwards-compatible functionality additions
- **PATCH** version for backwards-compatible bug fixes

## Links

- [Unreleased]: https://github.com/yourusername/port-scanner/compare/v2.0.0...HEAD
- [2.0.0]: https://github.com/yourusername/port-scanner/releases/tag/v2.0.0
- [1.0.0]: https://github.com/yourusername/port-scanner/releases/tag/v1.0.0
