# SENTIENT NEXUS OS

**Professional AI Agent Framework**

A powerful, modular AI agent platform with 100+ LLM support, multi-channel communication, and extensible skills system.

---

## Quick Install

### Linux / macOS
```bash
curl -sSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/quick-install.sh | bash
```

### Windows (PowerShell)
```powershell
irm https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/quick-install.ps1 | iex
```

---

## Usage

After installation, restart your terminal and run:

```bash
# Configure SENTIENT (first time)
sentient setup

# Launch dashboard
sentient

# Check status
sentient status
```

---

## Features

### 100+ LLM Models
Professional `provider/model_id` format:
- **Anthropic**: claude-3-5-sonnet, claude-3-opus, claude-3-haiku
- **OpenAI**: gpt-4o, gpt-4-turbo, o1-preview, o1-mini
- **Google**: gemini-2.0-flash, gemini-1.5-pro
- **OpenRouter**: 70+ models from all providers
- **Ollama**: 50+ local models (llama3.3, qwen2.5, deepseek-r1, etc.)
- **Groq, DeepSeek, Mistral, Perplexity, Cohere, Together AI, X.AI** and more

### Multi-Channel Communication
20+ platforms: Telegram, Discord, Slack, WhatsApp, Matrix, Signal, Email, Twitter, LinkedIn, Reddit, Web Dashboard, REST API

### Professional Setup
- Interactive TUI with fuzzy search
- QuickStart (fast) or Manual (full control) modes
- Security policies for each channel
- Hidden API key input

---

## Requirements

- **OS**: Linux, macOS, Windows
- **RAM**: 8GB minimum (16GB+ recommended for local models)
- **Disk**: 5GB+ free space
- **Rust**: Auto-installed if missing

### Optional
- **Ollama**: For local models (auto-installed on Linux/macOS)
- **GPU**: NVIDIA GPU for faster local inference

---

## Manual Installation

<details>
<summary>Click to expand</summary>

```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Clone repository
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git ~/.sentient
cd ~/.sentient

# 3. Build
cargo build --release --bin sentient-setup --bin sentient-shell

# 4. Run setup
./target/release/sentient-setup

# 5. Launch
./target/release/sentient-shell
```

</details>

---

## Commands

| Command | Description |
|---------|-------------|
| `sentient` | Launch dashboard |
| `sentient setup` | Run setup wizard |
| `sentient status` | Show system status |
| `sentient config` | Edit configuration |
| `sentient update` | Update to latest version |
| `sentient logs` | View logs |
| `sentient help` | Show help |

---

## Architecture

```
SENTIENT_CORE/
├── crates/                 # Rust modules
│   ├── sentient_setup/     # Setup wizard
│   ├── sentient_cli/       # CLI interface
│   ├── sentient_gateway/   # HTTP/WebSocket gateway
│   ├── sentient_memory/    # Memory system
│   └── ...                 # 40+ modules
├── dashboard/              # Web dashboard
├── skills/                 # Extensible skills
├── sentient                # Linux/macOS launcher
├── sentient.bat            # Windows launcher
└── quick-install.sh        # One-command install
```

---

## Documentation

- [Architecture](ARCHITECTURE.md)
- [Installation Guide](INSTALL.md)
- [User Manual](USER_MANUAL.md)
- [Model Providers](MODEL_PROVIDERS.md)
- [API Reference](docs/)

---

## License

MIT License

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md)
