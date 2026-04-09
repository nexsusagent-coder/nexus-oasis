# ═══════════════════════════════════════════════════════════════════════════════
#  🧠 SENTIENT OS - The Operating System That Thinks
#  Interactive Setup Script v5.0.0 (Windows)
# ═══════════════════════════════════════════════════════════════════════════════

param(
    [switch]$Silent,
    [string]$Model,
    [string]$Provider
)

# Colors
function Write-ColorText {
    param([string]$Text, [string]$Color = "White")
    Write-Host $Text -ForegroundColor $Color
}

function Write-Step {
    param([int]$Step, [int]$Total, [string]$Title)
    Write-Host ""
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Magenta
    Write-Host "  ADIM $Step/$Total : $Title" -ForegroundColor Cyan
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Magenta
    Write-Host ""
}

function Write-Success {
    param([string]$Text)
    Write-Host "✓ $Text" -ForegroundColor Green
}

function Write-Warning {
    param([string]$Text)
    Write-Host "⚠ $Text" -ForegroundColor Yellow
}

function Write-Error {
    param([string]$Text)
    Write-Host "✗ $Text" -ForegroundColor Red
}

# Header
function Show-Header {
    Clear-Host
    Write-Host ""
    Write-Host "    ╔═══════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "    ║                                                               ║" -ForegroundColor Cyan
    Write-Host "    ║     ███████╗███████╗███╗   ██╗████████╗██╗ ██████╗ █████╗     ║" -ForegroundColor Cyan
    Write-Host "    ║     ██╔════╝██╔════╝████╗  ██║╚══██╔══╝██║██╔════╝██╔══██╗    ║" -ForegroundColor Cyan
    Write-Host "    ║     ███████╗█████╗  ██╔██╗ ██║   ██║   ██║██║     ███████║    ║" -ForegroundColor Cyan
    Write-Host "    ║     ╚════██║██╔══╝  ██║╚██╗██║   ██║   ██║██║     ██╔══██║    ║" -ForegroundColor Cyan
    Write-Host "    ║     ███████║███████╗██║ ╚████║   ██║   ██║╚██████╗██║  ██║    ║" -ForegroundColor Cyan
    Write-Host "    ║     ╚══════╝╚══════╝╚═╝  ╚═══╝   ╚═╝   ╚═╝ ╚═════╝╚═╝  ╚═╝    ║" -ForegroundColor Cyan
    Write-Host "    ║                                                               ║" -ForegroundColor Cyan
    Write-Host "    ║              The Operating System That Thinks                 ║" -ForegroundColor Cyan
    Write-Host "    ║                                                               ║" -ForegroundColor Cyan
    Write-Host "    ╚═══════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
    Write-Host ""
}

# ═══════════════════════════════════════════════════════════════════════════════
# STEP 1: Welcome
# ═══════════════════════════════════════════════════════════════════════════════
function Step-Welcome {
    Show-Header
    
    Write-Host "SENTIENT OS'e hoş geldiniz!" -ForegroundColor White
    Write-Host ""
    Write-Host "Bu kurulum scripti size adım adım rehberlik edecek:"
    Write-Host ""
    Write-Host "  1. Sistem kontrolü" -ForegroundColor Cyan
    Write-Host "  2. Model seçimi" -ForegroundColor Cyan
    Write-Host "  3. Bağımlılık kurulumu" -ForegroundColor Cyan
    Write-Host "  4. SENTIENT kurulumu" -ForegroundColor Cyan
    Write-Host "  5. Yapılandırma" -ForegroundColor Cyan
    Write-Host ""
    
    if (-not $Silent) {
        Read-Host "Devam etmek için Enter'a basın"
    }
}

# ═══════════════════════════════════════════════════════════════════════════════
# STEP 2: System Check
# ═══════════════════════════════════════════════════════════════════════════════
function Step-SystemCheck {
    Write-Step -Step 2 -Total 5 -Title "SİSTEM KONTROLÜ"
    
    # OS Info
    $OS = Get-CimInstance Win32_OperatingSystem
    Write-Host "📌 İşletim Sistemi: $($OS.Caption)" -ForegroundColor Cyan
    
    # RAM
    $RAM_GB = [math]::Round($OS.TotalVisibleMemorySize / 1MB, 0)
    if ($RAM_GB -lt 8) {
        Write-Warning "RAM: ${RAM_GB}GB (Önerilen: 16GB+)"
    } else {
        Write-Success "RAM: ${RAM_GB}GB"
    }
    
    # Disk
    $Disk = Get-CimInstance Win32_LogicalDisk -Filter "DeviceID='C:'"
    $Disk_GB = [math]::Round($Disk.FreeSpace / 1GB, 1)
    Write-Host "💾 Disk Alanı: ${Disk_GB}GB boş" -ForegroundColor Cyan
    
    # GPU
    $GPU = Get-CimInstance Win32_VideoController | Where-Object { $_.Name -like "*NVIDIA*" -or $_.Name -like "*RTX*" }
    if ($GPU) {
        Write-Success "GPU: $($GPU.Name)"
        $script:HasGPU = $true
    } else {
        Write-Warning "GPU: NVIDIA GPU tespit edilmedi (Yerel model için önerilir)"
        $script:HasGPU = $false
    }
    
    Write-Host ""
    Write-Success "Sistem kontrolü tamamlandı"
}

# ═══════════════════════════════════════════════════════════════════════════════
# STEP 3: Model Selection
# ═══════════════════════════════════════════════════════════════════════════════
function Step-ModelSelection {
    Write-Step -Step 3 -Total 5 -Title "MODEL SEÇİMİ"
    
    Write-Host "SENTIENT OS birden fazla model destekler:" -ForegroundColor White
    Write-Host ""
    
    Write-Host "╔═══════════════════════════════════════════════════════════════╗" -ForegroundColor Green
    Write-Host "║  🏠 YEREL MODELLER (API Key Gerektirmez)                     ║" -ForegroundColor Green
    Write-Host "╠═══════════════════════════════════════════════════════════════╣" -ForegroundColor Green
    Write-Host "║  1) Gemma 4 31B    - 256K context, Thinking Mode (ÖNERİLEN) ║" -ForegroundColor Green
    Write-Host "║  2) Llama 3.3 70B  - 128K context, Genel kullanım           ║" -ForegroundColor Green
    Write-Host "║  3) Qwen 2.5 72B   - 128K context, Coding optimize          ║" -ForegroundColor Green
    Write-Host "║  4) DeepSeek R1    - 128K context, Reasoning                ║" -ForegroundColor Green
    Write-Host "║  5) Mistral 24B    - 128K context, Hızlı                    ║" -ForegroundColor Green
    Write-Host "╚═══════════════════════════════════════════════════════════════╝" -ForegroundColor Green
    
    Write-Host "╔═══════════════════════════════════════════════════════════════╗" -ForegroundColor Blue
    Write-Host "║  🔑 API MODELLER (API Key Gerekli)                           ║" -ForegroundColor Blue
    Write-Host "╠═══════════════════════════════════════════════════════════════╣" -ForegroundColor Blue
    Write-Host "║  6) OpenAI GPT-4o         - Multimodal, Coding              ║" -ForegroundColor Blue
    Write-Host "║  7) Anthropic Claude 3.7  - Coding, Reasoning               ║" -ForegroundColor Blue
    Write-Host "║  8) Google Gemini 2.0     - 1M Context                      ║" -ForegroundColor Blue
    Write-Host "║  9) Groq Llama 3.3        - Hızlı Inference                 ║" -ForegroundColor Blue
    Write-Host "║ 10) OpenRouter Free       - Ücretsiz Tier                   ║" -ForegroundColor Blue
    Write-Host "╚═══════════════════════════════════════════════════════════════╝" -ForegroundColor Blue
    
    Write-Host ""
    
    if ($Silent -and $Model) {
        $Choice = $Model
    } else {
        $Choice = Read-Host "Model seçiniz [1-10] (varsayılan: 1)"
        if ([string]::IsNullOrWhiteSpace($Choice)) { $Choice = "1" }
    }
    
    switch ($Choice) {
        "1" { $script:SelectedModel = "gemma4:31b"; $script:Provider = "local"; $script:ModelDesc = "Gemma 4 31B (Yerel)" }
        "2" { $script:SelectedModel = "llama3.3:70b"; $script:Provider = "local"; $script:ModelDesc = "Llama 3.3 70B (Yerel)" }
        "3" { $script:SelectedModel = "qwen2.5:72b"; $script:Provider = "local"; $script:ModelDesc = "Qwen 2.5 72B (Yerel)" }
        "4" { $script:SelectedModel = "deepseek-r1:67b"; $script:Provider = "local"; $script:ModelDesc = "DeepSeek R1 (Yerel)" }
        "5" { $script:SelectedModel = "mistral:24b"; $script:Provider = "local"; $script:ModelDesc = "Mistral 24B (Yerel)" }
        "6" { $script:SelectedModel = "gpt-4o"; $script:Provider = "openai"; $script:ModelDesc = "GPT-4o (OpenAI)" }
        "7" { $script:SelectedModel = "claude-3.7-sonnet"; $script:Provider = "anthropic"; $script:ModelDesc = "Claude 3.7 (Anthropic)" }
        "8" { $script:SelectedModel = "gemini-2.0-flash"; $script:Provider = "google"; $script:ModelDesc = "Gemini 2.0 (Google)" }
        "9" { $script:SelectedModel = "llama-3.3-70b"; $script:Provider = "groq"; $script:ModelDesc = "Llama 3.3 (Groq)" }
        "10" { $script:SelectedModel = "google/gemma-4-31b-it:free"; $script:Provider = "openrouter"; $script:ModelDesc = "Gemma 4 Free (OpenRouter)" }
        default { $script:SelectedModel = "gemma4:31b"; $script:Provider = "local"; $script:ModelDesc = "Gemma 4 31B (Yerel)" }
    }
    
    Write-Success "Seçilen model: $script:ModelDesc"
    
    # API Key
    if ($script:Provider -ne "local" -and $script:Provider -ne "openrouter") {
        Write-Host ""
        Write-Warning "Bu model API key gerektiriyor."
        $ApiKey = Read-Host "$($script:Provider) API key giriniz (veya Enter'a basıp sonra .env'e ekleyin)"
        
        if (-not [string]::IsNullOrWhiteSpace($ApiKey)) {
            $script:ApiKey = $ApiKey
        }
    }
}

# ═══════════════════════════════════════════════════════════════════════════════
# STEP 4: Install Dependencies
# ═══════════════════════════════════════════════════════════════════════════════
function Step-InstallDeps {
    Write-Step -Step 4 -Total 5 -Title "BAĞIMLILIK KURULUMU"
    
    # Check admin
    $IsAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
    
    # Rust
    Write-Host "🦀 Rust kontrol ediliyor..." -ForegroundColor Cyan
    $RustPath = "$env:USERPROFILE\.cargo\bin\rustc.exe"
    
    if (Test-Path $RustPath) {
        $RustVersion = & $RustPath --version 2>$null
        Write-Success "Rust: $RustVersion"
    } else {
        Write-Host "⏳ Rust kuruluyor..." -ForegroundColor Cyan
        
        # Download rustup-init
        $RustupUrl = "https://win.rustup.rs/x86_64"
        $RustupPath = "$env:TEMP\rustup-init.exe"
        
        try {
            Invoke-WebRequest -Uri $RustupUrl -OutFile $RustupPath -UseBasicParsing
            Start-Process -FilePath $RustupPath -ArgumentList "-y" -Wait -NoNewWindow
            Remove-Item $RustupPath -Force
            
            # Refresh PATH
            $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "User") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "Machine")
            
            Write-Success "Rust kuruldu"
        } catch {
            Write-Error "Rust kurulumu başarısız: $_"
            Write-Host "Lütfen manuel olarak https://rustup.rs adresinden kurun"
        }
    }
    
    # Ollama (for local models)
    if ($script:Provider -eq "local") {
        Write-Host ""
        Write-Host "🤖 Ollama kontrol ediliyor..." -ForegroundColor Cyan
        
        $OllamaPath = "$env:LOCALAPPDATA\Programs\Ollama\ollama.exe"
        
        if (Test-Path $OllamaPath) {
            Write-Success "Ollama zaten kurulu"
        } else {
            Write-Host "⏳ Ollama kuruluyor..." -ForegroundColor Cyan
            
            $OllamaUrl = "https://ollama.com/download/OllamaSetup.exe"
            $OllamaSetup = "$env:TEMP\OllamaSetup.exe"
            
            try {
                Invoke-WebRequest -Uri $OllamaUrl -OutFile $OllamaSetup -UseBasicParsing
                Start-Process -FilePath $OllamaSetup -ArgumentList "/S" -Wait
                Remove-Item $OllamaSetup -Force -ErrorAction SilentlyContinue
                
                Write-Success "Ollama kuruldu"
            } catch {
                Write-Warning "Ollama kurulumu başarısız. Lütfen https://ollama.com/download adresinden manuel kurun."
            }
        }
        
        # Start Ollama and download model
        Write-Host ""
        Write-Host "📥 Model indiriliyor: $($script:SelectedModel)" -ForegroundColor Cyan
        Write-Host "   Bu işlem model boyutuna göre birkaç dakika sürebilir..." -ForegroundColor Yellow
        
        # Try to start Ollama
        $OllamaExe = Get-Command ollama -ErrorAction SilentlyContinue
        if ($OllamaExe) {
            & ollama pull $script:SelectedModel
            Write-Success "Model indirildi: $($script:SelectedModel)"
        } else {
            Write-Warning "Ollama PATH'te bulunamadı. Modeli manuel olarak indirin: ollama pull $($script:SelectedModel)"
        }
    }
}

# ═══════════════════════════════════════════════════════════════════════════════
# STEP 5: Install SENTIENT
# ═══════════════════════════════════════════════════════════════════════════════
function Step-InstallSentient {
    Write-Step -Step 5 -Total 5 -Title "SENTIENT KURULUMU"
    
    $InstallDir = "$env:USERPROFILE\sentient"
    $script:InstallDir = $InstallDir
    
    # Git
    Write-Host "📂 SENTIENT OS indiriliyor..." -ForegroundColor Cyan
    
    if (Test-Path $InstallDir) {
        Write-Warning "$InstallDir zaten mevcut"
        
        if (-not $Silent) {
            $Update = Read-Host "Güncellensin mi? (y/n)"
            if ($Update -eq "y") {
                Set-Location $InstallDir
                git pull origin main 2>$null
            }
        }
    } else {
        git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git $InstallDir 2>$null
        Write-Success "SENTIENT OS indirildi"
    }
    
    # Build
    Set-Location $InstallDir
    
    Write-Host ""
    Write-Host "🔨 SENTIENT derleniyor..." -ForegroundColor Cyan
    Write-Host "   İlk derleme 5-10 dakika sürebilir..." -ForegroundColor Yellow
    
    # Check cargo
    $CargoPath = "$env:USERPROFILE\.cargo\bin\cargo.exe"
    if (-not (Test-Path $CargoPath)) {
        Write-Error "Cargo bulunamadı. Rust'ı kurun: https://rustup.rs"
        return
    }
    
    # Build
    & $CargoPath build --release 2>&1 | Out-Null
    
    Write-Success "SENTIENT derlendi"
    
    # Create .env
    Write-Host ""
    Write-Host "⚙️  Yapılandırma oluşturuluyor..." -ForegroundColor Cyan
    
    $EnvContent = @"
# ═════════════════════════════════════════════════════════════
#  SENTIENT OS Yapılandırma
# ═════════════════════════════════════════════════════════════

# Model Ayarları
SENTIENT_MODEL=$($script:SelectedModel)
SENTIENT_PROVIDER=$($script:Provider)

# API Keys
$("$($script:Provider.ToUpper())_API_KEY=$($script:ApiKey)" -if $script:ApiKey)

# OpenRouter (ücretsiz modeller için)
# OPENROUTER_API_KEY=sk-or-...

# Yerel Model (Ollama)
OLLAMA_HOST=http://localhost:11434

# Gateway
GATEWAY_HTTP_ADDR=0.0.0.0:8080
JWT_SECRET=change-this-in-production

# Memory
MEMORY_DB_PATH=data/sentient.db

# Logging
RUST_LOG=info
"@
    
    $EnvContent | Out-File -FilePath "$InstallDir\.env" -Encoding utf8
    
    Write-Success ".env dosyası oluşturuldu"
}

# ═══════════════════════════════════════════════════════════════════════════════
# COMPLETE
# ═══════════════════════════════════════════════════════════════════════════════
function Show-Complete {
    Clear-Host
    
    Write-Host ""
    Write-Host "    ╔═══════════════════════════════════════════════════════════════╗" -ForegroundColor Green
    Write-Host "    ║                                                               ║" -ForegroundColor Green
    Write-Host "    ║              🎉 SENTIENT OS KURULUMU TAMAMLANDI!             ║" -ForegroundColor Green
    Write-Host "    ║                                                               ║" -ForegroundColor Green
    Write-Host "    ╚═══════════════════════════════════════════════════════════════╝" -ForegroundColor Green
    Write-Host ""
    
    Write-Host "📋 Kurulum Özeti:" -ForegroundColor White
    Write-Host "  Model:     $($script:ModelDesc)" -ForegroundColor Cyan
    Write-Host "  Provider:  $($script:Provider)" -ForegroundColor Cyan
    Write-Host "  Dizin:     $($script:InstallDir)" -ForegroundColor Cyan
    Write-Host ""
    
    Write-Host "🚀 Başlatmak için:" -ForegroundColor White
    Write-Host ""
    Write-Host "  cd $($script:InstallDir)" -ForegroundColor White
    Write-Host "  .\target\release\sentient.exe" -ForegroundColor White
    Write-Host ""
    
    Write-Host "🌐 Dashboard:" -ForegroundColor White
    Write-Host ""
    Write-Host "  .\target\release\sentient-dashboard.exe" -ForegroundColor White
    Write-Host "  # http://localhost:8080" -ForegroundColor Gray
    Write-Host ""
    
    Write-Host "⚙️  Yapılandırma:" -ForegroundColor White
    Write-Host ""
    Write-Host "  notepad $($script:InstallDir)\.env" -ForegroundColor White
    Write-Host ""
    
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Magenta
    Write-Host "🧠 SENTIENT OS - The Operating System That Thinks" -ForegroundColor Green
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Magenta
    
    # Add to PATH
    if (-not $Silent) {
        Write-Host ""
        $AddPath = Read-Host "SENTIENT'i PATH'e eklemek ister misiniz? (y/n)"
        
        if ($AddPath -eq "y") {
            $CurrentPath = [Environment]::GetEnvironmentVariable("Path", "User")
            $NewPath = "$($script:InstallDir)\target\release"
            
            if ($CurrentPath -notlike "*$NewPath*") {
                [Environment]::SetEnvironmentVariable("Path", "$CurrentPath;$NewPath", "User")
                Write-Success "PATH'e eklendi. Yeni terminalde 'sentient' komutu çalışacaktır."
            }
        }
    }
}

# ═══════════════════════════════════════════════════════════════════════════════
# MAIN
# ═══════════════════════════════════════════════════════════════════════════════
function Main {
    Step-Welcome
    Step-SystemCheck
    Step-ModelSelection
    Step-InstallDeps
    Step-InstallSentient
    Show-Complete
}

# Run
Main
