# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS - SİSTEMİ AYAĞA KALDIRMA REHBERİ
# ═══════════════════════════════════════════════════════════════════════════════
#  Tarih: 2026-04-13
#  Durum: PRODUCTION READY - Kurulum ve Başlatma
# ═══════════════════════════════════════════════════════════════════════════════

---

## 📊 MEVCUT SİSTEM DURUMU

```
┌─────────────────────────────────────────────────────────────────────┐
│                     SİSTEM KONTROL SONUÇLARI                        │
├─────────────────────────────────────────────────────────────────────┤
│  ✅ Docker         : 29.1.3                                         │
│  ✅ docker-compose : 1.29.2                                         │
│  ✅ Rust           : 1.94.1                                         │
│  ✅ Cargo          : 1.94.1                                         │
│  ⚠️  Ollama        : 0.20.5 (çalışmıyor - başlatılmalı)            │
│  ✅ Disk           : 64GB boş (yeterli)                             │
│  ✅ RAM            : 15GB (yeterli)                                 │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 🚀 AŞAMA 1: ORTAM HAZIRLAMA

### 1.1 Proje Dizinine Git

```bash
cd /root/SENTIENT_CORE
```

### 1.2 .env Dosyasını Oluştur

```bash
cp .env.example .env
```

### 1.3 API Key'leri Ekle

```bash
nano .env
```

**Minimum gerekli API key'ler:**

```bash
# SEÇENEK A: OpenRouter (Önerilen - 100+ model erişimi)
OPENROUTER_API_KEY=sk-or-v1-your-key-here

# SEÇENEK B: OpenAI
OPENAI_API_KEY=sk-proj-your-key-here

# SEÇENEK C: Anthropic
ANTHROPIC_API_KEY=sk-ant-your-key-here

# SEÇENEK D: Yerel (API Key gerektirmez - Ollama)
# Sadece Ollama kullanılacaksa API key gerekmez
```

**API Key Alma Linkleri:**
| Provider | Link | Ücret |
|----------|------|-------|
| OpenRouter | https://openrouter.ai/keys | Kullanıma göre |
| OpenAI | https://platform.openai.com/api-keys | Kullanıma göre |
| Anthropic | https://console.anthropic.com/ | Kullanıma göre |
| Groq (Hızlı) | https://console.groq.com/keys | Ücretsiz tier |
| Ollama | Yerel | **Ücretsiz** |

### 1.4 JWT Secret ve API Key Üret

```bash
# JWT Secret
echo "JWT_SECRET=$(openssl rand -base64 64 | tr -d '\n')" >> .env

# Gateway API Key
echo "GATEWAY_API_KEYS=$(openssl rand -hex 32)" >> .env
```

---

## 🔧 AŞAMA 2: OLLAMA'YI BAŞLAT (YEREL LLM)

### 2.1 Ollama Servisini Başlat

```bash
# Systemd ile (önerilen)
sudo systemctl start ollama
sudo systemctl enable ollama

# Veya direkt
ollama serve &
```

### 2.2 Model İndir

```bash
# Varsayılan KERNEL modeli
ollama pull gemma2:27b

# Alternatifler
ollama pull llama3.3:70b      # En güçlü
ollama pull qwen2.5:72b       # Çok güçlü
ollama pull deepseek-r1:70b   # Reasoning
ollama pull mistral:7b        # Hafif
```

### 2.3 Test Et

```bash
ollama run gemma2:27b "Merhaba, nasılsın?"
```

---

## 🐳 AŞAMA 3: DOCKER SERVİSLERİNİ BAŞLAT

### 3.1 Docker Servislerini Kontrol Et

```bash
docker --version
docker-compose --version
```

### 3.2 Tüm Servisleri Başlat

```bash
cd /root/SENTIENT_CORE

# İlk başlatma (build + up)
docker-compose up -d --build

# Sonraki başlatmalar
docker-compose up -d
```

**Başlayan servisler:**
```
┌────────────────────────────────────────────────────────────────────┐
│  SERVIS         │ PORT   │ AÇIKLAMA                                │
├────────────────────────────────────────────────────────────────────┤
│  sentient       │ 8080   │ Ana Gateway API                         │
│  postgres       │ 5432   │ PostgreSQL Veritabanı                   │
│  redis          │ 6379   │ Redis Cache                             │
│  minio          │ 9000   │ S3-Compatible Storage                   │
│  minio-console  │ 9001   │ MinIO Web UI                            │
│  qdrant         │ 6333   │ Vector Database                         │
│  prometheus     │ 9090   │ Metrics                                 │
│  grafana        │ 3000   │ Dashboard (admin/sentient)              │
│  nginx          │ 80/443 │ Reverse Proxy                           │
└────────────────────────────────────────────────────────────────────┘
```

### 3.3 Servis Durumunu Kontrol Et

```bash
docker-compose ps
```

**Beklenen çıktı:**
```
NAME                  STATUS    PORTS
sentient-gateway      Up        0.0.0.0:8080->8080/tcp
sentient-postgres     Up        5432/tcp
sentient-redis        Up        6379/tcp
sentient-minio        Up        0.0.0.0:9000-9001->9000-9001/tcp
sentient-qdrant       Up        0.0.0.0:6333-6334->6333-6334/tcp
sentient-prometheus   Up        0.0.0.0:9090->9090/tcp
sentient-grafana      Up        0.0.0.0:3000->3000/tcp
```

---

## 🔨 AŞAMA 4: RUST PROJESİNİ DERLE

### 4.1 Derleme (Release Mode)

```bash
cd /root/SENTIENT_CORE

# Tüm workspace'i derle
cargo build --release

# Sadece CLI ve Gateway
cargo build --release -p sentient_cli -p sentient_gateway
```

**Derleme süresi:** ~15-20 dakika (ilk derleme)

### 4.2 Derleme Sonucu

```bash
ls -la target/release/sentient
ls -la target/release/sentient-gateway
```

---

## 🎯 AŞAMA 5: SETUP WIZARD ÇALIŞTIR

### 5.1 Interaktif Kurulum

```bash
./target/release/sentient setup
```

**Veya environment variable ile:**
```bash
OPENAI_API_KEY=sk-... ./target/release/sentient setup
```

### 5.2 Kurulum Adımları

```
┌─────────────────────────────────────────────────────────────────────┐
│  ADIM 0: Security Warning                                           │
│  ? Do you want to continue? [Y/n]                                   │
├─────────────────────────────────────────────────────────────────────┤
│  ADIM 1: Setup Mode                                                 │
│  ? QuickStart (Recommended)                                         │
├─────────────────────────────────────────────────────────────────────┤
│  ADIM 2: LLM Provider                                               │
│  ? Select provider: openai/gpt-4o                                   │
│  ? OpenAI API Key: ********                                         │
├─────────────────────────────────────────────────────────────────────┤
│  ADIM 3: Channels                                                   │
│  ? Select: [ ] Telegram  [ ] Discord  [x] Web Dashboard             │
├─────────────────────────────────────────────────────────────────────┤
│  ADIM 4: Tools                                                      │
│  ? Select: [x] DuckDuckGo  [ ] Tavily                               │
├─────────────────────────────────────────────────────────────────────┤
│  ADIM 5: Save                                                       │
│  [OK] Configuration saved to ~/.sentient/config.json                │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 🚀 AŞAMA 6: SİSTEMİ BAŞLAT

### 6.1 Gateway Başlat

```bash
# Foreground (test için)
./target/release/sentient gateway

# Background
./target/release/sentient gateway &

# Docker ile
docker-compose up -d sentient
```

### 6.2 REPL Başlat

```bash
./target/release/sentient repl
```

**REPL İçinde:**
```
SENTIENT> Merhaba, bana yardımcı olabilir misin?
SENTIENT> Python'da bir web scraper yaz
SENTIENT> /help
SENTIENT> /exit
```

### 6.3 Agent Modu

```bash
# Otonom agent
./target/release/sentient agent --goal "Analyze the codebase and create a summary"

# Task spesifik
./target/release/sentient agent --task "Fix the bug in src/main.rs"
```

---

## 🌐 AŞAMA 7: WEB DASHBOARD

### 7.1 Dashboard Erişimi

```
http://localhost:3000   (Grafana)
http://localhost:8080   (API Gateway)
http://localhost:9001   (MinIO Console)
http://localhost:9090   (Prometheus)
```

### 7.2 Grafana Giriş

```
Username: admin
Password: sentient
```

---

## ✅ AŞAMA 8: TEST VE DOĞRULAMA

### 8.1 API Health Check

```bash
curl http://localhost:8080/health
```

**Beklenen:**
```json
{"status": "healthy", "version": "4.0.0"}
```

### 8.2 LLM Test

```bash
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "model": "gpt-4o",
    "messages": [{"role": "user", "content": "Hello!"}]
  }'
```

### 8.3 Vector DB Test

```bash
curl http://localhost:6333/collections
```

### 8.4 Redis Test

```bash
docker exec sentient-redis redis-cli ping
```

**Beklenen:** `PONG`

---

## 📦 ALTERNATİF: TEK KOMUTLA KURULUM

### Hızlı Başlatma Script'i

```bash
#!/bin/bash
# quick_start.sh

set -e

echo "🚀 SENTIENT OS Quick Start"

# 1. .env kontrol
if [ ! -f .env ]; then
    echo "📝 Creating .env from template..."
    cp .env.example .env
    echo "⚠️  Please edit .env and add your API keys"
    exit 1
fi

# 2. Docker servisleri
echo "🐳 Starting Docker services..."
docker-compose up -d

# 3. Ollama kontrol
echo "🤖 Checking Ollama..."
if ! pgrep -x "ollama" > /dev/null; then
    echo "Starting Ollama..."
    ollama serve &
    sleep 5
fi

# 4. Rust derleme
echo "🔨 Building Rust project..."
cargo build --release -p sentient_cli -p sentient_gateway

# 5. Setup wizard
echo "⚙️  Running setup wizard..."
./target/release/sentient setup

# 6. Başlat
echo "🎯 Starting Gateway..."
./target/release/sentient gateway &

echo "✅ SENTIENT OS is running!"
echo "   Dashboard: http://localhost:3000"
echo "   API: http://localhost:8080"
```

---

## 🔧 SORUN GİDERME

### Docker Hataları

```bash
# Container logları
docker-compose logs sentient

# Restart
docker-compose restart

# Temiz başlangıç
docker-compose down -v
docker-compose up -d --build
```

### Rust Derleme Hataları

```bash
# Temiz derleme
cargo clean
cargo build --release

# Tek crate
cargo build --release -p sentient_cli
```

### Ollama Bağlantı Hatası

```bash
# Servis başlat
sudo systemctl start ollama

# Manuel
ollama serve &

# Test
ollama list
```

### Port Çakışması

```bash
# Kullanılan portları kontrol et
netstat -tlnp | grep -E '8080|3000|5432|6379'

# Öldür
kill -9 <PID>
```

---

## 📊 SİSTEM MİMARİSİ

```
┌─────────────────────────────────────────────────────────────────────────┐
│                          SENTIENT OS ARCHITECTURE                        │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                  │
│  │   CLI       │    │   Web       │    │   API       │                  │
│  │   sentient  │    │   Dashboard │    │   Gateway   │                  │
│  └──────┬──────┘    └──────┬──────┘    └──────┬──────┘                  │
│         │                  │                  │                          │
│         └──────────────────┼──────────────────┘                          │
│                            │                                             │
│                            ▼                                             │
│  ┌──────────────────────────────────────────────────────────────────┐   │
│  │                    CORE LAYERS (17 Katman)                        │   │
│  ├──────────────────────────────────────────────────────────────────┤   │
│  │  L1: Core        │  L2: Memory     │  L3: Agent                  │   │
│  │  L4: LLM         │  L5: Storage    │  L6: Integration            │   │
│  │  L7: Skills      │  L8: Enterprise │  L9: Media                  │   │
│  │  L10: Presentation│ L11: OASIS     │  L12: AI/ML                 │   │
│  │  L13: DevOps     │  L14: Data      │  L15: Security              │   │
│  │  L16: Utility    │  L17: Extension │                             │   │
│  └──────────────────────────────────────────────────────────────────┘   │
│                            │                                             │
│                            ▼                                             │
│  ┌──────────────────────────────────────────────────────────────────┐   │
│  │                    INFRASTRUCTURE                                 │   │
│  ├──────────────────────────────────────────────────────────────────┤   │
│  │  PostgreSQL │ Redis │ Qdrant │ MinIO │ Prometheus │ Grafana     │   │
│  └──────────────────────────────────────────────────────────────────┘   │
│                            │                                             │
│                            ▼                                             │
│  ┌──────────────────────────────────────────────────────────────────┐   │
│  │                    EXTERNAL SERVICES                              │   │
│  ├──────────────────────────────────────────────────────────────────┤   │
│  │  Ollama (Local) │ OpenAI │ Anthropic │ Google │ Groq │ ...      │   │
│  └──────────────────────────────────────────────────────────────────┘   │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 🎯 ÖZET: 3 SEÇENEK

### SEÇENEK A: Docker ile Tam Kurulum (Önerilen)

```bash
cd /root/SENTIENT_CORE
cp .env.example .env
nano .env  # API key'leri ekle
docker-compose up -d
cargo build --release -p sentient_cli -p sentient_gateway
./target/release/sentient setup
./target/release/sentient gateway
```

### SEÇENEK B: Sadece Yerel (Ollama)

```bash
cd /root/SENTIENT_CORE
ollama serve &
ollama pull gemma2:27b
cargo build --release -p sentient_cli
./target/release/sentient setup  # ollama/gemma2:27b seç
./target/release/sentient repl
```

### SEÇENEK C: Development Mode

```bash
cd /root/SENTIENT_CORE
cargo run --release -p sentient_cli -- setup
cargo run --release -p sentient_cli -- repl
```

---

## 📋 KONTROL LİSTESİ

```
[ ] 1. .env dosyası oluşturuldu
[ ] 2. API key'ler eklendi
[ ] 3. Ollama başlatıldı
[ ] 4. Model indirildi (ollama pull)
[ ] 5. Docker servisleri başlatıldı (docker-compose up -d)
[ ] 6. Rust projesi derlendi (cargo build --release)
[ ] 7. Setup wizard çalıştırıldı (sentient setup)
[ ] 8. Gateway başlatıldı (sentient gateway)
[ ] 9. Health check başarılı (curl localhost:8080/health)
[ ] 10. REPL test edildi (sentient repl)
```

---

## 🔗 ERİŞİM NOKTALARI

| Servis | URL | Kullanıcı/Şifre |
|--------|-----|-----------------|
| **API Gateway** | http://localhost:8080 | API Key |
| **Grafana** | http://localhost:3000 | admin/sentient |
| **MinIO Console** | http://localhost:9001 | minioadmin/minioadmin |
| **Prometheus** | http://localhost:9090 | - |
| **Qdrant Dashboard** | http://localhost:6333/dashboard | - |

---

*Rapor Tarihi: 2026-04-13*
*Durum: PRODUCTION READY - Kurulum Rehberi*
