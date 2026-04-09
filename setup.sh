#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
#  🧠 SENTIENT OS - Interactive Onboarding Wizard
#  The Operating System That Thinks
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# ─────────────────────────────────────────────────────────────────────────────
# COLORS & STYLING
# ─────────────────────────────────────────────────────────────────────────────
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
BOLD='\033[1m'
DIM='\033[2m'
NC='\033[0m'

# ─────────────────────────────────────────────────────────────────────────────
# GLOBAL STATE
# ─────────────────────────────────────────────────────────────────────────────
declare -A SELECTED_COMPONENTS
SELECTED_LLM=""
SELECTED_LLM_PROVIDER=""
SELECTED_CHANNELS=()
INSTALL_DIR="$HOME/sentient"
API_KEYS={}

# ─────────────────────────────────────────────────────────────────────────────
# UI HELPERS
# ─────────────────────────────────────────────────────────────────────────────

clear_screen() {
    clear
}

print_header() {
    local title="$1"
    local step="$2"
    local total="$3"
    
    clear_screen
    echo -e "${CYAN}"
    echo "╔═══════════════════════════════════════════════════════════════════════════╗"
    echo "║                                                                           ║"
    echo "║     ███████╗███████╗███╗   ██╗████████╗██╗ ██████╗ █████╗                 ║"
    echo "║     ██╔════╝██╔════╝████╗  ██║╚══██╔══╝██║██╔════╝██╔══██╗                ║"
    echo "║     ███████╗█████╗  ██╔██╗ ██║   ██║   ██║██║     ███████║                ║"
    echo "║     ╚════██║██╔══╝  ██║╚██╗██║   ██║   ██║██║     ██╔══██║                ║"
    echo "║     ███████║███████╗██║ ╚████║   ██║   ██║╚██████╗██║  ██║                ║"
    echo "║     ╚══════╝╚══════╝╚═╝  ╚═══╝   ╚═╝   ╚═╝ ╚═════╝╚═╝  ╚═╝                ║"
    echo "║                                                                           ║"
    echo "║              ${BOLD}🧠 The Operating System That Thinks${NC}${CYAN}                          ║"
    echo "║                                                                           ║"
    echo "╚═══════════════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    
    if [[ -n "$step" && -n "$total" ]]; then
        echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "${BOLD}${WHITE}  ADIM $step/$total: $title${NC}"
        echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    fi
    echo ""
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_checkbox() {
    local checked="$1"
    local label="$2"
    local desc="$3"
    
    if [[ "$checked" == "true" ]]; then
        echo -e "  ${GREEN}[✓]${NC} ${BOLD}$label${NC} ${DIM}$desc${NC}"
    else
        echo -e "  ${DIM}[ ]${NC} $label ${DIM}$desc${NC}"
    fi
}

print_menu_item() {
    local num="$1"
    local label="$2"
    local desc="$3"
    local status="$4"
    
    if [[ "$status" == "selected" ]]; then
        echo -e "  ${GREEN}→${NC} ${BOLD}${num})${NC} $label ${DIM}$desc${NC}"
    else
        echo -e "    ${num}) $label ${DIM}$desc${NC}"
    fi
}

press_enter() {
    echo ""
    echo -e "${DIM}Devam etmek için Enter'a basın...${NC}"
    read -r
}

confirm() {
    local prompt="$1"
    local default="${2:-y}"
    
    local choice
    if [[ "$default" == "y" ]]; then
        echo -ne "${CYAN}$prompt [Y/n]: ${NC}"
    else
        echo -ne "${CYAN}$prompt [y/N]: ${NC}"
    fi
    
    read -r choice
    choice=${choice:-$default}
    
    [[ "$choice" =~ ^[Yy]$ ]]
}

# ─────────────────────────────────────────────────────────────────────────────
# STEP 1: WELCOME & ACCEPTANCE
# ─────────────────────────────────────────────────────────────────────────────

step_welcome() {
    print_header "KARŞILAMA VE KURULUM ONAYI" "1" "4"
    
    echo -e "${WHITE}SENTIENT OS'e hoş geldiniz!${NC}"
    echo ""
    echo -e "Bu sihirbaz, size adım adım rehberlik edecek ve ihtiyaçlarınıza göre"
    echo -e "özelleştirilmiş bir kurulum yapacaktır."
    echo ""
    echo -e "${BOLD}Kurulum şunları içerecek:${NC}"
    echo ""
    echo -e "  ${CYAN}◆${NC} Sistem gereksinimleri kontrolü"
    echo -e "  ${CYAN}◆${NC} LLM (Yapay Zeka) modeli seçimi"
    echo -e "  ${CYAN}◆${NC} Mesajlaşma kanalları yapılandırması"
    echo -e "  ${CYAN}◆${NC} Temel bağımlılıkların kurulumu"
    echo ""
    
    # Sistem bilgisi
    echo -e "${DIM}────────────────────────────────────────────────────────────────────────${NC}"
    echo -e "${DIM}Sistem: $(uname -s) $(uname -m)${NC}"
    echo -e "${DIM}Kullanıcı: $USER${NC}"
    echo -e "${DIM}Dizin: $INSTALL_DIR${NC}"
    echo -e "${DIM}────────────────────────────────────────────────────────────────────────${NC}"
    echo ""
    
    if confirm "Kuruluma başlamak istiyor musunuz?" "y"; then
        return 0
    else
        echo ""
        print_info "Kurulum iptal edildi. Görüşmek üzere!"
        exit 0
    fi
}

# ─────────────────────────────────────────────────────────────────────────────
# STEP 2: LLM SELECTION
# ─────────────────────────────────────────────────────────────────────────────

step_llm_selection() {
    print_header "LLM (YAPAY ZEKA) MODELİ SEÇİMİ" "2" "4"
    
    echo -e "${WHITE}SENTIENT OS hangi yapay zeka modelini kullanmasını istersiniz?${NC}"
    echo ""
    
    # Local Models
    echo -e "${GREEN}╔═════════════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║  🏠 YEREL MODELLER (API Key Gerektirmez, Tam Gizlilik)               ║${NC}"
    echo -e "${GREEN}╠═════════════════════════════════════════════════════════════════════════╣${NC}"
    echo -e "${GREEN}║${NC}  1) Ollama - Gemma 4 31B     ${DIM}256K context, Thinking Mode${NC}        ${GREEN}║${NC}"
    echo -e "${GREEN}║${NC}  2) Ollama - Llama 3.3 70B   ${DIM}128K context, Genel kullanım${NC}       ${GREEN}║${NC}"
    echo -e "${GREEN}║${NC}  3) Ollama - Qwen 2.5 72B    ${DIM}128K context, Coding optimize${NC}      ${GREEN}║${NC}"
    echo -e "${GREEN}║${NC}  4) Ollama - DeepSeek R1     ${DIM}128K context, Reasoning${NC}           ${GREEN}║${NC}"
    echo -e "${GREEN}║${NC}  5) Ollama - Mistral 24B     ${DIM}128K context, Hızlı${NC}               ${GREEN}║${NC}"
    echo -e "${GREEN}╚═════════════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    
    # Cloud Models
    echo -e "${BLUE}╔═════════════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║  ☁️  BULUT MODELLER (API Key Gerekli)                                 ║${NC}"
    echo -e "${BLUE}╠═════════════════════════════════════════════════════════════════════════╣${NC}"
    echo -e "${BLUE}║${NC}  6) OpenAI GPT-4o            ${DIM}Multimodal, Coding${NC}                ${BLUE}║${NC}"
    echo -e "${BLUE}║${NC}  7) Anthropic Claude 3.7     ${DIM}Coding, Reasoning${NC}                 ${BLUE}║${NC}"
    echo -e "${BLUE}║${NC}  8) Google Gemini 2.0        ${DIM}1M Context${NC}                        ${BLUE}║${NC}"
    echo -e "${BLUE}║${NC}  9) Groq Llama 3.3           ${DIM}Hızlı Inference${NC}                   ${BLUE}║${NC}"
    echo -e "${BLUE}║${NC} 10) OpenRouter (Free Tier)   ${DIM}Ücretsiz modeller${NC}                  ${BLUE}║${NC}"
    echo -e "${BLUE}╚═════════════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    
    # Skip option
    echo -e "${YELLOW}  0) LLM kurulumunu atla${NC} ${DIM}(Daha sonra yapılandıracağım)${NC}"
    echo ""
    
    while true; do
        echo -ne "${CYAN}Seçiminiz [1-10, 0=atla]: ${NC}"
        read -r choice
        
        case "$choice" in
            0)
                SELECTED_LLM="skip"
                SELECTED_LLM_PROVIDER="none"
                print_info "LLM kurulumu atlandı. Daha sonra .env dosyasından yapılandırabilirsiniz."
                return 0
                ;;
            1)
                SELECTED_LLM="gemma4:31b"
                SELECTED_LLM_PROVIDER="ollama"
                break
                ;;
            2)
                SELECTED_LLM="llama3.3:70b"
                SELECTED_LLM_PROVIDER="ollama"
                break
                ;;
            3)
                SELECTED_LLM="qwen2.5:72b"
                SELECTED_LLM_PROVIDER="ollama"
                break
                ;;
            4)
                SELECTED_LLM="deepseek-r1:67b"
                SELECTED_LLM_PROVIDER="ollama"
                break
                ;;
            5)
                SELECTED_LLM="mistral:24b"
                SELECTED_LLM_PROVIDER="ollama"
                break
                ;;
            6)
                SELECTED_LLM="gpt-4o"
                SELECTED_LLM_PROVIDER="openai"
                break
                ;;
            7)
                SELECTED_LLM="claude-3.7-sonnet"
                SELECTED_LLM_PROVIDER="anthropic"
                break
                ;;
            8)
                SELECTED_LLM="gemini-2.0-flash"
                SELECTED_LLM_PROVIDER="google"
                break
                ;;
            9)
                SELECTED_LLM="llama-3.3-70b-versatile"
                SELECTED_LLM_PROVIDER="groq"
                break
                ;;
            10)
                SELECTED_LLM="google/gemma-4-31b-it:free"
                SELECTED_LLM_PROVIDER="openrouter"
                break
                ;;
            *)
                print_warning "Geçersiz seçim. Lütfen 0-10 arası bir sayı girin."
                continue
                ;;
        esac
    done
    
    print_success "Seçilen model: $SELECTED_LLM ($SELECTED_LLM_PROVIDER)"
    
    # API Key for cloud providers
    if [[ "$SELECTED_LLM_PROVIDER" != "ollama" && "$SELECTED_LLM_PROVIDER" != "none" ]]; then
        echo ""
        print_warning "$SELECTED_LLM_PROVIDER API key gereklidir."
        echo -ne "${CYAN}$SELECTED_LLM_PROVIDER API Key: ${NC}"
        read -r api_key
        
        if [[ -n "$api_key" ]]; then
            API_KEYS["$SELECTED_LLM_PROVIDER"]="$api_key"
            print_success "API key kaydedildi."
        else
            print_warning "API key girilmedi. Daha sonra .env dosyasından ekleyebilirsiniz."
        fi
    fi
    
    echo ""
}

# ─────────────────────────────────────────────────────────────────────────────
# STEP 3: MESSAGING CHANNELS
# ─────────────────────────────────────────────────────────────────────────────

step_messaging_channels() {
    print_header "MESAJLAŞMA KANALLARI" "3" "4"
    
    echo -e "${WHITE}SENTIENT OS'i hangi platformlarda kullanmak istersiniz?${NC}"
    echo ""
    echo -e "${DIM}İstediğiniz kadar kanal seçebilirsiniz. Seçmediklerinizi atlayabilirsiniz.${NC}"
    echo ""
    
    local channels=("telegram" "whatsapp" "discord" "slack" "web" "api")
    local descriptions=("Telegram Bot" "WhatsApp Business" "Discord Bot" "Slack App" "Web Arayüzü" "REST API")
    local selected_channels=()
    
    echo -e "${BOLD}Kanal Seçimi:${NC}"
    echo ""
    
    # Telegram
    echo -e "${CYAN}━━━ Telegram Bot ━━━${NC}"
    if confirm "Telegram bot bağlamak istiyor musunuz?" "n"; then
        selected_channels+=("telegram")
        echo -ne "  ${YELLOW}Telegram Bot Token: ${NC}"
        read -r telegram_token
        echo -ne "  ${YELLOW}Telegram Chat ID: ${NC}"
        read -r telegram_chat_id
        API_KEYS["telegram_token"]="$telegram_token"
        API_KEYS["telegram_chat_id"]="$telegram_chat_id"
        print_success "Telegram yapılandırıldı."
    fi
    echo ""
    
    # WhatsApp
    echo -e "${CYAN}━━━ WhatsApp Business ━━━${NC}"
    if confirm "WhatsApp Business API bağlamak istiyor musunuz?" "n"; then
        selected_channels+=("whatsapp")
        echo -ne "  ${YELLOW}WhatsApp Phone Number ID: ${NC}"
        read -r wa_phone_id
        echo -ne "  ${YELLOW}WhatsApp Access Token: ${NC}"
        read -r wa_token
        API_KEYS["whatsapp_phone_id"]="$wa_phone_id"
        API_KEYS["whatsapp_token"]="$wa_token"
        print_success "WhatsApp yapılandırıldı."
    fi
    echo ""
    
    # Discord
    echo -e "${CYAN}━━━ Discord Bot ━━━${NC}"
    if confirm "Discord bot bağlamak istiyor musunuz?" "n"; then
        selected_channels+=("discord")
        echo -ne "  ${YELLOW}Discord Bot Token: ${NC}"
        read -r discord_token
        API_KEYS["discord_token"]="$discord_token"
        print_success "Discord yapılandırıldı."
    fi
    echo ""
    
    # Slack
    echo -e "${CYAN}━━━ Slack App ━━━${NC}"
    if confirm "Slack app bağlamak istiyor musunuz?" "n"; then
        selected_channels+=("slack")
        echo -ne "  ${YELLOW}Slack Bot Token (xoxb-...): ${NC}"
        read -r slack_token
        API_KEYS["slack_token"]="$slack_token"
        print_success "Slack yapılandırıldı."
    fi
    echo ""
    
    # Web Interface (always included)
    echo -e "${CYAN}━━━ Web Arayüzü ━━━${NC}"
    if confirm "Web arayüzü kurulacak (önerilir)?" "y"; then
        selected_channels+=("web")
        print_success "Web arayüzü eklendi."
    fi
    echo ""
    
    # API
    echo -e "${CYAN}━━━ REST API ━━━${NC}"
    if confirm "REST API erişimi açılsın mı?" "y"; then
        selected_channels+=("api")
        print_success "REST API eklendi."
    fi
    echo ""
    
    SELECTED_CHANNELS=("${selected_channels[@]}")
    
    if [[ ${#SELECTED_CHANNELS[@]} -eq 0 ]]; then
        print_warning "Hiçbir kanal seçilmedi. Yerel kullanım için devam edilecek."
    else
        print_success "Seçilen kanallar: ${SELECTED_CHANNELS[*]}"
    fi
}

# ─────────────────────────────────────────────────────────────────────────────
# STEP 4: INSTALLATION
# ─────────────────────────────────────────────────────────────────────────────

step_installation() {
    print_header "KURULUM" "4" "4"
    
    echo -e "${WHITE}Seçimleriniz kaydedildi. Şimdi kurulum başlıyor...${NC}"
    echo ""
    
    # Summary
    echo -e "${BOLD}Kurulum Özeti:${NC}"
    echo ""
    echo -e "  ${CYAN}LLM:${NC}     ${SELECTED_LLM:-Atlandı}"
    echo -e "  ${CYAN}Kanallar:${NC} ${SELECTED_CHANNELS[*]:-Yok}"
    echo -e "  ${CYAN}Dizin:${NC}   $INSTALL_DIR"
    echo ""
    
    if ! confirm "Kuruluma devam edilsin mi?" "y"; then
        print_info "Kurulum iptal edildi."
        exit 0
    fi
    
    echo ""
    echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    
    # 1. System Check
    print_info "Sistem kontrol ediliyor..."
    
    # Check Rust
    if ! command -v rustc &> /dev/null; then
        print_info "Rust kuruluyor..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        print_success "Rust kuruldu."
    else
        print_success "Rust mevcut: $(rustc --version)"
    fi
    
    # Check Git
    if ! command -v git &> /dev/null; then
        print_warning "Git bulunamadı. Lütfen Git'i kurun."
    else
        print_success "Git mevcut."
    fi
    
    # 2. Clone Repository
    echo ""
    print_info "SENTIENT OS indiriliyor..."
    
    if [[ -d "$INSTALL_DIR" ]]; then
        print_warning "$INSTALL_DIR zaten mevcut."
        cd "$INSTALL_DIR"
        git pull origin main 2>/dev/null || true
        print_success "Depo güncellendi."
    else
        git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git "$INSTALL_DIR" 2>/dev/null
        print_success "Depo klonlandı."
    fi
    
    cd "$INSTALL_DIR"
    
    # 3. Install Ollama if local model selected
    if [[ "$SELECTED_LLM_PROVIDER" == "ollama" ]]; then
        echo ""
        print_info "Ollama kontrol ediliyor..."
        
        if ! command -v ollama &> /dev/null; then
            print_info "Ollama kuruluyor..."
            curl -fsSL https://ollama.com/install.sh | sh
            print_success "Ollama kuruldu."
        else
            print_success "Ollama mevcut."
        fi
        
        # Pull model
        print_info "Model indiriliyor: $SELECTED_LLM"
        ollama pull "$SELECTED_LLM" || print_warning "Model indirme başarısız. Manuel olarak deneyin: ollama pull $SELECTED_LLM"
    fi
    
    # 4. Build
    echo ""
    print_info "SENTIENT derleniyor... (Bu işlem birkaç dakika sürebilir)"
    
    if cargo build --release; then
        print_success "SENTIENT derlendi."
    else
        print_error "Derleme başarısız. Hataları kontrol edin."
        exit 1
    fi
    
    # 5. Create .env
    echo ""
    print_info "Yapılandırma dosyası oluşturuluyor..."
    
    local env_content="# ═════════════════════════════════════════════════════════════
#  SENTIENT OS Yapılandırma
# ═════════════════════════════════════════════════════════════

# Model
SENTIENT_MODEL=$SELECTED_LLM
SENTIENT_PROVIDER=$SELECTED_LLM_PROVIDER

# API Keys
"

    for provider in "${!API_KEYS[@]}"; do
        env_content+="${provider^^}=${API_KEYS[$provider]}
"
    done

    env_content+="
# Gateway
GATEWAY_HTTP_ADDR=0.0.0.0:8080
JWT_SECRET=change-this-in-production

# Memory
MEMORY_DB_PATH=data/sentient.db

# Logging
RUST_LOG=info
"

    echo "$env_content" > "$INSTALL_DIR/.env"
    print_success ".env dosyası oluşturuldu."
    
    # 6. Install System Dependencies (optional)
    echo ""
    if confirm "Sistem bağımlılıkları kurulacak mı? (SQLite, OpenSSL, vs.)" "y"; then
        print_info "Bağımlılıklar kuruluyor..."
        
        if [[ -f /etc/debian_version ]]; then
            sudo apt-get update && sudo apt-get install -y build-essential pkg-config libssl-dev sqlite3 libsqlite3-dev
        elif [[ -f /etc/fedora-release ]]; then
            sudo dnf install -y gcc gcc-c++ openssl-devel sqlite sqlite-devel
        elif [[ "$(uname)" == "Darwin" ]]; then
            brew install openssl sqlite
        fi
        
        print_success "Bağımlılıklar kuruldu."
    fi
}

# ─────────────────────────────────────────────────────────────────────────────
# FINAL: SUCCESS SCREEN
# ─────────────────────────────────────────────────────────────────────────────

show_success() {
    print_header "KURULUM TAMAMLANDI" "" ""
    
    echo -e "${GREEN}╔═════════════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║                                                                           ║${NC}"
    echo -e "${GREEN}║              🎉 SENTIENT OS KURULUMU BAŞARIYLA TAMAMLANDI!               ║${NC}"
    echo -e "${GREEN}║                                                                           ║${NC}"
    echo -e "${GREEN}╚═════════════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    
    echo -e "${BOLD}📋 Kurulum Özeti:${NC}"
    echo ""
    echo -e "  ${CYAN}Model:${NC}    ${SELECTED_LLM:-Atlandı}"
    echo -e "  ${CYAN}Provider:${NC} ${SELECTED_LLM_PROVIDER:-Yok}"
    echo -e "  ${CYAN}Kanallar:${NC} ${SELECTED_CHANNELS[*]:-Yok}"
    echo -e "  ${CYAN}Dizin:${NC}    $INSTALL_DIR"
    echo ""
    
    echo -e "${BOLD}🚀 Başlatmak için:${NC}"
    echo ""
    echo -e "  ${WHITE}cd $INSTALL_DIR${NC}"
    echo -e "  ${WHITE}./target/release/sentient-shell${NC}"
    echo ""
    
    echo -e "${BOLD}⚙️  Yapılandırma:${NC}"
    echo ""
    echo -e "  ${WHITE}nano $INSTALL_DIR/.env${NC}"
    echo ""
    
    if [[ "$SELECTED_LLM_PROVIDER" == "ollama" ]]; then
        echo -e "${BOLD}🤖 Model Yönetimi:${NC}"
        echo ""
        echo -e "  ${WHITE}ollama list${NC}          ${DIM}# Yüklü modeller${NC}"
        echo -e "  ${WHITE}ollama run $SELECTED_LLM${NC}  ${DIM}# Modeli çalıştır${NC}"
        echo ""
    fi
    
    echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}🧠 SENTIENT OS - The Operating System That Thinks${NC}"
    echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
}

# ─────────────────────────────────────────────────────────────────────────────
# MAIN ENTRY POINT
# ─────────────────────────────────────────────────────────────────────────────

main() {
    step_welcome
    step_llm_selection
    step_messaging_channels
    step_installation
    show_success
}

# Run
main "$@"
