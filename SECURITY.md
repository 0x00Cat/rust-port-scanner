# Security Policy

## Supported Versions

We release patches for security vulnerabilities. Currently supported versions:

| Version | Supported          |
| ------- | ------------------ |
| 2.x.x   | :white_check_mark: |
| 1.x.x   | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability, please do the following:

### 🔒 Private Disclosure

**DO NOT** open a public GitHub issue.

Instead, please email security details to:
- **Email**: your.email@example.com
- **Subject**: [SECURITY] Port Scanner Vulnerability

### What to Include

Please include the following information:

1. **Description** of the vulnerability
2. **Steps to reproduce** the issue
3. **Potential impact** of the vulnerability
4. **Suggested fix** (if you have one)
5. **Your name/handle** (for credit in disclosure)

### Response Timeline

- **24 hours**: Initial response acknowledging receipt
- **7 days**: Assessment and severity classification
- **30 days**: Fix developed and tested
- **45 days**: Public disclosure (coordinated with reporter)

### Security Updates

Security updates will be:

1. **Patched** in the latest release
2. **Announced** in GitHub Security Advisories
3. **Published** with CVE if applicable
4. **Documented** in CHANGELOG.md

## Security Best Practices

When using this port scanner:

### ✅ DO

- Run with minimum required privileges
- Use in authorized environments only
- Keep software updated to latest version
- Review scan results before sharing
- Use rate limiting options
- Enable logging for audit trails

### ❌ DON'T

- Run with root/admin privileges unnecessarily
- Scan networks without permission
- Share credentials or sensitive data
- Disable security features
- Run on production systems without testing

## Security Features

This scanner includes:

- ✅ **No elevated privileges required** (for most scans)
- ✅ **Input validation** on all user inputs
- ✅ **Timeout controls** to prevent DoS
- ✅ **Rate limiting** options available
- ✅ **Secure defaults** in configuration
- ✅ **No remote code execution** vectors
- ✅ **Dependency scanning** in CI/CD
- ✅ **Regular security audits**

## Known Limitations

- Port scanning may be detected by IDS/IPS systems
- Some scans require elevated privileges
- Network policies may block scanning
- Timeout settings affect accuracy

## Security Audits

We perform:

- **Daily**: Automated vulnerability scans (cargo-audit)
- **Weekly**: Dependency updates review
- **Monthly**: Manual security review
- **Quarterly**: Penetration testing

## Compliance

This tool is designed for:

- Security testing (with authorization)
- Network administration
- Educational purposes
- Research and development

**Not intended for:**
- Unauthorized network scanning
- Malicious activities
- Violation of terms of service

## Legal Notice

Users are responsible for:

1. Obtaining proper authorization
2. Complying with local laws
3. Respecting terms of service
4. Following ethical guidelines

## Security Resources

- [RustSec Advisory Database](https://rustsec.org/)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [CWE Database](https://cwe.mitre.org/)
- [CVE Database](https://cve.mitre.org/)

## Contact

For security-related questions:
- **Email**: your.email@example.com
- **GPG Key**: [Your GPG Key ID]

For general issues:
- **GitHub Issues**: https://github.com/yourusername/port-scanner/issues

## Acknowledgments

We thank the following security researchers for responsible disclosure:

(List will be updated as researchers are credited)

---

**Last Updated**: October 4, 2025
