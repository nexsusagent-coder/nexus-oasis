# 📖 SENTIENT OS KULLANIM KILAVUZU

> **Detaylı kullanım rehberi - Başlangıçtan ileri seviyeye**

---

## 📑 İçindekiler

1. [Kurulum](#1-kurulum)
2. [İlk Çalıştırma](#2-ilk-çalıştırma)
3. [LLM Yapılandırması](#3-llm-yapılandırması)
4. [CLI Komutları](#4-cli-komutları)
5. [Sesli Asistan (Voice)](#5-sesli-asistan-voice)
6. [Kanal Entegrasyonları](#6-kanal-entegrasyonları)
7. [Skill Sistemi](#7-skill-sistemi)
8. [Desktop Automation](#8-desktop-automation)
9. [API Gateway](#9-api-gateway)
10. [Dashboard](#10-dashboard)
11. [Enterprise Özellikler](#11-enterprise-özellikler)
12. [Geliştirici Araçları](#12-geliştirici-araçları)
13. [Sorun Giderme](#13-sorun-giderme)

---

## 1. Kurulum

### 1.1 Otomatik Kurulum

```bash
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
cd SENTIENT_CORE
./install.sh
```

Kurulum adımları:
1. **Sistem kontrolü** → Rust, bağımlılıklar
2. **LLM seçimi** → API Key / Lokal / Atla
3. **Ek özellikler** → Voice, Dashboard
4. **Derleme** → Otomatik

### 1.2 Hızlı Kurulum

```bash
./quick-start.sh
```

Soru sormadan minimal kurulum. Ollama yoksa kurmaz.

### 1.3 Manuel Kurulum

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# Derle
cargo build --release --bin sentient

# Config
cp .env.template .env
# .env düzenle

# Çalıştır
./target/release/sentient chat
```

### 1.4 Docker ile Kurulum

```bash
# Tüm servisler
docker-compose up -d

# Servisleri gör
docker-compose ps

# Durdur
docker-compose down
```

**Docker Servisleri:**

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

## 2. İlk Çalıştırma

### 2.1 Sistem Kontrolü

```bash
./target/release/sentient doctor
```

Çıktı:
```
🧠 SENTIENT OS Doctor v4.0.0

✓ Rust: 1.75.0
✓ Binary: target/release/sentient
✓ Config: .env
✓ Database: data/sentient_memory.db
✓ LLM Provider: ollama (gemma3:27b)
✓ Voice: enabled (whisper_cpp + piper)

Sistem hazır!
```

### 2.2 Sohbet Başlatma

```bash
./target/release/sentient chat
```

```
🧠 SENTIENT OS v4.0.0
Provider: ollama | Model: gemma3:27b
Language: tr

Sen: Merhaba, kendini tanıtır mısın?
Asistan: Merhaba! Ben SENTIENT OS, düşünen işletim sistemi...

Sen: /help
Komutlar:
  /clear    - Sohbeti temizle
  /model    - Model değiştir
  /save     - Sohbeti kaydet
  /exit     - Çık

Sen: /exit
Güle güle!
```

### 2.3 Tek Soru Sorma

```bash
./target/release/sentient ask "Rust'ta ownership nedir?"
```

---

## 3. LLM Yapılandırması

### 3.1 API Key ile Kullanım

#### OpenRouter (Önerilen)

```bash
# .env dosyasına
OPENROUTER_API_KEY=sk-or-v1-xxxxx
LLM_PROVIDER=openrouter
LLM_MODEL=openai/gpt-4o-mini
```

```bash
# Çalıştır
OPENROUTER_API_KEY=sk-or-v1-xxxxx ./target/release/sentient chat
```

**Model Seçenekleri:**
- `openai/gpt-4o-mini` - Hızlı, ucuz
- `openai/gpt-4o` - Dengeli
- `anthropic/claude-3.5-sonnet` - En iyi
- `google/gemini-2.0-flash-exp` - Hızlı
- `meta-llama/llama-3.3-70b-instruct` - Açık kaynak

#### OpenAI

```bash
OPENAI_API_KEY=sk-proj-xxxxx ./target/release/sentient chat
```

#### Anthropic

```bash
ANTHROPIC_API_KEY=sk-ant-xxxxx ./target/release/sentient chat
```

#### Google AI

```bash
GOOGLE_AI_API_KEY=xxxxx ./target/release/sentient chat
```

#### DeepSeek (En Ucuz)

```bash
DEEPSEEK_API_KEY=xxxxx ./target/release/sentient chat
```

#### Groq (En Hızlı)

```bash
GROQ_API_KEY=xxxxx ./target/release/sentient chat
```

### 3.2 Lokal LLM (Ollama)

```bash
# Ollama kur
curl -fsSL https://ollama.com/install.sh | sh

# Servis başlat
ollama serve &

# Model indir
ollama pull gemma3:27b       # Önerilen (16GB RAM)
ollama pull llama3.2:3b      # Hafif (4GB RAM)
ollama pull deepseek-r1:7b   # Reasoning (8GB RAM)
ollama pull qwen2.5:7b       # Türkçe iyi
ollama pull mistral:7b       # Dengeli

# .env
LLM_PROVIDER=ollama
OLLAMA_HOST=http://localhost:11434
OLLAMA_MODEL=gemma3:27b

# Çalıştır
./target/release/sentient chat
```

### 3.3 Hibrit Mod

```bash
# .env
LLM_MODE=hybrid
LLM_LOCAL_MODEL=gemma3:27b
LLM_API_MODEL=openai/gpt-4o
LLM_FALLBACK_THRESHOLD=0.7  # Zorluk eşiği
```

### 3.4 Tüm Provider'lar (42 Adet)

| Provider | Tür | Ücretsiz | Özellik |
|----------|-----|----------|---------|
| OpenAI | Direct | ❌ | GPT-4o, o1, o3 |
| Anthropic | Direct | ❌ | Claude 4, 3.5 |
| Google | Direct | Gemini Flash | Gemini 2.0 |
| Mistral | Direct | ❌ | Mistral Large |
| DeepSeek | Direct | ❌ | **EN UCUZ** |
| Groq | Direct | ❌ | **EN HIZLI** |
| xAI | Direct | ❌ | Grok 2 |
| Cohere | Direct | ❌ | Command R+ |
| Perplexity | Direct | ❌ | Sonar (web search) |
| Together | Aggregator | ❌ | 100+ açık kaynak |
| Fireworks | Direct | ❌ | Hızlı inference |
| Replicate | Direct | ❌ | Cloud run |
| Ollama | Local | ✅ | **ÜCRETSİZ** |
| OpenRouter | Aggregator | ❌ | **200+ model** |
| LiteLLM | Aggregator | ❌ | **100+ provider** |
| Hugging Face | Aggregator | Bazıları | **200K+ model** |
| Azure OpenAI | Cloud | ❌ | Enterprise |
| AWS Bedrock | Cloud | ❌ | Claude, Llama |
| Vertex AI | Cloud | ❌ | Gemini, Claude |
| vLLM | Local | ✅ | High perf |
| LM Studio | Local | ✅ | GUI |
| + 22 provider daha | | | |

---

## 4. CLI Komutları

### 4.1 Temel Komutlar

```bash
# Sohbet
sentient chat

# Tek soru
sentient ask "Soru?"

# Sistem durumu
sentient status

# Sorun giderme
sentient doctor

# Versiyon
sentient --version

# Yardım
sentient --help
```

### 4.2 Model Komutları

```bash
# Model listesi
sentient model list

# Model değiştir
sentient model set gemma3:27b

# Model bilgisi
sentient model info
```

### 4.3 Bellek Komutları

```bash
# Bellek listesi
sentient memory list

# Bellek ara
sentient memory search "geçen hafta"

# Bellek temizle
sentient memory clear

# Bellek dışa aktar
sentient memory export memories.json
```

### 4.4 Skill Komutları

```bash
# Skill listesi
sentient skill list

# Skill ara
sentient skill search "browser"

# Skill çalıştır
sentient skill run code-review --path ./src

# Skill bilgisi
sentient skill info web-scraper
```

### 4.5 Agent Komutları

```bash
# Agent listesi
sentient agent list

# Agent oluştur
sentient agent create coder --model gpt-4o

# Agent çalıştır
sentient agent run coder --goal "API yaz"

# Swarm oluştur
sentient swarm create team --agents 5

# Swarm çalıştır
sentient swarm run team --goal "Proje analiz et"
```

### 4.6 Kanal Komutları

```bash
# Kanal listesi
sentient channel list

# Kanal başlat
sentient channel start telegram

# Kanal durdur
sentient channel stop telegram

# Kanal durumu
sentient channel status
```

---

## 5. Sesli Asistan (Voice)

### 5.1 Nasıl Çalışır

```
🎤 Mikrofon → Whisper STT → LLM → TTS → 🔊 Hoparlör
     ↑                                    |
     └──── Wake Word ("Hey Sentient") ────┘
```

### 5.2 Lokal Voice (Ücretsiz)

```bash
# Whisper.cpp kur
git clone https://github.com/ggerganov/whisper.cpp
cd whisper.cpp && make
bash ./models/download-ggml-model.sh medium

# Piper TTS kur
wget https://github.com/rhasspy/piper/releases/download/v1.2.0/piper_1.2.0_amd64.tar.gz
tar -xzf piper_1.2.0_amd64.tar.gz

# Türkçe model
cd ~/.local/share/piper/models
wget https://huggingface.co/rhasspy/piper-voices/resolve/main/tr/tr_TR/medium/tr_TR-medium.onnx
wget https://huggingface.co/rhasspy/piper-voices/resolve/main/tr/tr_TR/medium/tr_TR-medium.onnx.json

# .env
VOICE_ENABLED=true
VOICE_STT=whisper_cpp
VOICE_TTS=piper
WHISPER_MODEL=medium
```

### 5.3 API Voice (Ücretli)

```bash
# .env
VOICE_ENABLED=true
VOICE_STT=openai_whisper
VOICE_TTS=elevenlabs
OPENAI_API_KEY=sk-...
ELEVENLABS_API_KEY=...
```

### 5.4 Voice Komutları

```bash
# Temel sesli sohbet
sentient voice

# Uyandırma kelimesi ile
sentient voice --wake-word "hey sentient"

# Türkçe
sentient voice --language tr

# Kesintisiz dinleme
sentient voice --continuous

# Belirli süre dinle
sentient voice --timeout 30
```

### 5.5 Wake Word

```bash
# Porcupine (önerilen)
sentient voice --wake-word "hey sentient" --wake-engine porcupine

# Vosk
sentient voice --wake-word "sentient" --wake-engine vosk

# Whisper (daha yavaş ama daha doğru)
sentient voice --wake-word "hey sentient" --wake-engine whisper
```

---

## 6. Kanal Entegrasyonları

### 6.1 Telegram

```bash
# Bot oluştur: @BotFather
TELEGRAM_BOT_TOKEN=123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11

# Başlat
TELEGRAM_BOT_TOKEN=xxx ./target/release/sentient channel start telegram
```

### 6.2 Discord

```bash
# Bot oluştur: https://discord.com/developers/applications
DISCORD_BOT_TOKEN=xxx
DISCORD_APPLICATION_ID=xxx

# Başlat
DISCORD_BOT_TOKEN=xxx ./target/release/sentient channel start discord
```

### 6.3 WhatsApp

```bash
# Meta Business API
WHATSAPP_TOKEN=xxx
WHATSAPP_PHONE_ID=xxx

# Başlat
WHATSAPP_TOKEN=xxx ./target/release/sentient channel start whatsapp
```

### 6.4 Slack

```bash
# https://api.slack.com/apps
SLACK_BOT_TOKEN=xoxb-xxx
SLACK_APP_TOKEN=xapp-xxx

# Başlat
SLACK_BOT_TOKEN=xxx ./target/release/sentient channel start slack
```

### 6.5 Email

```bash
# .env
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USER=your@gmail.com
SMTP_PASSWORD=app-password

# IMAP (gelen)
IMAP_HOST=imap.gmail.com
IMAP_PORT=993
IMAP_USER=your@gmail.com
IMAP_PASSWORD=app-password
```

### 6.6 Tüm Kanallar (24)

| Kanal | Tür | Durum |
|-------|-----|-------|
| Telegram | Mesajlaşma | ✅ |
| Discord | Mesajlaşma | ✅ |
| WhatsApp | Mesajlaşma | ✅ |
| Slack | İş | ✅ |
| Email | İletişim | ✅ |
| Microsoft Teams | İş | 🔄 |
| Signal | Güvenli | 🔄 |
| Matrix | Açık kaynak | 🔄 |
| iMessage | Apple | 🔄 |
| Instagram | Sosyal | 🔄 |
| LinkedIn | Profesyonel | 🔄 |
| Twitter/X | Sosyal | 🔄 |
| Zoom | Video | 🔄 |
| Webex | Video | 🔄 |
| Google Chat | İş | 🔄 |
| Mattermost | Açık kaynak | 🔄 |
| Messenger | Sosyal | 🔄 |
| Snapchat | Sosyal | 🔄 |
| Viber | Mesajlaşma | 🔄 |
| WeChat | Mesajlaşma | 🔄 |
| Line | Mesajlaşma | 🔄 |
| Chime | İş | 🔄 |
| + 2 daha | | |

---

## 7. Skill Sistemi

### 7.1 Skill Kategorileri

| Kategori | Skill Sayısı | Açıklama |
|----------|--------------|----------|
| Dev | 2,965+ | Coding, IDE, DevOps |
| OSINT | 1,050+ | Search, Browser, Data |
| Social | 238+ | Communication, Marketing |
| Automation | 306+ | Productivity, Smart Home |
| Media | 246+ | Image/Video, Speech |
| Productivity | 214+ | Notes, PDF |
| Security | 52+ | Security tools |
| Mobile | 233+ | Transportation, Health |
| Gaming | 108+ | Gaming, Personal Dev |

### 7.2 Skill Kullanımı

```bash
# Skill ara
sentient skill search "web scraper"

# Skill listele
sentient skill list --category Dev

# Skill çalıştır
sentient skill run code-review --path ./src

# Skill detay
sentient skill info browser-navigate
```

### 7.3 Popüler Skill'ler

**Dev:**
- `code-review` - Kod inceleme
- `debug-helper` - Hata ayıklama
- `git-workflow` - Git işlemleri
- `web-researcher` - Web araştırma

**OSINT:**
- `browser-automation` - Tarayıcı otomasyonu
- `web-scraper` - Web kazıma
- `search-engine` - Arama motoru

**Automation:**
- `task-automation` - Görev otomasyonu
- `calendar-sync` - Takvim senkronizasyonu
- `smart-home` - Akıllı ev kontrolü

**Media:**
- `image-generation` - Görsel üretimi
- `video-generation` - Video üretimi
- `speech-to-text` - Sesten metne

### 7.4 Skill Kaynakları

| Kaynak | Skill Sayısı |
|--------|--------------|
| OpenClaw Skills | 5,143 |
| Everything Claude Code | 181 |
| DeerFlow Skills | 100+ |
| Gstack | 37 |

---

## 8. Desktop Automation

### 8.1 Özellikler

| Özellik | Açıklama |
|---------|----------|
| Fare Kontrolü | İnsan gibi hareket (Bezier eğrileri) |
| Klavye Kontrolü | Doğal yazma dinamiği |
| Ekran Okuma | OCR, görüntü analizi |
| GUI Kontrol | Tüm uygulamalar |
| Safety | 50+ tehlikeli komut engeli |

### 8.2 Kullanım

```bash
# Desktop agent başlat
sentient desktop

# Belirli görev
sentient desktop --goal "Firefox'ta YouTube'a git"

# Güvenli mod
sentient desktop --safe-mode

# Sovereign policy aktif
sentient desktop --sovereign
```

### 8.3 İzin Verilen/Uzak Durulan Komutlar

**İzin Verilen:**
- `libreoffice`, `firefox`, `vscode`
- `gnome-terminal`, `nautilus`
- `git`, `cargo`, `npm`

**Yasaklı (50+ komut):**
- `rm -rf`, `format`, `dd`
- `chmod 777`, `curl | bash`
- `sudo`, `su`, `chown`

### 8.4 Browser Automation

```bash
# Browser agent
sentient browser

# URL'ye git
sentient browser --url "https://github.com"

# Headless mod
sentient browser --headless

# Stealth mod
sentient browser --stealth
```

---

## 9. API Gateway

### 9.1 Başlatma

```bash
sentient gateway
# http://localhost:8080
```

### 9.2 Endpoints

```
POST /chat              - Sohbet
POST /ask               - Tek soru
GET  /status            - Sistem durumu
GET  /models            - Model listesi
GET  /skills            - Skill listesi
POST /skills/:id/run    - Skill çalıştır
GET  /memory            - Bellek
GET  /health            - Sağlık kontrolü
WS   /ws                - WebSocket
```

### 9.3 Örnek İstekler

```bash
# Chat
curl -X POST http://localhost:8080/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Merhaba"}'

# Ask
curl -X POST http://localhost:8080/ask \
  -H "Content-Type: application/json" \
  -d '{"question": "Rust nedir?"}'

# Status
curl http://localhost:8080/status

# Models
curl http://localhost:8080/models
```

### 9.4 WebSocket

```javascript
const ws = new WebSocket('ws://localhost:8080/ws');

ws.onopen = () => {
  ws.send(JSON.stringify({type: 'chat', message: 'Merhaba'}));
};

ws.onmessage = (event) => {
  console.log(JSON.parse(event.data));
};
```

---

## 10. Dashboard

### 10.1 Başlatma

```bash
sentient dashboard
# http://localhost:8080/dashboard
```

### 10.2 Özellikler

| Panel | Açıklama |
|-------|----------|
| Skills Hub | Yüklenmiş skill'ler |
| Tool Monitor | Aktif araçlar |
| V-GATE Panel | API bağlantı durumu |
| Memory Viz | Bellek görselleştirme |
| Chat | İnteraktif sohbet |

---

## 11. Enterprise Özellikler

### 11.1 RBAC (Rol Bazlı Erişim)

```bash
# Rol oluştur
sentient role create admin --permissions "all"

# Kullanıcı rol ata
sentient user assign admin user@example.com

# İzin kontrol
sentient auth check user@example.com "skill:run:code-review"
```

### 11.2 SSO (Single Sign-On)

```bash
# .env
SSO_PROVIDER=okta  # veya auth0, azure
SSO_CLIENT_ID=xxx
SSO_CLIENT_SECRET=xxx
SSO_DOMAIN=xxx.okta.com
```

### 11.3 Audit Logging

```bash
# Audit logları gör
sentient audit list

# Belirli kullanıcı
sentient audit list --user admin@example.com

# Belirli tarih
sentient audit list --from 2024-01-01 --to 2024-01-31
```

### 11.4 Multi-Tenant

```bash
# Tenant oluştur
sentient tenant create company-a

# Tenant yapılandır
sentient tenant config company-a --llm openai --model gpt-4o

# Tenant kullanıcı ekle
sentient tenant user-add company-a user@company-a.com
```

---

## 12. Geliştirici Araçları

### 12.1 Test

```bash
# Tüm testler
cargo test --workspace

# Belirli crate
cargo test -p sentient_core

# Coverage
cargo tarpaulin --workspace --out Html
```

### 12.2 Benchmark

```bash
# Benchmark çalıştır
cargo bench

# Sonuçlar
cat benchmarks/results/latest.json
```

### 12.3 Debug

```bash
# Debug mod
RUST_LOG=debug ./target/release/sentient chat

# Trace mod
RUST_LOG=trace ./target/release/sentient chat

# Belirli modül
RUST_LOG=sentient_llm=debug ./target/release/sentient chat
```

### 12.4 Profil

```bash
# CPU profil
cargo flamegraph --bin sentient -- chat

# Memory profil
cargo valgrind --bin sentient -- chat
```

---

## 13. Sorun Giderme

### 13.1 Yaygın Sorunlar

#### Ollama Bağlantı Hatası

```bash
# Ollama çalışıyor mu?
curl http://localhost:11434/api/tags

# Başlat
ollama serve &

# Model var mı?
ollama list
```

#### API Key Hatası

```bash
# Key doğru mu?
echo $OPENAI_API_KEY

# .env yüklendi mi?
source .env 2>/dev/null || true

# Doğrudan kullan
OPENAI_API_KEY=sk-xxx ./target/release/sentient chat
```

#### Derleme Hatası

```bash
# Temizle
cargo clean

# Bağımlılıkları güncelle
cargo update

# Tekrar derle
cargo build --release
```

#### Memory Hatası

```bash
# DB kontrol
ls -la data/sentient_memory.db

# Yeniden oluştur
rm data/sentient_memory.db
./target/release/sentient init
```

### 13.2 Loglar

```bash
# Log dosyası
tail -f data/logs/sentient.log

# Sistem logları
journalctl -u sentient -f
```

### 13.3 Destek

| Kanal | Link |
|-------|------|
| GitHub Issues | [github.com/nexsusagent-coder/SENTIENT_CORE/issues](https://github.com/nexsusagent-coder/SENTIENT_CORE/issues) |
| Email | sentient@sentient-os.ai |

---

## 📚 Ek Kaynaklar

- [ARCHITECTURE.md](ARCHITECTURE.md) - Sistem mimarisi
- [MODEL_PROVIDERS.md](MODEL_PROVIDERS.md) - LLM provider detayları
- [SECURITY.md](SECURITY.md) - Güvenlik dokümantasyonu
- [DEPLOYMENT.md](DEPLOYMENT.md) - Production deployment
- [docs/API.md](docs/API.md) - REST API dokümantasyonu
- [SISTEM_DOKUMANTASYONU.md](SISTEM_DOKUMANTASYONU.md) - Tam sistem dokümantasyonu

---

<p align="center">
  <b>SENTIENT OS</b> - The Operating System That Thinks<br>
  Made with 🦀 by the SENTIENT Team
</p>
