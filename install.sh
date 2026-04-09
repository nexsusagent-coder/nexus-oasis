#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT - Quick Install Script
#  https://get.sentient.ai
# ═══════════════════════════════════════════════════════════════════════════════
#
#  Usage:
#    curl -sSL https://get.sentient.ai | bash
#    curl -sSL https://get.sentient.ai | bash -s -- --version 4.0.0
#
#  Options:
#    --version VERSION    Specify version (default: latest)
#    --prefix PATH        Install directory (default: ~/.sentient)
#    --no-confirm         Skip confirmation prompts
#    --uninstall          Remove SENTIENT
#
#  Supported:
#    - Linux (x86_64, arm64)
#    - macOS (x86_64, arm64)
#    - Windows (via PowerShell script)
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

# Defaults
VERSION="latest"
PREFIX="$HOME/.sentient"
NO_CONFIRM=false
UNINSTALL=false
REPO="nexsusagent-coder/SENTIENT_CORE"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --version)
            VERSION="$2"
            shift 2
            ;;
        --prefix)
            PREFIX="$2"
            shift 2
            ;;
        --no-confirm)
            NO_CONFIRM=true
            shift
            ;;
        --uninstall)
            UNINSTALL=true
            shift
            ;;
        -h|--help)
            echo "SENTIENT Quick Installer"
            echo ""
            echo "Usage: curl -sSL https://get.sentient.ai | bash"
            echo ""
            echo "Options:"
            echo "  --version VERSION    Specify version (default: latest)"
            echo "  --prefix PATH        Install directory (default: ~/.sentient)"
            echo "  --no-confirm         Skip confirmation prompts"
            echo "  --uninstall          Remove SENTIENT"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            exit 1
            ;;
    esac
done

# ═══════════════════════════════════════════════════════════════════════════════
#  UNINSTALL
# ═══════════════════════════════════════════════════════════════════════════════
if [ "$UNINSTALL" = true ]; then
    echo -e "${YELLOW}Uninstalling SENTIENT...${NC}"
    
    # Remove binary
    rm -f "$PREFIX/bin/sentient"
    rm -f "$PREFIX/bin/sentient-gateway"
    rm -f "$PREFIX/bin/sentient-agent"
    
    # Remove from PATH (shell configs)
    for rc in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile"; do
        if [ -f "$rc" ]; then
            sed -i '/# SENTIENT/d' "$rc" 2>/dev/null || true
            sed -i '/\.sentient\/bin/d' "$rc" 2>/dev/null || true
        fi
    done
    
    # Remove directory
    rm -rf "$PREFIX"
    
    echo -e "${GREEN}✓ SENTIENT uninstalled${NC}"
    exit 0
fi

# ═══════════════════════════════════════════════════════════════════════════════
#  DETECT PLATFORM
# ═══════════════════════════════════════════════════════════════════════════════
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux)
        OS_NAME="linux"
        ;;
    Darwin)
        OS_NAME="macos"
        ;;
    *)
        echo -e "${RED}Unsupported OS: $OS${NC}"
        echo "Please use the Windows PowerShell installer:"
        echo "  irm https://get.sentient.ai/ps | iex"
        exit 1
        ;;
esac

case "$ARCH" in
    x86_64|amd64)
        ARCH_NAME="x86_64"
        ;;
    aarch64|arm64)
        ARCH_NAME="arm64"
        ;;
    *)
        echo -e "${RED}Unsupported architecture: $ARCH${NC}"
        exit 1
        ;;
esac

# ═══════════════════════════════════════════════════════════════════════════════
#  BANNER
# ═══════════════════════════════════════════════════════════════════════════════
clear
echo -e "${CYAN}"
cat << 'EOF'
  ╔════════════════════════════════════════════════════════════╗
  ║     █████╗ ███╗   ██╗███████╗██╗      ██████╗ ██╗   ██╗    ║
  ║    ██╔══██╗████╗  ██║██╔════╝██║     ██╔═══██╗██║   ██║    ║
  ║    ███████║██╔██╗ ██║█████╗  ██║     ██║   ██║██║   ██║    ║
  ║    ██╔══██║██║╚██╗██║██╔══╝  ██║     ██║   ██║██║   ██║    ║
  ║    ██║  ██║██║ ╚████║███████╗███████╗╚██████╔╝╚██████╔╝    ║
  ║    ╚═╝  ╚═╝╚═╝  ╚═══╝╚══════╝╚══════╝ ╚═════╝  ╚═════╝     ║
  ║                                                            ║
  ║          NEXUS OASIS — AI Operating System                 ║
  ╚════════════════════════════════════════════════════════════╝
EOF
echo -e "${NC}"
echo ""

# ═══════════════════════════════════════════════════════════════════════════════
#  GET LATEST VERSION
# ═══════════════════════════════════════════════════════════════════════════════
if [ "$VERSION" = "latest" ]; then
    echo -e "${CYAN}🔍  Fetching latest version...${NC}"
    VERSION=$(curl -sSL "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | sed 's/.*"v\([^"]*\)".*/\1/')
    
    if [ -z "$VERSION" ]; then
        echo -e "${YELLOW}⚠ Could not fetch latest version, using default${NC}"
        VERSION="4.0.0"
    fi
fi

echo -e "${GREEN}📦  Version: $VERSION${NC}"
echo -e "${BLUE}🖥️  Platform: $OS_NAME-$ARCH_NAME${NC}"
echo -e "${MAGENTA}📁  Install location: $PREFIX${NC}"
echo ""

# ═══════════════════════════════════════════════════════════════════════════════
#  CONFIRM
# ═══════════════════════════════════════════════════════════════════════════════
if [ "$NO_CONFIRM" = false ]; then
    echo -e "${YELLOW}Continue with installation? [Y/n]${NC}"
    read -r CONFIRM
    if [[ "$CONFIRM" =~ ^[Nn]$ ]]; then
        echo "Installation cancelled."
        exit 0
    fi
fi

# ═══════════════════════════════════════════════════════════════════════════════
#  CREATE DIRECTORIES
# ═══════════════════════════════════════════════════════════════════════════════
echo ""
echo -e "${CYAN}📁  Creating directories...${NC}"
mkdir -p "$PREFIX/bin"
mkdir -p "$PREFIX/data"
mkdir -p "$PREFIX/config"
mkdir -p "$PREFIX/logs"

# ═══════════════════════════════════════════════════════════════════════════════
#  DOWNLOAD BINARY
# ═══════════════════════════════════════════════════════════════════════════════
DOWNLOAD_URL="https://github.com/$REPO/releases/download/v$VERSION/sentient-$OS_NAME-$ARCH_NAME.tar.gz"
TEMP_FILE="/tmp/sentient-$VERSION.tar.gz"

echo -e "${CYAN}📥  Downloading SENTIENT v$VERSION...${NC}"
echo -e "    ${DOWNLOAD_URL}"

if ! curl -fSL --progress-bar -o "$TEMP_FILE" "$DOWNLOAD_URL"; then
    echo -e "${RED}❌  Download failed!${NC}"
    echo ""
    echo "Possible reasons:"
    echo "  1. Version $VERSION doesn't exist"
    echo "  2. Binary for $OS_NAME-$ARCH_NAME not available"
    echo ""
    echo "Available versions: https://github.com/$REPO/releases"
    exit 1
fi

# ═══════════════════════════════════════════════════════════════════════════════
#  EXTRACT
# ═══════════════════════════════════════════════════════════════════════════════
echo -e "${CYAN}📦  Extracting...${NC}"
tar -xzf "$TEMP_FILE" -C "$PREFIX/bin"
rm -f "$TEMP_FILE"

# Make executable
chmod +x "$PREFIX/bin/sentient" 2>/dev/null || true
chmod +x "$PREFIX/bin/sentient-gateway" 2>/dev/null || true
chmod +x "$PREFIX/bin/sentient-agent" 2>/dev/null || true
chmod +x "$PREFIX/bin/sentient-setup" 2>/dev/null || true

# ═══════════════════════════════════════════════════════════════════════════════
#  ADD TO PATH
# ═══════════════════════════════════════════════════════════════════════════════
echo -e "${CYAN}🔧  Adding to PATH...${NC}"

# Add to shell config
SHELL_CONFIG=""
if [ -n "$ZSH_VERSION" ]; then
    SHELL_CONFIG="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
    SHELL_CONFIG="$HOME/.bashrc"
else
    SHELL_CONFIG="$HOME/.profile"
fi

# Check if already in PATH
if ! grep -q "SENTIENT" "$SHELL_CONFIG" 2>/dev/null; then
    echo "" >> "$SHELL_CONFIG"
    echo "# SENTIENT AI Operating System" >> "$SHELL_CONFIG"
    echo "export PATH=\"\$PATH:$PREFIX/bin\"" >> "$SHELL_CONFIG"
    echo -e "${GREEN}✓  Added to $SHELL_CONFIG${NC}"
else
    echo -e "${YELLOW}⚠  Already in PATH${NC}"
fi

# Export for current session
export PATH="$PATH:$PREFIX/bin"

# ═══════════════════════════════════════════════════════════════════════════════
#  VERIFY INSTALLATION
# ═══════════════════════════════════════════════════════════════════════════════
echo ""
echo -e "${CYAN}🔍  Verifying installation...${NC}"

if [ -f "$PREFIX/bin/sentient" ]; then
    INSTALLED_VERSION=$("$PREFIX/bin/sentient" --version 2>/dev/null || echo "v$VERSION")
    echo -e "${GREEN}✓  SENTIENT $INSTALLED_VERSION installed successfully!${NC}"
else
    echo -e "${RED}❌  Installation failed - binary not found${NC}"
    exit 1
fi

# ═══════════════════════════════════════════════════════════════════════════════
#  NEXT STEPS
# ═══════════════════════════════════════════════════════════════════════════════
echo ""
echo -e "${BOLD}${GREEN}══════════════════════════════════════════════════════════════${NC}"
echo -e "${BOLD}${GREEN}  🎉  INSTALLATION COMPLETE!${NC}"
echo -e "${BOLD}${GREEN}══════════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo ""
echo -e "  1. ${CYAN}Reload your shell:${NC}"
echo "     source ~/.bashrc  # or ~/.zshrc"
echo ""
echo -e "  2. ${CYAN}Run setup wizard:${NC}"
echo "     sentient setup"
echo ""
echo -e "  3. ${CYAN}Start interactive REPL:${NC}"
echo "     sentient repl"
echo ""
echo -e "  4. ${CYAN}Run autonomous agent:${NC}"
echo "     sentient agent --goal \"Your task description\""
echo ""
echo -e "${BLUE}Documentation: https://github.com/$REPO${NC}"
echo ""
