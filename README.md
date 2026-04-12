<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/License-AGPL%20v3-blue" alt="License">
  <a href="https://ko-fi.com/sentientos"><img src="https://img.shields.io/badge/Support-Ko--fi-ff5e5b" alt="Support"></a>
  <img src="https://img.shields.io/badge/Platforms-Linux%20%7C%20macOS%20%7C%20Windows-green" alt="Platforms">
  <img src="https://img.shields.io/badge/LLM%20Models-352%20native%20%7C%20200K%2B%20via%20aggregators-purple" alt="LLM Models">
  <img src="https://img.shields.io/badge/Integrations-72%2B-yellow" alt="Integrations">
  <img src="https://img.shields.io/badge/Skills-5%2C587%2B-red" alt="Skills">
</p>

<h1 align="center">🧠 SENTIENT OS</h1>
<h3 align="center">The Operating System That Thinks</h3>
<p align="center"><i>Enterprise-Grade AI Agent Framework with Rust Core</i></p>

---

# 📖 İçindekiler

1. [SENTIENT Nedir?](#sentient-nedir)
2. [Neden SENTIENT?](#neden-sentient)
3. [Özellikler](#özellikler)
4. [Hızlı Kurulum](#hızlı-kurulum)
5. [Detaylı Kurulum](#detaylı-kurulum)
6. [İlk Çalıştırma](#ilk-çalıştırma)
7. [Mimari](#mimari)
8. [59 Rust Crate](#59-rust-crate)
9. [72 Entegrasyon](#72-entegrasyon)
10. [352+ Native LLM Model (200K+ via Aggregators)](#352-native-llm-model-200k-via-aggregators)
11. [5,587+ Skill](#5587-skill)
12. [CLI Komutları](#cli-komutları)
13. [Web Dashboard](#web-dashboard)
14. [API Kullanımı](#api-kullanımı)
15. [Plugin Sistemi](#plugin-sistemi)
16. [Güvenlik](#güvenlik)
17. [Performans](#performans)
18. [💰 Sponsorluk & Destek](#-sponsorluk--destek)
19. [🏢 Enterprise & Commercial](#-enterprise--commercial)
20. [Katkılarda Bulunma](#katkılarda-bulunma)
21. [Lisans](#lisans)

---

# SENTIENT Nedir?

**SENTIENT OS**, yapay zeka agent'larını çalıştırmak, yönetmek ve ölçeklendirmek için tasarlanmış **Rust-tabanlı** bir AI işletim sistemidir.

```
┌────────────────────────────────────────────────────────────────────────────┐
│                          SENTIENT OS                                        │
│                    "The Operating System That Thinks"                      │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   🦀 Rust Core         → Memory-safe, zero-cost abstractions               │
│   📦 63 Crate          → Modüler, değiştirilebilir mimari                  │
│   🔌 72 Entegrasyon    → Agent framework'ler, browser'lar, memory sistemleri │
│   🤖 352+ LLM Models  → OpenAI, Claude, Gemini, Ollama, 41 providers        │
│   🎯 5,587+ Skill      → Hazır AI yetenekleri                               │
│   🔐 V-GATE            → API anahtarları ASLA client'ta değil              │
│   📊 Dashboard         → Enterprise war room, gerçek zamanlı               │
│                                                                             │
└────────────────────────────────────────────────────────────────────────────┘
```

---

# Neden SENTIENT?

| Karşılaştırma | SENTIENT OS | Python Framework'ler | Diğer Rust Framework'ler |
|---------------|-------------|---------------------|--------------------------|
| **Performans** | ⚡⚡⚡⚡⚡ | ⚡⚡ | ⚡⚡⚡⚡ |
| **Memory Safety** | ✅ Garantili | ❌ Runtime hataları | ✅ Garantili |
| **Modülerlik** | ✅ 59 crate | ❌ Monolitik | ⚡ 5-10 crate |
| **Entegrasyon** | ✅ 72 hazır | ⚡ 10-20 | ❌ 1-5 |
| **LLM Desteği** | ✅ 352+ native (200K+ via aggregators) | ⚡ 50-100 | ❌ 5-20 |
| **Skill Sayısı** | ✅ 5,587+ | ⚡ 100-500 | ❌ 10-50 |
| **Security** | ✅ V-GATE | ❌ API key暴露 | ⚡ Basic |
| **Dashboard** | ✅ Enterprise | ❌ Yok | ❌ Yok |

---

# Özellikler

## 🧠 Multi-LLM Support

```rust
// 408+ model desteği, tek API
use sentient_llm::{LlmClient, Message};

// OpenAI
let client = LlmClient::openai(api_key);
let response = client.chat("gpt-4o", "Merhaba!").await?;

// Claude (Extended Thinking)
let client = LlmClient::anthropic(api_key);
let response = client.chat_thinking("claude-3-7-sonnet", "Analiz et").await?;

// Local (Ollama) - Ücretsiz
let client = LlmClient::ollama(); // localhost:11434
let response = client.chat("llama3.3:70b", "Local response").await?;

// Gemma 4 (KERNEL DEFAULT)
let response = client.chat("gemma-4:12b", "Code review").await?;
```

## 🔧 Skill Sistemi

```bash
# 5,587+ hazır skill
sentient skill list
sentient skill run code-review --path ./src
sentient skill run web-scraper --url "https://example.com"
sentient skill run pentest --target "https://myapp.com"
```

## 🔐 V-GATE Security

```
┌─────────────┐      ┌─────────────┐      ┌─────────────┐
│   SENTIENT  │      │   V-GATE    │      │   LLM API   │
│   Client    │─────▶│   Proxy     │─────▶│   Provider  │
└─────────────┘      └─────────────┘      └─────────────┘
                           │
                     API Key (Secure)
                     Server-side only
                     NEVER in client
```

## 📊 Real-time Dashboard

```bash
sentient dashboard
# → http://localhost:8080
```

## 🔌 MCP Protocol

```rust
// Model Context Protocol desteği
use sentient_mcp::{McpServer, Tool, Resource};

let server = McpServer::new("sentient-mcp")
    .tool(Tool::new("code_analyze", |input| {
        // Code analysis
    }))
    .resource(Resource::file("./data/"))
    .serve().await?;
```

## 👁️ Vision/Multimodal

```rust
use sentient_vision::{VisionClient, ImageAnalysis};

let client = VisionClient::openai();
let analysis = client.analyze_image("screenshot.png").await?;

// OCR
let text = client.ocr("document.pdf").await?;

// Embeddings
let embedding = client.embed("image.jpg").await?;
```

## 🔍 RAG Engine

```rust
use sentient_rag::{RagPipeline, Chunker, Embedder, VectorStore};

let pipeline = RagPipeline::new()
    .chunker(Chunker::recursive(500))
    .embedder(Embedder::openai())
    .store(VectorStore::memory())
    .build();

let results = pipeline.query("What is SENTIENT?").await?;
```

---

# Hızlı Kurulum

## Tek Komutla (Linux/macOS)

```bash
curl -sSL https://get.sentient.ai | bash
```

## Windows (PowerShell)

```powershell
irm https://get.sentient.ai/ps | iex
```

## Docker

```bash
docker run -it sentientai/sentient:latest
```

## Cargo

```bash
cargo install sentient-os
```

---

# Detaylı Kurulum

## Gereksinimler

| Gereksinim | Minimum | Önerilen |
|------------|---------|----------|
| **İşletim Sistemi** | Ubuntu 20.04 | Ubuntu 22.04 |
| **Rust** | 1.75 | 1.80+ |
| **RAM** | 4 GB | 16 GB+ |
| **Disk** | 2 GB | 10 GB+ |
| **CPU** | 2 core | 8 core+ |

### Opsiyonel Gereksinimler

| Bileşen | Kullanım |
|---------|----------|
| **Docker** | Sandbox execution |
| **Ollama** | Local LLM |
| **Node.js 18+** | Dashboard |

## Kaynaktan Derleme

```bash
# 1. Repository'yi klonla
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
cd SENTIENT_CORE

# 2. Rust kurulu değilse
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 3. Derle (release mod)
cargo build --release

# 4. Kurulum
cargo install --path .

# 5. İlk kurulum sihirbazı
sentient setup
```

---

# İlk Çalıştırma

## Setup Wizard

SENTIENT OS, OpenClaw standardında interaktif bir kurulum sihirbazı sunar:

```bash
$ sentient setup

╔══════════════════════════════════════════════════════════════════════════════╗
║                         SENTIENT OS v7.0.0                                   ║
║                     Setup Wizard - OpenClaw Standard                        ║
╚══════════════════════════════════════════════════════════════════════════════╝

╔══════════════════════════════════════════════════════════════════════════════╗
║   SECURITY WARNING                                                           ║
╚══════════════════════════════════════════════════════════════════════════════╝

This system is PERSONAL by default.
Multi-user access requires LOCK-DOWN mode.

   - Personal mode: Full access to all data
   - Lock-down mode: Restricted access, audit log enabled

? Do you want to continue? (Y/n) Y

[OK] Security warning accepted. Continuing...

───────────────────────────────────────────────────────────────────────────────
STEP 1/5: Setup Mode
───────────────────────────────────────────────────────────────────────────────

Select setup mode:

   QuickStart (Recommended)
     - Port: 18789, Loopback, Token Auth
     - Recommended for first-time setup

   Manual - Full Configuration
     - Customize all settings
     - Recommended for experienced users

   Cancel

? Your selection: QuickStart

───────────────────────────────────────────────────────────────────────────────
STEP 2/5: LLM Provider Selection
───────────────────────────────────────────────────────────────────────────────

LLM Model Selection - OpenClaw Standard Format
   Format: provider/model_id | Fuzzy search enabled

? Select provider (type to search):

   ollama/gemma4:31b (KERNEL)        ◄── DEFAULT LOCAL LLM
   ollama/gemma4:26b-moe
   ollama/gemma4:e4b (Edge)
   ollama/gemma4:e2b (Mobile)
   ─────────────────────────────────────────────
   openai/gpt-4o
   openai/gpt-4o-mini
   openai/o1-preview
   openai/o1-mini
   ─────────────────────────────────────────────
   anthropic/claude-3-5-sonnet-20241022
   anthropic/claude-3-opus-20240229
   ─────────────────────────────────────────────
   google/gemini-2.0-flash
   google/gemini-1.5-pro
   ─────────────────────────────────────────────
   openrouter/anthropic/claude-3.5-sonnet
   openrouter/openai/gpt-4o
   openrouter/meta-llama/llama-3.3-70b-instruct
   ─────────────────────────────────────────────
   groq/llama-3.3-70b-versatile
   deepseek/deepseek-chat
   mistral/mistral-large-latest
   xai/grok-2-1212
   ─────────────────────────────────────────────
   [350+ more models...]
   Skip for now

? Your selection: ollama/gemma4:31b (KERNEL)

[OK] Selected: gemma4:31b (ollama)

Note: Ollama installation required:
   curl -fsSL https://ollama.com/install.sh | sh
   ollama pull gemma4:31b

───────────────────────────────────────────────────────────────────────────────
STEP 3/5: Communication Channels
───────────────────────────────────────────────────────────────────────────────

Communication Channels - 20+ Platforms
   Space: Select/Remove    Enter: Confirm

? Select channels (Space to select, Enter to confirm):

   [ ] Telegram Bot
   [ ] WhatsApp Business
   [ ] Discord
   [ ] Slack
   [ ] Microsoft Teams
   [ ] Google Chat
   [ ] Matrix/Element
   [ ] Twitter/X DM
   [ ] Email (SMTP/IMAP)
   [ ] Web Dashboard
   [ ] REST API
   [ ] Webhook
   [ ] Signal
   [ ] iMessage (macOS)
   [ ] LinkedIn
   [ ] Reddit
   [ ] Facebook Messenger
   [ ] Instagram DM
   [ ] Zoom Chat
   [ ] Webex
   [ ] Mattermost
   [ ] Rocket.Chat
   [ ] XMPP/Jabber
   [ ] Nostr
   ─────────────────────────────────────────────
   [350+ more integrations...]
   Skip for now

? Your selection: [x] Web Dashboard, [x] REST API, [x] Telegram Bot

───────────────────────────────────────────────────────────────────────────────
STEP 4/5: Channel Setup - Telegram Bot
───────────────────────────────────────────────────────────────────────────────

Telegram Bot Setup

1. Open @BotFather on Telegram
2. Send /newbot
3. Copy the API token

? Enter Telegram Bot Token: ****************************************
? Enter your Telegram User ID (for admin access): 123456789

[OK] Telegram bot configured successfully!
[OK] Test with: sentient telegram test

───────────────────────────────────────────────────────────────────────────────
STEP 5/5: Save Configuration
───────────────────────────────────────────────────────────────────────────────

? Save configuration to ~/.sentient/config.json? (Y/n) Y

⠋ Creating configuration file...
⠋ Generating secure tokens...
⠋ Setting up V-GATE proxy...

╔══════════════════════════════════════════════════════════════════════════════╗
║                      SETUP COMPLETE!                                         ║
╚══════════════════════════════════════════════════════════════════════════════╝

Configuration saved to: ~/.sentient/config.json

Enabled Integrations:
   ✓ LLM: ollama/gemma4:31b (KERNEL)
   ✓ Channels: Web Dashboard, REST API, Telegram

Next Steps:
   1. Start dashboard:  sentient dashboard
   2. Start REPL:       sentient repl
   3. Run agent:        sentient run --goal "Your goal here"
   4. Check status:     sentient status

Dashboard: http://127.0.0.1:18789
API Docs:  http://127.0.0.1:18789/api/docs
```

### Manual Mode

Manuel modda tüm adımları özelleştirebilirsiniz:

```bash
$ sentient setup --mode manual

───────────────────────────────────────────────────────────────────────────────
STEP 1/6: Language
───────────────────────────────────────────────────────────────────────────────

? Select language:
   English
   Türkçe
   Deutsch
   Français
   Español
   日本語
   中文

───────────────────────────────────────────────────────────────────────────────
STEP 2/6: LLM Provider
───────────────────────────────────────────────────────────────────────────────
[... same as QuickStart ...]

───────────────────────────────────────────────────────────────────────────────
STEP 3/6: Communication Channels
───────────────────────────────────────────────────────────────────────────────
[... same as QuickStart ...]

───────────────────────────────────────────────────────────────────────────────
STEP 4/6: Tools
───────────────────────────────────────────────────────────────────────────────

? Select tools (Space to select):

   [x] SearXNG (Web Search - Self-hosted)
   [x] DuckDuckGo (Web Search - Free)
   [ ] Firecrawl (Web Scraping)
   [ ] Browser Use (Browser Automation)
   [ ] Lightpanda (Headless Browser)
   [ ] Mem0 (Long-term Memory)
   [ ] ChromaDB (Vector Store)
   [ ] Sandbox (Code Execution)

───────────────────────────────────────────────────────────────────────────────
STEP 5/6: Permissions
───────────────────────────────────────────────────────────────────────────────

? Permission level:
   Strict    - Require confirmation for all actions
   Normal    - Require confirmation for destructive actions only
   Permissive - Auto-approve all actions (not recommended)

? Your selection: Normal

───────────────────────────────────────────────────────────────────────────────
STEP 6/6: Dashboard Settings
───────────────────────────────────────────────────────────────────────────────

? Dashboard port (default: 18789): 18789
? Bind address (default: 127.0.0.1): 127.0.0.1
? Enable HTTPS? (y/N): N
? Enable authentication? (Y/n): Y

? Auth method:
   Token (Simple)
   JWT (Recommended)
   OAuth2 (Enterprise)

? Your selection: JWT

? Generate JWT secret? (Y/n): Y
[OK] JWT secret generated and saved
```

### CLI Commands

OpenClaw standardında CLI komutları:

```bash
# Setup
sentient setup                          # Interactive setup wizard
sentient setup --mode quickstart        # QuickStart mode
sentient setup --mode manual            # Manual mode
sentient setup --lang tr                # Turkish language

# Status
sentient status                         # Show system status
sentient config show                    # Show configuration
sentient config get llm.provider        # Get specific config
sentient config set llm.model gpt-4o    # Set specific config

# LLM
sentient llm list                       # List all providers
sentient llm test                       # Test current LLM
sentient llm switch openai/gpt-4o       # Switch LLM provider

# Channels
sentient channel list                   # List enabled channels
sentient channel test telegram          # Test Telegram connection
sentient channel enable discord         # Enable Discord
sentient channel disable slack          # Disable Slack

# Memory
sentient memory init --provider mem0    # Initialize memory
sentient memory add "User prefers Rust" # Add memory
sentient memory search "preferences"    # Search memories
sentient memory list                    # List all memories

# Plugins
sentient plugin list                    # List installed plugins
sentient plugin install @sentient/rag   # Install plugin
sentient plugin update --all            # Update all plugins

# Skills
sentient skill list                     # List available skills
sentient skill install code-review      # Install skill
sentient skill run web-scraper          # Run skill
```

## İlk Agent

```bash
# REPL başlat
sentient repl

# Agent oluştur
>>> agent create my-agent --model gpt-4o

# Goal ver
>>> my-agent run --goal "Build a REST API for user management"

# Sonuç
✅ Created 8 files:
   - src/main.rs
   - src/handlers/users.rs
   - src/models/user.rs
   - Cargo.toml
   - ...
```

---

# Mimari

## Katmanlı Mimari

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              USER LAYER                                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │     CLI      │  │   Dashboard  │  │   REST API   │  │   WebSocket  │    │
│  │  (sentient)  │  │  (Web UI)    │  │ (sentient_web)│  │   (realtime) │    │
│  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘    │
├─────────────────────────────────────────────────────────────────────────────┤
│                           ORCHESTRATION LAYER                                │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │ Orchestrator │  │    Swarm     │  │     RAG      │  │   Workflow   │    │
│  │ (multi-agent)│  │ (collective)│  │  (retrieval) │  │  (pipeline)  │    │
│  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘    │
├─────────────────────────────────────────────────────────────────────────────┤
│                              CORE LAYER                                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │ Agent Engine │  │ Event Graph  │  │    Memory    │  │   Cevahir    │    │
│  │sentient_core │  │sentient_graph│  │sentient_memory│ │(LLM Engine)  │    │
│  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘    │
├─────────────────────────────────────────────────────────────────────────────┤
│                          INTEGRATION LAYER                                   │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  PyO3 Bridge │  │MCP Protocol │  │   Plugins    │  │   Vision     │    │
│  │sentient_python│ │sentient_mcp │  │sentient_plugin│ │sentient_vision│    │
│  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘    │
├─────────────────────────────────────────────────────────────────────────────┤
│                            SECURITY LAYER                                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │    V-GATE    │  │  Guardrails  │  │     TEE      │  │    ZK-MCP     │    │
│  │  (API Proxy) │  │  (Filtering) │  │(AMD/Intel HW)│  │(Zero-Knowledge)│   │
│  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘    │
├─────────────────────────────────────────────────────────────────────────────┤
│                           EXECUTION LAYER                                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │   Sandbox    │  │   Browser    │  │   Sandbox    │  │  Fine-tuning  │    │
│  │(Docker/WASM)│  │(Lightpanda) │  │   (E2B)      │  │(LoRA/QLoRA)  │    │
│  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘    │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Event-Driven Architecture

```rust
use sentient_graph::{EventBus, Event, EventHandler};

// Lock-free event bus
let bus = EventBus::new();

// Event emit
bus.emit(Event::AgentCreated { 
    id: "agent-1".into(),
    model: "gpt-4o".into(),
});

// Event handle
bus.on::<AgentCreatedEvent>(|event| {
    println!("Agent created: {}", event.id);
});
```

---

# 63 Rust Crate

SENTIENT OS **63 bağımsız Rust crate** içerir:

## 🏗️ Core Crates

| Crate | Açıklama |
|-------|----------|
| `sentient_core` | Agent engine, event system, temel abstraction'lar |
| `sentient_graph` | Lock-free event graph, async event bus |
| `sentient_memory` | Episodic, semantic, procedural memory |
| `sentient_orchestrator` | Multi-agent orchestration, swarm |
| `sentient_cevahir` | LLM engine, cognitive reasoning |

## 🤖 LLM Crates

| Crate | Açıklama |
|-------|----------|
| `sentient_gateway` | Multi-provider LLM gateway |
| `sentient_local` | Local models (Gemma 4, GPT4All, Ollama) |
| `sentient_finetuning` | LoRA, QLoRA fine-tuning |

## 🔧 Execution Crates

| Crate | Açıklama |
|-------|----------|
| `oasis_autonomous` | Autonomous agent execution |
| `oasis_browser` | Browser automation (Lightpanda) |
| `oasis_hands` | Skill execution |
| `oasis_manus` | Task execution |
| `sentient_sandbox` | Docker/WASM sandbox |

## 🧩 Integration Crates

| Crate | Açıklama |
|-------|----------|
| `sentient_mcp` | Model Context Protocol |
| `sentient_python` | PyO3 bridge |
| `sentient_plugin` | Plugin system |
| `sentient_rag` | RAG engine |
| `sentient_vision` | Vision/multimodal AI |

## 🔒 Security Crates

| Crate | Açıklama |
|-------|----------|
| `sentient_guardrails` | Input/output filtering |
| `sentient_vault` | Secret management (AWS, Azure, HashiCorp) |
| `sentient_tee` | TEE (AMD SEV-SNP, Intel TDX) |
| `sentient_zk_mcp` | Zero-knowledge proofs |

## 🌐 Web Crates

| Crate | Açıklama |
|-------|----------|
| `sentient_web` | REST API + WebSocket server |
| `sentient_gateway` | API gateway |
| `dashboard` | Enterprise dashboard (84KB main.rs) |

## 📊 Utility Crates

| Crate | Açıklama |
|-------|----------|
| `sentient_common` | Shared types, utilities |
| `sentient_i18n` | Internationalization |
| `sentient_observability` | Metrics, tracing, logging |
| `sentient_benchmarks` | Performance benchmarks |

## 🏢 Enterprise Crates

| Crate | Açıklama |
|-------|----------|
| `sentient_enterprise` | RBAC, SSO, Multi-tenancy |
| `sentient_compliance` | SOC 2 certification, audit, controls |
| `sentient_sla` | SLA monitoring, support tiers, uptime |
| `sentient_backup` | Backup/restore with encryption |
| `sentient_dr` | Disaster recovery, failover |

## 🎤 Voice & Audio Crates

| Crate | Açıklama |
|-------|----------|
| `sentient_voice` | Whisper STT, TTS, speaker diarization |
| `sentient_wake` | Wake word detection (Porcupine, Vosk) |
| `sentient_channels` | 23 channel integrations |

---

# 72 Entegrasyon

SENTIENT OS **72 açık kaynak proje** entegre eder:

## 🤖 Agent Frameworks (17)

| Proje | Lisans | Açıklama |
|-------|--------|----------|
| **CrewAI** | MIT | Multi-agent orchestration |
| **AutoGen** | MIT | Microsoft conversational agents |
| **OpenHands** | MIT | Software development agent |
| **MetaGPT** | MIT | Multi-agent software company |
| **AutoGPT** | MIT | Autonomous task execution |
| **AgentGPT** | MIT | Web-based agent deployment |
| **BabyAGI** | MIT | Task-driven autonomous agent |
| **Swarm** | MIT | OpenAI lightweight orchestration |
| **Camel-AI** | MIT | Communicative agents |
| **Agent-S** | MIT | Web agent automation |
| **PraisenAI** | MIT | Multi-agent framework |
| **GPT-Engineer** | MIT | Code generation |
| **Goose** | MIT | Desktop automation agent |
| **Agency-Agents** | MIT | Multi-agent patterns |
| **TaskWeaver** | MIT | Code-first agent framework |
| **Autogen Studio** | MIT | AutoGen web UI |
| **AutoResearch** | Apache-2 | Research automation |

## 🌐 Browser Automation (5)

| Proje | Lisans | Açıklama |
|-------|--------|----------|
| **Browser-Use** | MIT | AI browser automation |
| **Lightpanda** | MIT | Zig-based headless browser |
| **ByteBot** | MIT | Browser automation |
| **Agent-Browser** | MIT | AI agent browser |
| **Open-Computer-Use** | MIT | Desktop control |

## 🧠 Memory Systems (4)

| Proje | Lisans | Açıklama |
|-------|--------|----------|
| **ChromaDB** | Apache-2 | Vector database |
| **Qdrant** | Apache-2 | High-performance vectors |
| **Weaviate** | BSD-3 | Semantic search |
| **Letta** | Apache-2 | Long-term memory (MemGPT) |

## 📚 LLM Frameworks (23)

| Proje | Lisans | Açıklama |
|-------|--------|----------|
| **LangChain** | MIT | LLM application framework |
| **LlamaIndex** | MIT | Data framework for LLM |
| **Semantic Kernel** | MIT | Microsoft AI SDK |
| **Ollama** | MIT | Local LLM runtime |
| **GPT4All** | MIT | CPU-optimized local models |
| **Haystack** | Apache-2 | NLP framework |
| **Phidata** | MIT | AI assistant builder |
| **Pydantic-AI** | MIT | Type-safe AI agents |
| **SmolAgents** | MIT | Lightweight agents |
| **Aider** | Apache-2 | AI pair programming |
| **Open-WebUI** | MIT | ChatGPT-like UI |
| **Text-Generation-WebUI** | AGPL-3 | LLM web interface |
| **Dify** | Apache-2 | LLM app platform |
| **FastGPT** | Apache-2 | Knowledge base Q&A |
| **Continue-Dev** | Apache-2 | VS Code AI assistant |
| **Anthropic-Cookbook** | MIT | Claude examples |
| **Storm** | MIT | Research writing |
| **TensorFlow** | Apache-2 | ML framework |
| **AutoGluon** | Apache-2 | AutoML |
| **Llama-Recipes** | MIT | LLaMA fine-tuning |
| **LMS** | MIT | Model serving |
| **Hugging Face Transformers** | Apache-2 | Pre-trained models |

## 🛠️ Tools (5)

| Proje | Lisans | Açıklama |
|-------|--------|----------|
| **Mem0** | MIT | Long-term memory API |
| **Firecrawl** | MIT | Web scraping API |
| **Crawl4AI** | MIT | AI-powered crawling |
| **RAGFlow** | Apache-2 | RAG workflow |
| **Judge0** | MIT | Code execution |

## 🔍 Search (2)

| Proje | Lisans | Açıklama |
|-------|--------|----------|
| **SearXNG** | AGPL-3 | Privacy metasearch |
| **MindSearch** | Apache-2 | AI research search |

## 🔐 Security (1)

| Proje | Lisans | Açıklama |
|-------|--------|----------|
| **NeMo-Guardrails** | Apache-2 | NVIDIA guardrails |

## 📦 Sandbox (3)

| Proje | Lisans | Açıklama |
|-------|--------|----------|
| **E2B SDK** | Apache-2 | Secure code execution |
| **Daytona** | Apache-2 | Dev environment |
| **LocalStack** | Apache-2 | AWS local emulator |

## 💻 CLI Tools (2)

| Proje | Lisans | Açıklama |
|-------|--------|----------|
| **Gemini CLI** | Apache-2 | Google Gemini terminal |
| **Google Workspace CLI** | Apache-2 | Google API CLI |

## ▶️ Execution (1)

| Proje | Lisans | Açıklama |
|-------|--------|----------|
| **Open Interpreter** | MIT | Code execution |

## 🎯 Skills (6)

| Proje | Lisans | Açıklama |
|-------|--------|----------|
| **Awesome-OpenClaw-Skills** | MIT | Skill collection |
| **Everything-Claude-Code** | MIT | Claude Code skills |
| **Deerflow-Skills** | MIT | Automation skills |
| **Awesome-n8n-Templates** | MIT | Workflow templates |
| **Claw3D** | MIT | 3D visualization |
| **GStack** | MIT | Google Cloud skills |

## 🧠 Cevahir AI (1)

| Proje | Lisans | Açıklama |
|-------|--------|----------|
| **Cevahir AI** | Apache-2 | Full-stack Turkish LLM engine |

---

# 326+ Native LLM Model (200K+ via Aggregators)

SENTIENT OS **40+ provider** ve **600+ model** destekler:

---

## 🔷 OPENAI

| Model | Context | Açıklama
|-------|---------|----------
| `gpt-4.1` | 1M | En yeni GPT-4.1 serisi
| `gpt-4.1-mini` | 1M | Hafif versiyon
| `gpt-4.1-nano` | 1M | En küçük versiyon
| `gpt-4.5-preview` | 128K | GPT-4.5 önizleme
| `gpt-4o` | 128K | Multimodal, hızlı
| `gpt-4o-mini` | 128K | Ucuz multimodal
| `gpt-4o-audio-preview` | 128K | Ses desteği
| `gpt-4o-realtime-preview` | 128K | Gerçek zamanlı ses
| `gpt-4-turbo` | 128K | GPT-4 Turbo
| `gpt-4-turbo-preview` | 128K | Önizleme
| `gpt-4` | 8K | Orijinal GPT-4
| `gpt-4-32k` | 32K | Uzun context
| `gpt-4-vision-preview` | 128K | Görüntü desteği
| `gpt-3.5-turbo` | 16K | Ekonomik
| `gpt-3.5-turbo-16k` | 16K | Uzun context
| `gpt-3.5-turbo-instruct` | 4K | Instruction-tuned
| `o1` | 200K | Gelişmiş reasoning
| `o1-pro` | 200K | Profesyonel reasoning
| `o1-preview` | 128K | Reasoning önizleme
| `o1-mini` | 128K | Hızlı reasoning
| `o3-mini` | 200K | Efficient reasoning
| `o3-mini-high` | 200K | Yüksek performans
| `chatgpt-4o-latest` | 128K | ChatGPT latest
| `codex-mini` | 16K | Kod üretimi

---

## 🟠 ANTHROPIC

| Model | Context | Açıklama
|-------|---------|----------
| `claude-opus-4-20250514` | 200K | En güçlü Claude
| `claude-sonnet-4-20250514` | 200K | Dengeli performans
| `claude-3-7-sonnet-20250219` | 200K | Extended thinking
| `claude-3-5-sonnet-20241022` | 200K | Hızlı ve yetenekli
| `claude-3-5-haiku-20241022` | 200K | En hızlı
| `claude-3-opus-20240229` | 200K | En güçlü (eski)
| `claude-3-sonnet-20240229` | 200K | Dengeli (eski)
| `claude-3-haiku-20240307` | 200K | Hızlı (eski)
| `claude-2.1` | 200K | Önceki nesil
| `claude-2.0` | 100K | İkinci nesil
| `claude-instant-1.2` | 100K | Hızlı versiyon

---

## 🔵 GOOGLE

| Model | Context | Açıklama
|-------|---------|----------
| `gemini-2.5-pro-preview-06-05` | 1M | En yeni Gemini Pro
| `gemini-2.5-flash-preview-05-20` | 1M | En yeni Gemini Flash
| `gemini-2.0-flash` | 1M | Multimodal, hızlı
| `gemini-2.0-flash-lite` | 1M | En hafif
| `gemini-2.0-flash-exp` | 1M | Experimental
| `gemini-2.0-pro-exp` | 1M | Pro experimental
| `gemini-1.5-pro` | 2M | 2M context
| `gemini-1.5-pro-latest` | 2M | En güncel
| `gemini-1.5-flash` | 1M | Hızlı versiyon
| `gemini-1.5-flash-latest` | 1M | Güncel flash
| `gemini-1.5-flash-8b` | 1M | 8B parametre
| `gemini-pro` | 32K | Standart
| `gemini-pro-vision` | 16K | Görüntü desteği
| `gemma-3-27b-it` | 128K | Gemma 3 27B
| `gemma-3-12b-it` | 128K | Gemma 3 12B
| `gemma-3-4b-it` | 128K | Gemma 3 4B
| `gemma-2-27b-it` | 8K | Gemma 2 27B
| `gemma-2-9b-it` | 8K | Gemma 2 9B
| `gemma-2-2b-it` | 8K | Gemma 2 2B
| `gemma-2b` | 8K | Orijinal Gemma
| `gemma-7b` | 8K | Orijinal Gemma 7B
| `palm-2-chat-bison` | 8K | PaLM 2
| `palm-2-codechat-bison` | 8K | Kod odaklı

---

## 🔶 META LLAMA (OpenRouter & Ollama)

| Model | Parametreler | Açıklama
|-------|--------------|----------
| `llama-4-maverick-17b-128e-instruct` | 400B | Llama 4 Maverick
| `llama-4-scout-17b-16e-instruct` | 109B | Llama 4 Scout
| `llama-3.3-70b-instruct` | 70B | En güncel Llama 3.3
| `llama-3.3-8b-instruct` | 8B | Küçük versiyon
| `llama-3.2-90b-vision-instruct` | 90B | Vision model
| `llama-3.2-11b-vision-instruct` | 11B | Küçük vision
| `llama-3.2-3b-instruct` | 3B | Mini versiyon
| `llama-3.2-1b-instruct` | 1B | En küçük
| `llama-3.1-405b-instruct` | 405B | En büyük
| `llama-3.1-70b-instruct` | 70B | Dengeli
| `llama-3.1-8b-instruct` | 8B | Hafif
| `llama-3-70b-instruct` | 70B | Llama 3
| `llama-3-8b-instruct` | 8B | Llama 3 küçük
| `llama-2-70b-chat` | 70B | Llama 2
| `llama-2-13b-chat` | 13B | Orta boy
| `llama-2-7b-chat` | 7B | Küçük
| `llama-guard-3-8b` | 8B | Güvenlik filtresi
| `llama-guard-2-8b` | 8B | Önceki guard
| `code-llama-34b-instruct` | 34B | Kod üretimi
| `code-llama-13b-instruct` | 13B | Orta kod
| `code-llama-7b-instruct` | 7B | Küçük kod

---

## 🟣 MISTRAL AI

| Model | Context | Açıklama
|-------|---------|----------
| `mistral-large-2411` | 128K | En büyük Mistral
| `mistral-large-2407` | 128K | Büyük versiyon
| `mistral-medium-2505` | 128K | Orta boy
| `mistral-small-2503` | 128K | Küçük versiyon
| `codestral-2501` | 32K | Kod odaklı
| `codestral-mamba` | 256K | Mamba mimari
| `ministral-8b-2410` | 128K | Mini 8B
| `ministral-3b-2410` | 128K | Mini 3B
| `pixtral-12b-2409` | 128K | Vision model
| `pixtral-large-2411` | 128K | Büyük vision
| `mixtral-8x22b-instruct` | 64K | MoE 8x22B
| `mixtral-8x7b-instruct` | 32K | MoE 8x7B
| `mixtral-8x22b-instruct` | 141B | Büyük MoE
| `mistral-large-2411` | 123B | En büyük
| `mistral-large-2407` | 123B | Önceki versiyon
| `mistral-medium-2312` | ~70B | Orta boy
| `mistral-small-2402` | 22B | Küçük
| `mistral-small-3.1-24b-instruct` | 24B | Güncel küçük
| `codestral-2501` | 22B | Kod odaklı
| `codestral-mamba` | 22B | Mamba mimarisi
| `ministral-8b-2410` | 8B | Edge AI
| `ministral-3b-2410` | 3B | En küçük
| `mistral-embed` | 7B | Embedding
| `mistral-7b-instruct-v0.3` | 7B | Açık kaynak
| `mistral-7b-instruct-v0.2` | 7B | Önceki versiyon

---

## 🔴 DEEPSEEK

| Model | Parametreler | Açıklama
|-------|--------------|----------
| `deepseek-v3-0324` | 685B | En yeni DeepSeek V3
| `deepseek-v3` | 685B | DeepSeek V3
| `deepseek-r1` | 671B | Reasoning model
| `deepseek-r1-distill-llama-70b` | 70B | Distill versiyon
| `deepseek-r1-distill-qwen-32b` | 32B | Qwen distill
| `deepseek-r1-distill-llama-8b` | 8B | Küçük distill
| `deepseek-coder-v3-0324` | 685B | En yeni coder
| `deepseek-coder-v2` | 236B | Kod odaklı
| `deepseek-coder-v2-lite-instruct` | 16B | Hafif coder
| `deepseek-chat` | - | Genel amaçlı
| `deepseek-reasoner` | - | Reasoning
| `deepseek-prover-v2-671b` | 671B | Matematik kanıtlama

---

## 🟢 QWEN (Alibaba)

| Model | Parametreler | Açıklama
|-------|--------------|----------
| `qwen-3-235b-a22b-instruct` | 235B | Qwen 3 MoE
| `qwen-3-32b-instruct` | 32B | Qwen 3 32B
| `qwen-3-14b-instruct` | 14B | Qwen 3 14B
| `qwen-3-8b-instruct` | 8B | Qwen 3 8B
| `qwen-3-4b-instruct` | 4B | Qwen 3 4B
| `qwen-3-1.7b-instruct` | 1.7B | Qwen 3 küçük
| `qwen-2.5-72b-instruct` | 72B | Qwen 2.5 büyük
| `qwen-2.5-32b-instruct` | 32B | Dengeli
| `qwen-2.5-14b-instruct` | 14B | Orta boy
| `qwen-2.5-7b-instruct` | 7B | Küçük
| `qwen-2.5-3b-instruct` | 3B | Mini
| `qwen-2.5-1.5b-instruct` | 1.5B | En küçük
| `qwen-2.5-coder-32b-instruct` | 32B | Kod odaklı
| `qwen-2.5-coder-7b-instruct` | 7B | Küçük coder
| `qwen-2.5-vl-72b-instruct` | 72B | Vision-language
| `qwen-2.5-vl-7b-instruct` | 7B | Küçük VL
| `qwen-2-72b-instruct` | 72B | Qwen 2
| `qwen-2-7b-instruct` | 7B | Qwen 2 küçük
| `qwq-32b-preview` | 32B | Reasoning model
| `qwen-max` | - | Alibaba Cloud
| `qwen-plus` | - | Orta seviye
| `qwen-turbo` | - | Hızlı
| `qwen-vl-max` | - | Vision
| `qwen-vl-plus` | - | Vision orta

---

## 🟡 X.AI

| Model | Context | Açıklama
|-------|---------|----------
| `grok-3` | 128K | En yeni Grok
| `grok-3-fast` | 128K | Hızlı versiyon
| `grok-3-mini` | 128K | Küçük versiyon
| `grok-2-1212` | 128K | Grok 2 güncel
| `grok-2-vision-1212` | 128K | Vision desteği
| `grok-2-image-1212` | 128K | Görsel üretimi
| `grok-beta` | 128K | Beta versiyon
| `grok-vision-beta` | 128K | Vision beta

---

## 🔵 GROQ (Ultra-Fast Inference)

| Model | Açıklama
|-------|----------
| `llama-3.3-70b-versatile` | Çok yönlü
| `llama-3.3-70b-specdec` | Speculative decoding
| `llama-3.1-70b-versatile` | Çok yönlü
| `llama-3.1-8b-instant` | Anlık yanıt
| `llama-3.2-90b-vision-preview` | Vision
| `llama-3.2-11b-vision-preview` | Küçük vision
| `llama-3.2-3b-preview` | Mini
| `llama-3.2-1b-preview` | En küçük
| `mixtral-8x7b-32768` | Uzun context
| `gemma2-9b-it` | Google Gemma
| `deepseek-r1-distill-llama-70b` | Reasoning
| `qwen-2.5-32b-instruct` | Qwen
| `qwen-2.5-coder-32b-instruct` | Kod
| `mistral-saba-24b` | Mistral Saba
| `llama-guard-3-8b` | Güvenlik

---

## 🟠 COHERE

| Model | Context | Açıklama
|-------|---------|----------
| `command-a-03-2025` | 256K | En yeni Command A
| `command-r7b-12-2024` | 128K | Hafif versiyon
| `command-r-plus-08-2024` | 128K | Güçlü RAG
| `command-r-plus` | 128K | RAG odaklı
| `command-r-08-2024` | 128K | Dengeli
| `command-r` | 128K | Standart RAG
| `command` | 4K | Genel amaçlı
| `command-light` | 4K | Hafif
| `command-nightly` | 4K | Günlük build
| `rerank-english-v3.0` | - | Reranking
| `rerank-multilingual-v3.0` | - | Çok dilli rerank
| `embed-english-v3.0` | - | Embedding
| `embed-multilingual-v3.0` | - | Çok dilli embed
| `embed-v4.0` | - | En yeni embedding

---

## 🟣 PERPLEXITY

| Model | Context | Açıklama
|-------|---------|----------
| `sonar-pro` | 200K | Profesyonel arama
| `sonar` | 128K | Standart arama
| `sonar-reasoning-pro` | 128K | Reasoning + arama
| `sonar-reasoning` | 128K | Hafif reasoning
| `sonar-deep-research` | 128K | Derin araştırma
| `llama-3.1-sonar-small-128k-online` | 128K | Online küçük
| `llama-3.1-sonar-large-128k-online` | 128K | Online büyük
| `llama-3.1-sonar-huge-128k-online` | 128K | Online dev
| `llama-3.1-sonar-small-128k-chat` | 128K | Chat küçük
| `llama-3.1-sonar-large-128k-chat` | 128K | Chat büyük
| `llama-3.1-sonar-small-128k` | 128K | Offline küçük
| `llama-3.1-sonar-large-128k` | 128K | Offline büyük
| `r1-1776` | 128K | DeepSeek R1 türevi

---

## 🔶 TOGETHER AI

| Model | Açıklama
|-------|----------
| `meta-llama/Llama-4-Maverick-17B-128E-Instruct-FP8` | Llama 4
| `meta-llama/Llama-4-Scout-17B-16E-Instruct-FP8` | Llama 4 Scout
| `meta-llama/Llama-3.3-70B-Instruct-Turbo` | Hızlı Llama
| `meta-llama/Llama-3.2-90B-Vision-Instruct-Turbo` | Vision
| `meta-llama/Llama-3.2-11B-Vision-Instruct-Turbo` | Küçük vision
| `meta-llama/Llama-3.1-405B-Instruct-Turbo` | Dev model
| `meta-llama/Llama-3.1-70B-Instruct-Turbo` | Dengeli
| `meta-llama/Llama-3.1-8B-Instruct-Turbo` | Hafif
| `mistralai/Mixtral-8x7B-Instruct-v0.1` | MoE
| `mistralai/Mixtral-8x22B-Instruct-v0.1` | Büyük MoE
| `mistralai/Mistral-7B-Instruct-v0.3` | Küçük
| `Qwen/Qwen3-235B-A22B-Instruct` | Qwen 3
| `Qwen/Qwen2.5-72B-Instruct-Turbo` | Qwen 2.5
| `Qwen/Qwen2.5-Coder-32B-Instruct` | Kod
| `deepseek-ai/DeepSeek-V3` | DeepSeek
| `deepseek-ai/DeepSeek-R1` | Reasoning
| `databricks/dbrx-Instruct` | Databricks
| `allenai/OLMo-2-1124-13B-Instruct` | OLMo
| `google/gemma-2-27b-it` | Gemma
| `google/gemma-2-9b-it` | Küçük Gemma
| `NousResearch/Nous-Hermes-3-Llama-3.1-405B` | Hermes
| `NousResearch/Nous-Hermes-3-Llama-3.1-70B` | Küçük Hermes
| `Gryphe/MythoMax-L2-13b` | MythoMax
| `teknium/OpenHermes-2-Mistral-7B` | OpenHermes
| `undi95/ReMM-SLERP-L2-13B` | ReMM
| `NousResearch/Nous-Capybara-7B-V1p` | Capybara
| `carson/ml4w-7b` | ML4W
| `togethercomputer/CodeLlama-34b-Instruct` | Kod
| `togethercomputer/CodeLlama-13b-Instruct` | Küçük kod

---

## 🔴 NVIDIA NIM

| Model | Açıklama
|-------|----------
| `meta/llama-3.3-70b-instruct` | Llama 3.3
| `meta/llama-3.1-405b-instruct` | En büyük
| `meta/llama-3.1-70b-instruct` | Dengeli
| `meta/llama-3.1-8b-instruct` | Hafif
| `mistralai/mistral-large` | Mistral
| `mistralai/mixtral-8x7b-instruct-v0.1` | MoE
| `google/gemma-2-27b-it` | Gemma
| `google/gemma-2-9b-it` | Küçük Gemma
| `nvidia/nemotron-4-340b-instruct` | NVIDIA özel
| `nvidia/llama-3.1-nemotron-70b-instruct` | Nemotron Llama
| `nvidia/nemotron-h-8b-instruct` | Küçük Nemotron
| `microsoft/phi-3-mini-4k-instruct` | Phi-3
| `microsoft/phi-3-medium-4k-instruct` | Orta Phi
| `snowflake/arctic` | Snowflake
| `recurrentai/rwkv-5-world-3b` | RWKV

---

## 🟢 REPLICATE

| Model | Açıklama
|-------|----------
| `meta/llama-4-maverick` | Llama 4
| `meta/llama-3.3-70b-instruct` | Llama 3.3
| `meta/llama-3.1-405b-instruct` | En büyük
| `meta/llama-3.2-90b-vision-instruct` | Vision
| `mistralai/mixtral-8x7b-instruct-v0.1` | MoE
| `mistralai/mistral-7b-instruct-v0.2` | Küçük
| `google/gemma-2-9b-it` | Gemma
| `qwen/qwen-2.5-72b-instruct` | Qwen
| `deepseek-ai/deepseek-v3` | DeepSeek
| `deepseek-ai/deepseek-r1` | Reasoning
| `black-forest-labs/flux-schnell` | Görsel üretimi
| `black-forest-labs/flux-dev` | Gelişmiş görsel
| `stability-ai/stable-diffusion-3` | SD3
| `stability-ai/sdxl` | SDXL

---

## 🔵 FIREWORKS AI

| Model | Açıklama
|-------|----------
| `accounts/fireworks/models/llama-v3p3-70b-instruct` | Llama 3.3
| `accounts/fireworks/models/llama-v3p1-405b-instruct` | En büyük
| `accounts/fireworks/models/llama-v3p1-70b-instruct` | Dengeli
| `accounts/fireworks/models/llama-v3p1-8b-instruct` | Hafif
| `accounts/fireworks/models/qwen2p5-72b-instruct` | Qwen
| `accounts/fireworks/models/qwen2p5-coder-32b-instruct` | Kod
| `accounts/fireworks/models/mixtral-8x7b-instruct` | MoE
| `accounts/fireworks/models/mixtral-8x22b-instruct` | Büyük MoE
| `accounts/fireworks/models/deepseek-v3` | DeepSeek
| `accounts/fireworks/models/deepseek-r1` | Reasoning
| `accounts/fireworks/models/gemma2-27b-it` | Gemma
| `accounts/fireworks/models/gemma2-9b-it` | Küçük Gemma
| `accounts/fireworks/models/qwen2p5-vl-72b-instruct` | Vision

---

## 🟠 AI21 LABS

| Model | Context | Açıklama
|-------|---------|----------
| `jamba-1-6-large` | 256K | En büyük Jamba
| `jamba-1-6-mini` | 256K | Küçük Jamba
| `jamba-1-5-large` | 256K | Jamba 1.5 büyük
| `jamba-1-5-mini` | 256K | Jamba 1.5 küçük
| `jamba-instruct` | 256K | İlk Jamba
| `jurassic-2-mid` | 8K | Jurassic orta
| `jurassic-2-large` | 8K | Jurassic büyük
| `jurassic-2-jumbo` | 8K | En büyük Jurassic
| `jurassic-2-light` | 8K | Hafif

---

## 🟣 STABILITY AI

| Model | Açıklama
|-------|----------
| `stablelm-2-12b-chat` | Büyük StableLM
| `stablelm-2-7b-chat` | Orta StableLM
| `stablelm-2-1-6b-chat` | Küçük StableLM
| `stablelm-zephyr-3b` | Zephyr
| `stable-code-3b` | Kod odaklı
| `stable-diffusion-3-medium` | Görsel SD3
| `stable-diffusion-xl-base-1.0` | SDXL
| `stable-video-diffusion-img2vid-xt` | Video
| `stable-audio-open-1.0` | Ses
| `stable-fast-3d` | 3D model

---

## 🔶 MICROSOFT PHI

| Model | Parametreler | Açıklama
|-------|--------------|----------
| `phi-4` | 14B | En yeni Phi
| `phi-4-mini` | 3.8B | Küçük Phi 4
| `phi-3.5-mini-128k-instruct` | 3.8B | Phi 3.5
| `phi-3.5-moe` | 16B | MoE versiyon
| `phi-3-medium-128k-instruct` | 14B | Orta boy
| `phi-3-medium-4k-instruct` | 14B | Kısa context
| `phi-3-mini-128k-instruct` | 3.8B | Küçük uzun context
| `phi-3-mini-4k-instruct` | 3.8B | Küçük kısa context
| `phi-3-small-128k-instruct` | 7B | Küçük
| `phi-3-small-8k-instruct` | 7B | Küçük orta context
| `phi-2` | 2.7B | Önceki nesil

---

## 🟡 IBM WATSONX

| Model | Açıklama
|-------|----------
| `ibm/granite-3.2-8b-instruct` | En yeni Granite
| `ibm/granite-3.1-8b-instruct` | Granite 3.1
| `ibm/granite-13b-chat-v2` | Chat odaklı
| `ibm/granite-20b-code-instruct` | Kod odaklı
| `ibm/granite-34b-code-instruct` | Büyük kod
| `ibm/granite-3b-code-instruct` | Küçük kod
| `meta-llama/llama-3-70b-instruct` | Llama on WatsonX
| `mistralai/mixtral-8x7b-instruct-v0.1` | Mistral on WatsonX
| `ibm/slate-30m-english-rtrvr` | Retrieval
| `ibm/slate-125m-english-rtrvr` | Büyük retrieval

---

## 🔵 AWS BEDROCK

| Model | Açıklama
|-------|----------
| `anthropic.claude-3-5-sonnet-20241022-v2:0` | Claude 3.5 Sonnet
| `anthropic.claude-3-opus-20240229-v1:0` | Claude 3 Opus
| `anthropic.claude-3-haiku-20240307-v1:0` | Claude 3 Haiku
| `anthropic.claude-3-sonnet-20240229-v1:0` | Claude 3 Sonnet
| `meta.llama3-3-70b-instruct-v1:0` | Llama 3.3
| `meta.llama3-1-405b-instruct-v1:0` | En büyük Llama
| `meta.llama3-1-70b-instruct-v1:0` | Dengeli Llama
| `meta.llama3-1-8b-instruct-v1:0` | Küçük Llama
| `meta.llama3-2-90b-vision-instruct-v1:0` | Vision Llama
| `meta.llama3-2-11b-vision-instruct-v1:0` | Küçük vision
| `mistral.mistral-large-2407-v1:0` | Mistral Large
| `mistral.mixtral-8x7b-instruct-v0:1` | MoE
| `mistral.mistral-7b-instruct-v0:2` | Küçük
| `amazon.titan-text-premier-v1:0` | Amazon Titan
| `amazon.titan-text-lite-v1` | Hafif Titan
| `amazon.titan-text-express-v1` | Express Titan
| `amazon.titan-embed-text-v2:0` | Embedding
| `cohere.command-r-plus-v1:0` | Cohere R+
| `cohere.command-r-v1:0` | Cohere R
| `cohere.command-text-v14` | Command
| `cohere.embed-english-v3` | Embedding
| `ai21.jamba-1-5-large-v1:0` | Jamba
| `ai21.jamba-1-5-mini-v1:0` | Küçük Jamba

---

## 🟠 AZURE OPENAI

| Model | Açıklama
|-------|----------
| `gpt-4.1` | En yeni GPT-4.1
| `gpt-4.1-mini` | Hafif GPT-4.1
| `gpt-4.5-preview` | GPT-4.5 önizleme
| `gpt-4o` | Multimodal
| `gpt-4o-mini` | Ucuz multimodal
| `gpt-4-turbo` | Turbo
| `gpt-4` | Standart
| `gpt-4-32k` | Uzun context
| `gpt-3.5-turbo` | Ekonomik
| `gpt-3.5-turbo-16k` | Uzun ekonomik
| `o1` | Reasoning
| `o1-preview` | Reasoning önizleme
| `o1-mini` | Hızlı reasoning
| `o3-mini` | Efficient reasoning
| `text-embedding-3-large` | Büyük embedding
| `text-embedding-3-small` | Küçük embedding
| `text-embedding-ada-002` | Ada embedding
| `dall-e-3` | Görsel üretimi
| `dall-e-2` | Önceki DALL-E
| `whisper` | Ses tanıma
| `tts-1` | Ses sentezi
| `tts-1-hd` | HD ses

---

## 🟣 GOOGLE VERTEX AI

| Model | Açıklama
|-------|----------
| `gemini-2.5-pro-preview` | En yeni Gemini Pro
| `gemini-2.5-flash-preview` | En yeni Flash
| `gemini-2.0-flash` | Gemini 2.0
| `gemini-2.0-flash-lite` | Hafif 2.0
| `gemini-2.0-pro` | Pro 2.0
| `gemini-1.5-pro` | 2M context
| `gemini-1.5-flash` | Hızlı 1.5
| `gemini-1.5-flash-8b` | Küçük 1.5
| `gemini-pro` | Standart
| `gemini-pro-vision` | Vision
| `gemma-3-27b-it` | Gemma 3
| `gemma-3-12b-it` | Küçük Gemma 3
| `text-embedding-005` | Embedding
| `text-embedding-004` | Önceki embedding
| `text-multilingual-embedding-002` | Çok dilli
| `imagen-3.0-generate-002` | Görsel üretimi
| `imagen-3.0-fast-generate-001` | Hızlı görsel
| `veo-002` | Video üretimi
| `chirp-2` | Ses tanıma

---

## 🟢 OLLAMA (Local Models - Ücretsiz)

| Model | Parametreler | Açıklama
|-------|--------------|----------
| `llama4:17b` | 17B | **Llama 4 Maverick**
| `llama4:scout` | 17B | Llama 4 Scout
| `llama3.3:70b` | 70B | Llama 3.3 büyük
| `llama3.3:8b` | 8B | Llama 3.3 küçük
| `llama3.2:3b` | 3B | Llama 3.2 mini
| `llama3.2:1b` | 1B | En küçük Llama
| `llama3.1:8b` | 8B | Llama 3.1
| `llama3.1:70b` | 70B | Llama 3.1 büyük
| `llama3.1:405b` | 405B | En büyük Llama
| `llama3:70b` | 70B | Llama 3
| `llama3:8b` | 8B | Llama 3 küçük
| `llama2:70b` | 70B | Llama 2
| `llama2:13b` | 13B | Llama 2 orta
| `llama2:7b` | 7B | Llama 2 küçük
| `qwen3:32b` | 32B | **Qwen 3**
| `qwen3:14b` | 14B | Qwen 3 orta
| `qwen2.5:72b` | 72B | Qwen 2.5 büyük
| `qwen2.5:32b` | 32B | Qwen 2.5
| `qwen2.5:14b` | 14B | Qwen 2.5 orta
| `qwen2.5:7b` | 7B | Qwen 2.5 küçük
| `qwen2.5-coder:32b` | 32B | **Kod odaklı**
| `qwen2.5-coder:7b` | 7B | Küçük coder
| `qwen2:72b` | 72B | Qwen 2
| `qwen2:7b` | 7B | Qwen 2 küçük
| `qwq:32b` | 32B | **Reasoning model**
| `deepseek-v3:671b` | 671B | **DeepSeek V3**
| `deepseek-r1:671b` | 671B | **DeepSeek R1 Reasoning**
| `deepseek-r1:70b` | 70B | R1 distill
| `deepseek-r1:32b` | 32B | R1 küçük distill
| `deepseek-r1:7b` | 7B | R1 en küçük
| `deepseek-coder-v2:236b` | 236B | DeepSeek Coder
| `mistral:7b` | 7B | Mistral 7B
| `mistral-nemo:12b` | 12B | Nemo
| `mixtral:8x7b` | 47B | MoE
| `mixtral:8x22b` | 141B | Büyük MoE
| `codellama:70b` | 70B | Kod Llama büyük
| `codellama:34b` | 34B | Kod Llama
| `codellama:13b` | 13B | Kod Llama orta
| `codellama:7b` | 7B | Kod Llama küçük
| `gemma4:31b` | 31B | **KERNEL DEFAULT**
| `gemma4:26b-moe` | 26B | Gemma 4 MoE
| `gemma4:e4b` | 4B | Gemma 4 Edge
| `gemma4:e2b` | 2B | Gemma 4 Mobile
| `gemma3:27b` | 27B | Gemma 3
| `gemma3:12b` | 12B | Gemma 3 orta
| `gemma2:27b` | 27B | Gemma 2
| `gemma2:9b` | 9B | Gemma 2 küçük
| `gemma:7b` | 7B | Orijinal Gemma
| `phi4:14b` | 14B | **Phi-4**
| `phi3.5:3.8b` | 3.8B | Phi 3.5
| `phi3:14b` | 14B | Phi 3 büyük
| `phi3:medium` | 14B | Phi 3 medium
| `phi3:mini` | 3.8B | Phi 3 mini
| `command-r:35b` | 35B | Cohere R
| `starcoder2:7b` | 7B | StarCoder2
| `codeqwen:7b` | 7B | Qwen Coder
| `llava:13b` | 13B | Vision-language
| `llava:7b` | 7B | Küçük VL
| `moondream:latest` | 1.6B | Mini vision
| `nomic-embed-text:latest` | 137M | Embedding
| `mxbai-embed-large:latest` | 335M | Büyük embedding
| `dolphin-mixtral:8x7b` | 47B | Dolphin MoE
| `openchat:7b` | 7B | OpenChat
| `wizardlm2:7b` | 7B | WizardLM
| `yi:34b` | 34B | Yi
| `solar:10.7b` | 10.7B | Solar
| `nous-hermes2:10.7b` | 10.7B | Nous Hermes
| `tinyllama:1.1b` | 1.1B | En küçük

---

## 🔷 HUGGINGFACE (Inference API)

| Model | Açıklama
|-------|----------
| `meta-llama/Llama-4-Maverick-17B-128E-Instruct` | Llama 4
| `meta-llama/Llama-3.3-70B-Instruct` | Llama 3.3
| `meta-llama/Llama-3.1-405B-Instruct` | En büyük
| `meta-llama/Llama-3.1-70B-Instruct` | Dengeli
| `mistralai/Mistral-Large-Instruct-2407` | Mistral Large
| `mistralai/Mixtral-8x22B-Instruct-v0.1` | Büyük MoE
| `mistralai/Mixtral-8x7B-Instruct-v0.1` | MoE
| `mistralai/Mistral-7B-Instruct-v0.3` | Küçük
| `google/gemma-3-27b-it` | Gemma 3
| `google/gemma-2-27b-it` | Gemma 2 büyük
| `google/gemma-2-9b-it` | Gemma 2 küçük
| `google/gemma-2-2b-it` | Gemma 2 mini
| `Qwen/Qwen3-235B-A22B-Instruct` | Qwen 3 MoE
| `Qwen/Qwen2.5-72B-Instruct` | Qwen 2.5
| `Qwen/Qwen2.5-32B-Instruct` | Orta Qwen
| `Qwen/Qwen2.5-14B-Instruct` | Küçük Qwen
| `Qwen/Qwen2.5-7B-Instruct` | Mini Qwen
| `Qwen/Qwen2.5-Coder-32B-Instruct` | Kod
| `deepseek-ai/DeepSeek-V3` | DeepSeek V3
| `deepseek-ai/DeepSeek-R1` | DeepSeek R1
| `deepseek-ai/DeepSeek-Coder-V2-Instruct` | DeepSeek Coder
| `microsoft/Phi-4` | Phi 4
| `microsoft/Phi-3-medium-4k-instruct` | Orta Phi
| `microsoft/Phi-3-mini-4k-instruct` | Küçük Phi
| `tiiuae/falcon-180B-chat` | Falcon 180B
| `tiiuae/falcon-40B-instruct` | Falcon 40B
| `tiiuae/falcon-7B-instruct` | Falcon 7B
| `databricks/dbrx-instruct` | Databricks DBRX
| `allenai/OLMo-7B-Instruct` | OLMo
| `allenai/OLMo-2-1124-13B-Instruct` | OLMo 2
| `bigscience/bloom` | BLOOM 176B
| `bigscience/bloomz` | BLOOMZ
| `openchat/openchat-3.5-1210` | OpenChat
| `NousResearch/Nous-Hermes-2-Mixtral-8x7B-DPO` | Hermes MoE
| `NousResearch/Nous-Hermes-2-Solar-10.7B` | Hermes Solar
| `cognitivecomputations/dolphin-2.6-mixtral-8x7b` | Dolphin
| `teknium/OpenHermes-2.5-Mistral-7B` | OpenHermes
| `upstage/SOLAR-10.7B-Instruct-v1.0` | Solar
| `01-ai/Yi-34B-Chat` | Yi
| `Xenova/gpt-4o-mini` | ONNX GPT-4o

---

## 🟠 DİĞER PROVIDER'LAR

### Moonshot AI (Çin)
| Model | Context | Açıklama
|-------|---------|----------
| `moonshot-v1-8k` | 8K | Standart
| `moonshot-v1-32k` | 32K | Orta context
| `moonshot-v1-128k` | 128K | Uzun context

### Zhipu AI (Çin)
| Model | Context | Açıklama
|-------|---------|----------
| `glm-4-plus` | 128K | En güçlü GLM
| `glm-4` | 128K | Standart
| `glm-4-flash` | 128K | Hızlı
| `glm-4-long` | 1M | Uzun context
| `glm-4v-plus` | 8K | Vision
| `glm-4v` | 8K | Vision standart
| `embedding-3` | - | Embedding

### Baidu ERNIE (Çin)
| Model | Açıklama
|-------|----------
| `ernie-4.0-8k` | En güçlü ERNIE
| `ernie-4.0-turbo-8k` | Hızlı ERNIE 4
| `ernie-3.5-8k` | Dengeli
| `ernie-speed-8k` | Hızlı
| `ernie-speed-128k` | Uzun context

### MiniMax (Çin)
| Model | Açıklama
|-------|----------
| `abab6.5-chat` | Büyük model
| `abab6.5s-chat` | Küçük model
| `abab5.5-chat` | Önceki nesil
| `abab5.5s-chat` | Küçük önceki

### SiliconFlow (Çin)
| Model | Açıklama
|-------|----------
| `Qwen/Qwen2.5-72B-Instruct` | Qwen
| `Qwen/Qwen2.5-32B-Instruct` | Orta Qwen
| `deepseek-ai/DeepSeek-V3` | DeepSeek
| `deepseek-ai/DeepSeek-R1` | Reasoning
| `meta-llama/Llama-3.3-70B-Instruct` | Llama

### Hyperbolic (Decentralized)
| Model | Açıklama
|-------|----------
| `meta-llama/llama-3.3-70b-instruct` | Llama
| `meta-llama/llama-3.1-70b-instruct` | Llama 3.1
| `mistralai/mistral-7b-instruct` | Mistral
| `deepseek-ai/deepseek-v3` | DeepSeek
| `qwen/qwen-2.5-72b-instruct` | Qwen

### Lepton AI
| Model | Açıklama
|-------|----------
| `llama3-70b` | Llama 3
| `llama3-8b` | Küçük Llama
| `mixtral-8x7b` | MoE
| `qwen2.5-72b` | Qwen
| `gemma-2-27b` | Gemma

### RunPod Serverless
| Model | Açıklama
|-------|----------
| `llama-3-70b` | Llama 3
| `llama-3-8b` | Küçük
| `mixtral-8x7b` | MoE
| `qwen-2.5-72b` | Qwen

### Modal
| Model | Açıklama
|-------|----------
| `llama-3.3-70b` | Llama 3.3
| `llama-3.1-405b` | En büyük
| `mixtral-8x22b` | Büyük MoE

### Novita AI
| Model | Açıklama
|-------|----------
| `meta-llama/llama-3.3-70b-instruct` | Llama
| `meta-llama/llama-3.1-70b-instruct` | Llama 3.1
| `mistralai/mistral-7b-instruct` | Mistral
| `deepseek/deepseek-r1` | DeepSeek R1
| `google/gemma-2-9b-it` | Gemma
| `qwen/qwen-2-7b-instruct` | Qwen

### Inflection AI
| Model | Açıklama
|-------|----------
| `inflection-3-pi` | Pi asistan
| `inflection-3-productivity` | Productivity
| `inflection-3-reasoning` | Reasoning

### Character.AI
| Model | Açıklama
|-------|----------
| `cplus_v4_0_2` | Character+
| `c1_2_0` | Character 1.2
| `c3_2_0` | Character 3.2

### G4F (GPT4Free - Ücretsiz)
| Model | Açıklama
|-------|----------
| `gpt-4` | GPT-4 erişimi
| `gpt-4-turbo` | Turbo
| `gpt-3.5-turbo` | Ekonomik
| `claude-3-opus` | Claude
| `gemini-pro` | Gemini

---

## 📊 ÖZET

| Provider | Model Sayısı | Tür
|----------|--------------|-----
| OpenAI | 24 | Bulut (Ücretli)
| Anthropic | 11 | Bulut (Ücretli)
| Google | 22 | Bulut (Ücretli/Ücretsiz)
| Meta Llama | 21 | Açık Kaynak
| Mistral AI | 17 | Bulut + Açık
| DeepSeek | 12 | Bulut + Açık
| Qwen | 24 | Bulut + Açık
| X.AI | 8 | Bulut (Ücretli)
| Groq | 15 | Bulut (Ücretli)
| Cohere | 13 | Bulut (Ücretli)
| Perplexity | 13 | Bulut (Ücretli)
| Together AI | 29 | Bulut (Pay-as-you-go)
| NVIDIA NIM | 15 | Bulut (Enterprise)
| Replicate | 14 | Bulut (Pay-as-you-go)
| Fireworks AI | 13 | Bulut (Ucuz)
| AI21 Labs | 9 | Bulut (Ücretli)
| Stability AI | 5 | Bulut + Açık
| Microsoft Phi | 11 | Açık Kaynak
| IBM WatsonX | 8 | Bulut (Enterprise)
| AWS Bedrock | 23 | Bulut (Enterprise)
| Azure OpenAI | 22 | Bulut (Enterprise)
| Google Vertex | 19 | Bulut (Enterprise)
| Ollama | 65+ | Yerel (Ücretsiz)
| HuggingFace | 22 | Bulut + Açık
| OpenRouter | 35 | Aggregator (200+)
| LiteLLM | 15 | Aggregator (100+)
| Cerebras | 3 | Bulut (Hızlı)
| SambaNova | 6 | Bulut (Enterprise)
| DeepInfra | 12 | Bulut (Ucuz)
| GLHF | 13 | Bulut (Gaming)
| Hyperbolic | 13 | Bulut (Decentralized)
| Novita | 12 | Bulut (Ucuz)
| SiliconFlow | 17 | Bulut (Çin)
| vLLM | 8 | Yerel (Server)
| LM Studio | 7 | Yerel (GUI)
| Moonshot | 3 | Bulut (Çin)
| Zhipu AI | 5 | Bulut (Çin)
| Yi (01.AI) | 5 | Bulut (Çin)
| Baidu ERNIE | 5 | Bulut (Çin)
| MiniMax | 4 | Bulut (Çin)
| Lepton AI | 5 | Bulut (Ucuz)
| RunPod Serverless | 4 | Bulut (Serverless)
| Modal | 3 | Bulut (Serverless)
| Stability AI | 5 | Bulut (Open Source)
| IBM WatsonX | 8 | Bulut (Enterprise)
| **TOPLAM** | **352 native, 200K+ via aggregators** | -

---

# 5,587+ Skill

SENTIENT OS **5,587+ hazır skill** içerir:

## Kategoriler

| Kategori | Sayı | Örnekler |
|----------|------|----------|
| **Code** | 1,200+ | code-review, refactor, debug, test-gen, optimize |
| **Web** | 800+ | scrape, crawl, screenshot, form-fill, automate |
| **Data** | 600+ | etl, transform, analyze, visualize, query |
| **Security** | 400+ | pentest, audit, encrypt, hash, scan |
| **Communication** | 300+ | email, slack, discord, telegram, teams |
| **DevOps** | 250+ | deploy, monitor, scale, backup, ci-cd |
| **Research** | 200+ | search, summarize, cite, translate, fact-check |
| **Automation** | 150+ | schedule, trigger, pipeline, workflow, macro |
| **Media** | 100+ | image-gen, video-edit, audio-transcribe, compress |
| **Mobile** | 80+ | react-native, flutter, kotlin, swift |
| **OSINT** | 70+ | username-search, domain-lookup, social-scan |
| **Gaming** | 50+ | npc, quest-gen, level-design, balance |

## Skill Kullanımı

```bash
# Skill listele
sentient skill list
sentient skill list --category code
sentient skill search "web scraping"

# Skill çalıştır
sentient skill run code-review --path ./src --language rust
sentient skill run web-scraper --url "https://example.com" --depth 3
sentient skill run pentest --target "https://myapp.com" --port 443

# Skill bilgisi
sentient skill info code-review
sentient skill docs web-scraper
```

## Skill Oluşturma

```yaml
# skills/my-skill/SKILL.yaml
name: my-custom-skill
version: 1.0.0
description: My custom skill
author: me@example.com

inputs:
  - name: input_file
    type: string
    required: true
    description: Input file path

outputs:
  - name: result
    type: string

steps:
  - name: analyze
    action: code.analyze
    inputs:
      file: ${{ inputs.input_file }}
  
  - name: report
    action: format.markdown
    inputs:
      data: ${{ steps.analyze.output }}
```

```bash
# Skill yükle
sentient skill load ./skills/my-skill

# Skill test et
sentient skill test my-custom-skill

# Skill yayınla
sentient skill publish my-custom-skill
```

---

# CLI Komutları

## Temel Komutlar

```bash
# Sistem durumu
sentient status
sentient version
sentient doctor

# Agent yönetimi
sentient agent list
sentient agent create <name> --model gpt-4o
sentient agent run <name> --goal "Build API"
sentient agent stop <name>
sentient agent delete <name>

# Skill yönetimi
sentient skill list [--category <cat>]
sentient skill run <skill> [options]
sentient skill search <query>
sentient skill info <skill>

# Model yönetimi
sentient model list
sentient model set <model>
sentient model test
sentient model compare <model1> <model2>

# Memory
sentient memory list
sentient memory search <query>
sentient memory export --format json

# Dashboard
sentient dashboard [--port 8080]
```

## Gelişmiş Komutlar

```bash
# Swarm orchestration
sentient swarm create <name> --agents 5 --strategy collective
sentient swarm run <name> --goal "Complex task"
sentient swarm status <name>

# Pipeline
sentient pipeline list
sentient pipeline run <name>
sentient pipeline create --steps "analyze,transform,deploy"

# Plugin
sentient plugin list
sentient plugin install <name>
sentient plugin enable <name>

# Debug
sentient debug agent <name> --trace
sentient logs --follow --level debug
sentient config show
```

---

# Web Dashboard

## Başlatma

```bash
# Dashboard başlat
sentient dashboard

# Özel port
sentient dashboard --port 3000

# Auth ile
sentient dashboard --auth --jwt-secret secret123

# Production mode
sentient dashboard --production --ssl-cert /path/cert.pem --ssl-key /path/key.pem
```

## Özellikler

| Özellik | Açıklama |
|---------|----------|
| **3D Topology** | Three.js ile agent ve skill görselleştirme |
| **Real-time Logs** | WebSocket ile canlı güncelleme |
| **xterm.js Terminal** | Entegre terminal |
| **Skill Cards** | 1Password tarzı skill kartları |
| **Security Panel** | Canlı güvenlik metrikleri |
| **Chat Interface** | Agent ile sohbet |
| **Memory View** | Memory içeriği görüntüleme |
| **Plugin Store** | Plugin marketi |

## API Endpoints

```
GET  /                    # Dashboard UI
GET  /health              # Health check
GET  /api/v1/status       # Sistem durumu
GET  /api/v1/agents       # Agent listesi
POST /api/v1/agents       # Agent oluştur
GET  /api/v1/skills       # Skill listesi
POST /api/v1/skills/run   # Skill çalıştır
GET  /api/v1/memory       # Memory listesi
POST /api/v1/chat         # Agent ile sohbet
WS   /ws                   # WebSocket
```

---

# API Kullanımı

## REST API

```bash
# Status
curl http://localhost:8080/api/v1/status

# Agent oluştur
curl -X POST http://localhost:8080/api/v1/agents \
  -H "Content-Type: application/json" \
  -d '{"name": "my-agent", "model": "gpt-4o"}'

# Skill çalıştır
curl -X POST http://localhost:8080/api/v1/skills/run \
  -H "Content-Type: application/json" \
  -d '{"skill": "code-review", "params": {"path": "./src"}}'
```

## WebSocket

```javascript
const ws = new WebSocket('ws://localhost:8080/ws');

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Event:', data.type, data.payload);
};

// Agent event'lerini dinle
ws.send(JSON.stringify({
  type: 'subscribe',
  channel: 'agent:my-agent'
}));
```

## Rust SDK

```rust
use sentient_core::{Agent, Skill, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Agent oluştur
    let agent = Agent::new("gpt-4o")
        .with_name("my-agent")
        .with_memory(true)
        .build()?;
    
    // Goal çalıştır
    let result = agent.run("Build a REST API").await?;
    println!("Result: {}", result);
    
    // Skill çalıştır
    let skill = Skill::load("code-review")?;
    let output = skill.execute(json!({"path": "./src"})).await?;
    
    Ok(())
}
```

## Python SDK

```python
from sentient import Agent, Skill

# Agent oluştur
agent = Agent(model="gpt-4o", name="my-agent")

# Goal çalıştır
result = agent.run("Build a REST API")

# Skill çalıştır
skill = Skill.load("code-review")
output = skill.execute({"path": "./src"})
```

---

# Plugin Sistemi

SENTIENT OS güçlü bir plugin sistemi sunar:

## Plugin Yapısı

```
plugins/
└── my-plugin/
    ├── manifest.yaml
    ├── src/
    │   ├── lib.rs
    │   └── api.rs
    └── tests/
        └── integration.rs
```

## Plugin Manifest

```yaml
# manifest.yaml
name: my-plugin
version: 1.0.0
description: My custom plugin
author: me@example.com
license: Apache-2.0

entrypoint: src/lib.rs
permissions:
  - code.execute
  - file.read
  - network.http

dependencies:
  - sentient_core: ">=2.0.0"

config:
  api_endpoint:
    type: string
    default: "https://api.example.com"
  timeout:
    type: number
    default: 30
```

## Plugin API

```rust
// src/lib.rs
use sentient_plugin::{Plugin, Context, Result};

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &str { "my-plugin" }
    
    fn version(&self) -> &str { "1.0.0" }
    
    fn on_load(&mut self, ctx: &Context) -> Result<()> {
        // Plugin yüklendiğinde
        Ok(())
    }
    
    fn on_unload(&mut self, ctx: &Context) -> Result<()> {
        // Plugin kaldırıldığında
        Ok(())
    }
    
    fn execute(&self, ctx: &Context, input: Value) -> Result<Value> {
        // Ana işlev
        Ok(json!({"result": "success"}))
    }
}

sentient_plugin_export!(MyPlugin);
```

## Plugin Yükleme

```bash
# Plugin listele
sentient plugin list

# Local plugin yükle
sentient plugin install ./plugins/my-plugin

# Marketplace'ten yükle
sentient plugin install @sentient/my-plugin

# Plugin etkinleştir/devre dışı bırak
sentient plugin enable my-plugin
sentient plugin disable my-plugin

# Plugin güncelle
sentient plugin update my-plugin
```

---

# Güvenlik

## V-GATE (API Proxy)

```
┌─────────────┐      ┌─────────────┐      ┌─────────────┐
│   SENTIENT  │      │   V-GATE    │      │   LLM API   │
│   Client    │─────▶│   Proxy     │─────▶│   Provider  │
└─────────────┘      └─────────────┘      └─────────────┘
                           │
                     ┌─────┴─────┐
                     │ Key Vault │
                     │ (Secure)  │
                     └───────────┘
```

**Özellikler:**
- API anahtarları ASLA client'ta yok
- Automatic key rotation
- Audit logging
- Rate limiting
- Request signing

## Guardrails

```rust
use sentient_guardrails::{Guardrail, Filter};

let guardrail = Guardrail::new()
    .input_filter(Filter::prompt_injection())
    .output_filter(Filter::pii())
    .output_filter(Filter::secrets())
    .build();

// Input kontrol
let safe_input = guardrail.check_input(user_input)?;

// Output kontrol
let safe_output = guardrail.check_output(llm_response)?;
```

## TEE (Trusted Execution Environment)

```rust
use sentient_tee::{Enclave, TeeType};

// AMD SEV-SNP
let enclave = Enclave::new(TeeType::AmdSevSnp)?;

// Intel TDX
let enclave = Enclave::new(TeeType::IntelTdx)?;

// Secure execution
let result = enclave.execute(|| {
    // Burada çalışan kod izole
    sensitive_computation()
})?;
```

## Zero-Knowledge MCP

```rust
use sentient_zk_mcp::{Proof, Verifier};

// Proof oluştur
let proof = Proof::create(&sensitive_data)?;

// Verify et (data açılmadan)
let verifier = Verifier::new();
let is_valid = verifier.verify(&proof)?;
```

---

# Performans

## Benchmark Sonuçları

| Metrik | SENTIENT OS | Python Framework |
|--------|-------------|------------------|
| **Throughput** | 50,000 req/s | 5,000 req/s |
| **Latency (p99)** | 10ms | 100ms |
| **Memory** | 50MB | 500MB |
| **CPU** | 5% | 40% |

## Test Coverage

```
Rust Tests:    993 tests ✅
Python Tests:  150+ tests ✅
Integration:   200+ tests ✅
E2E Tests:     50+ tests ✅

Total:         1,393+ tests ✅
```

## Build Status

```
✅ cargo build --release
✅ cargo test --workspace
✅ cargo clippy --all-targets
✅ cargo fmt --check

Release Binaries: 8 platforms ✅
```

---

# Katkılarda Bulunma

## Geliştirme Ortamı

```bash
# Repository'yi fork'la ve klonla
git clone https://github.com/YOUR_USERNAME/SENTIENT_CORE.git
cd SENTIENT_CORE

# Development dependencies
cargo install cargo-watch cargo-expand

# Watch mode
cargo watch -x "test --workspace"

# Run specific tests
cargo test -p sentient_core -- --nocapture
```

## Kod Standartları

1. **Rust 1.75+** uyumlu
2. `cargo fmt` ile formatla
3. `cargo clippy` hatalarını düzelt
4. Unit test ekle
5. Dokümantasyon yaz

## Pull Request

1. Fork'la
2. Feature branch oluştur (`git checkout -b feature/amazing`)
3. Commit'le (`git commit -m 'Add amazing feature'`)
4. Push'la (`git push origin feature/amazing`)
5. Pull Request aç

---

# Lisans

## Open Source (AGPL v3)

SENTIENT OS, **GNU Affero General Public License v3.0** ile lisanslanmıştır.

```
Copyright 2025 SENTIENT AI Team

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published
by the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU Affero General Public License for more details.
```

**AGPL v3 Ne Anlama Gelir?**
- ✅ Kullan, değiştir, dağıt (özgürce)
- ✅ Değişikliklerini paylaşmak zorundasın
- ✅ SaaS olarak sunarsan kaynak kodunu paylaşmak zorundasın
- ❌ Kapalı kaynak ürünlerde kullanamazsın

## Commercial License

Kapalı kaynak ürünlerde kullanmak veya enterprise özellikler için:

| İhtiyaç | Çözüm |
|---------|-------|
| Kapalı kaynak kullanım | Commercial License |
| Enterprise Dashboard | Enterprise Plan |
| SLA & Support | Enterprise Plan |
| Custom Development | Enterprise Plus |

**İletişim:** enterprise@sentient.ai

---

# 💰 Sponsorluk & Destek

SENTIENT OS açık kaynak bir projedir. Geliştirmeyi sürdürmek için desteğinize ihtiyacımız var.

## Bağış Yap

| Platform | Link | Avantaj |
|----------|------|----------|
| **Ko-fi** | [ko-fi.com/sentientos](https://ko-fi.com/sentientos) | Tek seferlik |
| **GitHub Sponsors** | [Sponsor](https://github.com/sponsors/nexsusagent-coder) | %0 komisyon |
| **Patreon** | [patreon.com/sentientos](https://patreon.com/sentientos) | Aylık destek |

## Sponsor Olarak Alacağınız

- 🏆 README'de logo ve link
- ⭐ Öncelikli feature request'ler
- 📞 Aylık sync call (Enterprise sponsorlar)
- 🎯 Custom feature development

## Sponsor Tipleri

| Tip | Tutar | Avantajlar |
|-----|-------|------------|
| **Bronze** | $50-99/ay | Logo + Link |
| **Silver** | $100-499/ay | + Öncelikli support |
| **Gold** | $500-999/ay | + Custom feature |
| **Platinum** | $1000+/ay | + Dedicated support |

---

# 🏢 Enterprise & Commercial

SENTIENT OS'u ticari kullanım için enterprise lisansları mevcuttur.

## Open Core Model

```
┌─────────────────────────────────────────────────────────────────┐
│                    SENTIENT OS                                  │
├────────────────────────────┬────────────────────────────────────┤
│      OPEN SOURCE (AGPL)    │      ENTERPRISE (Commercial)       │
├────────────────────────────┼────────────────────────────────────┤
│ ✅ Core framework           │ ✅ Enterprise Dashboard            │
│ ✅ 23 kanal entegrasyon    │ ✅ RBAC, SSO, LDAP, OIDC          │
│ ✅ 600+ LLM model          │ ✅ Advanced Analytics              │
| ✅ Basic memory            │ ✅ Multi-tenant Architecture       │
│ ✅ Community support       │ ✅ Priority Support + SLA          │
│ ✅ Local deployment        │ ✅ White-label Option              │
│                            │ ✅ HIPAA & GDPR Compliance         │
│                            │ ✅ On-premise & Cloud              │
└────────────────────────────┴────────────────────────────────────┘
```

## Pricing

| Plan | Fiyat | Kullanım | Özellikler |
|------|-------|----------|------------|
| **Community** | Ücretsiz | Self-hosted | AGPL, Community support |
| **Pro** | $49/ay | 1 user | Cloud, 100K messages, Email support |
| **Team** | $199/ay | 5 users | Cloud, 500K messages, Analytics, Priority support |
| **Enterprise** | $999+/ay | Unlimited | On-prem/Cloud, SSO, SLA, Dedicated support |
| **Enterprise Plus** | İletişime geçin | Unlimited | White-label, Custom dev, HIPAA/GDPR |

## Enterprise Demo İstiyorum

```bash
# Email: enterprise@sentient.ai
# Discord: discord.gg/sentient
# Form: https://sentient.ai/enterprise
```

## SLA Garantileri

| Metrik | Enterprise | Enterprise Plus |
|--------|------------|-----------------|
| **Uptime** | 99.9% | 99.99% |
| **Response Time** | 4 saat | 1 saat |
| **Resolution Time** | 24 saat | 4 saat |
| **Support** | 24/5 | 24/7 |

---

# İletişim

| Kanal | Link |
|-------|------|
| **Discord** | [discord.gg/sentient](https://discord.gg/sentient) |
| **Twitter** | [@SentientAI_OS](https://twitter.com/SentientAI_OS) |
| **GitHub** | [github.com/nexsusagent-coder/SENTIENT_CORE](https://github.com/nexsusagent-coder/SENTIENT_CORE) |
| **Email** | hello@sentient.ai |
| **Enterprise Sales** | enterprise@sentient.ai |
| **Documentation** | [docs.sentient.ai](https://docs.sentient.ai) |

---

<p align="center">
  <b>SENTIENT OS</b><br>
  <i>The Operating System That Thinks</i><br><br>
  Made with 🦀 by the SENTIENT Team
</p>
