# 🐺 SENTIENT OS — Düşünen İşletim Sistemi

> **JARVIS seviyesi yapay zeka asistanı. Tek komutla kur, konuşarak kullan.**

SENTIENT OS, Rust ile yazılmış tam donanımlı bir yapay zeka işletim sistemidir. 93 crate, 42+ LLM provider, 5.587+ skill, 24 kanal entegrasyonu ve JARVIS seviyesi sesli asistan içerir.

---

## 🚀 Tek Komutla Kurulum

```bash
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
cd SENTIENT_CORE
chmod +x install.sh
./install.sh
```

Kurulum sihirbazı adım adım yönlendirir:
1. Sistem bağımlılıkları (Rust, Docker, vs.)
2. LLM seçimi → **Lokal** (ücretsiz) / **API** (ücretli) / **Hibrit**
3. Voice kurulumu → **Whisper.cpp + Piper** (ücretsiz) / **OpenAI + ElevenLabs** (ücretli)
4. Docker servisleri (PostgreSQL, Redis, Qdrant, MinIO)
5. Rust derleme (release mode)

---

## ⚡ Hızlı Başlangıç

### İlk Çalıştırma

```bash
# Yapılandırma sihirbazı
./target/release/sentient init

# Sistem kontrolü
./target/release/sentient doctor

# Sohbete başla
./target/release/sentient chat

# Tek soru sor
./target/release/sentient ask "Rust'ta async nasıl çalışır?"
```

### Sesli Asistan (JARVIS Modu)

```bash
# Mikrofondan dinle, sesle cevapla
./target/release/sentient voice

# Belirli bir uyandırma kelimesiyle ("Hey Sentient")
./target/release/sentient voice --wake-word "hey sentient"
```

### Web Dashboard

```bash
# Dashboard'u başlat
./target/release/sentient dashboard

# Tarayıcıda aç
# http://localhost:8080/dashboard
```

### Arka Plan Servisi (7/24)

```bash
# Daemon modunda çalışır — her zaman hazır
./target/release/sentient serve
```

---

## 🧠 LLM Seçenekleri

### Lokal (Ücretsiz — İnternet Gerekmez)

```bash
# Ollama kur ve model indir
curl -fsSL https://ollama.com/install.sh | sh
ollama pull gemma3:27b      # Önerilen (16GB RAM)
ollama pull llama3.2:3b     # Hafif (4GB RAM)
ollama pull deepseek-r1:7b  # Reasoning (8GB RAM)

# .env'e ekle
echo "LLM_PROVIDER=ollama" >> .env
echo "OLLAMA_HOST=http://localhost:11434" >> .env
```

### API (Ücretli — En Yüksek Kalite)

```bash
# OpenRouter (önerilen) — 200+ model, $5 başlangıç bonusu
echo "LLM_PROVIDER=openrouter" >> .env
echo "OPENROUTER_API_KEY=sk-or-v1-..." >> .env

# Veya doğrudan provider
echo "OPENAI_API_KEY=sk-..." >> .env          # GPT-4, GPT-4o
echo "ANTHROPIC_API_KEY=sk-ant-..." >> .env    # Claude 3.5 Sonnet
echo "GOOGLE_AI_API_KEY=..." >> .env           # Gemini 2.0
```

### Hibrit (En İyi Denge)

Lokal model kullanır, zor sorularda API'ye başvurur:

```bash
echo "LLM_MODE=hybrid" >> .env
echo "LLM_LOCAL_MODEL=gemma3:27b" >> .env
echo "LLM_API_MODEL=openai/gpt-4o" >> .env
```

---

## 🎙️ Voice — JARVIS Seviyesi Sesli Asistan

### Nasıl Çalışır

```
🎤 Mikrofon → Whisper STT → LLM → TTS → 🔊 Hoparlör
     ↑                                    |
     └── Wake Word (Porcupine/Vosk) ──────┘
```

### Lokal Voice (Ücretsiz)

```bash
# Whisper.cpp (STT — Konuşmadan metne)
git clone https://github.com/ggerganov/whisper.cpp
cd whisper.cpp && make
bash ./models/download-ggml-model.sh medium

# Piper TTS (Metinden konuşmaya — Türkçe destekli)
wget https://github.com/rhasspy/piper/releases/download/v1.2.0/piper_1.2.0_amd64.tar.gz
# Türkçe model: tr_TR-medium.onnx

# .env
echo "VOICE_ENABLED=true" >> .env
echo "VOICE_STT=whisper_cpp" >> .env
echo "VOICE_TTS=piper" >> .env
```

### API Voice (Ücretli)

```bash
echo "VOICE_STT=openai_whisper" >> .env
echo "VOICE_TTS=elevenlabs" >> .env
echo "ELEVENLABS_API_KEY=..." >> .env
```

### Voice Komutları

```bash
sentient voice                          # Temel sesli sohbet
sentient voice --wake-word "hey sentient"  # Uyandırma kelimesi
sentient voice --language tr             # Türkçe
sentient voice --continuous              # Kesintisiz dinleme
```

---

## 📱 Kanal Entegrasyonları (24 Platform)

SENTIENT aynı anda 24 platformda çalışabilir:

| Platform | Tür | Kurulum |
|----------|-----|---------|
| **Telegram** | Mesajlaşma | `TELEGRAM_BOT_TOKEN=... ./target/release/sentient gateway` |
| **Discord** | Mesajlaşma | `DISCORD_BOT_TOKEN=... ./target/release/sentient gateway` |
| **Slack** | İş | `SLACK_BOT_TOKEN=... ./target/release/sentient gateway` |
| **WhatsApp** | Mesajlaşma | `WHATSAPP_TOKEN=... ./target/release/sentient gateway` |
| **Email** | İletişim | `.env` → SMTP ayarları |
| **Microsoft Teams** | İş | `.env` → Teams bot ayarları |
| **Signal** | Güvenli mesaj | `SIGNAL_PHONE_NUMBER=...` |
| **Matrix** | Açık kaynak | `.env` → Matrix ayarları |
| **Zoom** | Video | `.env` → Zoom bot ayarları |
| **Webex** | Video | `.env` → Webex ayarları |
| + 14 platform daha | | `.env.template` dosyasına bakın |

---

## 🔧 CLI Komutları

### Temel

```bash
sentient status          # Sistem durumu
sentient doctor          # Sorun giderme
sentient init            # İlk kurulum sihirbazı
sentient chat            # İnteraktif sohbet
sentient ask "..."       # Tek soru sor
sentient voice           # Sesli asistan
sentient serve           # 7/24 arka plan servisi
```

### Agent & Swarm

```bash
sentient agent list                        # Agent'ları listele
sentient agent create coder --model gpt-4o # Agent oluştur
sentient agent run coder --goal "API yaz"  # Agent çalıştır
sentient swarm create team --agents 5      # Swarm oluştur
sentient swarm run team --goal "Proje"     # Swarm çalıştır
```

### Skill & Memory

```bash
sentient skill list                       # 5.587+ skill listele
sentient skill run code-review --path ./src
sentient skill search "web scraping"
sentient memory list                      # Bellek girişleri
sentient memory search "geçen hafta"      # Bellek ara
```

### Model & Gateway

```bash
sentient model list                       # Desteklenen modeller
sentient model set gemma3:27b             # Varsayılan model
sentient gateway                          # API Gateway başlat
sentient dashboard                        # Web Dashboard başlat
```

---

## 🐳 Docker ile Kurulum

```bash
# Tüm servisleri başlat (PostgreSQL, Redis, Qdrant, MinIO, Prometheus, Grafana)
docker-compose up -d

# Sadece temel servisler
docker-compose up -d postgres redis qdrant

# Durum kontrolü
docker-compose ps

# Durdur
docker-compose down
```

### Docker Servisleri

| Servis | Port | Açıklama |
|--------|------|----------|
| PostgreSQL | 5432 | Ana veritabanı |
| Redis | 6379 | Cache & Queue |
| Qdrant | 6333 | Vektör veritabanı |
| MinIO | 9000/9001 | Object storage |
| Prometheus | 9090 | Metrik toplama |
| Grafana | 3001 | Dashboard |
| SearXNG | 8888 | Arama motoru |

---

## 🏗️ Mimari

```
┌─────────────────────────────────────────────────────────────────┐
│                      SENTIENT OS v4.0.0                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  🧠 CORE ENGINE           🔒 SECURITY         🎤 VOICE        │
│  ├── sentient_core         ├── guardrails       ├── stt         │
│  ├── sentient_memory       ├── vgate (proxy)    ├── tts         │
│  ├── sentient_graph        ├── tee              ├── wake word   │
│  ├── sentient_orchestrator ├── zk_mcp           ├── emotion     │
│  └── sentient_cevahir      ├── vault            └── diarization │
│                             └── anomaly                          │
│  🤖 LLM HUB               📱 CHANNELS                          │
│  ├── 42+ providers         ├── Telegram                         │
│  ├── 355+ native models    ├── Discord                          │
│  ├── smart router          ├── Slack                            │
│  ├── circuit breaker       ├── WhatsApp                         │
│  ├── streaming             ├── Email                            │
│  └── cost tracker          └── 19 more...                       │
│                                                                 │
│  🛠️ SKILLS & TOOLS         🏢 ENTERPRISE                        │
│  ├── 5.587+ skills         ├── RBAC                             │
│  ├── skill weaver          ├── SSO (Okta, Auth0, Azure)         │
│  ├── marketplace           ├── Multi-tenant                     │
│  ├── code execution        ├── Audit logging                    │
│  ├── sandbox               ├── MFA                             │
│  └── plugin system         └── SCIM                             │
│                                                                 │
│  🌐 WEB                    🏠 SMART HOME                         │
│  ├── REST API              ├── Home Assistant                   │
│  ├── WebSocket             ├── Türkçe komut                     │
│  ├── Dashboard             ├── Otomasyon                        │
│  └── Mobile-First          └── Sensör kontrolü                  │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│  🦀 93 Rust Crate  ·  72 Entegrasyon  ·  993+ Test             │
└─────────────────────────────────────────────────────────────────┘
```

---

## 📦 Crate Yapısı

| Kategori | Crate'ler | Açıklama |
|----------|-----------|----------|
| **Core** | sentient_core, sentient_memory, sentient_graph, sentient_orchestrator, sentient_cevahir | Agent motoru, bellek, event graph |
| **LLM** | sentient_llm, sentient_gateway, sentient_local, sentient_groq, sentient_embed, sentient_rerank | 42+ provider, 355+ model |
| **Voice** | sentient_voice, sentient_wake | STT, TTS, wake word, diarization |
| **Channels** | sentient_channels | 24 platform entegrasyonu |
| **Security** | sentient_guardrails, sentient_tee, sentient_zk_mcp, oasis_vault, sentient_anomaly | Güvenlik katmanı |
| **Enterprise** | sentient_enterprise, sentient_compliance, sentient_sla, sentient_audit | RBAC, SSO, audit |
| **AI** | sentient_rag, sentient_vision, sentient_mcp, sentient_patterns, sentient_knowledge | RAG, görü, MCP |
| **Execution** | sentient_sandbox, sentient_python, sentient_plugin, oasis_autonomous | Sandbox, PyO3, plugin |
| **Web** | sentient_web, dashboard | API server, web UI |
| **Personal** | sentient_email, sentient_calendar, sentient_todo, sentient_home, sentient_social, sentient_digest | Günlük asistan |
| **Infra** | sentient_observability, sentient_benchmarks, sentient_i18n, sentient_backup, sentient_dr | İzleme, benchmark, yedek |

---

## 🌍 Dil Desteği

SENTIENT OS Türkçe ve İngilizce tam destekler:

```bash
# Türkçe kullanım
sentient chat --language tr
sentient voice --language tr

# Türkçe oda komutları
"Salon ışığını aç"
"Koridor ısısı 22 derece yap"
"Yatak odası lambasını kıs"
```

Cevahir AI entegrasyonu ile Türkçe LLM cognitive engine dahildir.

---

## 🔐 Güvenlik

| Özellik | Açıklama |
|---------|----------|
| **V-GATE** | API key'ler asla istemcide değil, proxy üzerinden |
| **Guardrails** | Prompt injection, PII, secret filtreleme |
| **Vault** | AES-256-GCM şifreleme, key rotation |
| **TEE** | AMD SEV-SNP, Intel TDX (simülasyon) |
| **ZK-MCP** | Zero-knowledge proof (simülasyon) |
| **RBAC** | Rol bazlı erişim kontrolü |
| **Audit** | Tüm işlemler loglanır |

---

## 📊 Sistem Gereksinimleri

| Mod | RAM | Disk | GPU | Açıklama |
|-----|-----|------|-----|----------|
| **API Only** | 2 GB | 5 GB | Yok | En hafif — sadece API provider |
| **Local Small** | 8 GB | 10 GB | İsteğe bağlı | Ollama 3B-7B modeller |
| **Local Medium** | 16 GB | 20 GB | 8GB VRAM | Ollama 27B (önerilen) |
| **Local Large** | 64 GB | 50 GB | 24GB VRAM | Ollama 70B+ modeller |
| **Full Stack** | 32 GB | 30 GB | 8GB VRAM | Tüm servisler + dashboard |
| **Enterprise** | 64 GB+ | 100 GB+ | 24GB+ VRAM | Multi-tenant + monitoring |

---

## 🛠️ Geliştirme

```bash
# Repository'yi klonla
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
cd SENTIENT_CORE

# Release build
cargo build --release --workspace

# Test
cargo test -p sentient_core

# Belirli crate derle
cargo build --release -p sentient_voice

# Clippy
cargo clippy --workspace
```

---

## 📁 Proje Yapısı

```
SENTIENT_CORE/
├── crates/             # 93 Rust crate
│   ├── sentient_core/  # Ana motor
│   ├── sentient_llm/   # LLM hub (42+ provider)
│   ├── sentient_voice/ # Ses modülü
│   ├── sentient_channels/ # 24 kanal
│   └── ...
├── integrations/       # 72 entegre proje
│   ├── agents/         # AutoGPT, CrewAI, etc.
│   ├── framework/      # LangChain, LlamaIndex, etc.
│   ├── tools/          # Firecrawl, Mem0, etc.
│   └── cevahir_ai/     # Türkçe LLM engine
├── data/               # Veritabanları + 5.587 skill YAML
├── dashboard/          # Web dashboard
├── deploy/             # Production deployment
├── docker/             # Docker yapılandırması
├── docs/               # Dokümantasyon
├── scripts/            # Yardımcı scriptler
├── skills/             # Rust skill motoru
├── .env.template       # Yapılandırma şablonu
├── install.sh          # Tek komutla kurulum
├── Cargo.toml          # Workspace yapılandırması
└── docker-compose.yml  # Docker servisleri
```

---

## 📜 Lisans

**GNU AGPL v3.0** — Kullan, değiştir, paylaş. SaaS olarak sunarsan kaynak kodunu paylaşmak zorundasın.

Ticari kullanım için: enterprise@sentient.ai

---

## 🤝 Katkıda Bulunma

1. Fork'la
2. Feature branch oluştur: `git checkout -b feature/yeni-ozellik`
3. Commit'le: `git commit -m 'Yeni özellik eklendi'`
4. Push'la: `git push origin feature/yeni-ozellik`
5. Pull Request aç

---

## 📞 İletişim

| Kanal | Link |
|-------|------|
| GitHub | [github.com/nexsusagent-coder/SENTIENT_CORE](https://github.com/nexsusagent-coder/SENTIENT_CORE) |
| Discord | Yakında |
| Email | hello@sentient.ai |

---

<p align="center">
  <b>SENTIENT OS</b><br>
  <i>The Operating System That Thinks</i><br><br>
  Made with 🦀 by the SENTIENT Team
</p>
