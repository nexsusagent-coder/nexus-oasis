#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
#  🧠 SENTIENT OS - The Operating System That Thinks
#  Interactive Setup Script v5.0.0
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

# Animation
show_spinner() {
    local pid=$1
    local msg=$2
    local spin='⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏'
    local i=0
    while kill -0 $pid 2>/dev/null; do
        i=$(( (i+1) % 10 ))
        printf "\r${CYAN}${spin:$i:1}${NC} ${msg}..."
        sleep 0.1
    done
    printf "\r${GREEN}✓${NC} ${msg}\n"
}

# Clear screen and show header
show_header() {
    clear
    echo -e "${CYAN}"
    cat << 'EOF'
    ╔═══════════════════════════════════════════════════════════════╗
    ║                                                               ║
    ║     ███████╗███████╗███╗   ██╗████████╗██╗ ██████╗ █████╗     ║
    ║     ██╔════╝██╔════╝████╗  ██║╚══██╔══╝██║██╔════╝██╔══██╗    ║
    ║     ███████╗█████╗  ██╔██╗ ██║   ██║   ██║██║     ███████║    ║
    ║     ╚════██║██╔══╝  ██║╚██╗██║   ██║   ██║██║     ██╔══██║    ║
    ║     ███████║███████╗██║ ╚████║   ██║   ██║╚██████╗██║  ██║    ║
    ║     ╚══════╝╚══════╝╚═╝  ╚═══╝   ╚═╝   ╚═╝ ╚═════╝╚═╝  ╚═╝    ║
    ║                                                               ║
    ║              The Operating System That Thinks                 ║
    ║                                                               ║
    ╚═══════════════════════════════════════════════════════════════╝
EOF
    echo -e "${NC}"
    echo ""
}

# Step indicator
show_step() {
    local step=$1
    local total=$2
    local title=$3
    echo -e "\n${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BOLD}${CYAN}  ADIM ${step}/${total}: ${title}${NC}"
    echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"
}

# Success message
show_success() {
    echo -e "\n${GREEN}✓ $1${NC}"
}

# Error message
show_error() {
    echo -e "\n${RED}✗ $1${NC}"
}

# Warning message
show_warning() {
    echo -e "\n${YELLOW}⚠ $1${NC}"
}

# Check command exists
command_exists() {
    command -v "$1" &> /dev/null
}

# ═══════════════════════════════════════════════════════════════════════════════
# STEP 1: Welcome & OS Detection
# ═══════════════════════════════════════════════════════════════════════════════
step_welcome() {
    show_header
    
    echo -e "${BOLD}SENTIENT OS'e hoş geldiniz!${NC}\n"
    echo -e "Bu kurulum scripti size adım adım rehberlik edecek:"
    echo ""
    echo -e "  ${CYAN}1.${NC} Sistem kontrolü"
    echo -e "  ${CYAN}2.${NC} Model seçimi"
    echo -e "  ${CYAN}3.${NC} Bağımlılık kurulumu"
    echo -e "  ${CYAN}4.${NC} SENTIENT kurulumu"
    echo -e "  ${CYAN}5.${NC} Yapılandırma"
    echo ""
    
    read -p "Devam etmek için Enter'a basın..."
}

# ═══════════════════════════════════════════════════════════════════════════════
# STEP 2: System Check
# ═══════════════════════════════════════════════════════════════════════════════
step_system_check() {
    show_step 2 5 "SİSTEM KONTROLÜ"
    
    # OS Detection
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        OS=$ID
        OS_VER=$VERSION_ID
        OS_NAME=$PRETTY_NAME
    elif [ "$(uname)" = "Darwin" ]; then
        OS="macos"
        OS_VER=$(sw_vers -productVersion)
        OS_NAME="macOS $OS_VER"
    else
        OS="unknown"
        OS_NAME="Unknown"
    fi
    
    echo -e "${CYAN}📌 İşletim Sistemi:${NC} $OS_NAME"
    
    # RAM Check
    if [ "$OS" = "macos" ]; then
        TOTAL_RAM=$(sysctl -n hw.memsize 2>/dev/null | awk '{print int($1/1024/1024)}')
    else
        TOTAL_RAM=$(free -m 2>/dev/null | awk '/^Mem:/{print $2}')
    fi
    
    RAM_GB=$((TOTAL_RAM / 1024))
    if [ "$TOTAL_RAM" -lt 8000 ]; then
        show_warning "RAM: ${RAM_GB}GB (Önerilen: 16GB+)"
    else
        show_success "RAM: ${RAM_GB}GB"
    fi
    
    # Disk Check
    DISK_AVAIL=$(df -h . 2>/dev/null | awk 'NR==2{print $4}')
    echo -e "${CYAN}💾 Disk Alanı:${NC} ${DISK_AVAIL} boş"
    
    # GPU Check (optional)
    if command_exists nvidia-smi; then
        GPU_INFO=$(nvidia-smi --query-gpu=name,memory.total --format=csv,noheader 2>/dev/null | head -1)
        show_success "GPU: ${GPU_INFO}"
        HAS_GPU=true
    else
        echo -e "${YELLOW}GPU: NVIDIA GPU tespit edilmedi (Yerel model için önerilir)${NC}"
        HAS_GPU=false
    fi
    
    echo ""
    show_success "Sistem kontrolü tamamlandı"
}

# ═══════════════════════════════════════════════════════════════════════════════
# STEP 3: Model Selection
# ═══════════════════════════════════════════════════════════════════════════════
step_model_selection() {
    show_step 3 5 "MODEL SEÇİMİ"
    
    echo -e "${BOLD}SENTIENT OS birden fazla model destekler:${NC}\n"
    
    echo -e "${GREEN}╔═══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║  🏠 YEREL MODELLER (API Key Gerektirmez)                     ║${NC}"
    echo -e "${GREEN}╠═══════════════════════════════════════════════════════════════╣${NC}"
    echo -e "${GREEN}║  1) Gemma 4 31B    - 256K context, Thinking Mode (ÖNERİLEN) ║${NC}"
    echo -e "${GREEN}║  2) Llama 3.3 70B  - 128K context, Genel kullanım           ║${GREEN}"
    echo -e "${GREEN}║  3) Qwen 2.5 72B   - 128K context, Coding optimize          ║${GREEN}"
    echo -e "${GREEN}║  4) DeepSeek R1    - 128K context, Reasoning                ║${GREEN}"
    echo -e "${GREEN}║  5) Mistral 24B    - 128K context, Hızlı                    ║${GREEN}"
    echo -e "${GREEN}╚═══════════════════════════════════════════════════════════════╝${NC}"
    
    echo -e "${BLUE}╔═══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║  🔑 API MODELLER (API Key Gerekli)                           ║${NC}"
    echo -e "${BLUE}╠═══════════════════════════════════════════════════════════════╣${NC}"
    echo -e "${BLUE}║  6) OpenAI GPT-4o         - Multimodal, Coding              ║${NC}"
    echo -e "${BLUE}║  7) Anthropic Claude 3.7  - Coding, Reasoning               ║${NC}"
    echo -e "${BLUE}║  8) Google Gemini 2.0     - 1M Context                      ║${NC}"
    echo -e "${BLUE}║  9) Groq Llama 3.3        - Hızlı Inference                 ║${NC}"
    echo -e "${BLUE}║ 10) OpenRouter Free       - Ücretsiz Tier                   ║${NC}"
    echo -e "${BLUE}╚═══════════════════════════════════════════════════════════════╝${NC}"
    
    echo ""
    read -p "Model seçiniz [1-10] (varsayılan: 1): " MODEL_CHOICE
    MODEL_CHOICE=${MODEL_CHOICE:-1}
    
    case $MODEL_CHOICE in
        1) SELECTED_MODEL="gemma4:31b"; PROVIDER="local"; MODEL_DESC="Gemma 4 31B (Yerel)" ;;
        2) SELECTED_MODEL="llama3.3:70b"; PROVIDER="local"; MODEL_DESC="Llama 3.3 70B (Yerel)" ;;
        3) SELECTED_MODEL="qwen2.5:72b"; PROVIDER="local"; MODEL_DESC="Qwen 2.5 72B (Yerel)" ;;
        4) SELECTED_MODEL="deepseek-r1:67b"; PROVIDER="local"; MODEL_DESC="DeepSeek R1 (Yerel)" ;;
        5) SELECTED_MODEL="mistral:24b"; PROVIDER="local"; MODEL_DESC="Mistral 24B (Yerel)" ;;
        6) SELECTED_MODEL="gpt-4o"; PROVIDER="openai"; MODEL_DESC="GPT-4o (OpenAI)" ;;
        7) SELECTED_MODEL="claude-3.7-sonnet"; PROVIDER="anthropic"; MODEL_DESC="Claude 3.7 (Anthropic)" ;;
        8) SELECTED_MODEL="gemini-2.0-flash"; PROVIDER="google"; MODEL_DESC="Gemini 2.0 (Google)" ;;
        9) SELECTED_MODEL="llama-3.3-70b"; PROVIDER="groq"; MODEL_DESC="Llama 3.3 (Groq)" ;;
        10) SELECTED_MODEL="google/gemma-4-31b-it:free"; PROVIDER="openrouter"; MODEL_DESC="Gemma 4 Free (OpenRouter)" ;;
        *) SELECTED_MODEL="gemma4:31b"; PROVIDER="local"; MODEL_DESC="Gemma 4 31B (Yerel)" ;;
    esac
    
    show_success "Seçilen model: $MODEL_DESC"
    
    # API Key prompt if needed
    if [ "$PROVIDER" != "local" ] && [ "$PROVIDER" != "openrouter" ]; then
        echo ""
        echo -e "${YELLOW}⚠️  Bu model API key gerektiriyor.${NC}"
        read -p "$PROVIDER API key giriniz (veya Enter'a basıp sonra .env'e ekleyin): " API_KEY
        
        if [ -n "$API_KEY" ]; then
            API_KEY_UPPER=$(echo "$PROVIDER" | tr '[:lower:]' '[:upper:]')
            API_KEYS="${API_KEYS}${API_KEY_UPPER}_API_KEY=${API_KEY}\n"
        fi
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# STEP 4: Install Dependencies
# ═══════════════════════════════════════════════════════════════════════════════
step_install_deps() {
    show_step 4 5 "BAĞIMLILIK KURULUMU"
    
    # Rust
    if command_exists rustc; then
        show_success "Rust: $(rustc --version)"
    else
        echo -e "${CYAN}⏳ Rust kuruluyor...${NC}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env" 2>/dev/null || true
        show_success "Rust kuruldu"
    fi
    
    # System dependencies
    echo -e "\n${CYAN}📦 Sistem bağımlılıkları...${NC}"
    
    case $OS in
        ubuntu|debian)
            sudo apt update -qq
            sudo apt install -y -qq build-essential pkg-config libssl-dev sqlite3 libsqlite3-dev git curl wget cmake python3 python3-pip python3-venv bc > /dev/null 2>&1
            ;;
        macos)
            if ! command_exists brew; then
                echo -e "${CYAN}⏳ Homebrew kuruluyor...${NC}"
                /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)" > /dev/null 2>&1
            fi
            brew install openssl sqlite cmake python3 > /dev/null 2>&1
            ;;
    esac
    
    show_success "Sistem bağımlılıkları kuruldu"
    
    # Ollama (for local models)
    if [ "$PROVIDER" = "local" ]; then
        echo -e "\n${CYAN}🤖 Ollama kuruluyor...${NC}"
        
        if command_exists ollama; then
            show_success "Ollama zaten kurulu"
        else
            curl -fsSL https://ollama.com/install.sh | sh > /dev/null 2>&1
            show_success "Ollama kuruldu"
        fi
        
        # Start Ollama
        ollama serve > /dev/null 2>&1 &
        sleep 3
        
        # Download model
        echo -e "\n${CYAN}📥 Model indiriliyor: ${SELECTED_MODEL}${NC}"
        echo -e "${YELLOW}   Bu işlem model boyutuna göre birkaç dakika sürebilir...${NC}"
        ollama pull "$SELECTED_MODEL"
        show_success "Model indirildi: $SELECTED_MODEL"
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# STEP 5: Install SENTIENT
# ═══════════════════════════════════════════════════════════════════════════════
step_install_sentient() {
    show_step 5 5 "SENTIENT KURULUMU"
    
    # Clone
    echo -e "${CYAN}📂 SENTIENT OS indiriliyor...${NC}"
    
    INSTALL_DIR="$HOME/sentient"
    
    if [ -d "$INSTALL_DIR" ]; then
        echo -e "${YELLOW}⚠️  $INSTALL_DIR zaten mevcut${NC}"
        read -p "Güncellensin mi? (y/n): " UPDATE
        if [ "$UPDATE" = "y" ]; then
            cd "$INSTALL_DIR"
            git pull origin main > /dev/null 2>&1
        fi
    else
        git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git "$INSTALL_DIR" > /dev/null 2>&1
    fi
    
    show_success "SENTIENT OS indirildi"
    
    # Build
    cd "$INSTALL_DIR"
    
    echo -e "\n${CYAN}🔨 SENTIENT derleniyor...${NC}"
    echo -e "${YELLOW}   İlk derleme 5-10 dakika sürebilir...${NC}"
    
    cargo build --release > /dev/null 2>&1 &
    BUILD_PID=$!
    
    # Progress indicator
    local spin='⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏'
    local i=0
    while kill -0 $BUILD_PID 2>/dev/null; do
        i=$(( (i+1) % 10 ))
        printf "\r${CYAN}${spin:$i:1}${NC} Derleniyor...  "
        sleep 0.2
    done
    wait $BUILD_PID
    
    show_success "SENTIENT derlendi"
    
    # Create .env
    echo -e "\n${CYAN}⚙️  Yapılandırma oluşturuluyor...${NC}"
    
    cat > .env << ENVFILE
# ═════════════════════════════════════════════════════════════
#  SENTIENT OS Yapılandırma
# ═════════════════════════════════════════════════════════════

# Model Ayarları
SENTIENT_MODEL=${SELECTED_MODEL}
SENTIENT_PROVIDER=${PROVIDER}

# API Keys (varsa)
${API_KEYS}
# OpenRouter (ücretsiz modeller için)
# OPENROUTER_API_KEY=sk-or-...

# Yerel Model (Ollama)
OLLAMA_HOST=http://localhost:11434

# Gateway
GATEWAY_HTTP_ADDR=0.0.0.0:8080
JWT_SECRET=change-this-in-production

# Memory
MEMORY_DB_PATH=data/sentient.db

# Logging
RUST_LOG=info
ENVFILE
    
    show_success ".env dosyası oluşturuldu"
}

# ═══════════════════════════════════════════════════════════════════════════════
# COMPLETE
# ═══════════════════════════════════════════════════════════════════════════════
show_complete() {
    clear
    echo -e "${GREEN}"
    cat << 'EOF'
    ╔═══════════════════════════════════════════════════════════════╗
    ║                                                               ║
    ║              🎉 SENTIENT OS KURULUMU TAMAMLANDI!             ║
    ║                                                               ║
    ╚═══════════════════════════════════════════════════════════════╝
EOF
    echo -e "${NC}"
    
    echo -e "${BOLD}📋 Kurulum Özeti:${NC}"
    echo -e "  ${CYAN}Model:${NC}     $MODEL_DESC"
    echo -e "  ${CYAN}Provider:${NC}  $PROVIDER"
    echo -e "  ${CYAN}Dizin:${NC}     $INSTALL_DIR"
    echo ""
    
    echo -e "${BOLD}🚀 Başlatmak için:${NC}"
    echo ""
    echo -e "  ${WHITE}cd $INSTALL_DIR${NC}"
    echo -e "  ${WHITE}./target/release/sentient${NC}"
    echo ""
    
    echo -e "${BOLD}🌐 Dashboard:${NC}"
    echo ""
    echo -e "  ${WHITE}./target/release/sentient-dashboard${NC}"
    echo -e "  ${WHITE}# http://localhost:8080${NC}"
    echo ""
    
    echo -e "${BOLD}⚙️  Yapılandırma:${NC}"
    echo ""
    echo -e "  ${WHITE}nano $INSTALL_DIR/.env${NC}"
    echo ""
    
    echo -e "${PURPLE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}🧠 SENTIENT OS - The Operating System That Thinks${NC}"
    echo -e "${PURPLE}═══════════════════════════════════════════════════════════════${NC}"
    
    # Add to PATH
    echo ""
    read -p "SENTIENT'i PATH'e eklemek ister misiniz? (y/n): " ADD_PATH
    if [ "$ADD_PATH" = "y" ]; then
        SHELL_RC="$HOME/.bashrc"
        [ -f "$HOME/.zshrc" ] && SHELL_RC="$HOME/.zshrc"
        
        echo "" >> "$SHELL_RC"
        echo "# SENTIENT OS" >> "$SHELL_RC"
        echo "export PATH=\"\$PATH:$INSTALL_DIR/target/release\"" >> "$SHELL_RC"
        echo "alias sentient='$INSTALL_DIR/target/release/sentient'" >> "$SHELL_RC"
        
        show_success "PATH'e eklendi. Yeni terminalde 'sentient' komutu çalışacaktır."
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# MAIN
# ═══════════════════════════════════════════════════════════════════════════════
main() {
    step_welcome
    step_system_check
    step_model_selection
    step_install_deps
    step_install_sentient
    show_complete
}

main
