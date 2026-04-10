# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS - TEMEL YAPI VE SİSTEM RAPORU
# ═══════════════════════════════════════════════════════════════════════════════
#  Tarih: 2026-04-10
#  Geliştirici: Pi
#  Sürüm: 1.0.0
# ═══════════════════════════════════════════════════════════════════════════════

## 🎯 MİSYON VE AMAÇ

**SENTIENT OS**, otonom, güvenli ve yüksek performanslı bir AI İşletim Sistemi'dir. 
Rust çekirdeği üzerine inşa edilmiş, Python entegrasyonlarıyla zenginleştirilmiş 
bir platform olarak farklı open-source projelerin uyumlu bir şekilde çalışmasını sağlar.

### Temel Özellikler

| Özellik | Açıklama |
|---------|----------|
| **Otonom Ajentlar** | Kendi kendine görev yapabilen AI ajanları |
| **Bilişsel Bellek** | Kısa/uzun süreli hafıza ve bilgi grafları |
| **Web Etkileşimi** | Headless browser otomasyonu |
| **Güvenli Sandbox** | Docker tabanlı izole kod çalıştırma |
| **Çoklu LLM Desteği** | OpenAI, Claude, Gemini, Ollama, GPT4All |
| **Skill Sistemi** | 5,587+ hazır skill ile genişletilebilirlik |

---

## 📊 SİSTEM İSTATİSTİKLERİ

| Metrik | Değer |
|--------|-------|
| **Rust Crate** | 53 |
| **Rust Dosyası** | 684 |
| **Kod Satırı** | 148,323 |
| **Test Sayısı** | 1,358 |
| **Entegre GitHub Repo** | 72 |
| **Skill Sayısı** | 5,587+ |
| **Tool Sayısı** | 75+ |

---

## 🏗️ MİMARİ KATMANLAR

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           L7: PRESENTATION                                   │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │   CLI       │  │  Dashboard  │  │    API      │  │   Voice     │        │
│  │ sentient_cli│  │  TUI/Web    │  │sentient_gw  │  │sentient_voice│       │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
┌─────────────────────────────────────────────────────────────────────────────┐
│                           L6: ORCHESTRATION                                  │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                    sentient_orchestrator                             │    │
│  │      Agent Loop • Task Queue • Event Processing • Tool Chain        │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
┌─────────────────────────────────────────────────────────────────────────────┐
│                           L5: EXECUTION                                      │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │oasis_browser│  │oasis_manus  │  │oasis_hands  │  │ sentient_sb │        │
│  │ Web Agent   │  │ Code Exec   │  │ Desktop     │  │ Docker      │        │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
┌─────────────────────────────────────────────────────────────────────────────┐
│                           L4: COGNITION                                      │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │sentient_mem │  │sentient_cev │  │sentient_res │  │sentient_per │        │
│  │ Hippocampus │  │ LLM Engine  │  │ Research    │  │ Persona     │        │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
┌─────────────────────────────────────────────────────────────────────────────┐
│                           L3: INTEGRATION                                    │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │sentient_py  │  │sentient_skill│  │sentient_ch  │  │sentient_vec │        │
│  │ PyO3 Bridge │  │ Skills      │  │ Channels    │  │ Vector DB   │        │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
┌─────────────────────────────────────────────────────────────────────────────┐
│                           L2: SECURITY                                       │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │sentient_grd │  │sentient_tee │  │sentient_zk  │  │ oasis_vault │        │
│  │ Guardrails  │  │ TEE/Enclave │  │ ZK-MCP      │  │ Secrets     │        │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
┌─────────────────────────────────────────────────────────────────────────────┐
│                           L1: CORE                                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │sentient_core│  │sentient_grph│  │sentient_comm│  │ oasis_core  │        │
│  │ Event Graph │  │ Lock-free   │  │ Common Lib  │  │ Creusot     │        │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 📦 CRATE KATALOĞU

### OASIS Serisi (Execution Layer)

| Crate | Açıklama | Satır |
|-------|----------|-------|
| `oasis_core` | Trusted Runtime with Creusot Contracts | ~500 |
| `oasis_browser` | Web Agent - Browser Automation | ~3,500 |
| `oasis_hands` | Desktop Agent - Mouse/Keyboard Control | ~2,800 |
| `oasis_manus` | Code Execution Agent | ~1,200 |
| `oasis_vault` | Military-Grade Secrets Manager | ~1,500 |
| `oasis_autonomous` | Fully Autonomous Desktop Agent | ~1,800 |
| `oasis_brain` | Autonomous Thinking Module | ~600 |

### SENTIENT Çekirdek

| Crate | Açıklama | Satır |
|-------|----------|-------|
| `sentient_core` | Central Core Library | ~2,500 |
| `sentient_graph` | Lock-free Event Graph | ~1,200 |
| `sentient_common` | Shared Utilities | ~800 |
| `sentient_orchestrator` | Agent Loop & Task Management | ~2,200 |

### Bilişsel Katman

| Crate | Açıklama | Satır |
|-------|----------|-------|
| `sentient_memory` | Hippocampus - RAG Memory | ~2,800 |
| `sentient_cevahir` | LLM Engine (Cevahir AI) | ~1,500 |
| `sentient_research` | MindSearch Integration | ~1,800 |
| `sentient_persona` | Personality System | ~600 |
| `sentient_vector` | Vector Database Interface | ~400 |

### Güvenlik Katmanı

| Crate | Açıklama | Satır |
|-------|----------|-------|
| `sentient_guardrails` | Input/Output Security Filters | ~900 |
| `sentient_tee` | Trusted Execution Environment | ~700 |
| `sentient_zk_mcp` | Zero-Knowledge Proofs | ~800 |

### Entegrasyon Katmanı

| Crate | Açıklama | Satır |
|-------|----------|-------|
| `sentient_python` | PyO3 Bridge | ~600 |
| `sentient_skills` | Skill Management System | ~1,200 |
| `sentient_skills_import` | Skill Importer | ~400 |
| `sentient_channels` | Multi-Platform Messaging | ~1,500 |

### Çalışma Zamanı

| Crate | Açıklama | Satır |
|-------|----------|-------|
| `sentient_cli` | Command Line Interface | ~2,500 |
| `sentient_gateway` | HTTP/WebSocket Gateway | ~1,800 |
| `sentient_voice` | Speech-to-Text/Text-to-Speech | ~500 |
| `sentient_dashboard` | Web Dashboard (TUI) | ~800 |

### Enterprise

| Crate | Açıklama | Satır |
|-------|----------|-------|
| `sentient_enterprise` | SSO, RBAC, Audit, Tenancy | ~1,500 |
| `sentient_cluster` | Kubernetes Operator | ~600 |
| `sentient_observability` | Prometheus Metrics | ~400 |
| `sentient_anomaly` | Real-time Anomaly Detection | ~500 |

### Diğer Modüller

| Crate | Açıklama |
|-------|----------|
| `sentient_benchmarks` | Performance Benchmarking Suite |
| `sentient_checkpoint` | Ratchet Pattern Progress Saving |
| `sentient_devtools` | AI-Powered Development Tools |
| `sentient_execution` | Secure Code Execution |
| `sentient_forge` | Agent Generation from Data |
| `sentient_i18n` | Internationalization (8 Languages) |
| `sentient_ingestor` | Mass Skill Assimilation |
| `sentient_lancedb` | Long-term Memory Storage |
| `sentient_local` | Local LLM Integration |
| `sentient_marketplace` | Skills Discovery |
| `sentient_modes` | Six Operation Modes |
| `sentient_reporting` | Research Report Generation |
| `sentient_sandbox` | Docker Isolation |
| `sentient_scout` | Social Media Data Extraction |
| `sentient_selfcoder` | Self-Improving Code |
| `sentient_session` | Session Tree Management |
| `sentient_settings` | Multi-Key Vault Settings |
| `sentient_setup` | Interactive Setup Wizard |
| `sentient_storage` | Task Persistence Layer |
| `sentient_sync` | Silent Auto-Update Engine |
| `sentient_vgate` | Vekil Sunucu (API Proxy) |
| `sentient_wake` | Wake Word Detection |

---

## 🛠️ TOOLS SİSTEMİ

### Tool Trait Arayüzü

```rust
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters_schema(&self) -> serde_json::Value;
    async fn execute(&self, params: serde_json::Value) -> ToolResult;
}
```

###SENTIENT Native Tools (30+)

| Tool | Açıklama | Kategori |
|------|----------|----------|
| `bash_tool` | Shell komutları çalıştırma | System |
| `file_read_tool` | Dosya okuma | File |
| `file_write_tool` | Dosya yazma | File |
| `file_edit_tool` | Dosya düzenleme | File |
| `grep_tool` | Regex arama | Search |
| `glob_tool` | Glob pattern dosya bulma | Search |
| `web_search_tool` | Web araması | Web |
| `web_fetch_tool` | Web sayfası çekme | Web |
| `browser_tool` | Browser otomasyonu | Web |
| `screenshot_tool` | Ekran görüntüsü | Media |
| `git_tool` | Git işlemleri | Dev |
| `lsp_tool` | Language Server Protocol | Dev |
| `mcp_tool` | MCP protokolü | Integration |
| `skill_tool` | Skill çalıştırma | Skills |
| `agent_tool` | Alt agent başlatma | Agents |
| `memory_tool` | Bellek işlemleri | Memory |
| `calendar_tool` | Takvim yönetimi | Productivity |
| `email_tool` | E-posta gönderme | Communication |
| `notify_tool` | Bildirim gönderme | System |
| `pdf_tool` | PDF işleme | Documents |
| `translate_tool` | Çeviri | NLP |
| `todo_write_tool` | TODO yönetimi | Productivity |
| `ask_user_question_tool` | Kullanıcı sorgusu | Interaction |
| `config_tool` | Konfigürasyon | System |
| `task_tool` | Görev yönetimi | System |
| `n8n_tool` | n8n workflow | Automation |
| `brief_tool` | Brief oluşturma | Reporting |
| `sed_tool` | Stream Editor | Text |

### OpenHarness Tools (45+)

OpenHarness uyumlu tool wrapper'ları:
- `openharness_bash_tool`
- `openharness_file_read_tool`
- `openharness_file_write_tool`
- `openharness_file_edit_tool`
- `openharness_grep_tool`
- `openharness_glob_tool`
- `openharness_web_search_tool`
- `openharness_web_fetch_tool`
- `openharness_mcp_tool`
- `openharness_skill_tool`
- `openharness_agent_tool`
- `openharness_task_create_tool`
- `openharness_task_list_tool`
- `openharness_task_get_tool`
- `openharness_task_update_tool`
- `openharness_task_stop_tool`
- `openharness_team_create_tool`
- `openharness_team_delete_tool`
- `openharness_cron_create_tool`
- `openharness_cron_list_tool`
- `openharness_cron_toggle_tool`
- `openharness_cron_delete_tool`
- `openharness_lsp_tool`
- `openharness_sleep_tool`
- `openharness_brief_tool`
- ... ve daha fazlası

---

## 🎭 SKILLS SİSTEMİ

### Skill Formatı

```yaml
name: skill-name
description: What this skill does
author: creator-name
tags: [tag1, tag2]
github_url: https://github.com/...
```

### Skill Kategorileri (5,587+ Skills)

| Kategori | Skill Sayısı | Açıklama |
|----------|--------------|----------|
| **Dev** | 2,965+ | Coding, Web, DevOps, CLI Tools |
| **OSINT** | 1,050+ | Search, Research, Browser Automation |
| **Social** | 238+ | Communication, Marketing |
| **Automation** | 306+ | Productivity, Calendar, Smart Home |
| **Media** | 246+ | Image/Video, Streaming, Speech |
| **Productivity** | 214+ | Notes, PDF, Apple Apps |
| **Security** | 52+ | Security, Passwords |
| **Mobile** | 233+ | Transportation, Health, Shopping |
| **Gaming** | 108+ | Gaming, Personal Development |

### Skill Kaynakları

| Kaynak | Skill Sayısı | Lisans |
|--------|--------------|--------|
| awesome-openclaw-skills | 5,143 | CC0 |
| everything-claude-code | 181 | MIT |
| gstack-skills | 37 | MIT |
| deerflow-skills | 20+ | MIT |
| OpenHarness Community | 200+ | MIT |

### Native Skills

```
skills/
├── Automation/       → Calendar, Productivity, Smart-Home
├── Data/            → Data processing skills
├── Dev/             → CLI-Tools, Coding-Agents, DevOps-Cloud, Git-GitHub
├── Gaming/          → Gaming, Moltbook, Personal-Dev
├── Media/           → Image-Video-Gen, Streaming, Speech
├── Mobile/          → Health-Fitness, Shopping, Transportation
├── OSINT/           → Search, Research, Browser-Automation
├── Productivity/    → Notes, PDF, Apple-Apps
├── Security/        → Security-Passwords
├── Social/          → Communication, Marketing-Sales
├── analysis/        → Deep analysis skill
├── automation/      → Task automation skill
├── code-review/     → Code review skill
├── codegen/         → Code generation skill
├── research/        → Deep research skill
└── web-researcher/  → Web research skill
```

---

## 🔗 ENTEGRE GITHUB REPOLARI (72)

### Agent Frameworks (17)

| Proje | Açıklama |
|-------|----------|
| `crewai` | Multi-agent orchestration |
| `autogen` | Microsoft AutoGen |
| `autogen-studio` | AutoGen UI |
| `openhands` | OpenHands agent |
| `auto-gpt` | AutoGPT |
| `babyagi` | BabyAGI |
| `metagpt` | MetaGPT |
| `swarm` | OpenAI Swarm |
| `praisonai` | PraisonAI |
| `camel-ai` | CAMEL-AI |
| `taskweaver` | Microsoft TaskWeaver |
| `gpt-engineer` | GPT Engineer |
| `agent-s` | Agent-S |
| `agency-agents` | Agency Swarm |
| `agentgpt` | AgentGPT |
| `autoresearch` | AutoResearch |
| `goose` | Goose Agent |

### LLM Frameworks (21)

| Proje | Açıklama |
|-------|----------|
| `langchain` | LangChain |
| `llama_index` | LlamaIndex |
| `haystack` | Haystack |
| `phidata` | Phidata/Agno |
| `semantic-kernel` | Microsoft Semantic Kernel |
| `pydantic-ai` | PydanticAI |
| `smolagents` | HuggingFace SmolAgents |
| `ollama` | Ollama |
| `gpt4all` | GPT4All |
| `open-webui` | Open WebUI |
| `dify` | Dify |
| `fastgpt` | FastGPT |
| `continue-dev` | Continue.dev |
| `aider` | Aider |
| `storm` | STORM |
| `text-generation-webui` | Text Generation WebUI |
| `llama-recipes` | LLaMA Recipes |
| `autogluon` | AutoGluon |
| `tensorflow` | TensorFlow |
| `lms` | LMS |
| `anthropic-cookbook` | Anthropic Cookbook |

### Browser Automation (5)

| Proje | Açıklama |
|-------|----------|
| `browser-use` | Browser-Use |
| `lightpanda` | Lightpanda (Zig) |
| `agent-browser` | Agent Browser |
| `bytebot` | ByteBot |
| `open-computer-use` | Open Computer Use |

### Memory & Vector DB (4)

| Proje | Açıklama |
|-------|----------|
| `chromadb` | ChromaDB |
| `qdrant` | Qdrant |
| `weaviate` | Weaviate |
| `letta` | Letta/MemGPT |

### Tools (5)

| Proje | Açıklama |
|-------|----------|
| `mem0` | Mem0 Memory |
| `ragflow` | RAGFlow |
| `firecrawl` | Firecrawl |
| `crawl4ai` | Crawl4AI |
| `judge0` | Judge0 Code Execution |

### Security (1)

| Proje | Açıklama |
|-------|----------|
| `nemo-guardrails` | NVIDIA NeMo Guardrails |

### Sandbox (3)

| Proje | Açıklama |
|-------|----------|
| `e2b-sdk` | E2B Sandbox |
| `daytona` | Daytona |
| `localstack` | LocalStack |

### CLI Tools (2)

| Proje | Açıklama |
|-------|----------|
| `gemini-cli` | Google Gemini CLI |
| `google-workspace-cli` | Google Workspace CLI |

### Skills (6)

| Proje | Açıklama |
|-------|----------|
| `awesome-openclaw-skills` | 5,143 skills |
| `everything-claude-code` | 181 skills |
| `deerflow-skills` | DeerFlow skills |
| `gstack` | Gstack skills |
| `Claw3D` | Claw3D skills |
| `awesome-n8n-templates` | n8n templates |

### LLM Engine (1)

| Proje | Açıklama |
|-------|----------|
| `cevahir-ai` | Cevahir AI Engine |

---

## 🧠 CEVAHIR AI ENTTEGRASYONU

### Mimari Özellikler

| Özellik | Standart | Açıklama |
|---------|----------|----------|
| **RoPE** | GPT-3+/LLaMA | Rotary Position Embedding |
| **RMSNorm** | GPT-3+/LLaMA | Root Mean Square Normalization |
| **SwiGLU** | GPT-4/PaLM | Gated Linear Unit |
| **KV Cache** | GPT-4/Claude | Inference Optimizasyonu |
| **GQA** | LLaMA-2/3/Mistral | Grouped Query Attention |
| **MoE** | GPT-4/Gemini | Mixture of Experts |
| **YaRN** | LLaMA-3.1 | Uzun Context Desteği |

### Cognitive Stratejiler

```rust
use sentient_cevahir::{CevahirBridge, CognitiveStrategy};

// Basit sorgular
let output = bridge.process_with_strategy(
    "Merhaba",
    CognitiveStrategy::Direct,
).await?;

// Kod analizi
let output = bridge.process_with_strategy(
    "Bu kodu analiz et",
    CognitiveStrategy::Think,
).await?;

// Tasarım kararları
let output = bridge.process_with_strategy(
    "Hangi yaklaşım daha iyi?",
    CognitiveStrategy::Debate,
).await?;

// Karmaşık problemler
let output = bridge.process_with_strategy(
    "Bu hatanın kök nedenini bul",
    CognitiveStrategy::TreeOfThoughts,
).await?;
```

---

## 🔐 GÜVENLİK MİMARİSİ

### V-GATE (API Proxy)

```
┌─────────────┐      ┌─────────────┐      ┌─────────────┐
│   SENTIENT  │      │   V-GATE    │      │   LLM API   │
│   Client    │─────▶│   Proxy     │─────▶│   Provider  │
└─────────────┘      └─────────────┘      └─────────────┘
                           │
                     API Key (Secure)
                     Stored on server
```

**API anahtarları ASLA istemci kodunda tutulmaz.**

### Guardrails Sistemi

- **Prompt Injection Detection**: Zararlı prompt tespiti
- **Data Leak Prevention**: Veri sızıntısı önleme
- **SQL Injection Protection**: SQL enjeksiyon koruması
- **PII Redaction**: Hassas veri gizleme

### TEE (Trusted Execution Environment)

- **AMD SEV-SNP**: AMD güvenli işlemci desteği
- **Intel TDX**: Intel güvenli işlemci desteği
- **Feature-gated**: Donanım gerektirir

### ZK-MCP (Zero-Knowledge Proofs)

- **Groth16**: zkSNARK proofs
- **PLONK**: Universal setup proofs
- **Bulletproofs**: Range proofs

---

## 📁 DİZİN YAPISI

```
SENTIENT_CORE/
├── crates/                 # 53 Rust crate
│   ├── oasis_*/           # Execution layer (7 crate)
│   ├── sentient_*/        # Core modules (46 crate)
│   └── ...
├── integrations/          # 72 GitHub repo
│   ├── agents/           # 17 agent frameworks
│   ├── framework/        # 21 LLM frameworks
│   ├── browser/          # 5 browser tools
│   ├── memory/           # 4 vector DBs
│   ├── tools/            # 5 tools
│   ├── security/         # 1 security
│   ├── sandbox/          # 3 sandboxes
│   ├── cli/              # 2 CLIs
│   └── skills/           # 6 skill repos
├── skills/               # 5,587+ native skills
├── data/                 # SQLite databases
│   ├── asena.db         # Main database
│   ├── asena_memory.db  # Memory database
│   ├── asena_vectors.db # Vector database
│   └── asena_skills.db  # Skills database
├── core/                 # Core components
│   ├── io/              # Lightpanda FFI
│   └── research/        # MindSearch
├── dashboard/            # TUI Dashboard
├── examples/             # 11 örnek proje
├── tools/                # CLI tools
├── AGENTS.md            # Proje dokümantasyonu
├── SISTEM_SORUN_RAPORU.md # Sorun raporu
└── Cargo.toml           # Workspace config
```

---

## 🚀 KULLANIM SENARYOLARI

### 1. Otonom Web Agent

```rust
use oasis_browser::BrowserAgent;

let agent = BrowserAgent::new(config).await?;
agent.navigate("https://example.com").await?;
agent.fill_form("#email", "user@example.com").await?;
agent.click("#submit").await?;
```

### 2. Multi-Agent Orchestration

```rust
use sentient_orchestrator::{Swarm, AgentRole};

let swarm = Swarm::new()
    .add_agent("researcher", AgentRole::Researcher)
    .add_agent("coder", AgentRole::Developer)
    .add_agent("reviewer", AgentRole::Reviewer);

swarm.execute("Bu API'yi implement et").await?;
```

### 3. Skill Execution

```rust
use sentient_skills::SkillManager;

let manager = SkillManager::new()?;
let skill = manager.load("deep-research")?;
let result = skill.execute(params).await?;
```

### 4. Memory Operations

```rust
use sentient_memory::{MemoryStore, MemoryType};

let store = MemoryStore::new()?;
store.remember("user_preference", "dark_mode", MemoryType::Semantic).await?;
let preference = store.recall("user_preference").await?;
```

---

## 📊 PERFORMANS METRİKLERİ

| Metrik | Değer |
|--------|-------|
| **unwrap()** | 0 |
| **unsafe blokları** | 10 (FFI için gerekli) |
| **TODO** | 0 |
| **Test Coverage** | 1,358 test |
| **Build Time** | ~30 saniye (workspace) |
| **Memory Usage** | ~50MB (boşta) |

---

## 📈 SON DURUM

| Kategori | Önceki | Şimdi | Değişim |
|----------|--------|-------|---------|
| **Toplam Sorun** | 91 | 0 | ✅ %100 |
| **unwrap()** | 14,379 | 0 | ✅ %100 |
| **unsafe** | 2,929 | 10 | ✅ %99 |
| **TODO** | 67 | 0 | ✅ %100 |
| **Build** | ❌ Error | ✅ Success | ✅ |

---

## 🔮 GELECEK PLANLARI

1. **Production Deployment**: Kubernetes ile dağıtık部署
2. **Voice Interface**: Tam sesli etkileşim
3. **Mobile SDK**: iOS/Android SDK'ları
4. **Plugin System**: Üçüncü parti plugin desteği
5. **Enterprise Features**: Daha fazla SSO provider

---

## 📜 LİSANS

SENTIENT OS - Apache License 2.0

Entegre projelerin lisansları `THIRD_PARTY_NOTICES.md` dosyasında listelenmiştir.

---

*SENTIENT OS - The Operating System That Thinks*
*Generated: 2026-04-10*
