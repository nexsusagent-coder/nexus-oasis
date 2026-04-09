# ═══════════════════════════════════════════════════════════════════════════════
#  🧠 SENTIENT OS - The Operating System That Thinks
#  Interactive Setup Script v5.0.0 (Windows)
# ═══════════════════════════════════════════════════════════════════════════════

# Hata durumunda devam et
$ErrorActionPreference = "Continue"

# Renkli yazı
function Write-ColorText {
    param([string]$Text, [string]$Color = "White")
    Write-Host $Text -ForegroundColor $Color
}

# Header
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

Read-Host "Devam etmek için Enter'a basın"
Write-Host ""

# ═══════════════════════════════════════════════════════════════════════════════
# ADIM 1: Sistem Kontrolü
# ═══════════════════════════════════════════════════════════════════════════════

Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Magenta
Write-Host "  ADIM 1/5: SİSTEM KONTROLÜ" -ForegroundColor Cyan
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Magenta
Write-Host ""

# OS Info
$OS = Get-CimInstance Win32_OperatingSystem -ErrorAction SilentlyContinue
if ($OS) {
    Write-Host "📌 İşletim Sistemi: $($OS.Caption)" -ForegroundColor Cyan
} else {
    Write-Host "📌 İşletim Sistemi: Windows" -ForegroundColor Cyan
}

# RAM
$RAM_GB = [math]::Round((Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory / 1GB, 0)
if ($RAM_GB -lt 8) {
    Write-Host "⚠ RAM: ${RAM_GB}GB (Önerilen: 16GB+)" -ForegroundColor Yellow
} else {
    Write-Host "✓ RAM: ${RAM_GB}GB" -ForegroundColor Green
}

# Disk
$Disk = Get-CimInstance Win32_LogicalDisk -Filter "DeviceID='C:'" -ErrorAction SilentlyContinue
if ($Disk) {
    $Disk_GB = [math]::Round($Disk.FreeSpace / 1GB, 1)
    Write-Host "💾 Disk Alanı: ${Disk_GB}GB boş" -ForegroundColor Cyan
}

# GPU
$GPU = Get-CimInstance Win32_VideoController -ErrorAction SilentlyContinue | Where-Object { $_.Name -like "*NVIDIA*" -or $_.Name -like "*RTX*" -or $_.Name -like "*GTX*" }
if ($GPU) {
    Write-Host "✓ GPU: $($GPU.Name)" -ForegroundColor Green
} else {
    Write-Host "⚠ GPU: NVIDIA GPU tespit edilmedi (Yerel model için önerilir)" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "✓ Sistem kontrolü tamamlandı" -ForegroundColor Green

# ═══════════════════════════════════════════════════════════════════════════════
# ADIM 2: Model Seçimi
# ═══════════════════════════════════════════════════════════════════════════════

Write-Host ""
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Magenta
Write-Host "  ADIM 2/5: MODEL SEÇİMİ" -ForegroundColor Cyan
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Magenta
Write-Host ""

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
$Choice = Read-Host "Model seçiniz [1-10] (varsayılan: 1)"
if ([string]::IsNullOrWhiteSpace($Choice)) { $Choice = "1" }

switch ($Choice) {
    "1" { $SelectedModel = "gemma4:31b"; $Provider = "local"; $ModelDesc = "Gemma 4 31B (Yerel)" }
    "2" { $SelectedModel = "llama3.3:70b"; $Provider = "local"; $ModelDesc = "Llama 3.3 70B (Yerel)" }
    "3" { $SelectedModel = "qwen2.5:72b"; $Provider = "local"; $ModelDesc = "Qwen 2.5 72B (Yerel)" }
    "4" { $SelectedModel = "deepseek-r1:67b"; $Provider = "local"; $ModelDesc = "DeepSeek R1 (Yerel)" }
    "5" { $SelectedModel = "mistral:24b"; $Provider = "local"; $ModelDesc = "Mistral 24B (Yerel)" }
    "6" { $SelectedModel = "gpt-4o"; $Provider = "openai"; $ModelDesc = "GPT-4o (OpenAI)" }
    "7" { $SelectedModel = "claude-3.7-sonnet"; $Provider = "anthropic"; $ModelDesc = "Claude 3.7 (Anthropic)" }
    "8" { $SelectedModel = "gemini-2.0-flash"; $Provider = "google"; $ModelDesc = "Gemini 2.0 (Google)" }
    "9" { $SelectedModel = "llama-3.3-70b"; $Provider = "groq"; $ModelDesc = "Llama 3.3 (Groq)" }
    "10" { $SelectedModel = "google/gemma-4-31b-it:free"; $Provider = "openrouter"; $ModelDesc = "Gemma 4 Free (OpenRouter)" }
    default { $SelectedModel = "gemma4:31b"; $Provider = "local"; $ModelDesc = "Gemma 4 31B (Yerel)" }
}

Write-Host ""
Write-Host "✓ Seçilen model: $ModelDesc" -ForegroundColor Green

# API Key
$ApiKey = ""
$ApiKeyLine = ""
if ($Provider -ne "local" -and $Provider -ne "openrouter") {
    Write-Host ""
    Write-Host "⚠️  Bu model API key gerektiriyor." -ForegroundColor Yellow
    $ApiKey = Read-Host "$Provider API key giriniz (veya Enter'a basıp sonra .env'e ekleyin)"
    
    if (-not [string]::IsNullOrWhiteSpace($ApiKey)) {
        $ApiKeyLine = "$($Provider.ToUpper())_API_KEY=$ApiKey"
    }
}

# ═══════════════════════════════════════════════════════════════════════════════
# ADIM 3: Bağımlılık Kurulumu
# ═══════════════════════════════════════════════════════════════════════════════

Write-Host ""
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Magenta
Write-Host "  ADIM 3/5: BAĞIMLILIK KURULUMU" -ForegroundColor Cyan
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Magenta
Write-Host ""

# Rust Check
Write-Host "🦀 Rust kontrol ediliyor..." -ForegroundColor Cyan
$RustPath = "$env:USERPROFILE\.cargo\bin\rustc.exe"

if (Test-Path $RustPath) {
    $RustVersion = & $RustPath --version 2>$null
    Write-Host "✓ Rust: $RustVersion" -ForegroundColor Green
} else {
    Write-Host "⏳ Rust kuruluyor..." -ForegroundColor Cyan
    
    $RustupUrl = "https://win.rustup.rs/x86_64"
    $RustupPath = "$env:TEMP\rustup-init.exe"
    
    try {
        Invoke-WebRequest -Uri $RustupUrl -OutFile $RustupPath -UseBasicParsing
        Start-Process -FilePath $RustupPath -ArgumentList "-y" -Wait -NoNewWindow
        Remove-Item $RustupPath -Force -ErrorAction SilentlyContinue
        
        # Refresh PATH
        $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "User") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "Machine")
        
        Write-Host "✓ Rust kuruldu" -ForegroundColor Green
    } catch {
        Write-Host "⚠ Rust kurulumu başarısız: $_" -ForegroundColor Yellow
        Write-Host "Lütfen manuel olarak https://rustup.rs adresinden kurun" -ForegroundColor Yellow
    }
}

# Git Check
Write-Host ""
Write-Host "📦 Git kontrol ediliyor..." -ForegroundColor Cyan
if (Get-Command git -ErrorAction SilentlyContinue) {
    Write-Host "✓ Git yüklü" -ForegroundColor Green
} else {
    Write-Host "⚠ Git yüklü değil. https://git-scm.com/download/win adresinden kurun" -ForegroundColor Yellow
}

# Ollama (for local models)
if ($Provider -eq "local") {
    Write-Host ""
    Write-Host "🤖 Ollama kontrol ediliyor..." -ForegroundColor Cyan
    
    $OllamaPath = "$env:LOCALAPPDATA\Programs\Ollama\ollama.exe"
    
    if (Test-Path $OllamaPath) {
        Write-Host "✓ Ollama zaten kurulu" -ForegroundColor Green
    } else {
        Write-Host "⏳ Ollama kuruluyor..." -ForegroundColor Cyan
        
        $OllamaUrl = "https://ollama.com/download/OllamaSetup.exe"
        $OllamaSetup = "$env:TEMP\OllamaSetup.exe"
        
        try {
            Invoke-WebRequest -Uri $OllamaUrl -OutFile $OllamaSetup -UseBasicParsing
            Write-Host "Ollama kurulumu başlatılıyor..." -ForegroundColor Cyan
            Start-Process -FilePath $OllamaSetup -Wait
            Remove-Item $OllamaSetup -Force -ErrorAction SilentlyContinue
            
            Write-Host "✓ Ollama kuruldu" -ForegroundColor Green
        } catch {
            Write-Host "⚠ Ollama kurulumu başarısız. Lütfen https://ollama.com/download adresinden manuel kurun." -ForegroundColor Yellow
        }
    }
    
    # Download model
    Write-Host ""
    Write-Host "📥 Model indiriliyor: $SelectedModel" -ForegroundColor Cyan
    Write-Host "   Bu işlem model boyutuna göre birkaç dakika sürebilir..." -ForegroundColor Yellow
    
    $OllamaExe = Get-Command ollama -ErrorAction SilentlyContinue
    if ($OllamaExe) {
        & ollama pull $SelectedModel
        Write-Host "✓ Model hazır: $SelectedModel" -ForegroundColor Green
    } else {
        Write-Host "⚠ Ollama PATH'te bulunamadı. Modeli manuel olarak indirin: ollama pull $SelectedModel" -ForegroundColor Yellow
    }
}

# ═══════════════════════════════════════════════════════════════════════════════
# ADIM 4: SENTIENT Kurulumu
# ═══════════════════════════════════════════════════════════════════════════════

Write-Host ""
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Magenta
Write-Host "  ADIM 4/5: SENTIENT KURULUMU" -ForegroundColor Cyan
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Magenta
Write-Host ""

$InstallDir = "$env:USERPROFILE\sentient"

Write-Host "📂 SENTIENT OS indiriliyor..." -ForegroundColor Cyan

if (Test-Path $InstallDir) {
    Write-Host "⚠ $InstallDir zaten mevcut" -ForegroundColor Yellow
    $Update = Read-Host "Güncellensin mi? (y/n)"
    if ($Update -eq "y") {
        Set-Location $InstallDir
        git pull origin main 2>$null
    }
} else {
    git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git $InstallDir 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ SENTIENT OS indirildi" -ForegroundColor Green
    } else {
        Write-Host "✗ İndirme başarısız!" -ForegroundColor Red
        exit 1
    }
}

# Build
Set-Location $InstallDir

Write-Host ""
Write-Host "🔨 SENTIENT derleniyor..." -ForegroundColor Cyan
Write-Host "   İlk derleme 5-10 dakika sürebilir..." -ForegroundColor Yellow

$CargoPath = "$env:USERPROFILE\.cargo\bin\cargo.exe"
if (Test-Path $CargoPath) {
    & $CargoPath build --release
    Write-Host "✓ SENTIENT derlendi" -ForegroundColor Green
} else {
    Write-Host "✗ Cargo bulunamadı. Rust'ı kurun: https://rustup.rs" -ForegroundColor Red
}

# Create .env
Write-Host ""
Write-Host "⚙️  Yapılandırma oluşturuluyor..." -ForegroundColor Cyan

$EnvContent = @"
# ═════════════════════════════════════════════════════════════
#  SENTIENT OS Yapılandırma
# ═════════════════════════════════════════════════════════════

# Model Ayarları
SENTIENT_MODEL=$SelectedModel
SENTIENT_PROVIDER=$Provider

# API Keys
$ApiKeyLine
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

Write-Host "✓ .env dosyası oluşturuldu" -ForegroundColor Green

# ═══════════════════════════════════════════════════════════════════════════════
# ADIM 5: Tamamlandı
# ═══════════════════════════════════════════════════════════════════════════════

Clear-Host
Write-Host ""
Write-Host "    ╔═══════════════════════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "    ║                                                               ║" -ForegroundColor Green
Write-Host "    ║              🎉 SENTIENT OS KURULUMU TAMAMLANDI!             ║" -ForegroundColor Green
Write-Host "    ║                                                               ║" -ForegroundColor Green
Write-Host "    ╚═══════════════════════════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""

Write-Host "📋 Kurulum Özeti:" -ForegroundColor White
Write-Host "  Model:     $ModelDesc" -ForegroundColor Cyan
Write-Host "  Provider:  $Provider" -ForegroundColor Cyan
Write-Host "  Dizin:     $InstallDir" -ForegroundColor Cyan
Write-Host ""

Write-Host "🚀 Başlatmak için:" -ForegroundColor White
Write-Host ""
Write-Host "  cd $InstallDir" -ForegroundColor White
Write-Host "  .\target\release\sentient.exe" -ForegroundColor White
Write-Host ""

Write-Host "⚙️  Yapılandırma:" -ForegroundColor White
Write-Host ""
Write-Host "  notepad $InstallDir\.env" -ForegroundColor White
Write-Host ""

Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Magenta
Write-Host "🧠 SENTIENT OS - The Operating System That Thinks" -ForegroundColor Green
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Magenta

# Add to PATH
Write-Host ""
$AddPath = Read-Host "SENTIENT'i PATH'e eklemek ister misiniz? (y/n)"

if ($AddPath -eq "y") {
    $CurrentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $NewPath = "$InstallDir\target\release"
    
    if ($CurrentPath -notlike "*$NewPath*") {
        [Environment]::SetEnvironmentVariable("Path", "$CurrentPath;$NewPath", "User")
        Write-Host "✓ PATH'e eklendi. Yeni terminalde 'sentient' komutu çalışacaktır." -ForegroundColor Green
    }
}

Write-Host ""
Write-Host "Kurulum tamamlandı! İyi kullanımlar! 🚀" -ForegroundColor Green
