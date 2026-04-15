#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS - HIZLI BAŞLANGIÇ (INTERAKTİFSIZ)
# ═══════════════════════════════════════════════════════════════════════════════
#  Kullanım: ./quick-start.sh
#
#  Bu script minimal kurulum yapar:
#  - Rust (yoksa)
#  - SENTIENT binary
#  - Varsayılan config
#  - Ollama SADECE zaten kuruluysa kullanır
# ═══════════════════════════════════════════════════════════════════════════════

set -e

CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${CYAN}"
echo "╔════════════════════════════════════════════════════════════╗"
echo "║            SENTIENT OS - Quick Start                      ║"
echo "╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Script dizinine geç
cd "$(dirname "$0")"

# 1. RUST
echo -e "${YELLOW}[1/4] Rust...${NC}"
if ! command -v rustc &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env" 2>/dev/null || true
fi
echo -e "${GREEN}✓ $(rustc --version)${NC}"

# 2. DERLEME
echo -e "${YELLOW}[2/4] Derleniyor...${NC}"
if [ ! -f "target/release/sentient" ]; then
    cargo build --release --bin sentient 2>&1 | grep -E "Compiling|Finished" || true
fi
echo -e "${GREEN}✓ target/release/sentient${NC}"

# 3. CONFIG
echo -e "${YELLOW}[3/4] Config...${NC}"
mkdir -p data

if [ ! -f .env ]; then
    cat > .env << 'EOF'
# SENTIENT Config
SENTIENT_LANG=tr
LLM_PROVIDER=ollama
OLLAMA_HOST=http://127.0.0.1:11434
OLLAMA_MODEL=gemma3:4b
MEMORY_DB_PATH=data/sentient_memory.db
VOICE_ENABLED=false
EOF
fi
echo -e "${GREEN}✓ .env${NC}"

# 4. OLLAMA (SADECE KONTROL ET, KURMAYA ZORLAMA)
echo -e "${YELLOW}[4/4] LLM kontrol...${NC}"
if command -v ollama &> /dev/null; then
    # Servis çalışıyor mu?
    if curl -s http://127.0.0.1:11434/api/tags > /dev/null 2>&1; then
        echo -e "${GREEN}✓ Ollama çalışıyor${NC}"
    else
        ollama serve &
        sleep 2
        echo -e "${GREEN}✓ Ollama başlatıldı${NC}"
    fi
else
    echo -e "${YELLOW}⚠ Ollama yok. API key kullanın veya install.sh çalıştırın.${NC}"
    echo "  export OPENAI_API_KEY=sk-..."
fi

# SONUÇ
echo ""
echo -e "${GREEN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║              ✅ HAZIR!                                    ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo "  ./target/release/sentient chat        # Sohbet"
echo "  ./target/release/sentient gateway     # API"
echo "  ./target/release/sentient status      # Durum"
echo ""
