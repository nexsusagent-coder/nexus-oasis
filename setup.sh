#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
#  🧠 SENTIENT OS - Setup Bootstrap Script
#  Interactive TUI Wizard with Arrow-Key Navigation
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# ─────────────────────────────────────────────────────────────────────────────
# COLORS
# ─────────────────────────────────────────────────────────────────────────────
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BOLD='\033[1m'
NC='\033[0m'

# ─────────────────────────────────────────────────────────────────────────────
# WELCOME
# ─────────────────────────────────────────────────────────────────────────────

echo -e "${CYAN}"
echo "╔══════════════════════════════════════════════════════════════════════════════╗"
echo "║                                                                              ║"
echo "║   ███████╗██╗███████╗███╗   ██╗██████╗ ███████╗██████╗ ██████╗ ██╗███╗   ██╗ ║"
echo "║   ██╔════╝██║██╔════╝████╗  ██║██╔══██╗██╔════╝██╔══██╗██╔══██╗██║████╗  ██║ ║"
echo "║   ███████╗██║█████╗  ██╔██╗ ██║██║  ██║█████╗  ██████╔╝██████╔╝██║██╔██╗ ██║ ║"
echo "║   ╚════██║██║██╔══╝  ██║╚██╗██║██║  ██║██╔══╝  ██╔══██╗██╔══██╗██║██║╚██╗██║ ║"
echo "║   ███████║██║███████╗██║ ╚████║██████╔╝███████╗██║  ██║██║  ██║██║██║ ╚████║ ║"
echo "║   ╚══════╝╚═╝╚══════╝╚═╝  ╚═══╝╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝ ║"
echo "║                                                                              ║"
echo "║                     ${BOLD}🧠 The Operating System That Thinks${NC}${CYAN}                     ║"
echo "║                                                                              ║"
echo "║                     🎮 Interactive TUI Setup Wizard                         ║"
echo "║                     ↑↓ Navigate    Space: Select    Enter: Confirm         ║"
echo "║                                                                              ║"
echo "╚══════════════════════════════════════════════════════════════════════════════╝"
echo -e "${NC}"
echo ""

# ─────────────────────────────────────────────────────────────────────────────
# DEPENDENCY CHECK
# ─────────────────────────────────────────────────────────────────────────────

echo -e "${BOLD}📋 Sistem Kontrolü...${NC}"
echo ""

# Check Rust
if ! command -v rustc &> /dev/null; then
    echo -e "${YELLOW}⚠️  Rust bulunamadı. Kuruluyor...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo -e "${GREEN}✅ Rust kuruldu${NC}"
else
    echo -e "${GREEN}✅ Rust: $(rustc --version)${NC}"
fi

# Check Cargo
if ! command -v cargo &> /dev/null; then
    source "$HOME/.cargo/env" 2>/dev/null || true
fi

# Check Git
if ! command -v git &> /dev/null; then
    echo -e "${YELLOW}⚠️  Git bulunamadı. Lütfen kurun: apt install git / brew install git${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Git: $(git --version)${NC}"

echo ""

# ─────────────────────────────────────────────────────────────────────────────
# CLONE REPO (if needed)
# ─────────────────────────────────────────────────────────────────────────────

INSTALL_DIR="${SENTIENT_DIR:-$HOME/sentient}"

if [ ! -d "$INSTALL_DIR" ]; then
    echo -e "${BOLD}📥 SENTIENT indiriliyor...${NC}"
    git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git "$INSTALL_DIR"
    echo -e "${GREEN}✅ Depo klonlandı: $INSTALL_DIR${NC}"
else
    echo -e "${GREEN}✅ Depo mevcut: $INSTALL_DIR${NC}"
    cd "$INSTALL_DIR"
    echo -e "${BOLD}📥 Güncellemeler kontrol ediliyor...${NC}"
    git pull || true
fi

cd "$INSTALL_DIR"

# ─────────────────────────────────────────────────────────────────────────────
# BUILD TUI WIZARD
# ─────────────────────────────────────────────────────────────────────────────

echo ""
echo -e "${BOLD}🔨 TUI Sihirbazı derleniyor...${NC}"
echo "   (Bu işlem birkaç dakika sürebilir...)"
echo ""

cargo build --release --bin sentient-setup

if [ $? -eq 0 ]; then
    echo ""
    echo -e "${GREEN}✅ TUI Sihirbazı derlendi!${NC}"
else
    echo ""
    echo -e "${YELLOW}⚠️  Derleme başarısız oldu!${NC}"
    echo ""
    echo "Alternatif kurulum yöntemleri:"
    echo ""
    echo "1. Docker ile:"
    echo "   docker run -it ghcr.io/nexsusagent-coder/sentient:latest"
    echo ""
    echo "2. Binary indir:"
    echo "   https://github.com/nexsusagent-coder/SENTIENT_CORE/releases"
    echo ""
    exit 1
fi

# ─────────────────────────────────────────────────────────────────────────────
# RUN TUI WIZARD
# ─────────────────────────────────────────────────────────────────────────────

echo ""
echo -e "${CYAN}🎮 Interactive TUI Sihirbazı başlatılıyor...${NC}"
echo "   ↑↓ Ok tuşlarıyla gezinin"
echo "   Space ile çoklu seçim yapın"
echo "   Enter ile onaylayın"
echo ""

sleep 1

# Run the interactive TUI wizard
SETUP_EXE="$INSTALL_DIR/target/release/sentient-setup"
if [ -f "$SETUP_EXE" ]; then
    "$SETUP_EXE"
else
    echo -e "${YELLOW}⚠️  Setup executable bulunamadı${NC}"
fi

# ─────────────────────────────────────────────────────────────────────────────
# DONE
# ─────────────────────────────────────────────────────────────────────────────

echo ""
echo -e "${GREEN}╔══════════════════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║  🎉 SENTIENT kurulumu tamamlandı!                                            ║${NC}"
echo -e "${GREEN}╚══════════════════════════════════════════════════════════════════════════════╝${NC}"
echo ""

echo "🚀 Başlatmak için:"
echo -e "   ${CYAN}cd $INSTALL_DIR${NC}"
echo -e "   ${CYAN}./target/release/sentient-shell${NC}"
echo ""

echo "🌐 Dashboard:"
echo -e "   ${CYAN}http://localhost:8080${NC}"
echo ""
