#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT NEXUS OS - Quick Install Script v1.0.0
#  One-command installation: curl -sSL https://get.sentient.ai | bash
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

# Installation directory
INSTALL_DIR="${SENTIENT_HOME:-$HOME/.sentient}"
REPO_URL="https://github.com/nexsusagent-coder/SENTIENT_CORE.git"

# Print banner
print_banner() {
    echo -e "${CYAN}"
    echo "  ╔═══════════════════════════════════════════════════════════════╗"
    echo "  ║                                                               ║"
    echo "  ║   ███████╗██╗███████╗███╗   ██╗██████╗ ███████╗██████╗        ║"
    echo "  ║   ██╔════╝██║██╔════╝████╗  ██║██╔══██╗██╔════╝██╔══██╗       ║"
    echo "  ║   ███████╗██║█████╗  ██╔██╗ ██║██║  ██║█████╗  ██████╔╝       ║"
    echo "  ║   ╚════██║██║██╔══╝  ██║╚██╗██║██║  ██║██╔══╝  ██╔══██╗       ║"
    echo "  ║   ███████║██║███████╗██║ ╚████║██████╔╝███████╗██║  ██║       ║"
    echo "  ║   ╚══════╝╚═╝╚══════╝╚═╝  ╚═══╝╚═════╝ ╚══════╝╚═╝  ╚═╝       ║"
    echo "  ║                                                               ║"
    echo "  ║              SENTIENT NEXUS OS v7.0.0                        ║"
    echo "  ║              Professional AI Agent Framework                 ║"
    echo "  ║                                                               ║"
    echo "  ╚═══════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

# Check and install Rust
install_rust() {
    if command -v rustc &> /dev/null && command -v cargo &> /dev/null; then
        echo -e "${GREEN}[OK]${NC} Rust: $(rustc --version)"
        return 0
    fi
    
    echo -e "${YELLOW}[...]${NC} Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env" 2>/dev/null || true
    echo -e "${GREEN}[OK]${NC} Rust installed"
}

# Check and install Git
install_git() {
    if command -v git &> /dev/null; then
        echo -e "${GREEN}[OK]${NC} Git: $(git --version | cut -d' ' -f3)"
        return 0
    fi
    
    echo -e "${YELLOW}[...]${NC} Installing Git..."
    
    if command -v apt-get &> /dev/null; then
        sudo apt-get update -qq && sudo apt-get install -y -qq git
    elif command -v dnf &> /dev/null; then
        sudo dnf install -y -q git
    elif command -v brew &> /dev/null; then
        brew install git
    else
        echo -e "${RED}[ERR]${NC} Please install Git manually"
        exit 1
    fi
    
    echo -e "${GREEN}[OK]${NC} Git installed"
}

# Install system dependencies
install_deps() {
    echo -e "${YELLOW}[...]${NC} Installing system dependencies..."
    
    if command -v apt-get &> /dev/null; then
        sudo apt-get update -qq 2>/dev/null || true
        sudo apt-get install -y -qq build-essential pkg-config libssl-dev libsqlite3-dev cmake 2>/dev/null || true
    elif command -v dnf &> /dev/null; then
        sudo dnf install -y -q gcc pkg-config openssl-devel sqlite-devel cmake 2>/dev/null || true
    elif command -v brew &> /dev/null; then
        brew install openssl sqlite cmake 2>/dev/null || true
    fi
    
    echo -e "${GREEN}[OK]${NC} System dependencies ready"
}

# Clone or update repository
clone_repo() {
    if [ -d "$INSTALL_DIR" ]; then
        echo -e "${YELLOW}[...]${NC} Updating existing installation..."
        cd "$INSTALL_DIR"
        git pull origin main 2>/dev/null || git pull origin master 2>/dev/null || true
    else
        echo -e "${YELLOW}[...]${NC} Cloning SENTIENT..."
        git clone "$REPO_URL" "$INSTALL_DIR"
    fi
    echo -e "${GREEN}[OK]${NC} Repository ready: $INSTALL_DIR"
}

# Build the project
build_project() {
    cd "$INSTALL_DIR"
    
    echo -e "${YELLOW}[...]${NC} Building SENTIENT (this may take a few minutes)..."
    echo ""
    
    # Build only essential binaries
    cargo build --release --bin sentient-setup --bin sentient-shell 2>&1 | while IFS= read -r line; do
        if [[ "$line" == Compiling* ]]; then
            echo -ne "\r${CYAN}   $line${NC}                              "
        fi
    done
    
    echo ""
    echo -e "${GREEN}[OK]${NC} Build complete"
}

# Create global launcher
create_launcher() {
    local LAUNCHER_PATH="$HOME/.local/bin/sentient"
    local LAUNCHER_DIR
    LAUNCHER_DIR=$(dirname "$LAUNCHER_PATH")
    
    mkdir -p "$LAUNCHER_DIR"
    
    cat > "$LAUNCHER_PATH" << 'LAUNCHER'
#!/bin/bash
# SENTIENT Global Launcher
SENTIENT_DIR="${SENTIENT_HOME:-$HOME/.sentient}"
SENTIENT_BIN="$SENTIENT_DIR/target/release"

case "${1:-dashboard}" in
    dashboard|ui|"")
        if [ -f "$SENTIENT_BIN/sentient-shell" ]; then
            cd "$SENTIENT_DIR" && "$SENTIENT_BIN/sentient-shell"
        else
            echo "SENTIENT not found. Run: curl -sSL https://get.sentient.ai | bash"
            exit 1
        fi
        ;;
    setup)
        if [ -f "$SENTIENT_BIN/sentient-setup" ]; then
            "$SENTIENT_BIN/sentient-setup"
        else
            echo "Setup not found. Reinstall SENTIENT."
            exit 1
        fi
        ;;
    status)
        echo "SENTIENT: $SENTIENT_DIR"
        [ -f "$SENTIENT_BIN/sentient-shell" ] && echo "  Status: Installed" || echo "  Status: Not installed"
        [ -f "$SENTIENT_DIR/.env" ] && echo "  Config: Yes" || echo "  Config: No"
        command -v ollama &>/dev/null && echo "  Ollama: Yes" || echo "  Ollama: No"
        ;;
    help|--help|-h)
        echo "Usage: sentient [command]"
        echo ""
        echo "Commands:"
        echo "  dashboard, ui    Launch dashboard (default)"
        echo "  setup            Run setup wizard"
        echo "  status           Show status"
        echo "  help             Show this help"
        ;;
    *)
        echo "Unknown command: $1"
        echo "Run 'sentient help' for usage."
        exit 1
        ;;
esac
LAUNCHER
    
    chmod +x "$LAUNCHER_PATH"
    echo -e "${GREEN}[OK]${NC} Global launcher created: $LAUNCHER_PATH"
}

# Add to PATH
add_to_path() {
    local SHELL_RC
    
    if [ -n "$ZSH_VERSION" ]; then
        SHELL_RC="$HOME/.zshrc"
    else
        SHELL_RC="$HOME/.bashrc"
    fi
    
    # Check if already in PATH
    if grep -q 'sentient' "$SHELL_RC" 2>/dev/null; then
        return 0
    fi
    
    echo "" >> "$SHELL_RC"
    echo "# SENTIENT" >> "$SHELL_RC"
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_RC"
    
    echo -e "${GREEN}[OK]${NC} Added to PATH (restart terminal or run: source $SHELL_RC)"
}

# Install Ollama (optional)
install_ollama() {
    if command -v ollama &> /dev/null; then
        echo -e "${GREEN}[OK]${NC} Ollama: $(ollama --version 2>/dev/null | head -1 || echo 'installed')"
        return 0
    fi
    
    echo -e "${YELLOW}[...]${NC} Installing Ollama..."
    curl -fsSL https://ollama.com/install.sh | sh 2>/dev/null || {
        echo -e "${YELLOW}[SKIP]${NC} Ollama installation failed. Install manually: https://ollama.com"
        return 0
    }
    
    echo -e "${GREEN}[OK]${NC} Ollama installed"
}

# Main installation
main() {
    clear
    print_banner
    
    echo -e "${BOLD}Installation starting...${NC}"
    echo ""
    
    # Step 1: Dependencies
    install_git
    install_rust
    install_deps
    
    # Step 2: Clone
    clone_repo
    
    # Step 3: Build
    build_project
    
    # Step 4: Create launcher
    create_launcher
    add_to_path
    
    # Step 5: Ollama (optional)
    install_ollama
    
    # Done
    echo ""
    echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}  INSTALLATION COMPLETE!${NC}"
    echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
    echo ""
    echo -e "  ${BOLD}Start:${NC}    sentient"
    echo -e "  ${BOLD}Setup:${NC}    sentient setup"
    echo -e "  ${BOLD}Status:${NC}   sentient status"
    echo ""
    echo -e "  ${CYAN}Restart your terminal, then run: sentient setup${NC}"
    echo ""
}

# Run
main "$@"
