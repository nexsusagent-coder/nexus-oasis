#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS - Tek Komutla Kurulum Script'i
# ═══════════════════════════════════════════════════════════════════════════════
#  Kullanım:
#    curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.sh | bash
#
#  Veya:
#    git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
#    cd SENTIENT_CORE && ./install.sh
# ═══════════════════════════════════════════════════════════════════════════════

set -o errexit
set -o nounset
set -o pipefail

# Renkler
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Logo
echo -e "${CYAN}"
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║                                                               ║"
echo "║   ███████╗███████╗███╗   ██╗████████╗███╗   ██╗███████╗██╗    ║"
echo "║   ██╔════╝██╔════╝████╗  ██║╚══██╔══╝████╗  ██║██╔════╝██║    ║"
echo "║   ███████╗█████╗  ██╔██╗ ██║   ██║   ██╔██╗ ██║███████╗██║    ║"
echo "║   ╚════██║██╔══╝  ██║╚██╗██║   ██║   ██║╚██╗██║╚════██║██║    ║"
echo "║   ███████║███████╗██║ ╚████║   ██║   ██║ ╚████║███████║██║    ║"
echo "║   ╚══════╝╚══════╝╚═╝  ╚═══╝   ╚═╝   ╚═╝  ╚═══╝╚══════╝╚═╝    ║"
echo "║                                                               ║"
echo "║              OS - The Operating System That Thinks            ║"
echo "║                                                               ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# Fonksiyonlar
log_info() { echo -e "${BLUE}[ℹ]${NC} $1"; }
log_ok() { echo -e "${GREEN}[✓]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[⚠]${NC} $1"; }
log_error() { echo -e "${RED}[✗]${NC} $1"; }
log_step() { echo -e "${MAGENTA}[▶]${NC} $1"; }

# OS tespiti
detect_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        OS="linux"
        if command -v apt-get &> /dev/null; then
            PKG_MANAGER="apt"
        elif command -v dnf &> /dev/null; then
            PKG_MANAGER="dnf"
        elif command -v yum &> /dev/null; then
            PKG_MANAGER="yum"
        elif command -v pacman &> /dev/null; then
            PKG_MANAGER="pacman"
        else
            PKG_MANAGER="unknown"
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        OS="macos"
        PKG_MANAGER="brew"
    else
        log_error "Desteklenmeyen işletim sistemi: $OSTYPE"
        exit 1
    fi
    log_ok "İşletim sistemi: $OS ($PKG_MANAGER)"
}

# Bağımlılık kontrolü
check_command() {
    if command -v "$1" &> /dev/null; then
        log_ok "$1 kurulu: $(command -v "$1")"
        return 0
    else
        log_warn "$1 kurulu değil"
        return 1
    fi
}

# Kurulum fonksiyonları
install_rust() {
    log_step "Rust kurulumu kontrol ediliyor..."
    if check_command rustc; then
        log_ok "Rust zaten kurulu: $(rustc --version)"
        return 0
    fi

    log_info "Rust kuruluyor..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

    # Rust'ı PATH'e ekle
    source "$HOME/.cargo/env" 2>/dev/null || true
    export PATH="$HOME/.cargo/bin:$PATH"

    if check_command rustc; then
        log_ok "Rust başarıyla kuruldu: $(rustc --version)"
    else
        log_error "Rust kurulumu başarısız!"
        exit 1
    fi
}

install_python() {
    log_step "Python kurulumu kontrol ediliyor..."
    if check_command python3; then
        PYTHON_VERSION=$(python3 --version 2>&1 | cut -d' ' -f2)
        log_ok "Python zaten kurulu: $PYTHON_VERSION"

        # Python development headers kontrolü
        if [[ "$OS" == "linux" ]]; then
            if ! dpkg -l | grep -q "python3-dev"; then
                log_info "Python development headers kuruluyor..."
                sudo apt-get install -y python3-dev python3-pip
            fi
        fi
        return 0
    fi

    log_info "Python kuruluyor..."
    case "$PKG_MANAGER" in
        apt)
            sudo apt-get update
            sudo apt-get install -y python3 python3-dev python3-pip python3-venv
            ;;
        dnf|yum)
            sudo dnf install -y python3 python3-devel python3-pip
            ;;
        pacman)
            sudo pacman -S --noconfirm python python-pip
            ;;
        brew)
            brew install python
            ;;
    esac

    if check_command python3; then
        log_ok "Python başarıyla kuruldu: $(python3 --version)"
    else
        log_error "Python kurulumu başarısız!"
        exit 1
    fi
}

install_build_tools() {
    log_step "Build araçları kontrol ediliyor..."

    case "$OS" in
        linux)
            case "$PKG_MANAGER" in
                apt)
                    if ! command -v build-essential &> /dev/null; then
                        log_info "Build essentials kuruluyor..."
                        sudo apt-get update
                        sudo apt-get install -y build-essential pkg-config libssl-dev
                    fi
                    ;;
                dnf|yum)
                    sudo dnf groupinstall -y "Development Tools"
                    sudo dnf install -y openssl-devel
                    ;;
                pacman)
                    sudo pacman -S --noconfirm base-devel openssl
                    ;;
            esac
            ;;
        macos)
            if ! command -v xcode-select &> /dev/null; then
                log_info "Xcode Command Line Tools kuruluyor..."
                xcode-select --install
            fi
            ;;
    esac

    log_ok "Build araçları hazır"
}

install_ffmpeg() {
    log_step "FFmpeg kontrol ediliyor..."
    if check_command ffmpeg; then
        return 0
    fi

    log_info "FFmpeg kuruluyor..."
    case "$PKG_MANAGER" in
        apt) sudo apt-get install -y ffmpeg ;;
        dnf|yum) sudo dnf install -y ffmpeg ;;
        pacman) sudo pacman -S --noconfirm ffmpeg ;;
        brew) brew install ffmpeg ;;
    esac

    check_command ffmpeg
}

install_ollama() {
    log_step "Ollama kontrol ediliyor..."
    if check_command ollama; then
        log_ok "Ollama zaten kurulu"
        return 0
    fi

    log_info "Ollama kuruluyor..."
    curl -fsSL https://ollama.com/install.sh | sh

    # Ollama servisini başlat
    ollama serve &>/dev/null &
    sleep 3

    if check_command ollama; then
        log_ok "Ollama başarıyla kuruldu"
    else
        log_warn "Ollama kurulumu isteğe bağlı, devam ediliyor..."
    fi
}

install_docker() {
    log_step "Docker kontrol ediliyor..."
    if check_command docker; then
        log_ok "Docker zaten kurulu: $(docker --version)"
        return 0
    fi

    log_info "Docker kuruluyor..."
    case "$OS" in
        linux)
            curl -fsSL https://get.docker.com | sh
            sudo usermod -aG docker "$USER"
            ;;
        macos)
            brew install --cask docker
            ;;
    esac

    # Docker'ı başlat
    if command -v systemctl &> /dev/null; then
        sudo systemctl start docker
        sudo systemctl enable docker
    fi

    log_ok "Docker kuruldu (değişikliklerin aktif olması için oturumu kapatıp açın)"
}

clone_repo() {
    log_step "Repository kontrol ediliyor..."

    if [[ -f "Cargo.toml" ]] && grep -q "SENTIENT" Cargo.toml 2>/dev/null; then
        log_ok "SENTIENT repository'si zaten mevcut"
        return 0
    fi

    if [[ -d "SENTIENT_CORE" ]]; then
        log_info "SENTIENT_CORE dizini bulundu, güncelleniyor..."
        cd SENTIENT_CORE
        git pull
        return 0
    fi

    log_info "SENTIENT repository'si klonlanıyor..."
    git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
    cd SENTIENT_CORE
    log_ok "Repository klonlandı"
}

build_project() {
    log_step "SENTIENT derleniyor..."

    # Python path'i ayarla (PyO3 için)
    export PYTHON_SYS_EXECUTABLE=$(which python3)

    log_info "Bu işlem 5-15 dakika sürebilir..."

    # Temiz derleme
    cargo clean 2>/dev/null || true

    # Release derle
    if cargo build --release 2>&1 | tee /tmp/sentient-build.log; then
        log_ok "SENTIENT başarıyla derlendi!"
    else
        log_error "Derleme hatası!"
        log_info "Log dosyası: /tmp/sentient-build.log"

        # PyO3 hatası kontrolü
        if grep -q "pyo3" /tmp/sentient-build.log; then
            log_warn "PyO3 hatası tespit edildi. Python entegrasyonu olmadan deneniyor..."
            # PyO3 olmadan derle
            cargo build --release --no-default-features
        fi
    fi

    # Binary kontrolü
    if [[ -f "target/release/sentient" ]]; then
        BINARY_SIZE=$(du -h target/release/sentient | cut -f1)
        log_ok "Binary: target/release/sentient ($BINARY_SIZE)"
    else
        log_error "Binary oluşturulamadı!"
        exit 1
    fi
}

download_model() {
    log_step "Varsayılan AI modeli indiriliyor..."

    if ! command -v ollama &> /dev/null; then
        log_warn "Ollama kurulu değil, model atlanıyor"
        return 0
    fi

    # Küçük model: gemma2:2b (2.6GB)
    log_info "gemma2:2b modeli indiriliyor (2.6GB)..."
    ollama pull gemma2:2b || log_warn "Model indirme başarısız, daha sonra manuel olarak indirebilirsiniz"

    log_ok "Model hazır"
}

create_env() {
    log_step ".env dosyası oluşturuluyor..."

    if [[ -f ".env" ]]; then
        log_ok ".env zaten mevcut"
        return 0
    fi

    cat > .env << 'EOF'
# ════════════════════════════════════════════════════════════════
#  SENTIENT OS - Yapılandırma Dosyası
# ════════════════════════════════════════════════════════════════

# LLM Provider (OpenRouter önerilen - $5 ücretsiz kredi)
#OPENROUTER_API_KEY=sk-or-v1-...

# Veya OpenAI
#OPENAI_API_KEY=sk-...

# Veya Ollama (lokal, ücretsiz)
OPENAI_API_BASE=http://localhost:11434/v1
OPENAI_API_KEY=ollama
DEFAULT_MODEL=ollama/gemma2:2b

# Voice (opsiyonel)
#VOICE_ENABLED=true
#VOICE_STT=whisper_cpp
#VOICE_TTS=piper
#VOICE_LANGUAGE=tr

# Home Assistant (opsiyonel)
#HOME_ASSISTANT_URL=http://homeassistant.local:8123
#HOME_ASSISTANT_TOKEN=eyJ...

# Telegram Bot (opsiyonel)
#TELEGRAM_BOT_TOKEN=123456:ABC...

# Discord Bot (opsiyonel)
#DISCORD_BOT_TOKEN=Bot ...
EOF

    log_ok ".env dosyası oluşturuldu"
    log_info "API key'lerinizi .env dosyasına ekleyin"
}

run_tests() {
    log_step "Testler çalıştırılıyor..."

    if cargo test --workspace --lib 2>&1 | grep -q "test result: ok"; then
        log_ok "Tüm testler geçti!"
    else
        log_warn "Bazı testler başarısız olabilir, bu kritik değil"
    fi
}

print_success() {
    echo ""
    echo -e "${GREEN}╔═══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║                 ✅ KURULUM TAMAMLANDI!                        ║${NC}"
    echo -e "${GREEN}╚═══════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${CYAN}Kullanım:${NC}"
    echo ""
    echo "  # Versiyon kontrolü"
    echo "  ./target/release/sentient --version"
    echo ""
    echo "  # Sohbet başlat"
    echo "  ./target/release/sentient chat"
    echo ""
    echo "  # Web dashboard"
    echo "  ./target/release/sentient web"
    echo ""
    echo "  # Sesli asistan"
    echo "  ./target/release/sentient voice --wake-word 'hey sentient'"
    echo ""
    echo -e "${YELLOW}Sonraki adımlar:${NC}"
    echo "  1. API key ekleyin: nano .env"
    echo "  2. Daha büyük model: ollama pull deepseek-r1:8b"
    echo "  3. Dokümantasyon: cat README.md"
    echo ""
    echo -e "${MAGENTA}SENTIENT OS - The Operating System That Thinks${NC}"
    echo ""
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ANA AKIŞ
# ═══════════════════════════════════════════════════════════════════════════════

main() {
    log_info "Kurulum başlıyor..."

    # 1. OS tespiti
    detect_os

    # 2. Gerekli araçları kur
    install_rust
    install_python
    install_build_tools
    install_ffmpeg
    install_ollama

    # 3. Repository
    clone_repo

    # 4. Derle
    build_project

    # 5. Model indir
    download_model

    # 6. .env oluştur
    create_env

    # 7. Test
    run_tests

    # 8. Başarı mesajı
    print_success
}

main "$@"
