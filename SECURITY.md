# 🔒 Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 4.x     | ✅ Active support  |
| 3.x     | ⚠️ Security fixes only |
| < 3.0   | ❌ End of life     |

## Reporting a Vulnerability

We take security seriously. If you discover a security vulnerability, please follow these steps:

### 1. Do NOT create a public issue

Security vulnerabilities should be reported privately.

### 2. Email us

Send details to: **security@sentientos.dev**

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

### 3. Response Timeline

| Time | Action |
|------|--------|
| 24 hours | Acknowledge receipt |
| 72 hours | Initial assessment |
| 7 days | Detailed response |
| 14 days | Fix or mitigation plan |
| 30 days | CVE assignment (if applicable) |

### 4. Responsible Disclosure

We follow responsible disclosure:
- We'll work with you to understand and fix the issue
- We'll credit you in the security advisory (unless you prefer to remain anonymous)
- We ask that you don't disclose the issue publicly until we've released a fix

## Security Features

SENTIENT OS includes several security features:

### 🔐 V-GATE Architecture

```
┌─────────────┐      ┌─────────────┐      ┌─────────────┐
│   SENTIENT  │      │   V-GATE    │      │   LLM API   │
│   Client    │─────▶│   Proxy     │─────▶│   Provider  │
└─────────────┘      └─────────────┘      └─────────────┘
                           │
                     API Key (Secure)
                     Stored on server
```

**API keys are NEVER in client code.**

### 🛡️ Security Crates

| Crate | Purpose |
|-------|---------|
| `sentient_guardrails` | Input/output filtering, prompt injection detection |
| `sentient_vault` | Secret management (AWS, Azure, HashiCorp Vault) |
| `sentient_tee` | Trusted Execution Environment (AMD SEV-SNP, Intel TDX) |
| `sentient_zk_mcp` | Zero-knowledge proofs for MCP |
| `sentient_compliance` | SOC 2 controls, audit logging |

### 🔒 Secure by Design

- **Memory Safety**: Rust guarantees memory safety at compile time
- **No unwrap()**: All code uses proper error handling with `.expect()` or `?`
- **Minimal unsafe**: Only 10 unsafe blocks, all FFI-required
- **Encrypted Storage**: AES-256-GCM for sensitive data
- **Audit Logging**: Complete audit trail for compliance

## Security Best Practices

### For Users

1. **Never commit API keys** - Use V-GATE or environment variables
2. **Enable encryption** - Use `sentient_vault` for secrets
3. **Regular updates** - Keep SENTIENT OS updated
4. **Audit logs** - Monitor `sentient_compliance` audit logs

### For Developers

1. **Run security audit** before commits:
   ```bash
   cargo audit
   cargo clippy -- -W clippy::all
   ```

2. **Check dependencies**:
   ```bash
   cargo outdated
   cargo deny check
   ```

3. **Format code**:
   ```bash
   cargo fmt --all -- --check
   ```

## Known Security Considerations

### 1. LLM Provider APIs

- API keys are transmitted to LLM providers
- Use V-GATE to keep keys server-side
- Consider self-hosted models (Ollama, GPT4All) for sensitive data

### 2. Web Dashboard

- Default port: 8080
- Enable authentication for production
- Use HTTPS in production

### 3. Plugin System

- Plugins run in sandboxed environment
- Review plugin code before installation
- Use `sentient_plugin` security features

## Security Audit History

| Date | Auditor | Scope | Result |
|------|---------|-------|--------|
| 2025-04 | Internal | Code review | ✅ Passed |
| 2025-04 | cargo audit | Dependencies | ✅ No vulnerabilities |

## Contact

- **Security Team**: security@sentientos.dev
- **PGP Key**: [security.asc](security.asc)
- **GitHub Security**: [Security Advisories](https://github.com/nexsusagent-coder/SENTIENT_CORE/security/advisories)

---

**Last Updated**: April 2025

**Next Review**: July 2025
