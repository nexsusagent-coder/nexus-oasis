#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════════════
#  ███████╗███████╗███╗   ██╗████████╗███╗   ██╗███████╗██╗
#  ██╔════╝██╔════╝████╗  ██║╚══██╔══╝████╗  ██║██╔════╝██║
#  ███████╗█████╗  ██╔██╗ ██║   ██║   ██╔██╗ ██║███████╗██║
#  ╚════██║██╔══╝  ██║╚██╗██║   ██║   ██║╚██╗██║╚════██║██║
#  ███████║███████╗██║ ╚████║   ██║   ██║ ╚████║███████║██║
#  ╚══════╝╚══════╝╚═╝  ╚═══╝   ╚═╝   ╚═╝  ╚═══╝╚══════╝╚═╝
#
#  OS - The Operating System That Thinks
#  Linux/macOS Installer v4.0.0
# ═══════════════════════════════════════════════════════════════════════════════
#
#  KULLANIM:
#    curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.sh | bash
#
#  PARAMETRELER:
#    --mode <quick|standard|full|custom>   : Kurulum modu
#    --provider <name>                     : LLM provider
#    --model <name>                        : Model adı
#    --api-key <key>                       : API key
#    --skip-prompts                        : Tüm soruları atla
#    --uninstall                           : Kaldır
# ═══════════════════════════════════════════════════════════════════════════════

set -o errexit
set -o nounset
set -o pipefail

# ═══════════════════════════════════════════════════════════════════════════════
#  ANSI RENKLERİ
# ═══════════════════════════════════════════════════════════════════════════════

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'
BOLD='\033[1m'
DIM='\033[2m'
UNDERLINE='\033[4m'
NC='\033[0m'

# ═══════════════════════════════════════════════════════════════════════════════
#  LOG FONKSİYONLARI
# ═══════════════════════════════════════════════════════════════════════════════

log_step()   { echo -e "${CYAN}━━━${NC} $1"; }
log_info()   { echo -e "  ${BLUE}ℹ${NC}  $1"; }
log_ok()     { echo -e "  ${GREEN}✓${NC}  $1"; }
log_warn()   { echo -e "  ${YELLOW}⚠${NC}  $1"; }
log_error()  { echo -e "  ${RED}✗${NC}  $1"; }
log_menu()   { echo -e "    ${WHITE}[${CYAN}$1${WHITE}]${NC} $2"; }
log_separator() { echo -e "${DIM}  ═════════════════════════════════════════════════════════════${NC}"; }

# ═══════════════════════════════════════════════════════════════════════════════
#  GLOBAL DEĞİŞKENLER
# ═══════════════════════════════════════════════════════════════════════════════

# Yapılandırma
CONFIG_DIR="${HOME}/.sentient"
INSTALL_DIR="${HOME}/.sentient"
CONFIG_FILE="${CONFIG_DIR}/install.conf"

# Varsayılanlar
MODE="standard"
PROVIDER="ollama"
MODEL="gemma3:27b"
INSTALL_OLLAMA=true
INSTALL_DOCKER=false
INSTALL_VOICE=false
INSTALL_DASHBOARD=false
INSTALL_DEV_TOOLS=false
INSTALL_PYTHON=true
INSTALL_RUST=true
DOWNLOAD_MODEL=true
START_SERVICES=true
ADD_TO_PATH=true

# Sistem bilgileri
SYSTEM_RAM=0
SYSTEM_VRAM=0
HAS_NVIDIA=false
CPU_CORES=0
GPU_NAME=""
OS_NAME=""
PKG_MANAGER=""
RIS_CPU=false

# API keys (declare associative array)
declare -A API_KEYS

# Parametreler
SKIP_PROMPTS=false
UNINSTALL=false
MODE_ARG=""
PROVIDER_ARG=""
MODEL_ARG=""
API_KEY_ARG=""

# ═══════════════════════════════════════════════════════════════════════════════
#  BANNER
# ═══════════════════════════════════════════════════════════════════════════════

show_banner() {
    clear
    echo -e "${CYAN}"
    echo "╔═══════════════════════════════════════════════════════════════════════════╗"
    echo "║                                                                           ║"
    echo "║   ${BOLD}${WHITE}███████╗███████╗███╗   ██╗████████╗███╗   ██╗███████╗██╗${NC}${CYAN}               ║"
    echo "║   ${BOLD}${WHITE}██╔════╝██╔════╝████╗  ██║╚══██╔══╝████╗  ██║██╔════╝██║${NC}${CYAN}               ║"
    echo "║   ${BOLD}${WHITE}███████╗█████╗  ██╔██╗ ██║   ██║   ██╔██╗ ██║███████╗██║${NC}${CYAN}               ║"
    echo "║   ${BOLD}${WHITE}╚════██║██╔══╝  ██║╚██╗██║   ██║   ██║╚██╗██║╚════██║██║${NC}${CYAN}               ║"
    echo "║   ${BOLD}${WHITE}███████║███████╗██║ ╚████║   ██║   ██║ ╚████║███████║██║${NC}${CYAN}               ║"
    echo "║   ${BOLD}${WHITE}╚══════╝╚══════╝╚═╝  ╚═══╝   ╚═╝   ╚═╝  ╚═══╝╚══════╝╚═╝${NC}${CYAN}               ║"
    echo "║                                                                           ║"
    echo "║              ${MAGENTA}OS${NC} ${DIM}-${NC} ${YELLOW}The Operating System That Thinks${NC}                          ${CYAN}║"
    echo "║                                                                           ║"
    echo "║   ${DIM}Version 4.0.0  •  AGPL v3 License  •  Made with${NC} ${RED}❤${NC} ${DIM}by Community${NC}            ${CYAN}║"
    echo "║                                                                           ║"
    echo "╚═══════════════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

# ═══════════════════════════════════════════════════════════════════════════════
#  PARAMETRE PARSE
# ═══════════════════════════════════════════════════════════════════════════════

parse_args() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --mode|-m)
                MODE_ARG="$2"
                shift 2
                ;;
            --provider|-p)
                PROVIDER_ARG="$2"
                shift 2
                ;;
            --model|-M)
                MODEL_ARG="$2"
                shift 2
                ;;
            --api-key|-k)
                API_KEY_ARG="$2"
                shift 2
                ;;
            --skip-prompts|-y)
                SKIP_PROMPTS=true
                shift
                ;;
            --uninstall)
                UNINSTALL=true
                shift
                ;;
            --help|-h)
                show_help
                exit 0
                ;;
            *)
                shift
                ;;
        esac
    done
}

show_help() {
    echo ""
    echo "SENTIENT OS - Kurulum Scripti"
    echo ""
    echo "Kullanım:"
    echo "  curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.sh | bash"
    echo ""
    echo "Parametreler:"
    echo "  --mode, -m <quick|standard|full|custom>   Kurulum modu"
    echo "  --provider, -p <name>                     LLM provider"
    echo "  --model, -M <name>                        Model adı"
    echo "  --api-key, -k <key>                       API key"
    echo "  --skip-prompts, -y                        Tüm soruları atla"
    echo "  --uninstall                               Kaldır"
    echo "  --help, -h                                Bu yardım"
    echo ""
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 1: HOŞGELDİNİZ
# ═══════════════════════════════════════════════════════════════════════════════

show_welcome() {
    show_banner
    
    echo -e "${WHITE}  SENTIENT OS'e hoş geldiniz!${NC}"
    echo ""
    echo "  Bu sihirbaz size adım adım rehberlik edecek:"
    echo ""
    echo -e "    ${CYAN}◆${NC} Sistem gereksinimlerinizi analiz edecek"
    echo -e "    ${CYAN}◆${NC} Size en uygun kurulum modunu önerecek"
    echo -e "    ${CYAN}◆${NC} Donanımınıza göre model seçimi yapacak"
    echo -e "    ${CYAN}◆${NC} Gerekli tüm bağımlılıkları kuracak"
    echo -e "    ${CYAN}◆${NC} İlk yapılandırmanızı otomatik oluşturacak"
    echo ""
    log_separator
    echo ""
    
    # Kurulum yolları
    echo -e "${WHITE}  Kurulum Seçenekleri:${NC}"
    echo ""
    echo -e "    ${GREEN}QUICK${NC}    ${DIM}→${NC} Hazır profil, hızlı kurulum (5 dk)"
    echo -e "             ${DIM}CLI + Ollama + Küçük model${NC}"
    echo ""
    echo -e "    ${YELLOW}STANDARD${NC} ${DIM}→${NC} Dengeli kurulum (15 dk)"
    echo -e "             ${DIM}CLI + Tools + Orta boy model${NC}"
    echo ""
    echo -e "    ${MAGENTA}FULL${NC}     ${DIM}→${NC} Tam kurulum (30 dk)"
    echo -e "             ${DIM}Docker + Voice + Dashboard + Büyük model${NC}"
    echo ""
    echo -e "    ${BLUE}CUSTOM${NC}    ${DIM}→${NC} Özelleştirilmiş kurulum"
    echo -e "             ${DIM}Her bileşeni kendiniz seçin${NC}"
    echo ""
    log_separator
    echo ""
    
    if [[ "$SKIP_PROMPTS" != "true" ]]; then
        echo -e "  ${YELLOW}Enter${NC} tuşuna basarak devam edin..."
        read -r
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 2: LİSANS
# ═══════════════════════════════════════════════════════════════════════════════

show_license() {
    echo ""
    echo -e "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "${WHITE}  │${NC} ${BOLD}LİSANS SÖZLEŞMESİ${NC}                                                   ${WHITE}│${NC}"
    echo -e "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${NC}"
    echo ""
    
    echo -e "  ${CYAN}◆${NC} ${WHITE}Lisans Türü:${NC} AGPL v3 (Affero GNU General Public License)"
    echo -e "  ${CYAN}◆${NC} ${WHITE}Kaynak Kod:${NC} https://github.com/nexsusagent-coder/SENTIENT_CORE"
    echo ""
    
    # Lisans kutusu
    echo -e "${DIM}  ┌───────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "${DIM}  │${NC}  ${WHITE}ÖNEMLİ NOKTALAR:${NC}                                                 ${DIM}│${NC}"
    echo -e "${DIM}  │${NC}                                                                   ${DIM}│${NC}"
    echo -e "${DIM}  │${NC}  ${GREEN}✓${NC} Özgür ve açık kaynak yazılım                            ${DIM}│${NC}"
    echo -e "${DIM}  │${NC}  ${GREEN}✓${NC} Kişisel ve ticari kullanım serbest                       ${DIM}│${NC}"
    echo -e "${DIM}  │${NC}  ${GREEN}✓${NC} Değişiklik yapma hakkı                                    ${DIM}│${NC}"
    echo -e "${DIM}  │${NC}  ${GREEN}✓${NC} Dağıtma hakkı                                              ${DIM}│${NC}"
    echo -e "${DIM}  │${NC}                                                                   ${DIM}│${NC}"
    echo -e "${DIM}  │${NC}  ${YELLOW}⚠${NC} Değişiklik yaparsanız kaynak kodunu paylaşmalısınız       ${DIM}│${NC}"
    echo -e "${DIM}  │${NC}  ${YELLOW}⚠${NC} AGPL lisansı network kullanımını kapsar                   ${DIM}│${NC}"
    echo -e "${DIM}  │${NC}                                                                   ${DIM}│${NC}"
    echo -e "${DIM}  │${NC}  ${RED}✗${NC} Kapalı kaynak türev ürünler yasak                         ${DIM}│${NC}"
    echo -e "${DIM}  │${NC}  ${RED}✗${NC} Lisans bildirimlerini kaldırmak yasak                     ${DIM}│${NC}"
    echo -e "${DIM}  │${NC}                                                                   ${DIM}│${NC}"
    echo -e "${DIM}  └───────────────────────────────────────────────────────────────────┘${NC}"
    echo ""
    
    # AI Uyarısı
    echo -e "  ${WHITE}┌─────────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "  ${WHITE}│${NC} ${YELLOW}⚠ AI SİSTEMİ UYARISI${NC}                                              ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}                                                                     ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}  SENTIENT bir AI asistanıdır. Ürettiği içerikler:                  ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}  ${DIM}• Hatalı veya yanıltıcı olabilir${NC}                              ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}  ${DIM}• Güncel olmayabilir${NC}                                          ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}  ${DIM}• Profesyonel tavsiye yerine geçmez${NC}                            ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}                                                                     ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}  ${RED}Kritik kararlar için her zaman doğrulama yapın!${NC}                ${WHITE}│${NC}"
    echo -e "  ${WHITE}└─────────────────────────────────────────────────────────────────────┘${NC}"
    echo ""
    
    if [[ "$SKIP_PROMPTS" == "true" ]]; then
        log_ok "Lisans otomatik kabul edildi"
        return 0
    fi
    
    echo "  Lisans sözleşmesini kabul ediyor musunuz?"
    echo ""
    echo -e "    ${WHITE}[${GREEN}Y${WHITE}]${NC} Evet, kabul ediyorum ve devam et"
    echo -e "    ${WHITE}[${RED}N${WHITE}]${NC} Hayır, kurulumdan çık"
    echo -e "    ${WHITE}[${BLUE}R${WHITE}]${NC} Lisansın tamamını oku"
    echo ""
    
    while true; do
        read -p "  Seçiminiz [Y/N/R]: " choice
        
        case "$choice" in
            [Yy]|"")
                log_ok "Lisans kabul edildi"
                return 0
                ;;
            [Nn])
                log_error "Kurulum iptal edildi"
                exit 0
                ;;
            [Rr])
                echo ""
                echo -e "  ${CYAN}AGPL v3 Lisans Özeti:${NC}"
                echo -e "  ${DIM}────────────────────────────────────────────────────────────${NC}"
                echo "  Bu program özgür yazılımdır; dağıtabilir ve/veya değiştirebilirsiniz."
                echo "  GNU Affero General Public License koşulları altında yayımlanmıştır."
                echo "  Lisansın 3. sürümü veya (isteğe bağlı) daha yeni sürümü geçerlidir."
                echo ""
                echo "  Tam lisans metni için: https://www.gnu.org/licenses/agpl-3.0.html"
                echo -e "  ${DIM}────────────────────────────────────────────────────────────${NC}"
                echo ""
                ;;
        esac
    done
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 3: SİSTEM ANALİZİ
# ═══════════════════════════════════════════════════════════════════════════════

analyze_system() {
    echo ""
    echo -e "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "${WHITE}  │${NC} ${BOLD}SİSTEM ANALİZİ${NC}                                                     ${WHITE}│${NC}"
    echo -e "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${NC}"
    echo ""
    
    # OS tespiti
    if [[ "$(uname)" == "Darwin" ]]; then
        OS_NAME="macOS $(sw_vers -productVersion 2>/dev/null || echo 'Unknown')"
        PKG_MANAGER="brew"
    elif [[ -f /etc/os-release ]]; then
        source /etc/os-release
        OS_NAME="$PRETTY_NAME"
        if command -v apt-get &>/dev/null; then
            PKG_MANAGER="apt"
        elif command -v dnf &>/dev/null; then
            PKG_MANAGER="dnf"
        elif command -v yum &>/dev/null; then
            PKG_MANAGER="yum"
        elif command -v pacman &>/dev/null; then
            PKG_MANAGER="pacman"
        else
            PKG_MANAGER="unknown"
        fi
    else
        OS_NAME="Linux"
        PKG_MANAGER="unknown"
    fi
    
    log_info "İşletim Sistemi: $OS_NAME"
    
    # CPU
    if [[ "$(uname)" == "Darwin" ]]; then
        CPU_CORES=$(sysctl -n hw.ncpu 2>/dev/null || echo "4")
        CPU_NAME=$(sysctl -n machdep.cpu.brand_string 2>/dev/null || echo "Unknown")
    else
        CPU_CORES=$(nproc 2>/dev/null || echo "4")
        CPU_NAME=$(lscpu 2>/dev/null | grep "Model name" | cut -d: -f2 | xargs || echo "Unknown")
    fi
    log_ok "CPU: $CPU_NAME ($CPU_CORES çekirdek)"
    
    # ARM kontrolü
    if [[ "$(uname -m)" == "arm64" ]] || [[ "$CPU_NAME" =~ "Apple" ]]; then
        RIS_CPU=true
        log_warn "ARM mimarisi tespit edildi - bazı özellikler sınırlı olabilir"
    fi
    
    # RAM
    if [[ "$(uname)" == "Darwin" ]]; then
        SYSTEM_RAM=$(sysctl -n hw.memsize 2>/dev/null | awk '{print int($1/1024/1024/1024)}' || echo "8")
    else
        SYSTEM_RAM=$(free -g 2>/dev/null | awk '/^Mem:/{print $2}' || echo "8")
    fi
    
    if [[ $SYSTEM_RAM -ge 32 ]]; then
        log_ok "RAM: ${SYSTEM_RAM}GB ${GREEN}(Mükemmel)${NC}"
    elif [[ $SYSTEM_RAM -ge 16 ]]; then
        log_ok "RAM: ${SYSTEM_RAM}GB ${GREEN}(İyi)${NC}"
    elif [[ $SYSTEM_RAM -ge 8 ]]; then
        log_warn "RAM: ${SYSTEM_RAM}GB ${YELLOW}(Minimum)${NC}"
    else
        log_error "RAM: ${SYSTEM_RAM}GB ${RED}(Yetersiz)${NC}"
    fi
    
    # GPU
    if command -v nvidia-smi &>/dev/null; then
        HAS_NVIDIA=true
        GPU_NAME=$(nvidia-smi --query-gpu=name --format=csv,noheader 2>/dev/null | head -1 || echo "NVIDIA GPU")
        SYSTEM_VRAM=$(nvidia-smi --query-gpu=memory.total --format=csv,noheader,nounits 2>/dev/null | head -1 | awk '{print int($1/1024)}' || echo "8")
        log_ok "GPU: $GPU_NAME ${GREEN}(${SYSTEM_VRAM}GB VRAM)${NC}"
    else
        # Apple Silicon
        if [[ "$(uname)" == "Darwin" ]] && [[ "$RIS_CPU" == "true" ]]; then
            log_ok "GPU: Apple Silicon ${GREEN}(Metal hızlandırma)${NC}"
            # Apple Silicon için VRAM = sistem RAM'i
            SYSTEM_VRAM=$SYSTEM_RAM
        else
            log_warn "GPU: NVIDIA GPU bulunamadı - CPU inference kullanılacak"
        fi
    fi
    
    # Disk
    DISK_AVAIL=$(df -h . 2>/dev/null | awk 'NR==2{print $4}' || echo "Unknown")
    log_info "Disk: $DISK_AVAIL boş"
    
    # Sistem profili
    echo ""
    log_separator
    echo ""
    echo -e "  ${WHITE}Sistem Profiliniz:${NC}"
    echo ""
    
    local profile=""
    local recommendation=""
    
    if [[ $SYSTEM_RAM -ge 64 ]] && [[ $SYSTEM_VRAM -ge 24 ]]; then
        profile="${GREEN}WORKSTATION${NC}"
        recommendation="Büyük modeller (70B+) için uygun"
        MODEL="llama3.3:70b"
    elif [[ $SYSTEM_RAM -ge 32 ]] && [[ $SYSTEM_VRAM -ge 16 ]]; then
        profile="${GREEN}HIGH-END${NC}"
        recommendation="Orta-büyük modeller (27B-70B) için uygun"
        MODEL="gemma3:27b"
    elif [[ $SYSTEM_RAM -ge 16 ]] && [[ $SYSTEM_VRAM -ge 8 ]]; then
        profile="${YELLOW}MID-RANGE${NC}"
        recommendation="Orta boy modeller (8B-27B) için uygun"
        MODEL="gemma3:12b"
    elif [[ $SYSTEM_RAM -ge 8 ]]; then
        profile="${YELLOW}ENTRY-LEVEL${NC}"
        recommendation="Küçük modeller veya API kullanımı önerilir"
        MODEL="qwen3:30b-a3b"
    else
        profile="${RED}MINIMAL${NC}"
        recommendation="API modu önerilir (Cloud LLM)"
        PROVIDER="openrouter"
        INSTALL_OLLAMA=false
        DOWNLOAD_MODEL=false
    fi
    
    echo -e "    ${CYAN}◆${NC} Profil:    $profile"
    echo -e "    ${CYAN}◆${NC} Öneri:     $recommendation"
    echo -e "    ${CYAN}◆${NC} Model:     $MODEL"
    echo ""
    log_separator
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 4: KURULUM MODU
# ═══════════════════════════════════════════════════════════════════════════════

select_mode() {
    echo ""
    echo -e "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "${WHITE}  │${NC} ${BOLD}KURULUM MODU SEÇİMİ${NC}                                                ${WHITE}│${NC}"
    echo -e "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${NC}"
    echo ""
    
    # Parametre ile verildiyse
    if [[ -n "$MODE_ARG" ]]; then
        MODE="$MODE_ARG"
        log_info "Mod parametreden: ${MODE^^}"
        apply_mode_defaults
        return 0
    fi
    
    if [[ "$SKIP_PROMPTS" == "true" ]]; then
        MODE="standard"
        apply_mode_defaults
        return 0
    fi
    
    echo "  Kurulum modunu seçin:"
    echo ""
    
    # Quick
    echo -e "  ${WHITE}┌─────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "  ${WHITE}│${NC} ${GREEN}${BOLD}QUICK${NC} ${DIM}- Hızlı Başlangıç${NC}                                         ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}                                                                 ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}   ${DIM}Süre: ~5 dakika${NC}                                              ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}   ${DIM}• CLI + temel araçlar${NC}                                      ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}   ${DIM}• Ollama + küçük model (qwen3:30b-a3b)${RESET}                       ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}   ${DIM}• Minimum yapılandırma${NC}                                     ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}   ${DIM}• Yeni başlayanlar için ideal${NC}                                ${WHITE}│${NC}"
    echo -e "  ${WHITE}└─────────────────────────────────────────────────────────────────┘${NC}"
    echo ""
    
    # Standard
    echo -e "  ${WHITE}┌─────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "  ${WHITE}│${NC} ${YELLOW}${BOLD}STANDARD${NC} ${DIM}- Önerilen${NC} ${GREEN}★${NC}                                        ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}                                                                 ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}   ${DIM}Süre: ~15 dakika${NC}                                             ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}   ${DIM}• CLI + araçlar + Python entegrasyonu${NC}                        ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}   ${DIM}• Ollama + donanımınıza uygun model${NC}                          ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}   ${DIM}• Tam yapılandırma${NC}                                          ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}   ${DIM}• Çoğu kullanıcı için en iyi seçenek${NC}                         ${WHITE}│${NC}"
    echo -e "  ${WHITE}└─────────────────────────────────────────────────────────────────┘${NC}"
    echo ""
    
    # Full
    echo -e "  ${WHITE}┌─────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "  ${WHITE}│${NC} ${MAGENTA}${BOLD}FULL${NC} ${DIM}- Tam Kurulum${NC}                                            ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}                                                                 ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}   ${DIM}Süre: ~30 dakika${NC}                                             ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}   ${DIM}• Tüm STANDARD özellikler +${NC}                                  ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}   ${DIM}• Docker servisleri (PostgreSQL, Redis, Qdrant...)${NC}            ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}   ${DIM}• Voice (Whisper + Piper)${NC}                                   ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}   ${DIM}• Dashboard (Web UI)${NC}                                        ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}   ${DIM}• Geliştiriciler ve power users için${NC}                         ${WHITE}│${NC}"
    echo -e "  ${WHITE}└─────────────────────────────────────────────────────────────────┘${NC}"
    echo ""
    
    # Custom
    echo -e "  ${WHITE}┌─────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "  ${WHITE}│${NC} ${BLUE}${BOLD}CUSTOM${NC} ${DIM}- Özelleştirilmiş${NC}                                        ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}                                                                 ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}   ${DIM}Süre: Değişken${NC}                                               ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}   ${DIM}• Her bileşeni kendiniz seçin${NC}                                ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}   ${DIM}• Provider ve model seçimi${NC}                                  ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}   ${DIM}• Modül ekleme/çıkarma${NC}                                      ${WHITE}│${NC}"
    echo -e "  ${WHITE}│${NC}   ${DIM}• Deneyimli kullanıcılar için${NC}                                ${WHITE}│${NC}"
    echo -e "  ${WHITE}└─────────────────────────────────────────────────────────────────┘${NC}"
    echo ""
    
    echo -e "  Seçiminiz: ${WHITE}[${GREEN}1${WHITE}]${NC} Quick  ${WHITE}[${GREEN}2${WHITE}]${NC} Standard  ${WHITE}[${GREEN}3${WHITE}]${NC} Full  ${WHITE}[${GREEN}4${WHITE}]${NC} Custom"
    echo ""
    
    while true; do
        read -p "  [1-4]: " choice
        
        case "$choice" in
            1) MODE="quick"; break ;;
            2|"") MODE="standard"; break ;;
            3) MODE="full"; break ;;
            4) MODE="custom"; break ;;
        esac
    done
    
    log_ok "${MODE^^} modu seçildi"
    apply_mode_defaults
}

apply_mode_defaults() {
    case "$MODE" in
        quick)
            INSTALL_OLLAMA=true
            INSTALL_DOCKER=false
            INSTALL_VOICE=false
            INSTALL_DASHBOARD=false
            INSTALL_DEV_TOOLS=false
            MODEL="qwen3:30b-a3b"
            ;;
        standard)
            INSTALL_OLLAMA=true
            INSTALL_DOCKER=false
            INSTALL_VOICE=false
            INSTALL_DASHBOARD=false
            INSTALL_DEV_TOOLS=false
            ;;
        full)
            INSTALL_OLLAMA=true
            INSTALL_DOCKER=true
            INSTALL_VOICE=true
            INSTALL_DASHBOARD=true
            INSTALL_DEV_TOOLS=true
            ;;
        custom)
            # Custom modda seçimler sonraki adımlarda yapılır
            ;;
    esac
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 5: LLM PROVIDER
# ═══════════════════════════════════════════════════════════════════════════════

select_provider() {
    echo ""
    echo -e "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "${WHITE}  │${NC} ${BOLD}LLM PROVIDER SEÇİMİ${NC}                                                ${WHITE}│${NC}"
    echo -e "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${NC}"
    echo ""
    
    # Parametre ile verildiyse
    if [[ -n "$PROVIDER_ARG" ]]; then
        PROVIDER="$PROVIDER_ARG"
        log_info "Provider parametreden: ${PROVIDER^^}"
        if [[ -n "$API_KEY_ARG" ]]; then
            API_KEYS["$PROVIDER"]="$API_KEY_ARG"
        fi
        return 0
    fi
    
    if [[ "$SKIP_PROMPTS" == "true" ]] || [[ "$MODE" != "custom" ]]; then
        log_info "Varsayılan provider: OLLAMA (lokal)"
        return 0
    fi
    
    echo "  AI modelinizi nasıl çalıştırmak istersiniz?"
    echo ""
    
    # Lokal
    echo -e "  ${GREEN}╔═══════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "  ${GREEN}║${NC}              ${WHITE}${BOLD}LOKAL MODELLER${NC} ${DIM}(Ücretsiz)${NC}                           ${GREEN}║${NC}"
    echo -e "  ${GREEN}╠═══════════════════════════════════════════════════════════════════╣${NC}"
    echo -e "  ${GREEN}║${NC}                                                                   ${GREEN}║${NC}"
    echo -e "  ${GREEN}║${NC}  ${WHITE}[1]${NC} Ollama ${GREEN}★${NC}        En popüler, 50K+ model             ${GREEN}║${NC}"
    echo -e "  ${GREEN}║${NC}      ${DIM}Kolay kullanım, otomatik GPU desteği${NC}                        ${GREEN}║${NC}"
    echo -e "  ${GREEN}║${NC}                                                                   ${GREEN}║${NC}"
    echo -e "  ${GREEN}║${NC}  ${WHITE}[2]${NC} LM Studio       GUI ile model yönetimi               ${GREEN}║${NC}"
    echo -e "  ${GREEN}║${NC}      ${DIM}Model indirme, parametre ayarı${NC}                               ${GREEN}║${NC}"
    echo -e "  ${GREEN}║${NC}                                                                   ${GREEN}║${NC}"
    echo -e "  ${GREEN}║${NC}  ${WHITE}[3]${NC} vLLM            Yüksek performans server            ${GREEN}║${NC}"
    echo -e "  ${GREEN}║${NC}      ${DIM}Production-grade, batch inference${NC}                           ${GREEN}║${NC}"
    echo -e "  ${GREEN}║${NC}                                                                   ${GREEN}║${NC}"
    echo -e "  ${GREEN}╚═══════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    
    # Cloud API
    echo -e "  ${YELLOW}╔═══════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "  ${YELLOW}║${NC}              ${WHITE}${BOLD}CLOUD API${NC} ${DIM}(API Key Gerekli)${NC}                          ${YELLOW}║${NC}"
    echo -e "  ${YELLOW}╠═══════════════════════════════════════════════════════════════════╣${NC}"
    echo -e "  ${YELLOW}║${NC}                                                                   ${YELLOW}║${NC}"
    echo -e "  ${YELLOW}║${NC}  ${WHITE}[4]${NC} OpenRouter ${GREEN}★${NC}    200+ model, \$5 ücretsiz kredi      ${YELLOW}║${NC}"
    echo -e "  ${YELLOW}║${NC}      ${DIM}openrouter.ai${NC}                                               ${YELLOW}║${NC}"
    echo -e "  ${YELLOW}║${NC}                                                                   ${YELLOW}║${NC}"
    echo -e "  ${YELLOW}║${NC}  ${WHITE}[5]${NC} OpenAI          GPT-4o, o1, o3, o4-mini               ${YELLOW}║${NC}"
    echo -e "  ${YELLOW}║${NC}      ${DIM}platform.openai.com${NC}                                         ${YELLOW}║${NC}"
    echo -e "  ${YELLOW}║${NC}                                                                   ${YELLOW}║${NC}"
    echo -e "  ${YELLOW}║${NC}  ${WHITE}[6]${NC} Anthropic       Claude 4 Sonnet, Opus 4.1             ${YELLOW}║${NC}"
    echo -e "  ${YELLOW}║${NC}      ${DIM}console.anthropic.com${NC}                                       ${YELLOW}║${NC}"
    echo -e "  ${YELLOW}║${NC}                                                                   ${YELLOW}║${NC}"
    echo -e "  ${YELLOW}║${NC}  ${WHITE}[7]${NC} DeepSeek ${GREEN}★${NC}      ${BOLD}EN UCUZ${NC} - V3, R1 reasoning           ${YELLOW}║${NC}"
    echo -e "  ${YELLOW}║${NC}      ${DIM}platform.deepseek.com${NC}                                        ${YELLOW}║${NC}"
    echo -e "  ${YELLOW}║${NC}                                                                   ${YELLOW}║${NC}"
    echo -e "  ${YELLOW}║${NC}  ${WHITE}[8]${NC} Google AI        Gemini Flash ${GREEN}(FREE tier!)${NC}          ${YELLOW}║${NC}"
    echo -e "  ${YELLOW}║${NC}      ${DIM}aistudio.google.com${NC}                                          ${YELLOW}║${NC}"
    echo -e "  ${YELLOW}║${NC}                                                                   ${YELLOW}║${NC}"
    echo -e "  ${YELLOW}║${NC}  ${WHITE}[9]${NC} Groq ${GREEN}★${NC}          ${BOLD}EN HIZLI${NC} - Llama 3.3 70B           ${YELLOW}║${NC}"
    echo -e "  ${YELLOW}║${NC}      ${DIM}console.groq.com${NC}                                              ${YELLOW}║${NC}"
    echo -e "  ${YELLOW}║${NC}                                                                   ${YELLOW}║${NC}"
    echo -e "  ${YELLOW}╚═══════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    
    # Gateway
    echo -e "  ${MAGENTA}╔═══════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "  ${MAGENTA}║${NC}              ${WHITE}${BOLD}AI GATEWAY / ROUTER${NC}                                       ${MAGENTA}║${NC}"
    echo -e "  ${MAGENTA}╠═══════════════════════════════════════════════════════════════════╣${NC}"
    echo -e "  ${MAGENTA}║${NC}                                                                   ${MAGENTA}║${NC}"
    echo -e "  ${MAGENTA}║${NC}  ${WHITE}[10]${NC} Unify AI        Akıllı routing (kalite+maliyet)    ${MAGENTA}║${NC}"
    echo -e "  ${MAGENTA}║${NC}       ${DIM}unify.ai${NC}                                                    ${MAGENTA}║${NC}"
    echo -e "  ${MAGENTA}║${NC}                                                                   ${MAGENTA}║${NC}"
    echo -e "  ${MAGENTA}║${NC}  ${WHITE}[11]${NC} LiteLLM         Self-hosted proxy server          ${MAGENTA}║${NC}"
    echo -e "  ${MAGENTA}║${NC}       ${DIM}github.com/BerriAI/litellm${NC}                                 ${MAGENTA}║${NC}"
    echo -e "  ${MAGENTA}║${NC}                                                                   ${MAGENTA}║${NC}"
    echo -e "  ${MAGENTA}╚═══════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    
    echo -e "  ${WHITE}[0]${NC} API Key olmadan devam et (daha sonra yapılandır)"
    echo ""
    
    while true; do
        read -p "  Provider seçiniz [1-11]: " choice
        
        case "$choice" in
            1) PROVIDER="ollama"; break ;;
            2) PROVIDER="lmstudio"; break ;;
            3) PROVIDER="vllm"; break ;;
            4) PROVIDER="openrouter"; break ;;
            5) PROVIDER="openai"; break ;;
            6) PROVIDER="anthropic"; break ;;
            7) PROVIDER="deepseek"; break ;;
            8) PROVIDER="google"; break ;;
            9) PROVIDER="groq"; break ;;
            10) PROVIDER="unify"; break ;;
            11) PROVIDER="litellm"; break ;;
            0) PROVIDER="none"; break ;;
        esac
    done
    
    # API Key gerektiren provider'lar
    local needs_api_key=("openrouter" "openai" "anthropic" "deepseek" "google" "groq" "unify")
    
    if [[ " ${needs_api_key[@]} " =~ " ${PROVIDER} " ]]; then
        echo ""
        
        local key_name=""
        local url=""
        
        case "$PROVIDER" in
            openrouter) key_name="OPENROUTER_API_KEY"; url="https://openrouter.ai/keys" ;;
            openai) key_name="OPENAI_API_KEY"; url="https://platform.openai.com/api-keys" ;;
            anthropic) key_name="ANTHROPIC_API_KEY"; url="https://console.anthropic.com/settings/keys" ;;
            deepseek) key_name="DEEPSEEK_API_KEY"; url="https://platform.deepseek.com/api_keys" ;;
            google) key_name="GOOGLE_AI_API_KEY"; url="https://aistudio.google.com/apikey" ;;
            groq) key_name="GROQ_API_KEY"; url="https://console.groq.com/keys" ;;
            unify) key_name="UNIFY_API_KEY"; url="https://unify.ai/keys" ;;
        esac
        
        echo -e "  ${YELLOW}API Key Gerekli!${NC}"
        echo -e "  Almak için: ${CYAN}$url${NC}"
        echo ""
        
        if [[ -n "$API_KEY_ARG" ]]; then
            API_KEYS["$PROVIDER"]="$API_KEY_ARG"
            log_ok "API Key parametreden alındı"
        else
            read -p "  $key_name: " key
            if [[ -n "$key" ]]; then
                API_KEYS["$PROVIDER"]="$key"
            else
                log_warn "API Key girilmedi - .env dosyasından ekleyebilirsiniz"
            fi
        fi
        
        # Cloud provider ise Ollama kurulumuna gerek yok
        INSTALL_OLLAMA=false
        DOWNLOAD_MODEL=false
    fi
    
    log_ok "Provider: ${PROVIDER^^}"
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 6: MODEL SEÇİMİ
# ═══════════════════════════════════════════════════════════════════════════════

select_model() {
    # Cloud provider ise model seçimi atlanır
    if [[ "$PROVIDER" != "ollama" ]] && [[ "$PROVIDER" != "lmstudio" ]] && [[ "$PROVIDER" != "vllm" ]]; then
        log_info "Cloud provider seçildi - model .env'de yapılandırılacak"
        return 0
    fi
    
    # Parametre ile verildiyse
    if [[ -n "$MODEL_ARG" ]]; then
        MODEL="$MODEL_ARG"
        log_info "Model parametreden: $MODEL"
        return 0
    fi
    
    if [[ "$SKIP_PROMPTS" == "true" ]] || [[ "$MODE" != "custom" ]]; then
        log_info "Önerilen model: $MODEL"
        return 0
    fi
    
    echo ""
    echo -e "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "${WHITE}  │${NC} ${BOLD}MODEL SEÇİMİ${NC} ${DIM}(VRAM: ${SYSTEM_VRAM} GB)${NC}                              ${WHITE}│${NC}"
    echo -e "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${NC}"
    echo ""
    
    local vram=$SYSTEM_VRAM
    
    # 24GB+ VRAM
    if [[ $vram -ge 24 ]]; then
        echo -e "  ${GREEN}╔═══════════════════════════════════════════════════════════════════╗${NC}"
        echo -e "  ${GREEN}║${NC}              ${WHITE}${BOLD}24GB+ VRAM - BÜYÜK MODELLER${NC}                            ${GREEN}║${NC}"
        echo -e "  ${GREEN}╠═══════════════════════════════════════════════════════════════════╣${NC}"
        echo -e "  ${GREEN}║${NC}                                                                   ${GREEN}║${NC}"
        echo -e "  ${GREEN}║${NC}  ${WHITE}[1]${NC} llama3.3:70b       70B parametre, güçlü reasoning    ${GREEN}║${NC}"
        echo -e "  ${GREEN}║${NC}  ${WHITE}[2]${NC} deepseek-r1:67b    67B, mükemmel matematik/kod       ${GREEN}║${NC}"
        echo -e "  ${GREEN}║${NC}  ${WHITE}[3]${NC} llama4:scout       109B MoE, 10M context            ${GREEN}║${NC}"
        echo -e "  ${GREEN}║${NC}  ${WHITE}[4]${NC} gemma3:27b ${GREEN}★${NC}       27B, dengeli performans         ${GREEN}║${NC}"
        echo -e "  ${GREEN}║${NC}                                                                   ${GREEN}║${NC}"
        echo -e "  ${GREEN}╚═══════════════════════════════════════════════════════════════════╝${NC}"
    # 16GB VRAM
    elif [[ $vram -ge 16 ]]; then
        echo -e "  ${YELLOW}╔═══════════════════════════════════════════════════════════════════╗${NC}"
        echo -e "  ${YELLOW}║${NC}              ${WHITE}${BOLD}16GB VRAM - ORTA-BÜYÜK MODELLER${NC}                        ${YELLOW}║${NC}"
        echo -e "  ${YELLOW}╠═══════════════════════════════════════════════════════════════════╣${NC}"
        echo -e "  ${YELLOW}║${NC}                                                                   ${YELLOW}║${NC}"
        echo -e "  ${YELLOW}║${NC}  ${WHITE}[1]${NC} gemma3:27b ${GREEN}★${NC}       27B, dengeli performans         ${YELLOW}║${NC}"
        echo -e "  ${YELLOW}║${NC}  ${WHITE}[2]${NC} gemma3:12b         12B, hızlı inference             ${YELLOW}║${NC}"
        echo -e "  ${YELLOW}║${NC}  ${WHITE}[3]${NC} mistral-small3.1   24B, Avrupa yapımı               ${YELLOW}║${NC}"
        echo -e "  ${YELLOW}║${NC}  ${WHITE}[4]${NC} pixtral:12b        12B, multimodal (görüntü)        ${YELLOW}║${NC}"
        echo -e "  ${YELLOW}║${NC}                                                                   ${YELLOW}║${NC}"
        echo -e "  ${YELLOW}╚═══════════════════════════════════════════════════════════════════╝${NC}"
    # 8GB VRAM
    elif [[ $vram -ge 8 ]]; then
        echo -e "  ${YELLOW}╔═══════════════════════════════════════════════════════════════════╗${NC}"
        echo -e "  ${YELLOW}║${NC}              ${WHITE}${BOLD}8GB VRAM - ORTA MODELLER${NC}                               ${YELLOW}║${NC}"
        echo -e "  ${YELLOW}╠═══════════════════════════════════════════════════════════════════╣${NC}"
        echo -e "  ${YELLOW}║${NC}                                                                   ${YELLOW}║${NC}"
        echo -e "  ${YELLOW}║${NC}  ${WHITE}[1]${NC} deepseek-r1:8b     8B, iyi reasoning               ${YELLOW}║${NC}"
        echo -e "  ${YELLOW}║${NC}  ${WHITE}[2]${NC} mistral-small3.1   24B quantized                   ${YELLOW}║${NC}"
        echo -e "  ${YELLOW}║${NC}  ${WHITE}[3]${NC} qwen2.5-coder:7b   7B, coding optimize             ${YELLOW}║${NC}"
        echo -e "  ${YELLOW}║${NC}  ${WHITE}[4]${NC} gemma3:12b ${GREEN}★${NC}       12B, dengeli                    ${YELLOW}║${NC}"
        echo -e "  ${YELLOW}║${NC}                                                                   ${YELLOW}║${NC}"
        echo -e "  ${YELLOW}╚═══════════════════════════════════════════════════════════════════╝${NC}"
    # Düşük VRAM
    else
        echo -e "  ${RED}╔═══════════════════════════════════════════════════════════════════╗${NC}"
        echo -e "  ${RED}║${NC}              ${WHITE}${BOLD}DÜŞÜK VRAM - KÜÇÜK MODELLER${NC}                               ${RED}║${NC}"
        echo -e "  ${RED}╠═══════════════════════════════════════════════════════════════════╣${NC}"
        echo -e "  ${RED}║${NC}                                                                   ${RED}║${NC}"
        echo -e "  ${RED}║${NC}  ${WHITE}[1]${NC} qwen3:30b-a3b ${GREEN}★${NC}   30B MoE (3B aktif) - ÖNERİLEN  ${RED}║${NC}"
        echo -e "  ${RED}║${NC}  ${WHITE}[2]${NC} phi4-mini          3.8B, Microsoft                 ${RED}║${NC}"
        echo -e "  ${RED}║${NC}  ${WHITE}[3]${NC} llama3.2:3b       3B, Meta                        ${RED}║${NC}"
        echo -e "  ${RED}║${NC}  ${WHITE}[4]${NC} llama3.2:1b       1.2B, çok hızlı                 ${RED}║${NC}"
        echo -e "  ${RED}║${NC}                                                                   ${RED}║${NC}"
        echo -e "  ${RED}║${NC}  ${YELLOW}⚠ Düşük VRAM: Cloud API kullanımı önerilir${NC}                  ${RED}║${NC}"
        echo -e "  ${RED}║${NC}                                                                   ${RED}║${NC}"
        echo -e "  ${RED}║${NC}  ${WHITE}[5]${NC} Cloud API kullan                                  ${RED}║${NC}"
        echo -e "  ${RED}║${NC}                                                                   ${RED}║${NC}"
        echo -e "  ${RED}╚═══════════════════════════════════════════════════════════════════╝${NC}"
    fi
    
    echo ""
    echo -e "  ${WHITE}[0]${NC} Model indirmeden devam et"
    echo ""
    
    read -p "  Seçiminiz: " choice
    
    # Seçime göre model ata
    case "$choice" in
        0)
            DOWNLOAD_MODEL=false
            log_info "Model indirme atlanıyor"
            ;;
        5)
            # Cloud'a dön
            select_provider
            select_model
            ;;
        *)
            # VRAM'a göre seçenekleri map'le
            if [[ $vram -ge 24 ]]; then
                models=("llama3.3:70b" "deepseek-r1:67b" "llama4:scout" "gemma3:27b")
            elif [[ $vram -ge 16 ]]; then
                models=("gemma3:27b" "gemma3:12b" "mistral-small3.1" "pixtral:12b")
            elif [[ $vram -ge 8 ]]; then
                models=("deepseek-r1:8b" "mistral-small3.1" "qwen2.5-coder:7b" "gemma3:12b")
            else
                models=("qwen3:30b-a3b" "phi4-mini" "llama3.2:3b" "llama3.2:1b")
            fi
            
            local idx=$((choice - 1))
            if [[ $idx -ge 0 ]] && [[ $idx -lt ${#models[@]} ]]; then
                MODEL="${models[$idx]}"
            fi
            ;;
    esac
    
    log_ok "Model: $MODEL"
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 7: BİLEŞEN SEÇİMİ (CUSTOM)
# ═══════════════════════════════════════════════════════════════════════════════

select_components() {
    if [[ "$MODE" != "custom" ]]; then
        return 0
    fi
    
    echo ""
    echo -e "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "${WHITE}  │${NC} ${BOLD}BİLEŞEN SEÇİMİ${NC}                                                    ${WHITE}│${NC}"
    echo -e "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${NC}"
    echo ""
    
    # Ollama
    echo -e "  ${WHITE}╔═══════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "  ${WHITE}║${NC}  ${CYAN}🤖 OLLAMA${NC} ${DIM}(Lokal LLM Runtime)${NC}                                ${WHITE}║${NC}"
    echo -e "  ${WHITE}║${NC}                                                                   ${WHITE}║${NC}"
    echo -e "  ${WHITE}║${NC}  ${DIM}Lokal AI modelleri çalıştırmak için gereklidir.${NC}                 ${WHITE}║${NC}"
    echo -e "  ${WHITE}║${NC}  ${DIM}Cloud API kullanacaksanız kurmanıza gerek yok.${NC}                 ${WHITE}║${NC}"
    echo -e "  ${WHITE}║${NC}                                                                   ${WHITE}║${NC}"
    echo -e "  ${WHITE}╚═══════════════════════════════════════════════════════════════════╝${NC}"
    
    read -p "  Kurulsun mu? [Y/n]: " ollama_choice
    [[ "$ollama_choice" != "n" ]] && [[ "$ollama_choice" != "N" ]] && INSTALL_OLLAMA=true || INSTALL_OLLAMA=false
    
    echo ""
    
    # Docker
    echo -e "  ${WHITE}╔═══════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "  ${WHITE}║${NC}  ${CYAN}🐳 DOCKER SERVİSLERİ${NC}                                              ${WHITE}║${NC}"
    echo -e "  ${WHITE}║${NC}                                                                   ${WHITE}║${NC}"
    echo -e "  ${WHITE}║${NC}  ${DIM}PostgreSQL, Redis, Qdrant, MinIO, Prometheus, Grafana${NC}        ${WHITE}║${NC}"
    echo -e "  ${WHITE}║${NC}  ${DIM}Production ortamı için önerilir.${NC}                               ${WHITE}║${NC}"
    echo -e "  ${WHITE}║${NC}                                                                   ${WHITE}║${NC}"
    echo -e "  ${WHITE}╚═══════════════════════════════════════════════════════════════════╝${NC}"
    
    read -p "  Kurulsun mu? [y/N]: " docker_choice
    [[ "$docker_choice" == "y" ]] || [[ "$docker_choice" == "Y" ]] && INSTALL_DOCKER=true || INSTALL_DOCKER=false
    
    echo ""
    
    # Voice
    echo -e "  ${WHITE}╔═══════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "  ${WHITE}║${NC}  ${CYAN}🎤 VOICE${NC} ${DIM}(Sesli Asistan)${NC}                                        ${WHITE}║${NC}"
    echo -e "  ${WHITE}║${NC}                                                                   ${WHITE}║${NC}"
    echo -e "  ${WHITE}║${NC}  ${DIM}Whisper.cpp ile Speech-to-Text${NC}                                 ${WHITE}║${NC}"
    echo -e "  ${WHITE}║${NC}  ${DIM}Piper ile Text-to-Speech${NC}                                      ${WHITE}║${NC}"
    echo -e "  ${WHITE}║${NC}  ${DIM}Wake word desteği${NC}                                             ${WHITE}║${NC}"
    echo -e "  ${WHITE}║${NC}                                                                   ${WHITE}║${NC}"
    echo -e "  ${WHITE}╚═══════════════════════════════════════════════════════════════════╝${NC}"
    
    read -p "  Kurulsun mu? [y/N]: " voice_choice
    [[ "$voice_choice" == "y" ]] || [[ "$voice_choice" == "Y" ]] && INSTALL_VOICE=true || INSTALL_VOICE=false
    
    echo ""
    
    # Dashboard
    echo -e "  ${WHITE}╔═══════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "  ${WHITE}║${NC}  ${CYAN}📊 DASHBOARD${NC} ${DIM}(Web Arayüzü)${NC}                                      ${WHITE}║${NC}"
    echo -e "  ${WHITE}║${NC}                                                                   ${WHITE}║${NC}"
    echo -e "  ${WHITE}║${NC}  ${DIM}Tauri tabanlı masaüstü uygulaması${NC}                               ${WHITE}║${NC}"
    echo -e "  ${WHITE}║${NC}  ${DIM}WebSocket ile gerçek zamanlı iletişim${NC}                           ${WHITE}║${NC}"
    echo -e "  ${WHITE}║${NC}  ${DIM}Skill ve tool yönetimi${NC}                                        ${WHITE}║${NC}"
    echo -e "  ${WHITE}║${NC}                                                                   ${WHITE}║${NC}"
    echo -e "  ${WHITE}╚═══════════════════════════════════════════════════════════════════╝${NC}"
    
    read -p "  Kurulsun mu? [y/N]: " dashboard_choice
    [[ "$dashboard_choice" == "y" ]] || [[ "$dashboard_choice" == "Y" ]] && INSTALL_DASHBOARD=true || INSTALL_DASHBOARD=false
    
    echo ""
    log_separator
    echo ""
    echo -e "  ${WHITE}Seçilen Bileşenler:${NC}"
    [[ "$INSTALL_OLLAMA" == "true" ]] && log_ok "Ollama (Lokal LLM)"
    [[ "$INSTALL_DOCKER" == "true" ]] && log_ok "Docker Servisleri"
    [[ "$INSTALL_VOICE" == "true" ]] && log_ok "Voice (Sesli Asistan)"
    [[ "$INSTALL_DASHBOARD" == "true" ]] && log_ok "Dashboard (Web UI)"
    [[ "$PROVIDER" != "ollama" ]] && log_ok "${PROVIDER^^} API"
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 8: ÖN KOŞULLAR KURULUMU
# ═══════════════════════════════════════════════════════════════════════════════

install_prerequisites() {
    echo ""
    echo -e "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "${WHITE}  │${NC} ${BOLD}ÖN KOŞULLAR KURULUYOR...${NC}                                         ${WHITE}│${NC}"
    echo -e "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${NC}"
    echo ""
    
    local sudo_cmd=""
    if [[ "$(id -u)" -ne 0 ]] && command -v sudo &>/dev/null; then
        sudo_cmd="sudo"
    fi
    
    # Git
    log_step "Git kontrol ediliyor..."
    if command -v git &>/dev/null; then
        log_ok "Git: $(git --version)"
    else
        log_info "Git kuruluyor..."
        case "$PKG_MANAGER" in
            apt) $sudo_cmd apt-get install -y git ;;
            dnf|yum) $sudo_cmd dnf install -y git ;;
            pacman) $sudo_cmd pacman -S --noconfirm git ;;
            brew) brew install git ;;
        esac
        log_ok "Git kuruldu"
    fi
    
    # Rust
    log_step "Rust kontrol ediliyor..."
    if command -v rustc &>/dev/null; then
        log_ok "Rust: $(rustc --version)"
    else
        log_info "Rust kuruluyor..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env" 2>/dev/null || true
        log_ok "Rust kuruldu"
    fi
    
    # Python
    if [[ "$INSTALL_PYTHON" == "true" ]]; then
        log_step "Python kontrol ediliyor..."
        if command -v python3 &>/dev/null; then
            log_ok "Python: $(python3 --version)"
        else
            log_info "Python kuruluyor..."
            case "$PKG_MANAGER" in
                apt) $sudo_cmd apt-get install -y python3 python3-dev python3-pip python3-venv ;;
                dnf|yum) $sudo_cmd dnf install -y python3 python3-devel python3-pip ;;
                pacman) $sudo_cmd pacman -S --noconfirm python python-pip ;;
                brew) brew install python ;;
            esac
            log_ok "Python kuruldu"
        fi
    fi
    
    # Build tools
    log_step "Build araçları kontrol ediliyor..."
    case "$PKG_MANAGER" in
        apt)
            if ! command -v make &>/dev/null; then
                log_info "Build essentials kuruluyor..."
                $sudo_cmd apt-get update
                $sudo_cmd apt-get install -y build-essential pkg-config libssl-dev sqlite3 libsqlite3-dev cmake
            fi
            ;;
        dnf|yum)
            $sudo_cmd dnf groupinstall -y "Development Tools" 2>/dev/null || true
            $sudo_cmd dnf install -y openssl-devel sqlite-devel cmake 2>/dev/null || true
            ;;
        pacman)
            $sudo_cmd pacman -S --noconfirm base-devel openssl sqlite cmake 2>/dev/null || true
            ;;
        brew)
            brew install openssl sqlite cmake 2>/dev/null || true
            ;;
    esac
    log_ok "Build araçları hazır"
    
    # FFmpeg
    log_step "FFmpeg kontrol ediliyor..."
    if command -v ffmpeg &>/dev/null; then
        log_ok "FFmpeg: mevcut"
    else
        log_info "FFmpeg kuruluyor..."
        case "$PKG_MANAGER" in
            apt) $sudo_cmd apt-get install -y ffmpeg ;;
            dnf|yum) $sudo_cmd dnf install -y ffmpeg ;;
            pacman) $sudo_cmd pacman -S --noconfirm ffmpeg ;;
            brew) brew install ffmpeg ;;
        esac
        log_ok "FFmpeg kuruldu"
    fi
    
    # Ollama
    if [[ "$INSTALL_OLLAMA" == "true" ]]; then
        log_step "Ollama kontrol ediliyor..."
        if command -v ollama &>/dev/null; then
            log_ok "Ollama: mevcut"
            
            # Servis kontrol
            if ! pgrep -x "ollama" &>/dev/null; then
                log_info "Ollama servisi başlatılıyor..."
                ollama serve &>/dev/null &
                sleep 3
            fi
        else
            log_info "Ollama kuruluyor..."
            curl -fsSL https://ollama.com/install.sh | sh
            ollama serve &>/dev/null &
            sleep 5
            log_ok "Ollama kuruldu"
        fi
    fi
    
    # Docker
    if [[ "$INSTALL_DOCKER" == "true" ]]; then
        log_step "Docker kontrol ediliyor..."
        if command -v docker &>/dev/null; then
            log_ok "Docker: mevcut"
        else
            log_info "Docker kuruluyor..."
            curl -fsSL https://get.docker.com | sh
            log_warn "Docker kurulumu tamamlandı - kullanıcınızı docker grubuna eklemeyi unutmayın"
        fi
    fi
    
    echo ""
    log_ok "Tüm ön koşullar hazır!"
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 9: KAYNAK İNDİRME
# ═══════════════════════════════════════════════════════════════════════════════

download_source() {
    echo ""
    echo -e "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "${WHITE}  │${NC} ${BOLD}KAYNAK İNDİRİLİYOR...${NC}                                              ${WHITE}│${NC}"
    echo -e "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${NC}"
    echo ""
    
    if [[ -f "$INSTALL_DIR/Cargo.toml" ]]; then
        log_info "Mevcut kurulum bulundu, güncelleniyor..."
        cd "$INSTALL_DIR"
        git pull 2>/dev/null || true
        log_ok "Repository güncellendi"
    else
        log_info "SENTIENT repository'si klonlanıyor..."
        git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git "$INSTALL_DIR"
        cd "$INSTALL_DIR"
        log_ok "Repository klonlandı"
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 10: DERLEME
# ═══════════════════════════════════════════════════════════════════════════════

build_project() {
    echo ""
    echo -e "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "${WHITE}  │${NC} ${BOLD}SENTIENT DERLENİYOR...${NC}                                             ${WHITE}│${NC}"
    echo -e "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${NC}"
    echo ""
    
    log_info "Bu işlem 5-15 dakika sürebilir..."
    log_info "İlk derleme uzun sürer, lütfen bekleyin..."
    echo ""
    
    # Python path
    export PYTHON_SYS_EXECUTABLE=$(which python3 2>/dev/null || echo "")
    
    # Build
    if cargo build --release 2>&1 | tee /tmp/sentient-build.log; then
        if [[ -f "target/release/sentient" ]]; then
            local size=$(du -h target/release/sentient | cut -f1)
            echo ""
            log_ok "SENTIENT derlendi! ($size)"
            return 0
        fi
    fi
    
    log_error "Derleme başarısız!"
    log_info "Log: /tmp/sentient-build.log"
    return 1
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 11: YAPILANDIRMA
# ═══════════════════════════════════════════════════════════════════════════════

configure_environment() {
    echo ""
    echo -e "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "${WHITE}  │${NC} ${BOLD}YAPILANDIRMA OLUŞTURULUYOR...${NC}                                     ${WHITE}│${NC}"
    echo -e "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${NC}"
    echo ""
    
    # .env dosyası
    if [[ ! -f ".env" ]]; then
        local env_content="# ════════════════════════════════════════════════════════════════
#  SENTIENT OS - Yapılandırma Dosyası
#  Oluşturulma: $(date '+%Y-%m-%d %H:%M:%S')
# ════════════════════════════════════════════════════════════════

# LLM PROVIDER: $PROVIDER
"
        
        case "$PROVIDER" in
            ollama)
                env_content+="\n# Ollama (Lokal - Ücretsiz)
OLLAMA_HOST=http://localhost:11434
OPENAI_API_BASE=http://localhost:11434/v1
OPENAI_API_KEY=ollama
DEFAULT_MODEL=ollama/$MODEL
"
                ;;
            openrouter)
                env_content+="\n# OpenRouter (200+ Model)
OPENROUTER_API_KEY=${API_KEYS[openrouter]}
DEFAULT_MODEL=openrouter/auto
"
                ;;
            openai)
                env_content+="\n# OpenAI
OPENAI_API_KEY=${API_KEYS[openai]}
DEFAULT_MODEL=openai/gpt-4o
"
                ;;
            anthropic)
                env_content+="\n# Anthropic
ANTHROPIC_API_KEY=${API_KEYS[anthropic]}
DEFAULT_MODEL=anthropic/claude-4-sonnet
"
                ;;
            deepseek)
                env_content+="\n# DeepSeek (EN UCUZ)
DEEPSEEK_API_KEY=${API_KEYS[deepseek]}
DEFAULT_MODEL=deepseek/deepseek-chat
"
                ;;
            groq)
                env_content+="\n# Groq (EN HIZLI)
GROQ_API_KEY=${API_KEYS[groq]}
DEFAULT_MODEL=groq/llama-3.3-70b-versatile
"
                ;;
            google)
                env_content+="\n# Google AI (Gemini)
GOOGLE_AI_API_KEY=${API_KEYS[google]}
DEFAULT_MODEL=google/gemini-2.0-flash
"
                ;;
            *)
                env_content+="\n# Provider yapılandırması gerekli
# Diğer provider'lar için .env.template dosyasına bakın
"
                ;;
        esac
        
        env_content+="\n\n# ════════════════════════════════════════════════════════════════
#  OPSİYONEL YAPILANDIRMA
# ════════════════════════════════════════════════════════════════

# Voice
VOICE_ENABLED=$INSTALL_VOICE

# Dashboard
DASHBOARD_ENABLED=$INSTALL_DASHBOARD

# Logging
RUST_LOG=info
"
        
        echo -e "$env_content" > .env
        log_ok ".env dosyası oluşturuldu"
    else
        log_ok ".env dosyası zaten mevcut"
    fi
    
    # Model indir
    if [[ "$DOWNLOAD_MODEL" == "true" ]] && [[ "$PROVIDER" == "ollama" ]]; then
        echo ""
        log_step "$MODEL modeli indiriliyor..."
        ollama pull "$MODEL" || log_warn "Model indirme başarısız - manuel: ollama pull $MODEL"
        log_ok "Model hazır: $MODEL"
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 12: DOĞRULAMA
# ═══════════════════════════════════════════════════════════════════════════════

validate_installation() {
    echo ""
    echo -e "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "${WHITE}  │${NC} ${BOLD}KURULUM DOĞRULANIYOR...${NC}                                          ${WHITE}│${NC}"
    echo -e "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${NC}"
    echo ""
    
    local all_ok=true
    
    # Binary
    if [[ -f "target/release/sentient" ]]; then
        log_ok "sentient binary"
    else
        log_error "sentient binary bulunamadı"
        all_ok=false
    fi
    
    # .env
    if [[ -f ".env" ]]; then
        log_ok ".env yapılandırması"
    else
        log_error ".env bulunamadı"
        all_ok=false
    fi
    
    # Ollama
    if [[ "$INSTALL_OLLAMA" == "true" ]]; then
        if command -v ollama &>/dev/null; then
            log_ok "Ollama"
        else
            log_warn "Ollama (PATH'e eklenmeli)"
        fi
    fi
    
    [[ "$all_ok" == "true" ]]
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 13: TAMAMLAMA
# ═══════════════════════════════════════════════════════════════════════════════

show_complete() {
    show_banner
    
    echo -e "${GREEN}  ╔═══════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}  ║${NC}                                                                   ${GREEN}║${NC}"
    echo -e "${GREEN}  ║${NC}          ${WHITE}${BOLD}🎉 KURULUM BAŞARIYLA TAMAMLANDI! 🎉${NC}                       ${GREEN}║${NC}"
    echo -e "${GREEN}  ║${NC}                                                                   ${GREEN}║${NC}"
    echo -e "${GREEN}  ╚═══════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    
    log_separator
    echo ""
    echo -e "  ${WHITE}KURULUM ÖZETİ${NC}"
    echo ""
    echo -e "    ${CYAN}Mod:${NC}         ${MODE^^}"
    echo -e "    ${CYAN}Provider:${NC}    ${PROVIDER^^}"
    echo -e "    ${CYAN}Model:${NC}       $MODEL"
    echo -e "    ${CYAN}Dizin:${NC}       $INSTALL_DIR"
    echo ""
    
    echo -e "    ${CYAN}Ollama:${NC}      $([ "$INSTALL_OLLAMA" == "true" ] && echo "✓" || echo "✗")"
    echo -e "    ${CYAN}Docker:${NC}      $([ "$INSTALL_DOCKER" == "true" ] && echo "✓" || echo "✗")"
    echo -e "    ${CYAN}Voice:${NC}       $([ "$INSTALL_VOICE" == "true" ] && echo "✓" || echo "✗")"
    echo -e "    ${CYAN}Dashboard:${NC}   $([ "$INSTALL_DASHBOARD" == "true" ] && echo "✓" || echo "✗")"
    echo ""
    log_separator
    echo ""
    
    echo -e "  ${WHITE}KULLANIM${NC}"
    echo ""
    echo -e "    ${DIM}# Versiyon kontrolü${NC}"
    echo -e "    ${GREEN}./target/release/sentient --version${NC}"
    echo ""
    echo -e "    ${DIM}# Sohbet başlat${NC}"
    echo -e "    ${GREEN}./target/release/sentient chat${NC}"
    echo ""
    echo -e "    ${DIM}# Web dashboard${NC}"
    echo -e "    ${GREEN}./target/release/sentient web${NC}"
    echo ""
    
    if [[ "$PROVIDER" == "ollama" ]]; then
        echo -e "    ${DIM}# Model yönetimi${NC}"
        echo -e "    ${GREEN}ollama list${NC}              ${DIM}# Yüklü modeller${NC}"
        echo -e "    ${GREEN}ollama pull <model>${NC}      ${DIM}# Model indir${NC}"
        echo ""
    fi
    
    log_separator
    echo ""
    echo -e "  ${WHITE}SONRAKİ ADIMLAR${NC}"
    echo ""
    echo -e "    1. API key ekleyin:    ${CYAN}nano .env${NC}"
    echo -e "    2. Farklı model indir: ${CYAN}ollama pull deepseek-r1:8b${NC}"
    echo -e "    3. Dokümantasyon:      ${CYAN}cat README.md${NC}"
    echo ""
    
    # PATH'e ekle
    if [[ "$ADD_TO_PATH" == "true" ]]; then
        local shell_rc=""
        case "$(basename "$SHELL")" in
            zsh) shell_rc="$HOME/.zshrc" ;;
            bash) shell_rc="$HOME/.bashrc" ;;
            *) shell_rc="$HOME/.profile" ;;
        esac
        
        if ! grep -q "sentient" "$shell_rc" 2>/dev/null; then
            echo "" >> "$shell_rc"
            echo "# SENTIENT OS" >> "$shell_rc"
            echo "export PATH=\"\$PATH:$INSTALL_DIR/target/release\"" >> "$shell_rc"
            echo "alias sentient='$INSTALL_DIR/target/release/sentient'" >> "$shell_rc"
            log_ok "SENTIENT PATH'e eklendi ($shell_rc)"
        fi
    fi
    
    echo -e "${MAGENTA}  ╔═══════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${MAGENTA}  ║${NC}                                                                   ${MAGENTA}║${NC}"
    echo -e "${MAGENTA}  ║${NC}        ${WHITE}SENTIENT OS${NC} ${DIM}-${NC} ${YELLOW}The Operating System That Thinks${NC}              ${MAGENTA}║${NC}"
    echo -e "${MAGENTA}  ║${NC}                                                                   ${MAGENTA}║${NC}"
    echo -e "${MAGENTA}  ╚═══════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

# ═══════════════════════════════════════════════════════════════════════════════
#  KALDIRMA
# ═══════════════════════════════════════════════════════════════════════════════

uninstall_sentient() {
    show_banner
    
    echo -e "  ${RED}KALDIRMA İŞLEMİ${NC}"
    echo ""
    
    read -p "  SENTIENT OS'i kaldırmak istediğinizden emin misiniz? [y/N]: " confirm
    if [[ "$confirm" != "y" ]] && [[ "$confirm" != "Y" ]]; then
        log_info "İptal edildi"
        return
    fi
    
    # Dizini sil
    if [[ -d "$INSTALL_DIR" ]]; then
        log_info "$INSTALL_DIR siliniyor..."
        rm -rf "$INSTALL_DIR"
        log_ok "Dizin silindi"
    fi
    
    # PATH'ten kaldır
    local shell_rc=""
    case "$(basename "$SHELL")" in
        zsh) shell_rc="$HOME/.zshrc" ;;
        bash) shell_rc="$HOME/.bashrc" ;;
        *) shell_rc="$HOME/.profile" ;;
    esac
    
    if [[ -f "$shell_rc" ]]; then
        # SENTIENT ile ilgili satırları kaldır
        sed -i '/# SENTIENT OS/d' "$shell_rc" 2>/dev/null || true
        sed -i '/sentient/d' "$shell_rc" 2>/dev/null || true
        log_ok "PATH'ten kaldırıldı"
    fi
    
    echo ""
    log_ok "SENTIENT OS başarıyla kaldırıldı"
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ANA AKIŞ
# ═══════════════════════════════════════════════════════════════════════════════

main() {
    # Parametreleri parse et
    parse_args "$@"
    
    # Kaldırma modu
    if [[ "$UNINSTALL" == "true" ]]; then
        uninstall_sentient
        return
    fi
    
    # Adımlar
    local steps=(
        "show_welcome"
        "show_license"
        "analyze_system"
        "select_mode"
        "select_provider"
        "select_model"
        "select_components"
        "install_prerequisites"
        "download_source"
        "build_project"
        "configure_environment"
        "validate_installation"
    )
    
    local step_num=0
    local total_steps=${#steps[@]}
    
    for step_fn in "${steps[@]}"; do
        step_num=$((step_num + 1))
        
        echo ""
        echo -e "${DIM}  ─────────────────────────────────────────────────────────────────────${NC}"
        echo -e "${WHITE}  ADIM $step_num/$total_steps: ${step_fn//_/ }${NC}"
        echo -e "${DIM}  ─────────────────────────────────────────────────────────────────────${NC}"
        
        if ! $step_fn; then
            log_error "${step_fn//_/ } başarısız!"
            log_info "Kurulum durduruldu."
            exit 1
        fi
    done
    
    # Tamamlandı
    show_complete
}

# Script'i çalıştır
main "$@"
