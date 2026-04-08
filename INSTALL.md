# 📦 SENTIENT OS - KAPSAMLI KURULUM KILAVUZU

**Tam Sistem Kurulum, Yapılandırma ve Entegrasyon Rehberi**

---

## 📋 İÇİNDEKİLER

1. [Sistem Gereksinimleri](#1-sistem-gereksinimleri)
2. [Gemma 4 Kernel Kurulumu](#2-gemma-4-kernel-kurulumu)
3. [Hızlı Kurulum](#3-hızlı-kurulum)
4. [Manuel Kurulum](#4-manuel-kurulum)
5. [Docker Kurulumu](#5-docker-kurulumu)
6. [Rust Crate Kurulumları](#6-rust-crate-kurulumları)
7. [Entegrasyon Kurulumları](#7-entegrasyon-kurulumları)
8. [LLM Provider Kurulumu](#8-llm-provider-kurulumu)
9. [API Anahtarı Yapılandırması](#9-api-anahtarı-yapılandırması)
10. [Telegram Bot Kurulumu](#10-telegram-bot-kurulumu)
11. [Discord Bot Kurulumu](#11-discord-bot-kurulumu)
12. [Slack Entegrasyonu](#12-slack-entegrasyonu)
13. [Email (SMTP) Kurulumu](#13-email-smtp-kurulumu)
14. [GitHub Entegrasyonu](#14-github-entegrasyonu)
15. [V-GATE Proxy Kurulumu](#15-v-gate-proxy-kurulumu)
16. [Dashboard Erişimi](#16-dashboard-erişimi)
17. [Skill ve Tool Yönetimi](#17-skill-ve-tool-yönetimi)
18. [Otonom Mod Yapılandırması](#18-otonom-mod-yapılandırması)
19. [Güvenlik Yapılandırması](#19-güvenlik-yapılandırması)
20. [Sorun Giderme](#20-sorun-giderme)

---

## 1. SİSTEM GEREKSİNİMLERİ

### Minimum Gereksinimler

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  Bileşen        │  Minimum          │  Önerilen         │  Gemma 4 31B     │
├─────────────────┼───────────────────┼───────────────────┼──────────────────┤
│  CPU            │  4 çekirdek       │  8+ çekirdek      │  12+ çekirdek    │
│  RAM            │  8 GB             │  16+ GB           │  32+ GB          │
│  GPU VRAM       │  -                │  8 GB             │  24 GB           │
│  Disk           │  20 GB            │  50+ GB SSD       │  100+ GB SSD     │
│  İşletim Sistemi│  Linux/macOS/Win  │  Ubuntu 22.04+    │  Ubuntu 22.04+   │
│  Rust           │  1.75+            │  1.80+            │  1.80+           │
│  Git            │  2.30+            │  Latest           │  Latest          │
│  Docker         │  20.10+           │  Latest           │  Latest          │
│  Ollama         │  0.1.26+          │  Latest           │  Latest          │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Yazılım Gereksinimleri

| Yazılım | Sürüm | Kurulum Komutu |
|---------|-------|----------------|
| **Rust** | 1.75+ | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| **Git** | 2.30+ | `sudo apt install git -y` |
| **Docker** | 20.10+ | `sudo apt install docker.io -y` |
| **Make** | 4.0+ | `sudo apt install make -y` |
| **Python** | 3.10+ | `sudo apt install python3 python3-pip -y` |
| **SQLite** | 3.35+ | `sudo apt install sqlite3 -y` |
| **Ollama** | 0.1.26+ | `curl -fsSL https://ollama.com/install.sh \| sh` |

### Port Gereksinimleri

| Port | Servis | Açıklama |
|------|--------|----------|
| **8080** | Dashboard | Web arayüzü |
| **8100** | V-GATE | API proxy |
| **1071** | V-GATE Internal | Internal proxy |
| **11434** | Ollama | Gemma 4 Kernel |

---

## 2. GEMMA 4 KERNEL KURULUMU

### Gemma 4 Nedir?

```
╔═══════════════════════════════════════════════════════════════════════════╗
║                        GEMMA 4 - SENTIENT OS KERNEL                       ║
╠═══════════════════════════════════════════════════════════════════════════╣
║                                                                           ║
║  📊 PARAMETRE:        31 Milyar                                          ║
║  📏 CONTEXT LENGTH:   256,000 tokens (256K)                              ║
║  🎨 MULTIMODAL:       Metin + Görüntü                                    ║
║  🧠 THINKING MODE:    Native chain-of-thought                            ║
║  🔧 FUNCTION CALL:    Native tool use                                    ║
║  📜 LİSANS:           Apache 2.0 - TAMAMEN ÜCRETSİZ                      ║
║  🔑 API KEY:          GEREKMİYOR - TAMAMEN YEREL                         ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝
```

### Adım 1: Ollama Kurulumu

```bash
# Linux/macOS
curl -fsSL https://ollama.com/install.sh | sh

# Ollama servisini başlat
ollama serve

# Servis olarak çalıştır (Linux)
sudo systemctl start ollama
sudo systemctl enable ollama

# macOS
brew services start ollama
```

### Adım 2: Gemma 4 Modelini İndir

```bash
# Gemma 4 31B (ÖNERİLEN - SENTIENT Kernel)
ollama pull gemma4:31b

# Gemma 4 12B (Daha az RAM gerektiren versiyon)
ollama pull gemma4:12b

# Gemma 4 4B (Minimum sistemler için)
ollama pull gemma4:4b
```

### Adım 3: Gemma 4 Test

```bash
# Direkt test
ollama run gemma4:31b "Merhaba, SENTIENT OS için bir Rust fonksiyonu yaz"

# API test
curl http://localhost:11434/api/generate -d '{
  "model": "gemma4:31b",
  "prompt": "Explain Rust ownership in 3 sentences"
}'
```

### Adım 4: SENTIENT'ya Gemma 4 Bağlama

```bash
# Provider'ı ayarla
sentient config set llm.provider "gemma4"

# Base URL (varsayılan)
sentient config set llm.base_url "http://localhost:11434/v1"

# Model seç (FIXED KERNEL)
sentient config set llm.model "gemma4:31b"

# Diğer ayarlar
sentient config set llm.max_tokens 8192
sentient config set llm.temperature 0.7
```

### Gemma 4 Avantajları

| Özellik | Gemma 4 | GPT-4 | Claude 3.5 |
|---------|---------|-------|------------|
| **API Key** | ❌ Gerekmez | ✅ Gerekli | ✅ Gerekli |
| **Maliyet/1M token** | $0 | $30+ | $15+ |
| **Context Length** | 256K | 128K | 200K |
| **Vision** | ✅ | ✅ | ✅ |
| **Thinking Mode** | ✅ | ✅ | ✅ |
| **Open Source** | ✅ | ❌ | ❌ |
| **Local Execution** | ✅ | ❌ | ❌ |

---

## 3. HIZLI KURULUM

### Tek Komutla Kurulum

```bash
# Repoyu klonla ve kur
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
cd SENTIENT_CORE
chmod +x setup.sh && ./setup.sh
```

### Kurulum Sonrası

```bash
# SENTIENT'ı başlat
make run

# Veya doğrudan
cargo run --release

# Dashboard
make dashboard

# Gemma 4 ile sohbet
sentient chat "Merhaba SENTIENT!"
```

---

## 4. MANUEL KURULUM

### Adım 1: Rust Kurulumu

```bash
# Rust kur
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Rust'ı yükle
source $HOME/.cargo/env

# Sürümü kontrol et
rustc --version
cargo --version

# Gerekli component'ler
rustup component add clippy rustfmt
```

### Adım 2: Sistem Bağımlılıkları

```bash
# Ubuntu/Debian
sudo apt update && sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    sqlite3 \
    libsqlite3-dev \
    git \
    curl \
    wget

# macOS
brew install openssl sqlite
```

### Adım 3: Repoyu Klonlama

```bash
# GitHub'dan klonla
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
cd SENTIENT_CORE

# Submodule'leri çek (varsa)
git submodule update --init --recursive
```

### Adım 4: Ortam Değişkenlerini Ayarlama

```bash
# .env dosyasını oluştur
cp .env.example .env

# Düzenle
nano .env
```

**.env Dosyası İçeriği:**

```env
# ═══════════════════════════════════════════════════════════
#  GEMMA 4 KERNEL - YEREL LLM (API KEY GEREKMİYOR!)
# ═══════════════════════════════════════════════════════════
GEMMA4_MODEL=gemma4:31b
GEMMA4_BASE_URL=http://localhost:11434/v1
GEMMA4_CONTEXT_LENGTH=262144
GEMMA4_THINKING_MODE=true

# ═══════════════════════════════════════════════════════════
#  V-GATE (API ANAHTARI YÖNETİMİ - Opsiyonel)
# ═══════════════════════════════════════════════════════════
OPENROUTER_API_KEY=sk-or-v1-xxxxx
OPENAI_API_KEY=sk-xxxxx
ANTHROPIC_API_KEY=sk-ant-xxxxx

# ═══════════════════════════════════════════════════════════
#  V-GATE SUNUCU YAPILANDIRMASI
# ═══════════════════════════════════════════════════════════
V_GATE_URL=http://localhost:8100
V_GATE_LISTEN=127.0.0.1:1071
V_GATE_TIMEOUT=120

# ═══════════════════════════════════════════════════════════
#  GATEWAY (API SUNUCUSU)
# ═══════════════════════════════════════════════════════════
GATEWAY_HTTP_ADDR=0.0.0.0:8080
GATEWAY_PORT=8080
JWT_SECRET=your-jwt-secret-change-this

# ═══════════════════════════════════════════════════════════
#  BELLEK (MEMORY CUBE)
# ═══════════════════════════════════════════════════════════
MEMORY_DB_PATH=data/sentient.db
MEMORY_SHORT_TTL=3600
MEMORY_LONG_TTL=0
ZERO_COPY_ENABLED=true

# ═══════════════════════════════════════════════════════════
#  OASIS BRAIN (OTONOM DÜŞÜNCE)
# ═══════════════════════════════════════════════════════════
OASIS_BRAIN_MODEL=gemma4:31b
OASIS_BRAIN_THINKING=true
OASIS_BRAIN_MAX_STEPS=10

# ═══════════════════════════════════════════════════════════
#  GUARDRAILS (GÜVENLİK)
# ═══════════════════════════════════════════════════════════
GUARDRAILS_MODE=normal
GUARDRAILS_PROMPT_INJECTION=true
GUARDRAILS_DATA_EXFILTRATION=true

# ═══════════════════════════════════════════════════════════
#  LOGGING
# ═══════════════════════════════════════════════════════════
RUST_LOG=info
LOG_FILE=logs/sentient.log
```

### Adım 5: Projeyi Derleme

```bash
# Debug modda derleme (hızlı)
cargo build

# Release modda derleme (optimizasyonlu, önerilen)
cargo build --release

# Belirli crate derleme
cargo build --release -p sentient_core
cargo build --release -p sentient_local  # Gemma 4 engine
cargo build --release -p oasis_brain     # OASIS Brain
```

### Adım 6: Test Etme

```bash
# Tüm testleri çalıştır
cargo test --all

# Belirli crate testi
cargo test -p sentient_core
cargo test -p sentient_local
cargo test -p oasis_brain

# Test coverage
cargo tarpaulin --out Html
```

---

## 5. DOCKER KURULUMU

### Dockerfile ile Kurulum

```dockerfile
# Dockerfile
FROM rust:1.80-slim as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    sqlite3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install Ollama
RUN curl -fsSL https://ollama.com/install.sh | sh

WORKDIR /app
COPY --from=builder /app/target/release/sentient-dashboard /app/
COPY --from=builder /app/target/release/sentient-shell /app/

EXPOSE 8080 8100 11434

# Pull Gemma 4 on startup
RUN ollama pull gemma4:31b

CMD ["./sentient-dashboard"]
```

### Docker Image Oluşturma

```bash
# Image oluştur
docker build -t sentient-os:latest .

# Container başlat
docker run -d \
  --name sentient \
  --gpus all \
  -v ~/.sentient:/root/.sentient \
  -v $(pwd)/.env:/app/.env \
  -p 8080:8080 \
  -p 8100:8100 \
  -p 11434:11434 \
  sentient-os:latest
```

### Docker Compose ile Kurulum

```yaml
# docker-compose.yml
version: '3.8'

services:
  sentient:
    image: sentient/os:latest
    container_name: sentient
    ports:
      - "8080:8080"
      - "8100:8100"
    volumes:
      - ./data:/root/.sentient
      - ./logs:/var/log/sentient
      - ./.env:/app/.env
    environment:
      - RUST_LOG=info
      - V_GATE_URL=http://localhost:8100
      - GEMMA4_MODEL=gemma4:31b
    restart: unless-stopped
    depends_on:
      - ollama
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: 1
              capabilities: [gpu]

  ollama:
    image: ollama/ollama:latest
    container_name: ollama
    ports:
      - "11434:11434"
    volumes:
      - ollama_data:/root/.ollama
    restart: unless-stopped
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: 1
              capabilities: [gpu]

  chromadb:
    image: chromadb/chroma:latest
    container_name: chromadb
    ports:
      - "8000:8000"
    volumes:
      - chroma_data:/chroma/chroma
    restart: unless-stopped

volumes:
  ollama_data:
  chroma_data:
```

```bash
# Başlat
docker-compose up -d

# Gemma 4 modelini yükle
docker exec -it ollama ollama pull gemma4:31b

# Logları izle
docker-compose logs -f sentient

# Durdur
docker-compose down
```

---

## 6. RUST CRATE KURUMLARI

### Çekirdek Crate'ler

SENTIENT 38 Rust crate'ten oluşur. Her biri ayrı ayrı derlenebilir:

```bash
# Tüm crate'leri derle
cargo build --release --workspace

# Gemma 4 Engine
cargo build --release -p sentient_local

# OASIS Brain (Gemma 4 Fixed Kernel)
cargo build --release -p oasis_brain

# Memory Cube
cargo build --release -p sentient_memory

# V-GATE Proxy
cargo build --release -p sentient_vgate

# Guardrails
cargo build --release -p sentient_guardrails

# Core
cargo build --release -p sentient_core
```

### Binary Çalıştırma

```bash
# SENTIENT Shell
cargo run --release --bin sentient-shell

# Dashboard
cargo run --release --bin sentient-dashboard

# Skill Ingestor
cargo run --release --bin sentient-ingestor

# V-GATE Proxy
cargo run --release --bin sentient-vgate

# Self-Coder
cargo run --release --bin sentient-selfcoder
```

---

## 7. ENTEGRASYON KURUMLARI

### Agent Frameworks (17 Entegrasyon)

#### AutoGen Kurulumu

```bash
cd integrations/agents/autogen
pip install -e .
```

#### CrewAI Kurulumu

```bash
cd integrations/agents/crewai
pip install -e .
```

#### OpenHands Kurulumu

```bash
cd integrations/agents/openhands
pip install -e .
```

### Memory/Vector DB (4 Entegrasyon)

#### ChromaDB Kurulumu

```bash
cd integrations/memory/chromadb
pip install -e .

# ChromaDB sunucusu başlat
chroma run --host 0.0.0.0 --port 8000
```

#### Qdrant Kurulumu

```bash
# Docker ile Qdrant
docker run -d -p 6333:6333 qdrant/qdrant
```

### Browser Automation (5 Entegrasyon)

#### Browser-Use Kurulumu

```bash
cd integrations/browser/browser-use
pip install -e .

# Playwright kur
playwright install
```

#### Lightpanda Kurulumu

```bash
cd integrations/browser/lightpanda
# Lightpanda binary derle
make build
```

### Search Engine (2 Entegrasyon)

#### MindSearch Kurulumu

```bash
cd integrations/search/MindSearch
pip install -e .

# MindSearch başlat
python -m mindsearch
```

#### Searxng Kurulumu

```bash
cd integrations/search/searxng
docker-compose up -d
```

### Sandbox (3 Entegrasyon)

#### LocalStack Kurulumu

```bash
# Docker ile LocalStack
docker run -d -p 4566:4566 localstack/localstack
```

#### E2B SDK Kurulumu

```bash
cd integrations/sandbox/e2b-sdk
pip install -e .
```

---

## 8. LLM PROVIDER KURULUMU

### ⭐ Gemma 4 (ÖNERİLEN - NATIVE KERNEL)

```bash
# Ollama kur
curl -fsSL https://ollama.com/install.sh | sh

# Gemma 4 31B indir
ollama pull gemma4:31b

# SENTIENT'ya bağla
sentient config set llm.provider "gemma4"
sentient config set llm.model "gemma4:31b"
```

**Avantajlar:**
- ✅ API Key GEREKMİYOR
- ✅ Tamamen yerel
- ✅ 256K context
- ✅ Ücretsiz

### OpenAI

```bash
# API anahtarı al
# https://platform.openai.com/api-keys

# SENTIENT'ya ekle
sentient config set llm.provider "openai"
sentient config set openai.api_key "sk-..."
sentient config set llm.model "gpt-4-turbo"
```

### Anthropic Claude

```bash
# API anahtarı al
# https://console.anthropic.com/

# SENTIENT'ya ekle
sentient config set llm.provider "anthropic"
sentient config set anthropic.api_key "sk-ant-..."
sentient config set llm.model "claude-3-opus-20240229"
```

### Groq (Ücretsiz Tier Mevcut)

```bash
# API anahtarı al
# https://console.groq.com/keys

# SENTIENT'ya ekle
sentient config set llm.provider "groq"
sentient config set groq.api_key "gsk_..."
sentient config set llm.model "llama-3.1-70b-versatile"
```

### OpenRouter (Çoklu Model Erişimi)

```bash
# API anahtarı al
# https://openrouter.ai/keys

# SENTIENT'ya ekle
sentient config set llm.provider "openrouter"
sentient config set openrouter.api_key "sk-or-..."
sentient config set llm.model "anthropic/claude-3.5-sonnet"
```

---

## 9. API ANAHTARI YAPILANDIRMASI

### .env Dosyası ile

```bash
# .env dosyasını düzenle
nano .env
```

```env
# GEMMA 4 - API KEY GEREKMİYOR!
GEMMA4_MODEL=gemma4:31b
GEMMA4_BASE_URL=http://localhost:11434/v1

# Diğer provider'lar (opsiyonel)
OPENROUTER_API_KEY=sk-or-v1-xxxxx
OPENAI_API_KEY=sk-xxxxx
ANTHROPIC_API_KEY=sk-ant-xxxxx

# V-GATE YAPILANDIRMASI
V_GATE_URL=http://localhost:8100
V_GATE_LISTEN=127.0.0.1:1071

# TELEGRAM BOT
TELEGRAM_BOT_TOKEN=123456:ABC-DEF...

# GITHUB
GITHUB_TOKEN=ghp_xxxxx

# GATEWAY
GATEWAY_PORT=8080
JWT_SECRET=your-jwt-secret-change-this
```

### CLI ile

```bash
# Interactive yapılandırma
sentient setup

# Tek tek ekleme
sentient config set gemma4.model "gemma4:31b"
sentient config set openai.api_key "sk-..."
sentient config set telegram.bot_token "123456:ABC..."

# Konfigürasyonu kaydet
sentient config save
```

---

## 10. TELEGRAM BOT KURULUMU

### Adım 1: Bot Oluşturma

1. Telegram'ı açın
2. **@BotFather**'ı arayın ve başlatın
3. `/newbot` komutunu gönderin
4. Bot için bir isim girin (ör: "SENTIENT Assistant")
5. Bot için bir username girin (ör: "sentient_my_bot")
6. **Token'ı kopyalayın** (ör: `123456789:ABCdefGHI...`)

### Adım 2: SENTIENT'ya Bağlama

```bash
# Token'ı ekle
sentient config set telegram.bot_token "123456789:ABCdefGHI..."

# Chat ID'yi ekle
sentient config set telegram.chat_id "123456789"

# Aktif et
sentient config set telegram.enabled true

# Konfigürasyonu kaydet
sentient config save
```

### Telegram Komutları

```
/start      - Botu başlat
/help       - Yardım mesajı
/status     - Sistem durumu
/run <skill> - Skill çalıştır
/chat <msg> - AI ile sohbet (Gemma 4)
```

---

## 11. DISCORD BOT KURULUMU

### Adım 1: Discord Developer Portal

1. https://discord.com/developers/applications adresine gidin
2. **"New Application"** butonuna tıklayın
3. Bot için bir isim girin
4. **"Bot"** sekmesine gidin
5. **"Add Bot"** butonuna tıklayın
6. **Token'ı kopyalayın**

### Adım 2: SENTIENT'ya Bağlama

```bash
# Token'ı ekle
sentient config set discord.bot_token "OTkXXXXXXXXXXXXXX..."

# Guild (Sunucu) ID'yi ekle
sentient config set discord.guild_id "123456789012345678"

# Aktif et
sentient config set discord.enabled true
```

---

## 12. SLACK ENTEGRASYONU

### Adım 1: Incoming Webhook Oluşturma

1. https://api.slack.com/apps adresine gidin
2. **"Create New App"** > **"From scratch"**
3. App ismi ve workspace seçin
4. **"Incoming Webhooks"** özelliğini aktif edin
5. **"Add New Webhook to Workspace"** butonuna tıklayın
6. Kanal seçin ve **"Allow"** deyin
7. **Webhook URL'yi** kopyalayın

### Adım 2: SENTIENT'ya Bağlama

```bash
# Webhook URL ekle
sentient config set slack.webhook_url "https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXX"

# Aktif et
sentient config set slack.enabled true
```

---

## 13. EMAIL (SMTP) KURULUMU

### Gmail için

```bash
# Gmail SMTP ayarları
sentient config set email.smtp_host "smtp.gmail.com"
sentient config set email.smtp_port 587
sentient config set email.smtp_user "your@gmail.com"
sentient config set email.smtp_pass "your-app-password"
sentient config set email.use_tls true
```

---

## 14. GITHUB ENTEGRASYONU

### Adım 1: Personal Access Token Oluşturma

1. https://github.com/settings/tokens adresine gidin
2. **"Generate new token (classic)"** seçin
3. Token'a bir isim verin
4. İzinleri seçin: `repo`, `read:org`, `read:user`, `workflow`
5. **"Generate token"** butonuna tıklayın
6. Token'ı kopyalayın

### Adım 2: SENTIENT'ya Bağlama

```bash
# Token ekle
sentient config set github.token "ghp_xxxxxxxxxxxxxxxxxxxx"

# Aktif et
sentient config set github.enabled true
```

---

## 15. V-GATE PROXY KURULUMU

### V-GATE Nedir?

V-GATE, API anahtarlarını güvenli bir şekilde saklayan ve yöneten proxy sunucusudur. API anahtarları asla istemci kodunda tutulmaz.

### V-GATE Başlatma

```bash
# V-GATE proxy sunucusunu başlat
cargo run --release --bin sentient-vgate

# Veya config ile
sentient vgate start --config ~/.config/sentient/vgate.toml
```

### V-GATE Yapılandırması

```toml
# ~/.config/sentient/vgate.toml

[server]
listen = "127.0.0.1:1071"
timeout = 120

[providers.gemma4]
base_url = "http://localhost:11434/v1"
api_key = ""
enabled = true

[providers.openai]
base_url = "https://api.openai.com/v1"
api_key = "sk-..."
enabled = true

[providers.anthropic]
base_url = "https://api.anthropic.com"
api_key = "sk-ant-..."
api_format = "anthropic"
enabled = true

[security]
encrypt_keys = true
allowed_origins = ["http://localhost:8080"]

[routing]
default_provider = "gemma4"
fallback_provider = "gemma4"
```

---

## 16. DASHBOARD ERİŞİMİ

### Dashboard'ı Başlatma

```bash
# Dashboard başlat
cargo run --release --bin sentient-dashboard

# Veya make ile
make dashboard

# Arka planda çalıştır
nohup cargo run --release --bin sentient-dashboard > logs/dashboard.log 2>&1 &
```

### Tarayıcıda Erişim

```
http://localhost:8080
```

### Dashboard Özellikleri

| Sekme | Açıklama |
|-------|----------|
| **Home** | Sistem durumu, Core Engines (Gemma 4 Kernel) |
| **Chat** | AI ile sohbet (Gemma 4) |
| **Agents** | Aktif ajanlar ve durumları |
| **Tools** | Tool yönetimi ve çalıştırma |
| **Skills** | Skill kütüphanesi ve arama |
| **Terminal** | Gömülü xterm.js terminal |
| **Logs** | Canlı log akışı |
| **Settings** | Tüm sistem ayarları |

---

## 17. SKILL VE TOOL YÖNETİMİ

### Skill Yönetimi

```bash
# Tüm skill'leri listele
sentient skills list

# Kategoriye göre listele
sentient skills list --category dev

# Skill ara
sentient skills search "python"

# Skill çalıştır
sentient skills run web-scraper --url "https://example.com"
```

### Tool Yönetimi

```bash
# Tüm tool'ları listele
sentient tools list

# Tool durumu
sentient tools status mindsearch

# Tool başlat
sentient tools start crawl4ai
```

---

## 18. OTONOM MOD YAPILANDIRMASI

### OASIS Brain - Gemma 4 ile Otonom Düşünce

```bash
# OASIS Brain'i aktif et
sentient config set oasis_brain.enabled true
sentient config set oasis_brain.model "gemma4:31b"
sentient config set oasis_brain.thinking_mode true
sentient config set oasis_brain.max_reasoning_steps 10
```

### Yetki Seviyeleri

```bash
# Level 1: Sadece okuma
sentient auth level 1

# Level 2: Dosya işlemleri
sentient auth level 2

# Level 3: Klavye/Mouse kontrolü
sentient auth level 3

# Level 4: Tam otonom
sentient auth level 4

# Level 5: Sistem yönetimi
sentient auth level 5
```

### Otonom Örnekler

```bash
# Web otomasyonu
sentient auto "Go to gmail.com and compose an email"

# Kod üretimi
sentient auto "Create a Python script that downloads images from a URL"

# Araştırma
sentient auto "Research the latest Rust async frameworks"
```

---

## 19. GÜVENLİK YAPILANDIRMASI

### Guardrails Ayarları

```bash
# Guardrails modu: strict, normal, permissive
sentient config set guardrails.mode "strict"

# Prompt injection koruması
sentient config set guardrails.prompt_injection true

# Veri sızıntısı koruması
sentient config set guardrails.data_exfiltration true
```

### V-GATE Güvenlik

```bash
# API anahtarı şifreleme
sentient config set vgate.encrypt_keys true

# Key rotation
sentient config set vgate.key_rotation_days 30
```

---

## 20. SORUN GIDERME

### Yaygın Sorunlar

#### Gemma 4 / Ollama Bağlantı Hatası

```bash
# Ollama servisini kontrol et
systemctl status ollama

# Ollama'yı yeniden başlat
ollama serve

# Model'in yüklü olduğunu kontrol et
ollama list

# Gemma 4'ü tekrar indir
ollama pull gemma4:31b
```

#### Rust Derleme Hatası

```bash
# Rust'ı güncelle
rustup update

# Bağımlılıkları temizle
cargo clean

# Yeniden derle
cargo build --release
```

#### Port Kullanımda Hatası

```bash
# Hangi port kullanımda kontrol et
sudo lsof -i :8080
sudo lsof -i :11434

# İşlemi sonlandır
sudo kill -9 <PID>
```

### Sıfırlama

```bash
# Konfigürasyonu sıfırla
sentient config reset

# Belleği temizle
sentient memory clear

# Tam sıfırlama
rm -rf ~/.sentient
rm -rf data/
cargo clean
```

---

## ✅ KURULUM DOĞRULAMA

```bash
# Tüm bileşenleri kontrol et
sentient doctor

# Örnek çıktı:
# ✅ Rust 1.80.0
# ✅ Cargo 1.80.0
# ✅ SENTIENT Core v4.0.0
# ✅ GEMMA 4 KERNEL: gemma4:31b (256K context)
# ✅ OASIS BRAIN: Active
# ✅ Memory Cube: Zero-Copy enabled
# ✅ V-GATE Proxy: Running on port 1071
# ✅ Dashboard: Running on port 8080
# ✅ Skills: 5587 loaded
# ✅ Tools: 43 available
# ✅ Ollama: Connected (gemma4:31b)
```

---

## 📁 DOSYA YAPISI ÖZETİ

```
SENTIENT_CORE/                    # 13 GB
├── crates/                    # 38 Rust Crate
│   ├── sentient_local/        # Gemma 4 Engine
│   │   └── src/gemma4.rs      # Gemma 4 Kernel
│   ├── oasis_brain/           # OASIS Brain
│   │   └── src/*.rs           # Reasoning, Perception, Action
│   ├── sentient_memory/       # Memory Cube (L3)
│   └── ...
├── integrations/              # 71 Entegre Repo
├── skills/                    # 5,587+ Skills
├── tools/                     # External Tools
├── README.md                  # Proje Özeti
├── INSTALL.md                 # Bu Dosya
└── .env.example               # Environment Template
```

---

**🧠 SENTIENT OS - Kurulum Tamamlandı!**

*Son Güncelleme: 2026-04-08*
*Versiyon: 4.0.0*
*Kernel: Gemma 4 31B*
