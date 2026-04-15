#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS - Startup Script
# ═══════════════════════════════════════════════════════════════════════════════
#  Kullanım: ./scripts/start.sh [--minimal]
#  --minimal: Sadece temel servisleri başlat (postgres, redis, qdrant)
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# Renk kodları
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Banner
echo -e "${CYAN}"
echo "╔══════════════════════════════════════════════════════════════════════════════╗"
echo "║                         🧠 SENTIENT OS - Başlatılıyor                        ║"
echo "╚══════════════════════════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# Dizine git
cd "$(dirname "$0")/.."
SENTIENT_DIR=$(pwd)

# .env dosyası kontrolü
if [ ! -f ".env" ]; then
    echo -e "${YELLOW}⚠️  .env dosyası bulunamadı!${NC}"
    echo -e "${YELLOW}   .env.template dosyasını kopyalayıp düzenleyin:${NC}"
    echo -e "${CYAN}   cp .env.template .env${NC}"
    echo -e "${YELLOW}   Ardından API key'lerinizi girin.${NC}"
    echo ""
    read -p "Devam etmek için Enter'a basın (varsayılan değerlerle devam edilecek)..."
    cp .env.template .env
    echo -e "${GREEN}✅ .env.template -> .env kopyalandı${NC}"
fi

# .env dosyasını yükle
export $(grep -v '^#' .env | xargs)

# Docker kontrolü
echo -e "${BLUE}🐳 Docker kontrol ediliyor...${NC}"
if ! command -v docker &> /dev/null; then
    echo -e "${RED}❌ Docker kurulu değil!${NC}"
    echo -e "${YELLOW}   Kurulum: https://docs.docker.com/get-docker/${NC}"
    exit 1
fi

if ! docker info &> /dev/null; then
    echo -e "${RED}❌ Docker çalışmıyor!${NC}"
    echo -e "${YELLOW}   sudo systemctl start docker${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Docker hazır${NC}"

# Docker Compose kontrolü
if ! command -v docker-compose &> /dev/null && ! docker compose version &> /dev/null; then
    echo -e "${RED}❌ Docker Compose kurulu değil!${NC}"
    exit 1
fi

# Docker Compose komutu (v2 için "docker compose", v1 için "docker-compose")
if docker compose version &> /dev/null; then
    DOCKER_COMPOSE="docker compose"
else
    DOCKER_COMPOSE="docker-compose"
fi

echo -e "${GREEN}✅ Docker Compose hazır${NC}"
echo ""

# Mod seçimi
MINIMAL=false
if [ "$1" == "--minimal" ]; then
    MINIMAL=true
    echo -e "${YELLOW}📋 MINIMAL MOD: Sadece temel servisler${NC}"
else
    echo -e "${BLUE}📋 FULL MOD: Tüm servisler${NC}"
fi
echo ""

# Servisleri başlat
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${PURPLE}  🚀 Servisler Başlatılıyor${NC}"
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo ""

# Temel servisler
echo -e "${CYAN}┌── 📦 Temel Altyapı${NC}"
echo -e "${CYAN}│${NC}"

echo -e "${CYAN}│  🗄️  PostgreSQL...${NC}"
$DOCKER_COMPOSE up -d postgres
sleep 2

echo -e "${CYAN}│  🔴 Redis...${NC}"
$DOCKER_COMPOSE up -d redis
sleep 2

echo -e "${CYAN}│  🔍 Qdrant (Vector DB)...${NC}"
$DOCKER_COMPOSE up -d qdrant
sleep 2

echo -e "${CYAN}│  📦 MinIO (Object Storage)...${NC}"
$DOCKER_COMPOSE up -d minio
sleep 2

echo -e "${CYAN}│${NC}"
echo -e "${CYAN}└── ✅ Temel servisler hazır${NC}"
echo ""

if [ "$MINIMAL" = false ]; then
    echo -e "${CYAN}┌── 📊 Monitoring & Observability${NC}"
    echo -e "${CYAN}│${NC}"

    echo -e "${CYAN}│  📈 Prometheus...${NC}"
    $DOCKER_COMPOSE up -d prometheus
    sleep 2

    echo -e "${CYAN}│  📊 Grafana...${NC}"
    $DOCKER_COMPOSE up -d grafana
    sleep 2

    echo -e "${CYAN}│${NC}"
    echo -e "${CYAN}└── ✅ Monitoring hazır${NC}"
    echo ""

    echo -e "${CYAN}┌── 🤖 AI Services${NC}"
    echo -e "${CYAN}│${NC}"

    echo -e "${CYAN}│  🦙 Ollama (Local LLM)...${NC}"
    $DOCKER_COMPOSE up -d ollama 2>/dev/null || echo -e "${YELLOW}│  ⚠️  Ollama başlatılamadı (GPU gerekebilir)${NC}"
    sleep 3

    echo -e "${CYAN}│${NC}"
    echo -e "${CYAN}└── ⚠️  Ollama için GPU gerekli (yoksa local çalıştırın)${NC}"
    echo ""

    echo -e "${CYAN}┌── 🔧 Opsiyonel Servisler${NC}"
    echo -e "${CYAN}│${NC}"

    echo -e "${CYAN}│  🔎 SearXNG (Local Search)...${NC}"
    $DOCKER_COMPOSE up -d searxng 2>/dev/null || echo -e "${YELLOW}│  ⚠️  SearXNG atlanıyor${NC}"

    echo -e "${CYAN}│  🐰 RabbitMQ (Message Queue)...${NC}"
    $DOCKER_COMPOSE up -d rabbitmq 2>/dev/null || echo -e "${YELLOW}│  ⚠️  RabbitMQ atlanıyor${NC}"

    echo -e "${CYAN}│${NC}"
    echo -e "${CYAN}└── ✅ Opsiyonel servisler hazır${NC}"
    echo ""
fi

# Health check
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${PURPLE}  🏥 Health Check${NC}"
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo ""

# PostgreSQL health
echo -e "${BLUE}PostgreSQL: ${NC}"
if docker exec sentient-postgres pg_isready -U ${POSTGRES_USER:-sentient} 2>/dev/null; then
    echo -e "${GREEN}  ✅ Çalışıyor${NC}"
else
    echo -e "${YELLOW}  ⏳ Başlatılıyor...${NC}"
fi

# Redis health
echo -e "${BLUE}Redis: ${NC}"
if docker exec sentient-redis redis-cli -a ${REDIS_PASSWORD:-redis_secret} ping 2>/dev/null | grep -q PONG; then
    echo -e "${GREEN}  ✅ Çalışıyor${NC}"
else
    echo -e "${YELLOW}  ⏳ Başlatılıyor...${NC}"
fi

# Qdrant health
echo -e "${BLUE}Qdrant: ${NC}"
if curl -s http://localhost:${QDRANT_PORT:-6333}/health 2>/dev/null | grep -q "ok"; then
    echo -e "${GREEN}  ✅ Çalışıyor${NC}"
else
    echo -e "${YELLOW}  ⏳ Başlatılıyor...${NC}"
fi

# MinIO health
echo -e "${BLUE}MinIO: ${NC}"
if curl -s http://localhost:${MINIO_API_PORT:-9000}/minio/health/live 2>/dev/null | head -1 | grep -q "."; then
    echo -e "${GREEN}  ✅ Çalışıyor${NC}"
else
    echo -e "${YELLOW}  ⏳ Başlatılıyor...${NC}"
fi

echo ""

# Gateway başlat
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${PURPLE}  🚪 API Gateway${NC}"
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo ""

echo -e "${BLUE}Gateway başlatılıyor...${NC}"
echo -e "${CYAN}   cargo run --release --bin sentient-gateway${NC}"
echo ""

# Gateway'i arka planda başlat
cargo run --release --bin sentient-gateway &
GATEWAY_PID=$!

# Gateway başlangıç bekleyişi
echo -n "Gateway başlatılıyor"
for i in {1..10}; do
    sleep 1
    echo -n "."
done
echo ""

# Gateway health check
if curl -s http://localhost:${GATEWAY_PORT:-8080}/health 2>/dev/null | grep -q "healthy"; then
    echo -e "${GREEN}✅ Gateway çalışıyor (PID: $GATEWAY_PID)${NC}"
else
    echo -e "${YELLOW}⚠️  Gateway henüz hazır olmayabilir. Kontrol edin:${NC}"
    echo -e "${CYAN}   curl http://localhost:8080/health${NC}"
fi

echo ""

# Özet
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${PURPLE}  ✅ Sistem Hazır!${NC}"
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo ""

echo -e "${CYAN}📡 Endpoint'ler:${NC}"
echo -e "${CYAN}   API Gateway:  http://localhost:${GATEWAY_PORT:-8080}${NC}"
echo -e "${CYAN}   Dashboard:    http://localhost:${GATEWAY_PORT:-8080}/dashboard${NC}"
echo -e "${CYAN}   Health:       http://localhost:${GATEWAY_PORT:-8080}/health${NC}"
echo -e "${CYAN}   GraphQL:      http://localhost:${GATEWAY_PORT:-8080}/graphql${NC}"
echo ""

if [ "$MINIMAL" = false ]; then
    echo -e "${CYAN}📊 Monitoring:${NC}"
    echo -e "${CYAN}   Prometheus:   http://localhost:${PROMETHEUS_PORT:-9090}${NC}"
    echo -e "${CYAN}   Grafana:      http://localhost:${GRAFANA_PORT:-3001}${NC}"
    echo -e "${CYAN}                  (User: ${GRAFANA_USER:-admin}, Pass: ${GRAFANA_PASSWORD:-admin})${NC}"
    echo ""
fi

echo -e "${CYAN}🗄️  Veritabanları:${NC}"
echo -e "${CYAN}   PostgreSQL:   localhost:${POSTGRES_PORT:-5432}${NC}"
echo -e "${CYAN}   Redis:        localhost:${REDIS_PORT:-6379}${NC}"
echo -e "${CYAN}   Qdrant:       http://localhost:${QDRANT_PORT:-6333}${NC}"
echo -e "${CYAN}   MinIO:        http://localhost:${MINIO_CONSOLE_PORT:-9001}${NC}"
echo ""

echo -e "${CYAN}🤖 Local LLM (Ollama):${NC}"
echo -e "${CYAN}   API:          http://localhost:${OLLAMA_PORT:-11434}${NC}"
echo -e "${YELLOW}   Modelleri indir:${NC}"
echo -e "${CYAN}   ollama pull gemma3:27b${NC}"
echo -e "${CYAN}   ollama pull deepseek-r1:67b${NC}"
echo ""

echo -e "${GREEN}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  🎉 SENTIENT OS Başarıyla Başlatıldı!${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo ""

# PID'i dosyaya kaydet
echo $GATEWAY_PID > /tmp/sentient-gateway.pid

# Durdurmak için
echo -e "${YELLOW}Durdurmak için: ./scripts/stop.sh${NC}"
