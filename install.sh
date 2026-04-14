#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS - OTOMATİK KURULUM SCRIPTİ
# ═══════════════════════════════════════════════════════════════════════════════
#  Tek komutla: curl -fsSL https://.../install.sh | bash
#  Veya manuel: ./install.sh
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# Renkler
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color
BOLD='\033[1m'

# Logo
show_logo() {
    echo -e "${CYAN}"
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║                                                            ║"
    echo "║     █████╗ ███╗   ██╗███████╗██╗      ██████╗ ██╗   ██╗   ║"
    echo "║    ██╔══██╗████╗  ██║██╔════╝██║     ██╔═══██╗██║   ██║   ║"
    echo "║    ███████║██╔██╗ ██║█████╗  ██║     ██║   ██║██║   ██║   ║"
    echo "║    ██╔══██║██║╚██╗██║██╔══╝  ██║     ██║   ██║██║   ██║   ║"
    echo "║    ██║  ██║██║ ╚████║███████╗███████╗╚██████╔╝╚██████╔╝   ║"
    echo "║    ╚═╝  ╚═╝╚═╝  ╚═══╝╚══════╝╚══════╝ ╚═════╝  ╚═════╝    ║"
    echo "║                                                            ║"
    echo "║            NEXUS OASIS — Yapay Zeka İşletim Sistemi        ║"
    echo "╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

# Progress göstergesi
progress() {
    local step=$1
    local total=$2
    local desc=$3
    echo -e "${BLUE}[${step}/${total}]${NC} ${BOLD}${desc}${NC}"
}

# Başarı mesajı
success() {
    echo -e "${GREEN}✅ $1${NC}"
}

# Hata mesajı
error() {
    echo -e "${RED}❌ $1${NC}"
    exit 1
}

# Uyarı mesajı
warn() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

# Soru sor
ask() {
    echo -e -n "${CYAN}$1${NC}"
    read -r answer
    echo "$answer"
}

# ═══════════════════════════════════════════════════════════════════════════════
#  LLM SEÇİMİ
# ═══════════════════════════════════════════════════════════════════════════════

select_llm() {
    echo ""
    echo -e "${BOLD}══════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${BOLD}  🧠 LLM (Yapay Zeka Modeli) Seçimi${NC}"
    echo -e "${BOLD}══════════════════════════════════════════════════════════════════════${NC}"
    echo ""
    echo -e "Lütfen kullanmak istediğiniz LLM türünü seçin:"
    echo ""
    echo -e "  ${GREEN}[1]${NC} ${BOLD}LOKAL (Ücretsiz)${NC} - Ollama ile bilgisayarınızda çalışır"
    echo "      • İnternet gerektirmez"
    echo "      • Tamamen gizli (veri dışarı çıkmaz)"
    echo "      • GPU destekli (NVIDIA, AMD, Apple Silicon)"
    echo "      • Modeller: Llama3.2, Gemma3, DeepSeek, Qwen..."
    echo ""
    echo -e "  ${GREEN}[2]${NC} ${BOLD}API KEY (Ücretli)${NC} - OpenRouter, OpenAI, Anthropic"
    echo "      • En iyi kalite (GPT-4, Claude, Gemini)"
    echo "      • 200+ model erişimi (OpenRouter)"
    echo "      • Kurulum çok hızlı"
    echo "      • Pay-as-you-go定价"
    echo ""
    echo -e "  ${GREEN}[3]${NC} ${BOLD}HİBRİT${NC} - Hem lokal hem API"
    echo "      • Normalde lokal kullanır"
    echo "      • Zor sorularda API'ye başvurur"
    echo "      • En iyi denge"
    echo ""
    
    answer=$(ask "Seçiminiz [1/2/3]: ")
    
    case $answer in
        1)
            LLM_TYPE="local"
            setup_local_llm
            ;;
        2)
            LLM_TYPE="api"
            setup_api_llm
            ;;
        3)
            LLM_TYPE="hybrid"
            setup_hybrid_llm
            ;;
        *)
            error "Geçersiz seçim. Lütfen 1, 2 veya 3 girin."
            ;;
    esac
}

# ═══════════════════════════════════════════════════════════════════════════════
#  LOKAL LLM KURULUMU (OLLAMA)
# ═══════════════════════════════════════════════════════════════════════════════

setup_local_llm() {
    echo ""
    echo -e "${BOLD}══════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${BOLD}  🦙 Ollama (Lokal LLM) Kurulumu${NC}"
    echo -e "${BOLD}══════════════════════════════════════════════════════════════════════${NC}"
    echo ""
    
    # Model seçimi
    echo -e "Hangi modeli kurmak istersiniz?"
    echo ""
    echo -e "  ${GREEN}[1]${NC} Llama 3.2 (3B)    ~2GB   - Hızlı, genel amaçlı"
    echo -e "  ${GREEN}[2]${NC} Gemma 3 (4B)     ~3GB   - Google, dengeli"
    echo -e "  ${GREEN}[3]${NC} Qwen 2.5 (7B)    ~4GB   - Çok dilli, Türkçe iyi"
    echo -e "  ${GREEN}[4]${NC} DeepSeek R1 (7B) ~4GB   - Reasoning, düşünme"
    echo -e "  ${GREEN}[5]${NC} Llama 3.2 (11B)  ~7GB   - Daha akıllı"
    echo -e "  ${GREEN}[6]${NC} Gemma 3 (27B)    ~16GB  - En iyi kalite (önerilen)"
    echo -e "  ${GREEN}[7]${NC} DeepSeek R1 (67B)~40GB  - En akıllı (40GB+ RAM)"
    echo ""
    
    model_choice=$(ask "Model seçimi [1-7, default=6]: ")
    model_choice=${model_choice:-6}
    
    case $model_choice in
        1) OLLAMA_MODEL="llama3.2" ;;
        2) OLLAMA_MODEL="gemma3:4b" ;;
        3) OLLAMA_MODEL="qwen2.5:7b" ;;
        4) OLLAMA_MODEL="deepseek-r1:7b" ;;
        5) OLLAMA_MODEL="llama3.2:11b" ;;
        6) OLLAMA_MODEL="gemma3:27b" ;;
        7) OLLAMA_MODEL="deepseek-r1:67b" ;;
        *) OLLAMA_MODEL="gemma3:27b" ;;
    esac
    
    echo ""
    progress "1" "4" "Ollama kuruluyor..."
    
    # Ollama kur
    if command -v ollama &> /dev/null; then
        success "Ollama zaten kurulu"
    else
        curl -fsSL https://ollama.com/install.sh | sh
        success "Ollama kuruldu"
    fi
    
    progress "2" "4" "Model indiriliyor: $OLLAMA_MODEL (bu biraz sürebilir)..."
    ollama pull "$OLLAMA_MODEL"
    success "$OLLAMA_MODEL indirildi"
    
    progress "3" "4" "Ollama servisi başlatılıyor..."
    ollama serve &
    sleep 3
    success "Ollama çalışıyor"
    
    progress "4" "4" "Test..."
    ollama run "$OLLAMA_MODEL" "Merhaba, tek kelimeyle cevap ver"
    success "LLM çalışıyor!"
    
    # .env güncelle
    echo "OLLAMA_HOST=http://localhost:11434" >> .env
    echo "OLLAMA_MODEL=$OLLAMA_MODEL" >> .env
    echo "LLM_PROVIDER=ollama" >> .env
    
    LLM_ENDPOINT="http://localhost:11434"
}

# ═══════════════════════════════════════════════════════════════════════════════
#  API LLM KURULUMU
# ═══════════════════════════════════════════════════════════════════════════════

setup_api_llm() {
    echo ""
    echo -e "${BOLD}══════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${BOLD}  🔑 API Key ile LLM Kurulumu${NC}"
    echo -e "${BOLD}══════════════════════════════════════════════════════════════════════${NC}"
    echo ""
    
    echo -e "Hangi sağlayıcıyı kullanmak istersiniz?"
    echo ""
    echo -e "  ${GREEN}[1]${NC} ${BOLD}OpenRouter${NC} (önerilen) - 200+ model, $5 başlangıç bonusu"
    echo "      • GPT-4, Claude, Gemini, Llama, Mistral..."
    echo "      • Site: https://openrouter.ai/keys"
    echo ""
    echo -e "  ${GREEN}[2]${NC} ${BOLD}OpenAI${NC} - GPT-4, GPT-4o, GPT-4o-mini"
    echo "      • Site: https://platform.openai.com/api-keys"
    echo ""
    echo -e "  ${GREEN}[3]${NC} ${BOLD}Anthropic${NC} - Claude 3.5 Sonnet, Claude 3 Opus"
    echo "      • Site: https://console.anthropic.com/"
    echo ""
    echo -e "  ${GREEN}[4]${NC} ${BOLD}Google AI${NC} - Gemini 1.5 Pro, Gemini 2.0"
    echo "      • Site: https://aistudio.google.com/apikey"
    echo ""
    
    provider_choice=$(ask "Sağlayıcı seçimi [1-4]: ")
    
    case $provider_choice in
        1)
            API_PROVIDER="openrouter"
            echo ""
            echo -e "${CYAN}OpenRouter API Key almak için:${NC}"
            echo "  1. https://openrouter.ai/keys adresine git"
            echo "  2. 'Create Key' tıkla"
            echo "  3. Key'i kopyala"
            echo ""
            API_KEY=$(ask "API Key'inizi girin: ")
            
            echo ""
            echo -e "Varsayılan model:"
            echo -e "  ${GREEN}[1]${NC} GPT-4o-mini (hızlı, ucuz)"
            echo -e "  ${GREEN}[2]${NC} GPT-4o (dengeli)"
            echo -e "  ${GREEN}[3]${NC} Claude 3.5 Sonnet (en iyi)"
            echo -e "  ${GREEN}[4]${NC} Gemini 2.0 Flash (hızlı)"
            echo -e "  ${GREEN}[5]${NC} Llama 3.3 70B (açık kaynak, güçlü)"
            echo ""
            model_choice=$(ask "Model [1-5, default=1]: ")
            model_choice=${model_choice:-1}
            
            case $model_choice in
                1) API_MODEL="openai/gpt-4o-mini" ;;
                2) API_MODEL="openai/gpt-4o" ;;
                3) API_MODEL="anthropic/claude-3.5-sonnet" ;;
                4) API_MODEL="google/gemini-2.0-flash-exp" ;;
                5) API_MODEL="meta-llama/llama-3.3-70b-instruct" ;;
                *) API_MODEL="openai/gpt-4o-mini" ;;
            esac
            ;;
        2)
            API_PROVIDER="openai"
            echo ""
            echo -e "${CYAN}OpenAI API Key almak için:${NC}"
            echo "  1. https://platform.openai.com/api-keys"
            echo "  2. 'Create new secret key'"
            echo ""
            API_KEY=$(ask "API Key'inizi girin: ")
            API_MODEL="gpt-4o-mini"
            ;;
        3)
            API_PROVIDER="anthropic"
            echo ""
            echo -e "${CYAN}Anthropic API Key almak için:${NC}"
            echo "  1. https://console.anthropic.com/"
            echo "  2. 'Get API Keys'"
            echo ""
            API_KEY=$(ask "API Key'inizi girin: ")
            API_MODEL="claude-3-5-sonnet-20241022"
            ;;
        4)
            API_PROVIDER="google"
            echo ""
            echo -e "${CYAN}Google AI API Key almak için:${NC}"
            echo "  1. https://aistudio.google.com/apikey"
            echo ""
            API_KEY=$(ask "API Key'inizi girin: ")
            API_MODEL="gemini-2.0-flash"
            ;;
        *)
            error "Geçersiz seçim"
            ;;
    esac
    
    # Test
    echo ""
    progress "1" "2" "API bağlantısı test ediliyor..."
    
    if [ "$API_PROVIDER" = "openrouter" ]; then
        response=$(curl -s -w "%{http_code}" "https://openrouter.ai/api/v1/chat/completions" \
            -H "Authorization: Bearer $API_KEY" \
            -H "Content-Type: application/json" \
            -d "{\"model\":\"$API_MODEL\",\"messages\":[{\"role\":\"user\",\"content\":\"Hi\"}]}" \
            --max-time 30)
        http_code="${response: -3}"
        if [ "$http_code" = "200" ]; then
            success "API bağlantısı başarılı!"
        else
            warn "API test başarısız (HTTP $http_code). Key'i kontrol edin."
        fi
    fi
    
    progress "2" "2" "Yapılandırma kaydediliyor..."
    
    # .env güncelle (API KEY'İ DOSYAYA YAZMIYORUZ - sadece ortam değişkeni)
    echo "LLM_PROVIDER=$API_PROVIDER" >> .env
    echo "LLM_MODEL=$API_MODEL" >> .env
    
    # API key'i export et (geçici)
    if [ "$API_PROVIDER" = "openrouter" ]; then
        export OPENROUTER_API_KEY="$API_KEY"
        LLM_ENV="OPENROUTER_API_KEY"
    elif [ "$API_PROVIDER" = "openai" ]; then
        export OPENAI_API_KEY="$API_KEY"
        LLM_ENV="OPENAI_API_KEY"
    elif [ "$API_PROVIDER" = "anthropic" ]; then
        export ANTHROPIC_API_KEY="$API_KEY"
        LLM_ENV="ANTHROPIC_API_KEY"
    elif [ "$API_PROVIDER" = "google" ]; then
        export GOOGLE_API_KEY="$API_KEY"
        LLM_ENV="GOOGLE_API_KEY"
    fi
    
    success "API yapılandırması tamamlandı"
    
    echo ""
    warn "API Key .env dosyasına YAZILMADI (güvenlik)"
    echo -e "Çalıştırırken şöyle kullanın:"
    echo -e "  ${CYAN}$LLM_ENV=$API_KEY ./target/release/sentient gateway${NC}"
}

# ═══════════════════════════════════════════════════════════════════════════════
#  HİBRİT LLM KURULUMU
# ═══════════════════════════════════════════════════════════════════════════════

setup_hybrid_llm() {
    echo ""
    echo -e "${BOLD}══════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${BOLD}  🔄 Hibrit LLM Kurulumu${NC}"
    echo -e "${BOLD}══════════════════════════════════════════════════════════════════════${NC}"
    echo ""
    
    # Önce lokal
    echo -e "${CYAN}>>> Önce Lokal LLM kurulumu:${NC}"
    setup_local_llm
    
    echo ""
    # Sonra API
    echo -e "${CYAN}>>> Şimdi API LLM kurulumu (fallback için):${NC}"
    setup_api_llm
    
    # Hibrit config
    echo "LLM_MODE=hybrid" >> .env
    echo "LLM_LOCAL_MODEL=$OLLAMA_MODEL" >> .env
    echo "LLM_API_MODEL=$API_MODEL" >> .env
    
    success "Hibrit yapılandırma tamamlandı!"
}

# ═══════════════════════════════════════════════════════════════════════════════
#  VOICE KURULUMU
# ═══════════════════════════════════════════════════════════════════════════════

setup_voice() {
    echo ""
    echo -e "${BOLD}══════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${BOLD}  🎙️  Voice (Ses) Kurulumu${NC}"
    echo -e "${BOLD}══════════════════════════════════════════════════════════════════════${NC}"
    echo ""
    
    echo -e "Ses özelliklerini kurmak ister misiniz?"
    echo ""
    echo -e "  ${GREEN}[1]${NC} Evet - Lokal (Whisper.cpp + Piper TTS, ücretsiz)"
    echo -e "  ${GREEN}[2]${NC} Evet - API (OpenAI Whisper + ElevenLabs, ücretli)"
    echo -e "  ${GREEN}[3]${NC} Hayır - Sadece metin"
    echo ""
    
    voice_choice=$(ask "Seçiminiz [1-3, default=1]: ")
    voice_choice=${voice_choice:-1}
    
    case $voice_choice in
        1) setup_local_voice ;;
        2) setup_api_voice ;;
        3) 
            echo "VOICE_ENABLED=false" >> .env
            warn "Voice atlanıyor"
            ;;
    esac
}

setup_local_voice() {
    echo ""
    progress "1" "4" "Whisper.cpp kuruluyor (STT - Konuşmadan metne)..."
    
    # Whisper.cpp
    if [ ! -d "whisper.cpp" ]; then
        git clone https://github.com/ggerganov/whisper.cpp
    fi
    cd whisper.cpp
    make
    bash ./models/download-ggml-model.sh medium
    cd ..
    
    success "Whisper.cpp kuruldu"
    
    progress "2" "4" "Piper TTS kuruluyor (Metinden konuşmaya)..."
    
    # Piper
    mkdir -p ~/.local/share/piper/models
    wget -q https://github.com/rhasspy/piper/releases/download/v1.2.0/piper_1.2.0_amd64.tar.gz
    tar -xzf piper_1.2.0_amd64.tar.gz
    sudo mv piper/piper /usr/local/bin/ 2>/dev/null || true
    
    # Türkçe model
    cd ~/.local/share/piper/models
    wget -q https://huggingface.co/rhasspy/piper-voices/resolve/main/tr/tr_TR/medium/tr_TR-medium.onnx
    wget -q https://huggingface.co/rhasspy/piper-voices/resolve/main/tr/tr_TR/medium/tr_TR-medium.onnx.json
    cd -
    
    success "Piper TTS kuruldu"
    
    progress "3" "4" "Ses sistemi bağımlılıkları..."
    sudo apt install -y portaudio19-dev libasound2-dev ffmpeg 2>/dev/null
    success "Bağımlılıklar kuruldu"
    
    progress "4" "4" "Yapılandırma..."
    echo "VOICE_ENABLED=true" >> .env
    echo "VOICE_STT=whisper_cpp" >> .env
    echo "VOICE_TTS=piper" >> .env
    echo "WHISPER_MODEL=medium" >> .env
    
    success "Voice kuruldu (tamamen lokal, ücretsiz)"
}

setup_api_voice() {
    echo ""
    echo -e "${CYAN}OpenAI Whisper API Key (STT):${NC}"
    echo "  https://platform.openai.com/api-keys"
    WHISPER_KEY=$(ask "API Key: ")
    
    echo ""
    echo -e "${CYAN}ElevenLabs API Key (TTS):${NC}"
    echo "  https://elevenlabs.io/app/settings/api-keys"
    ELEVENLABS_KEY=$(ask "API Key: ")
    
    echo "VOICE_ENABLED=true" >> .env
    echo "VOICE_STT=openai_whisper" >> .env
    echo "VOICE_TTS=elevenlabs" >> .env
    
    export OPENAI_API_KEY="$WHISPER_KEY"
    export ELEVENLABS_API_KEY="$ELEVENLABS_KEY"
    
    success "Voice API'leri yapılandırıldı"
    warn "API Key'ler .env'e yazılmadı (güvenlik)"
}

# ═══════════════════════════════════════════════════════════════════════════════
#  SİSTEM KURULUMU
# ═══════════════════════════════════════════════════════════════════════════════

install_dependencies() {
    echo ""
    echo -e "${BOLD}══════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${BOLD}  📦 Sistem Bağımlılıkları${NC}"
    echo -e "${BOLD}══════════════════════════════════════════════════════════════════════${NC}"
    echo ""
    
    progress "1" "4" "Paket listesi güncelleniyor..."
    sudo apt update
    success "Paket listesi güncellendi"
    
    progress "2" "4" "Temel bağımlılıklar kuruluyor..."
    sudo apt install -y \
        build-essential \
        pkg-config \
        libssl-dev \
        curl \
        git \
        ca-certificates
    success "Temel bağımlılıklar kuruldu"
    
    progress "3" "4" "Docker kontrol ediliyor..."
    if command -v docker &> /dev/null; then
        success "Docker zaten kurulu"
    else
        echo -e "${YELLOW}Docker kurulu değil. Kurmak ister misiniz? [y/N]:${NC}"
        read -r install_docker
        if [[ "$install_docker" =~ ^[Yy]$ ]]; then
            curl -fsSL https://get.docker.com | sh
            sudo usermod -aG docker $USER
            success "Docker kuruldu"
        else
            warn "Docker atlanıyor. Manuel kurmanız gerekecek."
        fi
    fi
    
    progress "4" "4" "Rust kontrol ediliyor..."
    if command -v rustc &> /dev/null; then
        success "Rust zaten kurulu ($(rustc --version))"
    else
        echo -e "${YELLOW}Rust kurulu değil. Kuruluyor...${NC}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source $HOME/.cargo/env
        success "Rust kuruldu"
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
#  DOCKER SERVİSLERİ
# ═══════════════════════════════════════════════════════════════════════════════

start_docker() {
    echo ""
    echo -e "${BOLD}══════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${BOLD}  🐳 Docker Servisleri${NC}"
    echo -e "${BOLD}══════════════════════════════════════════════════════════════════════${NC}"
    echo ""
    
    if ! command -v docker &> /dev/null; then
        warn "Docker kurulu değil, atlanıyor"
        return
    fi
    
    progress "1" "2" "Docker servisleri başlatılıyor..."
    docker-compose up -d
    success "Servisler başlatıldı"
    
    progress "2" "2" "Sağlık kontrolü..."
    sleep 5
    docker-compose ps
    success "Docker servisleri hazır"
}

# ═══════════════════════════════════════════════════════════════════════════════
#  RUST DERLEME
# ═══════════════════════════════════════════════════════════════════════════════

build_sentient() {
    echo ""
    echo -e "${BOLD}══════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${BOLD}  🦀 Rust Derleme${NC}"
    echo -e "${BOLD}══════════════════════════════════════════════════════════════════════${NC}"
    echo ""
    
    progress "1" "2" "SENTIENT derleniyor (release mode)..."
    echo -e "${YELLOW}Bu işlem 5-15 dakika sürebilir...${NC}"
    cargo build --release --bin sentient
    success "Derleme tamamlandı"
    
    progress "2" "2" "Binary kontrolü..."
    ls -lh target/release/sentient
    success "Binary hazır"
}

# ═══════════════════════════════════════════════════════════════════════════════
#  FINAL
# ═══════════════════════════════════════════════════════════════════════════════

show_final() {
    echo ""
    echo -e "${GREEN}╔══════════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║                    ✅ KURULUM TAMAMLANDI!                            ║${NC}"
    echo -e "${GREEN}╚══════════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${BOLD}Çalıştırmak için:${NC}"
    echo ""
    
    if [ "$LLM_TYPE" = "local" ]; then
        echo -e "  ${CYAN}./target/release/sentient gateway${NC}"
    elif [ "$LLM_TYPE" = "api" ]; then
        echo -e "  ${CYAN}$LLM_ENV=\$YOUR_KEY ./target/release/sentient gateway${NC}"
    else
        echo -e "  ${CYAN}$LLM_ENV=\$YOUR_KEY ./target/release/sentient gateway${NC}"
    fi
    
    echo ""
    echo -e "${BOLD}Dashboard:${NC} ${CYAN}http://localhost:8080/dashboard${NC}"
    echo -e "${BOLD}API Docs:${NC}  ${CYAN}http://localhost:8080/health${NC}"
    echo ""
    echo -e "${BOLD}Durdurmak için:${NC} Ctrl+C"
    echo ""
    
    echo -e "${YELLOW}Kurulum detayları:${NC}"
    echo "  • Config: .env"
    echo "  • Logs: ./logs/"
    echo "  • Docs: Arsiv/SISTEMI_AYAGA_KALDIRMA_REHBERI_TAM.md"
    echo ""
    
    echo -e "${GREEN}İyi kullanımlar! 🚀${NC}"
}

# ═══════════════════════════════════════════════════════════════════════════════
#  MAIN
# ═══════════════════════════════════════════════════════════════════════════════

main() {
    show_logo
    
    echo -e "${BOLD}Bu script SENTIENT OS'u bilgisayarınıza kuracak.${NC}"
    echo ""
    echo -e "Kurulum adımları:"
    echo "  1. Sistem bağımlılıkları"
    echo "  2. LLM seçimi (Lokal veya API)"
    echo "  3. Voice seçimi (isteğe bağlı)"
    echo "  4. Docker servisleri"
    echo "  5. Rust derleme"
    echo ""
    
    confirm=$(ask "Devam etmek istiyor musunuz? [Y/n]: ")
    if [[ "$confirm" =~ ^[Nn]$ ]]; then
        echo "İptal edildi."
        exit 0
    fi
    
    # .env oluştur
    cp .env.template .env 2>/dev/null || touch .env
    
    # Adımlar
    install_dependencies
    select_llm
    setup_voice
    start_docker
    build_sentient
    show_final
}

# Run
main "$@"
