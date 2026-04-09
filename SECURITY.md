# 🔒 Security Policy

## Supported Versions

We release patches for security vulnerabilities in the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | ✅ Active development |
| < 0.1   | ❌ Not supported   |

## Reporting a Vulnerability

We take security vulnerabilities seriously. Thank you for improving the security of SENTIENT.

### How to Report

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via:

1. **Email**: [security@sentient.ai](mailto:security@sentient.ai)
2. **GitHub Security Advisory**: [Create a security advisory](https://github.com/nexsusagent-coder/SENTIENT_CORE/security/advisories/new)

You should receive a response within 48 hours. If for some reason you do not, please follow up via email.

### What to Include

Please include the following information:

- **Description**: Description of the vulnerability
- **Steps to Reproduce**: Step-by-step instructions
- **Impact**: What can an attacker achieve?
- **Affected Versions**: Which versions are affected?
- **Proof of Concept**: Optional but helpful
- **Suggested Fix**: Optional but helpful

### Response Timeline

| Time | Action |
|------|--------|
| 0-48h | Initial response, confirm receipt |
| 48h-7d | Triage and assessment |
| 7-30d | Develop and test fix |
| 30d+ | Coordinate disclosure |

## Security Features

### Built-in Security

SENTIENT includes several security features:

| Feature | Description |
|---------|-------------|
| **Memory Safety** | Rust guarantees memory safety |
| **TEE Support** | Trusted Execution Environment integration |
| **ZK-MCP** | Zero-knowledge proofs for privacy |
| **Audit Logging** | Comprehensive audit trail |
| **Encryption** | At-rest and in-transit encryption |

### Secure Development

- All code is reviewed before merging
- Automated security scanning in CI
- Dependency auditing with `cargo audit`
- Regular security updates

## Security Best Practices

### Configuration

```toml
# sentient.toml

[security]
# Enable audit logging
audit_log = true

# Encryption settings
[security.encryption]
at_rest = true
in_transit = true
algorithm = "AES-256-GCM"

# TEE settings (if available)
[security.tee]
enabled = true
provider = "sgx"  # sgx, sev, trustzone
```

### API Keys

```bash
# Never commit API keys!
# Use environment variables:
export OPENAI_API_KEY="sk-..."
export ANTHROPIC_API_KEY="sk-ant-..."

# Or use a secrets manager:
sentient config set openai.api_key "$(vault read -field=api_key secret/openai)"
```

### Network Security

```bash
# Use TLS in production
SENTIENT_TLS_ENABLED=true
SENTIENT_TLS_CERT=/path/to/cert.pem
SENTIENT_TLS_KEY=/path/to/key.pem

# Restrict network access
SENTIENT_BIND_ADDRESS=127.0.0.1
SENTIENT_BIND_PORT=8080
```

## Known Security Considerations

### 1. API Key Storage

API keys are stored in configuration files. Ensure proper file permissions:

```bash
chmod 600 ~/.config/sentient/config.toml
```

### 2. Network Exposure

By default, SENTIENT binds to localhost. In production:

- Use TLS/HTTPS
- Implement rate limiting
- Use authentication
- Restrict network access

### 3. Plugin Security

Plugins run with the same privileges as SENTIENT:

- Only install trusted plugins
- Review plugin code before installation
- Use sandboxed execution when possible

### 4. Memory Forensics

While Rust prevents many memory issues, secrets in memory can be extracted:

- Use TEE for sensitive operations
- Clear secrets after use
- Avoid logging sensitive data

## Security Updates

Security updates are announced via:

1. **GitHub Security Advisories**
2. **Release Notes**
3. **Discord Announcements**

Subscribe to updates by watching the repository.

## Hall of Fame

We gratefully acknowledge the following security researchers:

| Date | Researcher | Vulnerability |
|------|------------|---------------|
| - | Be the first! | - |

## Contact

For security-related questions:

- **Security Email**: [security@sentient.ai](mailto:security@sentient.ai)
- **PGP Key**: [Download public key](https://sentient.ai/security.asc)
- **Fingerprint**: `AAAA BBBB CCCC DDDD EEEE FFFF GGGG HHHH`

---

Thank you for helping keep SENTIENT and our users safe! 🛡️
