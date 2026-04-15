#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS - Stop Script
# ═══════════════════════════════════════════════════════════════════════════════
#  Kullanım: ./scripts/stop.sh [--all]
#  --all: Tüm Docker container'ları durdur
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# Renk kodları
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}"
echo "╔══════════════════════════════════════════════════════════════════════════════╗"
echo "║                         🛑 SENTIENT OS - Durduruluyor                        ║"
echo "╚══════════════════════════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# Dizine git
cd "$(dirname "$0")/.."
SENTIENT_DIR=$(pwd)

# Docker Compose komutu
if docker compose version &> /dev/null; then
    DOCKER_COMPOSE="docker compose"
else
    DOCKER_COMPOSE="docker-compose"
fi

# Gateway'i durdur
echo -e "${BLUE}🚪 Gateway durduruluyor...${NC}"

# PID dosyasından
if [ -f /tmp/sentient-gateway.pid ]; then
    GATEWAY_PID=$(cat /tmp/sentient-gateway.pid)
    if kill -0 $GATEWAY_PID 2>/dev/null; then
        kill $GATEWAY_PID
        echo -e "${GREEN}   ✅ Gateway durduruldu (PID: $GATEWAY_PID)${NC}"
    else
        echo -e "${YELLOW}   ⚠️  Gateway zaten durmuş${NC}"
    fi
    rm /tmp/sentient-gateway.pid
else
    # Process ara ve öldür
    pkill -f "sentient-gateway" 2>/dev/null && echo -e "${GREEN}   ✅ Gateway durduruldu${NC}" || echo -e "${YELLOW}   ⚠️  Gateway bulunamadı${NC}"
fi

echo ""

# Docker servislerini durdur
echo -e "${BLUE}🐳 Docker servisleri durduruluyor...${NC}"
echo ""

if [ "$1" == "--all" ]; then
    echo -e "${YELLOW}📋 Tüm container'lar durduruluyor${NC}"
    $DOCKER_COMPOSE down
    echo -e "${GREEN}✅ Tüm container'lar durduruldu${NC}"
else
    echo -e "${CYAN}📋 Container'lar durduruluyor (veriler korunuyor)${NC}"
    $DOCKER_COMPOSE stop
    echo -e "${GREEN}✅ Container'lar durduruldu${NC}"
    echo ""
    echo -e "${YELLOW}💡 Verileri de silmek için: ./scripts/stop.sh --all${NC}"
    echo -e "${YELLOW}   veya: docker-compose down -v${NC}"
fi

echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  ✅ SENTIENT OS Durduruldu!${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${CYAN}Yeniden başlatmak için: ./scripts/start.sh${NC}"
