#!/bin/bash
# ─── SENTIENT SERVİS KURULUM BETİĞİ ───
# Bu betik SENTIENT'yı systemd servisi olarak kurar

set -e

echo "════════════════════════════════════════════════════════════"
echo "🐺  SENTIENT SERVİS KURULUMU"
echo "════════════════════════════════════════════════════════════"

# Kök dizin kontrolü
if [ "$EUID" -ne 0 ]; then
    echo "❌ Bu betik root olarak çalıştırılmalıdır"
    exit 1
fi

# Dizin kontrolü
SENTIENT_DIR="/root/SENTIENT_CORE"
if [ ! -d "$SENTIENT_DIR" ]; then
    echo "❌ SENTIENT dizini bulunamadı: $SENTIENT_DIR"
    exit 1
fi

cd "$SENTIENT_DIR"

# 1. Binary derle
echo "📦  SENTIENT derleniyor..."
cargo build --release

if [ ! -f "target/release/sentient-cli" ]; then
    echo "❌ Binary derlenemedi"
    exit 1
fi

# 2. Veri dizinlerini oluştur
echo "📁  Veri dizinleri oluşturuluyor..."
mkdir -p data/logs
mkdir -p data/memory
mkdir -p data/sandbox

# 3. systemd servisini kopyala
echo "🔧  systemd servisi kuruluyor..."
cp deploy/sentient.service /etc/systemd/system/

# 4. systemd'yi yenile
systemctl daemon-reload

# 5. Servisi etkinleştir
echo "✅  Servis etkinleştiriliyor..."
systemctl enable sentient

# 6. Servisi başlat
echo "🚀  Servis başlatılıyor..."
systemctl start sentient

# 7. Durumu göster
sleep 2
echo ""
echo "════════════════════════════════════════════════════════════"
echo "📊  SERVİS DURUMU"
echo "════════════════════════════════════════════════════════════"
systemctl status sentient --no-pager

echo ""
echo "════════════════════════════════════════════════════════════"
echo "✅  KURULUM TAMAMLANDI"
echo "════════════════════════════════════════════════════════════"
echo ""
echo "Komutlar:"
echo "  sudo systemctl status sentient    # Durum kontrolü"
echo "  sudo systemctl restart sentient   # Yeniden başlat"
echo "  sudo journalctl -u sentient -f    # Logları izle"
echo "  sudo systemctl stop sentient      # Durdur"
echo ""
