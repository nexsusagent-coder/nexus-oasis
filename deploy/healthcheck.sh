#!/bin/bash
# ─── SENTIENT HEALTH CHECK ───
# Tüm servislerin sağlık kontrolü

set -e

# Renkler
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}🐺  SENTIENT HEALTH CHECK${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
echo ""

ALL_OK=true

# ─── Systemd Servisleri ───
echo -e "${YELLOW}[1] SYSTEMD SERVİSİ${NC}"

if systemctl is-active --quiet sentient 2>/dev/null; then
    echo -e "  ${GREEN}✓${NC} SENTIENT Service: ${GREEN}ÇALIŞIYOR${NC}"
else
    echo -e "  ${RED}✗${NC} SENTIENT Service: ${RED}DURDURULDU${NC}"
    ALL_OK=false
fi

echo ""

# ─── HTTP Endpoints ───
echo -e "${YELLOW}[2] HTTP ENDPOINTS${NC}"

check_http() {
    local url=$1
    local name=$2
    
    if curl -sf --connect-timeout 5 "$url" > /dev/null 2>&1; then
        echo -e "  ${GREEN}✓${NC} $name: ${GREEN}ERİŞİLEBİLİR${NC}"
        return 0
    else
        echo -e "  ${RED}✗${NC} $name: ${RED}ERİŞİLEMEZ${NC}"
        return 1
    fi
}

check_http "http://localhost:8080/health" "Gateway Health" || ALL_OK=false
check_http "http://localhost:8080/api/stats" "Gateway Stats" || true

echo ""

# ─── Bellek Durumu ───
echo -e "${YELLOW}[3] BELLEK DURUMU${NC}"

DB_PATH="/root/SENTIENT_CORE/data/sentient.db"
if [ -f "$DB_PATH" ]; then
    SIZE=$(du -h "$DB_PATH" | cut -f1)
    echo -e "  ${GREEN}✓${NC} Veritabanı: ${GREEN}$SIZE${NC}"
else
    echo -e "  ${YELLOW}!${NC} Veritabanı bulunamadı"
fi

echo ""

# ─── Disk Kullanımı ───
echo -e "${YELLOW}[4] DİSK KULLANIMI${NC}"

DISK_USAGE=$(df -h /root | awk 'NR==2 {print $5}' | tr -d '%')
if [ "$DISK_USAGE" -lt 80 ]; then
    echo -e "  ${GREEN}✓${NC} Disk: ${GREEN}${DISK_USAGE}%${NC}"
elif [ "$DISK_USAGE" -lt 90 ]; then
    echo -e "  ${YELLOW}!${NC} Disk: ${YELLOW}${DISK_USAGE}% (Uyarı)${NC}"
else
    echo -e "  ${RED}✗${NC} Disk: ${RED}${DISK_USAGE}% (Kritik!)${NC}"
fi

echo ""

# ─── Bellek Kullanımı ───
echo -e "${YELLOW}[5] RAM KULLANIMI${NC}"

RAM_USAGE=$(free | awk '/Mem:/ {printf "%.0f", ($3/$2) * 100}')
if [ "$RAM_USAGE" -lt 80 ]; then
    echo -e "  ${GREEN}✓${NC} RAM: ${GREEN}${RAM_USAGE}%${NC}"
elif [ "$RAM_USAGE" -lt 90 ]; then
    echo -e "  ${YELLOW}!${NC} RAM: ${YELLOW}${RAM_USAGE}% (Uyarı)${NC}"
else
    echo -e "  ${RED}✗${NC} RAM: ${RED}${RAM_USAGE}% (Kritik!)${NC}"
fi

echo ""

# ─── Log Durumu ───
echo -e "${YELLOW}[6] LOG DURUMU${NC}"

LOG_DIR="/root/SENTIENT_CORE/data/logs"
if [ -d "$LOG_DIR" ]; then
    LOG_COUNT=$(find "$LOG_DIR" -name "*.log" 2>/dev/null | wc -l)
    LOG_SIZE=$(du -sh "$LOG_DIR" 2>/dev/null | cut -f1)
    echo -e "  ${GREEN}✓${NC} Log dosyaları: $LOG_COUNT adet ($LOG_SIZE)"
else
    echo -e "  ${YELLOW}!${NC} Log dizini yok"
fi

echo ""

# ─── Sonuç ───
echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"

if [ "$ALL_OK" = true ]; then
    echo -e "${GREEN}✅ TÜM SERVİSLER SAĞLIKLI${NC}"
    echo ""
    echo "Endpoints:"
    echo "  Dashboard:   http://localhost:8080/dashboard"
    echo "  Claw3D:      http://localhost:8080/claw3d"
    echo "  Memory:      http://localhost:8080/memory"
    exit 0
else
    echo -e "${RED}❌ BAZI SERVİSLER SORUNLU${NC}"
    echo ""
    echo "Sorun giderme:"
    echo "  sudo systemctl status sentient"
    echo "  sudo journalctl -u sentient -f"
    echo "  sudo systemctl restart sentient"
    exit 1
fi
