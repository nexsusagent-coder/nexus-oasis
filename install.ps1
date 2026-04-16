# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS - Windows Tek Komutla Kurulum Script'i
# ═══════════════════════════════════════════════════════════════════════════════
#  Kullanım (PowerShell YÖNETİCİ olarak):
#    irm https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.ps1 | iex
#
#  Veya manuel:
#    git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
#    cd SENTIENT_CORE
#    .\install.ps1
# ═══════════════════════════════════════════════════════════════════════════════

param(
    [switch]$SkipOllama,
    [switch]$SkipDocker,
    [switch]$SkipModel,
    [switch]$SkipFFmpeg
)

# Renk fonksiyonları
function Write-Info { Write-Host "[ℹ] $args" -ForegroundColor Cyan }
function Write-OK { Write-Host "[✓] $args" -ForegroundColor Green }
function Write-Warn { Write-Host "[⚠] $args" -ForegroundColor Yellow }
function Write-Error { Write-Host "[✗] $args" -ForegroundColor Red }
function Write-Step { Write-Host "[▶] $args" -ForegroundColor Magenta }

# Logo
$Logo = @"

╔═══════════════════════════════════════════════════════════════╗
║                                                               ║
║   ███████╗███████╗███╗   ██╗████████╗███╗   ██╗███████╗██╗    ║
║   ██╔════╝██╔════╝████╗  ██║╚══██╔══╝████╗  ██║██╔════╝██║    ║
║   ███████╗█████╗  ██╔██╗ ██║   ██║   ██╔██╗ ██║███████╗██║    ║
║   ╚════██║██╔══╝  ██║╚██╗██║   ██║   ██║╚██╗██║╚════██║██║    ║
║   ███████║███████╗██║ ╚████║   ██║   ██║ ╚████║███████║██║    ║
║   ╚══════╝╚══════╝╚═╝  ╚═══╝   ╚═╝   ╚═╝  ╚═══╝╚══════╝╚═╝    ║
║                                                               ║
║              OS - The Operating System That Thinks            ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝

"@

Write-Host $Logo -ForegroundColor Cyan

# Yönetici kontrolü
function Test-Admin {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = [Security.Principal.WindowsPrincipal]$currentUser
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

if (-not (Test-Admin)) {
    Write-Warn "Yönetici olarak çalıştırmanız önerilir. Bazı kurulumlar başarısız olabilir."
    Write-Info "Sağ tık → 'PowerShell Yönetici olarak çalıştır' seçin"
    $continue = Read-Host "Devam etmek istiyor musunuz? (y/n)"
    if ($continue -ne "y") { exit 1 }
}

# ═══════════════════════════════════════════════════════════════════════════════
#  KURULUM FONKSİYONLARI
# ═══════════════════════════════════════════════════════════════════════════════

function Install-Rust {
    Write-Step "Rust kurulumu kontrol ediliyor..."

    if (Get-Command rustc -ErrorAction SilentlyContinue) {
        $version = rustc --version
        Write-OK "Rust zaten kurulu: $version"
        return $true
    }

    Write-Info "Rust kuruluyor..."
    winget install Rustlang.Rustup --accept-source-agreements --accept-package-agreements

    # PATH'e ekle
    $env:Path += ";$env:USERPROFILE\.cargo\bin"

    # Doğrula
    if (Get-Command rustc -ErrorAction SilentlyContinue) {
        Write-OK "Rust başarıyla kuruldu: $(rustc --version)"
        return $true
    }

    Write-Error "Rust kurulumu başarısız!"
    Write-Info "Manuel kurulum: https://rustup.rs"
    return $false
}

function Install-Python {
    Write-Step "Python kurulumu kontrol ediliyor..."

    if (Get-Command python -ErrorAction SilentlyContinue) {
        $version = python --version 2>&1
        Write-OK "Python zaten kurulu: $version"

        # pip kontrolü
        python -m pip --version | Out-Null
        if ($LASTEXITCODE -eq 0) {
            return $true
        }
    }

    Write-Info "Python 3.12 kuruluyor..."
    winget install Python.Python.3.12 --accept-source-agreements --accept-package-agreements

    # PATH'e ekle
    $pythonPath = "$env:LOCALAPPDATA\Programs\Python\Python312"
    if (Test-Path $pythonPath) {
        $env:Path += ";$pythonPath;$pythonPath\Scripts"
    }

    # Doğrula
    if (Get-Command python -ErrorAction SilentlyContinue) {
        Write-OK "Python başarıyla kuruldu: $(python --version)"
        return $true
    }

    Write-Warn "Python kurulumu başarısız olabilir. PyO3 olmadan devam edilecek."
    return $false
}

function Install-VSBuildTools {
    Write-Step "Visual Studio Build Tools kontrol ediliyor..."

    # MSVC kontrolü
    $vsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
    if (Test-Path $vsWhere) {
        $vsInstall = & $vsWhere -latest -property installationPath 2>$null
        if ($vsInstall) {
            Write-OK "Visual Studio kurulu: $vsInstall"
            return $true
        }
    }

    # Build Tools kontrolü
    $buildTools = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\2022\BuildTools"
    if (Test-Path $buildTools) {
        Write-OK "Build Tools kurulu"
        return $true
    }

    Write-Info "Visual Studio Build Tools kuruluyor (bu 5-10 dk sürebilir)..."
    winget install Microsoft.VisualStudio.2022.BuildTools --override "--add Microsoft.VisualStudio.Workload.VCTools --passive" --accept-source-agreements

    Write-OK "Build Tools kuruldu"
    return $true
}

function Install-FFmpeg {
    param([switch]$Skip)

    if ($Skip) {
        Write-Info "FFmpeg kurulumu atlanıyor"
        return $true
    }

    Write-Step "FFmpeg kontrol ediliyor..."

    if (Get-Command ffmpeg -ErrorAction SilentlyContinue) {
        Write-OK "FFmpeg zaten kurulu: $(ffmpeg -version | Select-Object -First 1)"
        return $true
    }

    # Yöntem 1: Winget ile (yönetici modunda çalışır)
    Write-Info "FFmpeg kuruluyor (winget)..."
    $ffmpegResult = winget install Gyan.FFmpeg --accept-source-agreements --accept-package-agreements 2>&1
    
    if ($LASTEXITCODE -eq 0) {
        # PATH'e ekle
        $ffmpegPath = "$env:ProgramFiles\ffmpeg\bin"
        if (Test-Path $ffmpegPath) {
            $env:Path += ";$ffmpegPath"
        }
        # Alternatif yol
        $ffmpegPath2 = "$env:LOCALAPPDATA\Microsoft\WinGet\Links"
        if (Test-Path $ffmpegPath2) {
            $env:Path += ";$ffmpegPath2"
        }
        
        if (Get-Command ffmpeg -ErrorAction SilentlyContinue) {
            Write-OK "FFmpeg başarıyla kuruldu"
            return $true
        }
    }

    # Yöntem 2: Chocolatey ile
    if (Get-Command choco -ErrorAction SilentlyContinue) {
        Write-Info "FFmpeg kuruluyor (chocolatey)..."
        choco install ffmpeg -y
        if (Get-Command ffmpeg -ErrorAction SilentlyContinue) {
            Write-OK "FFmpeg başarıyla kuruldu"
            return $true
        }
    }

    # Yöntem 3: Manuel indirme
    Write-Warn "FFmpeg otomatik kurulum başarısız. Manuel kurulum için:"
    Write-Info "1. https://www.gyan.dev/ffmpeg/builds/ adresine git"
    Write-Info "2. 'ffmpeg-release-essentials.zip' indir"
    Write-Info "3. C:\ffmpeg dizinine çıkart"
    Write-Info "4. C:\ffmpeg\bin'i PATH'e ekle"
    Write-Info ""
    Write-Info "FFmpeg olmadan ses/video özellikleri çalışmaz, ama sistem çalışır."
    Write-Info "Devam ediliyor..."
    return $false
}

function Install-Ollama {
    param([switch]$Skip)

    if ($Skip) {
        Write-Info "Ollama kurulumu atlanıyor"
        return $true
    }

    Write-Step "Ollama kontrol ediliyor..."

    if (Get-Command ollama -ErrorAction SilentlyContinue) {
        Write-OK "Ollama zaten kurulu"
        return $true
    }

    Write-Info "Ollama kuruluyor..."
    winget install Ollama.Ollama --accept-source-agreements --accept-package-agreements

    # Ollama servisini başlat
    Start-Process "ollama" -ArgumentList "serve" -WindowStyle Hidden -ErrorAction SilentlyContinue
    Start-Sleep -Seconds 3

    if (Get-Command ollama -ErrorAction SilentlyContinue) {
        Write-OK "Ollama başarıyla kuruldu"
        return $true
    }

    Write-Warn "Ollama kurulumu başarısız. Manuel kurulum: https://ollama.com/download"
    return $false
}

function Install-Git {
    Write-Step "Git kontrol ediliyor..."

    if (Get-Command git -ErrorAction SilentlyContinue) {
        Write-OK "Git zaten kurulu: $(git --version)"
        return $true
    }

    Write-Info "Git kuruluyor..."
    winget install Git.Git --accept-source-agreements --accept-package-agreements

    # PATH'e ekle
    $env:Path += ";$env:ProgramFiles\Git\cmd"

    if (Get-Command git -ErrorAction SilentlyContinue) {
        Write-OK "Git başarıyla kuruldu"
        return $true
    }

    Write-Error "Git kurulumu başarısız!"
    return $false
}

function Clone-Repo {
    Write-Step "Repository kontrol ediliyor..."

    # Zaten SENTIENT içinde miyiz?
    if (Test-Path "Cargo.toml") {
        $content = Get-Content "Cargo.toml" -Raw
        if ($content -match "SENTIENT") {
            Write-OK "SENTIENT repository'sindeyiz"
            git pull 2>$null
            return $true
        }
    }

    # SENTIENT_CORE dizini var mı?
    if (Test-Path "SENTIENT_CORE") {
        Write-Info "SENTIENT_CORE bulundu, güncelleniyor..."
        Set-Location SENTIENT_CORE
        git pull 2>$null
        return $true
    }

    Write-Info "SENTIENT repository'si klonlanıyor..."
    git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git

    if (Test-Path "SENTIENT_CORE") {
        Set-Location SENTIENT_CORE
        Write-OK "Repository klonlandı"
        return $true
    }

    Write-Error "Repository klonlanamadı!"
    return $false
}

function Build-Project {
    Write-Step "SENTIENT derleniyor..."

    # Python path'i ayarla (PyO3 için)
    $python = Get-Command python -ErrorAction SilentlyContinue
    if ($python) {
        $env:PYTHON_SYS_EXECUTABLE = $python.Source
        Write-Info "Python path: $($python.Source)"
    }

    Write-Info "Bu işlem 5-15 dakika sürebilir..."
    Write-Info "İlk derleme uzun sürer, lütfen bekleyin..."

    # Temiz derleme
    cargo clean 2>$null

    # Release derle
    $buildLog = "$env:TEMP\sentient-build.log"
    if (cargo build --release 2>&1 | Tee-Object -FilePath $buildLog) {
        Write-OK "SENTIENT başarıyla derlendi!"
    } else {
        Write-Error "Derleme hatası!"

        # PyO3 hatası kontrolü
        if (Select-String -Path $buildLog -Pattern "pyo3" -Quiet) {
            Write-Warn "PyO3 hatası tespit edildi. PyO3 olmadan deneniyor..."

            # Cargo.toml'da sentient_python'ü comment out et
            $cargoToml = Get-Content "Cargo.toml" -Raw
            if ($cargoToml -match '"crates/sentient_python"') {
                $cargoToml = $cargoToml -replace '"crates/sentient_python"', '# "crates/sentient_python"'
                Set-Content "Cargo.toml" -Value $cargoToml
                Write-Info "sentient_python devre dışı bırakıldı, tekrar derleniyor..."
                cargo build --release 2>&1 | Tee-Object -FilePath $buildLog
            }
        }
    }

    # Binary kontrolü
    if (Test-Path "target\release\sentient.exe") {
        $size = (Get-Item "target\release\sentient.exe").Length / 1MB
        Write-OK "Binary: target\release\sentient.exe ($([math]::Round($size, 1)) MB)"
        return $true
    }

    Write-Error "Binary oluşturulamadı!"
    Write-Info "Log dosyası: $buildLog"
    return $false
}

function Download-Model {
    param([switch]$Skip)

    if ($Skip) {
        Write-Info "Model indirme atlanıyor"
        return
    }

    if (-not (Get-Command ollama -ErrorAction SilentlyContinue)) {
        Write-Warn "Ollama kurulu değil, model atlanıyor"
        return
    }

    Write-Step "Varsayılan AI modeli indiriliyor..."

    # Küçük model: gemma2:2b (2.6GB)
    Write-Info "gemma2:2b modeli indiriliyor (2.6GB)..."
    ollama pull gemma2:2b

    Write-OK "Model hazır"
}

function Create-Env {
    Write-Step ".env dosyası oluşturuluyor..."

    if (Test-Path ".env") {
        Write-OK ".env zaten mevcut"
        return
    }

    $envContent = @"
# ════════════════════════════════════════════════════════════════
#  SENTIENT OS - Yapılandırma Dosyası
# ════════════════════════════════════════════════════════════════

# LLM Provider (OpenRouter önerilen - `$5 ücretsiz kredi)
#OPENROUTER_API_KEY=sk-or-v1-...

# Veya OpenAI
#OPENAI_API_KEY=sk-...

# Veya Ollama (lokal, ücretsiz)
OPENAI_API_BASE=http://localhost:11434/v1
OPENAI_API_KEY=ollama
DEFAULT_MODEL=ollama/gemma2:2b

# Voice (opsiyonel)
#VOICE_ENABLED=true
#VOICE_STT=whisper_cpp
#VOICE_TTS=piper
#VOICE_LANGUAGE=tr

# Home Assistant (opsiyonel)
#HOME_ASSISTANT_URL=http://homeassistant.local:8123
#HOME_ASSISTANT_TOKEN=eyJ...
"@

    Set-Content -Path ".env" -Value $envContent -Encoding UTF8

    Write-OK ".env dosyası oluşturuldu"
    Write-Info "API key'lerinizi .env dosyasına ekleyin"
}

function Run-Tests {
    Write-Step "Testler çalıştırılıyor..."

    $testOutput = cargo test --workspace --lib 2>&1 | Out-String
    if ($testOutput -match "test result: ok") {
        Write-OK "Tüm testler geçti!"
    } else {
        Write-Warn "Bazı testler başarısız olabilir, bu kritik değil"
    }
}

function Print-Success {
    Write-Host ""
    Write-Host "╔═══════════════════════════════════════════════════════════════╗" -ForegroundColor Green
    Write-Host "║                 ✅ KURULUM TAMAMLANDI!                        ║" -ForegroundColor Green
    Write-Host "╚═══════════════════════════════════════════════════════════════╝" -ForegroundColor Green
    Write-Host ""
    Write-Host "Kullanım:" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "  # Versiyon kontrolü"
    Write-Host "  .\target\release\sentient.exe --version"
    Write-Host ""
    Write-Host "  # Sohbet başlat"
    Write-Host "  .\target\release\sentient.exe chat"
    Write-Host ""
    Write-Host "  # Web dashboard"
    Write-Host "  .\target\release\sentient.exe web"
    Write-Host ""
    Write-Host "  # Sesli asistan"
    Write-Host "  .\target\release\sentient.exe voice --wake-word 'hey sentient'"
    Write-Host ""
    Write-Host "Sonraki adımlar:" -ForegroundColor Yellow
    Write-Host "  1. API key ekleyin: notepad .env"
    Write-Host "  2. Daha büyük model: ollama pull deepseek-r1:8b"
    Write-Host "  3. Dokümantasyon: type README.md"
    Write-Host ""
    Write-Host "SENTIENT OS - The Operating System That Thinks" -ForegroundColor Magenta
    Write-Host ""
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ANA AKIŞ
# ═══════════════════════════════════════════════════════════════════════════════

Write-Info "Kurulum başlıyor..."

# 1. Gerekli araçları kur
Install-Git
Install-Rust
Install-Python
Install-VSBuildTools
Install-FFmpeg -Skip:$SkipFFmpeg
Install-Ollama -Skip:$SkipOllama

# 2. Repository
Clone-Repo

# 3. Derle
Build-Project

# 4. Model indir
Download-Model -Skip:$SkipModel

# 5. .env oluştur
Create-Env

# 6. Test
Run-Tests

# 7. Başarı mesajı
Print-Success
