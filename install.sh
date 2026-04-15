#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS - TEK KOMUT KURULUM
# ═══════════════════════════════════════════════════════════════════════════════
#  Kullanım: ./install.sh
#  Veya:     curl -fsSL https://sentient.dev/install.sh | bash
#
#  OpenClaw tarzı: Basit, temiz, seçim odaklı
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# ═══════════════════════════════════════════════════════════════════════════════
#  RENKLER VE YARDIMCI FONKSİYONLAR
# ═══════════════════════════════════════════════════════════════════════════════
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

logo() {
    echo -e "${CYAN}"
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║     ███████╗███████╗███╗   ██╗████████╗██████╗  ██████╗   ║"
    echo "║     ██╔════╝██╔════╝████╗  ██║╚══██╔══╝██╔══██╗██╔═══██╗  ║"
    echo "║     ███████╗█████╗  ██╔██╗ ██║   ██║   ██████╔╝██║   ██║  ║"
    echo "║     ╚════██║██╔══╝  ██║╚██╗██║   ██║   ██╔══██╗██║   ██║  ║"
    echo "║     ███████║███████╗██║ ╚████║   ██║   ██║  ██║╚██████╔╝  ║"
    echo "║     ╚══════╝╚══════╝╚═╝  ╚═══╝   ╚═╝   ╚═╝  ╚═╝ ╚═════╝   ║"
    echo "║                                                            ║"
    echo "║              🧠 Yapay Zeka İşletim Sistemi                 ║"
    echo "╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

step() { echo -e "\n${CYAN}▶ $1${NC}"; }
ok() { echo -e "${GREEN}✓ $1${NC}"; }
warn() { echo -e "${YELLOW}! $1${NC}"; }
err() { echo -e "${RED}✗ $1${NC}"; exit 1; }
ask() { echo -e -n "${BOLD}$1${NC}"; read -r ans; echo "$ans"; }

# ═══════════════════════════════════════════════════════════════════════════════
#  1. SİSTEM KONTROL
# ═══════════════════════════════════════════════════════════════════════════════
check_system() {
    step "Sistem kontrol ediliyor..."
    
    # OS kontrol
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        ok "Linux detected"
        PKG_MANAGER="apt"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        ok "macOS detected"
        PKG_MANAGER="brew"
    else
        warn "Bilinmeyen OS: $OSTYPE"
    fi
    
    # Rust kontrol
    if command -v rustc &> /dev/null; then
        ok "Rust: $(rustc --version)"
    else
        warn "Rust yok, kuruluyor..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env" 2>/dev/null || true
        ok "Rust kuruldu"
    fi
    
    # Diğer bağımlılıklar
    local deps="curl git"
    for dep in $deps; do
        command -v "$dep" &> /dev/null && ok "$dep" || err "$dep gerekli ama yok"
    done
}

# ═══════════════════════════════════════════════════════════════════════════════
#  2. LLM SEÇİMİ (ANA SEÇİM)
# ═══════════════════════════════════════════════════════════════════════════════
select_llm() {
    echo ""
    echo -e "${BOLD}════════════════════════════════════════════════════════════${NC}"
    echo -e "${BOLD}  🧠 LLM (Yapay Zeka) Nasıl Kullanmak İstersiniz?${NC}"
    echo -e "${BOLD}════════════════════════════════════════════════════════════${NC}"
    echo ""
    echo -e "  ${GREEN}[1]${NC} API Key kullanacağım (OpenAI, Anthropic, OpenRouter...)"
    echo "      → Hızlı kurulum, en iyi kalite, ücretli"
    echo ""
    echo -e "  ${GREEN}[2]${NC} Lokal model kullanacağım (Ollama)"
    echo "      → Ücretsiz, gizli, GPU gerektirir"
    echo ""
    echo -e "  ${GREEN}[3]${NC} Şimdilik atla, sonra ayarlayacağım"
    echo "      → Config dosyasını manuel düzenleyeceğim"
    echo ""
    
    local choice=$(ask "Seçiminiz [1/2/3]: ")
    
    case $choice in
        1) setup_api_llm ;;
        2) setup_local_llm ;;
        3) warn "LLM atlandı. .env dosyasını düzenleyin." ;;
        *) setup_api_llm ;;
    esac
}

# ═══════════════════════════════════════════════════════════════════════════════
#  API LLM KURULUMU
# ═══════════════════════════════════════════════════════════════════════════════
setup_api_llm() {
    echo ""
    echo -e "${BOLD}════════════════════════════════════════════════════════════${NC}"
    echo -e "${BOLD}  🔑 API Sağlayıcı Seçimi${NC}"
    echo -e "${BOLD}════════════════════════════════════════════════════════════${NC}"
    echo ""
    echo -e "  ${GREEN}[1]${NC} OpenRouter (önerilen) - 200+ model"
    echo -e "  ${GREEN}[2]${NC} OpenAI (GPT-4, GPT-4o)"
    echo -e "  ${GREEN}[3]${NC} Anthropic (Claude)"
    echo -e "  ${GREEN}[4]${NC} Google AI (Gemini)"
    echo ""
    
    local provider=$(ask "Sağlayıcı [1-4]: ")
    
    case $provider in
        1)
            LLM_PROVIDER="openrouter"
            echo ""
            echo -e "${CYAN}OpenRouter API Key:${NC} https://openrouter.ai/keys"
            local key=$(ask "API Key: ")
            
            echo ""
            echo -e "Varsayılan model:"
            echo -e "  [1] GPT-4o-mini (hızlı, ucuz)"
            echo -e "  [2] Claude 3.5 Sonnet"
            echo -e "  [3] Gemini 2.0 Flash"
            local model_choice=$(ask "Model [1-3]: ")
            
            case $model_choice in
                2) LLM_MODEL="anthropic/claude-3.5-sonnet" ;;
                3) LLM_MODEL="google/gemini-2.0-flash-exp" ;;
                *) LLM_MODEL="openai/gpt-4o-mini" ;;
            esac
            
            API_KEY="$key"
            API_ENV="OPENROUTER_API_KEY"
            ;;
        2)
            LLM_PROVIDER="openai"
            echo -e "${CYAN}OpenAI API Key:${NC} https://platform.openai.com/api-keys"
            API_KEY=$(ask "API Key: ")
            LLM_MODEL="gpt-4o-mini"
            API_ENV="OPENAI_API_KEY"
            ;;
        3)
            LLM_PROVIDER="anthropic"
            echo -e "${CYAN}Anthropic API Key:${NC} https://console.anthropic.com/"
            API_KEY=$(ask "API Key: ")
            LLM_MODEL="claude-3-5-sonnet-20241022"
            API_ENV="ANTHROPIC_API_KEY"
            ;;
        4)
            LLM_PROVIDER="google"
            echo -e "${CYAN}Google AI Key:${NC} https://aistudio.google.com/apikey"
            API_KEY=$(ask "API Key: ")
            LLM_MODEL="gemini-2.0-flash"
            API_ENV="GOOGLE_API_KEY"
            ;;
    esac
    
    # .env'e yaz (key hariç!)
    cat >> .env << EOF
LLM_PROVIDER=$LLM_PROVIDER
LLM_MODEL=$LLM_MODEL
EOF
    
    # Key'i shell'e export et ama dosyaya yazma
    export "$API_ENV"="$API_KEY"
    
    ok "API yapılandırıldı: $LLM_PROVIDER / $LLM_MODEL"
    warn "API Key .env'e YAZILMADI. Kullanırken: $API_ENV=... ./sentient"
}

# ═══════════════════════════════════════════════════════════════════════════════
#  LOKAL LLM KURULUMU (OLLAMA) - SADECE BU SEÇİLİRSE ÇALIŞIR
# ═══════════════════════════════════════════════════════════════════════════════
setup_local_llm() {
    echo ""
    echo -e "${BOLD}════════════════════════════════════════════════════════════${NC}"
    echo -e "${BOLD}  🦙 Ollama Kurulumu (Lokal LLM)${NC}"
    echo -e "${BOLD}════════════════════════════════════════════════════════════${NC}"
    echo ""
    
    # Ollama kurulu mu?
    if command -v ollama &> /dev/null; then
        ok "Ollama zaten kurulu"
    else
        echo -e "${YELLOW}Ollama kuruluyor...${NC}"
        curl -fsSL https://ollama.com/install.sh | sh
        ok "Ollama kuruldu"
    fi
    
    # Servis başlat
    if ! pgrep -x "ollama" > /dev/null; then
        ollama serve &
        sleep 3
    fi
    ok "Ollama servisi çalışıyor"
    
    # Model seçimi
    echo ""
    echo -e "Model seçin:"
    echo -e "  ${GREEN}[1]${NC} Llama 3.2 (3B)    ~2GB   - Hızlı"
    echo -e "  ${GREEN}[2]${NC} Gemma 3 (4B)     ~3GB   - Dengeli"
    echo -e "  ${GREEN}[3]${NC} Qwen 2.5 (7B)    ~4GB   - Türkçe iyi"
    echo -e "  ${GREEN}[4]${NC} DeepSeek R1 (7B) ~4GB   - Reasoning"
    echo ""
    
    local model_choice=$(ask "Model [1-4, default=2]: ")
    model_choice=${model_choice:-2}
    
    case $model_choice in
        1) OLLAMA_MODEL="llama3.2" ;;
        2) OLLAMA_MODEL="gemma3:4b" ;;
        3) OLLAMA_MODEL="qwen2.5:7b" ;;
        4) OLLAMA_MODEL="deepseek-r1:7b" ;;
        *) OLLAMA_MODEL="gemma3:4b" ;;
    esac
    
    # Model var mı kontrol et, yoksa indir
    if ollama list 2>/dev/null | grep -q "$OLLAMA_MODEL"; then
        ok "Model $OLLAMA_MODEL zaten mevcut"
    else
        echo -e "${YELLOW}Model indiriliyor: $OLLAMA_MODEL (bu biraz sürebilir)...${NC}"
        ollama pull "$OLLAMA_MODEL"
        ok "Model $OLLAMA_MODEL indirildi"
    fi
    
    # .env'e yaz
    cat >> .env << EOF
LLM_PROVIDER=ollama
OLLAMA_HOST=http://localhost:11434
OLLAMA_MODEL=$OLLAMA_MODEL
EOF
    
    ok "Ollama hazır: $OLLAMA_MODEL"
}

# ═══════════════════════════════════════════════════════════════════════════════
#  3. EK ÖZELLİKLER (İSTEĞE BAĞLI)
# ═══════════════════════════════════════════════════════════════════════════════
ask_extras() {
    echo ""
    echo -e "${BOLD}════════════════════════════════════════════════════════════${NC}"
    echo -e "${BOLD}  🔧 Ek Özellikler${NC}"
    echo -e "${BOLD}════════════════════════════════════════════════════════════${NC}"
    echo ""
    
    # Voice
    echo -e "${CYAN}Ses (Voice) özelliği kurulsun mu?${NC}"
    echo -e "  [y] Evet  [n] Hayır (default)"
    local voice=$(ask "Voice [y/N]: ")
    if [[ "$voice" =~ ^[Yy]$ ]]; then
        echo "VOICE_ENABLED=true" >> .env
        ok "Voice aktif edildi (ayarlar: .env)"
    else
        echo "VOICE_ENABLED=false" >> .env
    fi
    
    # Dashboard
    echo ""
    echo -e "${CYAN}Dashboard arayüzü kurulsun mu?${NC}"
    echo -e "  [y] Evet  [n] Hayır (default)"
    local dash=$(ask "Dashboard [y/N]: ")
    if [[ "$dash" =~ ^[Yy]$ ]]; then
        DASHBOARD=true
        echo "DASHBOARD_ENABLED=true" >> .env
    else
        DASHBOARD=false
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
#  4. DERLEME
# ═══════════════════════════════════════════════════════════════════════════════
build() {
    echo ""
    echo -e "${BOLD}════════════════════════════════════════════════════════════${NC}"
    echo -e "${BOLD}  🦀 Derleniyor...${NC}"
    echo -e "${BOLD}════════════════════════════════════════════════════════════${NC}"
    echo ""
    
    # Binary zaten varsa atla
    if [ -f "target/release/sentient" ]; then
        ok "Binary zaten mevcut: target/release/sentient"
        return
    fi
    
    # Cargo.toml var mı?
    if [ ! -f "Cargo.toml" ]; then
        err "Cargo.toml bulunamadı! Doğru dizinde misiniz?"
    fi
    
    echo -e "${YELLOW}Bu işlem birkaç dakika sürebilir...${NC}"
    cargo build --release --bin sentient 2>&1 | grep -E "Compiling|Finished|error" || true
    
    if [ -f "target/release/sentient" ]; then
        ok "Derleme tamamlandı: target/release/sentient"
    else
        err "Derleme başarısız!"
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
#  5. SONUÇ
# ═══════════════════════════════════════════════════════════════════════════════
show_result() {
    echo ""
    echo -e "${GREEN}╔════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║              ✅ KURULUM TAMAMLANDI!                            ║${NC}"
    echo -e "${GREEN}╚════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    
    echo -e "${BOLD}Kullanım:${NC}"
    echo ""
    
    if [ -n "$API_ENV" ]; then
        echo -e "  ${CYAN}$API_ENV=your_key ./target/release/sentient chat${NC}"
    elif [ -n "$OLLAMA_MODEL" ]; then
        echo -e "  ${CYAN}./target/release/sentient chat${NC}"
    else
        echo -e "  ${CYAN}./target/release/sentient chat${NC}"
        echo -e "  ${YELLOW}(Önce .env dosyasına LLM ayarlarını yapın)${NC}"
    fi
    
    echo ""
    echo -e "${BOLD}Diğer komutlar:${NC}"
    echo "  ./target/release/sentient status      # Durum"
    echo "  ./target/release/sentient gateway     # API sunucusu"
    echo "  ./target/release/sentient --help      # Yardım"
    echo ""
    
    if [ "$DASHBOARD" = true ]; then
        echo -e "${BOLD}Dashboard:${NC} http://localhost:8080"
        echo ""
    fi
    
    echo -e "${BOLD}Config:${NC} .env"
    echo ""
    echo -e "${GREEN}🚀 İyi kullanımlar!${NC}"
}

# ═══════════════════════════════════════════════════════════════════════════════
#  MAIN
# ═══════════════════════════════════════════════════════════════════════════════
main() {
    logo
    
    # Script'in bulunduğu dizine git
    cd "$(dirname "$0")"
    
    # .env oluştur (yoksa)
    touch .env
    
    # Adımlar
    check_system
    select_llm
    ask_extras
    build
    show_result
}

main "$@"
