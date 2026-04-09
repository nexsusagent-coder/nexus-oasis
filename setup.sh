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
echo "╔═══════════════════════════════════════════════════════════════════════════╗"
echo "║                                                                           ║"
echo "║     ███████╗███████╗███╗   ██╗████████╗██╗ ██████╗ █████╗                 ║"
echo "║     ██╔════╝██╔════╝████╗  ██║╚══██╔══╝██║██╔════╝██╔══██╗                ║"
echo "║     ███████╗█████╗  ██╔██╗ ██║   ██║   ██║██║     ███████║                ║"
echo "║     ╚════██║██╔══╝  ██║╚██╗██║   ██║   ██║██║     ██╔══██║                ║"
echo "║     ███████║███████╗██║ ╚████║   ██║   ██║╚██████╗██║  ██║                ║"
echo "║     ╚══════╝╚══════╝╚═╝  ╚═══╝   ╚═╝   ╚═╝ ╚═════╝╚═╝  ╚═╝                ║"
echo "║                                                                           ║"
echo "║              ${BOLD}🧠 The Operating System That Thinks${NC}${CYAN}                          ║"
echo "║                                                                           ║"
echo "║              🎮 Interactive TUI Setup Wizard                              ║"
echo "║              ↑↓ Navigate    Space: Select    Enter: Confirm              ║"
echo "║                                                                           ║"
echo "╚═══════════════════════════════════════════════════════════════════════════╝"
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
    echo -e "${YELLOW}⚠️  Git kurulu değil. Lütfen Git'i kurun.${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Git: $(git --version | cut -d' ' -f3)${NC}"

echo ""

# ─────────────────────────────────────────────────────────────────────────────
# CLONE REPO (if needed)
# ─────────────────────────────────────────────────────────────────────────────

INSTALL_DIR="${SENTIENT_DIR:-$HOME/sentient}"

if [[ ! -d "$INSTALL_DIR" ]]; then
    echo -e "${BOLD}📥 SENTIENT indiriliyor...${NC}"
    git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git "$INSTALL_DIR"
    echo -e "${GREEN}✅ Depo klonlandı: $INSTALL_DIR${NC}"
else
    echo -e "${GREEN}✅ Depo mevcut: $INSTALL_DIR${NC}"
fi

cd "$INSTALL_DIR"

# ─────────────────────────────────────────────────────────────────────────────
# BUILD TUI WIZARD
# ─────────────────────────────────────────────────────────────────────────────

echo ""
echo -e "${BOLD}🔨 TUI Sihirbazı derleniyor...${NC}"
echo ""

# Build the setup wizard binary
cargo build --release --bin sentient-setup 2>/dev/null || {
    echo -e "${YELLOW}⚠️  Setup binary derlenemedi, alternatif yöntem...${NC}"
    
    # Fallback: Run setup directly
    if [[ -f "./target/release/sentient-shell" ]]; then
        ./target/release/sentient-shell --setup
    else
        echo -e "${YELLOW}ℹ️  Manuel kurulum için README.md'yi okuyun${NC}"
    fi
    exit 0
}

# ─────────────────────────────────────────────────────────────────────────────
# RUN TUI WIZARD
# ─────────────────────────────────────────────────────────────────────────────

echo ""
echo -e "${BOLD}🎮 Interactive TUI Sihirbazı başlatılıyor...${NC}"
echo -e "${CYAN}   ↑↓ Ok tuşlarıyla gezinin${NC}"
echo -e "${CYAN}   Space ile çoklu seçim yapın${NC}"
echo -e "${CYAN}   Enter ile onaylayın${NC}"
echo ""
sleep 1

# Run the interactive TUI wizard
./target/release/sentient-setup

# ─────────────────────────────────────────────────────────────────────────────
# DONE
# ─────────────────────────────────────────────────────────────────────────────

echo ""
echo -e "${GREEN}╔═══════════════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║  🎉 SENTIENT kurulumu tamamlandı!                                          ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BOLD}🚀 Başlatmak için:${NC}"
echo -e "   ${CYAN}cd $INSTALL_DIR${NC}"
echo -e "   ${CYAN}./target/release/sentient-shell${NC}"
echo ""
echo -e "${BOLD}🌐 Dashboard:${NC}"
echo -e "   ${CYAN}http://localhost:8080${NC}"
echo ""
