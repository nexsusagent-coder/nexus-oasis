# 🚀 SENTIENT Production Deployment (L9)

## 📋 Genel Bakış

SENTIENT, 7/24 kesintisiz çalışacak şekilde production ortamına deploy edilir.

### Mimari

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         PRODUCTION STACK                                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   ┌─────────────┐     ┌─────────────┐     ┌─────────────┐                 │
│   │   NGINX     │────▶│   GATEWAY   │────▶│  V-GATE     │                 │
│   │  (Port 80)  │     │ (Port 8080) │     │ (Port 1071) │                 │
│   └─────────────┘     └──────┬──────┘     └──────┬──────┘                 │
│                              │                    │                        │
│                              ▼                    ▼                        │
│   ┌──────────────────────────────────────────────────────────────────┐    │
│   │                     SENTIENT CORE                                    │    │
│   │  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐            │    │
│   │  │ Memory  │  │ Orchestr.│  │  Swarm  │  │  Scout  │            │    │
│   │  └─────────┘  └─────────┘  └─────────┘  └─────────┘            │    │
│   └──────────────────────────────────────────────────────────────────┘    │
│                                                                             │
│   ┌──────────────────────────────────────────────────────────────────┐    │
│   │                     SANDBOX (Docker)                              │    │
│   │  ┌───────────────┐     ┌───────────────┐                         │    │
│   │  │ sandbox-python│     │ sandbox-secure│                         │    │
│   │  └───────────────┘     └───────────────┘                         │    │
│   └──────────────────────────────────────────────────────────────────┘    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## 🛠️ Kurulum

### Hızlı Kurulum

```bash
# Production deployment betiğini çalıştır
sudo ./deploy/install-production.sh
```

Bu betik şunları yapar:
1. Binary derler (release mode)
2. Veri dizinlerini oluşturur
3. `.env` dosyasını yapılandırır
4. Systemd servisini kurar
5. Log rotation ayarlar
6. Servisleri başlatır

### Manuel Kurulum

```bash
# 1. Binary derle
cargo build --release

# 2. .env dosyasını oluştur
cp .env.production .env
nano .env  # API anahtarlarını düzenle

# 3. systemd servisini kur
sudo cp deploy/sentient.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable sentient
sudo systemctl start sentient

# 4. Durumu kontrol et
sudo systemctl status sentient
```

## 🔧 Yapılandırma

### `.env` Dosyası

```bash
# ═══════════════════════════════════════════════════════════
#  V-GATE (API Anahtarı Yönetimi)
# ═══════════════════════════════════════════════════════════
V_GATE_URL=http://localhost:8100
V_GATE_LISTEN=127.0.0.1:1071

# API Anahtarları (SUNUCUDA SAKLANIR!)
OPENROUTER_API_KEY=sk-or-v1-xxxx
OPENAI_API_KEY=sk-xxxx
ANTHROPIC_API_KEY=sk-ant-xxxx

# ═══════════════════════════════════════════════════════════
#  GATEWAY
# ═══════════════════════════════════════════════════════════
GATEWAY_HTTP_ADDR=0.0.0.0:8080
JWT_SECRET=your-secret-key-change-this
MAX_CONCURRENT_TASKS=50

# ═══════════════════════════════════════════════════════════
#  LLM
# ═══════════════════════════════════════════════════════════
LLM_DEFAULT_MODEL=qwen/qwen3.6-plus:free
LLM_MAX_TOKENS=4096
LLM_TEMPERATURE=0.7

# ═══════════════════════════════════════════════════════════
#  GUARDRAILS
# ═══════════════════════════════════════════════════════════
GUARDRAILS_MODE=strict
GUARDRAILS_PROMPT_INJECTION=true
GUARDRAILS_DATA_EXFILTRATION=true
```

### systemd Servisi

```ini
[Unit]
Description=SENTIENT - NEXUS OASIS AI Operating System
After=network.target

[Service]
Type=simple
WorkingDirectory=/root/SENTIENT_CORE
EnvironmentFile=/root/SENTIENT_CORE/.env
ExecStart=/root/SENTIENT_CORE/target/release/sentient serve --scout --forge --self-healing
Restart=always
RestartSec=15
MemoryMax=4G

[Install]
WantedBy=multi-user.target
```

## 🐳 Docker Deployment

### Docker Compose

```bash
# Production stack'i başlat
docker-compose -f deploy/docker-compose.prod.yml up -d

# Logları izle
docker-compose -f deploy/docker-compose.prod.yml logs -f

# Durdur
docker-compose -f deploy/docker-compose.prod.yml down
```

### Container'lar

| Container | Port | İşlev |
|-----------|------|-------|
| `gateway` | 8080 | API + WebSocket + Dashboard |
| `vgate` | 1071 | Vekil sunucu (API anahtarları) |
| `sandbox-python` | - | Güvenli kod çalıştırma |
| `sandbox-secure` | - | Yüksek güvenlik sandbox |
| `nginx` | 80/443 | Reverse proxy |
| `prometheus` | 9090 | Monitoring |
| `grafana` | 3000 | Dashboard |

## 🌐 NGINX Reverse Proxy

```nginx
# /etc/nginx/nginx.conf
upstream sentient_gateway {
    server localhost:8080;
}

server {
    listen 80;
    
    # API
    location /api/ {
        proxy_pass http://sentient_gateway;
    }
    
    # WebSocket
    location /ws {
        proxy_pass http://sentient_gateway;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
    
    # Dashboard
    location /dashboard {
        proxy_pass http://sentient_gateway;
    }
    
    # Claw3D
    location /claw3d {
        proxy_pass http://sentient_gateway;
    }
}
```

## 📊 Monitoring

### Health Check

```bash
# Sistem sağlık kontrolü
./deploy/healthcheck.sh

# HTTP health endpoint
curl http://localhost:8080/health
```

### Prometheus Metrikleri

```bash
# Metrikleri görüntüle
curl http://localhost:8080/metrics
```

### Loglar

```bash
# systemd logları
sudo journalctl -u sentient -f

# Log dosyaları
tail -f /root/SENTIENT_CORE/data/logs/sentient.log
```

## 🔄 Bakım

### Servis Yönetimi

```bash
# Durum
sudo systemctl status sentient

# Yeniden başlat
sudo systemctl restart sentient

# Durdur
sudo systemctl stop sentient

# Logları izle
sudo journalctl -u sentient -f
```

### Log Rotation

Loglar otomatik olarak rotate edilir (`/etc/logrotate.d/sentient`):

- Günlük rotation
- 30 gün saklama
- Sıkıştırma

### Backup

```bash
# Veritabanı backup
cp /root/SENTIENT_CORE/data/sentient.db /backup/sentient_$(date +%Y%m%d).db

# Tam backup
tar -czf sentient_backup_$(date +%Y%m%d).tar.gz /root/SENTIENT_CORE/data/
```

## 🔒 Güvenlik

### Firewall

```bash
# UFW ile
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw allow 8080/tcp
sudo ufw enable
```

### API Anahtarı Güvenliği

⚠️ **ÖNEMLİ**: API anahtarları ASLA istemciye gönderilmez!

- Anahtarlar sadece sunucuda `.env` dosyasında tutulur
- `.env` dosyası `.gitignore`'a eklenmiştir
- V-GATE üzerinden tüm istekler proxy edilir

### Guardrails

```bash
# Guardrails modu
GUARDRAILS_MODE=strict      # En sıkı
GUARDRAILS_MODE=normal      # Normal
GUARDRAILS_MODE=permissive  # Gevşek
```

## 📱 Endpoints

### HTTP API

| Method | Endpoint | İşlev |
|--------|----------|-------|
| GET | `/health` | Sağlık kontrolü |
| GET | `/api/stats` | İstatistikler |
| POST | `/api/task` | Görev oluştur |
| GET | `/api/task/:id` | Görev durumu |
| GET | `/api/tasks` | Tüm görevler |
| DELETE | `/api/task/:id` | Görev iptal |

### WebSocket

| Endpoint | İşlev |
|----------|-------|
| `/ws` | Ana WebSocket |
| `/ws/claw3d` | 3D Swarm görselleştirme |
| `/ws/memory` | Memory Bridge |

### Web UI

| URL | Sayfa |
|-----|-------|
| `/dashboard` | Ana kontrol paneli |
| `/claw3d` | 3D Swarm görselleştirme |
| `/memory` | Bellek ısı haritası |

## 🧪 Test

```bash
# Health check
curl http://localhost:8080/health

# Görev oluştur
curl -X POST http://localhost:8080/api/task \
  -H "Content-Type: application/json" \
  -d '{"goal": "Merhaba SENTIENT"}'

# WebSocket test
wscat -c ws://localhost:8080/ws
```

## 📞 Sorun Giderme

### Servis başlamıyor

```bash
# Logları kontrol et
sudo journalctl -u sentient -n 100

# Binary'i kontrol et
./target/release/sentient --version

# Port kullanımını kontrol et
netstat -tlnp | grep 8080
```

### API anahtarları çalışmıyor

```bash
# .env dosyasını kontrol et
cat .env | grep API_KEY

# V-GATE test
./target/release/sentient vgate test
```

### Bellek sorunları

```bash
# Bellek kullanımı
free -h

# Process bellek
ps aux --sort=-%mem | head

# Limitleri kontrol et
cat /etc/systemd/system/sentient.service | grep Memory
```

---

## 🐺 NEXUS OASIS

*🇹🇷 Türkiye'de geliştirildi*

**L9 Production Deployment — Tamamlandı!**
