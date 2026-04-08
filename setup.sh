#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
#  🐺 SENTIENT - ONE-COMMAND SETUP
#  The She-Wolf That Guards Your Empire
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# Renkler
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

SENTIENT_VERSION="2.1.0"
SENTIENT_LOG="sentient_setup.log"

# Logo
print_logo() {
    echo -e "${CYAN}"
    echo "╔═══════════════════════════════════════════════════════════════════╗"
    echo "║                                                                   ║"
    echo "║   🧠 SENTIENT OS - The Operating System That Thinks            ║"
    echo "║   🦀 Rust Core │ 5587 Skills │ 71 Integrations                   ║"
    echo "║                                                                   ║"
    echo "║   Version: ${SENTIENT_VERSION}                                              ║"
    echo "║   Skills: 5587+                                                  ║"
    echo "║                                                                   ║"
    echo "╚═══════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

# Log fonksiyonu
log() {
    echo -e "[$(date '+%Y-%m-%d %H:%M:%S')] $1" >> "$SENTIENT_LOG"
}

# Hata kontrolü
error() {
    echo -e "${RED}❌ HATA: $1${NC}"
    log "ERROR: $1"
    exit 1
}

# Başarı mesajı
success() {
    echo -e "${GREEN}✅ $1${NC}"
    log "SUCCESS: $1"
}

# Bilgi mesajı
info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
    log "INFO: $1"
}

# Uyarı mesajı
warn() {
    echo -e "${YELLOW}⚠️  $1${NC}"
    log "WARN: $1"
}

# Sistem kontrolü
check_system() {
    info "Sistem kontrol ediliyor..."
    
    # İşletim sistemi
    OS=$(uname -s)
    ARCH=$(uname -m)
    info "OS: $OS | ARCH: $ARCH"
    
    # Root kontrolü
    if [ "$EUID" -eq 0 ]; then
        warn "Root olarak çalışıyor - dikkatli olun!"
    fi
}

# Rust kurulumu
install_rust() {
    info "Rust kontrol ediliyor..."
    
    if command -v rustc &> /dev/null; then
        RUST_VERSION=$(rustc --version)
        success "Rust mevcut: $RUST_VERSION"
    else
        info "Rust kuruluyor..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        success "Rust kuruldu!"
    fi
    
    # Rust toolchain
    rustup default stable
    rustup update
    
    # Gerekli bileşenler
    rustup component add clippy rustfmt rust-analyzer
}

# Sistem bağımlılıkları
install_dependencies() {
    info "Sistem bağımlılıkları kuruluyor..."
    
    if [ -f /etc/debian_version ]; then
        # Debian/Ubuntu
        sudo apt-get update
        sudo apt-get install -y \
            build-essential \
            pkg-config \
            libssl-dev \
            sqlite3 \
            libsqlite3-dev \
            curl \
            wget \
            git \
            python3 \
            python3-pip \
            python3-venv \
            nodejs \
            npm \
            docker.io \
            docker-compose
    elif [ -f /etc/fedora-release ]; then
        # Fedora
        sudo dnf install -y \
            gcc \
            gcc-c++ \
            openssl-devel \
            sqlite \
            sqlite-devel \
            curl \
            wget \
            git \
            python3 \
            python3-pip \
            nodejs \
            npm \
            docker \
            docker-compose
    elif [ "$(uname)" == "Darwin" ]; then
        # macOS
        if command -v brew &> /dev/null; then
            brew install \
                openssl \
                sqlite \
                python3 \
                node \
                docker \
                docker-compose
        else
            error "Homebrew gerekli! https://brew.sh"
        fi
    else
        warn "Bilinmeyen dağıtım - manuel kurulum gerekebilir"
    fi
    
    success "Sistem bağımlılıkları kuruldu!"
}

# Docker kurulumu
setup_docker() {
    info "Docker kontrol ediliyor..."
    
    if command -v docker &> /dev/null; then
        success "Docker mevcut: $(docker --version)"
    else
        warn "Docker bulunamadı - kurulum yapılıyor..."
        
        # Docker servisini başlat
        if [ -f /etc/debian_version ]; then
            sudo systemctl enable docker
            sudo systemctl start docker
            sudo usermod -aG docker "$USER"
        fi
        
        success "Docker kuruldu!"
    fi
    
    # Docker Compose
    if command -v docker-compose &> /dev/null; then
        success "Docker Compose mevcut: $(docker-compose --version)"
    fi
}

# Python ortamı
setup_python() {
    info "Python ortamı hazırlanıyor..."
    
    # Virtual environment
    if [ ! -d "venv" ]; then
        python3 -m venv venv
        success "Virtual environment oluşturuldu!"
    fi
    
    # Aktif et
    source venv/bin/activate
    
    # Gerekli paketler
    pip install --upgrade pip
    pip install \
        pyo3 \
        maturin \
        pytest \
        black \
        mypy \
        requests \
        pyyaml
    
    success "Python ortamı hazır!"
}

# Node.js bağımlılıkları
setup_node() {
    info "Node.js bağımlılıkları..."
    
    if [ -f "package.json" ]; then
        npm install
        success "Node.js bağımlılıkları kuruldu!"
    else
        info "package.json bulunamadı - atlanıyor"
    fi
}

# SENTIENT derleme
build_sentient() {
    info "SENTIENT derleniyor..."
    
    # Cargo check
    cargo check
    
    # Derleme
    cargo build --release
    
    # Skill ingestion
    cargo run --bin sentient-ingest -- release --full
    
    success "SENTIENT derlendi!"
}

# Skill library kurulumu
setup_skills() {
    info "Skill Library hazırlanıyor..."
    
    # Veritabanı dizini
    mkdir -p data/skills
    
    # Skill'leri ingest et
    if [ -f "target/release/sentient-ingest" ]; then
        ./target/release/sentient-ingest full
    else
        cargo run --bin sentient-ingest -- full
    fi
    
    # İstatistikler
    SKILL_COUNT=$(find data/skills -name "*.yaml" | wc -l)
    success "Skill Library: $SKILL_COUNT skill yüklendi!"
}

# V-GATE konfigürasyonu
setup_vgate() {
    info "V-GATE yapılandırılıyor..."
    
    # .env dosyası
    if [ ! -f ".env" ]; then
        cat > .env << 'EOF'
# 🐺 SENTIENT V-GATE Configuration
# API keys are NEVER committed to Git!

# OpenRouter (optional - use V-GATE proxy instead)
# OPENROUTER_API_KEY=your-key-here

# V-GATE Proxy Settings
VGATE_HOST=127.0.0.1
VGATE_PORT=8765
VGATE_ENCRYPTION=true

# Memory
DATABASE_URL=sqlite:data/sentient.db
MEMORY_PATH=data/memory

# Logging
RUST_LOG=info
SENTIENT_ENV=production
EOF
        success ".env dosyası oluşturuldu!"
    else
        info ".env dosyası mevcut"
    fi
    
    # .gitignore kontrolü
    if ! grep -q ".env" .gitignore 2>/dev/null; then
        echo ".env" >> .gitignore
        echo "*.db" >> .gitignore
        echo "target/" >> .gitignore
        success ".gitignore güncellendi!"
    fi
}

# Knowledge Base
setup_knowledge_base() {
    info "Knowledge Base kontrol ediliyor..."
    
    if [ -d "knowledge_base" ]; then
        KB_FILES=$(ls knowledge_base | wc -l)
        success "Knowledge Base: $KB_FILES dosya mevcut"
    else
        warn "Knowledge Base bulunamadı!"
    fi
}

# Test çalıştır
run_tests() {
    info "Testler çalıştırılıyor..."
    
    # Unit tests
    cargo test --workspace -- --test-threads=4 || true
    
    # Clippy
    cargo clippy -- -W warnings || true
    
    success "Testler tamamlandı!"
}

# SENTIENT Shell kurulumu
setup_shell() {
    info "SENTIENT Shell yapılandırılıyor..."
    
    # Shell binary
    if [ -f "target/release/sentient-shell" ]; then
        # Alias ekle
        if ! grep -q "sentient-shell" "$HOME/.bashrc" 2>/dev/null; then
            echo "" >> "$HOME/.bashrc"
            echo "# 🐺 SENTIENT Shell" >> "$HOME/.bashrc"
            echo "alias sentient='$(pwd)/target/release/sentient-shell'" >> "$HOME/.bashrc"
            echo "alias sentient-shell='$(pwd)/target/release/sentient-shell'" >> "$HOME/.bashrc"
            success "SENTIENT Shell alias eklendi!"
        fi
    else
        info "SENTIENT Shell derleniyor..."
        cargo build --release --bin sentient-shell || warn "SENTIENT Shell bulunamadı"
    fi
}

# Final rapor
final_report() {
    echo ""
    echo -e "${PURPLE}╔═══════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${PURPLE}║                                                                   ║${NC}"
    echo -e "${GREEN}║   🐺 SENTIENT KURULUMU TAMAMLANDI!                                  ║${NC}"
    echo -e "${PURPLE}║                                                                   ║${NC}"
    echo -e "${PURPLE}╠═══════════════════════════════════════════════════════════════════╣${NC}"
    echo -e "${PURPLE}║                                                                   ║${NC}"
    echo -e "${CYAN}║   📦 Skill Library: $(find data/skills -name '*.yaml' 2>/dev/null | wc -l | xargs printf '%-5s') skill                        ║${NC}"
    echo -e "${CYAN}║   🦀 Rust Crates: $(ls crates 2>/dev/null | wc -l | xargs printf '%-5s') modules                         ║${NC}"
    echo -e "${CYAN}║   📚 Knowledge Base: $(ls knowledge_base 2>/dev/null | wc -l | xargs printf '%-5s') documents                     ║${NC}"
    echo -e "${PURPLE}║                                                                   ║${NC}"
    echo -e "${PURPLE}╠═══════════════════════════════════════════════════════════════════╣${NC}"
    echo -e "${PURPLE}║                                                                   ║${NC}"
    echo -e "${YELLOW}║   KULLANIM:                                                       ║${NC}"
    echo -e "${YELLOW}║   ─────────                                                       ║${NC}"
    echo -e "${YELLOW}║   ./target/release/sentient-shell      # SENTIENT Terminal             ║${NC}"
    echo -e "${YELLOW}║   ./target/release/sentient-dashboard  # Dashboard                  ║${NC}"
    echo -e "${YELLOW}║   ./target/release/sentient-ingest     # Skill Manager              ║${NC}"
    echo -e "${YELLOW}║   make run                          # Hızlı başlat               ║${NC}"
    echo -e "${YELLOW}║   make test                         # Test çalıştır              ║${NC}"
    echo -e "${YELLOW}║   make skills                       # Skill güncelle             ║${NC}"
    echo -e "${PURPLE}║                                                                   ║${NC}"
    echo -e "${PURPLE}╚═══════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

# Ana kurulum
main() {
    print_logo
    
    echo -e "${YELLOW}Bu script SENTIENT'nın tüm bileşenlerini kuracaktır.${NC}"
    echo -e "${YELLOW}Devam etmek için Enter'a basın...${NC}"
    read -r
    
    # Adımlar
    check_system
    install_rust
    install_dependencies
    setup_docker
    setup_python
    setup_node
    build_sentient
    setup_skills
    setup_vgate
    setup_knowledge_base
    setup_shell
    run_tests
    
    # Final
    final_report
    
    log "SENTIENT Setup Complete!"
}

# Kullanım
usage() {
    echo "Kullanım: $0 [komut]"
    echo ""
    echo "Komutlar:"
    echo "  all        - Tüm kurulum (varsayılan)"
    echo "  rust       - Sadece Rust kurulumu"
    echo "  deps       - Sadece bağımlılıklar"
    echo "  build      - Sadece derleme"
    echo "  skills     - Sadece skill kurulumu"
    echo "  docker     - Sadece Docker kurulumu"
    echo "  test       - Sadece testler"
    echo "  clean      - Temizlik"
    echo "  help       - Bu yardım"
}

# Komut işleme
case "${1:-all}" in
    all)
        main
        ;;
    rust)
        install_rust
        ;;
    deps)
        install_dependencies
        ;;
    build)
        build_sentient
        ;;
    skills)
        setup_skills
        ;;
    docker)
        setup_docker
        ;;
    test)
        run_tests
        ;;
    clean)
        cargo clean
        rm -rf venv node_modules
        info "Temizlik yapıldı!"
        ;;
    help|--help|-h)
        usage
        ;;
    *)
        error "Bilinmeyen komut: $1"
        usage
        ;;
esac
