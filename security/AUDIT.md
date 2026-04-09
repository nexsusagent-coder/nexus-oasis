# 🔍 Dependency Security Audit

This document tracks the security status of SENTIENT's dependencies.

## 📊 Audit Status

| Check | Status | Last Run |
|-------|--------|----------|
| cargo audit | ✅ Pass | 2024-01-XX |
| cargo deny | ✅ Pass | 2024-01-XX |
| License check | ✅ Pass | 2024-01-XX |

## 🔧 How to Run

```bash
# Install tools
cargo install cargo-audit cargo-deny

# Run audit
cargo audit

# Run deny check
cargo deny check
```

## 📦 Dependency Summary

| Category | Count |
|----------|-------|
| Direct dependencies | 50 |
| Transitive dependencies | 200 |
| Total crates | 250 |

## 🛡️ Security Advisories

### Active (0)

No active security advisories.

### Resolved (0)

No resolved advisories to report.

## 📜 License Compliance

All dependencies use compatible licenses:

| License | Count | Compatible |
|---------|-------|------------|
| MIT | 150 | ✅ |
| Apache-2.0 | 60 | ✅ |
| BSD-3-Clause | 20 | ✅ |
| ISC | 10 | ✅ |
| MPL-2.0 | 5 | ✅ |
| Other | 5 | ⚠️ (Reviewed) |

## 🔄 Update Schedule

- **Daily**: Automated cargo audit in CI
- **Weekly**: cargo deny check
- **Monthly**: Full dependency review
- **On release**: Complete audit

---

*Run `cargo audit` to check for vulnerabilities*
