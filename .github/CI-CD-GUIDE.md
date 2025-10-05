# GitHub Actions CI/CD Pipeline Documentation

## 🚀 Overview

This repository includes a comprehensive CI/CD pipeline with multiple workflows:

1. **Main CI/CD Pipeline** (`ci.yml`) - Build, test, and release
2. **Security Scanning** (`security.yml`) - Daily security audits
3. **Dependency Review** (`dependency-review.yml`) - PR dependency checks

## 📋 Workflows

### 1. Main CI/CD Pipeline (ci.yml)

Triggers on:
- Push to `main` or `develop` branches
- Pull requests to `main` or `develop`
- Tags starting with `v*`
- Manual workflow dispatch

#### Jobs:

**Format Check (`fmt`)**
- Validates Rust code formatting
- Uses `cargo fmt --check`

**Clippy Lints (`clippy`)**
- Runs Rust linter
- Fails on warnings
- Caches dependencies for speed

**Test Suite (`test`)**
- Runs on Ubuntu, Windows, and macOS
- Tests on stable, beta, and nightly Rust
- Executes unit tests and doc tests
- Matrix strategy for comprehensive coverage

**Code Coverage (`coverage`)**
- Generates code coverage reports using `tarpaulin`
- Uploads to Codecov
- Helps track test coverage over time

**Security Audit (`security`)**
- Runs `cargo audit` for known vulnerabilities
- Checks against RustSec Advisory Database

**Dependency Check (`deps`)**
- Identifies unused dependencies
- Uses `cargo-udeps` on nightly

**Build Release (`build-release`)**
- Creates release binaries for multiple platforms:
  - Linux (GNU and musl)
  - Windows (x64)
  - macOS (x64 and ARM64)
- Strips binaries for smaller size
- Uploads artifacts

**Create Release (`release`)**
- Triggers on version tags (`v*`)
- Downloads all platform binaries
- Creates SHA256 checksums
- Publishes GitHub Release with binaries

**Publish to crates.io (`publish`)**
- Publishes library to crates.io
- Only on version tags
- Requires `CARGO_TOKEN` secret

**Docker Image (`docker`)**
- Builds and pushes Docker images
- Tags: latest, version tags, SHA
- Uses Docker Buildx for multi-arch support
- Requires Docker Hub credentials

**Benchmarks (`benchmark`)**
- Runs performance benchmarks
- Stores results for trend analysis
- Only on main branch pushes

### 2. Security Scan (security.yml)

Triggers on:
- Daily at 2 AM UTC
- Push to `main`
- Pull requests to `main`

#### Jobs:

**Security Audit**
- `cargo audit` for vulnerabilities
- Generates JSON report

**Cargo Deny**
- Checks licenses
- Validates dependencies
- Ensures compliance

**SAST Scan**
- Semgrep static analysis
- Security-focused rules

**Outdated Dependencies**
- Identifies outdated crates
- Creates GitHub issue if found

### 3. Dependency Review (dependency-review.yml)

Triggers on:
- Pull requests

#### Features:
- Reviews new dependencies
- Fails on moderate+ severity
- Blocks GPL/AGPL licenses
- Comments summary in PR

## 🔧 Setup Required

### 1. GitHub Secrets

Add these secrets in your repository settings:

```
CODECOV_TOKEN       - Codecov upload token (get from codecov.io)
CARGO_TOKEN         - crates.io API token (get from crates.io)
DOCKER_USERNAME     - Docker Hub username
DOCKER_PASSWORD     - Docker Hub password or access token
```

To add secrets:
1. Go to repository Settings
2. Navigate to Secrets and variables → Actions
3. Click "New repository secret"
4. Add each secret

### 2. Enable Codecov

1. Visit [codecov.io](https://codecov.io)
2. Sign in with GitHub
3. Add your repository
4. Copy the upload token
5. Add as `CODECOV_TOKEN` secret

### 3. Setup crates.io Publishing

1. Create account at [crates.io](https://crates.io)
2. Go to Account Settings → API Tokens
3. Create new token with publish rights
4. Add as `CARGO_TOKEN` secret

### 4. Setup Docker Hub

1. Create account at [hub.docker.com](https://hub.docker.com)
2. Create access token in Account Settings → Security
3. Add username as `DOCKER_USERNAME`
4. Add token as `DOCKER_PASSWORD`

## 📦 Creating a Release

1. **Update version in Cargo.toml**:
```toml
[package]
version = "2.1.0"
```

2. **Commit changes**:
```bash
git add Cargo.toml
git commit -m "Bump version to 2.1.0"
```

3. **Create and push tag**:
```bash
git tag v2.1.0
git push origin main
git push origin v2.1.0
```

4. **Automated process**:
   - CI builds all platform binaries
   - Creates GitHub Release
   - Publishes to crates.io
   - Builds and pushes Docker image

## 🐳 Docker Usage

### Build locally:
```bash
docker build -t port-scanner .
```

### Run with Docker:
```bash
docker run --rm port-scanner --help
```

### Scan localhost:
```bash
docker run --rm --network host port-scanner
```

### Using Docker Compose:
```bash
# Start test target
docker-compose up -d test-target

# Run scan
docker-compose run port-scanner 127.0.0.1

# Stop services
docker-compose down
```

## 🧪 Running CI Checks Locally

### Format check:
```bash
cargo fmt --all -- --check
```

### Clippy:
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Tests:
```bash
cargo test --all-features
```

### Security audit:
```bash
cargo install cargo-audit
cargo audit
```

### Unused dependencies:
```bash
cargo install cargo-udeps
cargo +nightly udeps
```

### Coverage:
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --all-features
```

## 📊 CI Status Badges

Add these to your README.md:

```markdown
![CI](https://github.com/yourusername/port-scanner/workflows/CI%2FCD%20Pipeline/badge.svg)
![Security](https://github.com/yourusername/port-scanner/workflows/Security%20Scan/badge.svg)
[![codecov](https://codecov.io/gh/yourusername/port-scanner/branch/main/graph/badge.svg)](https://codecov.io/gh/yourusername/port-scanner)
```

## 🔍 Troubleshooting

### Build fails on Windows
- Check line endings (use `git config core.autocrlf true`)
- Verify Windows-specific dependencies

### Docker push fails
- Verify Docker Hub credentials
- Check repository name matches `DOCKER_USERNAME/port-scanner`

### Coverage upload fails
- Verify Codecov token is correct
- Check repository is enabled on codecov.io

### Release creation fails
- Ensure tag follows `v*` format (e.g., `v2.0.0`)
- Verify `GITHUB_TOKEN` has correct permissions

## 🎯 Best Practices

1. **Always run tests locally** before pushing
2. **Use feature branches** for development
3. **Create PRs** to main/develop for review
4. **Tag releases** with semantic versioning
5. **Monitor CI results** in GitHub Actions tab
6. **Review security** scan results regularly
7. **Keep dependencies** up to date

## 📈 Metrics and Monitoring

- **Code coverage**: Tracked on Codecov
- **Benchmarks**: Stored in GitHub Pages
- **Security**: Daily scans and alerts
- **Dependencies**: Weekly outdated checks

## 🔒 Security

- Runs daily security audits
- Checks for known vulnerabilities
- Reviews licenses for compliance
- Scans for outdated dependencies
- SAST analysis on code changes

## 🚦 Workflow Optimization

The pipeline uses caching extensively:
- Cargo registry
- Cargo build artifacts
- Docker layer caching

This reduces build times significantly (typically 2-5 minutes after initial run).

## 📝 Customization

### Change target platforms:
Edit the matrix in `build-release` job in `ci.yml`

### Modify security rules:
Edit `deny.toml` for cargo-deny configuration

### Adjust test platforms:
Modify the matrix in `test` job

### Add new checks:
Create new jobs or workflows as needed

## 🆘 Support

For issues with:
- **GitHub Actions**: Check workflow logs
- **Codecov**: Visit codecov.io/gh/yourusername/port-scanner
- **Docker**: Check Docker Hub build logs
- **crates.io**: Verify package name availability

## 📚 Additional Resources

- [GitHub Actions Documentation](https://docs.github.com/actions)
- [Rust CI/CD Best Practices](https://doc.rust-lang.org/cargo/guide/continuous-integration.html)
- [Docker Best Practices](https://docs.docker.com/develop/dev-best-practices/)
- [Semantic Versioning](https://semver.org/)
