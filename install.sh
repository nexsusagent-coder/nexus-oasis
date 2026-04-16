#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS - Universal Installer
#  One command: curl -fsSL https://get.sentient.ai | sh
# ═══════════════════════════════════════════════════════════════════════════════

set -e

VERSION="${VERSION:-latest}"
PREFIX="${PREFIX:-$HOME/.sentient}"
REPO="nexsusagent-coder/SENTIENT_CORE"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

# ─────────────────────────────────────────────────────────────────────────────
#  Banner
# ─────────────────────────────────────────────────────────────────────────────
print_banner() {
    echo -e "${CYAN}"
    echo "  ╔════════════════════════════════════════════════════════════╗"
    echo "  ║     █████╗ ███╗   ██╗███████╗██╗      ██████╗ ██╗   ██╗    ║"
    echo "  ║    ██╔══██╗████╗  ██║██╔════╝██║     ██╔═══██╗██║   ██║    ║"
    echo "  ║    ███████║██╔██╗ ██║█████╗  ██║     ██║   ██║██║   ██║    ║"
    echo "  ║    ██╔══██║██║╚██╗██║██╔══╝  ██║     ██║   ██║██║   ██║    ║"
    echo "  ║    ██║  ██║██║ ╚████║███████╗███████╗╚██████╔╝╚██████╔╝    ║"
    echo "  ║    ╚═╝  ╚═╝╚═╝  ╚═══╝╚══════╝╚══════╝ ╚═════╝  ╚═════╝     ║"
    echo "  ║                                                            ║"
    echo "  ║          SENTIENT OS - AI Operating System                 ║"
    echo "  ╚════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

# ─────────────────────────────────────────────────────────────────────────────
#  Logging
# ─────────────────────────────────────────────────────────────────────────────
log_info()    { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[OK]${NC} $1"; }
log_warn()    { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error()   { echo -e "${RED}[ERROR]${NC} $1"; }

# ─────────────────────────────────────────────────────────────────────────────
#  Detect Platform
# ─────────────────────────────────────────────────────────────────────────────
detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"
    
    case "$OS" in
        Linux)  OS="unknown-linux-gnu" ;;
        Darwin) OS="apple-darwin" ;;
        *)      log_error "Unsupported OS: $OS"; exit 1 ;;
    esac
    
    case "$ARCH" in
        x86_64|amd64)   ARCH="x86_64" ;;
        aarch64|arm64)  ARCH="aarch64" ;;
        *)              log_error "Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    
    TARGET="${ARCH}-${OS}"
    log_info "Platform: $TARGET"
}

# ─────────────────────────────────────────────────────────────────────────────
#  Check Dependencies
# ─────────────────────────────────────────────────────────────────────────────
check_dependencies() {
    local missing=()
    
    # Required
    command -v curl >/dev/null 2>&1 || missing+=("curl")
    command -v tar >/dev/null 2>&1 || missing+=("tar")
    
    if [ ${#missing[@]} -gt 0 ]; then
        log_error "Missing required tools: ${missing[*]}"
        log_info "Installing dependencies..."
        
        if command -v apt-get >/dev/null 2>&1; then
            sudo apt-get update && sudo apt-get install -y "${missing[@]}"
        elif command -v dnf >/dev/null 2>&1; then
            sudo dnf install -y "${missing[@]}"
        elif command -v brew >/dev/null 2>&1; then
            brew install "${missing[@]}"
        else
            log_error "Please install: ${missing[*]}"
            exit 1
        fi
    fi
    
    log_success "All dependencies available"
}

# ─────────────────────────────────────────────────────────────────────────────
#  Install Optional Dependencies (Python, etc.)
# ─────────────────────────────────────────────────────────────────────────────
install_optional_deps() {
    log_info "Checking optional dependencies..."
    
    # Python 3.11+
    if ! command -v python3 >/dev/null 2>&1; then
        log_warn "Python 3 not found. Some features may be limited."
        log_info "To enable full functionality, install Python 3.11+"
    else
        PYTHON_VERSION=$(python3 --version 2>&1 | awk '{print $2}')
        log_success "Python $PYTHON_VERSION found"
    fi
    
    # Ollama (for local LLM)
    if ! command -v ollama >/dev/null 2>&1; then
        log_info "Ollama not found (optional for local LLM)"
    else
        log_success "Ollama found"
    fi
}

# ─────────────────────────────────────────────────────────────────────────────
#  Get Latest Version
# ─────────────────────────────────────────────────────────────────────────────
get_version() {
    if [ "$VERSION" = "latest" ]; then
        log_info "Fetching latest version..."
        VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')
        
        if [ -z "$VERSION" ]; then
            log_warn "Could not fetch version, using default"
            VERSION="v4.0.0"
        fi
    fi
    
    log_info "Version: $VERSION"
}

# ─────────────────────────────────────────────────────────────────────────────
#  Download & Install
# ─────────────────────────────────────────────────────────────────────────────
install() {
    local ARCHIVE_NAME="sentient-${TARGET}.tar.gz"
    local DOWNLOAD_URL="https://github.com/$REPO/releases/download/${VERSION}/${ARCHIVE_NAME}"
    local TEMP_DIR=$(mktemp -d)
    
    log_info "Downloading $ARCHIVE_NAME..."
    
    if ! curl -fsSL "$DOWNLOAD_URL" -o "$TEMP_DIR/$ARCHIVE_NAME"; then
        log_error "Download failed!"
        log_info "URL: $DOWNLOAD_URL"
        log_info "Available releases: https://github.com/$REPO/releases"
        exit 1
    fi
    
    log_info "Extracting..."
    mkdir -p "$PREFIX/bin"
    tar -xzf "$TEMP_DIR/$ARCHIVE_NAME" -C "$PREFIX" --strip-components=1
    
    # Cleanup
    rm -rf "$TEMP_DIR"
    
    log_success "Installed to $PREFIX"
}

# ─────────────────────────────────────────────────────────────────────────────
#  Configure Shell
# ─────────────────────────────────────────────────────────────────────────────
configure_shell() {
    local shell_rc=""
    
    case "$SHELL" in
        */bash)   shell_rc="$HOME/.bashrc" ;;
        */zsh)    shell_rc="$HOME/.zshrc" ;;
        */fish)   shell_rc="$HOME/.config/fish/config.fish" ;;
        *)        shell_rc="$HOME/.profile" ;;
    esac
    
    # Add to PATH
    if ! grep -q "SENTIENT_HOME" "$shell_rc" 2>/dev/null; then
        log_info "Adding to PATH in $shell_rc..."
        
        cat >> "$shell_rc" << 'EOF'

# SENTIENT OS
export SENTIENT_HOME="$HOME/.sentient"
export PATH="$SENTIENT_HOME/bin:$PATH"
EOF
        log_success "PATH updated"
    else
        log_info "PATH already configured"
    fi
    
    # Add to current session
    export PATH="$PREFIX/bin:$PATH"
}

# ─────────────────────────────────────────────────────────────────────────────
#  Verify Installation
# ─────────────────────────────────────────────────────────────────────────────
verify() {
    log_info "Verifying installation..."
    
    if [ -x "$PREFIX/bin/sentient" ]; then
        INSTALLED_VERSION=$("$PREFIX/bin/sentient" --version 2>/dev/null || echo "$VERSION")
        log_success "SENTIENT $INSTALLED_VERSION installed!"
        
        echo ""
        echo -e "${GREEN}══════════════════════════════════════════════════════════${NC}"
        echo -e "${GREEN}  INSTALLATION COMPLETE!${NC}"
        echo -e "${GREEN}══════════════════════════════════════════════════════════${NC}"
        echo ""
        echo -e "  ${YELLOW}Next steps:${NC}"
        echo ""
        echo "  1. Restart your shell or run:"
        echo "     source ~/.bashrc  # or ~/.zshrc"
        echo ""
        echo "  2. Run setup wizard:"
        echo "     sentient setup"
        echo ""
        echo "  3. Start interactive session:"
        echo "     sentient"
        echo ""
        echo "  4. Start web dashboard:"
        echo "     sentient-web"
        echo ""
    else
        log_error "Installation failed - binary not found"
        exit 1
    fi
}

# ─────────────────────────────────────────────────────────────────────────────
#  Uninstall
# ─────────────────────────────────────────────────────────────────────────────
uninstall() {
    log_info "Uninstalling SENTIENT..."
    
    rm -rf "$PREFIX"
    
    # Remove from shell config
    for rc in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile"; do
        if [ -f "$rc" ]; then
            sed -i '/# SENTIENT OS/,+3d' "$rc" 2>/dev/null || true
        fi
    done
    
    log_success "SENTIENT uninstalled"
    exit 0
}

# ─────────────────────────────────────────────────────────────────────────────
#  Main
# ─────────────────────────────────────────────────────────────────────────────
main() {
    # Parse arguments
    while [ $# -gt 0 ]; do
        case "$1" in
            --version|-v)    VERSION="$2"; shift 2 ;;
            --prefix|-p)     PREFIX="$2"; shift 2 ;;
            --uninstall|-u)  uninstall ;;
            --help|-h)       echo "Usage: $0 [--version VER] [--prefix PATH] [--uninstall]"; exit 0 ;;
            *)               shift ;;
        esac
    done
    
    print_banner
    
    log_info "Starting installation..."
    echo ""
    
    detect_platform
    check_dependencies
    get_version
    install
    install_optional_deps
    configure_shell
    verify
}

main "$@"
