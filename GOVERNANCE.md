# 🏛️ SENTIENT OS Governance

## 📜 Project Governance

This document outlines the governance structure and decision-making processes for the SENTIENT OS project.

---

## 🎯 Mission Statement

SENTIENT OS aims to build an autonomous, secure, and high-performance AI Operating System with a Rust core, enabling seamless integration of diverse AI tools and platforms while maintaining enterprise-grade security and performance.

---

## 👥 Roles and Responsibilities

### 🔵 Users

Anyone who uses SENTIENT OS. Users can:
- Report bugs and request features
- Participate in discussions
- Contribute code or documentation

### 🟢 Contributors

Users who have contributed to the project. Contributors can:
- Submit pull requests
- Review code from other contributors
- Participate in working groups

### 🟡 Maintainers

Experienced contributors with merge access. Maintainers:
- Review and merge pull requests
- Triaging issues
- Guide technical direction
- Mentor new contributors

**Current Maintainers:**
| Maintainer | Role | Focus Area |
|------------|------|------------|
| @nexsusagent-coder | Lead Maintainer | Architecture, Core Systems |

### 🟠 Core Team

Senior maintainers responsible for project direction. Core Team:
- Sets project roadmap
- Makes final decisions on disputes
- Manages releases
- Represents the project publicly

---

## 📋 Decision Making

### 🟢 Lazy Consensus

For minor changes (documentation, bug fixes):
- No explicit approval needed
- 72-hour silence = approval
- Any maintainer can merge

### 🟡 Consensus

For significant changes (features, refactors):
- Requires maintainer approval
- 2+ maintainer approvals required
- 7-day review period

### 🔴 Voting

For major changes (architecture, governance):
- Core Team vote required
- 2/3 majority to pass
- 14-day voting period

---

## 🚀 Release Process

### Version Numbering

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes

### Release Schedule

| Release Type | Frequency | Example |
|--------------|-----------|---------|
| Patch | As needed | 4.0.1 → 4.0.2 |
| Minor | Monthly | 4.0.0 → 4.1.0 |
| Major | Quarterly | 4.0.0 → 5.0.0 |

### Release Checklist

- [ ] All tests pass
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version bumped
- [ ] Tag created
- [ ] Release notes published

---

## 🤝 Contributing

### Getting Started

1. Read [CONTRIBUTING.md](CONTRIBUTING.md)
2. Find a [Good First Issue](https://github.com/nexsusagent-coder/SENTIENT_CORE/labels/good%20first%20issue)
3. Fork and clone the repository
4. Make your changes
5. Submit a pull request

### Code Review Guidelines

**For Authors:**
- Write clear commit messages
- Add tests for new functionality
- Update documentation
- Keep PRs focused and small

**For Reviewers:**
- Be constructive and respectful
- Focus on code quality, not style preferences
- Provide actionable feedback
- Approve or request changes explicitly

---

## 🏗️ Working Groups

Specialized groups focusing on specific areas:

| Working Group | Focus | Lead |
|---------------|-------|------|
| Core | sentient_core, sentient_graph | TBD |
| AI/ML | sentient_cevahir, sentient_vision | TBD |
| Security | sentient_tee, sentient_zk_mcp | TBD |
| Channels | sentient_channels, integrations | TBD |
| Enterprise | sentient_enterprise, RBAC | TBD |

### Joining a Working Group

1. Contribute to the relevant area
2. Express interest to the lead
3. Participate in meetings
4. Demonstrate expertise

---

## 📊 Project Metrics

We track the following metrics to measure project health:

| Metric | Target | Current |
|--------|--------|---------|
| Test Coverage | >90% | ✅ 993 tests |
| Open Issues | <50 | Track |
| PR Review Time | <7 days | Track |
| Release Cadence | Monthly | Track |
| Contributor Growth | +10/month | Track |

---

## 🔒 Security

For security issues, please follow our [Security Policy](SECURITY.md):

- **Do NOT** open public issues for security vulnerabilities
- Email security@sentient-os.ai instead
- Expect response within 48 hours
- Security releases within 72 hours

---

## 📜 Code of Conduct

All participants must follow our [Code of Conduct](CODE_OF_CONDUCT.md).

### Enforcement

1. **Warning**: Private warning from maintainers
2. **Temporary Ban**: 30-day ban from project spaces
3. **Permanent Ban**: Permanent removal from project

---

## 📞 Contact

| Channel | Purpose |
|---------|---------|
| GitHub Issues | Bug reports, feature requests |
| GitHub Discussions | General questions, ideas |
| Discord | Real-time chat (coming soon) |
| Email | security@sentient-os.ai |

---

## 📅 Governance Review

This governance document is reviewed:
- Annually (January)
- After major project changes
- Upon community request

**Last Review:** April 2025
**Next Review:** January 2026

---

*SENTIENT OS is committed to building an inclusive, transparent, and sustainable open-source community.*
