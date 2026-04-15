#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS - Ollama Model Kurulum Scripti
# ═══════════════════════════════════════════════════════════════════════════════
#  Kullanım: ./scripts/setup-ollama.sh
#  Local LLM modellerini indirir
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# Renk kodları
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
PURPLE='\033[0;35m'
NC='\033[0m'

echo -e "${CYAN}"
echo "╔══════════════════════════════════════════════════════════════════════════════╗"
echo "║                      🦙 Ollama Model Kurulumu                                ║"
echo "╚══════════════════════════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# Ollama kontrolü
echo -e "${BLUE}🔍 Ollama kontrol ediliyor...${NC}"

if ! command -v ollama &> /dev/null; then
    echo -e "${RED}❌ Ollama kurulu değil!${NC}"
    echo ""
    echo -e "${YELLOW}Kurulum seçenekleri:${NC}"
    echo ""
    echo -e "${CYAN}Linux/macOS:${NC}"
    echo -e "  curl -fsSL https://ollama.ai/install.sh | sh"
    echo ""
    echo -e "${CYAN}Windows:${NC}"
    echo -e "  https://ollama.ai/download/windows"
    echo ""
    echo -e "${CYAN}Docker:${NC}"
    echo -e "  docker run -d --gpus=all -v ollama:/root/.ollama -p 11434:11434 --name ollama ollama/ollama"
    echo ""
    exit 1
fi

echo -e "${GREEN}✅ Ollama kurulu: $(ollama --version)${NC}"
echo ""

# Ollama çalışıyor mu?
echo -e "${BLUE}🔍 Ollama servisi kontrol ediliyor...${NC}"

OLLAMA_URL=${OLLAMA_HOST:-http://localhost:11434}

if ! curl -s $OLLAMA_URL/api/tags &>/dev/null; then
    echo -e "${YELLOW}⚠️  Ollama servisi çalışmıyor. Başlatılıyor...${NC}"
    ollama serve &
    sleep 5
fi

if curl -s $OLLAMA_URL/api/tags &>/dev/null; then
    echo -e "${GREEN}✅ Ollama servisi çalışıyor${NC}"
else
    echo -e "${RED}❌ Ollama servisi başlatılamadı${NC}"
    exit 1
fi

echo ""

# Mevcut modelleri listele
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${PURPLE}  📋 Mevcut Modeller${NC}"
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo ""

ollama list

echo ""

# Önerilen modeller
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${PURPLE}  🎯 Önerilen Modeller${NC}"
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo ""

echo -e "${CYAN}┌── 🧠 Büyük Modeller (24GB+ VRAM)${NC}"
echo -e "${CYAN}│${NC}"
echo -e "${CYAN}│  gemma3:27b       - Google en güçlü açık model${NC}"
echo -e "${CYAN}│  deepseek-r1:67b  - DeepSeek reasoning model${NC}"
echo -e "${CYAN}│  llama3.3:70b     - Meta en büyük model${NC}"
echo -e "${CYAN}│${NC}"
echo -e "${CYAN}└──${NC}"
echo ""

echo -e "${CYAN}┌── ⚡ Orta Modeller (12-24GB VRAM)${NC}"
echo -e "${CYAN}│${NC}"
echo -e "${CYAN}│  gemma3:12b      - Dengeli performans${NC}"
echo -e "${CYAN}│  llama3.2:14b    - Meta orta boyut${NC}"
echo -e "${CYAN}│  mistral:7b      - Hızlı ve yetenekli${NC}"
echo -e "${CYAN}│  qwen2.5:14b     - Alibaba güçlü model${NC}"
echo -e "${CYAN}│${NC}"
echo -e "${CYAN}└──${NC}"
echo ""

echo -e "${CYAN}┌── 🚀 Hafif Modeller (8GB veya CPU)${NC}"
echo -e "${CYAN}│${NC}"
echo -e "${CYAN}│  gemma3:4b       - Çok hızlı${NC}"
echo -e "${CYAN}│  llama3.2:3b     - Minimal kaynak${NC}"
echo -e "${CYAN}│  phi4:mini       - Microsoft küçük model${NC}"
echo -e "${CYAN}│${NC}"
echo -e "${CYAN}└──${NC}"
echo ""

# Model seçimi
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${PURPLE}  📥 Model İndirme${NC}"
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo ""

# Varsayılan modeller
DEFAULT_MODELS=("gemma3:27b" "deepseek-r1:67b")

echo -e "${YELLOW}Varsayılan modeller: ${DEFAULT_MODELS[*]}${NC}"
echo -e "${YELLOW}Farklı modeller indirmek için model adını girin.${NC}"
echo -e "${YELLOW}Boş bırakırsanız varsayılanlar indirilir.${NC}"
echo ""

read -p "$(echo -e ${CYAN}İndirilecek modeller (boşlukla ayırın): ${NC})" INPUT_MODELS

if [ -z "$INPUT_MODELS" ]; then
    MODELS=("${DEFAULT_MODELS[@]}")
else
    MODELS=($INPUT_MODELS)
fi

echo ""
echo -e "${GREEN}İndirilecek modeller: ${MODELS[*]}${NC}"
echo ""

for MODEL in "${MODELS[@]}"; do
    echo -e "${BLUE}⬇️  $MODEL indiriliyor...${NC}"
    
    if ollama pull $MODEL; then
        echo -e "${GREEN}   ✅ $MODEL başarıyla indirildi${NC}"
    else
        echo -e "${RED}   ❌ $MODEL indirilemedi${NC}"
    fi
    
    echo ""
done

# Son durum
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${PURPLE}  📋 Yüklü Modeller${NC}"
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo ""

ollama list

echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  ✅ Ollama Kurulumu Tamamlandı!${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo ""

echo -e "${CYAN}🧪 Test etmek için:${NC}"
echo -e "${CYAN}   ollama run ${MODELS[0]}${NC}"
echo ""
echo -e "${CYAN}📡 API endpoint:${NC}"
echo -e "${CYAN}   http://localhost:11434/api/generate${NC}"
echo ""
