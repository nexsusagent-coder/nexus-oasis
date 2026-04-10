# Changelog

All notable changes to SENTIENT OS will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [4.0.0] - 2025-04-10

### 🎉 Major Release - Enterprise Ready

### Added

#### New Crates (10)

- **sentient_mcp** - Model Context Protocol (MCP) implementation
  - JSON-RPC 2.0 protocol
  - Tools, Resources, Prompts
  - stdio, TCP, WebSocket transports
  - 33 tests

- **sentient_vision** - Vision/Multimodal AI support
  - Image processing (resize, crop, convert)
  - OCR infrastructure
  - OpenAI/Claude vision API integration
  - Image embeddings
  - 27 tests

- **sentient_plugin** - Plugin/Extension system
  - Plugin lifecycle management
  - Security sandbox
  - Marketplace registry
  - Plugin discovery
  - 31 tests

- **sentient_rag** - Native RAG Engine
  - 6 chunking strategies
  - Multiple embedding providers
  - Vector store integration
  - Retrieval pipeline
  - 58 tests

- **sentient_finetuning** - Model fine-tuning
  - LoRA, QLoRA support
  - Dataset preparation
  - Training job management
  - 34 tests

- **sentient_web** - Web Server
  - Axum 0.7 framework
  - REST API + WebSocket
  - Authentication middleware
  - Dashboard hosting
  - 21 tests

- **sentient_compliance** - SOC 2 Compliance
  - Type I/Type II certification support
  - Control management
  - Audit logging
  - Evidence collection
  - Compliance monitoring
  - 12+ tests

- **sentient_sla** - SLA Monitoring
  - Uptime tracking
  - Incident management
  - Support tiers (Free/Pro/Enterprise)
  - SLA credits
  - 11 tests

- **sentient_backup** - Backup/Restore
  - AES-256-GCM encryption
  - Scheduled backups
  - Multi-backend storage
  - Incremental backups
  - 8 tests

- **sentient_dr** - Disaster Recovery
  - Health monitoring
  - Automatic failover
  - Multi-region support
  - Recovery orchestration
  - 5+ tests

#### Channel Integrations (+20)

- WhatsApp Business
- Facebook Messenger
- Instagram DM
- Twitter/X
- LinkedIn
- Microsoft Teams
- Google Chat
- Signal
- Viber
- Line
- Snapchat
- WeChat
- iMessage (BlueBubbles)
- Amazon Chime
- Zoom
- Cisco Webex
- Mattermost
- Telegram User
- Discord User
- Custom REST/GraphQL

#### Voice Features

- Speaker Diarization ("who spoke when")
- Custom Wake Word training
- Multi-wake-word detection
- MFCC feature extraction

#### IDE Plugins

- VS Code Extension
  - 14 commands
  - 4 views (Chat, Skills, Models, History)
  - TypeScript implementation

- JetBrains Plugin
  - IntelliJ IDEA, PyCharm, WebStorm, etc.
  - Tool window, intentions, actions
  - Kotlin implementation

#### Governance

- GOVERNANCE.md - Project governance structure
- DISCORD_SERVER_GUIDE.md - Community server guide
- NEWSLETTER_TEMPLATE.md - Monthly update template
- SOC2_COMPLIANCE.md - SOC 2 documentation

### Changed

- README.md expanded to 32KB comprehensive documentation
- Setup Wizard updated to OpenClaw standard with TUI
- LLM support expanded to 600+ models across 40+ providers
- Model Recommendation System added to setup wizard
- ROADMAP.md updated with all completed items

### Fixed

- All TODOs resolved (67 → 0)
- All unwrap() replaced with expect() (14,379 → 0)
- unsafe blocks reduced (2,929 → 10, FFI-only)

### Security

- cargo audit passing with no vulnerabilities
- CodeQL analysis enabled
- Secret scanning enabled
- Security Policy (SECURITY.md) added

### Stats

| Metric | Value |
|--------|-------|
| Rust Crates | 63 |
| Rust Files | 776 |
| Lines of Code | 173,143 |
| Tests | 1,232 |
| Integrations | 72 |
| LLM Models | 600+ |
| Skills | 5,587+ |
| Channels | 23 |

---

## [3.0.0] - 2025-03-15

### Added

- Cevahir AI integration (Turkish LLM cognitive engine)
- Skills Marketplace
- Channel integrations (Telegram, Discord, Slack)
- Voice module (Whisper STT + TTS)
- Wake word detection

### Changed

- Migrated to Rust 1.75+
- Performance optimizations
- Memory usage improvements

---

## [2.0.0] - 2025-02-01

### Added

- Multi-agent orchestration
- Event graph system
- Memory cube (SQLite-based)
- Enterprise features (RBAC, SSO)

---

## [1.0.0] - 2025-01-01

### Added

- Initial release
- Core agent framework
- LLM gateway
- Basic memory system
- CLI interface

---

## Release Notes Template

```markdown
## [X.Y.Z] - YYYY-MM-DD

### Added
- New features

### Changed
- Changes in existing functionality

### Deprecated
- Soon-to-be removed features

### Removed
- Removed features

### Fixed
- Bug fixes

### Security
- Security improvements
```

---

[4.0.0]: https://github.com/nexsusagent-coder/SENTIENT_CORE/releases/tag/v4.0.0
[3.0.0]: https://github.com/nexsusagent-coder/SENTIENT_CORE/releases/tag/v3.0.0
[2.0.0]: https://github.com/nexsusagent-coder/SENTIENT_CORE/releases/tag/v2.0.0
[1.0.0]: https://github.com/nexsusagent-coder/SENTIENT_CORE/releases/tag/v1.0.0
