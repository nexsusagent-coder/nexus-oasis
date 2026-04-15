#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS - Health Check Script
# ═══════════════════════════════════════════════════════════════════════════════
#  Kullanım: ./scripts/health-check.sh
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
echo "║                        🏥 SENTIENT OS - Health Check                         ║"
echo "╚══════════════════════════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# .env dosyasını yükle
if [ -f ".env" ]; then
    export $(grep -v '^#' .env | xargs)
fi

# Varsayılan değerler
POSTGRES_PORT=${POSTGRES_PORT:-5432}
REDIS_PORT=${REDIS_PORT:-6379}
QDRANT_PORT=${QDRANT_PORT:-6333}
MINIO_API_PORT=${MINIO_API_PORT:-9000}
OLLAMA_PORT=${OLLAMA_PORT:-11434}
PROMETHEUS_PORT=${PROMETHEUS_PORT:-9090}
GRAFANA_PORT=${GRAFANA_PORT:-3001}
GATEWAY_PORT=${GATEWAY_PORT:-8080}

TOTAL=0
HEALTHY=0

check_service() {
    local name=$1
    local check_cmd=$2
    
    TOTAL=$((TOTAL + 1))
    
    printf "${BLUE}%-20s${NC}" "$name"
    
    if eval "$check_cmd" &>/dev/null; then
        echo -e "${GREEN}✅ SAĞLIKLI${NC}"
        HEALTHY=$((HEALTHY + 1))
        return 0
    else
        echo -e "${RED}❌ ÇALIŞMIYOR${NC}"
        return 1
    fi
}

echo ""
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${PURPLE}  📦 Temel Servisler${NC}"
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo ""

check_service "PostgreSQL" "docker exec sentient-postgres pg_isready -U ${POSTGRES_USER:-sentient}"
check_service "Redis" "docker exec sentient-redis redis-cli -a ${REDIS_PASSWORD:-redis_secret} ping | grep -q PONG"
check_service "Qdrant" "curl -s http://localhost:$QDRANT_PORT/health | grep -q ok"
check_service "MinIO" "curl -s http://localhost:$MINIO_API_PORT/minio/health/live"

echo ""
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${PURPLE}  🚪 API Gateway${NC}"
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo ""

check_service "Gateway" "curl -s http://localhost:$GATEWAY_PORT/health"
check_service "Dashboard" "curl -s http://localhost:$GATEWAY_PORT/dashboard | grep -q SENTIENT"

echo ""
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${PURPLE}  🤖 AI Servisleri${NC}"
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo ""

check_service "Ollama" "curl -s http://localhost:$OLLAMA_PORT/api/tags | grep -q models"

echo ""
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${PURPLE}  📊 Monitoring${NC}"
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo ""

check_service "Prometheus" "curl -s http://localhost:$PROMETHEUS_PORT/-/healthy"
check_service "Grafana" "curl -s http://localhost:$GRAFANA_PORT/api/health"

echo ""
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${PURPLE}  📈 Özet${NC}"
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo ""

if [ $HEALTHY -eq $TOTAL ]; then
    echo -e "${GREEN}✅ Tüm servisler sağlıklı! ($HEALTHY/$TOTAL)${NC}"
else
    echo -e "${YELLOW}⚠️  $HEALTHY/$TOTAL servis sağlıklı${NC}"
    echo -e "${YELLOW}   Çalışmayan servisleri kontrol edin:${NC}"
    echo -e "${CYAN}   docker-compose ps${NC}"
    echo -e "${CYAN}   docker-compose logs [service]${NC}"
fi

echo ""

# Detaylı bilgiler
echo -e "${CYAN}📡 Bağlantı Bilgileri:${NC}"
echo ""
echo -e "   API Gateway:  ${CYAN}http://localhost:$GATEWAY_PORT${NC}"
echo -e "   Dashboard:    ${CYAN}http://localhost:$GATEWAY_PORT/dashboard${NC}"
echo -e "   GraphQL:      ${CYAN}http://localhost:$GATEWAY_PORT/graphql${NC}"
echo -e "   Prometheus:   ${CYAN}http://localhost:$PROMETHEUS_PORT${NC}"
echo -e "   Grafana:      ${CYAN}http://localhost:$GRAFANA_PORT${NC}"
echo -e "   Ollama API:   ${CYAN}http://localhost:$OLLAMA_PORT${NC}"
echo ""

# Docker konteyner durumları
echo -e "${CYAN}🐳 Docker Konteynerleri:${NC}"
echo ""
docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}" 2>/dev/null || echo -e "${YELLOW}   Docker bilgisi alınamadı${NC}"
echo ""

exit $((TOTAL - HEALTHY))
