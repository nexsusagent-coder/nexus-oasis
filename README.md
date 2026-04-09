# SENTIENT NEXUS OS

**Professional AI Agent Framework**

A powerful, modular AI agent platform with 100+ LLM support, multi-channel communication, voice integration, and extensible skills marketplace.

---

## 🚀 Quick Install

### Linux / macOS
```bash
curl -sSL https://get.sentient.ai | bash
```

### Windows (PowerShell)
```powershell
irm https://get.sentient.ai/ps | iex
```

### npm
```bash
npm install -g @sentient/ai
```

---

## 📦 Binary Downloads

| Platform | Architecture | Download |
|----------|-------------|----------|
| Linux | x86_64 | `sentient-linux-x86_64.tar.gz` |
| Linux | ARM64 | `sentient-linux-arm64.tar.gz` |
| macOS | Intel | `sentient-macos-x86_64.tar.gz` |
| macOS | Apple Silicon | `sentient-macos-arm64.tar.gz` |
| Windows | x86_64 | `sentient-windows-x86_64.zip` |

[Download from Releases](https://github.com/nexsusagent-coder/SENTIENT_CORE/releases)

---

## 🎯 Usage

After installation, restart your terminal and run:

```bash
# Configure SENTIENT (first time)
sentient setup

# Launch interactive REPL
sentient repl

# Start autonomous agent
sentient agent --goal "Build a REST API"

# Check status
sentient status
```

---

## ✨ Features

### 🧠 100+ LLM Models
Professional `provider/model_id` format:
- **Anthropic**: claude-3-5-sonnet, claude-3-opus, claude-3-haiku
- **OpenAI**: gpt-4o, gpt-4-turbo, o1-preview, o1-mini
- **Google**: gemini-2.0-flash, gemini-1.5-pro
- **OpenRouter**: 70+ models from all providers
- **Ollama**: 50+ local models (llama3.3, qwen2.5, deepseek-r1, etc.)
- **Groq, DeepSeek, Mistral, Perplexity, Cohere, Together AI, X.AI** and more

### 💬 Multi-Channel Communication
- **Telegram** - Bot API with commands
- **Discord** - Bot with slash commands
- **WhatsApp** - Business API (Cloud API)
- **Slack** - Bot API with blocks, modals, reactions
- **Signal** - signal-cli REST API
- **Matrix** - Client-Server API with E2EE
- **IRC** - RFC 1459 protocol
- **Email** - SMTP/IMAP integration

### 🎤 Voice Integration
- **Speech-to-Text** - OpenAI Whisper API or local Whisper
- **Text-to-Speech** - OpenAI, ElevenLabs, System TTS
- **Wake Word** - "Hey SENTIENT" activation (Porcupine, Vosk, Whisper)
- **Real-time Streaming** - Continuous voice interaction

### 📱 Native Applications
- **Desktop** - Tauri app (macOS, Windows, Linux)
- **iOS** - Native Swift app
- **Android** - Native Kotlin app

### ☸️ Kubernetes Operator
- **Distributed Agents** - Scale across clusters
- **CRDs** - SentientAgent, SentientTask
- **Auto-scaling** - Based on load
- **Prometheus Metrics** - Monitoring

### 🧩 Skills Marketplace
- Discover and install AI skills
- Publish your own skills
- Version management
- Ratings and reviews

### 🔒 Security
- Guardrails for safe AI interactions
- Sandbox for code execution
- Policy management

---

## 📋 Commands

| Command | Description |
|---------|-------------|
| `sentient` | Launch dashboard |
| `sentient setup` | Run setup wizard |
| `sentient repl` | Start interactive REPL |
| `sentient agent --goal "..."` | Run autonomous agent |
| `sentient llm chat` | Chat with LLM |
| `sentient gateway` | Start API server |
| `sentient serve` | Start 24/7 service |
| `sentient status` | Show system status |
| `sentient skill install <name>` | Install skill |
| `sentient skill search <query>` | Search skills |

---

## 🏗️ Architecture

```
SENTIENT_CORE/
├── crates/                     # Rust modules (50+ crates)
│   ├── sentient_cli/           # CLI interface
│   ├── sentient_channels/      # Telegram, Discord, WhatsApp, Slack, Signal, Matrix, IRC
│   ├── sentient_voice/         # Whisper STT + TTS
│   ├── sentient_wake/          # Wake word detection (Porcupine, Vosk)
│   ├── sentient_marketplace/   # Skills marketplace
│   ├── sentient_skills_import/ # ClawHub-compatible importer
│   ├── sentient_cluster/       # Kubernetes operator
│   ├── sentient_gateway/       # HTTP/WebSocket gateway
│   ├── sentient_memory/        # Memory system
│   ├── sentient_guardrails/    # Security policies
│   ├── sentient_orchestrator/   # Agent orchestration
│   └── ...                     # More modules
├── apps/
│   ├── desktop/                # Tauri desktop app
│   └── mobile/                 # iOS (Swift) + Android (Kotlin)
├── npm/                        # npm package (@sentient/ai)
├── dashboard/                   # Web dashboard
├── skills/                      # Extensible skills
├── install.sh                   # Quick install (Unix)
├── install.ps1                  # Quick install (Windows)
└── .github/workflows/          # CI/CD + Releases
```

---

## 🔧 Requirements

- **OS**: Linux, macOS, Windows
- **RAM**: 8GB minimum (16GB+ recommended for local models)
- **Disk**: 5GB+ free space
- **Rust**: Auto-installed if building from source

### Optional
- **Ollama**: For local models
- **GPU**: NVIDIA GPU for faster local inference
- **Porcupine**: For wake word detection

---

## 📚 Documentation

- [Getting Started](docs/GETTING_STARTED.md)
- [Architecture](docs/ARCHITECTURE.md)
- [API Reference](docs/API.md)
- [Model Providers](docs/MODEL_PROVIDERS.md)
- [Channels Guide](docs/CHANNELS.md)
- [Voice Guide](docs/VOICE.md)
- [Skills Development](docs/SKILLS.md)

---

## 🆚 Comparison

| Feature | OpenClaw | SENTIENT |
|---------|----------|----------|
| **Language** | TypeScript | Rust |
| **Performance** | Node.js | Native (10x faster) |
| **Memory** | ~500MB | ~50MB |
| **Install Time** | 30s (npm) | 30s (binary) |
| **Channels** | 50+ | 15+ (growing) |
| **Voice** | ✅ | ✅ Whisper + TTS |
| **Wake Word** | ✅ | ✅ Porcupine/Vosk/Whisper |
| **Skills** | ClawHub | Marketplace + ClawHub compatible |
| **Native Apps** | ✅ | ✅ Tauri + iOS + Android |
| **Kubernetes** | ❌ | ✅ Operator + CRDs |
| **Multi-Agent** | ❌ | ✅ Orchestration |
| **Self-Coding** | ❌ | ✅ Auto-improvement |
| **TEE Support** | ❌ | ✅ Trusted Execution |
| **ZK-MCP** | ❌ | ✅ Zero-knowledge proofs |

See [COMPARISON.md](COMPARISON.md) and [OPENCLAW_ANALYSIS.md](OPENCLAW_ANALYSIS.md) for detailed comparison.

---

## 📄 License

MIT License

---

## 🤝 Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md)

---

## 🙏 Credits

- OpenClaw - Inspiration for channel integrations
- OpenAI - Whisper and TTS APIs
- ElevenLabs - High-quality TTS
- The Rust community

---

**Built with ❤️ by NEXUS OASIS**
