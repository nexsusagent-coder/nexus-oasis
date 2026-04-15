<p align="center">
  <img src="https://img.shields.io/badge/🐺-SENTIENT%20OS-blue?style=for-the-badge&labelSize=20" alt="SENTIENT OS">
</p>

<h1 align="center">🐺 SENTIENT OS</h1>
<h3 align="center">The Operating System That Thinks</h3>
<h4 align="center">Düşünen İşletim Sistemi</h4>

<p align="center">
  <a href="#-kurulum">Kurulum</a> •
  <a href="#-özellikler">Özellikler</a> •
  <a href="#-kullanım">Kullanım</a> •
  <a href="#-entegrasyonlar">Entegrasyonlar</a> •
  <a href="#-dokümantasyon">Dokümantasyon</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-4.0.0-blue" alt="Version">
  <img src="https://img.shields.io/badge/rust-2021-orange" alt="Rust">
  <img src="https://img.shields.io/badge/license-AGPL%20v3-green" alt="License">
  <img src="https://img.shields.io/badge/crates-93-purple" alt="Crates">
  <img src="https://img.shields.io/badge/skills-5587+-yellow" alt="Skills">
</p>

---

## 📊 Proje Özeti

| Metrik | Değer | Açıklama |
|--------|-------|----------|
| **Rust Crate** | 93 | Tamamen Rust ile yazılmış |
| **Rust Kodu** | 152,877+ satır | Production-grade |
| **LLM Provider** | 42 | OpenAI, Anthropic, Google, vs. |
| **LLM Model** | 355 native | 200K+ aggregator ile |
| **Skill** | 5,587+ | Dünyanın en büyük koleksiyonu |
| **Entegrasyon** | 72+ proje | AutoGPT, LangChain, CrewAI, vs. |
| **Kanal** | 24 platform | Telegram, Discord, WhatsApp, vs. |
| **Örnek** | 19 proje | Hello-world'ten production'a |

---

## 🚀 Kurulum

### Tek Komutla

```bash
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
cd SENTIENT_CORE
./install.sh
```

Kurulum sırasında:
1. **LLM Seçimi** → API Key / Lokal (Ollama) / Atla
2. **Ek Özellikler** → Voice, Dashboard (opsiyonel)
3. **Derleme** → Otomatik

### Hızlı Başlangıç (Soru Sormadan)

```bash
./quick-start.sh
```

### Manuel Kurulum

```bash
# Rust (yoksa)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# Derle
cargo build --release --bin sentient

# Config
cp .env.template .env
# .env dosyasını düzenleyin

# Çalıştır
./target/release/sentient chat
```

---

## 🧠 LLM Seçenekleri

### API Key ile (Önerilen - Hızlı)

| Sağlayıcı | Modeller | Fiyat | Link |
|-----------|----------|-------|------|
| **OpenRouter** | 200+ model | $5 bonus | [openrouter.ai](https://openrouter.ai/keys) |
| OpenAI | GPT-4o, o1, o3 | $$ | [platform.openai.com](https://platform.openai.com/api-keys) |
| Anthropic | Claude 4, 3.5 | $$ | [console.anthropic.com](https://console.anthropic.com/) |
| Google | Gemini 2.0 | $ | [aistudio.google.com](https://aistudio.google.com/apikey) |
| DeepSeek | V3, R1 | **EN UCUZ** | [platform.deepseek.com](https://platform.deepseek.com/) |
| Groq | Llama, Mixtral | **EN HIZLI** | [console.groq.com](https://console.groq.com/) |

```bash
# .env dosyasına
OPENROUTER_API_KEY=sk-or-v1-...
# veya
OPENAI_API_KEY=sk-...
# veya
ANTHROPIC_API_KEY=sk-ant-...
```

### Lokal (Ücretsiz - Ollama)

```bash
# Ollama kur
curl -fsSL https://ollama.com/install.sh | sh

# Model indir
ollama pull gemma3:27b      # Önerilen (16GB RAM)
ollama pull llama3.2:3b     # Hafif (4GB RAM)
ollama pull deepseek-r1:7b  # Reasoning (8GB RAM)

# .env
LLM_PROVIDER=ollama
OLLAMA_MODEL=gemma3:27b
```

---

## ⚡ Hızlı Başlangıç

```bash
# Sohbet
./target/release/sentient chat

# Tek soru
./target/release/sentient ask "Rust'ta async nasıl çalışır?"

# Sesli asistan (JARVIS modu)
./target/release/sentient voice

# Web API
./target/release/sentient gateway
# http://localhost:8080

# Dashboard
./target/release/sentient dashboard
# http://localhost:8080/dashboard

# Sistem durumu
./target/release/sentient status
```

---

## 🌟 Özellikler

### 🧠 Core Intelligence

| Özellik | Açıklama |
|---------|----------|
| **42+ LLM Provider** | OpenAI, Anthropic, Google, Mistral, DeepSeek, Groq... |
| **355 Native Model** | GPT-4o, Claude 4, Gemini 2.0, Llama 3.2, DeepSeek R1... |
| **200K+ Model Erişimi** | OpenRouter, LiteLLM, HuggingFace aggregator'ları ile |
| **Smart Routing** | Otomatik provider seçimi, maliyet optimizasyonu |
| **Streaming** | Gerçek zamanlı yanıt akışı |
| **Circuit Breaker** | Hata toleransı, otomatik retry |

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
| **Duygu** | - | Hume AI |

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
| Microsoft Teams | 🔄 Beta | İş |
| Signal | 🔄 Beta | Güvenli |
| Matrix | 🔄 Beta | Açık kaynak |
| + 16 platform daha | | |

### 🔐 Güvenlik (Enterprise Grade)

| Özellik | Açıklama |
|---------|----------|
| **V-GATE** | API key'ler asla istemcide değil, proxy üzerinden |
| **Guardrails** | Prompt injection, PII, secret filtreleme |
| **Vault** | AES-256-GCM şifreleme, key rotation |
| **TEE** | AMD SEV-SNP, Intel TDX desteği |
| **ZK-MCP** | Zero-knowledge proof |
| **RBAC** | Rol bazlı erişim kontrolü |
| **Audit** | Tüm işlemler loglanır |

---

## 📦 Crate Yapısı (93 Crate)

### 🧠 Core (7 Crate)

| Crate | Satır | İşlev |
|-------|-------|-------|
| **sentient_core** | 2,326 | Ana motor |
| **sentient_memory** | 6,182 | Agent bellek sistemi |
| **sentient_graph** | 585 | Workflow graph |
| **sentient_orchestrator** | 11,235 | Agent loop & routing |
| **sentient_gateway** | 10,058 | API gateway |
| **sentient_cevahir** | 1,630 | Türkçe LLM cognitive engine |
| **oasis_brain** | 1,203 | Gemma 4 cognitive kernel |

### 🤖 LLM (5 Crate)

| Crate | Satır | İşlev |
|-------|-------|-------|
| **sentient_llm** | 14,445 | 42 provider, 355 model |
| **sentient_embed** | - | Embedding hub |
| **sentient_rerank** | - | Reranking engine |
| **sentient_local** | 1,157 | Lokal LLM |
| **sentient_groq** | 1,233 | Groq LPU (en hızlı) |

### 🎤 Voice (2 Crate)

| Crate | Satır | İşlev |
|-------|-------|-------|
| **sentient_voice** | 2,634 | STT + TTS |
| **sentient_wake** | 914 | Wake word detection |

### 🔒 Security (6 Crate)

| Crate | Satır | İşlev |
|-------|-------|-------|
| **sentient_vgate** | 3,525 | V-GATE proxy |
| **sentient_guardrails** | 307 | Input/output filtreleme |
| **oasis_vault** | 2,417 | Secrets manager |
| **sentient_tee** | 2,683 | TEE support |
| **sentient_zk_mcp** | 2,062 | Zero-knowledge |
| **sentient_anomaly** | 1,160 | Anomaly detection |

### 🖥️ Desktop (5 Crate)

| Crate | Satır | İşlev |
|-------|-------|-------|
| **oasis_hands** | 36,741 | Desktop automation (EN BÜYÜK) |
| **oasis_autonomous** | 6,773 | Tam otonom agent |
| **oasis_browser** | 5,311 | Browser automation |
| **oasis_manus** | 2,921 | Docker execution |
| **sentient_desktop** | 1,021 | Computer Use |

### 📱 Channels (1 Crate)

| Crate | Satır | İşlev |
|-------|-------|-------|
| **sentient_channels** | 3,736 | 24 platform |

### 🔧 Tools & Utils (20+ Crate)

| Crate | İşlev |
|-------|-------|
| sentient_rag | Native RAG engine |
| sentient_vision | Vision/multimodal |
| sentient_mcp | Model Context Protocol |
| sentient_sandbox | E2B sandbox |
| sentient_skills | Skills system |
| sentient_search | Web search (Tavily, Brave) |
| sentient_image | Image generation |
| sentient_video | Video generation |
| sentient_finetuning | Fine-tuning (LoRA) |
| sentient_benchmarks | Performance benchmarks |
| sentient_enterprise | RBAC, SSO, Audit |
| sentient_compliance | SOC 2 compliance |
| sentient_i18n | 8 dil desteği |
| sentient_backup | Backup & DR |
| sentient_observability | OpenTelemetry |
| ... | |

---

## 🎯 Skill Sistemi (5,587+ Skill)

### Kategoriler

| Kategori | Skill | Açıklama |
|----------|-------|----------|
| **Dev** | 2,965+ | Coding, IDE, DevOps, CLI |
| **OSINT** | 1,050+ | Search, Browser, Data |
| **Social** | 238+ | Communication, Marketing |
| **Automation** | 306+ | Productivity, Calendar, Smart Home |
| **Media** | 246+ | Image/Video, Streaming, Speech |
| **Productivity** | 214+ | Notes, PDF, Apple Apps |
| **Security** | 52+ | Security, Passwords |
| **Mobile** | 233+ | Transportation, Health |
| **Gaming** | 108+ | Gaming, Personal Dev |

### Skill Kaynakları

| Kaynak | Skill Sayısı |
|--------|--------------|
| OpenClaw Skills | 5,143 |
| Everything Claude Code | 181 |
| DeerFlow Skills | 100+ |
| Gstack | 37 |

### Skill Kullanımı

```bash
# Skill ara
./target/release/sentient skill search "browser automation"

# Skill çalıştır
./target/release/sentient skill run code-review --path ./src

# Skill listele
./target/release/sentient skill list
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
| Swarm | OpenAI agents |
| Phidata | AI apps |
| Semantic Kernel | Microsoft AI |

### Tools (5)

| Proje | Açıklama |
|-------|----------|
| Firecrawl | Web scraping |
| Mem0 | Memory layer |
| RAGFlow | RAG engine |
| Crawl4AI | Web extraction |
| Judge0 | Code execution |

### Browser (5)

| Proje | Açıklama |
|-------|----------|
| Browser-use | AI browser |
| LightPanda | Minimal browser |
| Agent-Browser | Headless browser |
| Open-Computer-Use | GUI control |

### Sandbox (3)

| Proje | Açıklama |
|-------|----------|
| E2B | Secure sandbox |
| Daytona | Dev environment |
| LocalStack | AWS local |

### Skills Libraries (6)

| Proje | Skill |
|-------|-------|
| Claw3D | 5,143 skills |
| Everything Claude Code | 181 commands/skills |
| DeerFlow Skills | 100+ skills |
| Awesome n8n Templates | 500+ templates |
| Gstack | 37 skills |

### Türkçe LLM (Cevahir AI)

| Modül | İşlev |
|-------|-------|
| model | Model yönetimi |
| tokenizer | Tokenizasyon |
| training | Eğitim sistemi |
| chatting | Sohbet |
| cognitive | Bilişsel motor |
| education | Eğitim modülü |
| + 13 modül daha | |

---

## 📁 Proje Yapısı

```
SENTIENT_CORE/
├── crates/                  # 93 Rust crate
│   ├── sentient_core/       # Ana motor
│   ├── sentient_llm/        # LLM hub (42 provider)
│   ├── sentient_voice/      # Ses modülü
│   ├── sentient_channels/   # 24 platform
│   ├── sentient_memory/     # Bellek sistemi
│   ├── sentient_rag/        # RAG engine
│   ├── sentient_vision/     # Vision/multimodal
│   ├── sentient_mcp/        # MCP protocol
│   ├── oasis_hands/         # Desktop automation
│   ├── oasis_autonomous/    # Tam otonom
│   └── ...                  # 83 crate daha
├── integrations/            # 72+ entegre proje
│   ├── agents/              # AutoGPT, CrewAI, LangChain...
│   ├── framework/           # LlamaIndex, Phidata...
│   ├── skills/              # Claw3D, DeerFlow, ECC...
│   ├── tools/               # Firecrawl, Mem0...
│   ├── cevahir_ai/          # Türkçe LLM engine
│   └── ...                  # 8 kategori daha
├── skills/                  # 5,587+ native skill
│   ├── Dev/                 # 2,965 skill
│   ├── OSINT/               # 1,050 skill
│   ├── Social/              # 238 skill
│   └── ...                  # 7 kategori daha
├── examples/                # 19 örnek proje
│   ├── hello-world/         # Başlangıç
│   ├── chatbot/             # Sohbet botu
│   ├── multi-agent/         # Multi-agent
│   ├── voice-agent/         # Sesli asistan
│   └── ...                  # 15 örnek daha
├── data/                    # Veritabanları
│   ├── sentient_memory.db   # Bellek DB
│   ├── sentient_skills.db   # Skill DB
│   └── skills/              # 5,587 skill YAML
├── dashboard/               # Web dashboard
├── deploy/                  # Production deployment
├── docs/                    # Dokümantasyon
├── scripts/                 # Yardımcı scriptler
├── config/                  # Konfigürasyon
├── .env.template            # Config şablonu
├── install.sh               # Tek komut kurulum
├── quick-start.sh           # Hızlı başlangıç
├── Makefile                 # Build komutları
├── docker-compose.yml       # Docker servisleri
└── Cargo.toml               # Workspace config
```

---

## 📚 Dokümantasyon

| Dosya | Açıklama |
|-------|----------|
| [README.md](README.md) | Bu dosya |
| [USAGE_GUIDE.md](USAGE_GUIDE.md) | Detaylı kullanım kılavuzu |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Sistem mimarisi |
| [INSTALL.md](INSTALL.md) | Kurulum rehberi |
| [SISTEM_DOKUMANTASYONU.md](SISTEM_DOKUMANTASYONU.md) | Tam sistem dokümantasyonu |
| [MODEL_PROVIDERS.md](MODEL_PROVIDERS.md) | LLM provider detayları |
| [SECURITY.md](SECURITY.md) | Güvenlik dokümantasyonu |
| [DEPLOYMENT.md](DEPLOYMENT.md) | Production deployment |
| [ENTERPRISE.md](ENTERPRISE.md) | Enterprise özellikler |

### docs/ Klasörü

| Dosya | Açıklama |
|-------|----------|
| [API.md](docs/API.md) | REST API dokümantasyonu |
| [USER_MANUAL.md](docs/USER_MANUAL.md) | Kullanıcı kılavuzu |
| [VOICE.md](docs/VOICE.md) | Ses sistemi |
| [CHANNELS.md](docs/CHANNELS.md) | Kanal entegrasyonları |
| [TESTING.md](docs/TESTING.md) | Test rehberi |

---

## 🛠️ Geliştirme

```bash
# Repository'yi klonla
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
cd SENTIENT_CORE

# Derle
cargo build --release

# Test
cargo test --workspace

# Clippy
cargo clippy --workspace

# Format
cargo fmt

# Belirli crate derle
cargo build --release -p sentient_voice
```

### Makefile Komutları

```bash
make build          # Derle
make run            # Shell çalıştır
make test           # Test
make docker-up      # Docker servisleri başlat
make skills         # Skill library güncelle
make clean          # Temizle
```

---

## 🐳 Docker

```bash
# Tüm servisler
docker-compose up -d

# Servisler:
# - PostgreSQL (5432)
# - Redis (6379)
# - Qdrant (6333)
# - MinIO (9000/9001)
# - Prometheus (9090)
# - Grafana (3001)
# - SearXNG (8888)
```

---

## 📊 Sistem Gereksinimleri

| Mod | RAM | Disk | GPU | Açıklama |
|-----|-----|------|-----|----------|
| **API Only** | 2 GB | 5 GB | Yok | API provider kullanımı |
| **Local Small** | 8 GB | 10 GB | Opsiyonel | Ollama 3B-7B |
| **Local Medium** | 16 GB | 20 GB | 8GB VRAM | Ollama 27B (önerilen) |
| **Local Large** | 64 GB | 50 GB | 24GB VRAM | Ollama 70B+ |
| **Full Stack** | 32 GB | 30 GB | 8GB VRAM | Tüm servisler |
| **Enterprise** | 64 GB+ | 100 GB+ | 24GB+ VRAM | Multi-tenant |

---

## 🌍 Dil Desteği

| Dil | Durum |
|-----|-------|
| Türkçe | ✅ Tam destek |
| English | ✅ Full support |
| Deutsch | 🔄 Partial |
| Français | 🔄 Partial |
| Español | 🔄 Partial |
| 日本語 | 🔄 Partial |
| 中文 | 🔄 Partial |
| 한국어 | 🔄 Partial |

---

## 📜 Lisans

**GNU AGPL v3.0** — Kullan, değiştir, paylaş.

Ticari kullanım için: enterprise@sentient.ai

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
| Ko-fi | [ko-fi.com/sentientos](https://ko-fi.com/sentientos) |
| Email | sentient@sentient-os.ai |

---

<p align="center">
  <b>SENTIENT OS</b><br>
  <i>The Operating System That Thinks</i><br><br>
  Made with 🦀 by the SENTIENT Team
</p>
