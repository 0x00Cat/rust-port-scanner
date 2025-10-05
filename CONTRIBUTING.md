# Contributing to Port Scanner

First off, thank you for considering contributing to Port Scanner! 🎉

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Pull Request Process](#pull-request-process)
- [Coding Guidelines](#coding-guidelines)
- [Testing](#testing)
- [Documentation](#documentation)

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code.

### Our Standards

- ✅ Be respectful and inclusive
- ✅ Welcome newcomers and help them learn
- ✅ Focus on what's best for the community
- ✅ Show empathy towards others

## How Can I Contribute?

### 🐛 Reporting Bugs

Before creating bug reports:

1. **Check existing issues** to avoid duplicates
2. **Collect information** about the bug:
   - OS and version
   - Rust version (`rustc --version`)
   - Steps to reproduce
   - Expected vs actual behavior
   - Error messages or logs

Create an issue with:
```markdown
**Environment:**
- OS: Ubuntu 22.04
- Rust: 1.75.0
- Version: 2.0.0

**Description:**
Clear description of the bug

**Steps to Reproduce:**
1. Run `cargo run --release`
2. Select option 1
3. Enter IP address

**Expected:**
Should scan ports

**Actual:**
Crashes with error X

**Logs:**
```

### 💡 Suggesting Enhancements

Enhancement suggestions are welcome! Please provide:

1. **Use case**: Why is this needed?
2. **Current behavior**: What happens now?
3. **Proposed behavior**: What should happen?
4. **Examples**: Show how it would work
5. **Alternatives**: Other approaches considered

### 🔧 Contributing Code

#### Good First Issues

Look for issues labeled:
- `good first issue` - Great for newcomers
- `help wanted` - Community help needed
- `documentation` - Improve docs

#### Feature Development

Before starting major work:

1. **Open an issue** to discuss the feature
2. **Wait for approval** from maintainers
3. **Fork the repository**
4. **Create a feature branch**
5. **Implement the feature**
6. **Submit a pull request**

## Development Setup

### Prerequisites

- Rust 1.75 or later
- Git
- (Optional) Docker

### Setup Steps

1. **Fork and clone**:
```bash
git clone https://github.com/yourusername/port-scanner.git
cd port-scanner
```

2. **Create a branch**:
```bash
git checkout -b feature/my-new-feature
```

3. **Install dependencies**:
```bash
cargo build
```

4. **Run tests**:
```bash
cargo test
```

5. **Make changes and test**:
```bash
# Edit code
cargo fmt
cargo clippy
cargo test
cargo build --release
```

### Development Tools

Install helpful tools:

```bash
# Formatter
rustup component add rustfmt

# Linter
rustup component add clippy

# Security audit
cargo install cargo-audit

# Coverage
cargo install cargo-tarpaulin

# Unused deps
cargo install cargo-udeps
```

## Pull Request Process

### Before Submitting

- [ ] Code follows project style guidelines
- [ ] All tests pass (`cargo test`)
- [ ] New tests added for new functionality
- [ ] Documentation updated
- [ ] Commit messages are clear
- [ ] No merge conflicts with main
- [ ] CI checks pass

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
How was this tested?

## Checklist
- [ ] Tests pass
- [ ] Documentation updated
- [ ] No breaking changes (or documented)
- [ ] Changelog updated
```

### Review Process

1. **Automated checks** run first
2. **Maintainer review** within 7 days
3. **Changes requested** or approved
4. **Merge** after approval

## Coding Guidelines

### Rust Style

Follow the official Rust style guide:

```bash
# Format code
cargo fmt

# Check style
cargo fmt -- --check

# Linting
cargo clippy -- -D warnings
```

### Naming Conventions

- **Types**: `PascalCase` (e.g., `PortScanner`)
- **Functions**: `snake_case` (e.g., `scan_port`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `DEFAULT_TIMEOUT`)
- **Modules**: `snake_case` (e.g., `network_utils`)

### Code Organization

```rust
// 1. Imports
use std::net::SocketAddr;
use crate::domain::Port;

// 2. Constants
const DEFAULT_PORT: u16 = 80;

// 3. Type definitions
pub struct Scanner { ... }

// 4. Trait implementations
impl Scanner { ... }

// 5. Helper functions
fn helper_function() { ... }

// 6. Tests
#[cfg(test)]
mod tests { ... }
```

### Error Handling

Use `thiserror` for errors:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Network error: {0}")]
    Network(#[from] std::io::Error),
    
    #[error("Invalid port: {0}")]
    InvalidPort(u16),
}
```

### Documentation

Every public item needs docs:

```rust
/// Scans a port on the target system.
///
/// # Arguments
///
/// * `port` - The port number to scan
/// * `timeout` - Maximum time to wait
///
/// # Returns
///
/// Returns `PortScanResult` with the scan status
///
/// # Examples
///
/// ```
/// use port_scanner::scan_port;
/// let result = scan_port(80, Duration::from_secs(1));
/// ```
pub fn scan_port(port: u16, timeout: Duration) -> PortScanResult {
    // ...
}
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_validation() {
        assert!(is_valid_port(80));
        assert!(!is_valid_port(0));
        assert!(!is_valid_port(65536));
    }
}
```

### Integration Tests

Place in `tests/` directory:

```rust
// tests/integration_test.rs
use port_scanner::prelude::*;

#[test]
fn test_full_scan() {
    let config = ScanConfigBuilder::new()
        .target("127.0.0.1".parse().unwrap())
        .range(1, 100)
        .build()
        .unwrap();
    
    let scanner = PortScanner::new(config).unwrap();
    let results = scanner.scan_all(|_| {});
    
    assert!(results.total_ports > 0);
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_port_validation

# With output
cargo test -- --nocapture

# Documentation tests
cargo test --doc

# Ignored tests
cargo test -- --ignored
```

## Documentation

### Code Documentation

- All public APIs must be documented
- Include examples where helpful
- Explain why, not just what
- Document panics and errors

### User Documentation

Update relevant docs:

- `README.md` - For user-facing changes
- `QUICKSTART.md` - For getting started
- `REFACTORING.md` - For architecture changes
- `CI-CD-GUIDE.md` - For CI/CD changes

### Changelog

Update `CHANGELOG.md`:

```markdown
## [2.1.0] - 2025-10-05

### Added
- New UDP scanning capability (#123)
- Rate limiting options (#124)

### Changed
- Improved parallel scanning performance (#125)

### Fixed
- Timeout handling on Windows (#126)
```

## Release Process

(For maintainers)

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Commit changes
4. Create and push tag
5. CI creates release automatically

## Getting Help

- 💬 **Discord**: [Join our server](#)
- 📧 **Email**: your.email@example.com
- 🐛 **Issues**: [GitHub Issues](https://github.com/yourusername/port-scanner/issues)
- 📖 **Docs**: [Documentation](https://docs.rs/port-scanner)

## Recognition

Contributors are:

- Listed in `CONTRIBUTORS.md`
- Mentioned in release notes
- Credited in commit history

Thank you for contributing! 🚀

---

**Questions?** Feel free to open an issue or reach out!
