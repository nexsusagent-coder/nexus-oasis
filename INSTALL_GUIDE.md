# SENTIENT OS - Windows Kurulum Rehberi

## Tarih: 2026-04-16

---

## Özet

SENTIENT OS, Windows dahil tüm işletim sistemlerinde **tek komutla** kurulabilir hale getirildi.

---

## Yapılan İşlemler

### 1. Windows Cross-Compile Desteği

**Sorun:** PyO3 (Python binding) Windows cross-compile için ekstra setup gerektiriyordu.

**Çözüm:** `sentient_python` optional feature'ı varsayılan olarak kapatıldı.

```toml
# crates/sentient_core/Cargo.toml
[features]
default = []  # Python feature kaldırıldı
python = ["sentient_python"]  # Optional olarak kaldı
```

**Derleme komutu:**
```bash
cargo build --release --target x86_64-pc-windows-gnu \
  --bin sentient --bin sentient-shell --bin sentient-setup --bin sentient-web \
  --bin sentient-selfcoder --bin sentient-sync-daemon --bin sentient-ingest
```

**Gereken araçlar:**
```bash
rustup target add x86_64-pc-windows-gnu
apt-get install mingw-w64
```

---

### 2. Derlenen Windows Binary'leri

| Binary | Boyut | Açıklama |
|--------|-------|----------|
| `sentient.exe` | 41 MB | Ana CLI |
| `sentient-web.exe` | 12 MB | Web dashboard |
| `sentient-setup.exe` | 4.7 MB | Kurulum sihirbazı |
| `sentient-shell.exe` | 2.3 MB | Interactive shell |

**Konum:** `target/x86_64-pc-windows-gnu/release/`

---

### 3. GitHub Release Workflow

**Dosya:** `.github/workflows/release.yml`

**Desteklenen Platformlar:**
- Linux x86_64
- Linux ARM64
- macOS Intel
- macOS Apple Silicon
- Windows x86_64

**Tetikleme:**
```bash
# Tag push ile
git tag v4.0.1 && git push --tags

# Veya manuel
gh workflow run release.yml -f version=v4.0.1
```

---

### 4. Universal Install Script (Linux/macOS)

**Dosya:** `install.sh`

**Kullanım:**
```bash
# Varsayılan kurulum
curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.sh | sh

# Belirli versiyon
curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.sh | sh -s -- --version v4.0.0

# Özel dizin
curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.sh | sh -s -- --prefix /opt/sentient

# Kaldırma
curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.sh | sh -s -- --uninstall
```

**Özellikleri:**
- Platform otomatik algılama (x86_64, ARM64)
- Bağımlılık kontrolü (curl, tar)
- Opsiyonel bağımlılıklar için uyarı (Python, Ollama)
- PATH otomatik yapılandırma
- Shell rc dosyasına ekleme (bash, zsh, fish)

---

### 5. Universal Install Script (Windows)

**Dosya:** `install.ps1`

**Kullanım:**
```powershell
# Varsayılan kurulum
irm https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.ps1 | iex

# Belirli versiyon
irm https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.ps1 | iex -Version "v4.0.0"

# Özel dizin
irm https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.ps1 | iex -Prefix "C:\Tools\Sentient"

# Kaldırma
irm https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.ps1 | iex -Uninstall
```

**Özellikleri:**
- Architecture otomatik algılama
- Visual C++ Redistributable kontrolü
- Python kontrolü (opsiyonel)
- PATH ve SENTIENT_HOME ayarlama
- İlerleme çubuğu (indirme)

---

### 6. GitHub Release

**Release URL:** https://github.com/nexsusagent-coder/SENTIENT_CORE/releases/tag/v4.0.0

**Oluşturma komutu:**
```bash
gh release create v4.0.0 \
  --title "SENTIENT OS v4.0.0 - Universal Installation" \
  --notes "Release notes..." \
  target/x86_64-pc-windows-gnu/release/sentient-windows-x86_64.tar.gz
```

---

## Kurulum Talimatları

### Windows

```powershell
# PowerShell'de çalıştır
irm https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.ps1 | iex
```

Kurulum sonrası:
```powershell
sentient setup    # Kurulum sihirbazı
sentient          # Interactive REPL
sentient-web      # Web dashboard (http://localhost:3000)
```

### Linux

```bash
curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.sh | sh
source ~/.bashrc  # veya ~/.zshrc
sentient setup
```

### macOS

```bash
curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.sh | sh
source ~/.zshrc
sentient setup
```

---

## Dosya Yapısı

```
SENTIENT_CORE/
├── .github/
│   └── workflows/
│       └── release.yml      # GitHub Actions release pipeline
├── crates/
│   └── sentient_core/
│       └── Cargo.toml       # Python feature optional
├── install.sh               # Linux/macOS installer
├── install.ps1              # Windows installer
└── target/
    └── x86_64-pc-windows-gnu/
        └── release/
            ├── sentient.exe
            ├── sentient-shell.exe
            ├── sentient-setup.exe
            ├── sentient-web.exe
            └── sentient-windows-x86_64.tar.gz
```

---

## Yeni Release Oluşturma

### Yöntem 1: Tag Push
```bash
# Versiyon numarasını güncelle
# Cargo.toml'de version = "4.0.1"

git add -A
git commit -m "chore: bump version to 4.0.1"
git tag v4.0.1
git push origin main --tags
```

### Yöntem 2: Manuel Workflow
```bash
gh workflow run release.yml -f version=v4.0.1
```

### Yöntem 3: Local Build + Upload
```bash
# Build
cargo build --release --target x86_64-pc-windows-gnu

# Package
tar -czvf sentient-windows-x86_64.tar.gz -C target/x86_64-pc-windows-gnu/release *.exe

# Create release
gh release create v4.0.1 sentient-windows-x86_64.tar.gz
```

---

## Bilinen Sorunlar

### 1. Python Entegrasyonu
Python feature şu anda varsayılan kapalı. Windows'ta Python entegrasyonu için:
- Windows'ta native build gerekir (Rust + Python 3.11+)
- Veya Python'u runtime'da subprocess ile çağırabilir

### 2. Visual C++ Redistributable
Windows'ta bazı binary'ler VC++ runtime gerektirebilir.
- İndirme: https://aka.ms/vs/17/release/vc_redist.x64.exe

---

## Platform Desteği

| Platform | Target | Durum |
|----------|--------|-------|
| Windows x86_64 | x86_64-pc-windows-msvc | ✅ |
| Windows x86_64 (GNU) | x86_64-pc-windows-gnu | ✅ |
| Linux x86_64 | x86_64-unknown-linux-gnu | ⏳ CI'da |
| Linux ARM64 | aarch64-unknown-linux-gnu | ⏳ CI'da |
| macOS Intel | x86_64-apple-darwin | ⏳ CI'da |
| macOS Apple Silicon | aarch64-apple-darwin | ⏳ CI'da |

---

## Commit Log

```
61b0b35 fix: Windows cross-compile support, universal install scripts
├── .github/workflows/release.yml   # Multi-platform release pipeline
├── crates/sentient_core/Cargo.toml # Python feature disabled by default
├── install.ps1                     # Windows universal installer
└── install.sh                      # Linux/macOS universal installer
```

---

## İletişim

- GitHub: https://github.com/nexsusagent-coder/SENTIENT_CORE
- Issues: https://github.com/nexsusagent-coder/SENTIENT_CORE/issues

---

*Bu dosya 2026-04-16 tarihinde oluşturuldu.*
