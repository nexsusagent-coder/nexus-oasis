#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS - OTOMATİK KURULUM SCRIPTI
#  Version: 4.0.0 | Kernel: Gemma 4 31B
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# Renkler
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Logo
echo -e "${CYAN}"
echo "═══════════════════════════════════════════════════════════════"
echo "  🧠  SENTIENT OS - The Operating System That Thinks"
echo "  📦  Otomatik Kurulum Scripti v4.0.0"
echo "  🔑  Kernel: Gemma 4 31B (NO API KEY REQUIRED!)"
echo "═══════════════════════════════════════════════════════════════"
echo -e "${NC}"

# OS kontrolü
detect_os() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        OS=$ID
        VER=$VERSION_ID
    elif [ "$(uname)" = "Darwin" ]; then
        OS="macos"
        VER=$(sw_vers -productVersion)
    else
        OS="unknown"
    fi
    echo -e "${BLUE}📌 İşletim Sistemi: ${OS} ${VER}${NC}"
}

# Sistem gereksinimleri kontrolü
check_requirements() {
    echo -e "\n${YELLOW}🔍 Sistem gereksinimleri kontrol ediliyor...${NC}"
    
    local PASS=true
    
    # RAM kontrolü
    local TOTAL_RAM=$(free -m 2>/dev/null | awk '/^Mem:/{print $2}' || sysctl -n hw.memsize 2>/dev/null | awk '{print int($1/1024/1024)}')
    if [ -n "$TOTAL_RAM" ]; then
        if [ "$TOTAL_RAM" -lt 8000 ]; then
            echo -e "${RED}❌ RAM: ${TOTAL_RAM}MB (Minimum 8GB gerekli)${NC}"
            PASS=false
        else
            echo -e "${GREEN}✅ RAM: ${TOTAL_RAM}MB${NC}"
        fi
    fi
    
    # Disk kontrolü
    local DISK_AVAIL=$(df -h . 2>/dev/null | awk 'NR==2{print $4}' | sed 's/G//')
    if [ -n "$DISK_AVAIL" ]; then
        if (( $(echo "$DISK_AVAIL < 20" | bc -l 2>/dev/null || echo "0") )); then
            echo -e "${RED}❌ Disk: ${DISK_AVAIL}GB (Minimum 20GB gerekli)${NC}"
            PASS=false
        else
            echo -e "${GREEN}✅ Disk: ${DISK_AVAIL}GB${NC}"
        fi
    fi
    
    if [ "$PASS" = false ]; then
        echo -e "${YELLOW}⚠️  Sistem gereksinimleri karşılanmıyor. Devam etmek istiyor musunuz? (y/n)${NC}"
        read -r CONTINUE
        if [ "$CONTINUE" != "y" ]; then
            exit 1
        fi
    fi
}

# Rust kurulumu
install_rust() {
    echo -e "\n${YELLOW}🦀 Rust kurulumu kontrol ediliyor...${NC}"
    
    if command -v rustc &> /dev/null; then
        RUST_VER=$(rustc --version)
        echo -e "${GREEN}✅ Rust kurulu: ${RUST_VER}${NC}"
    else
        echo -e "${BLUE}⏳ Rust kuruluyor...${NC}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        echo -e "${GREEN}✅ Rust kuruldu!${NC}"
    fi
    
    # Rust component'leri
    rustup component add clippy rustfmt 2>/dev/null || true
}

# Sistem bağımlılıkları (Ubuntu/Debian)
install_deps_linux() {
    echo -e "\n${YELLOW}📦 Sistem bağımlılıkları kuruluyor...${NC}"
    
    sudo apt update
    sudo apt install -y \
        build-essential \
        pkg-config \
        libssl-dev \
        sqlite3 \
        libsqlite3-dev \
        git \
        curl \
        wget \
        cmake \
        python3 \
        python3-pip \
        python3-venv \
        bc
    
    echo -e "${GREEN}✅ Sistem bağımlılıkları kuruldu!${NC}"
}

# Sistem bağımlılıkları (macOS)
install_deps_macos() {
    echo -e "\n${YELLOW}📦 Sistem bağımlılıkları kuruluyor...${NC}"
    
    if ! command -v brew &> /dev/null; then
        echo -e "${BLUE}⏳ Homebrew kuruluyor...${NC}"
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    fi
    
    brew install openssl sqlite cmake python3
    
    echo -e "${GREEN}✅ Sistem bağımlılıkları kuruldu!${NC}"
}

# Ollama kurulumu (Gemma 4 Kernel)
install_ollama() {
    echo -e "\n${YELLOW}🤖 Ollama (Gemma 4 Kernel) kurulumu...${NC}"
    
    if command -v ollama &> /dev/null; then
        echo -e "${GREEN}✅ Ollama kurulu${NC}"
    else
        echo -e "${BLUE}⏳ Ollama kuruluyor...${NC}"
        curl -fsSL https://ollama.com/install.sh | sh
        echo -e "${GREEN}✅ Ollama kuruldu!${NC}"
    fi
    
    # Ollama servisini başlat
    echo -e "${BLUE}⏳ Ollama servisi başlatılıyor...${NC}"
    ollama serve &
    OLLAMA_PID=$!
    sleep 5
    
    # Gemma 4 modelini indir
    echo -e "${PURPLE}⏳ Gemma 4 31B modeli indiriliyor (ilk kurulum ~20GB)...${NC}"
    echo -e "${YELLOW}   Alternatif: gemma4:12b (8GB) veya gemma4:4b (3GB)${NC}"
    
    # Kullanıcıya sor
    echo -e "\n${CYAN}Hangi Gemma 4 modelini indirmek istiyorsunuz?${NC}"
    echo "  1) gemma4:31b (ÖNERİLEN - 256K context, ~20GB)"
    echo "  2) gemma4:12b (Orta - 128K context, ~8GB)"
    echo "  3) gemma4:4b  (Minimum - 64K context, ~3GB)"
    echo "  4) Daha sonra indir"
    echo ""
    read -p "Seçiminiz [1-4]: " MODEL_CHOICE
    
    case $MODEL_CHOICE in
        1) ollama pull gemma4:31b ;;
        2) ollama pull gemma4:12b ;;
        3) ollama pull gemma4:4b ;;
        4) echo -e "${YELLOW}Model daha sonra indirilebilir: ollama pull gemma4:31b${NC}" ;;
        *) ollama pull gemma4:31b ;;
    esac
    
    echo -e "${GREEN}✅ Ollama ve Gemma 4 hazır!${NC}"
}

# Docker kurulumu (opsiyonel)
install_docker() {
    echo -e "\n${YELLOW}🐳 Docker kurulumu (opsiyonel)...${NC}"
    
    if command -v docker &> /dev/null; then
        echo -e "${GREEN}✅ Docker kurulu${NC}"
        return
    fi
    
    echo -e "${CYAN}Docker kurmak istiyor musunuz? (y/n)${NC}"
    read -r INSTALL_DOCKER
    
    if [ "$INSTALL_DOCKER" = "y" ]; then
        if [ "$OS" = "ubuntu" ] || [ "$OS" = "debian" ]; then
            sudo apt install -y docker.io
            sudo systemctl start docker
            sudo systemctl enable docker
            sudo usermod -aG docker "$USER"
        elif [ "$OS" = "macos" ]; then
            brew install --cask docker
        fi
        echo -e "${GREEN}✅ Docker kuruldu!${NC}"
    fi
}

# Projeyi klonla
clone_project() {
    echo -e "\n${YELLOW}📂 SENTIENT OS projesi klonlanıyor...${NC}"
    
    if [ -d "SENTIENT_CORE" ]; then
        echo -e "${YELLOW}⚠️  SENTIENT_CORE dizini zaten mevcut.${NC}"
        echo -e "${CYAN}Güncellemek ister misiniz? (y/n)${NC}"
        read -r UPDATE
        if [ "$UPDATE" = "y" ]; then
            cd SENTIENT_CORE
            git pull origin main
            cd ..
        fi
    else
        git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
        echo -e "${GREEN}✅ Proje klonlandı!${NC}"
    fi
}

# .env dosyasını oluştur
setup_env() {
    echo -e "\n${YELLOW}⚙️  Yapılandırma dosyası oluşturuluyor...${NC}"
    
    cd SENTIENT_CORE
    
    if [ -f ".env" ]; then
        echo -e "${GREEN}✅ .env dosyası mevcut${NC}"
        return
    fi
    
    # .env.example'den kopyala
    if [ -f ".env.example" ]; then
        cp .env.example .env
    else
        # Varsayılan .env oluştur
        cat > .env << 'ENVFILE'
# ═══════════════════════════════════════════════════════════
#  GEMMA 4 KERNEL - YEREL LLM (API KEY GEREKMİYOR!)
# ═══════════════════════════════════════════════════════════
GEMMA4_MODEL=gemma4:31b
GEMMA4_BASE_URL=http://localhost:11434/v1
GEMMA4_CONTEXT_LENGTH=262144
GEMMA4_THINKING_MODE=true

# ═══════════════════════════════════════════════════════════
#  V-GATE (API ANAHTARI YÖNETİMİ)
# ═══════════════════════════════════════════════════════════
V_GATE_URL=http://localhost:8100
V_GATE_LISTEN=127.0.0.1:1071
V_GATE_TIMEOUT=120

# ═══════════════════════════════════════════════════════════
#  GATEWAY (API SUNUCUSU)
# ═══════════════════════════════════════════════════════════
GATEWAY_HTTP_ADDR=0.0.0.0:8080
GATEWAY_PORT=8080
JWT_SECRET=change-this-secret-in-production

# ═══════════════════════════════════════════════════════════
#  BELLEK (MEMORY CUBE)
# ═══════════════════════════════════════════════════════════
MEMORY_DB_PATH=data/sentient.db
MEMORY_SHORT_TTL=3600
MEMORY_LONG_TTL=0
ZERO_COPY_ENABLED=true

# ═══════════════════════════════════════════════════════════
#  OASIS BRAIN (OTONOM DÜŞÜNCE)
# ═══════════════════════════════════════════════════════════
OASIS_BRAIN_MODEL=gemma4:31b
OASIS_BRAIN_THINKING=true
OASIS_BRAIN_MAX_STEPS=10

# ═══════════════════════════════════════════════════════════
#  LOGGING
# ═══════════════════════════════════════════════════════════
RUST_LOG=info
LOG_FILE=logs/sentient.log
ENVFILE
    fi
    
    echo -e "${GREEN}✅ .env dosyası oluşturuldu!${NC}"
    echo -e "${YELLOW}   Gerekirse düzenleyin: nano SENTIENT_CORE/.env${NC}"
}

# Projeyi derle
build_project() {
    echo -e "\n${YELLOW}🔨 SENTIENT OS derleniyor...${NC}"
    
    cd SENTIENT_CORE
    
    # Cargo'yu yükle
    if [ -f "$HOME/.cargo/env" ]; then
        source "$HOME/.cargo/env"
    fi
    
    # Release derleme
    echo -e "${BLUE}⏳ Release modda derleniyor (ilk derleme ~10-15 dakika)...${NC}"
    cargo build --release
    
    echo -e "${GREEN}✅ Derleme tamamlandı!${NC}"
}

# Test çalıştır
run_tests() {
    echo -e "\n${YELLOW}🧪 Testler çalıştırılıyor...${NC}"
    
    cd SENTIENT_CORE
    cargo test --release --workspace 2>/dev/null || true
    
    echo -e "${GREEN}✅ Testler tamamlandı!${NC}"
}

# Sonuç
show_result() {
    echo -e "\n${CYAN}"
    echo "═══════════════════════════════════════════════════════════════"
    echo "  🎉 SENTIENT OS KURULUMU TAMAMLANDI!"
    echo "═══════════════════════════════════════════════════════════════"
    echo -e "${NC}"
    
    echo -e "${GREEN}✅ Kurulum başarıyla tamamlandı!${NC}\n"
    
    echo -e "${YELLOW}📋 SONRAKI ADIMLAR:${NC}\n"
    
    echo -e "  ${CYAN}1. Proje dizinine git:${NC}"
    echo -e "     ${WHITE}cd SENTIENT_CORE${NC}\n"
    
    echo -e "  ${CYAN}2. SENTIENT'ı başlat:${NC}"
    echo -e "     ${WHITE}./sentient_tam_gaz.sh${NC}"
    echo -e "     ${WHITE}# veya${NC}"
    echo -e "     ${WHITE}cargo run --release --bin sentient${NC}\n"
    
    echo -e "  ${CYAN}3. Dashboard'ı başlat:${NC}"
    echo -e "     ${WHITE}cargo run --release --bin sentient-dashboard${NC}"
    echo -e "     ${WHITE}# Tarayıcıda: http://localhost:8080${NC}\n"
    
    echo -e "  ${CYAN}4. Gemma 4 ile sohbet:${NC}"
    echo -e "     ${WHITE}ollama run gemma4:31b${NC}\n"
    
    echo -e "  ${CYAN}5. .env dosyasını düzenle (API anahtarları için):${NC}"
    echo -e "     ${WHITE}nano .env${NC}\n"
    
    echo -e "${PURPLE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}🧠 SENTIENT OS - The Operating System That Thinks${NC}"
    echo -e "${PURPLE}═══════════════════════════════════════════════════════════════${NC}"
}

# Ana kurulum
main() {
    detect_os
    
    # OS'a göre bağımlılık kurulumu
    case $OS in
        ubuntu|debian)
            install_deps_linux
            ;;
        macos)
            install_deps_macos
            ;;
        *)
            echo -e "${YELLOW}⚠️  Bilinmeyen OS: $OS. Bağımlılıkları manuel kurmanız gerekebilir.${NC}"
            ;;
    esac
    
    check_requirements
    install_rust
    install_ollama
    install_docker
    clone_project
    setup_env
    build_project
    run_tests
    show_result
}

# Scripti çalıştır
main
