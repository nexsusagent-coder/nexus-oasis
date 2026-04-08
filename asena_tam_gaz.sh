#!/bin/bash
# ─── SENTIENT TAM GAZ ───
# SENTIENT projesini başlatır

# .env dosyasını kontrol et
if [ ! -f ".env" ]; then
    echo "⚠️  .env dosyası bulunamadı!"
    echo "📋  .env.example dosyasını .env olarak kopyalayıp düzenleyin."
    cp .env.example .env 2>/dev/null || true
    echo ""
    echo "🔧  Lütfen .env dosyasını düzenleyin:"
    echo "   nano .env"
    echo ""
    exit 1
fi

# Logları göster
echo "══════════════════════════════════════════════"
echo "  🧠  SENTIENT OS - The Operating System That Thinks"
echo "══════════════════════════════════════════════"
echo ""

# SENTIENT CLI'yi çalıştır
cargo run --bin sentient -- "$@"
