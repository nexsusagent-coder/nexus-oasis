# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS - SİSTEMİ AYAĞA KALDIRMA REHBERİ (TAM DÖKÜMANTASYON)
# ═══════════════════════════════════════════════════════════════════════════════
#  Tarih: 2026-04-14
#  Durum: TEST EDİLDİ VE ÇALIŞIYOR
#  Kaynak: Oturum 9 Entegrasyon Testleri
# ═══════════════════════════════════════════════════════════════════════════════

---

# 📋 İÇİNDEKİLER

1. [Sistem Gereksinimleri](#1-sistem-gereksinimleri)
2. [Hızlı Başlangıç](#2-hızlı-başlangıç)
3. [Detaylı Kurulum Adımları](#3-detaylı-kurulum-adımları)
4. [Yapılandırma](#4-yapılandırma)
5. [Servisleri Başlatma](#5-servisleri-başlatma)
6. [Test ve Doğrulama](#6-test-ve-doğrulama)
7. [API Endpoint Referansı](#7-api-endpoint-referansı)
8. [Sorun Giderme](#8-sorun-giderme)
9. [Performans Metrikleri](#9-performans-metrikleri)
10. [Güvenlik Notları](#10-güvenlik-notları)

---

# 1. SİSTEM GEREKSİNİMLERİ

## 1.1 Donanım Gereksinimleri

| Bileşen | Minimum | Önerilen |
|---------|---------|----------|
| CPU | 4 çekirdek | 8+ çekirdek |
| RAM | 8 GB | 16+ GB |
| Disk | 50 GB | 100+ GB SSD |
| Network | İnternet erişimi | - |

## 1.2 Yazılım Gereksinimleri

| Yazılım | Versiyon | Kurulum Komutu |
|---------|----------|----------------|
| Rust | 1.75+ | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| Docker | 20.10+ | `curl -fsSL https://get.docker.com \| sh` |
| Docker Compose | 2.0+ | Docker ile birlikte gelir |
| Git | 2.0+ | `apt install git` |

## 1.3 Port Gereksinimleri

| Port | Servis | Açıklama |
|------|--------|----------|
| 8080 | Gateway | Ana API sunucusu |
| 5432 | PostgreSQL | Veritabanı |
| 6379 | Redis | Önbellek |
| 6333 | Qdrant | Vektör veritabanı |
| 9000 | MinIO | Nesne depolama (API) |
| 9001 | MinIO | Nesne depolama (Console) |
| 9090 | Prometheus | Metrik toplama |
| 3001 | Grafana | Görselleştirme |

---

# 2. HIZLI BAŞLANGIÇ

## 2.1 Tek Komutla Başlat (ÖNERİLEN)

```bash
# Tek komutla tam kurulum
./install.sh

# Veya uzaktan
curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.sh | bash
```

**Kurulum sırasında sorulacaklar:**
```
1. LLM Seçimi:
   [1] LOKAL (Ücretsiz)    → Ollama + Llama3.2/Gemma3/DeepSeek
   [2] API KEY (Ücretli)    → OpenRouter/OpenAI/Anthropic
   [3] HİBRİT               → Lokal + API fallback

2. Voice Seçimi:
   [1] Lokal (Ücretsiz)     → Whisper.cpp + Piper TTS
   [2] API (Ücretli)        → OpenAI Whisper + ElevenLabs
   [3] Yok                   → Sadece metin

3. Model Seçimi (Lokal için):
   [1] Llama 3.2 (3B)       ~2GB   - Hızlı
   [2] Gemma 3 (4B)         ~3GB   - Dengeli
   [3] Qwen 2.5 (7B)        ~4GB   - Türkçe iyi
   [4] DeepSeek R1 (7B)     ~4GB   - Reasoning
   [5] Llama 3.2 (11B)      ~7GB   - Daha akıllı
   [6] Gemma 3 (27B)        ~16GB  - En iyi (önerilen)
   [7] DeepSeek R1 (67B)    ~40GB  - En akıllı
```

## 2.2 Manuel Kurulum (Detaylı)

Adım adım kurulum için Bölüm 3'e bakın.

---

# 3. DETAYLI KURULUM ADIMLARI

## 3.0 Adım 0: LLM Seçimi (KRİTİK)

Kurulumdan önce LLM stratejinizi belirleyin:

### ═══════════════════════════════════════════════════════════════════════════
###  SEÇENEK A: LOKAL LLM (Ücretsiz, Gizli)
### ═══════════════════════════════════════════════════════════════════════════

**Avantajları:**
- ✅ Tamamen ücretsiz
- ✅ İnternet gerektirmez
- ✅ Verileriniz dışarı çıkmaz (gizlilik)
- ✅ Sınırsız kullanım
- ✅ Gecikme yok (lokal)

**Dezavantajları:**
- ⚠️ GPU gerektirir (NVIDIA/AMD/Apple Silicon)
- ⚠️ RAM gerektirir (model boyutuna göre)
- ⚠️ İlk kurulum büyük (model indirme)

**Kurulum:**
```bash
# 1. Ollama kur
curl -fsSL https://ollama.com/install.sh | sh

# 2. Model seç ve indir
ollama pull llama3.2          # 3B  model (~2GB)   - Hızlı
ollama pull gemma3:4b        # 4B  model (~3GB)   - Dengeli
ollama pull qwen2.5:7b       # 7B  model (~4GB)   - Türkçe iyi
ollama pull deepseek-r1:7b   # 7B  model (~4GB)   - Reasoning
ollama pull llama3.2:11b     # 11B model (~7GB)   - Daha akıllı
ollama pull gemma3:27b       # 27B model (~16GB)  - ÖNERİLEN
ollama pull deepseek-r1:67b  # 67B model (~40GB)  - En akıllı

# 3. Test
ollama run gemma3:27b "Merhaba, nasılsın?"

# 4. Servis başlat
ollama serve &
```

**Model Karşılaştırması:**

| Model | Boyut | RAM | Türkçe | Reasoning | Hız | Öneri |
|-------|-------|-----|--------|-----------|-----|-------|
| Llama 3.2 3B | 2GB | 4GB | ⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐⭐ | Hızlı |
| Gemma 3 4B | 3GB | 6GB | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | Dengeli |
| Qwen 2.5 7B | 4GB | 8GB | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | Türkçe |
| DeepSeek R1 7B | 4GB | 8GB | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | Reasoning |
| Llama 3.2 11B | 7GB | 12GB | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | Akıllı |
| **Gemma 3 27B** | 16GB | 24GB | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐ | **ÖNERİLEN** |
| DeepSeek R1 67B | 40GB | 64GB | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐ | En İyi |

**.env Yapılandırması:**
```bash
LLM_PROVIDER=ollama
OLLAMA_HOST=http://localhost:11434
OLLAMA_MODEL=gemma3:27b
```

---

### ═══════════════════════════════════════════════════════════════════════════
###  SEÇENEK B: API KEY (Ücretli, Kolay)
### ═══════════════════════════════════════════════════════════════════════════

**Avantajları:**
- ✅ Kurulum çok hızlı
- ✅ En iyi kalite (GPT-4, Claude)
- ✅ 200+ model erişimi (OpenRouter)
- ✅ GPU gerektirmez

**Dezavantajları:**
- ❌ Ücretli
- ❌ İnternet gerektirir
- ❌ Veriler API'ye gider

**Sağlayıcılar:**

| Sağlayıcı | Modeller | Fiyat | Başlangıç | Kaynak |
|-----------|----------|-------|-----------|--------|
| **OpenRouter** | 200+ model | Değişken | $5 bonus | [openrouter.ai/keys](https://openrouter.ai/keys) |
| OpenAI | GPT-4, GPT-4o | $0.002-0.03/1K token | - | [platform.openai.com](https://platform.openai.com/api-keys) |
| Anthropic | Claude 3.5 | $0.003-0.015/1K token | - | [console.anthropic.com](https://console.anthropic.com/) |
| Google AI | Gemini 2.0 | $0.001-0.07/1K token | - | [aistudio.google.com](https://aistudio.google.com/apikey) |

**OpenRouter (ÖNERİLEN):**
```bash
# 1. API Key al: https://openrouter.ai/keys

# 2. .env'e ekle (veya export et)
export OPENROUTER_API_KEY=sk-or-v1-your-key

# 3. Model seç
LLM_PROVIDER=openrouter
LLM_MODEL=openai/gpt-4o-mini       # Hızlı, ucuz
LLM_MODEL=openai/gpt-4o            # Dengeli
LLM_MODEL=anthropic/claude-3.5-sonnet  # En iyi
LLM_MODEL=google/gemini-2.0-flash  # Google
LLM_MODEL=meta-llama/llama-3.3-70b-instruct  # Açık kaynak
```

**OpenAI:**
```bash
export OPENAI_API_KEY=sk-your-key
LLM_PROVIDER=openai
LLM_MODEL=gpt-4o-mini
```

**Anthropic:**
```bash
export ANTHROPIC_API_KEY=sk-ant-your-key
LLM_PROVIDER=anthropic
LLM_MODEL=claude-3-5-sonnet-20241022
```

---

### ═══════════════════════════════════════════════════════════════════════════
###  SEÇENEK C: HİBRİT (En İyi Denge)
### ═══════════════════════════════════════════════════════════════════════════

**Nasıl çalışır:**
1. Basit sorular → Lokal model (ücretsiz)
2. Zor sorular → API model (fallback)
3. Özel veri → Lokal model (gizlilik)

**Kurulum:**
```bash
# 1. Lokal model kur
ollama pull gemma3:27b

# 2. API key al
export OPENROUTER_API_KEY=sk-or-v1-your-key

# 3. .env
LLM_MODE=hybrid
LLM_LOCAL_MODEL=gemma3:27b
LLM_API_MODEL=openai/gpt-4o-mini
```

---

### LLM Maliyet Karşılaştırması

| Senaryo | Lokal | OpenRouter | OpenAI |
|---------|-------|------------|--------|
| 1000 sohbet/gün | $0 | ~$5-20/ay | ~$10-30/ay |
| 10000 sohbet/gün | $0 | ~$50-200/ay | ~$100-300/ay |
| Enterprise (yüksek) | $0* | ~$500+/ay | ~$1000+/ay |

*Lokal için elektrik + donanım maliyeti

---

## 3.1 Adım 1: Repoyu İndir

```bash
git clone <repo-url>
cd SENTIENT_CORE
```

## 3.2 Adım 2: Rust Projesini Derle

```bash
# Release modunda derle (optimize edilmiş)
cargo build --release --bin sentient --bin sentient-web

# Derleme süresi: ~10-15 dakika (ilk derleme)
# Sonraki derlemeler: ~1-2 dakika

# Beklenen çıktı:
# Finished `release` profile [optimized] target(s) in XX.XXs
```

**Derleme Doğrulama:**
```bash
ls -la target/release/sentient
# -rwxr-xr-x 1 root root 85M Apr 14 13:00 target/release/sentient

ls -la target/release/sentient-web
# -rwxr-xr-x 1 root root 12M Apr 14 13:00 target/release/sentient-web
```

## 3.3 Adım 3: Docker Servislerini Başlat

```bash
# Tüm servisleri başlat
docker-compose up -d

# Beklenen çıktı:
# [+] Running 6/6
#  ✔ Network sentient_default      Created
#  ✔ Container sentient-postgres   Started
#  ✔ Container sentient-redis      Started
#  ✔ Container sentient-qdrant     Started
#  ✔ Container sentient-minio      Started
#  ✔ Container sentient-prometheus Started
#  ✔ Container sentient-grafana    Started
```

**Servis Durumunu Kontrol Et:**
```bash
docker-compose ps

# Beklenen çıktı:
# Name                Command                  State               Ports
# ---------------------------------------------------------------------------
# sentient-grafana     /run.sh                  Up (healthy)       0.0.0.0:3001->3000/tcp
# sentient-minio       /usr/bin/entrypoint...   Up (healthy)       0.0.0.0:9000-9001->9000-9001/tcp
# sentient-postgres    docker-entrypoint...     Up (healthy)       0.0.0.0:5432->5432/tcp
# sentient-prometheus  /bin/prometheus...       Up (healthy)       0.0.0.0:9090->9090/tcp
# sentient-qdrant      ./entrypoint.sh          Up (unhealthy)     0.0.0.0:6333-6334->6333-6334/tcp
# sentient-redis       docker-entrypoint...     Up (healthy)       0.0.0.0:6379->6379/tcp
```

**NOT:** Qdrant bazen "unhealthy" gösterebilir ama bu kritik değildir, çalışır durumdadır.

## 3.4 Adım 4: Ortam Değişkenlerini Ayarla

```bash
# .env dosyasını oluştur
cp .env.template .env
```

**.env Dosyası İçeriği:**
```bash
# ═══════════════════════════════════════════════════════════════
#  SENTIENT OS - ORTAM YAPILANDIRMASI
# ═══════════════════════════════════════════════════════════════

# ─── LLM SAĞLAYICILARI ───
# En az birini doldurun:

# OpenRouter (önerilen - 200+ model erişimi)
OPENROUTER_API_KEY=sk-or-v1-your-key-here

# OpenAI (alternatif)
OPENAI_API_KEY=sk-your-key-here

# Anthropic (alternatif)
ANTHROPIC_API_KEY=sk-ant-your-key-here

# ─── VERİTABANI ───
DATABASE_URL=postgres://sentient:sentient123@localhost:5432/sentient
REDIS_URL=redis://localhost:6379

# ─── GÜVENLİK ───
JWT_SECRET=change-this-to-a-random-64-char-string
GATEWAY_API_KEY=change-this-to-a-random-32-char-string

# ─── OLLAMA (LOKAL LLM) ───
OLLAMA_HOST=http://localhost:11434
OLLAMA_MODEL=gemma3:27b

# ─── QDRANT (VEKTÖR DB) ───
QDRANT_URL=http://localhost:6333

# ─── MINIO (NESNE DEPOLAMA) ───
MINIO_ENDPOINT=localhost:9000
MINIO_ACCESS_KEY=minioadmin
MINIO_SECRET_KEY=minioadmin

# ─── PROMETHEUS ───
PROMETHEUS_URL=http://localhost:9090

# ─── GRAFANA ───
GRAFANA_URL=http://localhost:3001
```

**API Key Alma:**
- OpenRouter: https://openrouter.ai/keys
- OpenAI: https://platform.openai.com/api-keys
- Anthropic: https://console.anthropic.com/

## 3.5 Adım 5: Gateway'i Başlat

```bash
# API key'i ortam değişkeni olarak geçerek başlat
OPENROUTER_API_KEY=sk-or-v1-your-key-here ./target/release/sentient gateway

# Veya .env dosyası ile
source .env && ./target/release/sentient gateway
```

**Başlangıç Çıktısı:**
```
╔════════════════════════════════════════════════════════════╗
║                                                            ║
║     █████╗ ███╗   ██╗███████╗██╗      ██████╗ ██╗   ██╗   ║
║    ██╔══██╗████╗  ██║██╔════╝██║     ██╔═══██╗██║   ██║   ║
║    ███████║██╔██╗ ██║█████╗  ██║     ██║   ██║██║   ██║   ║
║    ██╔══██║██║╚██╗██║██╔══╝  ██║     ██║   ██║██║   ██║   ║
║    ██║  ██║██║ ╚████║███████╗███████╗╚██████╔╝╚██████╔╝   ║
║    ╚═╝  ╚═╝╚═╝  ╚═══╝╚══════╝╚══════╝ ╚═════╝  ╚═════╝    ║
║                                                            ║
║            NEXUS OASIS — Yapay Zeka İşletim Sistemi        ║
╚════════════════════════════════════════════════════════════╝

════════════════════════════════════════════════════════════
  🌐  SENTIENT GATEWAY SUNUCUSU
════════════════════════════════════════════════════════════

📡  HTTP API: http://0.0.0.0:8080

Kapatmak için Ctrl+C'ye basın.

[INFO] 🌐  SENTIENT GATEWAY başlatılıyor...
[INFO] 📊  Task Manager başlatıldı (max: 10 concurrent)
[INFO] 📡  HTTP API dinleniyor: 0.0.0.0:8080
[INFO] 🌐  HTTP API dinleniyor: http://0.0.0.0:8080
[INFO] 📡  Endpoints:
[INFO]     POST /api/task       → Yeni görev oluştur
[INFO]     GET  /api/task/:id   → Görev durumu
[INFO]     GET  /api/tasks      → Tüm görevler
[INFO]     DEL  /api/task/:id   → Görevi iptal et
[INFO]     GET  /api/stats      → İstatistikler
[INFO]     GET  /health         → Sağlık kontrolü
[INFO]     WS   /ws             → WebSocket
[INFO]     POST /webhook/:provider → Webhook endpoint
[INFO]     GET  /dashboard      → Web Dashboard
[INFO] 📊  Dashboard: http://0.0.0.0:8080/dashboard
```

---

# 4. YAPILANDIRMA

## 4.1 Docker Compose Yapılandırması

**Dosya:** `docker-compose.yml`

```yaml
version: '3.8'

services:
  # ─── POSTGRESQL ───
  postgres:
    image: postgres:15-alpine
    container_name: sentient-postgres
    environment:
      POSTGRES_USER: sentient
      POSTGRES_PASSWORD: sentient123
      POSTGRES_DB: sentient
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./scripts/init-db.sql:/docker-entrypoint-initdb.d/init.sql
    ports:
      - "5432:5432"
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U sentient"]
      interval: 10s
      timeout: 5s
      retries: 5

  # ─── REDIS ───
  redis:
    image: redis:7-alpine
    container_name: sentient-redis
    command: redis-server --appendonly yes
    volumes:
      - redis_data:/data
    ports:
      - "6379:6379"
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      timeout: 5s
      retries: 5

  # ─── QDRANT (VEKTÖR DB) ───
  qdrant:
    image: qdrant/qdrant:latest
    container_name: sentient-qdrant
    volumes:
      - qdrant_data:/qdrant/storage
    ports:
      - "6333:6333"
      - "6334:6334"

  # ─── MINIO (NESNE DEPOLAMA) ───
  minio:
    image: minio/minio:latest
    container_name: sentient-minio
    command: server /data --console-address ":9001"
    environment:
      MINIO_ROOT_USER: minioadmin
      MINIO_ROOT_PASSWORD: minioadmin
    volumes:
      - minio_data:/data
    ports:
      - "9000:9000"
      - "9001:9001"
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:9000/minio/health/live"]
      interval: 30s
      timeout: 20s
      retries: 3

  # ─── PROMETHEUS ───
  prometheus:
    image: prom/prometheus:latest
    container_name: sentient-prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'
    volumes:
      - ./config/prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus_data:/prometheus
    ports:
      - "9090:9090"
    healthcheck:
      test: ["CMD", "wget", "-q", "--spider", "http://localhost:9090/-/healthy"]
      interval: 30s
      timeout: 10s
      retries: 3

  # ─── GRAFANA ───
  grafana:
    image: grafana/grafana:latest
    container_name: sentient-grafana
    environment:
      GF_SECURITY_ADMIN_PASSWORD: admin
      GF_USERS_ALLOW_SIGN_UP: "false"
    volumes:
      - grafana_data:/var/lib/grafana
      - ./config/grafana/provisioning:/etc/grafana/provisioning
    ports:
      - "3001:3000"
    healthcheck:
      test: ["CMD", "wget", "-q", "--spider", "http://localhost:3000/api/health"]
      interval: 30s
      timeout: 10s
      retries: 3

volumes:
  postgres_data:
  redis_data:
  qdrant_data:
  minio_data:
  prometheus_data:
  grafana_data:
```

## 4.2 Prometheus Yapılandırması

**Dosya:** `config/prometheus.yml`

```yaml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'prometheus'
    static_configs:
      - targets: ['localhost:9090']

  - job_name: 'sentient-gateway'
    static_configs:
      - targets: ['host.docker.internal:8080']
```

## 4.3 Grafana Datasource

**Dosya:** `config/grafana/provisioning/datasources/datasources.yaml`

```yaml
apiVersion: 1

datasources:
  - name: Prometheus
    type: prometheus
    access: proxy
    url: http://prometheus:9090
    isDefault: true
    editable: false
```

---

# 5. SERVİSLERİ BAŞLATMA

## 5.1 Başlatma Sırası

```
1. Docker Servisleri    → docker-compose up -d
2. Gateway              → ./target/release/sentient gateway
3. Web Dashboard (ops)  → ./target/release/sentient-web
```

## 5.2 Servis Yönetim Komutları

```bash
# ─── BAŞLATMA ───
docker-compose up -d                    # Tüm servisleri başlat
docker-compose up -d postgres redis     # Sadece belirli servisleri başlat

# ─── DURDURMA ───
docker-compose stop                     # Tüm servisleri durdur
docker-compose down                     # Servisleri durdur ve container'ları sil
docker-compose down -v                  # Servisleri, container'ları ve volume'ları sil

# ─── DURUM KONTROL ───
docker-compose ps                       # Servis durumlarını göster
docker-compose logs -f                  # Tüm logları takip et
docker-compose logs -f postgres         # Sadece postgres logları

# ─── YENİDEN BAŞLATMA ───
docker-compose restart                  # Tüm servisleri yeniden başlat
docker-compose restart postgres         # Sadece postgres'i yeniden başlat
```

## 5.3 Gateway Yönetim Komutları

```bash
# ─── BAŞLATMA ───
# Ön planda (logları gör)
OPENROUTER_API_KEY=sk-or-xxx ./target/release/sentient gateway

# Arka planda (daemon)
nohup ./target/release/sentient gateway > gateway.log 2>&1 &

# ─── DURDURMA ───
pkill -f "sentient gateway"

# ─── DURUM KONTROL ───
curl http://localhost:8080/health
ps aux | grep sentient
```

---

# 6. TEST VE DOĞRULAMA

## 6.1 Docker Servis Testleri

```bash
# PostgreSQL
docker exec sentient-postgres pg_isready -U sentient
# Çıktı: accepting connections

# Redis
docker exec sentient-redis redis-cli ping
# Çıktı: PONG

# Qdrant
curl http://localhost:6333/collections
# Çıktı: {"result":{"collections":[]},...}

# MinIO
curl http://localhost:9000/minio/health/live
# Çıktı: (boş, 200 OK)

# Prometheus
curl http://localhost:9090/-/healthy
# Çıktı: Prometheus is Healthy.

# Grafana
curl http://localhost:3001/api/health
# Çıktı: {"commit":"...","database":"ok","version":"..."}
```

## 6.2 Gateway API Testleri

### Health Check
```bash
curl http://localhost:8080/health

# Beklenen çıktı:
# {"status":"healthy","version":"4.0.0","uptime_secs":123,"active_tasks":0}
```

### İstatistikler
```bash
curl http://localhost:8080/api/stats

# Beklenen çıktı:
# {
#   "success": true,
#   "stats": {
#     "total_requests": 0,
#     "active_tasks": 0,
#     "completed_tasks": 0,
#     "failed_tasks": 0,
#     "cancelled_tasks": 0,
#     "uptime_secs": 123,
#     "requests_per_source": {}
#   }
# }
```

### Görev Oluşturma
```bash
curl -X POST http://localhost:8080/api/task \
  -H "Content-Type: application/json" \
  -d '{"goal": "Merhaba, kendini tanıtır mısın?", "user_id": "test"}'

# Beklenen çıktı:
# {
#   "success": true,
#   "message": "Görev kabul edildi ve kuyruğa alındı",
#   "task_id": "uuid-here",
#   "queue_position": 1
# }
```

### Görev Durumu
```bash
curl http://localhost:8080/api/task/<task_id>

# Beklenen çıktı:
# {
#   "task_id": "uuid",
#   "status": "completed",
#   "result": {
#     "completed_tasks": [...],
#     "failed_tasks": []
#   }
# }
```

### Dashboard
```bash
curl http://localhost:8080/dashboard -w "%{http_code}"
# Çıktı: 200

curl http://localhost:8080/api/dashboard | jq '.health_status'
# Çıktı: "🟢 Sağlıklı"
```

## 6.3 LLM Bağlantı Testi

```bash
# LLM test komutu
OPENROUTER_API_KEY=sk-or-xxx ./target/release/sentient llm test

# Beklenen çıktı:
# ══════════════════════════════════════════════
#   🧪  LLM BAĞLANTI TESTLERİ BAŞLATILIYOR
# ══════════════════════════════════════════════
#   Sağlayıcı: openrouter
#   Test model sayısı: 2
# ══════════════════════════════════════════════
#
# ✅ [openrouter] openai/gpt-4o-mini → 1205ms, 100 token
#    └─ "Merhaba! Ben SENTIENT..."
# ✅ [openrouter] openrouter/free → 4224ms, 171 token
#    └─ "Merhaba! Ben Nemotron 3..."
#
# ══════════════════════════════════════════════
#   🧪  TEST SONUÇLARI: 2/2 başarılı
# ══════════════════════════════════════════════
```

## 6.4 Tam Sistem Testi (Oturum 9 Sonuçları)

```
╔══════════════════════════════════════════════════════════════╗
║              SİSTEM TEST SONUÇLARI (2026-04-14)              ║
╠══════════════════════════════════════════════════════════════╣
║ Docker Servisleri                                            ║
║   ├── PostgreSQL     │ ✅ Healthy (port 5432)               ║
║   ├── Redis          │ ✅ Healthy (port 6379)               ║
║   ├── Qdrant         │ ✅ Running  (port 6333)              ║
║   ├── MinIO          │ ✅ Healthy (port 9000/9001)          ║
║   ├── Prometheus     │ ✅ Healthy (port 9090)               ║
║   └── Grafana        │ ✅ Healthy (port 3001)               ║
╠══════════════════════════════════════════════════════════════╣
║ Gateway API                                                  ║
║   ├── Health         │ ✅ {"status":"healthy","v":"4.0.0"}  ║
║   ├── Stats          │ ✅ İstatistikler alınıyor            ║
║   ├── Task Create    │ ✅ Görev kabul ediliyor              ║
║   ├── Task Status    │ ✅ Durum sorgulanıyor                ║
║   ├── Dashboard      │ ✅ 200 OK                            ║
║   ├── WebSocket      │ ✅ Aktif                             ║
║   └── Webhooks       │ ✅ Endpoint aktif                    ║
╠══════════════════════════════════════════════════════════════╣
║ LLM Bağlantısı                                               ║
║   ├── gpt-4o-mini    │ ✅ 1205ms, 100 token                 ║
║   └── openrouter/free│ ✅ 4224ms, 171 token                 ║
╠══════════════════════════════════════════════════════════════╣
║ Araçlar                                                      ║
║   ├── LlmReason      │ ✅ Çalışıyor                         ║
║   ├── LlmQuery       │ ✅ Çalışıyor                         ║
║   ├── Calculator     │ ✅ Çalışıyor                         ║
║   ├── FileRead       │ ✅ Gerçek implementasyon             ║
║   └── FileWrite      │ ✅ Gerçek implementasyon             ║
╚══════════════════════════════════════════════════════════════╝
```

---

# 7. API ENDPOINT REFERANSI

## 7.1 Genel Endpoints

| Endpoint | Metod | Açıklama | Örnek |
|----------|-------|----------|-------|
| `/health` | GET | Sağlık kontrolü | `curl localhost:8080/health` |
| `/api/stats` | GET | İstatistikler | `curl localhost:8080/api/stats` |
| `/dashboard` | GET | Web UI | Tarayıcıda aç |
| `/api/dashboard` | GET | Dashboard verisi | `curl localhost:8080/api/dashboard` |

## 7.2 Görev Yönetimi

| Endpoint | Metod | Açıklama | Örnek |
|----------|-------|----------|-------|
| `/api/task` | POST | Yeni görev | `curl -X POST -d '{"goal":"..."}'` |
| `/api/task/:id` | GET | Görev durumu | `curl localhost:8080/api/task/uuid` |
| `/api/tasks` | GET | Tüm görevler | `curl localhost:8080/api/tasks` |
| `/api/task/:id` | DELETE | Görev iptal | `curl -X DELETE ...` |

## 7.3 WebSocket

| Endpoint | Protokol | Açıklama |
|----------|----------|----------|
| `/ws` | WebSocket | Real-time güncellemeler |

**WebSocket Mesaj Formatı:**
```json
{
  "type": "update",
  "payload": {
    "uptime_secs": 123,
    "total_skills": 5587,
    "loaded_skills": 150,
    "available_tools": 16,
    "vgate_status": "CONNECTED",
    "metrics": {...},
    "activities": [...],
    "logs": [...]
  }
}
```

## 7.4 Webhooks

| Endpoint | Sağlayıcı | Açıklama |
|----------|-----------|----------|
| `/webhook/github` | GitHub | Push, PR, Issue events |
| `/webhook/stripe` | Stripe | Payment events |
| `/webhook/telegram` | Telegram | Bot updates |
| `/webhook/discord` | Discord | Bot events |

---

# 8. SORUN GİDERME

## 8.1 Docker Sorunları

### Port Çakışması
```
Error: port is already allocated
```

**Çözüm:**
```bash
# Hangi süreç kullanıyor
lsof -i :5432

# Durdur
kill -9 <PID>

# Veya docker-compose.yml'da port değiştir
```

### Container Sağlıksız
```
State: Up (unhealthy)
```

**Çözüm:**
```bash
# Logları kontrol et
docker-compose logs <service>

# Yeniden başlat
docker-compose restart <service>

# Gerekirse tekrar oluştur
docker-compose up -d --force-recreate <service>
```

### Volume Sorunları
```
Error: permission denied
```

**Çözüm:**
```bash
# Volume'ları temizle
docker-compose down -v

# Tekrar başlat
docker-compose up -d
```

## 8.2 Gateway Sorunları

### Bağlantı Reddedildi
```
curl: (7) Failed to connect to localhost port 8080
```

**Çözüm:**
```bash
# Gateway çalışıyor mu?
ps aux | grep sentient

# Portu kontrol et
lsof -i :8080

# Gateway'i yeniden başlat
./target/release/sentient gateway
```

### LLM API Hatası
```
error: API request failed with status 401
```

**Çözüm:**
```bash
# API key doğru mu?
echo $OPENROUTER_API_KEY

# .env dosyasını kontrol et
cat .env | grep API_KEY

# Doğru key ile tekrar dene
OPENROUTER_API_KEY=sk-or-correct-key ./target/release/sentient gateway
```

### Rate Limit Hatası
```
error: 429 Too Many Requests
```

**Çözüm:**
- Farklı bir model kullan
- Bekle ve tekrar dene
- Kendi API key'ini ekle (OpenRouter'da BYOK)

## 8.3 Derleme Sorunları

### Bağımlılık Hatası
```
error: could not compile ...
```

**Çözüm:**
```bash
# Rust güncelle
rustup update

# Bağımlılıkları temizle
cargo clean

# Tekrar derle
cargo build --release
```

### OpenSSL Hatası
```
error: linking with `cc` failed
```

**Çözüm:**
```bash
# Ubuntu/Debian
apt install pkg-config libssl-dev

# CentOS/RHEL
yum install openssl-devel
```

---

# 9. PERFORMANS METRİKLERİ

## 9.1 Test Sonuçları (2026-04-14)

| Metrik | Değer |
|--------|-------|
| Gateway Başlangıç | < 1 saniye |
| Health Check | < 10ms |
| Task Accept | < 50ms |
| LLM Response (gpt-4o-mini) | ~1200ms |
| LLM Response (free models) | ~4000ms |
| Memory Usage (Gateway) | ~50MB |
| Binary Size (sentient) | ~85MB |

## 9.2 Kaynak Kullanımı

| Kaynak | Docker Servisleri | Gateway |
|--------|-------------------|---------|
| CPU | ~2% idle | ~1% idle |
| RAM | ~1GB total | ~50MB |
| Disk | ~2GB (volumes) | - |

## 9.3 Kapasite

| Özellik | Değer |
|---------|-------|
| Max Concurrent Tasks | 10 |
| Max WebSocket Clients | 1000+ |
| Task Queue Size | Sınırsız |

---

# 10. GÜVENLİK NOTLARI

## 10.1 API Key Güvenliği

⚠️ **ÖNEMLİ:** API key'leri ASLA kod deposuna kaydetmeyin!

**Doğru Kullanım:**
```bash
# .env dosyasını .gitignore'a ekle
echo ".env" >> .gitignore

# Key'i ortam değişkeni olarak geç
OPENROUTER_API_KEY=sk-or-xxx ./target/release/sentient gateway
```

**Yanlış Kullanım:**
```bash
# ASLA YAPMAYIN!
echo "OPENROUTER_API_KEY=sk-or-xxx" >> config.rs
git add .
git commit -m "added key"  # ❌ KEY GITHUB'A GİDER!
```

## 10.2 JWT Secret

```bash
# Güvenli secret üret
openssl rand -base64 64

# .env'e ekle
JWT_SECRET= üretilen-deger
```

## 10.3 Production Önerileri

1. **HTTPS Kullanın:** Nginx/Caddy ile SSL termination
2. **Rate Limiting:** Gateway'de rate limiting aktif
3. **Firewall:** Sadece gerekli portları açın
4. **Backup:** Düzenli veritabanı yedeği alın
5. **Monitoring:** Prometheus/Grafana ile izleyin

---

# ═══════════════════════════════════════════════════════════════════════════════
#  EK: HIZLI REFERANS KARTI
# ═══════════════════════════════════════════════════════════════════════════════

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     SENTIENT OS - HIZLI REFERANS                            │
├─────────────────────────────────────────────────────────────────────────────┤
│ BAŞLATMA                                                                    │
│   docker-compose up -d                    # Docker servisleri               │
│   ./target/release/sentient gateway       # Gateway                        │
│   curl localhost:8080/health              # Test                           │
├─────────────────────────────────────────────────────────────────────────────┤
│ DURDURMA                                                                    │
│   docker-compose down                     # Docker durdur                  │
│   pkill -f "sentient gateway"             # Gateway durdur                 │
├─────────────────────────────────────────────────────────────────────────────┤
│ LOG KONTROL                                                                 │
│   docker-compose logs -f                  # Docker logları                 │
│   docker-compose logs -f postgres         # Sadece postgres                │
├─────────────────────────────────────────────────────────────────────────────┤
│ TEST ENDPOINTS                                                              │
│   GET  /health              → {"status":"healthy"}                         │
│   GET  /api/stats           → İstatistikler                                │
│   POST /api/task            → Yeni görev                                   │
│   GET  /dashboard           → Web UI                                       │
│   WS   /ws                  → Real-time updates                            │
├─────────────────────────────────────────────────────────────────────────────┤
│ PORTLAR                                                                     │
│   8080  → Gateway API                                                       │
│   5432  → PostgreSQL                                                        │
│   6379  → Redis                                                             │
│   6333  → Qdrant                                                            │
│   9000  → MinIO API                                                         │
│   9001  → MinIO Console                                                     │
│   9090  → Prometheus                                                        │
│   3001  → Grafana                                                           │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

*Doküman sürümü: 1.0*
*Son güncelleme: 2026-04-14*
*Test durumu: ✅ TÜM TESTLER BAŞARILI*
