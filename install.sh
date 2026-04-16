#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
#  🧠 SENTIENT OS - Universal Installer (OpenClaw-Style)
#  Tek Komut: curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.sh | bash
# ═══════════════════════════════════════════════════════════════════════════════
#
#  Bu script OpenClaw kurulum mantığıyla çalışır:
#  1. Yasal uyarı (Yes/No)
#  2. Sistem tespiti ve bağımlılık kurulumu
#  3. Kurulum modu seçimi (Quick / Full / Custom)
#  4. LLM provider seçimi
#  5. Ek modüller (Voice, Browser, Desktop, Channels)
#  6. Docker servisleri (PostgreSQL, Redis, Qdrant, vs.)
#  7. Yapılandırma (.env)
#  8. Derleme
#  9. Doğrulama
#
# ═══════════════════════════════════════════════════════════════════════════════

set -e

VERSION="4.0.0"
REPO="nexsusagent-coder/SENTIENT_CORE"
INSTALL_DIR="${SENTIENT_HOME:-$HOME/sentient}"

# ── Renkler ──
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
BOLD='\033[1m'
DIM='\033[2m'
NC='\033[0m'

# ── Log ──
log_info()    { echo -e "  ${BLUE}ℹ${NC} $1"; }
log_ok()      { echo -e "  ${GREEN}✔${NC} $1"; }
log_warn()    { echo -e "  ${YELLOW}⚠${NC} $1"; }
log_err()     { echo -e "  ${RED}✖${NC} $1"; }
log_step()    { echo -e "\n${PURPLE}━━━${NC} ${BOLD}${CYAN}$1${NC} ${PURPLE}━━━${NC}\n"; }

# ── Argümanlar ──
SKIP_PROMPTS=false
INSTALL_MODE=""
while [ $# -gt 0 ]; do
    case "$1" in
        --yes|-y)       SKIP_PROMPTS=true; shift ;;
        --quick)        INSTALL_MODE="quick"; shift ;;
        --full)         INSTALL_MODE="full"; shift ;;
        --dir|-d)       INSTALL_DIR="$2"; shift 2 ;;
        --uninstall)    uninstall_sentient; exit 0 ;;
        --help|-h)      echo "Usage: $0 [--yes] [--quick|--full] [--dir PATH] [--uninstall]"; exit 0 ;;
        *)              shift ;;
    esac
done

# ═══════════════════════════════════════════════════════════════════════════════
#  BANNER
# ═══════════════════════════════════════════════════════════════════════════════

clear
echo -e "${CYAN}"
cat << 'BANNER'
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
    ║                     v4.0.0 Installer                          ║
    ║                                                               ║
    ╚═══════════════════════════════════════════════════════════════╝
BANNER
echo -e "${NC}"

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 0: YASAL UYARI (OpenClaw Standardı)
# ═══════════════════════════════════════════════════════════════════════════════

echo -e "${RED}"
echo "  ╔═════════════════════════════════════════════════════════════════════╗"
echo "  ║                     ⚠️  YASAL UYARI / LEGAL NOTICE                 ║"
echo "  ╠═════════════════════════════════════════════════════════════════════╣"
echo "  ║                                                                   ║"
echo "  ║  SENTIENT OS, yapay zeka tabanlı bir işletim sistemidir.          ║"
echo "  ║  Bu yazılımı kurarak aşağıdaki koşulları kabul etmiş olursunuz:   ║"
echo "  ║                                                                   ║"
echo "  ║  1. Bu yazılım ASILSIZ bilgi üretebilir. Üretilen içerikler      ║"
echo "  ║     doğrulanmadan kritik kararlarda kullanılmamalıdır.            ║"
echo "  ║                                                                   ║"
echo "  ║  2. API anahtarları (OpenAI, Anthropic, vb.) üçüncü taraf        ║"
echo "  ║     hizmetlerdir ve kendi kullanım koşullarına tabidir.          ║"
echo "  ║                                                                   ║"
echo "  ║  3. Yerel model çalıştırma, sistem kaynaklarını yoğun             ║"
echo "  ║     şekilde tüketebilir (RAM, GPU, disk).                         ║"
echo "  ║                                                                   ║"
echo "  ║  4. Bu yazılım AGPL v3 lisansı altında dağıtılmaktadır.          ║"
echo "  ║     Ticari kullanım için ayrı lisans gereklidir.                  ║"
echo "  ║                                                                   ║"
echo "  ║  5. Varsayılan mod: KİŞİSEL kullanım. Çok kullanıcılı erişim     ║"
echo "  ║     için LOCK-DOWN modu etkinleştirilmelidir.                     ║"
echo "  ║                                                                   ║"
echo "  ╠═════════════════════════════════════════════════════════════════════╣"
echo "  ║  This software may generate unverified information.            ║"
echo "  ║  Do NOT use generated content for critical decisions.           ║"
echo "  ║  API keys are subject to third-party terms of service.          ║"
echo "  ║  Local models may consume significant system resources.         ║"
echo "  ║  Licensed under AGPL v3. Commercial use requires a license.     ║"
echo "  ║  Default mode: PERSONAL. Multi-user requires LOCK-DOWN mode.    ║"
echo "  ╚═════════════════════════════════════════════════════════════════════╝"
echo -e "${NC}"
echo ""

if [ "$SKIP_PROMPTS" = true ]; then
    log_info "Otomatik onay (--yes flag)"
else
    read -p "  Devam etmek istiyor musunuz? [y/N]: " LEGAL_CONFIRM
    if [ "$LEGAL_CONFIRM" != "y" ] && [ "$LEGAL_CONFIRM" != "Y" ]; then
        echo ""
        log_warn "Kurulum iptal edildi."
        echo "  İstediğiniz zaman tekrar deneyebilirsiniz."
        exit 0
    fi
fi

echo ""
log_ok "Yasal uyarı kabul edildi."

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 1: SİSTEM TESPİTİ
# ═══════════════════════════════════════════════════════════════════════════════

log_step "1/8  SİSTEM TESPİTİ"

# OS
if [ -f /etc/os-release ]; then
    . /etc/os-release
    OS_ID=$ID
    OS_NAME=$PRETTY_NAME
elif [ "$(uname)" = "Darwin" ]; then
    OS_ID="macos"
    OS_NAME="macOS $(sw_vers -productVersion 2>/dev/null || echo 'Unknown')"
else
    OS_ID="linux"
    OS_NAME="Linux"
fi
log_info "OS: $OS_NAME"

# RAM
if [ "$OS_ID" = "macos" ]; then
    TOTAL_RAM=$(sysctl -n hw.memsize 2>/dev/null | awk '{print int($1/1024/1024)}' || echo "0")
else
    TOTAL_RAM=$(free -m 2>/dev/null | awk '/^Mem:/{print $2}' || echo "0")
fi
RAM_GB=$((TOTAL_RAM / 1024))
[ "$TOTAL_RAM" = "0" ] && RAM_GB="?"
if [ "$RAM_GB" != "?" ] && [ "$RAM_GB" -lt 8 ]; then
    log_warn "RAM: ${RAM_GB}GB (Önerilen: 16GB+)"
else
    log_ok "RAM: ${RAM_GB}GB"
fi

# Disk
DISK_AVAIL=$(df -h . 2>/dev/null | awk 'NR==2{print $4}' || echo "?")
log_info "Disk: ${DISK_AVAIL} boş"

# GPU
if command -v nvidia-smi &>/dev/null; then
    GPU_NAME=$(nvidia-smi --query-gpu=name --format=csv,noheader 2>/dev/null | head -1 || echo "NVIDIA GPU")
    GPU_VRAM=$(nvidia-smi --query-gpu=memory_total --format=csv,noheader 2>/dev/null | head -1 || echo "?")
    log_ok "GPU: $GPU_NAME ($GPU_VRAM)"
    HAS_GPU=true
else
    log_warn "GPU: NVIDIA GPU bulunamadı (opsiyonel)"
    HAS_GPU=false
fi

# CPU cores
CPU_CORES=$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo "?")
log_info "CPU: ${CPU_CORES} çekirdek"

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 2: KURULUM MODU
# ═══════════════════════════════════════════════════════════════════════════════

log_step "2/8  KURULUM MODU"

if [ -z "$INSTALL_MODE" ]; then
    if [ "$SKIP_PROMPTS" = true ]; then
        INSTALL_MODE="quick"
    else
        echo -e "  ${BOLD}SENTIENT OS'i nasıl kurmak istersiniz?${NC}"
        echo ""
        echo -e "  ${GREEN}1) Quick${NC}    — Hızlı kurulum (LLM + temel modüller)"
        echo "                  Önerilen: İlk kez kuruyorsanız"
        echo ""
        echo -e "  ${BLUE}2) Full${NC}     — Tam kurulum (Tüm 93 crate + Docker servisleri)"
        echo "                  Önerilen: Production deployment"
        echo ""
        echo -e "  ${YELLOW}3) Custom${NC}   — Özelleştirilmiş (Modül modül seçim)"
        echo "                  Önerilen: Geliştiriciler"
        echo ""
        echo -e "  ${DIM}4) Skip${NC}     — Sadece kaynak kodu indir, derleme"
        echo ""
        read -p "  Seçiminiz [1-4] (varsayılan: 1): " MODE_CHOICE
        MODE_CHOICE=${MODE_CHOICE:-1}

        case $MODE_CHOICE in
            1) INSTALL_MODE="quick" ;;
            2) INSTALL_MODE="full" ;;
            3) INSTALL_MODE="custom" ;;
            4) INSTALL_MODE="skip" ;;
            *) INSTALL_MODE="quick" ;;
        esac
    fi
fi

log_ok "Kurulum modu: $INSTALL_MODE"

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 3: BAĞIMLILIK KURULUMU
# ═══════════════════════════════════════════════════════════════════════════════

log_step "3/8  BAĞIMLILIK KURULUMU"

# ── Rust ──
if command -v rustc &>/dev/null; then
    log_ok "Rust: $(rustc --version)"
else
    log_info "Rust kuruluyor..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y 2>/dev/null
    source "$HOME/.cargo/env" 2>/dev/null || true
    log_ok "Rust: $(rustc --version)"
fi

# ── Sistem bağımlılıkları ──
log_info "Sistem bağımlılıkları kontrol ediliyor..."
case $OS_ID in
    ubuntu|debian|pop|linuxmint)
        sudo apt-get update -qq 2>/dev/null || true
        sudo apt-get install -y -qq build-essential pkg-config libssl-dev \
            sqlite3 libsqlite3-dev git curl wget cmake libclang-dev 2>/dev/null || true
        ;;
    fedora|rhel|centos|rocky|alma)
        sudo dnf install -y -q gcc gcc-c++ pkg-config openssl-devel \
            sqlite-devel git curl wget cmake clang-devel 2>/dev/null || true
        ;;
    macos)
        if command -v brew &>/dev/null; then
            brew install openssl sqlite3 cmake 2>/dev/null || true
        else
            log_warn "Homebrew bulunamadı. Xcode Command Line Tools gerekebilir."
        fi
        ;;
    arch|manjaro|endeavouros)
        sudo pacman -S --noconfirm --needed base-devel pkg-config openssl \
            sqlite git curl wget cmake clang 2>/dev/null || true
        ;;
    *)
        log_warn "Bilinmeyen dağıtım: $OS_ID. Manuel bağımlılık kurulumu gerekebilir."
        ;;
esac
log_ok "Sistem bağımlılıkları hazır"

# ── Docker (Full modda) ──
if [ "$INSTALL_MODE" = "full" ]; then
    if command -v docker &>/dev/null; then
        log_ok "Docker: $(docker --version 2>/dev/null | awk '{print $3}')"
    else
        log_info "Docker kuruluyor..."
        curl -fsSL https://get.docker.com | sh 2>/dev/null || log_warn "Docker kurulumu başarısız"
        sudo usermod -aG docker "$USER" 2>/dev/null || true
        log_ok "Docker kuruldu"
    fi

    if command -v docker-compose &>/dev/null || docker compose version &>/dev/null; then
        log_ok "Docker Compose hazır"
    else
        log_info "Docker Compose kuruluyor..."
        sudo curl -fsSL "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" \
            -o /usr/local/bin/docker-compose 2>/dev/null
        sudo chmod +x /usr/local/bin/docker-compose 2>/dev/null || true
        log_ok "Docker Compose kuruldu"
    fi
fi

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 4: LLM PROVIDER SEÇİMİ
# ═══════════════════════════════════════════════════════════════════════════════

log_step "4/8  LLM PROVIDER SEÇİMİ"

SELECTED_MODEL=""
SELECTED_PROVIDER=""
API_KEY_LINE=""

if [ "$SKIP_PROMPTS" = true ]; then
    # Otomatik: Ollama varsa lokal, yoksa OpenRouter
    if command -v ollama &>/dev/null; then
        SELECTED_MODEL="gemma3:4b"
        SELECTED_PROVIDER="ollama"
    else
        SELECTED_MODEL="openrouter/auto"
        SELECTED_PROVIDER="openrouter"
    fi
else
    echo -e "  ${BOLD}LLM (Büyük Dil Modeli) nasıl kullanmak istersiniz?${NC}"
    echo ""
    echo -e "  ${GREEN}╔═══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "  ${GREEN}║  🏠 YEREL (Ücretsiz, API Key Gerektirmez)                   ║${NC}"
    echo -e "  ${GREEN}╠═══════════════════════════════════════════════════════════════╣${NC}"
    echo -e "  ${GREEN}║  1) Gemma 3 4B       — 4GB VRAM, Vision  (EN HAFİF)        ║${NC}"
    echo -e "  ${GREEN}║  2) Qwen3 30B-A3B MoE — 4GB VRAM, Reasoning (ÖNERİLEN)      ║${NC}"
    echo -e "  ${GREEN}║  3) Llama 3.3 70B    — 24GB VRAM, Genel kullanım          ║${NC}"
    echo -e "  ${GREEN}║  4) DeepSeek R1 70B  — 24GB VRAM, Reasoning               ║${NC}"
    echo -e "  ${GREEN}║  5) Llama 4 Scout    — 48GB VRAM, 10M context (EN YENİ)    ║${NC}"
    echo -e "  ${GREEN}╚═══════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "  ${BLUE}╔═══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "  ${BLUE}║  🔑 API (Bulut, Daha Güçlü)                                 ║${NC}"
    echo -e "  ${BLUE}╠═══════════════════════════════════════════════════════════════╣${NC}"
    echo -e "  ${BLUE}║  6) OpenRouter       — 300+ model, $5 ücretsiz kredi       ║${NC}"
    echo -e "  ${BLUE}║  7) OpenAI GPT-4o    — Multimodal, Coding               ║${NC}"
    echo -e "  ${BLUE}║  8) Anthropic Claude — Reasoning, Coding                  ║${NC}"
    echo -e "  ${BLUE}║  9) Google Gemini    — 1M Context, Ücretsiz tier         ║${NC}"
    echo -e "  ${BLUE}║ 10) DeepSeek         — EN UCUZ API                       ║${NC}"
    echo -e "  ${BLUE}║ 11) Groq            — EN HIZLI inference                ║${NC}"
    echo -e "  ${BLUE}║ 12) Unify AI        — Akıllı router (ML-based)           ║${NC}"
    echo -e "  ${BLUE}║ 13) Portkey          — Enterprise gateway, failover       ║${NC}"
    echo -e "  ${BLUE}╚═══════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "  ${DIM}14) Atla — Daha sonra .env dosyasından yapılandır${NC}"
    echo ""
    read -p "  Seçiminiz [1-14] (varsayılan: 2): " LLM_CHOICE
    LLM_CHOICE=${LLM_CHOICE:-2}

    case $LLM_CHOICE in
        1)  SELECTED_MODEL="gemma3:4b";              SELECTED_PROVIDER="ollama" ;;
        2)  SELECTED_MODEL="qwen3:30b-a3b";          SELECTED_PROVIDER="ollama" ;;
        3)  SELECTED_MODEL="llama3.3:70b";            SELECTED_PROVIDER="ollama" ;;
        4)  SELECTED_MODEL="deepseek-r1:70b";         SELECTED_PROVIDER="ollama" ;;
        5)  SELECTED_MODEL="llama4:scout";             SELECTED_PROVIDER="ollama" ;;
        6)  SELECTED_PROVIDER="openrouter";            SELECTED_MODEL="openrouter/auto" ;;
        7)  SELECTED_PROVIDER="openai";               SELECTED_MODEL="gpt-4o" ;;
        8)  SELECTED_PROVIDER="anthropic";             SELECTED_MODEL="claude-4-sonnet" ;;
        9)  SELECTED_PROVIDER="google";                SELECTED_MODEL="gemini-2.5-flash" ;;
        10) SELECTED_PROVIDER="deepseek";              SELECTED_MODEL="deepseek-v3" ;;
        11) SELECTED_PROVIDER="groq";                  SELECTED_MODEL="llama-3.3-70b" ;;
        12) SELECTED_PROVIDER="unify";                 SELECTED_MODEL="router@q>0.9&c<0.001" ;;
        13) SELECTED_PROVIDER="portkey";                SELECTED_MODEL="portkey/gpt-4o" ;;
        14) SELECTED_PROVIDER="";                      SELECTED_MODEL="" ;;
        *)  SELECTED_MODEL="qwen3:30b-a3b";            SELECTED_PROVIDER="ollama" ;;
    esac
fi

if [ -n "$SELECTED_PROVIDER" ]; then
    log_ok "Provider: $SELECTED_PROVIDER → Model: $SELECTED_MODEL"
fi

# ── API Key prompt ──
if [ -n "$SELECTED_PROVIDER" ] && [ "$SELECTED_PROVIDER" != "ollama" ] && [ "$SELECTED_PROVIDER" != "openrouter" ]; then
    echo ""
    echo -e "  ${YELLOW}Bu model API key gerektiriyor.${NC}"
    read -p "  API Key girin (veya Enter → sonra .env'e ekleyin): " API_KEY_INPUT

    if [ -n "$API_KEY_INPUT" ]; then
        PROVIDER_UPPER=$(echo "$SELECTED_PROVIDER" | tr '[:lower:]' '[:upper:]')
        case $SELECTED_PROVIDER in
            openai)    API_KEY_LINE="OPENAI_API_KEY=${API_KEY_INPUT}" ;;
            anthropic) API_KEY_LINE="ANTHROPIC_API_KEY=${API_KEY_INPUT}" ;;
            google)    API_KEY_LINE="GOOGLE_API_KEY=${API_KEY_INPUT}" ;;
            deepseek)  API_KEY_LINE="DEEPSEEK_API_KEY=${API_KEY_INPUT}" ;;
            groq)      API_KEY_LINE="GROQ_API_KEY=${API_KEY_INPUT}" ;;
            unify)     API_KEY_LINE="UNIFY_API_KEY=${API_KEY_INPUT}" ;;
            portkey)   API_KEY_LINE="PORTKEY_API_KEY=${API_KEY_INPUT}" ;;
            *)         API_KEY_LINE="${PROVIDER_UPPER}_API_KEY=${API_KEY_INPUT}" ;;
        esac
    fi
fi

# OpenRouter özel
if [ "$SELECTED_PROVIDER" = "openrouter" ]; then
    echo ""
    echo -e "  ${YELLOW}OpenRouter API key gerektiriyor. Ücretsiz $5 kredi: https://openrouter.ai/keys${NC}"
    read -p "  OpenRouter API Key: " OR_KEY
    if [ -n "$OR_KEY" ]; then
        API_KEY_LINE="OPENROUTER_API_KEY=${OR_KEY}"
    fi
fi

# ── Ollama kurulumu (lokal seçildiyse) ──
if [ "$SELECTED_PROVIDER" = "ollama" ]; then
    if command -v ollama &>/dev/null; then
        log_ok "Ollama zaten kurulu"
    else
        log_info "Ollama kuruluyor..."
        curl -fsSL https://ollama.com/install.sh | sh 2>/dev/null || log_warn "Ollama kurulumu başarısız. Manuel: https://ollama.com"
        log_ok "Ollama kuruldu"
    fi

    # Ollama başlat
    if ! curl -s http://127.0.0.1:11434/api/tags >/dev/null 2>&1; then
        log_info "Ollama başlatılıyor..."
        ollama serve &
        sleep 3
    fi

    # Model indir
    if [ -n "$SELECTED_MODEL" ]; then
        log_info "Model indiriliyor: ${SELECTED_MODEL}..."
        log_warn "Bu işlem model boyutuna göre birkaç dakika sürebilir..."
        ollama pull "$SELECTED_MODEL" 2>/dev/null || log_warn "Model indirme başarısız. Manuel: ollama pull $SELECTED_MODEL"
        log_ok "Model hazır: $SELECTED_MODEL"
    fi
fi

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 5: EK MODÜLLER (Custom modda)
# ═══════════════════════════════════════════════════════════════════════════════

ENABLE_VOICE=false
ENABLE_BROWSER=false
ENABLE_DESKTOP=false
ENABLE_CHANNELS=false
ENABLE_DOCKER=false

if [ "$INSTALL_MODE" = "full" ]; then
    ENABLE_VOICE=true
    ENABLE_BROWSER=true
    ENABLE_DESKTOP=true
    ENABLE_CHANNELS=true
    ENABLE_DOCKER=true
elif [ "$INSTALL_MODE" = "custom" ] && [ "$SKIP_PROMPTS" = false ]; then
    log_step "5/8  EK MODÜLLER"

    echo -e "  ${BOLD}Hangi ek modülleri kurmak istersiniz?${NC}"
    echo ""

    read -p "  🎙️  Voice (Whisper STT + TTS)? [y/N]: " V
    [ "$V" = "y" ] || [ "$V" = "Y" ] && ENABLE_VOICE=true

    read -p "  🌐 Browser Automation? [y/N]: " B
    [ "$B" = "y" ] || [ "$B" = "Y" ] && ENABLE_BROWSER=true

    read -p "  🖥️  Desktop Automation (Computer Use)? [y/N]: " D
    [ "$D" = "y" ] || [ "$D" = "Y" ] && ENABLE_DESKTOP=true

    read -p "  📱 Channels (Telegram, Discord, WhatsApp)? [y/N]: " C
    [ "$C" = "y" ] || [ "$C" = "Y" ] && ENABLE_CHANNELS=true

    read -p "  🐳 Docker servisleri (PostgreSQL, Redis, Qdrant)? [y/N]: " DK
    [ "$DK" = "y" ] || [ "$DK" = "Y" ] && ENABLE_DOCKER=true
fi

if [ "$INSTALL_MODE" != "custom" ]; then
    log_info "Ek modüller: mod bazlı seçim atlandı ($INSTALL_MODE modu)"
fi

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 6: KAYNAK KODU İNDİRME
# ═══════════════════════════════════════════════════════════════════════════════

log_step "6/8  KURULUM"

if [ -d "$INSTALL_DIR/.git" ]; then
    log_info "Mevcut kurulum bulundu: $INSTALL_DIR"
    if [ "$SKIP_PROMPTS" = false ]; then
        read -p "  Güncellensin mi? [Y/n]: " UPDATE
        UPDATE=${UPDATE:-Y}
    else
        UPDATE="Y"
    fi

    if [ "$UPDATE" = "Y" ] || [ "$UPDATE" = "y" ]; then
        cd "$INSTALL_DIR"
        git pull origin main 2>/dev/null || git pull origin master 2>/dev/null || true
        log_ok "Kaynak kodu güncellendi"
    fi
else
    log_info "Kaynak kodu indiriliyor..."
    git clone https://github.com/$REPO.git "$INSTALL_DIR" 2>/dev/null || {
        log_err "Klonlama başarısız!"
        log_info "Manuel: git clone https://github.com/$REPO.git $INSTALL_DIR"
        exit 1
    }
    log_ok "Kaynak kodu indirildi"
fi

cd "$INSTALL_DIR"

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 7: YAPILANDIRMA (.env)
# ═══════════════════════════════════════════════════════════════════════════════

log_step "7/8  YAPILANDIRMA"

mkdir -p data

if [ ! -f .env ]; then
    cat > .env << ENVFILE
# ═════════════════════════════════════════════════════════════
#  SENTIENT OS Yapılandırma
#  Oluşturulma: $(date)
# ═════════════════════════════════════════════════════════════

# ── LLM ──
SENTIENT_MODEL=${SELECTED_MODEL}
SENTIENT_PROVIDER=${SELECTED_PROVIDER}
${API_KEY_LINE}
# OPENROUTER_API_KEY=sk-or-...
# OPENAI_API_KEY=sk-...
# ANTHROPIC_API_KEY=sk-ant-...

# ── Yerel LLM ──
OLLAMA_HOST=http://127.0.0.1:11434

# ── Gateway ──
GATEWAY_HTTP_ADDR=0.0.0.0:8080
JWT_SECRET=$(openssl rand -base64 64 2>/dev/null || echo "change-this-in-production")

# ── Bellek ──
MEMORY_DB_PATH=./data/sentient_memory.db

# ── Guardrails ──
GUARDRAILS_MODE=strict
GUARDRAILS_PROMPT_INJECTION=true

# ── Logging ──
RUST_LOG=info
ENVFILE
    log_ok ".env dosyası oluşturuldu"
else
    log_info ".env dosyası zaten mevcut (değiştirilmedi)"
fi

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 8: DERLEME
# ═══════════════════════════════════════════════════════════════════════════════

log_step "8/8  DERLEME"

if [ "$INSTALL_MODE" = "skip" ]; then
    log_info "Derleme atlandı (--skip modu)"
else
    source "$HOME/.cargo/env" 2>/dev/null || true

    if command -v cargo &>/dev/null; then
        log_info "SENTIENT OS derleniyor..."
        log_warn "İlk derleme 5-15 dakika sürebilir..."

        case $INSTALL_MODE in
            quick)
                cargo build --release --bin sentient 2>&1 | while IFS= read -r line; do
                    if echo "$line" | grep -q "Compiling"; then
                        echo -ne "\r  ${DIM}${line}${NC}                    "
                    elif echo "$line" | grep -q "Finished"; then
                        echo ""
                    fi
                done
                ;;
            full)
                cargo build --release 2>&1 | while IFS= read -r line; do
                    if echo "$line" | grep -q "Compiling"; then
                        echo -ne "\r  ${DIM}${line}${NC}                    "
                    elif echo "$line" | grep -q "Finished"; then
                        echo ""
                    fi
                done
                ;;
            custom)
                cargo build --release --bin sentient 2>&1 | while IFS= read -r line; do
                    if echo "$line" | grep -q "Compiling"; then
                        echo -ne "\r  ${DIM}${line}${NC}                    "
                    elif echo "$line" | grep -q "Finished"; then
                        echo ""
                    fi
                done
                ;;
        esac

        echo ""
        log_ok "SENTIENT OS derlendi"
    else
        log_err "Cargo bulunamadı! Rust'ı yükleyin: source ~/.cargo/env"
        exit 1
    fi
fi

# ── Docker servisleri (Full modda) ──
if [ "$ENABLE_DOCKER" = true ]; then
    log_info "Docker servisleri başlatılıyor..."
    docker compose up -d 2>/dev/null || docker-compose up -d 2>/dev/null || log_warn "Docker servisleri başlatılamadı"
    log_ok "Docker servisleri çalışıyor"
fi

# ═══════════════════════════════════════════════════════════════════════════════
#  PATH & ALIAS
# ═══════════════════════════════════════════════════════════════════════════════

SHELL_RC="$HOME/.bashrc"
[ -f "$HOME/.zshrc" ] && SHELL_RC="$HOME/.zshrc"

if ! grep -q "SENTIENT_HOME" "$SHELL_RC" 2>/dev/null; then
    cat >> "$SHELL_RC" << PATHFILE

# SENTIENT OS
export SENTIENT_HOME="$INSTALL_DIR"
export PATH="$INSTALL_DIR/target/release:\$PATH"
alias sentient='$INSTALL_DIR/target/release/sentient'
PATHFILE
    log_ok "PATH'e eklendi ($SHELL_RC)"
fi

export PATH="$INSTALL_DIR/target/release:$PATH"

# ═══════════════════════════════════════════════════════════════════════════════
#  SONUÇ
# ═══════════════════════════════════════════════════════════════════════════════

echo ""
echo -e "${GREEN}"
echo "  ╔═══════════════════════════════════════════════════════════════╗"
echo "  ║                                                               ║"
echo "  ║              🎉 SENTIENT OS KURULUMU TAMAMLANDI!             ║"
echo "  ║                                                               ║"
echo "  ╚═══════════════════════════════════════════════════════════════╝"
echo -e "${NC}"
echo ""
echo -e "  ${BOLD}📋 Kurulum Özeti:${NC}"
echo -e "  ${CYAN}Mod:${NC}       $INSTALL_MODE"
echo -e "  ${CYAN}Provider:${NC}  ${SELECTED_PROVIDER:-yapılandırılmadı}"
echo -e "  ${CYAN}Model:${NC}     ${SELECTED_MODEL:-yapılandırılmadı}"
echo -e "  ${CYAN}Dizin:${NC}     $INSTALL_DIR"
echo -e "  ${CYAN}Voice:${NC}    $([ "$ENABLE_VOICE" = true ] && echo "✅" || echo "❌")"
echo -e "  ${CYAN}Browser:${NC}  $([ "$ENABLE_BROWSER" = true ] && echo "✅" || echo "❌")"
echo -e "  ${CYAN}Desktop:${NC}  $([ "$ENABLE_DESKTOP" = true ] && echo "✅" || echo "❌")"
echo -e "  ${CYAN}Channels:${NC} $([ "$ENABLE_CHANNELS" = true ] && echo "✅" || echo "❌")"
echo -e "  ${CYAN}Docker:${NC}  $([ "$ENABLE_DOCKER" = true ] && echo "✅" || echo "❌")"
echo ""
echo -e "  ${BOLD}🚀 Başlatmak için:${NC}"
echo ""
echo "  source ~/.bashrc  # veya ~/.zshrc"
echo "  sentient chat     # İnteraktif sohbet"
echo "  sentient gateway  # API sunucusu (http://localhost:8080)"
echo "  sentient status   # Sistem durumu"
echo ""
echo -e "  ${BOLD}⚙️  Yapılandırma:${NC}"
echo ""
echo "  nano $INSTALL_DIR/.env"
echo ""
echo -e "  ${BOLD}📚 Dokümantasyon:${NC}"
echo ""
echo "  cat $INSTALL_DIR/README.md"
echo "  cat $INSTALL_DIR/docs/"
echo ""
echo -e "  ${PURPLE}═══════════════════════════════════════════════════════════════${NC}"
echo -e "  ${GREEN}🧠 SENTIENT OS - The Operating System That Thinks${NC}"
echo -e "  ${PURPLE}═══════════════════════════════════════════════════════════════${NC}"

# ── Uninstall function ──
uninstall_sentient() {
    log_info "SENTIENT OS kaldırılıyor..."
    rm -rf "$INSTALL_DIR"
    for rc in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile"; do
        [ -f "$rc" ] && sed -i '/# SENTIENT OS/,+3d' "$rc" 2>/dev/null || true
    done
    log_ok "SENTIENT OS kaldırıldı"
}
