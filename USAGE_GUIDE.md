# 📖 SENTIENT OS — Kullanım Kılavuzu

> Her şeyin nasıl kurulacağı, nasıl kullanılacağı ve senkron çalışacağı

---

## 📋 İçindekiler

1. [Tek Komutla Kurulum](#1-tek-komutla-kurulum)
2. [Detaylı Kurulum](#2-detaylı-kurulum)
3. [LLM Yapılandırması](#3-llm-yapılandırması)
4. [Sesli Asistan (JARVIS Modu)](#4-sesli-asistan-jarvis-modu)
5. [Kanal Entegrasyonları](#5-kanal-entegrasyonları)
6. [Senkron Çalışma](#6-senkron-çalışma)
7. [Smart Home](#7-smart-home)
8. [Daily Digest](#8-daily-digest)
9. [Agent & Swarm](#9-agent--swarm)
10. [Skill Sistemi](#10-skill-sistemi)
11. [API & Web Dashboard](#11-api--web-dashboard)
12. [Enterprise Özellikleri](#12-enterprise-özellikleri)
13. [Sorun Giderme](#13-sorun-giderme)

---

## 1. Tek Komutla Kurulum

```bash
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
cd SENTIENT_CORE
chmod +x install.sh
./install.sh
```

Sihirbaz 5 adımda tam kurulum yapar. Bittiğinde `./target/release/sentient chat` ile sohbete başla.

---

## 2. Detaylı Kurulum

### Adım 1: Sistem Bağımlılıkları

```bash
# Ubuntu/Debian
sudo apt update && sudo apt install -y build-essential pkg-config libssl-dev curl git

# Rust kur (yoksa)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# Docker kur (yoksa — servisler için gerekli)
curl -fsSL https://get.docker.com | sh
sudo usermod -aG docker $USER
```

### Adım 2: Projeyi Derle

```bash
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
cd SENTIENT_CORE

# Release mode ile derle (~10 dk)
cargo build --release --workspace

# Binary hazır
ls -lh target/release/sentient
```

### Adım 3: Yapılandırma

```bash
# Şablonu kopyala
cp .env.template .env

# Düzenle
nano .env
```

### Adım 4: Docker Servisleri

```bash
# Tüm servisleri başlat
docker-compose up -d

# Kontrol et
docker-compose ps
```

### Adım 5: İlk Çalıştırma

```bash
# Yapılandırma sihirbazı
./target/release/sentient init

# Sistem kontrolü
./target/release/sentient doctor

# Başla!
./target/release/sentient chat
```

---

## 3. LLM Yapılandırması

### Seçenek A: Lokal (Ücretsiz)

Hiçbir API key gerekmez. Ollama bilgisayarında çalışır.

```bash
# Ollama kur
curl -fsSL https://ollama.com/install.sh | sh

# Model indir (seç birini)
ollama pull gemma3:27b       # Önerilen — 16GB RAM
ollama pull llama3.2:3b      # Hafif — 4GB RAM
ollama pull deepseek-r1:7b   # Reasoning — 8GB RAM
ollama pull qwen2.5:7b       # Türkçe iyi — 8GB RAM

# .env'e yaz
echo "LLM_PROVIDER=ollama" >> .env
echo "OLLAMA_HOST=http://localhost:11434" >> .env
```

### Seçenek B: API (Ücretli)

```bash
# OpenRouter (önerilen — tek key ile 200+ model)
# https://openrouter.ai/keys → Key al
echo "OPENROUTER_API_KEY=sk-or-v1-..." >> .env

# Veya doğrudan provider
echo "OPENAI_API_KEY=sk-..." >> .env
echo "ANTHROPIC_API_KEY=sk-ant-..." >> .env
echo "GOOGLE_AI_API_KEY=..." >> .env
echo "DEEPSEEK_API_KEY=..." >> .env
echo "GROQ_API_KEY=..." >> .env
```

### Seçenek C: Hibrit

Normalde lokal kullanır, zor sorularda API'ye başvurur:

```bash
echo "LLM_MODE=hybrid" >> .env
echo "LLM_LOCAL_MODEL=gemma3:27b" >> .env
echo "LLM_API_MODEL=openai/gpt-4o" >> .env
echo "OPENROUTER_API_KEY=sk-or-v1-..." >> .env
```

### Model Değiştirme

```bash
# Varsayılan model değiştir
sentient model set gemma3:27b

# Model listele
sentient model list

# Model test et
sentient model test
```

---

## 4. Sesli Asistan (JARVIS Modu)

Bu, SENTIENT OS'un en güçlü özelliğidir. Tıpkı JARVIS gibi konuşarak her şeyi yaptırabilirsin.

### Nasıl Çalışır

```
┌──────────┐    ┌───────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐
│ Mikrofon  │───▶│  Whisper   │───▶│   LLM    │───▶│  Piper   │───▶│Hoparlör  │
│ (Wake     │    │  STT      │    │ (Brain)  │    │  TTS     │    │ (Cevap)  │
│  Word)    │    │ (Dinle)   │    │          │    │ (Konuş)  │    │          │
└──────────┘    └───────────┘    └──────────┘    └──────────┘    └──────────┘
```

### Lokal Voice Kurulumu (Ücretsiz)

```bash
# 1. Sistem bağımlılıkları
sudo apt install -y portaudio19-dev libasound2-dev ffmpeg

# 2. Whisper.cpp (STT — Konuşmadan metne)
git clone https://github.com/ggerganov/whisper.cpp
cd whisper.cpp
make
bash ./models/download-ggml-model.sh medium
cd ..

# 3. Piper TTS (Metinden konuşmaya)
wget https://github.com/rhasspy/piper/releases/download/v1.2.0/piper_1.2.0_amd64.tar.gz
tar -xzf piper_1.2.0_amd64.tar.gz
sudo mv piper/piper /usr/local/bin/

# Türkçe Piper modeli
mkdir -p ~/.local/share/piper/models
cd ~/.local/share/piper/models
wget https://huggingface.co/rhasspy/piper-voices/resolve/main/tr/tr_TR/medium/tr_TR-medium.onnx
wget https://huggingface.co/rhasspy/piper-voices/resolve/main/tr/tr_TR/medium/tr_TR-medium.onnx.json
cd -

# 4. .env yapılandırması
echo "VOICE_ENABLED=true" >> .env
echo "VOICE_STT=whisper_cpp" >> .env
echo "VOICE_TTS=piper" >> .env
echo "WHISPER_MODEL=medium" >> .env
```

### API Voice Kurulumu (Ücretli)

```bash
echo "VOICE_STT=openai_whisper" >> .env
echo "VOICE_TTS=elevenlabs" >> .env
echo "OPENAI_API_KEY=sk-..." >> .env       # Whisper için
echo "ELEVENLABS_API_KEY=..." >> .env       # TTS için
```

### Voice Komutları

```bash
# Temel sesli sohbet
sentient voice

# Uyandırma kelimesi ile ("Hey Sentient" de, dinlemeye başlar)
sentient voice --wake-word "hey sentient"

# Türkçe mod
sentient voice --language tr

# Kesintisiz dinleme (uyandırma kelimesi gerekmez)
sentient voice --continuous
```

### Sesle Ne Yapabilirsin?

| Komut | Örnek |
|-------|-------|
| Soru sor | "Rust'ta ownership nedir?" |
| Kod yazdır | "Bir REST API yaz" |
| Email gönder | "Ahmet'e toplantı hatırlatma maili gönder" |
| Takvim ekle | "Yarın 14:00'e toplantı ekle" |
| Işık kontrolü | "Salon ışığını aç" |
| Isı ayarla | "Koridor ısısı 22 derece yap" |
| Arama yap | "Quantum computing hakkında araştırma yap" |
| Hatırlatıcı kur | "15 dakika sonra beni uyar" |
| Özet al | "Bugünkü mail özetini ver" |
| Çeviri yap | "Bu metni İngilizceye çevir" |

---

## 5. Kanal Entegrasyonları

SENTIENT aynı anda 24 platformda çalışabilir. Her platforma aynı yapay zeka motoru hizmet eder.

### Telegram Bot

```bash
# 1. Bot oluştur: https://t.me/BotFather → /newbot
# 2. Token al
echo "TELEGRAM_BOT_TOKEN=123456:ABC-DEF..." >> .env

# 3. Başlat
./target/release/sentient gateway
```

### Discord Bot

```bash
# 1. Uygulama oluştur: https://discord.com/developers/applications
# 2. Bot token al
echo "DISCORD_BOT_TOKEN=MTIz..." >> .env
echo "DISCORD_APPLICATION_ID=123456789" >> .env

# 3. Başlat
./target/release/sentient gateway
```

### Slack Bot

```bash
# 1. App oluştur: https://api.slack.com/apps
# 2. Token al
echo "SLACK_BOT_TOKEN=xoxb-..." >> .env
echo "SLACK_APP_TOKEN=xapp-..." >> .env

# 3. Başlat
./target/release/sentient gateway
```

### Email

```bash
# Gmail ile (App Password gerekli)
echo "SMTP_HOST=smtp.gmail.com" >> .env
echo "SMTP_PORT=587" >> .env
echo "SMTP_USER=your@gmail.com" >> .env
echo "SMTP_PASSWORD=your-app-password" >> .env
```

---

## 6. Senkron Çalışma

Tüm modüller birbiriyle haberleşir. Bir şey söylediğinde hepsi aynı anda tepki verir.

### Mimarisi

```
                    ┌─────────────┐
                    │  Event Bus  │
                    │ (Graph)     │
                    └──────┬──────┘
           ┌───────────────┼───────────────┐
           │               │               │
    ┌──────▼──────┐ ┌──────▼──────┐ ┌──────▼──────┐
    │   Voice     │ │   Email     │ │  Calendar   │
    │  (Dinle)   │ │  (Oku)     │ │  (Planla)  │
    └──────┬──────┘ └──────┬──────┘ └──────┬──────┘
           │               │               │
           └───────────────┼───────────────┘
                    ┌──────▼──────┐
                    │  Orchestrator│
                    │  (Karar)    │
                    └──────┬──────┘
           ┌───────────────┼───────────────┐
           │               │               │
    ┌──────▼──────┐ ┌──────▼──────┐ ┌──────▼──────┐
    │  Channels   │ │ Smart Home  │ │   Skills    │
    │ (Gönder)   │ │ (Kontrol)   │ │ (Çalıştır)  │
    └─────────────┘ └─────────────┘ └─────────────┘
```

### Örnek Senkron Senaryosu

1. **Sesle** "Hey Sentient, yarınki toplantıya hazırlık yap" dersin
2. **Calendar** → yarınki toplantıyı bulur
3. **Email** → toplantıyla ilgili emailleri arar
4. **Research** → konu hakkında araştırma yapar
5. **Digest** → sabah bültenine ekler
6. **Channels** → Telegram/Discord'dan hatırlatma gönderir
7. **Voice** → "Toplantı hazırlığı tamam, sana özet sunuyorum" der

### 7/24 Arka Plan Servisi

```bash
# Sentinel modu — her zaman hazır
sentient serve

# Bu modda:
# - Email'ler otomatik okunur ve özetlenir
# - Takvim etkinlikleri izlenir
# - Kanal mesajlarına otomatik cevap verilir
# - Smart home cihazları izlenir
# - Sabah bülteni oluşturulur
# - Wake word dinlenir
```

---

## 7. Smart Home

Türkçe doğal dil ile evinizi kontrol edin:

```bash
# .env yapılandırması
echo "HOME_ASSISTANT_URL=http://homeassistant.local:8123" >> .env
echo "HOME_ASSISTANT_TOKEN=your-long-lived-token" >> .env
```

### Desteklenen Komutlar

| Sesli Komut | Eylem |
|-------------|-------|
| "Salon ışığını aç" | `light.living_room` → ON |
| "Yatak odası lambasını kapat" | `light.bedroom` → OFF |
| "Koridor ısısı 22 derece" | `climate.hallway` → 22°C |
| "Kilitleri kilitle" | `lock.*` → LOCK |
| "Perdeleri aç" | `cover.*` → OPEN |

### Desteklenen Oda İsimleri

Türkçe → Home Assistant mapping:

| Türkçe | İngilizce (HA) |
|--------|----------------|
| Salon / Oturma | living_room |
| Yatak odası | bedroom |
| Mutfak | kitchen |
| Banyo | bathroom |
| Ofis | office |
| Koridor | hallway |

---

## 8. Daily Digest

Her sabah otomatik bülten:

```bash
# Bülten oluştur
sentient digest

# Veya serviste otomatik (sentient serve)
```

Bültende:
- 📧 Önemli emaillerin özeti
- 📅 Bugünkü toplantılar
- 🌅 Hava durumu
- 📰 Önemli haberler
- ✅ Yapılacaklar listesi

---

## 9. Agent & Swarm

### Tek Agent

```bash
# Agent oluştur
sentient agent create coder --model gpt-4o

# Goal ver
sentient agent run coder --goal "REST API yaz: /users endpoint"

# Durum kontrolü
sentient agent list
```

### Swarm (Çoklu Ajan)

```bash
# 5 agent'lı swarm oluştur
sentient swarm create team --agents 5 --strategy collective

# Karmaşık görev ver
sentient swarm run team --goal "E-ticaret sitesi yap: frontend, backend, database, tests, deployment"

# Durum
sentient swarm status team
```

---

## 10. Skill Sistemi

### 5.587+ Hazır Skill

```bash
# Tüm skill'leri listele
sentient skill list

# Kategoriye göre
sentient skill list --category code
sentient skill list --category security

# Ara
sentient skill search "web scraping"

# Çalıştır
sentient skill run code-review --path ./src --language rust
sentient skill run web-scraper --url "https://example.com"
sentient skill run pentest --target "https://myapp.com"
```

### Özel Skill Oluştur

```yaml
# skills/my-skill/SKILL.yaml
name: my-custom-skill
version: 1.0.0
description: Özel skill
author: me@example.com

inputs:
  - name: input_file
    type: string
    required: true

steps:
  - name: analyze
    action: code.analyze
    inputs:
      file: ${{ inputs.input_file }}
```

```bash
# Skill yükle
sentient skill load ./skills/my-skill

# Test et
sentient skill test my-custom-skill
```

---

## 11. API & Web Dashboard

### REST API

```bash
# Gateway başlat
sentient gateway

# Status
curl http://localhost:8080/api/v1/status

# Sohbet
curl -X POST http://localhost:8080/api/v1/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Merhaba", "model": "gpt-4o"}'

# Skill çalıştır
curl -X POST http://localhost:8080/api/v1/skills/run \
  -H "Content-Type: application/json" \
  -d '{"skill": "code-review", "params": {"path": "./src"}}'
```

### Web Dashboard

```bash
sentient dashboard --port 8080
# http://localhost:8080/dashboard
```

Dashboard özellikleri:
- Real-time sistem metrikleri
- Agent yönetimi
- Skill kartları
- Bellek görüntüleme
- Canlı loglar
- Entegre terminal

---

## 12. Enterprise Özellikleri

### SSO (Single Sign-On)

```bash
# Okta
echo "SSO_PROVIDER=okta" >> .env
echo "OKTA_DOMAIN=your-domain.okta.com" >> .env
echo "OKTA_CLIENT_ID=..." >> .env
echo "OKTA_CLIENT_SECRET=..." >> .env

# Auth0
echo "SSO_PROVIDER=auth0" >> .env
echo "AUTH0_DOMAIN=your-tenant.auth0.com" >> .env

# Azure AD
echo "SSO_PROVIDER=azure_ad" >> .env
echo "AZURE_TENANT_ID=..." >> .env
```

### RBAC

```bash
# Rol ata
sentient enterprise role assign admin --user user@example.com

# İzin kontrolü
sentient enterprise permission check admin --resource api/v1/agents
```

### Multi-Tenant

```bash
# Tenant oluştur
sentient enterprise tenant create acme-corp --quota 10000

# Tenant durumu
sentient enterprise tenant status acme-corp
```

---

## 13. Sorun Giderme

### sentient doctor

```bash
./target/release/sentient doctor
```

Tüm bileşenleri kontrol eder: config, LLM bağlantısı, bellek, dil ayarları.

### Yaygın Sorunlar

| Sorun | Çözüm |
|-------|-------|
| "LLM bağlantısı YOK" | `.env` → API key ekle veya Ollama başlat (`ollama serve`) |
| "Ollama ulaşılamiyor" | `ollama serve &` ile servisi başlat |
| "Bellek sistemi YOK" | İlk çalıştırmada otomatik oluşur |
| "Voice çalışmıyor" | `sudo apt install portaudio19-dev libasound2-dev ffmpeg` |
| "Docker hata" | `docker-compose down && docker-compose up -d` |
| "Build hatası" | `cargo clean && cargo build --release` |
| "Disk dolu" | `cargo clean` (target/ temizlenir, kod silinmez) |

### Loglar

```bash
# Canlı log
sentient logs --follow --level debug

# Docker logları
docker-compose logs -f
```

### Build Profili

```bash
# Normal build (küçük, hızlı)
cargo build --release

# Tam debug build (gerekirse)
cargo build --profile dev-full

# Test (release benzeri, küçük)
cargo test -p sentient_core
```

---

## 🔗 Hızlı Referans

| Ne yapmak istiyorsun? | Komut |
|-----------------------|-------|
| Sohbet | `sentient chat` |
| Sesli asistan | `sentient voice` |
| Tek soru | `sentient ask "soru"` |
| 7/24 servisi | `sentient serve` |
| Dashboard | `sentient dashboard` |
| Kanal botu | `sentient gateway` |
| Sistem kontrolü | `sentient doctor` |
| Sabah bülteni | `sentient digest` |
| Agent çalıştır | `sentient agent run <name>` |
| Skill çalıştır | `sentient skill run <skill>` |
| Bellek ara | `sentient memory search "sorgu"` |
| Model değiştir | `sentient model set <model>` |

---

*SENTIENT OS — The Operating System That Thinks*
