# 📦 SENTIENT OS - Universal Kurulum Rehberi

> **Tüm platformlar — Linux, macOS, Windows, Docker, Kubernetes**

---

## 📑 İçindekiler

1. [Kurulum Yöntemleri](#1-kurulum-yöntemleri)
2. [Linux Kurulumu](#2-linux-kurulumu)
3. [macOS Kurulumu](#3-macos-kurulumu)
4. [Windows Kurulumu](#4-windows-kurulumu)
5. [Docker Kurulumu](#5-docker-kurulumu)
6. [Kubernetes Dağıtımı](#6-kubernetes-dağıtımı)
7. [Kaynaktan Derleme](#7-kaynaktan-derleme)
8. [LLM Yapılandırması](#8-llm-yapılandırması)
9. [Çapraz Derleme (Cross-Compile)](#9-çapraz-derleme-cross-compile)
10. [GitHub Release Oluşturma](#10-github-release-oluşturma)
11. [Sorun Giderme](#11-sorun-giderme)
12. [Doğrulama](#12-doğrulama)

---

## 1. Kurulum Yöntemleri

### Yöntem Karşılaştırma Tablosu

| Yöntem | Zorluk | Süre | İnternet | En İçin |
|--------|--------|------|----------|---------|
| **Tek Komut (curl)** | ⭐ Kolay | 5 dk | ✅ Gerekli | Yeni başlayanlar |
| **Homebrew** | ⭐ Kolay | 3 dk | ✅ Gerekli | macOS kullanıcıları |
| **npm** | ⭐ Kolay | 2 dk | ✅ Gerekli | JS geliştiricileri |
| **Binary İndirme** | ⭐⭐ Orta | 2 dk | ✅ Gerekli | Offline kurulum |
| **Docker** | ⭐⭐ Orta | 10 dk | ✅ Gerekli | Production |
| **Kaynaktan Derleme** | ⭐⭐⭐ İleri | 20-60 dk | ✅ Gerekli | Geliştiriciler |
| **Kubernetes** | ⭐⭐⭐ İleri | 30 dk | ✅ Gerekli | Enterprise |

---

## 2. Linux Kurulumu

### 2.1 Tek Komutla Kurulum (Önerilen)

```bash
# Varsayılan kurulum
curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.sh | bash

# Hızlı mod (soruları atla)
curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.sh | bash -s -- --yes --quick

# Tam kurulum (Docker dahil)
curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.sh | bash -s -- --full

# Belirli versiyon
curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.sh | bash -s -- --version v4.0.0

# Özel dizin
curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.sh | bash -s -- --prefix /opt/sentient

# Kaldırma
curl -fsSL ... | bash -s -- --uninstall
```

**Kurulum Adımları:**
1. ⚠️ Yasal uyarı → `y` onayla (AGPL v3, AI disclaimer)
2. 🖥️ Sistem tespiti → RAM, CPU, GPU algılama
3. 📦 Kurulum modu → Quick / Standard / Full
4. 🧠 LLM seçimi → 5 lokal (ücretsiz) + 8 API seçeneği
5. 🛠️ Modül seçimi → Voice, Dashboard, Channels
6. 🐳 Docker → Opsiyonel servisler
7. ⚙️ Yapılandırma → .env otomatik oluşturma
8. 🔨 Derleme → cargo build --release
9. ✅ Doğrulama → sentient doctor

### 2.2 Paket Yöneticileri

```bash
# Debian/Ubuntu (.deb)
wget https://github.com/nexsusagent-coder/SENTIENT_CORE/releases/download/v4.0.0/sentient_4.0.0_amd64.deb
sudo dpkg -i sentient_4.0.0_amd64.deb

# RHEL/Fedora (.rpm)
wget https://github.com/nexsusagent-coder/SENTIENT_CORE/releases/download/v4.0.0/sentient-4.0.0.x86_64.rpm
sudo rpm -i sentient-4.0.0.x86_64.rpm

# Arch Linux (AUR)
yay -S sentient-os
```

### 2.3 Sistem Gereksinimleri

| Mod | RAM | VRAM | Disk | Açıklama |
|-----|-----|------|------|----------|
| Minimal | 8 GB | - | 20 GB | API-only, Cloud LLM |
| Standard | 16 GB | 8 GB | 50 GB | Local LLM (küçük modeller) |
| Full | 32 GB | 24 GB+ | 100 GB | Büyük modeller (27B-70B) |
| Developer | 16 GB | 8 GB | 50 GB SSD | Kaynaktan derleme |

---

## 3. macOS Kurulumu

### 3.1 Homebrew ile

```bash
# Homebrew tap ekle
brew tap sentient-os/tap
brew install sentient

# Veya doğrudan
brew install --cask sentient
```

### 3.2 curl ile

```bash
curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.sh | bash
source ~/.zshrc
```

### 3.3 Apple Silicon (M1/M2/M3/M4) Notları

```bash
# Ollama Apple Silicon'de native çalışır (Metal加速)
ollama pull gemma3:27b

# MPS (Metal Performance Shaders) otomatik kullanılır
# Ekstra yapılandırma gerekmez
```

### 3.4 macOS Gereksinimleri

| Bileşen | Komut |
|---------|-------|
| Xcode CLI | `xcode-select --install` |
| Homebrew | `/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"` |
| Rust | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |

---

## 4. Windows Kurulumu

### 4.1 PowerShell ile (Önerilen)

```powershell
# Varsayılan kurulum
irm https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.ps1 | iex

# Belirli versiyon
irm https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.ps1 | iex -Version "v4.0.0"

# Özel dizin
irm ... | iex -Prefix "C:\Tools\Sentient"

# Kaldırma
irm ... | iex -Uninstall
```

### 4.2 WSL2 ile (Geliştiriciler için)

```bash
# WSL2 kur
wsl --install

# WSL2 içinde Ubuntu'yu aç
# Ardından Linux kurulumunu takip et
curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.sh | bash
```

### 4.3 Windows Gereksinimleri

| Bileşen | Sürüm | Kurulum |
|---------|-------|---------|
| Visual C++ Redistributable | 17+ | [İndir](https://aka.ms/vs/17/release/vc_redist.x64.exe) |
| PowerShell | 7+ | `winget install Microsoft.PowerShell` |
| Rust | 1.75+ | `winget install Rustlang.Rustup` |
| Git | 2.30+ | `winget install Git.Git` |

### 4.4 Windows Binary'leri

| Binary | Boyut | Açıklama |
|--------|-------|----------|
| `sentient.exe` | 41 MB | Ana CLI |
| `sentient-web.exe` | 12 MB | Web dashboard |
| `sentient-setup.exe` | 4.7 MB | Kurulum sihirbazı |
| `sentient-shell.exe` | 2.3 MB | Interactive shell |

### 4.5 Windows Cross-Compile

```bash
# Linux'tan Windows binary derle
rustup target add x86_64-pc-windows-gnu
apt-get install mingw-w64

cargo build --release --target x86_64-pc-windows-gnu \
  --bin sentient --bin sentient-shell --bin sentient-setup --bin sentient-web

# Binary'ler: target/x86_64-pc-windows-gnu/release/
```

**Not:** `sentient_python` (PyO3) Windows cross-compile için varsayılan kapalıdır.
Windows'ta Python entegrasyonu için native build gerekir.

---

## 5. Docker Kurulumu

### 5.1 Docker Compose (Önerilen)

```bash
# Tüm servisleri başlat
docker-compose up -d

# Minimal (DB + Cache + Vector)
docker-compose up -d postgres redis qdrant

# Logları izle
docker-compose logs -f sentient
```

**Docker Servisleri:**

| Servis | Port | İşlev | Zorunlu? |
|--------|------|-------|----------|
| PostgreSQL | 5432 | Ana veritabanı | ✅ |
| Redis | 6379 | Cache & Queue | ✅ |
| Qdrant | 6333 | Vector DB | ✅ RAG |
| MinIO | 9000/9001 | Object Storage | Opsiyonel |
| Prometheus | 9090 | Metrik | Opsiyonel |
| Grafana | 3001 | Dashboard | Opsiyonel |
| Ollama | 11434 | Lokal LLM | Lokal modda |
| SearXNG | 8888 | Arama motoru | Opsiyonel |
| RabbitMQ | 5672 | Message Queue | Opsiyonel |

### 5.2 Dockerfile

```bash
# Image oluştur
docker build -t sentient-os:latest .

# Container başlat (GPU)
docker run -d \
  --name sentient \
  --gpus all \
  -v ~/.sentient:/root/.sentient \
  -p 8080:8080 -p 8100:8100 -p 11434:11434 \
  sentient-os:latest

# GPU olmadan
docker run -d \
  --name sentient \
  -p 8080:8080 \
  -e LLM_PROVIDER=openrouter \
  -e OPENROUTER_API_KEY=sk-or-... \
  sentient-os:latest
```

### 5.3 Health Check

```bash
./scripts/health-check.sh
```

```
  PostgreSQL          ✅ SAĞLIKLI
  Redis               ✅ SAĞLIKLI
  Qdrant              ✅ SAĞLIKLI
  MinIO               ✅ SAĞLIKLI
  Ollama              ✅ SAĞLIKLI
  Prometheus          ✅ SAĞLIKLI
  Grafana             ✅ SAĞLIKLI
  SearXNG             ✅ SAĞLIKLI
  RabbitMQ            ✅ SAĞLIKLI
  Gateway             ✅ SAĞLIKLI
  
  10/10 servis sağlıklı
```

---

## 6. Kubernetes Dağıtımı

### 6.1 Namespace ve Deploy

```bash
# Namespace oluştur
kubectl apply -f deploy/kubernetes/namespace.yaml

# Tüm servisleri deploy et
kubectl apply -f deploy/kubernetes/

# Pod durumunu kontrol et
kubectl get pods -n sentient

# Logları izle
kubectl logs -f deployment/sentient-gateway -n sentient

# Service erişimi
kubectl port-forward svc/sentient-gateway 8080:8080 -n sentient
```

### 6.2 Helm Chart (Planlanan)

```bash
helm repo add sentient https://charts.sentient.ai
helm install sentient sentient/sentient-os \
  --set llm.provider=openrouter \
  --set llm.apiKey=sk-or-...
```

---

## 7. Kaynaktan Derleme

### 7.1 Bağımlılıklar

```bash
# Ubuntu/Debian
sudo apt update && sudo apt install -y \
    build-essential pkg-config libssl-dev \
    sqlite3 libsqlite3-dev git curl wget

# macOS
brew install openssl sqlite

# Fedora
sudo dnf install -y gcc openssl-devel sqlite-devel git
```

### 7.2 Rust Kurulumu

```bash
# Rust kur
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# Sürüm kontrol
rustc --version    # 1.75+ gerekli
cargo --version

# Gerekli bileşenler
rustup component add clippy rustfmt rust-analyzer
```

### 7.3 Projeyi Klonlama ve Derleme

```bash
# Klonla
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
cd SENTIENT_CORE

# Ortam değişkenleri
cp .env.example .env
# .env dosyasını düzenle

# Debug derleme (hızlı, geliştirme için)
cargo build

# Release derleme (optimizasyonlu, production için)
cargo build --release

# Belirli crate derleme
cargo build --release -p sentient_llm
cargo build --release -p sentient_orchestrator
cargo build --release -p sentient_gateway

# Tüm workspace'i kontrol et
cargo check --workspace
```

### 7.4 Binary'ler

```bash
# Derleme sonrası binary'ler
target/release/
├── sentient-shell       # Hybrid terminal + REPL
├── sentient-dashboard   # Web dashboard (Tauri)
├── sentient-ingest      # Skill ingestion
├── sentient-selfcoder   # Self-improvement loop
├── sentient-gateway     # API gateway + channels
├── sentient-setup       # Setup wizard
├── sentient-sync-daemon # Auto-update daemon
└── sentient-web         # Web server
```

### 7.5 Test

```bash
# Tüm testler
cargo test --workspace

# Belirli crate
cargo test -p sentient_llm
cargo test -p sentient_orchestrator
cargo test -p sentient_gateway

# Coverage
cargo tarpaulin --workspace --out Html
```

---

## 8. LLM Yapılandırması

### 8.1 Lokal LLM (Ücretsiz — Önerilen)

```bash
# Ollama kur
curl -fsSL https://ollama.com/install.sh | sh

# Model seç (VRAM'a göre)
ollama pull gemma3:27b       # 16GB VRAM — ÖNERİLEN
ollama pull llama3.2:3b      # 4GB VRAM — Hafif
ollama pull deepseek-r1:7b   # 8GB VRAM — Reasoning
ollama pull qwen3:30b-a3b    # 4GB VRAM MoE — Türkçe iyi
ollama pull mistral:7b       # 8GB VRAM — Dengeli

# .env
LLM_PROVIDER=ollama
OLLAMA_HOST=http://localhost:11434
OLLAMA_MODEL=gemma3:27b
```

### 8.2 Cloud LLM (API Key ile)

```bash
# OpenRouter (ÖNERİLEN — 200+ model, $5 ücretsiz kredi)
# https://openrouter.ai/keys
OPENROUTER_API_KEY=sk-or-v1-xxxxx

# OpenAI
OPENAI_API_KEY=sk-proj-xxxxx

# Anthropic
ANTHROPIC_API_KEY=sk-ant-xxxxx

# Google AI (Gemini Flash ücretsiz)
GOOGLE_AI_API_KEY=xxxxx

# DeepSeek (EN UCUZ)
DEEPSEEK_API_KEY=xxxxx

# Groq (EN HIZLI — ücretsiz tier mevcut)
GROQ_API_KEY=gsk_xxxxx
```

### 8.3 Hibrit Mod (Lokal + Cloud)

```bash
# .env
LLM_MODE=hybrid
LLM_LOCAL_MODEL=gemma3:27b          # Basit sorular → lokal
LLM_API_MODEL=openai/gpt-4o        # Karmaşık sorular → API
LLM_FALLBACK_THRESHOLD=0.7         # Zorluk eşiği
```

### 8.4 AI Gateway/Router ile

```bash
# Unify AI (ML bazlı akıllı routing)
UNIFY_API_KEY=xxx
# → "router@q>0.9&c<0.001" ile kalite+maliyet optimize

# Portkey (Enterprise gateway)
PORTKEY_API_KEY=xxx
PORTKEY_VIRTUAL_KEY=xxx

# LiteLLM (Self-hosted proxy)
pip install litellm[proxy]
litellm --config config.yaml --port 4000
```

---

## 9. Çapraz Derleme (Cross-Compile)

### 9.1 Hedef Platformlar

| Platform | Target | Durum |
|----------|--------|-------|
| Windows x86_64 (MSVC) | x86_64-pc-windows-msvc | ✅ |
| Windows x86_64 (GNU) | x86_64-pc-windows-gnu | ✅ |
| Linux x86_64 | x86_64-unknown-linux-gnu | ✅ |
| Linux ARM64 | aarch64-unknown-linux-gnu | ✅ |
| macOS Intel | x86_64-apple-darwin | ✅ |
| macOS Apple Silicon | aarch64-apple-darwin | ✅ |

### 9.2 Linux → Windows

```bash
rustup target add x86_64-pc-windows-gnu
sudo apt install mingw-w64

cargo build --release --target x86_64-pc-windows-gnu \
  --bin sentient --bin sentient-shell --bin sentient-setup --bin sentient-web

# Paketle
tar -czvf sentient-windows-x86_64.tar.gz \
  -C target/x86_64-pc-windows-gnu/release *.exe
```

### 9.3 Linux → ARM64

```bash
rustup target add aarch64-unknown-linux-gnu
sudo apt install gcc-aarch64-linux-gnu

cargo build --release --target aarch64-unknown-linux-gnu
```

### 9.4 Linux → macOS

```bash
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# NOT: macOS SDK gerektirir (osxcross)
```

---

## 10. GitHub Release Oluşturma

### 10.1 Tag Push ile

```bash
# Versiyon güncelle (Cargo.toml)
git add -A
git commit -m "chore: bump version to 4.0.1"
git tag v4.0.1
git push origin main --tags
```

### 10.2 GitHub Actions ile

```bash
# .github/workflows/release.yml otomatik çalışır
gh workflow run release.yml -f version=v4.0.1
```

### 10.3 Manuel Build + Upload

```bash
# Build
cargo build --release --target x86_64-pc-windows-gnu

# Paketle
tar -czvf sentient-windows-x86_64.tar.gz \
  -C target/x86_64-pc-windows-gnu/release *.exe

# Release oluştur
gh release create v4.0.1 \
  sentient-windows-x86_64.tar.gz \
  --title "SENTIENT OS v4.0.1" \
  --notes "Release notes..."
```

---

## 11. Sorun Giderme

### 11.1 Rust Derleme Hataları

```bash
# Rust güncelle
rustup update stable

# Temizle ve yeniden derle
cargo clean
cargo build --release

# Bağımlılık çakışması
cargo update
cargo build --release

# Belirli crate hatası
cargo build -p sentient_llm 2>&1 | less
```

### 11.2 Python (PyO3) Hataları

```bash
# PyO3 feature varsayılan kapalı — açmak için:
# Cargo.toml'da [features] altına ekle:
# python = ["sentient_python"]

# Virtual environment
python -m venv .venv
source .venv/bin/activate
pip install maturin pyo3
maturin develop
```

### 11.3 Ollama Bağlantı Hatası

```bash
# Servis çalışıyor mu?
systemctl status ollama
curl http://localhost:11434/api/tags

# Yeniden başlat
ollama serve &

# Model yüklü mü?
ollama list

# Tekrar indir
ollama pull gemma3:27b
```

### 11.4 Port Çakışması

```bash
# Hangi port kullanımda?
sudo lsof -i :8080
sudo lsof -i :11434

# İşlemi sonlandır
sudo kill -9 <PID>

# Veya portu değiştir (.env)
GATEWAY_PORT=8081
```

### 11.5 Windows VC++ Runtime Hatası

```powershell
# Visual C++ Redistributable kur
irm https://aka.ms/vs/17/release/vc_redist.x64.exe -OutFile vc_redist.exe
.\vc_redist.exe /install
```

### 11.6 macOS Gatekeeper Engeli

```bash
# İmzasız binary çalıştırma
xattr -cr /path/to/sentient

# Veya System Preferences > Security > Allow
```

### 11.7 Tam Sıfırlama

```bash
# Konfigürasyonu sıfırla
sentient config reset

# Belleği temizle
sentient memory clear

# Tam sıfırlama
rm -rf ~/.sentient
rm -rf data/
cargo clean

# Yeniden kur
./install.sh
```

---

## 12. Doğrulama

### 12.1 Sistem Kontrolü

```bash
sentient doctor
```

```
🧠 SENTIENT OS Doctor v4.0.0

✓ Rust: 1.80.0
✓ Binary: target/release/sentient
✓ Config: .env
✓ Database: data/sentient_memory.db
✓ LLM Provider: ollama (gemma3:27b)
✓ Voice: enabled (whisper_cpp + piper)
✓ Ollama: Connected
✓ Skills: 5587 loaded
✓ Tools: 43 available

Sistem hazır!
```

### 12.2 İlk Sohbet

```bash
sentient chat "Merhaba, kendini tanıtır mısın?"
```

### 12.3 Dil Testi

```bash
# Türkçe
sentient ask "Rust'ta ownership nedir?"

# English
sentient ask "What is ownership in Rust?"

# Kod üretimi
sentient code "Python fibonacci fonksiyonu yaz"
```

---

## 📊 Platform Desteği Özet

| Platform | Architect | Kurulum | Binary | Ollama | Docker |
|----------|-----------|---------|--------|--------|--------|
| Linux x86_64 | x86_64 | ✅ curl | ✅ | ✅ | ✅ |
| Linux ARM64 | aarch64 | ✅ curl | ✅ | ✅ | ✅ |
| macOS Intel | x86_64 | ✅ curl | ✅ | ✅ | ✅ |
| macOS Apple Silicon | aarch64 | ✅ curl | ✅ | ✅ Metal | ✅ |
| Windows x86_64 | x86_64 | ✅ PS1 | ✅ | ✅ WSL2 | ✅ WSL2 |
| Windows ARM64 | aarch64 | ✅ PS1 | ⏳ | ⏳ | ⏳ |

---

*Son Güncelleme: 2026-04-16*  
*Versiyon: 4.0.0*  
*GitHub: https://github.com/nexsusagent-coder/SENTIENT_CORE*
