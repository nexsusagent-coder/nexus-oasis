# 📖 SENTIENT OS - Kapsamlı Kullanım Kılavuzu

> **Kaynak koda dayalı — her komut, her senaryo, her mod gerçek crate'lerden türetilmiştir**

---

## 📑 İçindekiler

1. [Sistem Mimarisi](#1-sistem-mimarisi)
2. [Kurulum ve Yapılandırma](#2-kurulum-ve-yapılandırma)
3. [CLI Komutları](#3-cli-komutları)
4. [LLM Hub — 57+ Provider, 245+ Model](#4-llm-hub--57-provider-245-model)
5. [JARVIS Modu — Sesli Asistan](#5-jarvis-modu--sesli-asistan)
6. [Daemon Modu — 7/24 Arka Plan](#6-daemon-modu--724-arka-plan)
7. [Otonom Desktop Agent](#7-otonom-desktop-agent)
8. [Multi-Agent Orkestrasyonu](#8-multi-agent-orkestrasyonu)
9. [Proactive Engine — Zamanlı Görevler](#9-proactive-engine--zamanlı-görevler)
10. [Akıllı Ev Kontrolü](#10-akıllı-ev-kontrolü)
11. [Kanal Entegrasyonları](#11-kanal-entegrasyonları)
12. [Persona ve Mod Sistemi](#12-persona-ve-mod-sistemi)
13. [Skill ve Tool Sistemi](#13-skill-ve-tool-sistemi)
14. [MCP — Model Context Protocol](#14-mcp--model-context-protocol)
15. [Cevahir AI — Türkçe LLM Motoru](#15-cevahir-ai--türkçe-llm-motoru)
16. [Bellek Sistemi — Memory Cube](#16-bellek-sistemi--memory-cube)
17. [Güvenlik — V-GATE & Guardrails](#17-güvenlik--v-gate--guardrails)
18. [Workflow Engine — n8n Tarzı](#18-workflow-engine--n8n-tarzı)
19. [Rust API Referansı](#19-rust-api-referansı)
20. [Sorun Giderme](#20-sorun-giderme)

---

## 1. Sistem Mimarisi

SENTIENT OS, 93 Rust crate'ten oluşan bir AI işletim sistemidir.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           SENTIENT OS v4.0.0                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─── PRESENTATION ──────────────────────────────────────────────────────┐  │
│  │  sentient_cli    sentient_web    sentient_gateway    Dashboard(Tauri)│  │
│  └─────────────────────────────────────────────────────────────────────┘  │
│                                      │                                       │
│  ┌─── SECURITY ─────────────────────────────────────────────────────────┐  │
│  │  sentient_guardrails        sentient_vgate          oasis_vault      │  │
│  │  sentient_tee               sentient_zk_mcp         sentient_compliance│  │
│  └─────────────────────────────────────────────────────────────────────┘  │
│                                      │                                       │
│  ┌─── ORCHESTRATION ───────────────────────────────────────────────────┐  │
│  │  sentient_orchestrator   sentient_agents     sentient_persona        │  │
│  │  sentient_session       sentient_modes      sentient_workflow       │  │
│  │  sentient_proactive      sentient_daemon                          │  │
│  └─────────────────────────────────────────────────────────────────────┘  │
│                                      │                                       │
│  ┌─── CORE ────────────────────────────────────────────────────────────┐  │
│  │  sentient_core         sentient_memory      sentient_graph          │  │
│  │  sentient_llm          sentient_embed       sentient_rag            │  │
│  │  sentient_cevahir      sentient_mcp                               │  │
│  └─────────────────────────────────────────────────────────────────────┘  │
│                                      │                                       │
│  ┌─── TOOLS ───────────────────────────────────────────────────────────┐  │
│  │  oasis_hands (43 tools)   oasis_browser    oasis_manus (Docker)     │  │
│  │  oasis_autonomous         oasis_brain (Gemma4)  sentient_sandbox    │  │
│  │  sentient_voice           sentient_channels  sentient_skills        │  │
│  └─────────────────────────────────────────────────────────────────────┘  │
│                                      │                                       │
│  ┌─── INTEGRATIONS ────────────────────────────────────────────────────┐  │
│  │  72+ entegre proje: CrewAI, AutoGen, LangChain, Mem0, ChromaDB...  │  │
│  │  5587+ skill: Dev, OSINT, Automation, Media...                      │  │
│  └─────────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Crate İstatistikleri:**

| Metrik | Değer |
|--------|-------|
| Rust Crate | 93 |
| Rust Kaynak Satırı | 303,490 |
| Provider | 57+ |
| Model | 245+ native, 200K+ aggregator |
| Skill | 5,587+ |
| Entegrasyon | 72+ proje |
| Kanal | 20+ platform |
| Test | 189+ (LLM), tüm crate'lerde 560+ |

---

## 2. Kurulum ve Yapılandırma

Detaylı kurulum: [INSTALL_GUIDE.md](INSTALL_GUIDE.md) | [INSTALL.md](INSTALL.md)

### Hızlı Başlangıç

```bash
# 1. Kur
curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.sh | bash

# 2. LLM ayarla (en az biri)
export OPENROUTER_API_KEY=sk-or-v1-xxx   # Cloud
# VEYA
ollama pull gemma3:27b                  # Lokal (ücretsiz)

# 3. Başlat
sentient chat
```

### .env Yapılandırması

```env
# ═══ LLM ═══
LLM_PROVIDER=openrouter                # veya ollama, openai, anthropic
LLM_MODEL=openai/gpt-4o                # varsayılan model
OPENROUTER_API_KEY=sk-or-v1-xxx

# ═══ LOKAL LLM ═══
OLLAMA_HOST=http://localhost:11434
OLLAMA_MODEL=gemma3:27b

# ═══ V-GATE ═══
V_GATE_URL=http://localhost:8100
V_GATE_LISTEN=127.0.0.1:1071

# ═══ VOICE ═══
VOICE_ENABLED=true
VOICE_STT=whisper_cpp       # veya openai_whisper
VOICE_TTS=piper             # veya elevenlabs
VOICE_LANGUAGE=tr

# ═══ DAEMON ═══
DAEMON_ASSISTANT_NAME=Sentient
DAEMON_WAKE_WORD=Hey Sentient

# ═══ GATEWAY ═══
GATEWAY_HTTP_ADDR=0.0.0.0:8080
JWT_SECRET=change-this-in-production

# ═══ MEMORY ═══
MEMORY_DB_PATH=data/sentient_memory.db

# ═══ GUARDRAILS ═══
GUARDRAILS_MODE=normal     # strict, normal, permissive

# ═══ LOG ═══
RUST_LOG=info
```

---

## 3. CLI Komutları

### 3.1 Temel Komutlar

```bash
sentient chat                    # İnteraktif sohbet
sentient ask "Soru?"             # Tek soru
sentient code "Rust REST API"   # Kod üret
sentient analyze README.md      # Dosya analizi
sentient doctor                  # Sistem kontrolü
sentient status                  # Durum
sentient --version               # Versiyon
sentient --help                  # Yardım
```

### 3.2 Model Komutları

```bash
sentient model list              # Tüm modeller
sentient model set gemma3:27b   # Model değiştir
sentient model info             # Aktif model bilgisi
```

### 3.3 Bellek Komutları

```bash
sentient memory list             # Bellek girdileri
sentient memory search "konu"   # Ara
sentient memory clear            # Temizle
sentient memory export out.json  # Dışa aktar
```

### 3.4 Agent Komutları

```bash
sentient agent list                                    # Ajanları listele
sentient agent create coder --model gpt-4o             # Oluştur
sentient agent run coder --goal "API yaz"              # Çalıştır
sentient swarm create team --agents 5                  # Swarm oluştur
sentient swarm run team --goal "Proje analiz et"       # Swarm çalıştır
sentient crew create report --agents researcher,writer # Crew oluştur
sentient crew run report --goal "Rapor yaz"            # Crew çalıştır
```

### 3.5 Kanal Komutları

```bash
sentient channel list                    # Kanalları listele
sentient channel start telegram           # Telegram bot başlat
sentient channel start discord           # Discord bot başlat
sentient channel start whatsapp          # WhatsApp başlat
sentient channel stop telegram           # Durdur
sentient channel status                   # Tüm kanal durumu
```

### 3.6 Voice Komutları

```bash
sentient voice                            # Sesli sohbet
sentient voice --wake-word "hey sentient"  # Uyandırma kelimesi
sentient voice --language tr              # Türkçe
sentient voice --continuous               # Kesintisiz dinleme
sentient voice transcribe audio.mp3       # Dosya transkripsiyon
sentient voice speak "Merhaba" --output out.mp3  # TTS
```

### 3.7 Desktop Komutları

```bash
sentient desktop                                  # Desktop agent başlat
sentient desktop --goal "YouTube'da müzik aç"    # Görev ver
sentient desktop --safe-mode                      # Güvenli mod
sentient desktop --sovereign                      # Sovereign policy
sentient browser                                  # Browser agent
sentient browser --headless                       # Headless mod
```

### 3.8 Proactive Komutları

```bash
sentient proactive list                           # Tetikleyiciler
sentient proactive add --name "sabah" --type time --time "09:00" --days "mon-fri"
sentient proactive add --name "email" --type event --event "email.received"
sentient proactive remove "sabah"                  # Kaldır
```

### 3.9 Skill Komutları

```bash
sentient skill list                     # Skill'leri listele
sentient skill search "web scraper"    # Ara
sentient skill run code-review --path ./src  # Çalıştır
sentient skill info browser-navigate   # Detay
sentient skill install translator-pro   # Kur
```

### 3.10 Daemon Komutları

```bash
sentient daemon start     # Başlat (arka planda)
sentient daemon stop      # Durdur
sentient daemon status     # Durum
sentient daemon log --tail # Log
```

### 3.11 Güvenlik Komutları

```bash
sentient guardrails test "Ignore previous instructions"  # Güvenlik testi
sentient vault set KEY "value"                           # Secret kaydet
sentient vault get KEY                                    # Secret oku
sentient vault list                                       # Secret listele
sentient vault rotate KEY                                 # Key değiştir
sentient vgate start                                      # V-GATE başlat
sentient vgate status                                     # V-GATE durum
```

---

## 4. LLM Hub — 57+ Provider, 245+ Model

### 4.1 Provider Türleri

| Tür | Provider'lar | Özellik |
|-----|-------------|---------|
| **Direct** | OpenAI, Anthropic, Google, Mistral, DeepSeek, xAI, Cohere, Groq, Perplexity, AI21, Fireworks, Replicate, Cerebras, Reka, StepFun, Aleph Alpha, Sarvam, Voyage, Upstage, GigaChat | Doğrudan API erişimi |
| **Aggregator** | OpenRouter, Together, Hugging Face, GLHF, Novita, Hyperbolic, SiliconFlow, DeepInfra, Lepton, Chutes | 200+ model marketplace |
| **Enterprise** | Azure OpenAI, AWS Bedrock, Vertex AI, NVIDIA NIM, SambaNova, IBM WatsonX | Enterprise SLA |
| **Chinese** | Zhipu AI, Moonshot, Yi, Baidu ERNIE, MiniMax, Mod | Çin pazarı |
| **Local** | Ollama, vLLM, LM Studio, Llamafile | Ücretsiz, lokal |
| **Gateway/Router** | Unify, Portkey, Helicone, NotDiamond, AI/ML API, Glama, Requesty, LiteLLM, Cloudflare Workers AI | Akıllı routing |

### 4.2 Model Seçimi

```bash
# Ücretsiz modeller
sentient chat --model "ollama/gemma3:27b"
sentient chat --model "openrouter/google/gemma-4-31b-it:free"

# En ucuz
sentient chat --model "deepseek/deepseek-v3"

# En hızlı
sentient chat --model "groq/llama-3.3-70b-versatile"

# Reasoning
sentient chat --model "deepseek/deepseek-r1"

# Vision
sentient chat --model "openai/gpt-4o"

# Akıllı routing
sentient chat --model "unify/router@q>0.9&c<0.001"
sentient chat --model "openrouter/auto"
```

### 4.3 Smart Router

SENTIENT, görev zorluğuna göre otomatik model seçer:

```rust
// Basit soru → ucuz model (deepseek-v3, $0.0001/1K)
// Kod sorusu → güçlü model (gpt-4o, $0.0025/1K)
// Reasoning → reasoning model (deepseek-r1)
// Vision → vision model (gpt-4o, gemma3)
```

---

## 5. JARVIS Modu — Sesli Asistan

**Kaynak:** `sentient_voice` + `sentient_wake` + `sentient_daemon`

### 5.1 Mimari

```
🎤 Mikrofon
  │
  ├─→ Wake Word ("Hey Sentient") ← sentient_wake
  │     │
  │     ▼
  ├─→ STT (Ses → Metin) ← sentient_voice::stt
  │     │  whisper_cpp (lokal, ücretsiz)
  │     │  openai_whisper (API, daha iyi)
  │     │
  │     ▼
  ├─→ Command Parser ← sentient_daemon::commands
  │     │  PlayMusic, PlayVideo, WebSearch, Pause, Resume
  │     │  ControlHome, SetReminder, WhatTime, Weather
  │     │  GitHubTrending, ProjectAssign
  │     │
  │     ▼
  ├─→ Action Executor ← sentient_daemon::actions
  │     │  Browser automation (oasis_browser)
  │     │  Smart home (sentient_home)
  │     │  GitHub API (sentient_connectors)
  │     │
  │     ▼
  ├─→ TTS (Metin → Ses) ← sentient_voice::tts
  │     │  piper (lokal, Türkçe, ücretsiz)
  │     │  elevenlabs (API, daha doğal)
  │     │  openai (API, iyi kalite)
  │     │
  │     ▼
  └─→ 🔊 Hoparlör
```

### 5.2 Desteklenen Sesli Komutlar (Türkçe + İngilizce)

| Komut Örneği | Intent | Aksiyon |
|---|---|---|
| "rahatlatıcı müzik aç" | PlayMusic | YouTube'da arar, ilk videoyu açar |
| "sezen aksu şarkısını aç" | PlayMusic | YouTube'da "sezen aksu music" arar |
| "yapay zeka video aç" | PlayVideo | YouTube'da video arar |
| "google'da rust ara" | WebSearch | Google'da arama yapar |
| "durdur" / "dur" | Pause | YouTube play butonuna tıklar |
| "devam et" | Resume | Oynatmaya devam eder |
| "kapat" | Close | Tarayıcı sekmesini kapatır |
| "saat kaç" | WhatTime | Saati söyler |
| "hava nasıl" | Weather | Hava durumu bilgisini verir |
| "salon ışığını kapat" | ControlHome | Home Assistant'a komut gönderir |
| "yatak odası lambasını aç" | ControlHome | Işıkları açar |
| "film modu" | ControlHome | Home Assistant scene aktifleştirir |
| "klimayı 22 derece yap" | ControlHome | Sıcaklık ayarlar |
| "github trendlere bak" | GitHubTrending | GitHub trending sayfasını açar |
| "rust github trending" | GitHubTrending | Dil filtreli trending |
| "X projesini aç ajanları yetkilendir" | ProjectAssign | Multi-agent crew başlatır |

### 5.3 Lokal JARVIS (Tamamen Ücretsiz)

```bash
# Ollama + Whisper.cpp + Piper TTS
# API KEY GEREKMİYOR!

# 1. Ollama kur + model indir
ollama pull qwen3:30b-a3b   # 4GB VRAM, MoE, Türkçe iyi

# 2. Whisper.cpp kur
git clone https://github.com/ggerganov/whisper.cpp ~/whisper.cpp
cd ~/whisper.cpp && make
bash ./models/download-ggml-model.sh medium

# 3. Piper TTS kur (Türkçe ses modeli)
wget https://github.com/rhasspy/piper/releases/download/v1.2.0/piper_1.2.0_amd64.tar.gz
mkdir -p ~/.local/share/piper/models
# Türkçe model:
wget -O ~/.local/share/piper/models/tr_TR-medium.onnx \
  https://huggingface.co/rhasspy/piper-voices/resolve/main/tr/tr_TR/medium/tr_TR-medium.onnx

# 4. .env yapılandır
VOICE_ENABLED=true
VOICE_STT=whisper_cpp
VOICE_TTS=piper
VOICE_LANGUAGE=tr
WHISPER_MODEL_PATH=~/whisper.cpp/models/ggml-medium.bin
PIPER_MODEL_PATH=~/.local/share/piper/models/tr_TR-medium.onnx

# 5. Başlat
sentient voice
```

### 5.4 API JARVIS (Daha Doğal Ses)

```bash
# .env
VOICE_ENABLED=true
VOICE_STT=openai_whisper
VOICE_TTS=elevenlabs
OPENAI_API_KEY=sk-...
ELEVENLABS_API_KEY=...

sentient voice --wake-word "hey sentient"
```

---

## 6. Daemon Modu — 7/24 Arka Plan

**Kaynak:** `sentient_daemon`

### 6.1 Daemon Başlatma

```bash
# Başlat (arka planda sürekli çalışır)
sentient daemon start

# Durum
sentient daemon status
# → Daemon: ✅ Running (PID: 12345, uptime: 2h 15m)
# → Wake word: ✅ Listening ("Hey Sentient")
# → Proactive: ✅ 3 triggers active
# → Channels: ✅ Telegram, Discord

# Durdur
sentient daemon stop

# Log
sentient daemon log --tail
```

### 6.2 Daemon State Machine

```
Stopped → Starting → Listening ⇄ Processing ⇄ Executing → Speaking → Listening
                                                                    ↓
                                                              ShuttingDown → Stopped
```

| State | Açıklama |
|-------|----------|
| Stopped | Daemon kapalı |
| Starting | Başlatılıyor |
| Listening | Wake word dinliyor |
| Processing | Sesli komutu parse ediyor |
| Executing | Aksiyon gerçekleştiriyor |
| Speaking | TTS ile yanıt veriyor |
| ShuttingDown | Kapatılıyor |

### 6.3 Daemon Komple Senaryo

```
1. Kullanıcı yatakta: "Hey Sentient, rahatlatıcı müzik aç"
2. Wake word algılandı → State: Processing
3. STT: "rahatlatıcı müzik aç" → State: Executing
4. CommandParser: intent=PlayMusic, query="rahatlatıcı"
5. ActionExecutor:
   a. Browser aç → YouTube'ye git
   b. "relaxing music" ara → İlk videoyu tıkla
   c. StealthEngine: İnsan gibi gecikme ekle
6. State: Speaking
7. TTS: "Rahatlatıcı müziği açıyorum, iyi dinlemeler"
8. State: Listening (tekrar wake word dinliyor)
```

---

## 7. Otonom Desktop Agent

**Kaynak:** `oasis_autonomous` + `oasis_hands` + `oasis_browser`

### 7.1 Mimari

```
┌─────────────────────────────────────────────────────────────┐
│                  OASIS AUTONOMOUS AGENT                     │
│                                                             │
│  PERCEIVE → DECIDE → ACT → LEARN                           │
│     │           │        │       │                          │
│  ┌──▼──┐   ┌───▼───┐  ┌──▼──┐  ┌──▼──┐                    │
│  │Screen│   │Planner│  │Tools│  │Memory│                    │
│  │Vision│   │Safety │  │Chain│  │Heal  │                    │
│  └──────┘   └───────┘  └─────┘  └──────┘                    │
│                                                             │
│  Sovereign Constitution L1:                                │
│  ✗ rm -rf, format, dd, sudo, chmod 777 (50+ yasaklı)     │
│  ✓ libreoffice, firefox, vscode, git, cargo               │
└─────────────────────────────────────────────────────────────┘
```

### 7.2 Agent State'leri

| State | Açıklama |
|-------|----------|
| Idle | Boşta |
| Initializing | Başlatılıyor |
| Perceiving | Ekran analiz ediyor |
| Deciding | Karar veriyor |
| Acting | Aksiyon gerçekleştiriyor |
| Learning | Öğreniyor / Belleğe kaydediyor |
| Error | Hata durumunda |
| Paused | Duraklatılmış |

### 7.3 Desteklenen Aksiyonlar

| Aksiyon | Açıklama |
|---------|----------|
| MouseMove | Fare hareketi |
| MouseClick | Fare tıklama (sol/sağ/orta) |
| MouseDrag | Sürükleme |
| MouseScroll | Scroll |
| KeyPress | Tuş basma |
| KeyShortcut | Kısayol (Ctrl+C vs.) |
| TypeText | Metin yazma (human_like=true ise doğal) |
| BrowserNavigate | URL'ye git |
| BrowserClick | Elemente tıkla |
| BrowserType | Input'a yaz |
| Composite | Çoklu aksiyon zinciri |
| Custom | Özel aksiyon |

### 7.4 Kullanım

```bash
# Web araştırması
sentient desktop --goal "Rust web framework karşılaştırması yap"

# Kod yazma
sentient desktop --goal "Rust REST API projesi oluştur"

# Günlük rapor
sentient desktop --goal "Bugünkü GitHub commit'lerimi özetle"

# Güvenli mod (kritik aksiyonlarda insan onayı)
sentient desktop --safe-mode

# Sovereign (yasaklı komutlar asla çalışmaz)
sentient desktop --sovereign
```

### 7.5 Self-Healing

Agent hata aldığında otomatik kurtarır:
1. Hata paterni analiz eder
2. Yeniden deneme stratejisi seçer
3. Alternatif yaklaşım dener
4. Sonucu belleğe kaydeder (gelecekte aynı hatadan kaçınmak)

---

## 8. Multi-Agent Orkestrasyonu

**Kaynak:** `sentient_agents` + `sentient_orchestrator`

### 8.1 Desteklenen Framework'ler

| Framework | Kaynak | Durum | Tarz |
|-----------|--------|-------|------|
| CrewAI | integrations/agents/crewai | ✅ READY | Rol bazlı |
| AutoGen | integrations/agents/autogen | ✅ READY | Konversation |
| Swarm | integrations/agents/swarm | ✅ READY | Hafif orkestrasyon |
| MetaGPT | integrations/agents/metagpt | ✅ READY | Şirket modeli |
| Agent-S | integrations/agents/agent-s | ✅ READY | Desktop |
| SENTIENT Native | crates/sentient_agents | ✅ ACTIVE | Yerel |

### 8.2 CrewAI — Rol Bazlı Ekip

```bash
# CLI ile
sentient crew create report-team \
  --agents "researcher:deepseek-r1,writer:gpt-4o,editor:claude-4-sonnet"

sentient crew run report-team --goal "AI pazar analizi raporu yaz"
```

### 8.3 AutoGen — Konversation Bazlı

```bash
# Kod yazıcı + İnceleyici konuşuyor
sentient agent create coder --model gpt-4o --role "Kod yaz"
sentient agent create reviewer --model claude-4-sonnet --role "Kod incele"
sentient agent converse coder,reviewer --topic "Rust web API tasarımı"
```

### 8.4 MetaGPT — Şirket Modeli

```bash
# Product Manager + Architect + Engineer + QA
sentient agent create pm --role "Product Manager"
sentient agent create architect --role "Architect"
sentient agent create engineer --role "Engineer"
sentient agent create qa --role "QA Engineer"

sentient crew run software-team --framework metagpt --goal "Sosyal medya uygulaması geliştir"
```

### 8.5 Swarm Orkestrasyonu

Orchestrator, SwarmCoordinator ile çoklu ajan koordinasyonu yapar:
- `SwarmCoordinator` — Ajanları yönetir
- `Blackboard` — Paylaşılan bellek
- `CollectiveMemory` — Toplu deneyim
- `AgentMarketplace` — Ajan ticareti (planlanan)

### 8.6 Paralel Görevler

```rust
// Orchestrator paralel çalıştırabilir
let goals = vec![
    Goal::new("Piyasa araştırması yap"),
    Goal::new("Rakip analizi yap"),
    Goal::new("Teknik rapor yaz"),
];
orchestrator.execute_parallel(goals).await?;
```

### 8.7 Self-Healing (Orchestrator seviyesinde)

```
Hata → ErrorPattern analizi → HealingStrategy seçimi
  → Yeniden deneme (exponential backoff)
  → Alternatif ajan devreye girme
  → Kod düzeltme (self_healing.rs)
  → Sonuç belleğe kaydetme
```

### 8.8 Dynamic Router

Görev zorluğuna göre otomatik model seçimi:
```rust
DynamicRouter → ComplexityAnalyzer → TaskAnalysis
  → TaskType::Simple → ucuz model
  → TaskType::Moderate → dengeli model
  → TaskType::Complex → güçlü model
  → TaskType::Reasoning → reasoning model
```

---

## 9. Proactive Engine — Zamanlı Görevler

**Kaynak:** `sentient_proactive`

### 9.1 Trigger Türleri

| Tür | Örnek | Açıklama |
|-----|-------|----------|
| **TimeBased** | "Saat 09:00, Pazartesi-Cuma" | Belirli saatte |
| **EventBased** | "Email geldiğinde" | Olay tetiklemeli |
| **PatternBased** | "Her Cuma 17:00" | Düzenli tekrar |
| **Cron** | "0 9 * * 1-5" | Cron expression |

### 9.2 Sabah Bülteni Senaryosu

```bash
sentient proactive add \
  --name "morning-brief" \
  --type time \
  --time "09:00" \
  --days "mon-fri" \
  --action "generate-briefing"
```

**Sabah 9'da çalışan akış:**
1. `sentient_digest` — Günlük bülten oluşturur
2. `sentient_connectors` — Gmail, Calendar, GitHub, Weather API'lerine bağlanır
3. `sentient_llm` — LLM ile özet oluşturur
4. `sentient_voice` — TTS ile sesli okur (daemon aktifse)
5. `sentient_channels` — Telegram'a gönderir

### 9.3 Event Bazlı Trigger

```bash
# Email geldiğinde
sentient proactive add \
  --name "urgent-email" \
  --type event \
  --event "email.received" \
  --condition "subject contains 'ACIL'" \
  --action "notify-telegram"

# GitHub PR açıldığında
sentient proactive add \
  --name "pr-review" \
  --type event \
  --event "github.pr_opened" \
  --action "auto-review"
```

---

## 10. Akıllı Ev Kontrolü

**Kaynak:** `sentient_home`

### 10.1 Home Assistant Entegrasyonu

```bash
# .env
HOME_ASSISTANT_URL=http://homeassistant.local:8123
HOME_ASSISTANT_TOKEN=eyJ0eXAi...

# Sentinel Home client
sentient home connect
sentient home status
```

### 10.2 Sesli Komutlar (Daemon ile)

| Komut | Aksiyon | Cihaz |
|-------|---------|-------|
| "salon ışığını kapat" | turn_off | light.living_room |
| "yatak odası lambasını aç" | turn_on | light.bedroom |
| "klimayı 22 derece yap" | set_temperature | climate |
| "film modu" | activate_scene | movie |
| "uyku modu" | activate_scene | good_night |
| "sabah modu" | activate_scene | good_morning |

### 10.3 Rust API

```rust
use sentient_home::{HomeClient, DeviceCommand};

let home = HomeClient::connect("http://homeassistant.local:8123", "TOKEN").await?;

// Işık aç
home.execute_command(DeviceCommand::TurnOn("light.living_room".into())).await?;

// Scene aktifleştir
home.activate_scene("good_night").await?;
```

---

## 11. Kanal Entegrasyonları

**Kaynak:** `sentient_channels`

### 11.1 Desteklenen Platformlar (20+)

| Platform | Tür | Durum |
|----------|-----|-------|
| Telegram | Mesajlaşma | ✅ |
| Discord | Topluluk | ✅ |
| WhatsApp | Mesajlaşma | ✅ |
| Slack | İş | ✅ |
| Email (IMAP/SMTP) | İletişim | ✅ |
| Microsoft Teams | İş | 🔄 |
| Signal | Güvenli | 🔄 |
| Matrix | Açık kaynak | 🔄 |
| iMessage | Apple | 🔄 |
| Instagram | Sosyal | 🔄 |
| LinkedIn | Profesyonel | 🔄 |
| Twitter/X | Sosyal | 🔄 |
| Line | Asya | 🔄 |
| WeChat | Çin | 🔄 |
| Messenger | Sosyal | 🔄 |
| Mattermost | Açık kaynak | 🔄 |
| Google Chat | İş | 🔄 |
| Webex | Video | 🔄 |
| Zoom | Video | 🔄 |
| Chime | İş | 🔄 |

### 11.2 Telegram Bot Kurulumu

```bash
# 1. @BotFather'dan token al
# 2. .env'e ekle
TELEGRAM_BOT_TOKEN=123456:ABC-DEF1234ghIkl

# 3. Başlat
sentient channel start telegram

# Bot artık mesajları yanıtlar!
# /start, /help, /status, /ask, /code, /search
```

### 11.3 Çoklu Kanal Aynı Anda

```bash
TELEGRAM_BOT_TOKEN=xxx
DISCORD_BOT_TOKEN=xxx
SLACK_BOT_TOKEN=xxx

sentient daemon start --channels telegram,discord,slack
```

---

## 12. Persona ve Mod Sistemi

**Kaynak:** `sentient_persona` + `sentient_modes`

### 12.1 Persona Sistemi

```bash
# Persona oluştur
sentient persona create "Rust Uzmanı" --traits "concise,technical"

# Aktif persona ayarla
sentient persona set "Rust Uzmanı"

# Persona listele
sentient persona list

# Template'den oluştur
sentient persona from-template developer --name "Kod Asistanı"
```

**Persona Bileşenleri:**
- Identity (isim, açıklama, backstory)
- PersonalityTraits (dil, ton, stil)
- BehaviorPatterns (yanıt kalıpları)
- DynamicAdaptationEngine (kullanıcıya uyum sağlar)
- MultiLanguageSupport (8 dil)
- PersonaAnalytics (kullanım istatistikleri)

### 12.2 Mod Sistemi

```bash
sentient mode set coding      # Kod yazma modu
sentient mode set research    # Araştırma modu
sentient mode set chat        # Sohbet modu
sentient mode set enterprise  # Enterprise modu
```

---

## 13. Skill ve Tool Sistemi

**Kaynak:** `sentient_skills` + `oasis_hands`

### 13.1 Skill Kategorileri (5,587+)

| Kategori | Sayı | Alt Kategoriler |
|----------|------|-----------------|
| Dev | 2,965+ | Coding-Agents, Web-Frontend, DevOps-Cloud, Git-GitHub |
| OSINT | 1,050+ | Search-Research, Browser-Automation, Data-Analytics |
| Social | 238+ | Communication, Marketing-Sales |
| Automation | 306+ | Productivity, Calendar, Smart-Home |
| Media | 246+ | Image-Video-Gen, Streaming, Speech |
| Productivity | 214+ | Notes-PKM, PDF-Documents |
| Security | 52+ | Security-Passwords |
| Mobile | 233+ | Transportation, Health-Fitness |
| Gaming | 108+ | Gaming, Personal-Dev |

### 13.2 Oasis Hands — 43+ Native Tool

| Tool | Açıklama |
|------|----------|
| bash | Shell komut çalıştırma |
| browser | Web tarayıcı kontrolü |
| file_read | Dosya okuma |
| file_write | Dosya yazma |
| file_edit | Dosya düzenleme |
| git | Git işlemleri |
| grep | Metin arama |
| glob | Dosya deseni eşleştirme |
| memory | Bellek erişimi |
| email | Email gönderme/alma |
| calendar | Takvim erişimi |
| web_search | Web arama |
| web_fetch | URL içeriği çekme |
| screenshot | Ekran görüntüsü |
| pdf | PDF okuma/yazma |
| notify | Bildirim gönderme |
| translate | Çeviri |
| code_review | Kod inceleme |
| config | Yapılandırma |
| task | Görev yönetimi |
| todo_write | Todo listesi |
| lsp | Language Server Protocol |
| mcp | MCP araçları |
| n8n | n8n workflow |
| sed | Metin değiştirme |
| + 19 araç daha | |

---

## 14. MCP — Model Context Protocol

**Kaynak:** `sentient_mcp`

Anthropic'ın Model Context Protocol implementasyonu — Claude Desktop, Cursor, Windsurf uyumlu.

### 14.1 Server Oluşturma

```rust
use sentient_mcp::{Server, ServerConfig, tool::{ToolExecutor, ToolCall, Tool, ToolResult}};

struct EchoTool;

#[async_trait]
impl ToolExecutor for EchoTool {
    async fn execute(&self, call: ToolCall) -> sentient_mcp::Result<ToolResult> {
        let input = call.arguments.get("text").and_then(|v| v.as_str()).unwrap_or("");
        Ok(ToolResult::text(format!("Echo: {}", input)))
    }
    
    fn definition(&self) -> Tool {
        Tool::simple("echo", "Echo the input text back")
    }
}

let mut server = Server::new(ServerConfig::default());
server.register_tool(EchoTool);
// server.run().await?;
```

### 14.2 Transport Türleri

| Transport | Açıklama |
|-----------|----------|
| stdio | Standart giriş/çıkış |
| TCP | TCP socket |
| WebSocket | WS bağlantı |
| SSE | Server-Sent Events |

---

## 15. Cevahir AI — Türkçe LLM Motoru

**Kaynak:** `sentient_cevahir`

### 15.1 Mimari

| Bileşen | Açıklama |
|---------|----------|
| Neural Network (V-7) | RoPE, RMSNorm, SwiGLU, KV Cache, MoE, GQA |
| CognitiveManager | Direct, Think, Debate, TreeOfThoughts |
| TokenizerWrapper | Türkçe BPE Tokenizer (60K kelime) |
| CevahirBridge | SENTIENT'e PyO3 bridge |
| MemoryAdapter | Epizodik/semantik bellek |
| ToolExecutor | Dinamik araç kaydı |

### 15.2 Cognitive Stratejiler

```rust
use sentient_cevahir::{CevahirBridge, CognitiveStrategy};

// Basit soru
let output = bridge.process_with_strategy("Merhaba", CognitiveStrategy::Direct).await?;

// Kod analizi
let output = bridge.process_with_strategy("Bu kodu analiz et", CognitiveStrategy::Think).await?;

// Tasarım kararı
let output = bridge.process_with_strategy("Hangi framework?", CognitiveStrategy::Debate).await?;

// Debug/reasoning
let output = bridge.process_with_strategy("Kök neden analizi", CognitiveStrategy::TreeOfThoughts).await?;
```

---

## 16. Bellek Sistemi — Memory Cube

**Kaynak:** `sentient_memory`

### 16.1 Bellek Türleri

| Tür | Açıklama | Örnek |
|-----|----------|-------|
| Episodic | Deneyimler | "Dün toplantıda API konuştuk" |
| Semantic | Bilgi | "Kullanıcı Rust seviyor" |
| Procedural | Yöntem | "Test yazarken önce planla" |

### 16.2 Rust API

```rust
use sentient_memory::{MemoryCube, MemoryType, MemoryInput, Importance};

let mut cube = MemoryCube::new("data/memory.db")?;

// Kaydet
let id = cube.create_with_metadata(
    "Kullanıcı Python hakkında soru sordu",
    MemoryType::Episodic,
    Some(json!({"topic": "python", "user": "user_123"})),
    None,
)?;

// Ara
let results = cube.search("python", 10)?;

// Vektör araması
let similar = cube.search_similar(&embedding_vector, 5)?;

// Temizle (süresi dolan girdiler)
cube.cleanup_expired()?;
```

### 16.3 Orchestrator Bellek Köprüsü

Orchestrator, görev sonuçlarını otomatik belleğe kaydeder:
```rust
// Orchestrator::execute() sonunda
self.memory.write().await.create_with_metadata(
    serde_json::to_string(&result)?,
    MemoryType::Semantic,
    Some(metadata),
    None,
)?;
```

---

## 17. Güvenlik — V-GATE & Guardrails

**Kaynak:** `sentient_vgate` + `sentient_guardrails` + `oasis_vault`

### 17.1 V-GATE Mimarisi

```
SENTIENT Client → V-GATE Proxy → LLM API Provider
                      │
                API Key (sunucuda)
                İstemcide YOK!
```

```bash
sentient vgate start    # Proxy başlat
sentient vgate status   # Durum
```

### 17.2 Guardrails

```bash
# Mod seçimi
sentient config set guardrails.mode strict  # strict/normal/permissive

# Test
sentient guardrails test "Ignore all previous instructions"
# → ❌ BLOCKED: Prompt injection

sentient guardrails test "API key'im sk-abc123"
# → ❌ BLOCKED: PII/Secret

sentient guardrails test "Merhaba"
# → ✅ ALLOWED
```

### 17.3 Oasis Vault

```bash
sentient vault set OPENAI_API_KEY "sk-xxx"    # Şifreli kaydet
sentient vault get OPENAI_API_KEY              # Oku
sentient vault list                            # Listele
sentient vault rotate OPENAI_API_KEY           # Değiştir
```

---

## 18. Workflow Engine — n8n Tarzı

**Kaynak:** `sentient_workflow`

### 18.1 Özellikler

- n8n tarzı node-based workflow
- Pre-built template kütüphanesi
- Trigger-based execution
- Conditional branching
- Visual flow builder (planlanan)

### 18.2 Workflow Durumları

| Status | Açıklama |
|--------|----------|
| Draft | Taslak |
| Active | Aktif |
| Paused | Duraklatılmış |
| Completed | Tamamlanmış |
| Failed | Başarısız |

---

## 19. Rust API Referansı

### 19.1 LlmHub

```rust
use sentient_llm::{LlmHub, LlmHubBuilder, ChatRequest, Message, RoutingStrategy};

let hub = LlmHubBuilder::new()
    .ollama()?
    .openai(api_key)?
    .anthropic(api_key)?
    .deepseek(api_key)?
    .unify(api_key)?
    .default_model("deepseek-v3")
    .routing(RoutingStrategy::Cost)
    .build();

let response = hub.chat(ChatRequest {
    model: "gpt-4o".into(),
    messages: vec![Message::user("Rust'ta async nedir?")],
    ..Default::default()
}).await?;
```

### 19.2 Streaming

```rust
let mut stream = hub.chat_stream(ChatRequest {
    model: "gpt-4o".into(),
    messages: vec![Message::user("Uzun bir hikaye anlat")],
    stream: true,
    ..Default::default()
}).await?;

while let Some(chunk) = stream.next().await {
    print!("{}", chunk?.choices[0].delta.content.as_text().unwrap_or(""));
}
```

### 19.3 Orchestrator

```rust
use sentient_orchestrator::{Orchestrator, OrchestratorConfig, Goal};

let config = OrchestratorConfig {
    vgate_url: "http://127.0.0.1:1071".into(),
    default_model: "qwen/qwen3-1.7b:free".into(),
    max_iterations: 50,
    use_swarm: true,
    ..Default::default()
};

let orchestrator = Orchestrator::new(config).await?;

// Tek görev
let result = orchestrator.execute(Goal::new("API yaz")).await?;

// Paralel görevler
let results = orchestrator.execute_parallel(vec![
    Goal::new("Araştırma yap"),
    Goal::new("Kod yaz"),
]).await?;
```

### 19.4 Autonomous Agent

```rust
use oasis_autonomous::{AutonomousAgent, AgentConfig, Action};

let agent = AutonomousAgent::new(AgentConfig::default());

let result = agent.run("Firefox'ta YouTube'a git").await?;
```

### 19.5 Proactive Engine

```rust
use sentient_proactive::{ProactiveEngine, Trigger, TriggerType, Action};

let engine = ProactiveEngine::new();

engine.add_trigger(Trigger::new(
    "morning-brief",
    "Morning Briefing",
    TriggerType::TimeBased {
        time: "09:00".into(),
        days: vec![1, 2, 3, 4, 5],
    },
)).await;

engine.start().await;
```

### 19.6 Voice Assistant

```rust
use sentient_voice::{VoiceAssistant, VoiceConfig, VoiceProvider};

let assistant = VoiceAssistant::new(VoiceConfig {
    stt_provider: VoiceProvider::WhisperCpp,
    tts_provider: VoiceProvider::Piper,
    language: "tr".into(),
    ..Default::default()
});

assistant.start().await?;
```

---

## 20. Sorun Giderme

### 20.1 Yaygın Sorunlar

| Sorun | Çözüm |
|-------|-------|
| Ollama bağlantı hatası | `ollama serve &` + `ollama list` |
| API Key hatası | `.env` kontrol et veya `export OPENAI_API_KEY=...` |
| Rust derleme hatası | `rustup update && cargo clean && cargo build --release` |
| Port kullanımda | `sudo lsof -i :8080` + `sudo kill -9 PID` |
| Python modül hatası | `source .venv/bin/activate` + `pip install maturin` |
| Whisper bulunamadı | `WHISPER_MODEL_PATH` kontrol et |
| Piper ses gelmiyor | `PIPER_MODEL_PATH` kontrol et, Türkçe model indir |
| Home Assistant bağlantı yok | `HOME_ASSISTANT_URL` ve `TOKEN` kontrol et |
| Telegram bot cevap vermiyor | `TELEGRAM_BOT_TOKEN` doğru mu? |

### 20.2 Loglama

```bash
# Detaylı log
RUST_LOG=debug sentient chat
RUST_LOG=trace sentient chat

# Belirli modül
RUST_LOG=sentient_llm=debug sentient chat
RUST_LOG=sentient_daemon=debug sentient daemon start

# Log dosyası
tail -f logs/sentient.log
```

### 20.3 Sıfırlama

```bash
sentient config reset     # Yapılandırmayı sıfırla
sentient memory clear     # Belleği temizle
rm -rf ~/.sentient data/  # Tam sıfırlama
cargo clean               # Build temizle
./install.sh              # Yeniden kur
```

---

## 📚 Ek Kaynaklar

| Dosya | Açıklama |
|-------|----------|
| [INSTALL.md](INSTALL.md) | Kapsamlı kurulum (Gemma 4, Docker, tüm entegrasyonlar) |
| [INSTALL_GUIDE.md](INSTALL_GUIDE.md) | Universal kurulum (Linux/macOS/Windows/Docker/K8s) |
| [QUICKSTART.md](QUICKSTART.md) | Hızlı başlangıç |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Sistem mimarisi (A1-A12) |
| [docs/USAGE_SCENARIOS.md](docs/USAGE_SCENARIOS.md) | Kullanım senaryoları |
| [docs/GETTING_STARTED.md](docs/GETTING_STARTED.md) | Başlangıç rehberi |
| [SECURITY.md](SECURITY.md) | Güvenlik |
| [MODEL_PROVIDERS.md](MODEL_PROVIDERS.md) | LLM provider detayları |
| [SISTEM_DOKUMANTASYONU.md](SISTEM_DOKUMANTASYONU.md) | Tam sistem dokümantasyonu |

---

<p align="center">
  <b>SENTIENT OS</b> - The Operating System That Thinks<br>
  Made with 🦀 by the SENTIENT Team
</p>
