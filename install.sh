#!/bin/bash
set -euo pipefail

# SENTIENT OS Installer for macOS and Linux
# Usage: curl -fsSL https://sentientos.ai/install.sh | bash

ACCENT='\033[38;2;0;229;204m'
SUCCESS='\033[38;2;0;229;204m'
WARN='\033[38;2;255;176;32m'
ERROR='\033[38;2;230;57;70m'
MUTED='\033[38;2;90;100;128m'
NC='\033[0m'
BOLD='\033[1m'

INSTALL_DIR="${INSTALL_DIR:-$HOME/.sentient}"

step() { echo -e "${ACCENT}►${NC} $1"; }
ok() { echo -e "${SUCCESS}✓${NC} $1"; }
warn() { echo -e "${WARN}!${NC} $1"; }
err() { echo -e "${ERROR}✗${NC} $1"; }
info() { echo -e "${MUTED}·${NC} $1"; }

banner() {
    clear
    echo -e ""
    echo -e "${ACCENT}  ███████╗███████╗███╗   ██╗████████╗███╗   ██╗███████╗██╗${NC}"
    echo -e "${ACCENT}  ██╔════╝██╔════╝████╗  ██║╚══██╔══╝████╗  ██║██╔════╝██║${NC}"
    echo -e "${ACCENT}  ███████╗█████╗  ██╔██╗ ██║   ██║   ██╔██╗ ██║███████╗██║${NC}"
    echo -e "${ACCENT}  ╚════██║██╔══╝  ██║╚██╗██║   ██║   ██║╚██╗██║╚════██║██║${NC}"
    echo -e "${ACCENT}  ███████║███████╗██║ ╚████║   ██║   ██║ ╚████║███████║██║${NC}"
    echo -e "${ACCENT}  ╚══════╝╚══════╝╚═╝  ╚═══╝   ╚═╝   ╚═╝  ╚═══╝╚══════╝╚═╝${NC}"
    echo -e ""
    echo -e "${MUTED}  OS - The Operating System That Thinks${NC}"
    echo -e ""
}

check_cmd() { command -v "$1" &>/dev/null; }

get_rust_version() {
    if check_cmd rustc; then
        rustc --version 2>/dev/null | awk '{print $2}'
    fi
}

get_cargo_version() {
    if check_cmd cargo; then
        cargo --version 2>/dev/null | awk '{print $2}'
    fi
}

install_rust() {
    info "Rust not found - installing..."
    
    if check_cmd curl; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    elif check_cmd wget; then
        wget -qO- https://sh.rustup.rs | sh -s -- -y
    else
        err "Need curl or wget to install Rust"
        return 1
    fi
    
    # Source cargo
    source "$HOME/.cargo/env" 2>/dev/null || true
    export PATH="$HOME/.cargo/bin:$PATH"
    
    ok "Rust installed"
}

install_git() {
    info "Git not found - installing..."
    
    if check_cmd apt-get; then
        sudo apt-get update && sudo apt-get install -y git
    elif check_cmd dnf; then
        sudo dnf install -y git
    elif check_cmd brew; then
        brew install git
    elif check_cmd pacman; then
        sudo pacman -S --noconfirm git
    else
        err "Could not install git automatically"
        info "Install git from: https://git-scm.com"
        return 1
    fi
    
    ok "Git installed"
}

install_build_deps() {
    info "Checking build dependencies..."
    
    if check_cmd apt-get; then
        sudo apt-get update
        sudo apt-get install -y build-essential pkg-config libssl-dev python3-dev 2>/dev/null || true
    elif check_cmd dnf; then
        sudo dnf groupinstall -y "Development Tools" 2>/dev/null || true
        sudo dnf install -y openssl-devel python3-devel 2>/dev/null || true
    elif check_cmd pacman; then
        sudo pacman -S --noconfirm base-devel openssl python 2>/dev/null || true
    elif check_cmd brew; then
        brew install openssl python3 2>/dev/null || true
    fi
    
    ok "Build dependencies ready"
}

install_sentient() {
    local target_dir="$1"
    
    step "Cloning SENTIENT OS..."
    
    # Clone or update
    if [ -d "$target_dir/.git" ]; then
        info "Updating existing installation..."
        git -C "$target_dir" pull
    else
        rm -rf "$target_dir"
        git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git "$target_dir"
    fi
    ok "Repository ready"
    
    # Build
    step "Building SENTIENT OS (5-15 minutes)..."
    cd "$target_dir"
    
    export PYTHON_SYS_EXECUTABLE=$(which python3 2>/dev/null || which python 2>/dev/null || echo "")
    
    cargo build --release 2>&1 | while read line; do
        if [[ "$line" =~ Compiling|Building|Finished ]]; then
            info "$line"
        fi
    done
    
    if [ ! -f "target/release/sentient" ]; then
        err "Build failed!"
        return 1
    fi
    
    local size=$(du -h target/release/sentient | cut -f1)
    ok "Built successfully ($size)"
    
    # Create .env if not exists
    if [ ! -f ".env" ]; then
        step "Creating configuration..."
        cat > .env << 'EOF'
# SENTIENT OS Configuration
# Generated: $(date '+%Y-%m-%d %H:%M:%S')

# Default: Use Ollama (local, free)
OLLAMA_HOST=http://localhost:11434
DEFAULT_MODEL=ollama/gemma3:12b

# Alternative providers (uncomment and add your API key):
# OPENROUTER_API_KEY=sk-or-xxx
# OPENAI_API_KEY=sk-xxx
# ANTHROPIC_API_KEY=sk-ant-xxx
# DEEPSEEK_API_KEY=sk-xxx
# GROQ_API_KEY=gsk_xxx
# GOOGLE_AI_API_KEY=xxx

# To switch provider, change DEFAULT_MODEL to:
# DEFAULT_MODEL=openrouter/anthropic/claude-4-sonnet
# DEFAULT_MODEL=openai/gpt-4o
# DEFAULT_MODEL=anthropic/claude-4-sonnet
# DEFAULT_MODEL=deepseek/deepseek-chat
# DEFAULT_MODEL=groq/llama-3.3-70b-versatile
# DEFAULT_MODEL=google/gemini-2.0-flash

RUST_LOG=info
EOF
        ok "Configuration created"
    fi
    
    # Add to PATH
    local bin_path="$target_dir/target/release"
    if [[ ":$PATH:" != *":$bin_path:"* ]]; then
        # Detect shell
        local shell_profile=""
        if [ -n "$ZSH_VERSION" ]; then
            shell_profile="$HOME/.zshrc"
        elif [ -n "$BASH_VERSION" ]; then
            shell_profile="$HOME/.bashrc"
        fi
        
        if [ -n "$shell_profile" ]; then
            echo "export PATH=\"\$PATH:$bin_path\"" >> "$shell_profile"
            info "Added to PATH in $shell_profile"
        fi
        export PATH="$PATH:$bin_path"
    fi
    ok "Ready to use"
    
    return 0
}

uninstall_sentient() {
    local target_dir="$1"
    
    banner
    echo "  Uninstalling SENTIENT OS..."
    echo ""
    
    if [ -d "$target_dir" ]; then
        rm -rf "$target_dir"
        ok "Removed $target_dir"
    fi
    
    ok "SENTIENT OS uninstalled"
}

# ═══════════════════════════════════════════════════════════════════════════════
# MAIN
# ═══════════════════════════════════════════════════════════════════════════════

# Uninstall mode
if [ "${1:-}" = "--uninstall" ]; then
    uninstall_sentient "$INSTALL_DIR"
    exit 0
fi

banner

info "Installing to: $INSTALL_DIR"
echo ""

# Check prerequisites
step "Checking prerequisites..."

# Rust
rust_version=$(get_rust_version)
if [ -n "$rust_version" ]; then
    ok "Rust $rust_version"
else
    install_rust || exit 1
fi

# Cargo
cargo_version=$(get_cargo_version)
if [ -n "$cargo_version" ]; then
    ok "Cargo $cargo_version"
else
    err "Cargo not found after Rust installation"
    info "Please restart your terminal and try again"
    exit 1
fi

# Git
if check_cmd git; then
    ok "Git $(git --version | awk '{print $3}')"
else
    install_git || exit 1
fi

# Build deps
install_build_deps

echo ""

# Install
if ! install_sentient "$INSTALL_DIR"; then
    echo ""
    err "Installation failed"
    exit 1
fi

echo ""
echo -e "${ACCENT}════════════════════════════════════════════════════════════════${NC}"
echo -e "${SUCCESS}  ✓ SENTIENT OS installed successfully!${NC}"
echo -e "${ACCENT}════════════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "  ${MUTED}Quick Start:${NC}"
echo ""
echo -e "    ${ACCENT}sentient --version${NC}"
echo -e "    ${ACCENT}sentient chat${NC}"
echo ""
echo -e "  ${MUTED}To use Ollama (free, local):${NC}"
echo -e "    ${ACCENT}ollama pull gemma3:12b${NC}"
echo -e "    ${ACCENT}sentient chat${NC}"
echo ""
echo -e "  ${MUTED}To use cloud AI (OpenRouter, OpenAI, etc.):${NC}"
echo -e "    ${ACCENT}nano $INSTALL_DIR/.env${NC}"
echo -e "    ${MUTED}# Uncomment your preferred provider and add API key${NC}"
echo ""
echo -e "  ${MUTED}Documentation: https://github.com/nexsusagent-coder/SENTIENT_CORE${NC}"
echo ""
