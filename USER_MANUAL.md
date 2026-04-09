# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS v2.0.0 - COMPREHENSIVE USER MANUAL
#  The Operating System That Thinks
# ═══════════════════════════════════════════════════════════════════════════════

```
    🧠 SENTIENT OS - Universal AI Operations Platform
    
    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    
    📦 71 Integrated Projects  - LangChain, CrewAI, AutoGen, MindSearch...
    🤖 5,587+ Native Skills    - Largest AI skill collection
    🔧 43+ Native Tools        - MindSearch, Browser-Use, Mem0...
    🦀 37 Rust Crates          - 100% Native performance
    📁 57,309+ Files           - 13 GB total size
    🌐 Universal Gateway       - Connect to ANY LLM API
    💬 20+ Messaging Channels  - WhatsApp, Telegram, Discord, Slack...
    🔐 V-GATE Proxy            - API keys NEVER in code
    
    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
```

---

## 📋 TABLE OF CONTENTS

1. [Introduction](#1-introduction)
2. [Step-by-Step Installation](#2-step-by-step-installation)
3. [SENTIENT Shell Command Reference](#3-sentient-shell-command-reference)
4. [Agent-S3 Hardware Permissions](#4-agent-s3-hardware-permissions)
5. [Custom Provider Connection](#5-custom-provider-connection)
6. [Full Autonomous Mode](#6-full-autonomous-mode)
7. [7-Layer Architecture (L1-L7)](#7-7-layer-architecture-l1-l7)
8. [Integrations](#8-integrations)
9. [FAQ](#9-faq)
10. [Appendices](#10-appendices)

---

## 1. INTRODUCTION

### 1.1 What is SENTIENT OS?

**SENTIENT OS** is the world's most comprehensive AI Operating System. Built on 71 open-source projects with a Rust-based core, it delivers autonomous operation, enterprise-grade security, and high performance.

**Core Features:**

| Feature | Description |
|---------|-------------|
| 🦀 **100% Native Rust Core** | No Python dependency, maximum performance |
| 📦 **5,587+ Native Skills** | File, web, code, data, system operations |
| 🔧 **43+ Native Tools** | MindSearch, Browser-Use, Mem0, Lightpanda |
| 🔐 **V-GATE Proxy** | API keys NEVER stored in code |
| 🧠 **Memory Cube** | Episodic, semantic, procedural memory |
| 🤖 **Autonomous Mode** | Full automation including keyboard/mouse |
| 🌐 **Universal Gateway** | Any OpenAI-compatible API |
| 💬 **20+ Channels** | WhatsApp, Telegram, Discord, Slack and more |

### 1.2 System Requirements

| Level | CPU | RAM | Disk |
|--------|-----|-----|------|
| **Minimum** | 4 cores | 8 GB | 25 GB SSD |
| **Recommended** | 8+ cores | 16+ GB | 50+ GB SSD |
| **Production** | 16+ cores | 32+ GB | 100+ GB NVMe |

**Supported Operating Systems:**
- ✅ Ubuntu 22.04+ (recommended)
- ✅ Debian 12+
- ✅ Fedora 39+
- ✅ macOS 13+ (Apple Silicon)
- ✅ Windows 10/11 (Native + WSL2)

---

## 2. STEP-BY-STEP INSTALLATION

### 2.0 Platform Selection

| Platform | Setup Command | Notes |
|----------|---------------|-------|
| **🐧 Linux** | `./setup.sh` | Ubuntu/Debian/Fedora |
| **🪟 Windows** | `powershell -ExecutionPolicy ByPass -File setup.ps1` | Native PowerShell |
| **🍎 macOS** | `./setup.sh` | Homebrew required |

---

### 2.1 🐧 Linux / macOS Installation

```bash
# Open terminal and clone the repository
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
cd SENTIENT_CORE

# Give execution permission
chmod +x setup.sh

# One-command installation
./setup.sh
```

---

### 2.2 🪟 Windows Installation (Native)

#### YÖNTEM 1: Otomatik Kurulum (Önerilen)

```powershell
# ADIM 1: PowerShell'i YÖNETİCİ olarak açın
# Sağ tık -> "Run as Administrator"

# ADIM 2: Execution Policy ayarla
Set-ExecutionPolicy Bypass -Scope Process -Force

# ADIM 3: Kurulum scriptini çalıştır
# Seçenek A: Local dosyadan
.\setup.ps1

# Seçenek B: GitHub'dan doğrudan
iex ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/setup.ps1'))

# ADIM 4: Gemma 4 modelini seç
# 1 = gemma4:31b (Önerilen, ~20GB)
# 2 = gemma4:12b (~8GB)
# 3 = gemma4:4b (~3GB)
```

#### YÖNTEM 2: Manuel Kurulum

```powershell
# 1. Visual Studio Build Tools kur
# https://visualstudio.microsoft.com/visual-cpp-build-tools/
# "Desktop development with C++" iş yükünü seç

# 2. Rust kur
# https://rustup.rs adresinden indir
# Veya PowerShell'de:
winget install Rustlang.Rustup

# 3. Git kur
winget install Git.Git

# 4. Ollama kur (Gemma 4 Kernel)
# https://ollama.com/download adresinden Windows sürümünü indir

# 5. Projeyi klonla
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
cd SENTIENT_CORE

# 6. Derle
cargo build --release

# 7. Başlat
.\target\release\sentient-shell.exe
```

#### Windows Gereksinimleri

| Bileşen | Minimum | İndirme Linki |
|---------|---------|---------------|
| Windows | 10/11 64-bit | - |
| Visual Studio Build Tools | 2022 | [İndir](https://visualstudio.microsoft.com/visual-cpp-build-tools/) |
| Rust | 1.75+ | [İndir](https://rustup.rs) |
| Git | 2.30+ | [İndir](https://git-scm.com/download/win) |
| Ollama | Latest | [İndir](https://ollama.com/download) |
| Docker Desktop | Latest | [İndir](https://www.docker.com/products/docker-desktop/) (Opsiyonel) |

#### Windows Kurulum Sorunları

| Sorun | Çözüm |
|-------|-------|
| `cargo not found` | Terminal'i yeniden aç veya `~/.cargo/bin` PATH'e ekle |
| `linker 'link.exe' not found` | Visual Studio Build Tools kur |
| `openssl error` | `cargo install openssl` veya vcpkg kullan |
| `Permission denied` | PowerShell'i yönetici olarak çalıştır |
| `Execution Policy` hatası | `Set-ExecutionPolicy Bypass -Scope Process` |

---

### 2.3 Installation Progress

**Installation Steps (Automatic):**

```
╔════════════════════════════════════════════════════════════════════════════════╗
║  🧠 SENTIENT OS - ONE-COMMAND SETUP                                              ║
╠════════════════════════════════════════════════════════════════════════════════╣
║                                                                                ║
║  [1/8] Checking Rust toolchain...                                               ║
║        ✓ Rust 1.75+ installed                                                   ║
║                                                                                ║
║  [2/8] Installing system dependencies...                                       ║
║        ✓ build-essential, pkg-config, libssl-dev                               ║
║                                                                                ║
║  [3/8] Building SENTIENT Core (37 crates)...                                    ║
║        ✓ Release build complete                                                ║
║                                                                                ║
║  [4/8] Initializing Memory Cube (SQLite)...                                     ║
║        ✓ Database created at ~/.sentient/memory.db                             ║
║                                                                                ║
║  [5/8] Loading 5,587 native skills...                                           ║
║        ✓ Skills indexed and ready                                              ║
║                                                                                ║
║  [6/8] Configuring V-GATE Proxy...                                             ║
║        ✓ Listening on http://127.0.0.1:8080                                   ║
║                                                                                ║
║  [7/8] Setting up SENTIENT Shell...                                             ║
║        ✓ Shell ready at /usr/local/bin/sentient                               ║
║                                                                                ║
║  [8/8] Starting Dashboard...                                                   ║
║        ✓ Available at http://localhost:8080                                   ║
║                                                                                ║
╚════════════════════════════════════════════════════════════════════════════════╝

✅ SENTIENT OS installed successfully!
```

### 2.4 First Launch

#### Linux / macOS

```bash
# Start SENTIENT Shell
sentient

# Or use make
make run

# Dashboard
cargo run --release --bin sentient-dashboard
# http://localhost:8080
```

#### Windows

```powershell
# Start SENTIENT Shell
.\target\release\sentient-shell.exe

# Dashboard
.\target\release\sentient-dashboard.exe
# http://localhost:8080

# Or with cargo
cargo run --release --bin sentient-shell
cargo run --release --bin sentient-dashboard
```

**First Launch Output:**

```
    ╔═══════════════════════════════════════════════════════════════════════════╗
    ║                                                                           ║
    ║   🧠 SENTIENT OS v2.0.0                                                   ║
    ║   The Operating System That Thinks                                        ║
    ║                                                                           ║
    ║   Type 'help' for available commands                                     ║
    ║                                                                           ║
    ╚═══════════════════════════════════════════════════════════════════════════╝

sentient> _
```

---

## 3. SENTIENT SHELL COMMAND REFERENCE

### 3.1 System Commands

| Command | Description | Example |
|---------|-------------|---------|
| `help` | Show all commands | `help` |
| `status` | System status | `status` |
| `version` | Version info | `version` |
| `clear` | Clear screen | `clear` |
| `exit` | Exit shell | `exit` |

### 3.2 Skill Commands

| Command | Description | Example |
|---------|-------------|---------|
| `skills` | List all 5,587 skills | `skills` |
| `skills <category>` | Filter by category | `skills dev` |
| `skills search <query>` | Search skills | `skills search python` |
| `skill <id>` | Skill details | `skill dev_python_001` |

### 3.3 Tool Commands

| Command | Description | Example |
|---------|-------------|---------|
| `tools` | List all 43 tools | `tools` |
| `tool <name>` | Tool details | `tool mindsearch` |
| `run <tool>` | Execute tool | `run mindsearch "AI trends"` |

### 3.4 Agent Commands

| Command | Description | Example |
|---------|-------------|---------|
| `agents` | List agents | `agents` |
| `agent <name>` | Agent details | `agent coder` |
| `spawn <type>` | Create agent | `spawn researcher` |

### 3.5 V-GATE Commands

| Command | Description | Example |
|---------|-------------|---------|
| `vgate status` | Proxy status | `vgate status` |
| `vgate config` | Show config | `vgate config` |
| `vgate test` | Test connection | `vgate test` |

### 3.6 Memory Commands

| Command | Description | Example |
|---------|-------------|---------|
| `memory stats` | Memory statistics | `memory stats` |
| `memory search <query>` | Search memory | `memory search "project"` |
| `memory clear` | Clear short-term | `memory clear` |

---

## 4. AGENT-S3 HARDWARE PERMISSIONS

### 4.1 Linux (X11)

```bash
# Create udev rules for input devices
sudo tee /etc/udev/rules.d/99-sentient-input.rules << 'EOF'
KERNEL=="uinput", MODE="0666", GROUP="input"
KERNEL=="event*", MODE="0666", GROUP="input"
KERNEL=="mouse*", MODE="0666", GROUP="input"
KERNEL=="kbd", MODE="0666", GROUP="input"
EOF

# Reload udev rules
sudo udevadm control --reload-rules
sudo udevadm trigger

# Add user to input group
sudo usermod -a -G input $USER
```

### 4.2 Linux (Wayland)

```bash
# Install libinput tools
sudo apt install libinput-tools

# Grant permissions
sudo chmod 666 /dev/uinput
sudo chmod 666 /dev/input/event*
```

### 4.3 macOS

```bash
# Grant Accessibility permissions
# System Preferences → Security & Privacy → Privacy → Accessibility
# Add Terminal.app or your terminal emulator
```

### 4.4 Testing Hardware Access

```bash
# Test mouse control
sentient> run test_mouse

# Test keyboard control
sentient> run test_keyboard

# Full hardware test
sentient> run test_hardware
```

---

## 5. CUSTOM PROVIDER CONNECTION

### 5.1 Configuration

```bash
# Edit settings
nano ~/.sentient/settings.toml
```

**settings.toml Example:**

```toml
[llm.custom_provider]
name = "My Custom LLM"
base_url = "https://api.my-llm.com/v1"
api_key = "${MY_LLM_API_KEY}"  # From environment variable
model = "my-model-v1"
default = true

[llm.openai]
api_key = "${OPENAI_API_KEY}"
model = "gpt-4o"
default = false

[llm.anthropic]
api_key = "${ANTHROPIC_API_KEY}"
model = "claude-3-opus"
default = false
```

### 5.2 Environment Variables

```bash
# Add to ~/.bashrc or ~/.zshrc
export MY_LLM_API_KEY="sk-your-key-here"
export OPENAI_API_KEY="sk-your-openai-key"
export ANTHROPIC_API_KEY="sk-ant-your-key"
```

### 5.3 Testing Connection

```bash
sentient> vgate test

  Provider: My Custom LLM
  Base URL: https://api.my-llm.com/v1
  Model: my-model-v1
  Status: ✅ Connected
  Latency: 45ms
```

---

## 6. FULL AUTONOMOUS MODE

### 6.1 Safety First

```rust
// L1 Sovereign Policies ALWAYS active:
- GUI control ONLY with permitted applications
- File system ACCESS IS RESTRICTED (whitelist directories)
- Process launch controlled by WHITELIST
- Dangerous commands ARE BLOCKED
- All actions logged via V-GATE
```

### 6.2 Enabling Autonomous Mode

```bash
sentient> mode autonomous

⚠️  AUTONOMOUS MODE ENABLED

Safety Policies Active:
├── GUI: Whitelist only (LibreOffice, Firefox, Terminal)
├── Files: Home directory only
├── Process: User-level only
├── Network: V-GATE filtered
└── Actions: All logged

Type 'confirm' to proceed: confirm

✅ Autonomous mode active
```

### 6.3 Self-Coding Example

```bash
sentient> task "Create a Python script that fetches weather data"

🧠 SENTIENT Task Executor
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[PLANNING] Analyzing task...
  → Requires: HTTP requests, JSON parsing, API integration
  → Skills: python_http, json_parse, weather_api
  
[CODING] Generating solution...
  → Creating: ~/sentient_workspace/weather_fetcher.py
  → Using: requests library
  
[EXECUTING] Running in sandbox...
  → Environment: Docker (python:3.11-slim)
  → Timeout: 60 seconds
  
[VERIFYING] Testing output...
  → Test passed ✓
  → Coverage: 100%
  
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ Task Complete!

📁 Created: ~/sentient_workspace/weather_fetcher.py
```

---

## 7. 7-LAYER ARCHITECTURE (L1-L7)

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                    SENTIENT OS - 7-LAYER ARCHITECTURE                        │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  L7: USER INTERFACE LAYER                                                    │
│  ├── Dashboard (Web UI)                                                      │
│  ├── CLI (sentient-shell)                                                    │
│  ├── REST API (Axum)                                                         │
│  └── 20+ Messaging Channels                                                  │
│                                                                              │
│  L6: EXECUTION LAYER                                                          │
│  ├── oasis_hands (43 Tools)                                                  │
│  ├── oasis_browser (Lightpanda)                                              │
│  └── Human Mimicry Engine                                                    │
│                                                                              │
│  L5: ORCHESTRATION LAYER                                                      │
│  ├── Multi-Agent Coordinator                                                 │
│  ├── CrewAI Integration                                                      │
│  └── AutoGen Integration                                                     │
│                                                                              │
│  L4: AGENT LAYER                                                              │
│  ├── Persona System                                                           │
│  ├── Behavioral Modes                                                         │
│  └── Session Manager                                                          │
│                                                                              │
│  L3: AI CORE LAYER                                                            │
│  ├── V-GATE Proxy                                                             │
│  ├── Universal Gateway                                                        │
│  └── Model Router                                                             │
│                                                                              │
│  L2: MEMORY LAYER                                                             │
│  ├── Memory Cube (SQLite)                                                    │
│  ├── Vector DB (ChromaDB)                                                    │
│  └── MemOS Integration                                                       │
│                                                                              │
│  L1: SOVEREIGN CORE (Security Foundation)                                    │
│  ├── GraphBit Core (Rust)                                                    │
│  ├── Guardrails Security                                                     │
│  ├── TEE Execution                                                           │
│  └── ZK-MCP Privacy                                                          │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

---

## 8. INTEGRATIONS

### 8.1 Agent Frameworks (17)

| Project | Description |
|---------|-------------|
| **AutoGen** | Microsoft Conversation Agents |
| **CrewAI** | Role-based Orchestration |
| **OpenHands** | AI Software Engineer |
| **Swarm** | OpenAI Lightweight Orchestration |
| **MetaGPT** | Company-style Agents |
| **Auto-GPT** | Autonomous Agent |
| **GPT-Engineer** | Code Generator |
| **BabyAGI** | Task Agent |
| **AgentGPT** | Browser Agent |
| **Agent-S** | Desktop Automation |
| **PraisonAI** | Multi-Agent |
| **TaskWeaver** | Code Interpreter |
| **Letta** | MemGPT Memory Agents |
| **Camel-AI** | Communicative Agents |
| **Goose** | AI Developer Assistant |
| **Agency-Swarm** | Agency Framework |
| **AutoResearch** | Research Agent |

### 8.2 LLM Frameworks (22)

| Project | Description |
|---------|-------------|
| **LangChain** | LLM Orchestration |
| **LlamaIndex** | Data Framework for LLM |
| **Phidata** | AI Agents Framework |
| **Smolagents** | Lightweight Agents |
| **Ollama** | Local LLM Runner |
| **GPT4All** | CPU-Optimized Inference |

### 8.3 Memory Systems (6)

| Project | Description |
|---------|-------------|
| **ChromaDB** | Vector Database |
| **Qdrant** | High-Performance Search |
| **Mem0** | Cross-session Memory |
| **MemOS** | Memory Operating System |

---

## 9. FAQ

### Q1: How do I add a new LLM provider?

```bash
# Edit settings
nano ~/.sentient/settings.toml

# Add provider section
[llm.my_provider]
base_url = "https://api.provider.com/v1"
api_key = "${PROVIDER_KEY}"
model = "model-name"
```

### Q2: How do I check system status?

```bash
sentient> status

SENTIENT OS v2.0.0 Status
━━━━━━━━━━━━━━━━━━━━━━━━━━
CPU: 12% (8 cores)
RAM: 2.1 GB / 16 GB
Disk: 45 GB / 500 GB
Skills: 5,587 loaded
Tools: 43 active
Memory: 1,234 entries
V-GATE: ✅ Connected
```

### Q3: How do I use local LLMs?

```bash
# Install Ollama
curl -fsSL https://ollama.com/install.sh | sh

# Pull model
ollama pull llama3

# Configure SENTIENT
sentient> config set llm.provider ollama
sentient> config set llm.model llama3
```

### Q4: What if autonomous mode is too risky?

```bash
# Use safe mode (default)
sentient> mode safe

# Or semi-autonomous with confirmation
sentient> mode semi
```

---

## 10. APPENDICES

### 10.1 File Locations

| Path | Description |
|------|-------------|
| `~/.sentient/` | Main config directory |
| `~/.sentient/memory.db` | Memory Cube database |
| `~/.sentient/settings.toml` | User settings |
| `~/.sentient/logs/` | System logs |
| `~/sentient_workspace/` | Autonomous mode workspace |

### 10.2 Environment Variables

| Variable | Description |
|----------|-------------|
| `SENTIENT_CONFIG` | Config directory path |
| `SENTIENT_LOG_LEVEL` | Log level (debug/info/warn/error) |
| `SENTIENT_VGATE_URL` | V-GATE proxy URL |
| `OPENAI_API_KEY` | OpenAI API key |
| `ANTHROPIC_API_KEY` | Anthropic API key |

### 10.3 Support Channels

| Channel | Link |
|---------|------|
| 📧 Email | support@sentient-os.ai |
| 💬 Telegram | @sentient_support |
| 🐛 Issues | [GitHub Issues](https://github.com/nexsusagent-coder/sentient-os/issues) |
| 📖 Docs | https://docs.sentient-os.ai |

---

```
    ╔══════════════════════════════════════════════════════════════════════════╗
    ║                                                                          ║
    ║   🧠 SENTIENT OS - The Operating System That Thinks                      ║
    ║                                                                          ║
    ║   37 Rust Crates │ 71 Integrated Repos │ 5587 Skills │ 43 Tools         ║
    ║                                                                          ║
    ║   Made with ❤️  by Pi                                                    ║
    ║   https://github.com/nexsusagent-coder/sentient-os                       ║
    ║                                                                          ║
    ╚══════════════════════════════════════════════════════════════════════════╝
```

---

*SENTIENT OS v2.0.0 - The Operating System That Thinks*
*Last Updated: 2026-04-08*
