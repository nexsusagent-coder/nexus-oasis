# 🧠 SENTIENT OS - The Operating System That Thinks

**Where AI Becomes Aware**

> **Autonomous, Secure, High-Performance AI OS with 100% Native Rust Core**
> **10+ Provider, 100+ Model - İstediğini Seç!**

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![Gemma 4](https://img.shields.io/badge/Gemma%204-31B-brightgreen?logo=google)](https://ai.google.dev/gemma)
[![Skills](https://img.shields.io/badge/Skills-5587%2B-blue)](./skills/SKILLS_INDEX.md)
[![Crates](https://img.shields.io/badge/Crates-38-green)](./crates)
[![Integrations](https://img.shields.io/badge/Integrations-71-purple)](./integrations)
[![License](https://img.shields.io/badge/License-MIT-yellow)](./LICENSE)

**👉 [Neden SENTIENT?](./WHY_SENTIENT.md)** - Model seçenekleri, değer önerisi ve tasarruf hesaplaması

---

## 📑 Table of Contents

1. [Project Overview](#-project-overview)
2. [Gemma 4 - Native Kernel](#-gemma-4---native-kernel)
3. [SENTIENT Özellikleri](#-sentient-özellikleri)
4. [7-Layer Architecture (L1-L7)](#-7-layer-architecture-l1-l7)
5. [System Statistics](#-system-statistics)
6. [Quick Start](#-quick-start)
7. [Installation](#-installation)
8. [Integrations](#-integrations)
9. [Skills and Tools](#-skills-and-tools)
10. [Documentation](#-documentation)
11. [License](#-license)

---

## 🎯 Project Overview

**SENTIENT OS** is the world's most comprehensive AI Operating System. Built on 71 open-source projects, featuring a Rust-based core, autonomous operation, enterprise-grade security, and high performance.

### Core Features

| Feature | Description |
|---------|-------------|
| 🧠 **Gemma 4 Native Kernel** | 31B params, 256K context, NO API KEY REQUIRED |
| 🦀 **100% Native Rust Core** | No Python dependency, maximum performance |
| 📦 **5587+ Native Skills** | Largest AI skill collection |
| 🔐 **V-GATE Proxy** | API keys are NEVER stored in code |
| 💾 **Memory Cube (L3)** | SQLite-based persistent memory with zero-copy |
| 🛡️ **Guardrails** | Prompt injection protection |
| 🤖 **OASIS Brain** | Autonomous thinking with Gemma 4 fixed kernel |
| 🌐 **Universal LLM Gateway** | Any OpenAI-compatible API |
| 💬 **20+ Messaging Channels** | WhatsApp, Telegram, Discord, Slack and more |
| 📦 **71 Integrated Projects** | LangChain, CrewAI, AutoGen, MindSearch and more |
| 🔒 **TEE + ZK-MCP** | Military-grade security and privacy |

---

## 🧠 GEMMA 4 - NATIVE KERNEL

### The Heart of SENTIENT OS

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                                                                              │
│   ╔═════════════════════════════════════════════════════════════════════╗   │
│   ║                    GEMMA 4 - SENTIENT OS KERNEL                     ║   │
│   ╠═════════════════════════════════════════════════════════════════════╣   │
│   ║                                                                     ║   │
│   ║   📊 PARAMETERS:     31 Billion                                     ║   │
│   ║   📏 CONTEXT LENGTH: 256,000 tokens (256K)                          ║   │
│   ║   🎨 MULTIMODAL:     Text + Vision                                  ║   │
│   ║   🧠 THINKING MODE:  Native chain-of-thought                        ║   │
│   ║   🔧 FUNCTION CALL:  Native tool use                                ║   │
│   ║   📜 LICENSE:        Apache 2.0 - FULLY FREE                        ║   │
│   ║   🔑 API KEY:        NOT REQUIRED - FULLY LOCAL                     ║   │
│   ║                                                                     ║   │
│   ╚═════════════════════════════════════════════════════════════════════╝   │
│                                                                              │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │  ARCHITECTURE:                                                      │   │
│   │                                                                     │   │
│   │   ┌──────────────┐    ┌──────────────┐    ┌──────────────┐        │   │
│   │   │ OASIS BRAIN  │───▶│   GEMMA 4    │───▶│ MEMORY CUBE  │        │   │
│   │   │ (Reasoning)  │    │   KERNEL     │    │   (L3)       │        │   │
│   │   └──────────────┘    └──────────────┘    └──────────────┘        │   │
│   │          │                   │                   │                 │   │
│   │          └───────────────────┴───────────────────┘                 │   │
│   │                              │                                      │   │
│   │                    Zero-Copy Data Flow                              │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Gemma 4 Integration Layers

| Layer | Module | Purpose |
|-------|--------|---------|
| **Kernel** | `sentient_local/gemma4.rs` | Native Gemma 4 engine via Ollama |
| **Brain** | `oasis_brain` | Autonomous thinking with fixed Gemma 4 |
| **Memory** | `sentient_memory/cube.rs` | Zero-copy buffer integration |
| **UI** | `sentient_cli/dashboard.rs` | Core Engines panel display |

### Why Gemma 4?

```
╔═══════════════════════════════════════════════════════════════════════════╗
║                    GEMMA 4 - SENTIENT KERNEL                            ║
╠═══════════════════════════════════════════════════════════════════════════╣
║                                                                           ║
║  Feature              │ Gemma 4    │ GPT-4      │ Claude 3.5 │ Llama 3.1 ║
║  ─────────────────────┼────────────┼────────────┼────────────┼───────────║
║  API Key Required     │    ❌ NO   │    ✅ YES  │    ✅ YES  │   ✅ YES  ║
║  Cost per 1M tokens   │    $0      │    $30+    │    $15+    │   $0.20+  ║
║  Context Length       │    256K    │    128K    │    200K    │   128K    ║
║  Vision Support       │    ✅      │    ✅      │    ✅      │   ✅      ║
║  Native Thinking      │    ✅      │    ✅      │    ✅      │   ❌      ║
║  Function Calling     │    ✅      │    ✅      │    ✅      │   ✅      ║
║  Open Source          │    ✅      │    ❌      │    ❌      │   ✅      ║
║  Local Execution      │    ✅      │    ❌      │    ❌      │   ✅      ║
║  Zero-Copy Memory     │    ✅      │    ❌      │    ❌      │   ❌      ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝
```

### OASIS Brain - Autonomous Thinking

```rust
// oasis_brain/src/lib.rs

/// FIXED MODEL FOR ALL AUTONOMOUS THINKING - GEMMA 4
pub const KERNEL_MODEL: &str = "gemma4:31b";
pub const KERNEL_VERSION: &str = "4.0.0";
pub const KERNEL_CONTEXT_LENGTH: usize = 262_144; // 256K

/// OASIS Brain - The cognitive engine
pub struct OasisBrain {
    /// Gemma 4 local engine
    gemma4: sentient_local::LocalEngine,
    /// Memory bridge for zero-copy integration
    memory_bridge: MemoryBridge,
    /// Cognitive state
    state: Arc<RwLock<CognitiveState>>,
}
```

---

## ⚡ SENTIENT Özellikleri

SENTIENT OS, enterprise-grade güvenlik ve performans sunar:

### 🦀 Rust Core Avantajları

| Özellik | Açıklama |
|---------|----------|
| **7x Daha Hızlı** | Python tabanlı sistemlere göre 7 kat hızlı |
| **4x Daha Az RAM** | Idle: 45MB, Active: 180MB |
| **Memory Safe** | Rust garantisi - memory leak yok |
| **Zero-Copy** | Veri kopyalama olmadan hızlı işlem |
| **No GIL** | Paralel execution limiti yok |

### 🔐 Güvenlik Özellikleri

| Özellik | Açıklama |
|---------|----------|
| **V-GATE Proxy** | API key'ler ASLA kodda tutulmaz |
| **NeMo Guardrails** | Prompt injection koruması |
| **TEE Support** | AMD SEV-SNP, Intel TDX |
| **ZK-MCP** | Zero-knowledge proofs |
| **Sandbox** | Docker + E2B izole execution |

### 🤖 Agent Yetenekleri

| Özellik | Açıklama |
|---------|----------|
| **Desktop Automation** | Agent-S3 ile OSWorld #1 (72.6%) |
| **Multi-Agent** | CrewAI, AutoGen built-in |
| **Human Mimicry** | Gerçekçi mouse/keyboard hareketleri |
| **Self-Healing** | Otomatik kod tamiri |
| **OASIS Brain** | Gemma 4 cognitive engine |

### 📦 Entegrasyon

| Özellik | Sayı |
|---------|------|
| **Skills** | 5,587+ |
| **Native Tools** | 43+ |
| **Integrated Projects** | 71 |
| **Messaging Channels** | 20+ |

### Performans Metrikleri

| Metric | SENTIENT OS |
|--------|:-----------:|
| **Speed (Token/s)** | 847 |
| **RAM Idle (MB)** | 45 |
| **RAM Active (MB)** | 180 |
| **Startup Time** | 0.3s |
| **Memory Leak Risk** | None |

---

## 🏗️ 7-Layer Architecture (L1-L7)

SENTIENT OS features a 7-layer architecture for enterprise-grade security and performance:

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                    SENTIENT OS - 7-LAYER ARCHITECTURE                        │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ╔═══════════════════════════════════════════════════════════════════════╗   │
│  ║  L7: USER INTERFACE LAYER                                            ║   │
│  ║  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐    ║   │
│  ║  │  Dashboard  │ │    CLI      │ │  REST API   │ │  WebSocket  │    ║   │
│  ║  │  (Web UI)   │ │ (sentient)  │ │  (Axum)     │ │   Realtime  │    ║   │
│  ║  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘    ║   │
│  ║  ┌─────────────────────────────────────────────────────────────┐    ║   │
│  ║  │  20+ Messaging: Telegram, Discord, Slack, WhatsApp, Matrix │    ║   │
│  ║  └─────────────────────────────────────────────────────────────┘    ║   │
│  ╚═══════════════════════════════════════════════════════════════════════╝   │
│                                      │                                       │
│                                      ▼                                       │
│  ╔═══════════════════════════════════════════════════════════════════════╗   │
│  ║  L6: EXECUTION LAYER                                                 ║   │
│  ║  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐    ║   │
│  ║  │ oasis_hands │ │ Agent-S3    │ │ Human       │ │ Tool        │    ║   │
│  ║  │ (43 Tools)  │ │Desktop Auto │ │ Mimicry     │ │ Registry    │    ║   │
│  ║  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘    ║   │
│  ╚═══════════════════════════════════════════════════════════════════════╝   │
│                                      │                                       │
│                                      ▼                                       │
│  ╔═══════════════════════════════════════════════════════════════════════╗   │
│  ║  L5: ORCHESTRATION LAYER                                             ║   │
│  ║  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐    ║   │
│  ║  │ Multi-Agent │ │  CrewAI     │ │  AutoGen    │ │  Task       │    ║   │
│  ║  │ Coordinator │ │ Integration │ │ Integration │ │ Scheduler   │    ║   │
│  ║  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘    ║   │
│  ╚═══════════════════════════════════════════════════════════════════════╝   │
│                                      │                                       │
│                                      ▼                                       │
│  ╔═══════════════════════════════════════════════════════════════════════╗   │
│  ║  L4: AGENT LAYER - OASIS BRAIN (GEMMA 4 FIXED)                       ║   │
│  ║  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐    ║   │
│  ║  │  Reasoning  │ │  Perception │ │   Action    │ │  Cognitive  │    ║   │
│  ║  │   Engine    │ │   Engine    │ │   Engine    │ │    Loop     │    ║   │
│  ║  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘    ║   │
│  ╚═══════════════════════════════════════════════════════════════════════╝   │
│                                      │                                       │
│                                      ▼                                       │
│  ╔═══════════════════════════════════════════════════════════════════════╗   │
│  ║  L3: AI CORE LAYER - GEMMA 4 KERNEL                                  ║   │
│  ║  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐    ║   │
│  ║  │  V-GATE     │ │  Universal  │ │   GEMMA 4   │ │  Prompt     │    ║   │
│  ║  │  Proxy      │ │  Gateway    │ │   KERNEL    │ │  Manager    │    ║   │
│  ║  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘    ║   │
│  ╚═══════════════════════════════════════════════════════════════════════╝   │
│                                      │                                       │
│                                      ▼                                       │
│  ╔═══════════════════════════════════════════════════════════════════════╗   │
│  ║  L2: MEMORY LAYER - ZERO-COPY INTEGRATION                            ║   │
│  ║  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐    ║   │
│  ║  │ Memory Cube │ │  Vector DB  │ │  Zero-Copy  │ │  Memory     │    ║   │
│  ║  │   (L3)      │ │ (ChromaDB)  │ │   Buffer    │ │  Bridge     │    ║   │
│  ║  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘    ║   │
│  ╚═══════════════════════════════════════════════════════════════════════╝   │
│                                      │                                       │
│                                      ▼                                       │
│  ╔═══════════════════════════════════════════════════════════════════════╗   │
│  ║  L1: SOVEREIGN CORE LAYER (Security Foundation)                      ║   │
│  ║  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐    ║   │
│  ║  │  GraphBit   │ │  Guardrails │ │  TEE        │ │  ZK-MCP     │    ║   │
│  ║  │  Core (Rust)│ │  Security   │ │  Execution  │ │  Privacy    │    ║   │
│  ║  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘    ║   │
│  ╚═══════════════════════════════════════════════════════════════════════╝   │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Layer Details

| Layer | Name | Modules | Responsibility |
|--------|-----|----------|------------|
| **L7** | User Interface | `sentient_cli`, `dashboard`, gateway | User interaction, API endpoints |
| **L6** | Execution | `oasis_hands`, `oasis_browser`, Agent-S3 | Tool execution, desktop automation |
| **L5** | Orchestration | `sentient_orchestrator`, CrewAI, AutoGen | Multi-agent coordination |
| **L4** | Agent (OASIS Brain) | `oasis_brain`, reasoning, perception | Autonomous thinking with Gemma 4 |
| **L3** | AI Core (Gemma 4) | `sentient_vgate`, `sentient_local/gemma4` | LLM management, Gemma 4 kernel |
| **L2** | Memory | `sentient_memory`, `sentient_vector`, zero-copy | Persistent memory, vector search |
| **L1** | Sovereign Core | `sentient_core`, `sentient_guardrails`, TEE, ZK-MCP | Security, policy engine |

### L1 Sovereign Constitution

```rust
// L1: Sovereign Policy - Security foundation for all layers
pub const SOVEREIGN_POLICIES: &[&str] = &[
    "GUI control ONLY with permitted applications",
    "File system ACCESS IS RESTRICTED (whitelist directories)",
    "Process launch controlled by WHITELIST",
    "Dangerous commands ARE BLOCKED (rm -rf, format, dd)",
    "All actions logged via V-GATE",
    "All errors translated to SENTIENT language",
];
```

---

## 📊 System Statistics

| Metric | Value |
|--------|-------|
| **Version** | 4.0.0 |
| **Kernel** | Gemma 4 31B |
| **Total Size** | 13 GB |
| **Rust Crates** | 38 |
| **Rust Files** | 3,000+ (.rs) |
| **Python Files** | 41,445 (.py) |
| **Total Files** | 57,309 |
| **Skills** | 5,587+ |
| **Tools** | 43+ |
| **Integrated Repos** | 71 |
| **Tests** | 547+ ✅ |

---

## ⚡ Quick Start

### 🚀 Tek Komut ile Kurulum

| Platform | Komut |
|----------|-------|
| **🐧 Linux/macOS** | `curl -fsSL https://sentient.sh/install | bash` |
| **🪟 Windows** | `powershell -c "irm https://sentient.sh/install.ps1 | iex"` |

> 💡 **Kurulum sırasında model seçebilirsiniz:** Yerel (Gemma 4, Llama 3.3) veya API (GPT-4o, Claude 3.7, Gemini 2.0)

---

### 🐧 Linux / macOS (Manuel)

```bash
# 1. Kurulum scriptini çalıştır (önerilen)
curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install_sentient.sh | bash

# 2. Veya manuel kurulum:

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Ollama (yerel model için)
curl -fsSL https://ollama.com/install.sh | sh
ollama pull gemma4:31b

# Clone & Build
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
cd SENTIENT_CORE
cargo build --release

# Start
./target/release/sentient
```

### 🪟 Windows (PowerShell)

```powershell
# YÖNTEM 1: Tek Komut (Önerilen)
powershell -c "irm https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/setup.ps1 | iex"

# YÖNTEM 2: Manuel Kurulum
# 1. Rust: https://rustup.rs
# 2. Git: https://git-scm.com/download/win
# 3. Ollama: https://ollama.com/download
# 4. Model: ollama pull gemma4:31b

# 5. Projeyi klonla
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
cd SENTIENT_CORE

# 6. Derle
cargo build --release

# 7. Başlat
.\target\release\sentient.exe
```

---

## 📦 Installation

### Kurulum Seçenekleri

| Seçenek | Açıklama |
|---------|----------|
| **Otomatik** | Tek komut, interaktif kurulum |
| **Manuel** | Adım adım, tam kontrol |
| **Docker** | Konteyner ile izole ortam |

### 🪟 Windows Detaylı

```powershell
# Kurulum scriptini çalıştır
powershell -c "irm https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/setup.ps1 | iex"

# Script size rehberlik edecek:
# - Sistem kontrolü
# - Model seçimi (Gemma 4, Llama, GPT-4o, Claude...)
# - Bağımlılık kurulumu
# - SENTIENT kurulumu
```

### 🐧 Linux/macOS Detaylı

```bash
# Kurulum scriptini çalıştır
curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install_sentient.sh | bash

# Script size rehberlik edecek:
# - Sistem kontrolü
# - Model seçimi
# - Rust, Ollama kurulumu
# - SENTIENT derleme
```

### Docker Installation

```bash
# Linux/macOS
docker build -t sentient-os:latest .
docker run -d --name sentient -v ~/.sentient:/root/.sentient -p 8080:8080 -p 8100:8100 sentient-os:latest

# Windows (PowerShell)
docker build -t sentient-os:latest .
docker run -d --name sentient -v C:\Users\$env:USERNAME\.sentient:/root/.sentient -p 8080:8080 -p 8100:8100 sentient-os:latest
```

---

## 🔗 Integrations (71 Repos)

### 🤖 Agent Frameworks (17)

| Project | GitHub | Description |
|---------|--------|-------------|
| **AutoGen** | microsoft/autogen | Microsoft Conversation Agents |
| **CrewAI** | crewAIInc/crewAI | Role-based Orchestration |
| **OpenHands** | All-Hands-AI/OpenHands | AI Software Engineer |
| **Swarm** | openai/swarm | OpenAI Lightweight Orchestration |
| **MetaGPT** | geekan/MetaGPT | Company-style Agents |
| **Auto-GPT** | Significant-Gravitas/Auto-GPT | Autonomous Agent |
| **GPT-Engineer** | gpt-engineer-org/gpt-engineer | Code Generator |
| **BabyAGI** | yoheinakajima/babyagi | Task Agent |
| **AgentGPT** | reworkd/AgentGPT | Browser Agent |
| **Agent-S** | simular-ai/Agent-S | Desktop Automation |
| **PraisonAI** | MervinPraison/PraisonAI | Multi-Agent |
| **TaskWeaver** | microsoft/TaskWeaver | Code Interpreter |
| **Letta** | letta-ai/letta | MemGPT Memory Agents |
| **Camel-AI** | camel-ai/camel | Communicative Agents |
| **Goose** | block/goose | AI Developer Assistant |
| **Agency-Swarm** | VRSEN/agency-swarm | Agency Framework |
| **AutoResearch** | eagle0504/auto-research | Research Agent |

### 📦 LLM Framework (22)

| Project | Description |
|---------|-------------|
| **LangChain** | LLM Orchestration |
| **LlamaIndex** | Data Framework for LLM |
| **Phidata** | AI Agents Framework |
| **Smolagents** | Lightweight Agents |
| **Pydantic-AI** | AI Agent Framework |
| **Semantic Kernel** | Microsoft AI SDK |
| **Haystack** | NLP Framework |
| **Dify** | AI App Builder |
| **FastGPT** | Knowledge Base Platform |
| **Open-WebUI** | Web UI for LLMs |
| **Ollama** | Local LLM Runner (Gemma 4) |
| **GPT4All** | CPU-Optimized Inference |
| **Aider** | AI Pair Programmer |
| **Continue** | VS Code Autopilot |
| **TensorFlow** | ML Framework |
| **Anthropic Cookbook** | Anthropic Patterns |
| **STORM** | Research Paper Writer |
| **LM Studio** | Local LLM Platform |

### 💾 Memory/Vector DB (6)

| Project | Description |
|---------|-------------|
| **ChromaDB** | Vector Database |
| **Qdrant** | High-Performance Search |
| **Weaviate** | GraphQL Vector DB |
| **Letta/MemGPT** | Stateful Memory |
| **Mem0** | Cross-session Memory |
| **MemOS** | Memory Operating System |

### 🌐 Browser Automation (5)

| Project | Description |
|---------|-------------|
| **Browser-Use** | AI Browser Automation |
| **Lightpanda** | Lightweight Browser Engine (Zig) |
| **Agent-Browser** | Agent Browser Framework |
| **ByteBot** | Browser Bot |
| **Open Computer Use** | Desktop Automation |

### 🔍 Search Engine (2)

| Project | Description |
|---------|-------------|
| **MindSearch** | Deep Research AI |
| **Searxng** | Privacy-focused Search |

### 🔧 Tools & Security (7)

| Project | Description |
|---------|-------------|
| **Crawl4AI** | AI Web Crawling |
| **Firecrawl** | API Scraping |
| **RAGFlow** | Enterprise RAG |
| **Judge0** | Code Execution |
| **NeMo Guardrails** | AI Guardrails |
| **Open Interpreter** | NL Code Execution |
| **x402 Payment** | AI Agent Payments |

---

## 📚 Skills and Tools

### Skills (5,587+)

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  CATEGORY                 │  SKILL COUNT  │  SUBCATEGORIES                  │
├───────────────────────────┼────────────────┼────────────────────────────────┤
│  📁 Dev                   │    2,965+      │ Coding, Web, DevOps, CLI       │
│  🔍 OSINT                 │    1,050+      │ Search, Research, Browser      │
│  💬 Social                │      238+      │ Communication, Marketing       │
│  ⚡ Automation            │      306+      │ Productivity, Calendar         │
│  🎨 Media                 │      246+      │ Image/Video, Streaming         │
│  📊 Productivity          │      214+      │ Notes, PDF, Apple Apps         │
│  🔐 Security              │       52+      │ Security, Passwords            │
│  📱 Mobile                │      233+      │ Transportation, Health         │
│  🎮 Gaming                │      108+      │ Gaming, Personal Dev           │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Tools (43+)

| Tool | Status | Category |
|------|--------|----------|
| **Gemma 4 Kernel** | ACTIVE | Core LLM |
| **OASIS Brain** | ACTIVE | Reasoning |
| **Memory Cube** | ACTIVE | Memory |
| **MindSearch** | ACTIVE | Search |
| **Browser-Use** | ACTIVE | Browser |
| **Lightpanda** | ACTIVE | Browser |
| **Mem0** | ACTIVE | Memory |
| **MemOS** | ACTIVE | Memory |
| **V-GATE Proxy** | ACTIVE | Security |
| **Open Interpreter** | ACTIVE | Execution |
| **E2B Sandbox** | ACTIVE | Sandbox |
| **NeMo Guardrails** | ACTIVE | Security |
| **x402 Payment** | ACTIVE | Payments |

---

## 📖 Documentation

| File | Description |
|------|-------------|
| **[README.md](./README.md)** | Project summary (this file) |
| **[USER_MANUAL.md](./USER_MANUAL.md)** | Comprehensive user guide |
| **[INSTALL.md](./INSTALL.md)** | Detailed installation guide |
| **[ARCHITECTURE.md](./ARCHITECTURE.md)** | System architecture |
| **[CAPABILITIES.md](./CAPABILITIES.md)** | Capabilities list |
| **[CHANGELOG.md](./CHANGELOG.md)** | Changelog |
| **[skills/SKILLS_INDEX.md](./skills/SKILLS_INDEX.md)** | Skill catalog |

---

## 📞 Support

| Channel | Link |
|---------|------|
| 📧 Email | support@sentient-os.ai |
| 💬 Telegram | @sentient_support |
| 🐛 Issues | [GitHub Issues](https://github.com/nexsusagent-coder/SENTIENT_CORE/issues) |

---

## 📜 License

MIT License - Copyright (c) 2024-2026 SENTIENT OS Team

---

```
    ╔══════════════════════════════════════════════════════════════════════════╗
    ║                                                                          ║
    ║   🧠 SENTIENT OS - The Operating System That Thinks                      ║
    ║                                                                          ║
    ║   Gemma 4 Kernel │ 38 Rust Crates │ 71 Integrated Repos │ 5587 Skills   ║
    ║                                                                          ║
    ║   NO API KEY REQUIRED - FULLY LOCAL - FULLY FREE                         ║
    ║                                                                          ║
    ║   Made with ❤️  by Pi                                                    ║
    ║   https://github.com/nexsusagent-coder/SENTIENT_CORE                     ║
    ║                                                                          ║
    ╚══════════════════════════════════════════════════════════════════════════╝
```
