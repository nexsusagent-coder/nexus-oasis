<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/License-Apache%202.0-blue" alt="License">
  <img src="https://img.shields.io/badge/Platforms-Linux%20%7C%20macOS%20%7C%20Windows-green" alt="Platforms">
  <img src="https://img.shields.io/badge/LLM%20Models-408%2B-purple" alt="LLM Models">
  <img src="https://img.shields.io/badge/Skills-5%2C587%2B-yellow" alt="Skills">
</p>

<h1 align="center">🧠 SENTIENT OS</h1>
<h3 align="center">The Operating System That Thinks</h3>
<p align="center"><i>Enterprise-Grade AI Agent Framework with Rust Core</i></p>

---

## 📖 İçindekiler

- [SENTIENT Nedir?](#-sentient-nedir)
- [Özellikler](#-özellikler)
- [Hızlı Kurulum](#-hızlı-kurulum)
- [Detaylı Kurulum](#-detaylı-kurulum)
- [Mimari](#-mimari)
- [Modüller](#-modüller)
- [LLM Desteği](#-llm-desteği)
- [Skill Sistemi](#-skill-sistemi)
- [Entegrasyonlar](#-entegrasyonlar)
- [API Kullanımı](#-api-kullanımı)
- [CLI Komutları](#-cli-komutları)
- [Web Dashboard](#-web-dashboard)
- [Geliştirici Rehberi](#-geliştirici-rehberi)
- [Katkıda Bulunma](#-katkıda-bulunma)
- [Lisans](#-lisans)

---

## 🧠 SENTIENT Nedir?

**SENTIENT OS**, AI agent'larını yönetmek, çalıştırmak ve ölçeklendirmek için tasarlanmış **Rust-tabanlı** bir işletim sistemidir. Geleneksel AI framework'lerinin aksine, SENTIENT:

| Özellik | Açıklama |
|---------|----------|
| **🦀 Rust Core** | Memory-safe, high-performance, zero-cost abstractions |
| **🔌 Modüler** | 59 crate, her biri bağımsız ve değiştirilebilir |
| **🧩 72 Entegrasyon** | CrewAI, Mem0, Browser-Use, Lightpanda, ve daha fazlası |
| **🎯 5,587+ Skill** | Hazır AI yetenekleri, tek komutla aktif |
| **🤖 408+ LLM Model** | OpenAI, Claude, Gemini, Ollama, 35+ provider |
| **🔐 V-GATE** | API anahtarları ASLA client'ta yok |
| **📊 Dashboard** | Enterprise war room, gerçek zamanlı izleme |
| **⚡ High Performance** | 10x Python alternatiflerine göre |

---

## ✨ Özellikler

### 🧠 Multi-LLM Support

```rust
use sentient_core::{Agent, LlmProvider};

// 408+ model desteği
let agent = Agent::new("claude-3-5-sonnet")
    .with_fallback(["gpt-4o", "gemini-2.0-flash", "ollama://llama3.3"])
    .build();
```

### 🔧 Skill System

```bash
# 5,587+ skill hazır
sentient skill list
sentient skill run code-review --path ./src
sentient skill run web-scraper --url "https://example.com"
```

### 🔐 V-GATE Security

```
┌─────────────┐      ┌─────────────┐      ┌─────────────┐
│   SENTIENT  │      │   V-GATE    │      │   LLM API   │
│   Client    │─────▶│   Proxy     │─────▶│   Provider  │
└─────────────┘      └─────────────┘      └─────────────┘
                           │
                     API Key (Secure)
                     Server-side only
```

### 📊 Real-time Dashboard

```bash
sentient dashboard
# → http://localhost:8080
```

---

## 🚀 Hızlı Kurulum

### Tek Komutla Kurulum (Linux/macOS)

```bash
curl -sSL https://get.sentient.ai | bash
```

### Windows (PowerShell)

```powershell
irm https://get.sentient.ai/ps | iex
```

### Docker

```bash
docker run -it sentientai/sentient:latest
```

### Cargo

```bash
cargo install sentient-os
```

---

## 📦 Detaylı Kurulum

### Gereksinimler

| Gereksinim | Minimum | Önerilen |
|------------|---------|----------|
| Rust | 1.75+ | 1.80+ |
| RAM | 4GB | 16GB+ |
| Disk | 2GB | 10GB+ |
| OS | Linux/macOS/Windows | Ubuntu 22.04 |

### Kaynaktan Derleme

```bash
# Repository'yi klonla
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
cd SENTIENT_CORE

# Rust kurulu değilse
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Derle (release mod)
cargo build --release

# Kurulum
cargo install --path .

# İlk kurulum sihirbazı
sentient setup
```

### İlk Kurulum Sihirbazı

```bash
$ sentient setup

╔══════════════════════════════════════════════════════════════╗
║            SENTIENT OS Setup Wizard v2.1.0                  ║
╠══════════════════════════════════════════════════════════════╣
║                                                              ║
║  [1] LLM Provider Selection                                  ║
║      ├─ OpenAI                                              ║
║      ├─ Anthropic (Claude)                                  ║
║      ├─ Google (Gemini)                                     ║
║      ├─ Ollama (Local)                                      ║
║      ├─ Groq (Fast)                                         ║
║      └─ [35+ more providers]                                ║
║                                                              ║
║  [2] Model Selection                                        ║
║      ├─ KERNEL DEFAULT: Gemma 4 (12B)                      ║
║      ├─ OpenAI: GPT-4o, o1-preview, o1-mini                ║
║      ├─ Anthropic: Claude 3.5 Sonnet, Claude 3 Opus       ║
║      ├─ Google: Gemini 2.0 Flash, Gemini 1.5 Pro          ║
║      └─ Ollama: 54 local models                            ║
║                                                              ║
║  [3] API Key Configuration                                  ║
║      → V-GATE Proxy handles keys securely                  ║
║      → Keys NEVER stored locally                           ║
║                                                              ║
║  [4] Skill Selection (5,587 available)                      ║
║      ├─ Code Analysis                                       ║
║      ├─ Web Scraping                                        ║
║      ├─ Data Processing                                     ║
║      └─ [5584 more...]                                      ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
```

---

## 🏗️ Mimari

### Katmanlı Mimari

```
┌─────────────────────────────────────────────────────────────────────┐
│                         USER LAYER                                   │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │
│  │    CLI       │  │  Dashboard   │  │  REST API    │              │
│  │  (sentient)  │  │  (Web UI)    │  │  (sentient_web)│             │
│  └──────────────┘  └──────────────┘  └──────────────┘              │
├─────────────────────────────────────────────────────────────────────┤
│                       ORCHESTRATION LAYER                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │
│  │ Orchestrator │  │   Swarm      │  │   RAG        │              │
│  │ (multi-agent)│  │ (collective)│  │ (retrieval)  │              │
│  └──────────────┘  └──────────────┘  └──────────────┘              │
├─────────────────────────────────────────────────────────────────────┤
│                         CORE LAYER                                   │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │
│  │ Agent Engine │  │ Event Graph  │  │   Memory     │              │
│  │ (sentient_core)│(sentient_graph)│(sentient_memory)│             │
│  └──────────────┘  └──────────────┘  └──────────────┘              │
├─────────────────────────────────────────────────────────────────────┤
│                     INTEGRATION LAYER                                │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │
│  │ PyO3 Bridge  │  │ MCP Protocol │  │  Plugins     │              │
│  │(sentient_python)│(sentient_mcp) │ (sentient_plugin)│             │
│  └──────────────┘  └──────────────┘  └──────────────┘              │
├─────────────────────────────────────────────────────────────────────┤
│                       SECURITY LAYER                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │
│  │   V-GATE     │  │  Guardrails  │  │    TEE       │              │
│  │ (API Proxy)  │  │(sentient_guardrails)│(AMD SEV/Intel TDX)│      │
│  └──────────────┘  └──────────────┘  └──────────────┘              │
├─────────────────────────────────────────────────────────────────────┤
│                      EXECUTION LAYER                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │
│  │   Sandbox    │  │   Browser    │  │   Vision     │              │
│  │(Docker/WASM) │  │(Lightpanda)  │  │(sentient_vision)│            │
│  └──────────────┘  └──────────────┘  └──────────────┘              │
└─────────────────────────────────────────────────────────────────────┘
```

### Event-Driven Architecture

```rust
use sentient_graph::{EventBus, Event, EventHandler};

// Event-driven communication
let bus = EventBus::new();

// Agent event yayını
bus.emit(Event::AgentCreated { id: "agent-1".into() });

// Skill event'i
bus.emit(Event::SkillExecuted {
    skill: "code-review".into(),
    result: "✅ 3 issues found".into(),
});
```

---

## 📦 Modüller

SENTIENT OS **59 crate** içerir. Her biri bağımsız ve modüler:

### Core Crates

| Crate | Satır | Açıklama |
|-------|-------|----------|
| `sentient_core` | 4,200 | Agent engine, event system |
| `sentient_graph` | 3,100 | Lock-free event graph |
| `sentient_memory` | 2,800 | Episodic/semantic memory |
| `sentient_orchestrator` | 3,400 | Multi-agent orchestration |
| `sentient_cevahir` | 2,900 | LLM engine + cognitive reasoning |

### LLM Crates

| Crate | Satır | Açıklama |
|-------|-------|----------|
| `sentient_llm` | 2,100 | Multi-provider LLM interface |
| `sentient_local` | 1,800 | Local models (Gemma 4, GPT4All, Ollama) |
| `sentient_finetuning` | 2,195 | LoRA, QLoRA fine-tuning |

### Integration Crates

| Crate | Satır | Açıklama |
|-------|-------|----------|
| `sentient_mcp` | 3,003 | Model Context Protocol |
| `sentient_python` | 2,400 | PyO3 bridge |
| `sentient_plugin` | 2,868 | Plugin system |
| `sentient_rag` | 3,368 | RAG engine |

### Execution Crates

| Crate | Satır | Açıklama |
|-------|-------|----------|
| `oasis_browser` | 3,200 | Browser automation |
| `oasis_autonomous` | 2,900 | Autonomous execution |
| `sentient_sandbox` | 1,600 | Docker sandbox |
| `sentient_vision` | 2,201 | Vision/multimodal |

### Security Crates

| Crate | Satır | Açıklama |
|-------|-------|----------|
| `sentient_guardrails` | 2,100 | Input/output filtering |
| `sentient_vault` | 1,800 | Secret management |
| `sentient_tee` | 1,200 | TEE (AMD SEV, Intel TDX) |
| `sentient_zk_mcp` | 900 | Zero-knowledge proofs |

### Web Crates

| Crate | Satır | Açıklama |
|-------|-------|----------|
| `sentient_web` | 1,406 | REST API + WebSocket |
| `sentient_gateway` | 1,500 | API gateway |
| `dashboard` | 84,266 | Enterprise dashboard |

---

## 🤖 LLM Desteği

### Desteklenen Provider'lar (35+)

| Provider | Modeller | Özellikler |
|----------|----------|------------|
| **OpenAI** | GPT-4o, o1, o3, GPT-4.5 | Function calling, vision |
| **Anthropic** | Claude 3.5/3.7 Sonnet, Opus, Haiku | Extended thinking |
| **Google** | Gemini 2.0 Flash/Pro, Gemma 4 | Multimodal, local |
| **OpenRouter** | 70+ models | Tek API, tüm provider'lar |
| **Ollama** | 54 models | %100 local, offline |
| **Groq** | Llama 3.3, Mixtral | Ultra-fast inference |
| **DeepSeek** | R1, V3 | Reasoning, code |
| **Mistral** | Large, Medium, Small | European AI |
| **X.AI** | Grok-2 | Twitter entegrasyonu |
| **Together AI** | 30+ open-source models | Fine-tuning |

### Model Kullanımı

```rust
use sentient_llm::{LlmClient, Message};

// OpenAI
let client = LlmClient::openai(api_key);
let response = client.chat("gpt-4o", "Merhaba!").await?;

// Claude (Extended Thinking)
let client = LlmClient::anthropic(api_key);
let response = client.chat_thinking("claude-3-7-sonnet", "Analiz et").await?;

// Local (Ollama)
let client = LlmClient::ollama(); // localhost:11434
let response = client.chat("llama3.3:70b", "Local response").await?;

// Gemma 4 (KERNEL DEFAULT)
let response = client.chat("gemma-4:12b", "Code review").await?;
```

### Fallback Chain

```rust
// Otomatik fallback
let agent = Agent::new("primary-model")
    .with_fallback([
        "gpt-4o",
        "claude-3-5-sonnet",
        "gemini-2.0-flash",
        "ollama://llama3.3"
    ])
    .build();
```

---

## 🎯 Skill Sistemi

SENTIENT OS **5,587+ hazır skill** içerir:

### Kategoriler

| Kategori | Skill Sayısı | Örnekler |
|----------|-------------|----------|
| **Code** | 1,200+ | code-review, refactor, debug, test-gen |
| **Web** | 800+ | scrape, crawl, form-fill, screenshot |
| **Data** | 600+ | etl, transform, analyze, visualize |
| **Security** | 400+ | pentest, audit, encrypt, hash |
| **Communication** | 300+ | email, slack, discord, telegram |
| **DevOps** | 250+ | deploy, monitor, scale, backup |
| **Research** | 200+ | search, summarize, cite, translate |
| **Automation** | 150+ | schedule, trigger, pipeline, workflow |

### Skill Kullanımı

```bash
# Skill listesi
sentient skill list --category code

# Skill çalıştır
sentient skill run code-review --path ./src --language rust

# Interactive skill
sentient skill run web-scraper --url "https://example.com" --interactive

# Skill detayları
sentient skill info code-review
```

### Skill Oluşturma

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

# Skill yayınla (marketplace)
sentient skill publish my-custom-skill
```

---

## 🔌 Entegrasyonlar

SENTIENT OS **72 proje** entegre eder:

### Agent Frameworks

| Proje | Lisans | Kullanım |
|-------|--------|----------|
| CrewAI | MIT | Multi-agent orchestration |
| AutoGen | MIT | Conversational agents |
| LangChain | MIT | LLM chains |
| Semantic Kernel | MIT | Microsoft AI SDK |

### Browser & Web

| Proje | Lisans | Kullanım |
|-------|--------|----------|
| Browser-Use | MIT | Browser automation |
| Lightpanda | MIT | Headless browser (Zig) |
| Playwright | Apache-2 | Cross-browser testing |

### Memory & RAG

| Proje | Lisans | Kullanım |
|-------|--------|----------|
| Mem0 | MIT | Long-term memory |
| MemGPT | Apache-2 | Memory management |
| ChromaDB | Apache-2 | Vector database |
| LanceDB | Apache-2 | Serverless vectors |

### Security

| Proje | Lisans | Kullanım |
|-------|--------|----------|
| NeMo-Guardrails | Apache-2 | Input/output filtering |
| MoltGuard | MIT | Prompt injection defense |

---

## 🖥️ CLI Komutları

### Temel Komutlar

```bash
# Sistem durumu
sentient status

# Agent yönetimi
sentient agent list
sentient agent create my-agent --model gpt-4o
sentient agent run my-agent --goal "Build API"

# Skill yönetimi
sentient skill list
sentient skill run <skill-name>
sentient skill search <query>

# Model yönetimi
sentient model list
sentient model set gpt-4o
sentient model test

# Dashboard
sentient dashboard --port 8080
```

### Gelişmiş Komutlar

```bash
# Swarm orchestration
sentient swarm create team-1 --agents 5
sentient swarm run team-1 --goal "Complex task"

# Pipeline
sentient pipeline run analyze-transform-deploy

# Memory
sentient memory list
sentient memory search "previous conversation"

# Debug
sentient debug agent my-agent --trace
sentient logs --follow
```

---

## 🌐 Web Dashboard

### Başlatma

```bash
# Dashboard başlat
sentient dashboard

# Özel port
sentient dashboard --port 3000

# Auth ile
sentient dashboard --auth --jwt-secret secret123
```

### Özellikler

| Özellik | Açıklama |
|---------|----------|
| **3D Topology** | Three.js ile agent görselleştirme |
| **Real-time Logs** | WebSocket ile canlı güncelleme |
| **xterm.js Terminal** | Entegre terminal |
| **Skill Cards** | 1Password tarzı skill kartları |
| **Security Panel** | Canlı güvenlik metrikleri |
| **Chat Interface** | Agent ile sohbet |

### API Endpoints

```
GET  /                    # Dashboard UI
GET  /health              # Health check
GET  /api/v1/status       # Sistem durumu
GET  /api/v1/agents       # Agent listesi
POST /api/v1/agents       # Agent oluştur
GET  /api/v1/skills       # Skill listesi
POST /api/v1/chat         # Agent ile sohbet
WS   /ws                   # WebSocket
```

---

## 👨‍💻 Geliştirici Rehberi

### Proje Yapısı

```
SENTIENT_CORE/
├── crates/                 # 59 Rust crate
│   ├── sentient_core/      # Ana motor
│   ├── sentient_llm/       # LLM interface
│   ├── sentient_memory/    # Bellek sistemi
│   ├── sentient_mcp/       # MCP protocol
│   ├── sentient_rag/       # RAG engine
│   ├── sentient_vision/    # Vision AI
│   ├── sentient_plugin/    # Plugin system
│   ├── sentient_web/       # Web server
│   └── ...
├── dashboard/              # Web UI
├── integrations/           # 72 entegrasyon
├── tools/                  # CLI araçları
├── data/                   # Veri dizini
├── config/                 # Yapılandırma
└── tests/                  # Entegrasyon testleri
```

### Yeni Crate Oluşturma

```bash
# Yeni crate oluştur
cargo new --lib crates/sentient_newfeature

# Cargo.toml'a ekle
echo 'sentient_newfeature = { path = "crates/sentient_newfeature" }' >> Cargo.toml
```

### Örnek Crate

```rust
// crates/sentient_newfeature/src/lib.rs
use sentient_core::{Agent, Skill, Result};

pub struct NewFeature {
    config: NewFeatureConfig,
}

impl NewFeature {
    pub fn new(config: NewFeatureConfig) -> Self {
        Self { config }
    }

    pub async fn execute(&self, input: &str) -> Result<String> {
        // Implementation
        Ok(format!("Processed: {}", input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_feature() {
        let feature = NewFeature::new(Default::default());
        assert!(feature.execute("test").await.is_ok());
    }
}
```

### Test Çalıştırma

```bash
# Tüm testler
cargo test --workspace

# Belirli crate
cargo test -p sentient_core

# Coverage
cargo tarpaulin --workspace --out Html
```

---

## 🤝 Katkıda Bulunma

1. Fork'la
2. Branch oluştur (`git checkout -b feature/amazing`)
3. Commit'le (`git commit -m 'Add amazing feature'`)
4. Push'la (`git push origin feature/amazing`)
5. Pull Request aç

### Kod Standartları

- Rust 1.75+ uyumlu
- `cargo fmt` ile formatla
- `cargo clippy` hatalarını düzelt
- Unit test ekle
- Dokümantasyon yaz

---

## 📄 Lisans

Apache License 2.0

```
Copyright 2024 SENTIENT AI

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0
```

---

## 📞 İletişim

- **Discord**: [discord.gg/sentient](https://discord.gg/sentient)
- **Twitter**: [@SentientAI_OS](https://twitter.com/SentientAI_OS)
- **GitHub**: [github.com/nexsusagent-coder/SENTIENT_CORE](https://github.com/nexsusagent-coder/SENTIENT_CORE)
- **Email**: hello@sentient.ai

---

<p align="center">
  <b>SENTIENT OS</b><br>
  <i>The Operating System That Thinks</i>
</p>
