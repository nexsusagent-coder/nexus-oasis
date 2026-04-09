# 📖 Documentation Index

Welcome to the SENTIENT AI OS documentation. This index will help you find what you need.

## 🚀 Getting Started

| Document | Description |
|----------|-------------|
| [README.md](../README.md) | Project overview and quick start |
| [GETTING_STARTED.md](./GETTING_STARTED.md) | Step-by-step tutorial |
| [INSTALLATION.md](./INSTALLATION.md) | Installation methods |
| [CONFIGURATION.md](./CONFIGURATION.md) | Configuration options |

## 📚 Tutorials

| Tutorial | Difficulty | Description |
|----------|------------|-------------|
| [First Agent](../blog/tutorials/getting-started.md) | 🟢 Beginner | Create your first AI agent |
| [Custom Skills](../blog/tutorials/custom-skills.md) | 🟡 Intermediate | Build custom skills |
| [Kubernetes Deployment](../blog/tutorials/kubernetes-deployment.md) | 🟡 Intermediate | Deploy to K8s |
| [Multi-Agent Workflows](../blog/tutorials/multi-agent.md) | 🔴 Advanced | Coordinate multiple agents |

## 🔧 API Reference

| Document | Description |
|----------|-------------|
| [API.md](./API.md) | REST API documentation |
| [CHANNELS.md](./CHANNELS.md) | Channel integrations |
| [VOICE.md](./VOICE.md) | Voice API reference |
| [KUBERNETES.md](./KUBERNETES.md) | Kubernetes CRDs |

## 🏗️ Architecture

| Document | Description |
|----------|-------------|
| [Architecture Overview](./ARCHITECTURE.md) | System architecture |
| [Memory System](./MEMORY.md) | Memory management |
| [Security Model](./SECURITY_MODEL.md) | Security architecture |
| [Performance Guide](./PERFORMANCE.md) | Optimization tips |

## 🚢 Deployment

| Document | Description |
|----------|-------------|
| [DEPLOYMENT.md](./DEPLOYMENT.md) | Deployment guide |
| [Docker Guide](./DOCKER.md) | Docker deployment |
| [Kubernetes Guide](./KUBERNETES.md) | K8s deployment |
| [Cloud Deployment](./CLOUD.md) | AWS/GCP/Azure guides |

## 🔒 Security

| Document | Description |
|----------|-------------|
| [SECURITY.md](../SECURITY.md) | Security policy |
| [Security Model](./SECURITY_MODEL.md) | Architecture |
| [TEE Support](./TEE.md) | Trusted Execution |
| [ZK-MCP](./ZK_MCP.md) | Zero-knowledge proofs |

## 👥 Community

| Document | Description |
|----------|-------------|
| [CONTRIBUTING.md](../CONTRIBUTING.md) | Contribution guide |
| [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md) | Community standards |
| [HACKTOBERFEST.md](../HACKTOBERFEST.md) | Hacktoberfest guide |
| [CONTRIBUTORS.md](../CONTRIBUTORS.md) | Contributor list |

## 📊 Analysis & Roadmap

| Document | Description |
|----------|-------------|
| [ROADMAP.md](../ROADMAP.md) | 12-month roadmap |
| [COMPARISON.md](../COMPARISON.md) | vs OpenClaw |
| [COMPETITIVE_ANALYSIS.md](../COMPETITIVE_ANALYSIS.md) | 15+ frameworks |
| [CHANGELOG.md](../CHANGELOG.md) | Version history |

## 🗺️ Feature Map

```
SENTIENT AI OS
│
├── 🧠 Core
│   ├── sentient_core (types, traits)
│   ├── sentient_memory (storage)
│   ├── sentient_graph (knowledge)
│   └── sentient_common (utilities)
│
├── 🔒 Security
│   ├── sentient_tee (TEE)
│   ├── sentient_zk_mcp (ZK)
│   ├── sentient_anomaly (detection)
│   └── sentient_enterprise (RBAC, SSO)
│
├── 💬 Channels
│   ├── Telegram
│   ├── Discord
│   ├── WhatsApp
│   ├── Slack
│   ├── Signal
│   ├── Matrix
│   └── IRC
│
├── 🎤 Voice
│   ├── sentient_voice (STT/TTS)
│   └── sentient_wake (wake word)
│
├── 🛠️ Tools
│   ├── sentient_skills (skills)
│   ├── sentient_marketplace (registry)
│   └── sentient_skills_import (import)
│
├── ☸️ Infrastructure
│   ├── sentient_cluster (K8s)
│   ├── sentient_gateway (API)
│   └── sentient_cli (CLI)
│
└── 📱 Apps
    ├── Desktop (Tauri)
    ├── iOS (Swift)
    └── Android (Kotlin)
```

## 🔍 Search

Looking for something specific?

```bash
# Search documentation
grep -r "keyword" docs/

# Search code
grep -r "keyword" crates/
```

## ❓ FAQ

### General

**Q: What LLMs are supported?**
A: 100+ models from OpenAI, Anthropic, Google, Groq, DeepSeek, Mistral, Perplexity, Cohere, Together AI, X.AI, and local models via Ollama.

**Q: Is SENTIENT free?**
A: Yes! MIT/Apache-2.0 dual-licensed. Free forever for open source use.

**Q: What makes SENTIENT different?**
A: Rust-native (10x faster), TEE support, ZK-MCP, self-coding loop, Kubernetes operator.

### Technical

**Q: What Rust version is required?**
A: Rust 1.75 or later.

**Q: What databases are supported?**
A: PostgreSQL (primary), SQLite (embedded), LanceDB (vectors).

**Q: Can I run SENTIENT locally?**
A: Yes! Use local LLMs via Ollama integration.

### Enterprise

**Q: Is there enterprise support?**
A: Coming in Q4 2024. Join our waitlist!

**Q: What compliance standards?**
A: SOC 2, GDPR, HIPAA planned for Q4 2024.

**Q: Can I self-host?**
A: Yes! Docker and Kubernetes deployments available.

---

*Can't find what you need? [Open an issue](https://github.com/nexsusagent-coder/SENTIENT_CORE/issues) or [start a discussion](https://github.com/nexsusagent-coder/SENTIENT_CORE/discussions)!*
