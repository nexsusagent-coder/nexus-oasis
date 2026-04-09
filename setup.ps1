# ═══════════════════════════════════════════════════════════════════════════════
#  🧠 SENTIENT OS - WINDOWS POWERSHELL SETUP SCRIPT
#  Version: 2.1.0 | Kernel: Gemma 4 31B
# ═══════════════════════════════════════════════════════════════════════════════
#
#  Kullanım:
#    powershell -ExecutionPolicy ByPass -File setup.ps1
#    veya
#    .\setup.ps1 -ExecutionPolicy ByPass
#
# ═══════════════════════════════════════════════════════════════════════════════

param(
    [string]$Command = "all",
    [switch]$Help,
    [switch]$Force,
    [switch]$NoDocker,
    [switch]$NoPython
)

# Hata durumunda dur
$ErrorActionPreference = "Stop"

# Renk fonksiyonları
function Write-ColorOutput {
    param([string]$Message, [string]$Color = "White")
    Write-Host $Message -ForegroundColor $Color
}

function Write-Success { param([string]$Message) Write-ColorOutput "✅ $Message" "Green" }
function Write-Info { param([string]$Message) Write-ColorOutput "ℹ️  $Message" "Cyan" }
function Write-Warn { param([string]$Message) Write-ColorOutput "⚠️  $Message" "Yellow" }
function Write-Error { param([string]$Message) Write-ColorOutput "❌ $Message" "Red"; exit 1 }
function Write-Step { param([string]$Message) Write-ColorOutput "`n🔹 $Message" "Magenta" }

# Logo
function Print-Logo {
    Write-ColorOutput @"
╔═══════════════════════════════════════════════════════════════════╗
║                                                                   ║
║   🧠 SENTIENT OS - The Operating System That Thinks               ║
║   🦀 Rust Core │ 5587 Skills │ 71 Integrations                    ║
║                                                                   ║
║   Version: 2.1.0                                                  ║
║   Platform: Windows PowerShell                                    ║
║                                                                   ║
╚═══════════════════════════════════════════════════════════════════╝
"@ "Cyan"
}

# Yönetici kontrolü
function Test-Administrator {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

# Sistem kontrolü
function Check-System {
    Write-Step "Sistem kontrol ediliyor..."
    
    $os = Get-CimInstance -ClassName Win32_OperatingSystem
    Write-Info "OS: $($os.Caption) $($os.Version)"
    Write-Info "Architecture: $env:PROCESSOR_ARCHITECTURE"
    
    # RAM kontrolü
    $totalRAM = [math]::Round($os.TotalVisibleMemorySize / 1MB, 2)
    if ($totalRAM -lt 8) {
        Write-Warn "RAM: ${totalRAM}GB (Minimum 8GB önerilir)"
    } else {
        Write-Success "RAM: ${totalRAM}GB"
    }
    
    # Disk kontrolü
    $disk = Get-CimInstance -ClassName Win32_LogicalDisk -Filter "DeviceID='C:'"
    $freeSpace = [math]::Round($disk.FreeSpace / 1GB, 2)
    if ($freeSpace -lt 20) {
        Write-Warn "Disk: ${freeSpace}GB boş (Minimum 20GB önerilir)"
    } else {
        Write-Success "Disk: ${freeSpace}GB boş"
    }
}

# Chocolatey kurulumu
function Install-Chocolatey {
    Write-Step "Chocolatey paket yöneticisi kontrol ediliyor..."
    
    if (Get-Command choco -ErrorAction SilentlyContinue) {
        Write-Success "Chocolatey mevcut: $(choco --version)"
        return
    }
    
    Write-Info "Chocolatey kuruluyor..."
    Set-ExecutionPolicy Bypass -Scope Process -Force
    [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
    Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
    
    # PATH güncelle
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
    
    Write-Success "Chocolatey kuruldu!"
}

# Rust kurulumu
function Install-Rust {
    Write-Step "Rust kontrol ediliyor..."
    
    if (Get-Command rustc -ErrorAction SilentlyContinue) {
        $rustVersion = rustc --version
        Write-Success "Rust mevcut: $rustVersion"
        return
    }
    
    Write-Info "Rust kuruluyor..."
    
    # rustup-init.exe indir
    $rustupUrl = "https://win.rustup.rs/x86_64"
    $rustupPath = "$env:TEMP\rustup-init.exe"
    
    Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupPath -UseBasicParsing
    
    # Sessiz kurulum
    Start-Process -FilePath $rustupPath -ArgumentList "-y" -Wait -NoNewWindow
    
    # PATH güncelle
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
    $env:Path += ";$env:USERPROFILE\.cargo\bin"
    
    # Component'ler
    rustup default stable
    rustup component add clippy rustfmt rust-analyzer
    
    Write-Success "Rust kuruldu: $(rustc --version)"
}

# Visual Studio Build Tools (Rust için gerekli)
function Install-BuildTools {
    Write-Step "Visual Studio Build Tools kontrol ediliyor..."
    
    # MSVC kontrolü
    $vsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
    
    if (Test-Path $vsWhere) {
        $vsInstall = & $vsWhere -latest -property installationPath 2>$null
        if ($vsInstall) {
            Write-Success "Visual Studio mevcut: $vsInstall"
            return
        }
    }
    
    Write-Info "Visual Studio Build Tools kuruluyor (bu işlem birkaç dakika sürebilir)..."
    
    # Chocolatey ile kur
    choco install visualstudio2022buildtools -y --package-parameters "--add Microsoft.VisualStudio.Workload.VCTools --includeRecommended --passive --locale en-US"
    
    Write-Success "Visual Studio Build Tools kuruldu!"
}

# Git kurulumu
function Install-Git {
    Write-Step "Git kontrol ediliyor..."
    
    if (Get-Command git -ErrorAction SilentlyContinue) {
        Write-Success "Git mevcut: $(git --version)"
        return
    }
    
    Write-Info "Git kuruluyor..."
    choco install git -y
    
    # PATH güncelle
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
    
    Write-Success "Git kuruldu!"
}

# Python kurulumu
function Install-Python {
    if ($NoPython) {
        Write-Info "Python kurulumu atlanıyor (-NoPython)"
        return
    }
    
    Write-Step "Python kontrol ediliyor..."
    
    $pythonCmd = Get-Command python -ErrorAction SilentlyContinue
    if ($pythonCmd) {
        $pythonVersion = & python --version 2>&1
        Write-Success "Python mevcut: $pythonVersion"
        return
    }
    
    Write-Info "Python kuruluyor..."
    choco install python -y
    
    # PATH güncelle
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
    
    # Pip yükselt
    python -m pip install --upgrade pip
    
    Write-Success "Python kuruldu!"
}

# SQLite kurulumu
function Install-SQLite {
    Write-Step "SQLite kontrol ediliyor..."
    
    if (Get-Command sqlite3 -ErrorAction SilentlyContinue) {
        Write-Success "SQLite mevcut"
        return
    }
    
    Write-Info "SQLite kuruluyor..."
    choco install sqlite -y
    
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
    
    Write-Success "SQLite kuruldu!"
}

# Docker Desktop kurulumu
function Install-Docker {
    if ($NoDocker) {
        Write-Info "Docker kurulumu atlanıyor (-NoDocker)"
        return
    }
    
    Write-Step "Docker kontrol ediliyor..."
    
    if (Get-Command docker -ErrorAction SilentlyContinue) {
        Write-Success "Docker mevcut: $(docker --version)"
        return
    }
    
    Write-Info "Docker Desktop kuruluyor..."
    
    # Docker Desktop indir ve kur
    $dockerUrl = "https://desktop.docker.com/win/main/amd64/Docker%20Desktop%20Installer.exe"
    $dockerPath = "$env:TEMP\DockerDesktopInstaller.exe"
    
    Invoke-WebRequest -Uri $dockerUrl -OutFile $dockerPath -UseBasicParsing
    Start-Process -FilePath $dockerPath -ArgumentList "install", "--quiet" -Wait
    
    Write-Success "Docker Desktop kuruldu!"
    Write-Warn "Docker'ı başlatmak için bilgisayarı yeniden başlatmanız gerekebilir."
}

# Ollama kurulumu (Gemma 4 Kernel)
function Install-Ollama {
    Write-Step "Ollama (Gemma 4 Kernel) kontrol ediliyor..."
    
    if (Get-Command ollama -ErrorAction SilentlyContinue) {
        Write-Success "Ollama mevcut"
        
        # Model kontrolü
        $models = ollama list 2>$null
        if ($models -match "gemma4") {
            Write-Success "Gemma 4 modeli zaten yüklü"
            return
        }
    }
    
    Write-Info "Ollama kuruluyor..."
    
    # Ollama indir
    $ollamaUrl = "https://ollama.com/download/OllamaSetup.exe"
    $ollamaPath = "$env:TEMP\OllamaSetup.exe"
    
    Invoke-WebRequest -Uri $ollamaUrl -OutFile $ollamaPath -UseBasicParsing
    Start-Process -FilePath $ollamaPath -ArgumentList "/S" -Wait
    
    # PATH güncelle
    $env:Path += ";$env:LOCALAPPDATA\Programs\Ollama"
    
    Write-Success "Ollama kuruldu!"
    
    # Gemma 4 model seçimi
    Write-ColorOutput "`n🎨 Hangi Gemma 4 modelini indirmek istiyorsunuz?" "Cyan"
    Write-Host "  1) gemma4:31b (ÖNERİLEN - 256K context, ~20GB)"
    Write-Host "  2) gemma4:12b (Orta - 128K context, ~8GB)"
    Write-Host "  3) gemma4:4b  (Minimum - 64K context, ~3GB)"
    Write-Host "  4) Daha sonra indir"
    
    $choice = Read-Host "Seçiminiz [1-4]"
    
    switch ($choice) {
        "1" { 
            Write-Info "Gemma 4 31B indiriliyor (~20GB)..."
            ollama pull gemma4:31b 
        }
        "2" { 
            Write-Info "Gemma 4 12B indiriliyor (~8GB)..."
            ollama pull gemma4:12b 
        }
        "3" { 
            Write-Info "Gemma 4 4B indiriliyor (~3GB)..."
            ollama pull gemma4:4b 
        }
        default { 
            Write-Warn "Model daha sonra indirilebilir: ollama pull gemma4:31b" 
        }
    }
}

# Projeyi klonla
function Clone-Project {
    Write-Step "SENTIENT OS projesi kontrol ediliyor..."
    
    if (Test-Path "SENTIENT_CORE") {
        Write-Info "SENTIENT_CORE dizini mevcut"
        if (Test-Path "SENTIENT_CORE\.git") {
            Write-Info "Güncelleniyor..."
            Set-Location SENTIENT_CORE
            git pull origin main
            Set-Location ..
        }
        return
    }
    
    Write-Info "Proje klonlanıyor..."
    git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
    
    Write-Success "Proje klonlandı!"
}

# .env dosyası oluştur
function Setup-Env {
    Write-Step ".env yapılandırması oluşturuluyor..."
    
    Set-Location SENTIENT_CORE
    
    if (Test-Path ".env") {
        Write-Success ".env dosyası mevcut"
        Set-Location ..
        return
    }
    
    # Varsayılan .env
    $envContent = @"
# ═══════════════════════════════════════════════════════════
#  GEMMA 4 KERNEL - YEREL LLM (API KEY GEREKMİYOR!)
# ═══════════════════════════════════════════════════════════
GEMMA4_MODEL=gemma4:31b
GEMMA4_BASE_URL=http://localhost:11434/v1
GEMMA4_CONTEXT_LENGTH=262144
GEMMA4_THINKING_MODE=true

# ═══════════════════════════════════════════════════════════
#  V-GATE (API ANAHTARI YÖNETİMİ)
# ═══════════════════════════════════════════════════════════
V_GATE_URL=http://localhost:8100
V_GATE_LISTEN=127.0.0.1:1071
V_GATE_TIMEOUT=120

# ═══════════════════════════════════════════════════════════
#  GATEWAY (API SUNUCUSU)
# ═══════════════════════════════════════════════════════════
GATEWAY_HTTP_ADDR=0.0.0.0:8080
GATEWAY_PORT=8080
JWT_SECRET=change-this-secret-in-production

# ═══════════════════════════════════════════════════════════
#  BELLEK (MEMORY CUBE)
# ═══════════════════════════════════════════════════════════
MEMORY_DB_PATH=data/sentient.db
MEMORY_SHORT_TTL=3600
MEMORY_LONG_TTL=0
ZERO_COPY_ENABLED=true

# ═══════════════════════════════════════════════════════════
#  OASIS BRAIN (OTONOM DÜŞÜNCE)
# ═══════════════════════════════════════════════════════════
OASIS_BRAIN_MODEL=gemma4:31b
OASIS_BRAIN_THINKING=true
OASIS_BRAIN_MAX_STEPS=10

# ═══════════════════════════════════════════════════════════
#  LOGGING
# ═══════════════════════════════════════════════════════════
RUST_LOG=info
LOG_FILE=logs/sentient.log
"@
    
    $envContent | Out-File -FilePath ".env" -Encoding UTF8
    
    Write-Success ".env dosyası oluşturuldu!"
    Set-Location ..
}

# Projeyi derle
function Build-Project {
    Write-Step "SENTIENT OS derleniyor..."
    
    Set-Location SENTIENT_CORE
    
    # PATH'i güncelle (cargo için)
    $env:Path += ";$env:USERPROFILE\.cargo\bin"
    
    Write-Info "Cargo check..."
    cargo check
    
    Write-Info "Release derleme (ilk derleme 10-15 dakika sürebilir)..."
    cargo build --release
    
    Write-Success "Derleme tamamlandı!"
    Set-Location ..
}

# Skill library
function Setup-Skills {
    Write-Step "Skill Library hazırlanıyor..."
    
    Set-Location SENTIENT_CORE
    
    # Dizin oluştur
    New-Item -ItemType Directory -Force -Path "data\skills" | Out-Null
    
    # Skill ingest
    if (Test-Path "target\release\sentient-ingest.exe") {
        .\target\release\sentient-ingest.exe full
    } else {
        cargo run --release --bin sentient-ingest -- full 2>$null
    }
    
    # İstatistik
    $skillCount = (Get-ChildItem -Path "data\skills" -Filter "*.yaml" -Recurse -ErrorAction SilentlyContinue).Count
    Write-Success "Skill Library: $skillCount skill yüklendi!"
    
    Set-Location ..
}

# Test çalıştır
function Run-Tests {
    Write-Step "Testler çalıştırılıyor..."
    
    Set-Location SENTIENT_CORE
    
    cargo test --release --workspace 2>$null
    
    Write-Success "Testler tamamlandı!"
    Set-Location ..
}

# Final rapor
function Show-Result {
    Write-ColorOutput @"

╔═══════════════════════════════════════════════════════════════════╗
║                                                                   ║
║   🎉 SENTIENT OS KURULUMU TAMAMLANDI!                             ║
║                                                                   ║
╠═══════════════════════════════════════════════════════════════════╣
║                                                                   ║
║   📋 SONRAKI ADIMLAR:                                             ║
║                                                                   ║
║   1. Proje dizinine git:                                          ║
║      cd SENTIENT_CORE                                             ║
║                                                                   ║
║   2. SENTIENT'ı başlat:                                           ║
║      .\target\release\sentient-shell.exe                          ║
║      veya                                                         ║
║      cargo run --release --bin sentient                           ║
║                                                                   ║
║   3. Dashboard'ı başlat:                                          ║
║      .\target\release\sentient-dashboard.exe                      ║
║      Tarayıcıda: http://localhost:8080                            ║
║                                                                   ║
║   4. Gemma 4 ile sohbet:                                          ║
║      ollama run gemma4:31b                                        ║
║                                                                   ║
║   5. .env dosyasını düzenle (API anahtarları için):               ║
║      notepad .env                                                 ║
║                                                                   ║
╠═══════════════════════════════════════════════════════════════════╣
║                                                                   ║
║   🧠 SENTIENT OS - The Operating System That Thinks               ║
║                                                                   ║
╚═══════════════════════════════════════════════════════════════════╝
"@ "Green"
}

# Kullanım yardımı
function Show-Help {
    Write-ColorOutput @"
Kullanım: .\setup.ps1 [komut] [seçenekler]

Komutlar:
  all        - Tüm kurulum (varsayılan)
  rust       - Sadece Rust kurulumu
  deps       - Sadece bağımlılıklar
  build      - Sadece derleme
  skills     - Sadece skill kurulumu
  docker     - Sadece Docker kurulumu
  test       - Sadece testler
  clean      - Temizlik
  help       - Bu yardım

Seçenekler:
  -NoDocker   Docker kurulumunu atla
  -NoPython   Python kurulumunu atla
  -Force      Zorla yeniden kur

Örnekler:
  .\setup.ps1                          # Tam kurulum
  .\setup.ps1 -NoDocker               # Docker olmadan
  .\setup.ps1 build                   # Sadece derleme
  .\setup.ps1 clean                   # Temizlik

"@ "Cyan"
}

# Ana kurulum
function Main {
    Print-Logo
    
    # Yönetici kontrolü
    if (-not (Test-Administrator)) {
        Write-Warn "Yönetici olarak çalıştırmanız önerilir."
        Write-Info "Sağ tık -> 'Run as Administrator' ile çalıştırın."
        Write-Host ""
    }
    
    Write-ColorOutput "Bu script SENTIENT'nın tüm bileşenlerini kuracaktır." "Yellow"
    Write-ColorOutput "Devam etmek için Enter'a basın..." "Yellow"
    Read-Host
    
    # Adımlar
    Check-System
    Install-Chocolatey
    Install-BuildTools
    Install-Git
    Install-Rust
    Install-Python
    Install-SQLite
    Install-Docker
    Install-Ollama
    Clone-Project
    Setup-Env
    Build-Project
    Setup-Skills
    Run-Tests
    
    # Final
    Show-Result
}

# Komut işleme
switch ($Command.ToLower()) {
    "all" { Main }
    "rust" { Install-Rust }
    "deps" { 
        Install-Chocolatey
        Install-BuildTools
        Install-Git
        Install-Python
        Install-SQLite
    }
    "build" { Build-Project }
    "skills" { Setup-Skills }
    "docker" { Install-Docker }
    "test" { Run-Tests }
    "clean" {
        Set-Location SENTIENT_CORE
        cargo clean
        if (Test-Path "venv") { Remove-Item -Recurse -Force "venv" }
        if (Test-Path "node_modules") { Remove-Item -Recurse -Force "node_modules" }
        Set-Location ..
        Write-Info "Temizlik yapıldı!"
    }
    "help" { Show-Help }
    default { 
        Write-Error "Bilinmeyen komut: $Command"
        Show-Help
    }
}
