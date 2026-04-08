#!/bin/bash
# ═══════════════════════════════════════════════════════════════
#  SENTIENT PRODUCTION DEPLOYMENT
#  L9: NİHAİ ENTEGRASYON - 7/24 KESİNTİSİZ ÇALIŞMA
# ═══════════════════════════════════════════════════════════════
#
# Bu betik SENTIENT'yı production ortamına kurar:
#   - Binary derleme
#   - Systemd servisleri kurulumu
#   - Güvenli .env yapılandırması
#   - Log rotation
#   - Health check
#
# Kullanım:
#   chmod +x deploy/install-production.sh
#   sudo ./deploy/install-production.sh

set -e

# ─── Renkler ───
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# ─── Değişkenler ───
SENTIENT_DIR="/root/SENTIENT_CORE"
SENTIENT_USER="root"
SENTIENT_GROUP="root"

# ─── Başlık ───
echo -e "${CYAN}"
echo "══════════════════════════════════════════════════════════════"
echo "🐺  SENTIENT PRODUCTION DEPLOYMENT"
echo "══════════════════════════════════════════════════════════════"
echo -e "${NC}"
echo ""

# ─── Root kontrolü ───
if [ "$EUID" -ne 0 ]; then
    echo -e "${RED}❌ Bu betik root olarak çalıştırılmalıdır${NC}"
    echo "   sudo ./deploy/install-production.sh"
    exit 1
fi

# ─── Dizin kontrolü ───
if [ ! -d "$SENTIENT_DIR" ]; then
    echo -e "${RED}❌ SENTIENT dizini bulunamadı: $SENTIENT_DIR${NC}"
    exit 1
fi

cd "$SENTIENT_DIR"

# ═══════════════════════════════════════════════════════════════
#  1. BINARY DERLEME
# ═══════════════════════════════════════════════════════════════
echo -e "${BLUE}[1/7] BINARY DERLEME${NC}"
echo "────────────────────────────────────────────────────────────────"

if [ -f "target/release/sentient" ]; then
    BINARY_AGE=$(($(date +%s) - $(stat -c %Y target/release/sentient)))
    if [ $BINARY_AGE -lt 3600 ]; then
        echo -e "${GREEN}✓${NC} Binary zaten güncel (son $(($BINARY_AGE / 60)) dakika önce derlenmiş)"
    else
        echo -e "${YELLOW}!${NC} Binary eski, yeniden derleniyor..."
        cargo build --release
    fi
else
    echo "📦  Release binary derleniyor..."
    cargo build --release
fi

if [ ! -f "target/release/sentient" ]; then
    echo -e "${RED}❌ Binary derlenemedi!${NC}"
    exit 1
fi

echo -e "${GREEN}✓${NC} Binary hazır: $(du -h target/release/sentient | cut -f1)"
echo ""

# ═══════════════════════════════════════════════════════════════
#  2. VERİ DİZİNLERİ
# ═══════════════════════════════════════════════════════════════
echo -e "${BLUE}[2/7] VERİ DİZİNLERİ${NC}"
echo "────────────────────────────────────────────────────────────────"

mkdir -p data/logs
mkdir -p data/memory
mkdir -p data/sandbox
mkdir -p data/forge

chmod 755 data
chmod 755 data/logs
chmod 755 data/memory

echo -e "${GREEN}✓${NC} Dizinler oluşturuldu"
echo ""

# ═══════════════════════════════════════════════════════════════
#  3. ENV DOSYASI
# ═══════════════════════════════════════════════════════════════
echo -e "${BLUE}[3/7] ENV DOSYASI${NC}"
echo "────────────────────────────────────────────────────────────────"

if [ ! -f ".env" ]; then
    if [ -f ".env.production" ]; then
        cp .env.production .env
        chmod 600 .env
        echo -e "${GREEN}✓${NC} .env.production -> .env kopyalandı"
        echo -e "${YELLOW}⚠️  ÖNEMLİ: .env dosyasındaki API anahtarlarını düzenleyin!${NC}"
        echo "   nano /root/SENTIENT_CORE/.env"
    else
        echo -e "${YELLOW}!${NC} .env.production bulunamadı, .env.example kopyalanıyor..."
        cp .env.example .env
        chmod 600 .env
    fi
else
    echo -e "${GREEN}✓${NC} .env zaten mevcut"
    chmod 600 .env
fi

# .gitignore kontrolü
if ! grep -q ".env" .gitignore 2>/dev/null; then
    echo ".env" >> .gitignore
    echo ".env.production" >> .gitignore
    echo -e "${GREEN}✓${NC} .gitignore güncellendi"
fi
echo ""

# ═══════════════════════════════════════════════════════════════
#  4. SYSTEMD SERVİSLERİ
# ═══════════════════════════════════════════════════════════════
echo -e "${BLUE}[4/7] SYSTEMD SERVİSLERİ${NC}"
echo "────────────────────────────────────────────────────────────────"

# Servis dosyalarını kopyala
cp deploy/sentient-vgate.service /etc/systemd/system/
cp deploy/sentient-gateway.service /etc/systemd/system/
cp deploy/sentient-main.service /etc/systemd/system/

# systemd'yi yenile
systemctl daemon-reload

echo -e "${GREEN}✓${NC} Servis dosyaları kopyalandı"
echo ""

# ═══════════════════════════════════════════════════════════════
#  5. LOG ROTATION
# ═══════════════════════════════════════════════════════════════
echo -e "${BLUE}[5/7] LOG ROTATION${NC}"
echo "────────────────────────────────────────────────────────────────"

if [ -f "deploy/logrotate.conf" ]; then
    cp deploy/logrotate.conf /etc/logrotate.d/sentient
    chmod 644 /etc/logrotate.d/sentient
    echo -e "${GREEN}✓${NC} Logrotate yapılandırıldı"
else
    echo -e "${YELLOW}!${NC} logrotate.conf bulunamadı, atlanıyor"
fi
echo ""

# ═══════════════════════════════════════════════════════════════
#  6. SERVİSLERİ BAŞLAT
# ═══════════════════════════════════════════════════════════════
echo -e "${BLUE}[6/7] SERVİSLERİ BAŞLAT${NC}"
echo "────────────────────────────────────────────────────────────────"

# Önce eski servisleri durdur
echo "   Eski servisler durduruluyor..."
systemctl stop sentient-main 2>/dev/null || true
systemctl stop sentient-gateway 2>/dev/null || true
systemctl stop sentient-vgate 2>/dev/null || true

# V-GATE
echo "   V-GATE başlatılıyor..."
systemctl enable sentient-vgate
systemctl start sentient-vgate
sleep 3

# Gateway
echo "   Gateway başlatılıyor..."
systemctl enable sentient-gateway
systemctl start sentient-gateway
sleep 3

# Main
echo "   Main başlatılıyor..."
systemctl enable sentient-main
systemctl start sentient-main
sleep 3

echo ""

# ═══════════════════════════════════════════════════════════════
#  7. HEALTH CHECK
# ═══════════════════════════════════════════════════════════════
echo -e "${BLUE}[7/7] HEALTH CHECK${NC}"
echo "────────────────────────────────────────────────────────────────"

# Servis durumlarını kontrol et
check_service() {
    local service=$1
    local name=$2
    
    if systemctl is-active --quiet $service 2>/dev/null; then
        echo -e "   ${GREEN}✓${NC} $name: ${GREEN}ÇALIŞIYOR${NC}"
        return 0
    else
        echo -e "   ${RED}✗${NC} $name: ${RED}DURDURULDU${NC}"
        return 1
    fi
}

ALL_OK=true

check_service "sentient-vgate" "V-GATE" || ALL_OK=false
check_service "sentient-gateway" "Gateway" || ALL_OK=false
check_service "sentient-main" "Main" || ALL_OK=false

echo ""

# HTTP health check
echo "   HTTP endpoint kontrolü..."
sleep 2

if curl -sf http://localhost:1071/health > /dev/null 2>&1; then
    echo -e "   ${GREEN}✓${NC} V-GATE Health: ${GREEN}OK${NC}"
else
    echo -e "   ${YELLOW}!${NC} V-GATE Health: ${YELLOW}Bekleniyor...${NC}"
fi

if curl -sf http://localhost:8080/health > /dev/null 2>&1; then
    echo -e "   ${GREEN}✓${NC} Gateway Health: ${GREEN}OK${NC}"
else
    echo -e "   ${YELLOW}!${NC} Gateway Health: ${YELLOW}Bekleniyor...${NC}"
fi

echo ""

# ═══════════════════════════════════════════════════════════════
#  SONUÇ
# ═══════════════════════════════════════════════════════════════
echo -e "${CYAN}"
echo "══════════════════════════════════════════════════════════════"
echo "🐺  SENTIENT PRODUCTION DEPLOYMENT TAMAMLANDI"
echo "══════════════════════════════════════════════════════════════"
echo -e "${NC}"
echo ""

echo -e "${GREEN}✅ SERVİSLER AKTİF${NC}"
echo ""
echo "   V-GATE:      http://localhost:1071"
echo "   Gateway:     http://localhost:8080"
echo "   Dashboard:   http://localhost:8080/dashboard"
echo "   Claw3D:      http://localhost:8080/claw3d"
echo "   Memory:      http://localhost:8080/memory"
echo ""

echo -e "${YELLOW}📋 YÖNETİM KOMUTLARI${NC}"
echo ""
echo "   Durum:"
echo "     sudo systemctl status sentient-vgate"
echo "     sudo systemctl status sentient-gateway"
echo "     sudo systemctl status sentient-main"
echo ""
echo "   Loglar:"
echo "     sudo journalctl -u sentient-vgate -f"
echo "     sudo journalctl -u sentient-gateway -f"
echo "     sudo journalctl -u sentient-main -f"
echo ""
echo "   Yeniden başlat:"
echo "     sudo systemctl restart sentient-main"
echo "     sudo systemctl restart sentient-gateway"
echo "     sudo systemctl restart sentient-vgate"
echo ""
echo "   Durdur:"
echo "     sudo systemctl stop sentient-main sentient-gateway sentient-vgate"
echo ""

echo -e "${YELLOW}⚠️  ÖNEMLİ${NC}"
echo ""
echo "   1. API anahtarlarını .env dosyasına ekleyin:"
echo "      nano /root/SENTIENT_CORE/.env"
echo ""
echo "   2. API anahtarlarını ekledikten sonra servisleri yeniden başlatın:"
echo "      sudo systemctl restart sentient-vgate"
echo ""
echo "   3. Firewall ayarlarını kontrol edin:"
echo "      sudo ufw allow 8080/tcp"
echo "      sudo ufw allow 1071/tcp"
echo ""

echo -e "${CYAN}🐺 Bozkurt ruhuyla!${NC}"
