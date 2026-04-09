#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
#  🧠 SENTIENT OS - The Operating System That Thinks
#  Interactive Setup Script v5.0.0
# ═══════════════════════════════════════════════════════════════════════════════

# Hata durumunda devam et ama mesaj göster
trap 'echo -e "\n${RED}Hata oluştu! Devam ediliyor...${NC}"' ERR

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

# Clear screen and show header
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
echo -e "${BOLD}SENTIENT OS'e hoş geldiniz!${NC}"
echo ""
echo "Bu kurulum scripti size adım adım rehberlik edecek:"
echo ""
echo -e "  ${CYAN}1.${NC} Sistem kontrolü"
echo -e "  ${CYAN}2.${NC} Model seçimi"
echo -e "  ${CYAN}3.${NC} Bağımlılık kurulumu"
echo -e "  ${CYAN}4.${NC} SENTIENT kurulumu"
echo -e "  ${CYAN}5.${NC} Yapılandırma"
echo ""
read -p "Devam etmek için Enter'a basın..." -r
echo ""

# ═══════════════════════════════════════════════════════════════════════════════
# ADIM 1: Sistem Kontrolü
# ═══════════════════════════════════════════════════════════════════════════════

echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BOLD}${CYAN}  ADIM 1/5: SİSTEM KONTROLÜ${NC}"
echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# OS Detection
if [ -f /etc/os-release ]; then
    . /etc/os-release
    OS=$ID
    OS_NAME=$PRETTY_NAME
elif [ "$(uname)" = "Darwin" ]; then
    OS="macos"
    OS_NAME="macOS $(sw_vers -productVersion 2>/dev/null || echo 'Unknown')"
else
    OS="linux"
    OS_NAME="Linux"
fi

echo -e "${CYAN}📌 İşletim Sistemi:${NC} $OS_NAME"

# RAM Check
if [ "$OS" = "macos" ]; then
    TOTAL_RAM=$(sysctl -n hw.memsize 2>/dev/null | awk '{print int($1/1024/1024)}' || echo "0")
else
    TOTAL_RAM=$(free -m 2>/dev/null | awk '/^Mem:/{print $2}' || echo "0")
fi

if [ "$TOTAL_RAM" != "0" ]; then
    RAM_GB=$((TOTAL_RAM / 1024))
    if [ "$TOTAL_RAM" -lt 8000 ]; then
        echo -e "${YELLOW}⚠ RAM: ${RAM_GB}GB (Önerilen: 16GB+)${NC}"
    else
        echo -e "${GREEN}✓ RAM: ${RAM_GB}GB${NC}"
    fi
else
    echo -e "${YELLOW}⚠ RAM bilgisi alınamadı${NC}"
fi

# Disk Check
DISK_AVAIL=$(df -h . 2>/dev/null | awk 'NR==2{print $4}' || echo "Unknown")
echo -e "${CYAN}💾 Disk Alanı:${NC} ${DISK_AVAIL} boş"

# GPU Check
if command -v nvidia-smi &> /dev/null; then
    GPU_INFO=$(nvidia-smi --query-gpu=name --format=csv,noheader 2>/dev/null | head -1 || echo "NVIDIA GPU")
    echo -e "${GREEN}✓ GPU: ${GPU_INFO}${NC}"
    HAS_GPU=true
else
    echo -e "${YELLOW}⚠ GPU: NVIDIA GPU tespit edilmedi (Yerel model için önerilir)${NC}"
    HAS_GPU=false
fi

echo ""
echo -e "${GREEN}✓ Sistem kontrolü tamamlandı${NC}"

# ═══════════════════════════════════════════════════════════════════════════════
# ADIM 2: Model Seçimi
# ═══════════════════════════════════════════════════════════════════════════════

echo ""
echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BOLD}${CYAN}  ADIM 2/5: MODEL SEÇİMİ${NC}"
echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

echo -e "${BOLD}SENTIENT OS birden fazla model destekler:${NC}"
echo ""

echo -e "${GREEN}╔═══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║  🏠 YEREL MODELLER (API Key Gerektirmez)                     ║${NC}"
echo -e "${GREEN}╠═══════════════════════════════════════════════════════════════╣${NC}"
echo -e "${GREEN}║  1) Gemma 4 31B    - 256K context, Thinking Mode (ÖNERİLEN) ║${NC}"
echo -e "${GREEN}║  2) Llama 3.3 70B  - 128K context, Genel kullanım           ║${NC}"
echo -e "${GREEN}║  3) Qwen 2.5 72B   - 128K context, Coding optimize          ║${NC}"
echo -e "${GREEN}║  4) DeepSeek R1    - 128K context, Reasoning                ║${NC}"
echo -e "${GREEN}║  5) Mistral 24B    - 128K context, Hızlı                    ║${NC}"
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

echo ""
echo -e "${GREEN}✓ Seçilen model: $MODEL_DESC${NC}"

# API Key prompt
API_KEY_LINE=""
if [ "$PROVIDER" != "local" ] && [ "$PROVIDER" != "openrouter" ]; then
    echo ""
    echo -e "${YELLOW}⚠️  Bu model API key gerektiriyor.${NC}"
    read -p "$PROVIDER API key giriniz (veya Enter'a basıp sonra .env'e ekleyin): " API_KEY
    
    if [ -n "$API_KEY" ]; then
        PROVIDER_UPPER=$(echo "$PROVIDER" | tr '[:lower:]' '[:upper:]')
        API_KEY_LINE="${PROVIDER_UPPER}_API_KEY=${API_KEY}"
    fi
fi

# ═══════════════════════════════════════════════════════════════════════════════
# ADIM 3: Bağımlılık Kurulumu
# ═══════════════════════════════════════════════════════════════════════════════

echo ""
echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BOLD}${CYAN}  ADIM 3/5: BAĞIMLILIK KURULUMU${NC}"
echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Rust Check
echo -e "${CYAN}🦀 Rust kontrol ediliyor...${NC}"
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version 2>/dev/null || echo "Unknown")
    echo -e "${GREEN}✓ Rust: ${RUST_VERSION}${NC}"
else
    echo -e "${CYAN}⏳ Rust kuruluyor...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env" 2>/dev/null || true
    echo -e "${GREEN}✓ Rust kuruldu${NC}"
fi

# System dependencies
echo ""
echo -e "${CYAN}📦 Sistem bağımlılıkları...${NC}"

case $OS in
    ubuntu|debian)
        sudo apt update -qq 2>/dev/null || true
        sudo apt install -y -qq build-essential pkg-config libssl-dev sqlite3 libsqlite3-dev git curl wget cmake 2>/dev/null || true
        ;;
    fedora|rhel|centos)
        sudo dnf install -y -q gcc pkg-config openssl-devel sqlite-devel git curl wget cmake 2>/dev/null || true
        ;;
    macos)
        if command -v brew &> /dev/null; then
            brew install openssl sqlite cmake 2>/dev/null || true
        fi
        ;;
esac

echo -e "${GREEN}✓ Sistem bağımlılıkları hazır${NC}"

# Ollama for local models
if [ "$PROVIDER" = "local" ]; then
    echo ""
    echo -e "${CYAN}🤖 Ollama kontrol ediliyor...${NC}"
    
    if command -v ollama &> /dev/null; then
        echo -e "${GREEN}✓ Ollama zaten kurulu${NC}"
    else
        echo -e "${CYAN}⏳ Ollama kuruluyor...${NC}"
        curl -fsSL https://ollama.com/install.sh | sh 2>/dev/null || {
            echo -e "${YELLOW}⚠ Ollama kurulumu başarısız. Manuel kurun: https://ollama.com${NC}"
        }
    fi
    
    # Start Ollama
    echo ""
    echo -e "${CYAN}🚀 Ollama başlatılıyor...${NC}"
    ollama serve &
    sleep 3
    
    # Download model
    echo ""
    echo -e "${CYAN}📥 Model indiriliyor: ${SELECTED_MODEL}${NC}"
    echo -e "${YELLOW}   Bu işlem model boyutuna göre birkaç dakika sürebilir...${NC}"
    
    if command -v ollama &> /dev/null; then
        ollama pull "$SELECTED_MODEL" || echo -e "${YELLOW}⚠ Model indirme başarısız. Manuel: ollama pull $SELECTED_MODEL${NC}"
        echo -e "${GREEN}✓ Model hazır: ${SELECTED_MODEL}${NC}"
    fi
fi

# ═══════════════════════════════════════════════════════════════════════════════
# ADIM 4: SENTIENT Kurulumu
# ═══════════════════════════════════════════════════════════════════════════════

echo ""
echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BOLD}${CYAN}  ADIM 4/5: SENTIENT KURULUMU${NC}"
echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

INSTALL_DIR="$HOME/sentient"

echo -e "${CYAN}📂 SENTIENT OS indiriliyor...${NC}"

if [ -d "$INSTALL_DIR" ]; then
    echo -e "${YELLOW}⚠ $INSTALL_DIR zaten mevcut${NC}"
    read -p "Güncellensin mi? (y/n): " UPDATE
    if [ "$UPDATE" = "y" ] || [ "$UPDATE" = "Y" ]; then
        cd "$INSTALL_DIR"
        git pull origin main 2>/dev/null || git pull origin master 2>/dev/null || true
    fi
else
    git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git "$INSTALL_DIR" 2>/dev/null || {
        echo -e "${RED}✗ Klonlama başarısız!${NC}"
        exit 1
    }
fi

echo -e "${GREEN}✓ SENTIENT OS indirildi${NC}"

# Build
cd "$INSTALL_DIR"
echo ""
echo -e "${CYAN}🔨 SENTIENT derleniyor...${NC}"
echo -e "${YELLOW}   İlk derleme 5-10 dakika sürebilir...${NC}"

if command -v cargo &> /dev/null; then
    cargo build --release 2>&1 | while read -r line; do
        # Show progress
        if echo "$line" | grep -q "Compiling"; then
            echo -ne "\r${CYAN}⏳ ${line}${NC}                    "
        fi
    done
    echo ""
    echo -e "${GREEN}✓ SENTIENT derlendi${NC}"
else
    echo -e "${RED}✗ Cargo bulunamadı! Rust'ı yükleyin: source ~/.cargo/env${NC}"
fi

# Create .env
echo ""
echo -e "${CYAN}⚙️  Yapılandırma oluşturuluyor...${NC}"

cat > .env << ENVFILE
# ═════════════════════════════════════════════════════════════
#  SENTIENT OS Yapılandırma
# ═════════════════════════════════════════════════════════════

# Model Ayarları
SENTIENT_MODEL=${SELECTED_MODEL}
SENTIENT_PROVIDER=${PROVIDER}

# API Keys
${API_KEY_LINE}
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

echo -e "${GREEN}✓ .env dosyası oluşturuldu${NC}"

# ═══════════════════════════════════════════════════════════════════════════════
# ADIM 5: Tamamlandı
# ═══════════════════════════════════════════════════════════════════════════════

echo ""
echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BOLD}${CYAN}  ADIM 5/5: TAMAMLANDI${NC}"
echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

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

echo ""
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
if [ "$ADD_PATH" = "y" ] || [ "$ADD_PATH" = "Y" ]; then
    SHELL_RC="$HOME/.bashrc"
    [ -f "$HOME/.zshrc" ] && SHELL_RC="$HOME/.zshrc"
    
    echo "" >> "$SHELL_RC"
    echo "# SENTIENT OS" >> "$SHELL_RC"
    echo "export PATH=\"\$PATH:$INSTALL_DIR/target/release\"" >> "$SHELL_RC"
    echo "alias sentient='$INSTALL_DIR/target/release/sentient'" >> "$SHELL_RC"
    
    echo -e "${GREEN}✓ PATH'e eklendi. Yeni terminalde 'sentient' komutu çalışacaktır.${NC}"
fi

echo ""
echo -e "${GREEN}Kurulum tamamlandı! İyi kullanımlar! 🚀${NC}"
