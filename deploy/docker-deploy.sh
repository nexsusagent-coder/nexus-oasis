#!/bin/bash
# ═══════════════════════════════════════════════════════════════
#  SENTIENT DOCKER PRODUCTION DEPLOYMENT
#  Docker Compose ile 7/24 kesintisiz çalışma
# ═══════════════════════════════════════════════════════════════

set -e

# Renkler
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}"
echo "══════════════════════════════════════════════════════════════"
echo "🐺  SENTIENT DOCKER DEPLOYMENT"
echo "══════════════════════════════════════════════════════════════"
echo -e "${NC}"

cd /root/SENTIENT_CORE

# ─── Docker kontrolü ───
if ! command -v docker &> /dev/null; then
    echo -e "${RED}❌ Docker kurulu değil!${NC}"
    echo "   curl -fsSL https://get.docker.com | sh"
    exit 1
fi

if ! command -v docker-compose &> /dev/null && ! docker compose version &> /dev/null; then
    echo -e "${RED}❌ Docker Compose kurulu değil!${NC}"
    exit 1
fi

# ─── Env dosyası ───
if [ ! -f ".env" ]; then
    echo -e "${YELLOW}📋 .env dosyası oluşturuluyor...${NC}"
    cp .env.production .env
    chmod 600 .env
fi

# ─── Eski container'ları durdur ───
echo -e "${BLUE}[1/4] Eski container'lar durduruluyor...${NC}"
docker-compose -f deploy/docker-compose.prod.yml down --remove-orphans 2>/dev/null || true

# ─── Docker image'ları oluştur ───
echo -e "${BLUE}[2/4] Docker image'lar oluşturuluyor...${NC}"
docker-compose -f deploy/docker-compose.prod.yml build --no-cache

# ─── Container'ları başlat ───
echo -e "${BLUE}[3/4] Container'lar başlatılıyor...${NC}"
docker-compose -f deploy/docker-compose.prod.yml up -d

# ─── Health check ───
echo -e "${BLUE}[4/4] Health check...${NC}"
sleep 10

echo ""
echo -e "${CYAN}══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}✅ DOCKER DEPLOYMENT TAMAMLANDI${NC}"
echo -e "${CYAN}══════════════════════════════════════════════════════════════${NC}"
echo ""

# Container durumları
docker-compose -f deploy/docker-compose.prod.yml ps

echo ""
echo -e "${YELLOW}📋 KOMUTLAR${NC}"
echo "   Loglar:     docker-compose -f deploy/docker-compose.prod.yml logs -f"
echo "   Durdur:     docker-compose -f deploy/docker-compose.prod.yml down"
echo "   Yeniden:    docker-compose -f deploy/docker-compose.prod.yml restart"
echo ""
