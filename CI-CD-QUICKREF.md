# CI/CD Pipeline - Quick Reference

## 🎯 Status Badges

Add these to the top of your README.md:

```markdown
[![CI/CD Pipeline](https://github.com/yourusername/port-scanner/workflows/CI%2FCD%20Pipeline/badge.svg)](https://github.com/yourusername/port-scanner/actions)
[![Security Scan](https://github.com/yourusername/port-scanner/workflows/Security%20Scan/badge.svg)](https://github.com/yourusername/port-scanner/actions)
[![codecov](https://codecov.io/gh/yourusername/port-scanner/branch/main/graph/badge.svg)](https://codecov.io/gh/yourusername/port-scanner)
[![Crates.io](https://img.shields.io/crates/v/port-scanner.svg)](https://crates.io/crates/port-scanner)
[![Docker](https://img.shields.io/docker/v/yourusername/port-scanner?label=docker)](https://hub.docker.com/r/yourusername/port-scanner)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
```

## 🚀 Quick Start Commands

### Run all CI checks locally:
```bash
# Format check
cargo fmt --all -- --check

# Linting
cargo clippy --all-targets --all-features -- -D warnings

# Tests
cargo test --all-features

# Documentation tests
cargo test --doc

# Benchmarks
cargo bench

# Security audit
cargo audit

# Check for unused deps
cargo +nightly udeps
```

### One-liner to run all checks:
```bash
cargo fmt --all -- --check && \
cargo clippy --all-targets --all-features -- -D warnings && \
cargo test --all-features && \
cargo test --doc
```

## 📦 Release Process

1. Update version:
```bash
# Edit Cargo.toml
version = "2.1.0"
```

2. Commit and tag:
```bash
git add Cargo.toml
git commit -m "Release v2.1.0"
git tag v2.1.0
git push origin main --tags
```

3. Automated:
- ✅ Builds for all platforms
- ✅ Creates GitHub Release
- ✅ Publishes to crates.io
- ✅ Pushes Docker image

## 🐳 Docker Quick Commands

```bash
# Build
docker build -t port-scanner .

# Run
docker run --rm --network host port-scanner 127.0.0.1

# With output file
docker run --rm -v $(pwd):/output port-scanner 127.0.0.1 > /output/scan.json

# Using Docker Compose
docker-compose up -d test-target
docker-compose run port-scanner 127.0.0.1
```

## 🔐 Required Secrets

Set these in GitHub Settings → Secrets:

| Secret | Description | Get From |
|--------|-------------|----------|
| `CODECOV_TOKEN` | Code coverage upload | codecov.io |
| `CARGO_TOKEN` | Publish to crates.io | crates.io/settings/tokens |
| `DOCKER_USERNAME` | Docker Hub username | hub.docker.com |
| `DOCKER_PASSWORD` | Docker Hub token | hub.docker.com/settings/security |

## 📊 What Gets Tested

- ✅ Code formatting (rustfmt)
- ✅ Linting (clippy)
- ✅ Unit tests
- ✅ Integration tests
- ✅ Documentation tests
- ✅ Code coverage
- ✅ Security vulnerabilities
- ✅ License compliance
- ✅ Dependency updates
- ✅ Multi-platform builds

## 🎯 Workflow Triggers

| Workflow | Push | PR | Tag | Schedule |
|----------|------|----|----|----------|
| CI/CD | ✅ | ✅ | ✅ | ❌ |
| Security | ✅ | ✅ | ❌ | Daily |
| Dependency Review | ❌ | ✅ | ❌ | ❌ |

## 💡 Pro Tips

1. **Test before pushing**: Run `cargo test --all-features`
2. **Check formatting**: Run `cargo fmt` before commit
3. **Review clippy**: Run `cargo clippy` locally
4. **Monitor actions**: Check GitHub Actions tab regularly
5. **Update deps**: Run `cargo update` weekly

## 🆘 Troubleshooting

### CI fails on formatting
```bash
cargo fmt --all
git add -A
git commit --amend --no-edit
git push --force-with-lease
```

### CI fails on clippy
```bash
cargo clippy --all-targets --all-features --fix
git add -A
git commit -m "Fix clippy warnings"
```

### Docker build fails
```bash
# Build locally first
docker build -t port-scanner .

# Check logs
docker build --progress=plain -t port-scanner .
```

### Release failed
```bash
# Delete tag and recreate
git tag -d v2.1.0
git push origin :refs/tags/v2.1.0
git tag v2.1.0
git push origin v2.1.0
```

## 📈 Monitoring

- **Actions**: github.com/yourusername/port-scanner/actions
- **Coverage**: codecov.io/gh/yourusername/port-scanner
- **Crates.io**: crates.io/crates/port-scanner
- **Docker Hub**: hub.docker.com/r/yourusername/port-scanner

## 🔗 Links

- [Full CI/CD Guide](.github/CI-CD-GUIDE.md)
- [Security Policy](SECURITY.md)
- [Contributing Guide](CONTRIBUTING.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)
