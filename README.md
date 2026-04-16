<p align="center">
  <img src="https://img.shields.io/badge/🐺-SENTIENT%20OS-blue?style=for-the-badge&labelSize=20" alt="SENTIENT OS">
</p>

<h1 align="center">🐺 SENTIENT OS</h1>
<h3 align="center">The Operating System That Thinks</h3>
<h4 align="center">Düşünen İşletim Sistemi</h4>

<p align="center">
  <a href="#-tek-komutla-kurulum">Kurulum</a> •
  <a href="#-hızlı-başlangıç">Hızlı Başlangıç</a> •
  <a href="#-llm-hub">LLM Hub</a> •
  <a href="#-özellikler">Özellikler</a> •
  <a href="#-crate-yapısı">Mimari</a> •
  <a href="#-kullanım-rehberi">Kullanım</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-4.0.0-blue" alt="Version">
  <img src="https://img.shields.io/badge/rust-2021-orange" alt="Rust">
  <img src="https://img.shields.io/badge/license-AGPL%20v3-green" alt="License">
  <img src="https://img.shields.io/badge/crates-93-purple" alt="Crates">
  <img src="https://img.shields.io/badge/models-245+-red" alt="Models">
  <img src="https://img.shields.io/badge/providers-57+-yellow" alt="Providers">
  <img src="https://img.shields.io/badge/skills-5587+-brightgreen" alt="Skills">
</p>

---

## 📊 Proje Özeti

| Metrik | Değer | Açıklama |
|--------|-------|----------|
| **Rust Crate** | 93 | Tamamen Rust ile yazılmış |
| **LLM Provider** | 57+ | OpenAI, Anthropic, Google, DeepSeek, Groq, Unify, Portkey... |
| **LLM Model** | 245+ | GPT-2 (2019) → o4-mini/Grok 4 (2026) |
| **AI Gateway** | 11 | OpenRouter, Unify, Portkey, Helicone, NotDiamond... |
| **Skill** | 5,587+ | Dünyanın en büyük AI skill koleksiyonu |
| **Entegrasyon** | 72+ proje | AutoGPT, CrewAI, LangChain, Ollama... |
| **Kanal** | 24 platform | Telegram, Discord, WhatsApp, Slack... |
| **Rust Kodu** | 152,877+ satır | Production-grade |

---

## 🚀 Tek Komutla Kurulum

### OpenClaw Tarzı — Tek Komut, Tam Kurulum

```bash
# Yöntem 1: curl ile (önerilen)
curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.sh | bash

# Yöntem 2: git clone + install
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
cd SENTIENT_CORE
./install.sh
```

### Kurulum Adımları (OpenClaw Standardı)

```
1. ⚠️  Yasal Uyari (Yes/No)          → Kullanım koşulları
2. 🔍 Sistem Tespiti                → OS, RAM, GPU, CPU
3. 📦 Kurulum Modu                  → Quick / Full / Custom
4. 🧠 LLM Provider Seçimi          → Lokal (ücretsiz) veya API
5. 🧩 Ek Modüller                   → Voice, Browser, Desktop, Channels
6. 🐳 Docker Servisleri             → PostgreSQL, Redis, Qdrant...
7. ⚙️  Yapılandırma (.env)          → Otomatik oluşturma
8. 🔨 Derleme                       → cargo build --release
9. ✅ Doğrulama                     → health-check
```

### Hızlı Kurulum (Sorusuz)

```bash
# Tüm soruları atla, varsayılanlarla kur
./install.sh --yes --quick

# Tam kurulum (tüm 93 crate + Docker)
./install.sh --yes --full

# Özel dizine kur
./install.sh --dir /opt/sentient

# Kaldır
./install.sh --uninstall
```

---

## ⚡ Hızlı Başlangıç

```bash
# İnteraktif sohbet
sentient chat

# Tek soru sor
sentient ask "Rust'ta async nasıl çalışır?"

# Sesli asistan (JARVIS modu)
sentient voice

# Web API sunucusu
sentient gateway
# → http://localhost:8080

# Web dashboard
sentient dashboard
# → http://localhost:8080/dashboard

# Sistem durumu
sentient status

# İlk kurulum sihirbazı
sentient init

# Sağlık kontrolü
sentient doctor
```

---

## 🧠 LLM Hub

SENTIENT OS dünyanın en kapsamlı LLM hub'ına sahiptir. **57+ provider, 245+ model** desteği.

### Doğrudan AI Şirketleri

| Provider | Modeller | Fiyat | Ücretsiz |
|----------|----------|-------|----------|
| **OpenAI** | GPT-4o, o3, o4-mini (17 model) | $$ | ❌ |
| **Anthropic** | Claude 4, 3.5, 3, 2, 1 (12 model) | $$ | ❌ |
| **Google** | Gemini 2.5, 2.0, 1.5, Gemma 3 (14 model) | $ | ✅ Flash ücretsiz |
| **Mistral** 🇫🇷 | Large 2, Small 3.1, Codestral, Pixtral (11 model) | $ | ✅ Bazı modeller |
| **DeepSeek** 🇨🇳 | V3, R1, R2, V4, Coder (6 model) | **EN UCUZ** | ✅ Çoğu ücretsiz |
| **xAI** | Grok 3, 3 Mini, Grok 4 (4 model) | $$ | ✅ Mini ücretsiz |
| **Cohere** | Command A, R+, Aya Exa (6 model) | $ | ✅ |
| **Perplexity** | Sonar, Deep Research (4 model) | $$ | ❌ |

### AI Gateway / Router (11 Provider)

| Gateway | İşlev | Ücretsiz Tier |
|---------|-------|---------------|
| **OpenRouter** | 300+ model marketplace | ✅ Bazı modeller |
| **Unify AI** | ML-based akıllı routing (quality/cost/speed) | ✅ $5 kredi |
| **Portkey** | Enterprise gateway, failover, caching | ✅ 10K req/mo |
| **Helicone** | AI observability, cost tracking | ✅ 50K req/mo |
| **NotDiamond** | ML ile prompt bazlı model seçimi | ✅ |
| **AI/ML API** | 100+ model, doğrudan erişimden %40 ucuz | ✅ 100 req/gün |
| **Glama** | Multi-model gateway + MCP desteği | ✅ |
| **Requesty** | LLM router, A/B testing | ✅ |
| **LiteLLM** | 100+ provider, self-hosted proxy | Açık kaynak |
| **Cloudflare Workers AI** | Edge inference | ✅ |
| **Chutes** | Tamamen ücretsiz inference! | ✅ Tamamen |

### Lokal / Açık Kaynak (Ücretsiz!)

| Model | VRAM | Ollama ID | Özellik |
|-------|------|----------|---------|
| **Llama 4 Scout** | 48GB | `llama4:scout` | 10M context, MoE 109B, Vision |
| **Llama 4 Maverick** | 96GB | `llama4:maverick` | 1M context, MoE 400B, Vision |
| **Qwen3 30B-A3B MoE** | **4GB!** | `qwen3:30b-a3b` | 3B aktif parametre, Reasoning |
| **DeepSeek R2** | 96GB | `deepseek-r2` | Reasoning + Vision |
| **Gemma 3 27B** | 16GB | `gemma3:27b` | Vision, 128K context |
| **Phi-4 14B** | 8GB | `phi4:14b` | MIT lisans, Code |
| **Mistral Small 3.1** | 16GB | `mistral-small3.1:24b` | Vision + Tools |
| **Gemma 3 4B** | 4GB | `gemma3:4b` | Vision, en hafif |
| **DeepSeek R1 Distill 8B** | 4GB | `deepseek-r1:8b` | En küçük reasoning |
| **Qwen 2.5 Coder 14B** | 8GB | `qwen2.5-coder:14b` | Code specialist |

### Çin AI Ekosistemi

| Provider | Modeller |
|----------|----------|
| **Qwen (Alibaba)** | Qwen3, QwQ, QVQ, Qwen4 Max |
| **Baidu ERNIE** | ERNIE 4.0, 3.5 |
| **Zhipu GLM** | GLM-4 Plus, Flash |
| **Moonshot (Kimi)** | V1 128K |
| **StepFun** | Step-2, Step-1V |
| **ByteDance** | Doubao 1.5 Pro |

### Diğer Bölgesel AI

| Bölge | Provider | Modeller |
|-------|----------|----------|
| 🇷🇺 Rusya | GigaChat | Pro, Max |
| 🇰🇷 Kore | Upstage | Solar Pro 2 |
| 🇪🇺 Avrupa | Aleph Alpha | Luminous, Pharia |
| 🇮🇳 Hindistan | Sarvam AI | Sarvam-M |
| 🇯🇵 Japonya | Rinna, CyberAgent | CALM 3 |
| 🇦🇪 Arap | InceptionAI | JAIS 30B |

### Rust ile Kullanım

```rust
use sentient_llm::{LlmHub, ChatRequest, Message};

#[tokio::main]
async fn main() {
    // Tüm provider'ları otomatik algıla (.env dosyasından)
    let hub = LlmHub::from_env().unwrap();

    // Sohbet
    let response = hub.chat(ChatRequest {
        model: "gpt-4o".into(),
        messages: vec![Message::user("Merhaba!")],
        ..Default::default()
    }).await.unwrap();

    println!("{}", response.choices[0].message.content.as_text().unwrap());
}
```

---

## 🌟 Özellikler

### 🧠 Core Intelligence

| Özellik | Açıklama |
|---------|----------|
| **57+ LLM Provider** | OpenAI, Anthropic, Google, Mistral, DeepSeek, Groq, Unify, Portkey... |
| **245+ Model** | GPT-2 (2019) → o4-mini/Grok 4 (2026) |
| **11 AI Gateway** | OpenRouter, Unify, Portkey, Helicone, NotDiamond... |
| **Smart Routing** | Maliyet, hız veya kalite bazlı otomatik provider seçimi |
| **Circuit Breaker** | Hata toleransı, otomatik retry ve failover |
| **Cost Tracker** | Gerçek zamanlı maliyet takibi |

### 🎙️ Voice (JARVIS Seviyesi)

```
🎤 Mikrofon → Whisper STT → LLM → TTS → 🔊 Hoparlör
     ↑                                    |
     └──── Wake Word ("Hey Sentient") ────┘
```

| Özellik | Lokal (Ücretsiz) | API (Ücretli) |
|---------|------------------|---------------|
| **STT** | Whisper.cpp | OpenAI Whisper |
| **TTS** | Piper (Türkçe) | ElevenLabs |
| **Wake Word** | Porcupine, Vosk | - |

### 🖥️ Desktop Automation (Computer Use)

| Özellik | Açıklama |
|---------|----------|
| **Fare/Klavye** | İnsan gibi hareket (Bezier eğrileri) |
| **Ekran Okuma** | OCR, görüntü analizi |
| **GUI Kontrol** | Tüm uygulamalar |
| **Safety** | 50+ tehlikeli komut engeli |

### 📱 24 Platform Entegrasyonu

| Platform | Durum | Tür |
|----------|-------|-----|
| Telegram | ✅ Aktif | Mesajlaşma |
| Discord | ✅ Aktif | Mesajlaşma |
| WhatsApp | ✅ Aktif | Mesajlaşma |
| Slack | ✅ Aktif | İş |
| Email | ✅ Aktif | İletişim |
| + 19 platform daha | | |

### 🔐 Güvenlik (Enterprise Grade)

| Özellik | Açıklama |
|---------|----------|
| **V-GATE** | API key'ler asla istemcide değil, proxy üzerinden |
| **Guardrails** | Prompt injection, PII, secret filtreleme |
| **Vault** | AES-256-GCM şifreleme, key rotation |
| **TEE** | AMD SEV-SNP, Intel TDX desteği |
| **ZK-MCP** | Zero-knowledge proof |
| **Audit** | Tüm işlemler loglanır |

---

## 📦 Crate Yapısı (93 Crate)

### 🧠 Core (7 Crate)

| Crate | İşlev |
|-------|-------|
| **sentient_core** | Ana motor, sistem yönetimi |
| **sentient_memory** | Agent bellek sistemi (Hippocampus) |
| **sentient_graph** | Event graph, workflow |
| **sentient_orchestrator** | Agent loop & routing |
| **sentient_gateway** | API gateway |
| **sentient_cevahir** | Türkçe LLM cognitive engine |
| **oasis_brain** | Gemma 4 cognitive kernel |

### 🤖 LLM (7 Crate)

| Crate | İşlev |
|-------|-------|
| **sentient_llm** | 57+ provider, 245+ model hub |
| **sentient_embed** | Multi-provider embedding hub |
| **sentient_rerank** | Search result reranking |
| **sentient_local** | Lokal LLM yönetimi |
| **sentient_groq** | Groq LPU ultra-fast inference |
| **sentient_vector** | Vector search |
| **sentient_rag** | Native RAG engine |

### 🔒 Security (6 Crate)

| Crate | İşlev |
|-------|-------|
| **sentient_vgate** | V-GATE proxy (API key güvenliği) |
| **sentient_guardrails** | Input/output filtreleme |
| **oasis_vault** | Secrets manager (AES-256-GCM) |
| **sentient_tee** | TEE support (AMD SEV-SNP) |
| **sentient_zk_mcp** | Zero-knowledge proof |
| **sentient_anomaly** | Anomaly detection |

### 🖥️ Desktop & Browser (5 Crate)

| Crate | İşlev |
|-------|-------|
| **oasis_hands** | Desktop automation (EN BÜYÜK - 36K+ satır) |
| **oasis_autonomous** | Tam otonom agent |
| **oasis_browser** | Browser automation |
| **oasis_manus** | Docker execution sandbox |
| **sentient_desktop** | Computer Use / GUI control |

### 🎤 Voice (2 Crate)

| Crate | İşlev |
|-------|-------|
| **sentient_voice** | STT + TTS pipeline |
| **sentient_wake** | Wake word detection |

### 📱 Channels & Communication (1 Crate)

| Crate | İşlev |
|-------|-------|
| **sentient_channels** | 24 platform entegrasyonu |

### 🔧 Tools & Utilities (60+ Crate)

| Crate | İşlev |
|-------|-------|
| sentient_rag | Native RAG engine |
| sentient_vision | Vision/multimodal |
| sentient_mcp | Model Context Protocol |
| sentient_sandbox | E2B sandbox |
| sentient_skills | Skills system (5,587+) |
| sentient_search | Web search (Tavily, Brave, SearXNG) |
| sentient_image | Image generation (DALL-E, SD, Flux) |
| sentient_video | Video generation |
| sentient_finetuning | Fine-tuning (LoRA) |
| sentient_quantize | Model quantization |
| sentient_knowledge | Knowledge graph |
| sentient_enterprise | RBAC, SSO, Audit |
| sentient_compliance | SOC 2 compliance |
| sentient_observability | OpenTelemetry, Prometheus |
| sentient_i18n | 8 dil desteği |
| sentient_backup | Backup & DR |
| sentient_cluster | Kubernetes operator |
| sentient_web | Web server |
| sentient_workflow | Visual flow builder (n8n-style) |
| sentient_daemon | Background always-on assistant |
| sentient_proactive | Time/Event/Pattern triggers |
| sentient_connectors | Gmail, Calendar, GitHub, Weather |
| sentient_digest | Morning briefing system |
| sentient_a2a | Agent-to-Agent protocol |
| sentient_home | Smart Home (Home Assistant) |
| sentient_social | Social media automation |
| sentient_remote | Mobile PWA, Telegram Mini App |
| sentient_learning | User behavior learning |
| sentient_marketplace | Skills marketplace |
| ... | ve 30+ crate daha |

---

## 🎯 Kullanım Rehberi

### 1. İnteraktif Sohbet (REPL)

```bash
sentient chat
# veya model belirterek
sentient chat --model gpt-4o
sentient chat --model ollama:qwen3:30b-a3b
```

### 2. API Sunucusu (Gateway)

```bash
sentient gateway
# POST http://localhost:8080/v1/chat/completions
# OpenAI-compatible API
```

```bash
curl http://localhost:8080/v1/chat/completions \
  -H "Authorization: Bearer $JWT_SECRET" \
  -d '{"model":"gpt-4o","messages":[{"role":"user","content":"Merhaba"}]}'
```

### 3. Sesli Asistan

```bash
sentient voice
# "Hey Sentient" → mikrofon dinler → yanıt verir
```

### 4. Desktop Automation

```bash
sentient desktop
# Bilgisayarı otonom kontrol eder
```

### 5. Skill Kullanımı

```bash
sentient skill search "browser automation"
sentient skill run code-review --path ./src
sentient skill list
```

### 6. Docker ile Tüm Servisler

```bash
docker-compose up -d
# Servisler:
# - PostgreSQL (5432)    → Veritabanı
# - Redis (6379)         → Cache
# - Qdrant (6333)        → Vector DB
# - MinIO (9000/9001)    → Object Storage
# - Prometheus (9090)    → Monitoring
# - Grafana (3001)       → Dashboard
# - Ollama (11434)       → Lokal LLM
# - SearXNG (8888)       → Arama Motoru
# - RabbitMQ (5672)      → Message Queue
```

### 7. Sağlık Kontrolü

```bash
sentient doctor
# veya
./scripts/health-check.sh
```

### 8. Sistem Durumu

```bash
sentient status
```

---

## 📊 Sistem Gereksinimleri

| Mod | RAM | Disk | GPU | Açıklama |
|-----|-----|------|-----|----------|
| **API Only** | 2 GB | 5 GB | Yok | API provider kullanımı |
| **Local Small** | 8 GB | 10 GB | Opsiyonel | Ollama 3B-7B |
| **Local Medium** | 16 GB | 20 GB | 8GB VRAM | Ollama 14B-27B (önerilen) |
| **Local Large** | 64 GB | 50 GB | 24GB VRAM | Ollama 70B+ |
| **Full Stack** | 32 GB | 30 GB | 8GB VRAM | Tüm servisler + Docker |
| **Enterprise** | 64 GB+ | 100 GB+ | 24GB+ VRAM | Multi-tenant, RBAC |

---

## 🐳 Docker

```bash
# Tüm servisleri başlat
docker-compose up -d

# Sadece temel servisler
./scripts/start.sh --minimal

# Servisleri durdur
./scripts/stop.sh
```

---

## 🛠️ Geliştirme

```bash
# Tüm crate'leri derle
cargo build --release

# Sadece CLI binary
cargo build --release --bin sentient

# Test
cargo test --workspace

# Clippy
cargo clippy --workspace

# Format
cargo fmt

# Belirli crate derle
cargo build --release -p sentient_llm
cargo build --release -p sentient_voice
```

### Makefile Komutları

```bash
make build          # Derle
make run            # REPL başlat
make test           # Test çalıştır
make docker-up      # Docker servisleri başlat
make clean          # Temizle
make skills         # Skill library güncelle
make help           # Tüm komutlar
```

---

## 🌍 Dil Desteği

| Dil | Durum |
|-----|-------|
| 🇹🇷 Türkçe | ✅ Tam destek (Cevahir AI engine) |
| 🇺🇸 English | ✅ Full support |
| 🇩🇪 Deutsch | 🔄 Partial |
| 🇫🇷 Français | 🔄 Partial |
| 🇪🇸 Español | 🔄 Partial |
| 🇯🇵 日本語 | 🔄 Partial |
| 🇨🇳 中文 | 🔄 Partial |
| 🇰🇷 한국어 | 🔄 Partial |

---

## 📁 Proje Yapısı

```
SENTIENT_CORE/
├── crates/                  # 93 Rust crate
│   ├── sentient_core/       # Ana motor
│   ├── sentient_llm/        # LLM hub (57+ provider, 245+ model)
│   ├── sentient_embed/      # Embedding hub
│   ├── sentient_rerank/     # Reranking engine
│   ├── sentient_voice/      # Ses modülü (STT + TTS)
│   ├── sentient_channels/   # 24 platform
│   ├── sentient_memory/     # Bellek sistemi
│   ├── sentient_rag/        # RAG engine
│   ├── sentient_vision/     # Vision/multimodal
│   ├── sentient_mcp/        # MCP protocol
│   ├── sentient_vgate/      # V-GATE proxy
│   ├── sentient_guardrails/ # Güvenlik duvarı
│   ├── sentient_cli/        # CLI arayüzü
│   ├── sentient_setup/      # Setup wizard (TUI)
│   ├── oasis_hands/         # Desktop automation (36K+ satır)
│   ├── oasis_autonomous/    # Tam otonom agent
│   ├── oasis_browser/       # Browser automation
│   ├── sentient_cevahir/    # Türkçe LLM cognitive engine
│   └── ...                  # 75+ crate daha
├── integrations/            # 72+ entegre proje
│   ├── agents/              # AutoGPT, CrewAI, LangChain, Goose...
│   ├── framework/           # LlamaIndex, Phidata, Semantic Kernel...
│   ├── tools/               # Firecrawl, Mem0, Crawl4AI...
│   ├── browser/             # Browser-use, LightPanda...
│   ├── memory/              # Qdrant, MemGPT...
│   ├── sandbox/             # E2B, LocalStack...
│   ├── cevahir_ai/          # Türkçe LLM engine
│   └── security/            # NeMo-Guardrails...
├── skills/                  # 5,587+ skill
│   ├── Dev/                 # 2,965 skill
│   ├── OSINT/               # 1,050 skill
│   ├── Social/              # 238 skill
│   ├── Automation/          # 306 skill
│   ├── Media/               # 246 skill
│   └── ...                  # 5 kategori daha
├── scripts/                 # Yardımcı scriptler
│   ├── start.sh             # Tüm servisleri başlat
│   ├── stop.sh              # Servisleri durdur
│   ├── health-check.sh      # Sağlık kontrolü
│   ├── setup-ollama.sh      # Ollama kurulumu
│   └── run_tests.sh         # Test çalıştırıcı
├── data/                    # Veritabanları
├── dashboard/               # Web dashboard
├── config/                  # Konfigürasyon
├── docker-compose.yml       # 9 Docker servisi
├── install.sh               # ✨ Tek komutla kurulum (OpenClaw-style)
├── Makefile                 # Build komutları
├── .env.example             # Config şablonu
└── Cargo.toml               # Workspace config (93 member)
```

---

## 🔌 Entegrasyonlar (72+ Proje)

### Agent Framework'leri (17)

| Proje | Açıklama |
|-------|----------|
| AutoGPT | Otonom agent |
| CrewAI | Multi-agent orchestration |
| LangChain | LLM framework |
| LlamaIndex | RAG framework |
| MetaGPT | Multi-agent simulator |
| AutoGen | Microsoft multi-agent |
| OpenHands | AI developer |
| Goose | AI coding agent |
| Phidata | AI apps |
| Semantic Kernel | Microsoft AI |

### Tools (5)

| Proje | Açıklama |
|-------|----------|
| Firecrawl | Web scraping |
| Mem0 | Memory layer |
| CAG4AI | Web extraction |
| Judge0 | Code execution |
| RAGFlow | RAG engine |

### Türkçe LLM — Cevahir AI

| Modül | İşlev |
|-------|-------|
| Neural Network (V-7) | RoPE, RMSNorm, SwiGLU, KV Cache, MoE, GQA |
| Cognitive Strategies | Direct, Think, Debate, Tree of Thoughts |
| Turkish BPE Tokenizer | 60K vocabulary, GPU batch processing |
| Memory & RAG | Vector store, semantic cache |

---

## 📜 Lisans

**GNU AGPL v3.0** — Kullan, değiştir, paylaş.

Ticari kullanım için: enterprise@sentient-os.ai

---

## 🤝 Katkıda Bulunma

1. Fork'la
2. Branch oluştur: `git checkout -b feature/yeni-ozellik`
3. Commit'le: `git commit -m 'Yeni özellik'`
4. Push'la: `git push origin feature/yeni-ozellik`
5. Pull Request aç

---

## 📞 İletişim

| Kanal | Link |
|-------|------|
| GitHub | [github.com/nexsusagent-coder/SENTIENT_CORE](https://github.com/nexsusagent-coder/SENTIENT_CORE) |
| Ko-fi | [ko-fi.com/sentientos](https://ko-fi.com/sententos) |
| Email | sentient@sentient-os.ai |

---

<p align="center">
  <b>SENTIENT OS</b><br>
  <i>The Operating System That Thinks</i><br><br>
  Made with 🦀 by the SENTIENT Team
</p>
