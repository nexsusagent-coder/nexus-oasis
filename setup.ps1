# ═══════════════════════════════════════════════════════════════════════════════
#  🧠 SENTIENT OS - Interactive Onboarding Wizard
#  The Operating System That Thinks
# ═══════════════════════════════════════════════════════════════════════════════

# Error handling
$ErrorActionPreference = "Continue"

# ─────────────────────────────────────────────────────────────────────────────
# GLOBAL STATE
# ─────────────────────────────────────────────────────────────────────────────
$script:SelectedLLM = ""
$script:SelectedProvider = ""
$script:SelectedChannels = @()
$script:ApiKeys = @{}
$script:InstallDir = "$env:USERPROFILE\sentient"

# ─────────────────────────────────────────────────────────────────────────────
# UI HELPERS
# ─────────────────────────────────────────────────────────────────────────────

function Write-Header {
    param([string]$Title, [string]$Step, [string]$Total)
    
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
    Write-Host "    ║              🧠 The Operating System That Thinks              ║" -ForegroundColor Cyan
    Write-Host "    ║                                                               ║" -ForegroundColor Cyan
    Write-Host "    ╚═══════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
    Write-Host ""
    
    if ($Step -and $Total) {
        Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Magenta
        Write-Host "  ADIM $Step/$Total`: $Title" -ForegroundColor White
        Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Magenta
    }
    
    Write-Host ""
}

function Write-Success {
    param([string]$Message)
    Write-Host "✅ $Message" -ForegroundColor Green
}

function Write-Error {
    param([string]$Message)
    Write-Host "❌ $Message" -ForegroundColor Red
}

function Write-Info {
    param([string]$Message)
    Write-Host "ℹ️  $Message" -ForegroundColor Blue
}

function Write-Warning {
    param([string]$Message)
    Write-Host "⚠️  $Message" -ForegroundColor Yellow
}

function Confirm-Choice {
    param([string]$Prompt, [string]$Default = "Y")
    
    $choices = if ($Default -eq "Y") { "[Y/n]" } else { "[y/N]" }
    Write-Host "$Prompt $choices" -ForegroundColor Cyan -NoNewline
    Write-Host ": " -NoNewline
    
    $input = Read-Host
    $input = if ([string]::IsNullOrWhiteSpace($input)) { $Default } else { $input }
    
    return $input -match "^[Yy]$"
}

function Press-Enter {
    Write-Host ""
    Write-Host "Devam etmek için Enter'a basın..." -ForegroundColor DarkGray
    Read-Host | Out-Null
}

# ─────────────────────────────────────────────────────────────────────────────
# STEP 1: WELCOME & ACCEPTANCE
# ─────────────────────────────────────────────────────────────────────────────

function Step-Welcome {
    Write-Header -Title "KARŞILAMA VE KURULUM ONAYI" -Step "1" -Total "4"
    
    Write-Host "SENTIENT OS'e hoş geldiniz!" -ForegroundColor White
    Write-Host ""
    Write-Host "Bu sihirbaz, size adım adım rehberlik edecek ve ihtiyaçlarınıza göre"
    Write-Host "özelleştirilmiş bir kurulum yapacaktır."
    Write-Host ""
    Write-Host "Kurulum şunları içerecek:" -ForegroundColor White
    Write-Host ""
    Write-Host "  ◆ Sistem gereksinimleri kontrolü" -ForegroundColor Cyan
    Write-Host "  ◆ LLM (Yapay Zeka) modeli seçimi" -ForegroundColor Cyan
    Write-Host "  ◆ Mesajlaşma kanalları yapılandırması" -ForegroundColor Cyan
    Write-Host "  ◆ Temel bağımlılıkların kurulumu" -ForegroundColor Cyan
    Write-Host ""
    
    # System info
    $OS = Get-CimInstance Win32_OperatingSystem -ErrorAction SilentlyContinue
    $OSName = if ($OS) { $OS.Caption } else { "Windows" }
    
    Write-Host "────────────────────────────────────────────────────────────────────────" -ForegroundColor DarkGray
    Write-Host "Sistem: $OSName" -ForegroundColor DarkGray
    Write-Host "Kullanıcı: $env:USERNAME" -ForegroundColor DarkGray
    Write-Host "Dizin: $script:InstallDir" -ForegroundColor DarkGray
    Write-Host "────────────────────────────────────────────────────────────────────────" -ForegroundColor DarkGray
    Write-Host ""
    
    if (Confirm-Choice -Prompt "Kuruluma başlamak istiyor musunuz?" -Default "Y") {
        return $true
    } else {
        Write-Host ""
        Write-Info "Kurulum iptal edildi. Görüşmek üzere!"
        exit 0
    }
}

# ─────────────────────────────────────────────────────────────────────────────
# STEP 2: LLM SELECTION
# ─────────────────────────────────────────────────────────────────────────────

function Step-LLMSelection {
    Write-Header -Title "LLM (YAPAY ZEKA) MODELİ SEÇİMİ" -Step "2" -Total "4"
    
    Write-Host "SENTIENT OS hangi yapay zeka modelini kullanmasını istersiniz?" -ForegroundColor White
    Write-Host ""
    
    # Local Models
    Write-Host "╔═════════════════════════════════════════════════════════════════════════╗" -ForegroundColor Green
    Write-Host "║  🏠 YEREL MODELLER (API Key Gerektirmez, Tam Gizlilik)               ║" -ForegroundColor Green
    Write-Host "╠═════════════════════════════════════════════════════════════════════════╣" -ForegroundColor Green
    Write-Host "║  1) Ollama - Gemma 4 31B     256K context, Thinking Mode             ║" -ForegroundColor Green
    Write-Host "║  2) Ollama - Llama 3.3 70B   128K context, Genel kullanım            ║" -ForegroundColor Green
    Write-Host "║  3) Ollama - Qwen 2.5 72B    128K context, Coding optimize           ║" -ForegroundColor Green
    Write-Host "║  4) Ollama - DeepSeek R1     128K context, Reasoning                 ║" -ForegroundColor Green
    Write-Host "║  5) Ollama - Mistral 24B     128K context, Hızlı                     ║" -ForegroundColor Green
    Write-Host "╚═════════════════════════════════════════════════════════════════════════╝" -ForegroundColor Green
    Write-Host ""
    
    # Cloud Models
    Write-Host "╔═════════════════════════════════════════════════════════════════════════╗" -ForegroundColor Blue
    Write-Host "║  ☁️  BULUT MODELLER (API Key Gerekli)                                 ║" -ForegroundColor Blue
    Write-Host "╠═════════════════════════════════════════════════════════════════════════╣" -ForegroundColor Blue
    Write-Host "║  6) OpenAI GPT-4o            Multimodal, Coding                      ║" -ForegroundColor Blue
    Write-Host "║  7) Anthropic Claude 3.7     Coding, Reasoning                       ║" -ForegroundColor Blue
    Write-Host "║  8) Google Gemini 2.0        1M Context                              ║" -ForegroundColor Blue
    Write-Host "║  9) Groq Llama 3.3           Hızlı Inference                         ║" -ForegroundColor Blue
    Write-Host "║ 10) OpenRouter (Free Tier)   Ücretsiz modeller                       ║" -ForegroundColor Blue
    Write-Host "╚═════════════════════════════════════════════════════════════════════════╝" -ForegroundColor Blue
    Write-Host ""
    
    # Skip option
    Write-Host "  0) LLM kurulumunu atla (Daha sonra yapılandıracağım)" -ForegroundColor Yellow
    Write-Host ""
    
    while ($true) {
        Write-Host "Seçiminiz [1-10, 0=atla]: " -ForegroundColor Cyan -NoNewline
        $choice = Read-Host
        
        switch ($choice) {
            "0" {
                $script:SelectedLLM = "skip"
                $script:SelectedProvider = "none"
                Write-Info "LLM kurulumu atlandı. Daha sonra .env dosyasından yapılandırabilirsiniz."
                return
            }
            "1" {
                $script:SelectedLLM = "gemma4:31b"
                $script:SelectedProvider = "ollama"
                break
            }
            "2" {
                $script:SelectedLLM = "llama3.3:70b"
                $script:SelectedProvider = "ollama"
                break
            }
            "3" {
                $script:SelectedLLM = "qwen2.5:72b"
                $script:SelectedProvider = "ollama"
                break
            }
            "4" {
                $script:SelectedLLM = "deepseek-r1:67b"
                $script:SelectedProvider = "ollama"
                break
            }
            "5" {
                $script:SelectedLLM = "mistral:24b"
                $script:SelectedProvider = "ollama"
                break
            }
            "6" {
                $script:SelectedLLM = "gpt-4o"
                $script:SelectedProvider = "openai"
                break
            }
            "7" {
                $script:SelectedLLM = "claude-3.7-sonnet"
                $script:SelectedProvider = "anthropic"
                break
            }
            "8" {
                $script:SelectedLLM = "gemini-2.0-flash"
                $script:SelectedProvider = "google"
                break
            }
            "9" {
                $script:SelectedLLM = "llama-3.3-70b-versatile"
                $script:SelectedProvider = "groq"
                break
            }
            "10" {
                $script:SelectedLLM = "google/gemma-4-31b-it:free"
                $script:SelectedProvider = "openrouter"
                break
            }
            default {
                Write-Warning "Geçersiz seçim. Lütfen 0-10 arası bir sayı girin."
                continue
            }
        }
        break
    }
    
    Write-Success "Seçilen model: $script:SelectedLLM ($script:SelectedProvider)"
    
    # API Key for cloud providers
    if ($script:SelectedProvider -ne "ollama" -and $script:SelectedProvider -ne "none") {
        Write-Host ""
        Write-Warning "$script:SelectedProvider API key gereklidir."
        Write-Host "$script:SelectedProvider API Key: " -ForegroundColor Cyan -NoNewline
        $apiKey = Read-Host
        
        if (-not [string]::IsNullOrWhiteSpace($apiKey)) {
            $script:ApiKeys[$script:SelectedProvider] = $apiKey
            Write-Success "API key kaydedildi."
        } else {
            Write-Warning "API key girilmedi. Daha sonra .env dosyasından ekleyebilirsiniz."
        }
    }
    
    Write-Host ""
}

# ─────────────────────────────────────────────────────────────────────────────
# STEP 3: MESSAGING CHANNELS
# ─────────────────────────────────────────────────────────────────────────────

function Step-MessagingChannels {
    Write-Header -Title "MESAJLAŞMA KANALLARI" -Step "3" -Total "4"
    
    Write-Host "SENTIENT OS'i hangi platformlarda kullanmak istersiniz?" -ForegroundColor White
    Write-Host ""
    Write-Host "İstediğiniz kadar kanal seçebilirsiniz. Seçmediklerinizi atlayabilirsiniz." -ForegroundColor DarkGray
    Write-Host ""
    
    $selectedChannels = @()
    
    # Telegram
    Write-Host "━━━ Telegram Bot ━━━" -ForegroundColor Cyan
    if (Confirm-Choice -Prompt "Telegram bot bağlamak istiyor musunuz?" -Default "N") {
        $selectedChannels += "telegram"
        Write-Host "  Telegram Bot Token: " -ForegroundColor Yellow -NoNewline
        $token = Read-Host
        Write-Host "  Telegram Chat ID: " -ForegroundColor Yellow -NoNewline
        $chatId = Read-Host
        $script:ApiKeys["telegram_token"] = $token
        $script:ApiKeys["telegram_chat_id"] = $chatId
        Write-Success "Telegram yapılandırıldı."
    }
    Write-Host ""
    
    # WhatsApp
    Write-Host "━━━ WhatsApp Business ━━━" -ForegroundColor Cyan
    if (Confirm-Choice -Prompt "WhatsApp Business API bağlamak istiyor musunuz?" -Default "N") {
        $selectedChannels += "whatsapp"
        Write-Host "  WhatsApp Phone Number ID: " -ForegroundColor Yellow -NoNewline
        $phoneId = Read-Host
        Write-Host "  WhatsApp Access Token: " -ForegroundColor Yellow -NoNewline
        $waToken = Read-Host
        $script:ApiKeys["whatsapp_phone_id"] = $phoneId
        $script:ApiKeys["whatsapp_token"] = $waToken
        Write-Success "WhatsApp yapılandırıldı."
    }
    Write-Host ""
    
    # Discord
    Write-Host "━━━ Discord Bot ━━━" -ForegroundColor Cyan
    if (Confirm-Choice -Prompt "Discord bot bağlamak istiyor musunuz?" -Default "N") {
        $selectedChannels += "discord"
        Write-Host "  Discord Bot Token: " -ForegroundColor Yellow -NoNewline
        $discordToken = Read-Host
        $script:ApiKeys["discord_token"] = $discordToken
        Write-Success "Discord yapılandırıldı."
    }
    Write-Host ""
    
    # Slack
    Write-Host "━━━ Slack App ━━━" -ForegroundColor Cyan
    if (Confirm-Choice -Prompt "Slack app bağlamak istiyor musunuz?" -Default "N") {
        $selectedChannels += "slack"
        Write-Host "  Slack Bot Token (xoxb-...): " -ForegroundColor Yellow -NoNewline
        $slackToken = Read-Host
        $script:ApiKeys["slack_token"] = $slackToken
        Write-Success "Slack yapılandırıldı."
    }
    Write-Host ""
    
    # Web Interface
    Write-Host "━━━ Web Arayüzü ━━━" -ForegroundColor Cyan
    if (Confirm-Choice -Prompt "Web arayüzü kurulacak (önerilir)?" -Default "Y") {
        $selectedChannels += "web"
        Write-Success "Web arayüzü eklendi."
    }
    Write-Host ""
    
    # API
    Write-Host "━━━ REST API ━━━" -ForegroundColor Cyan
    if (Confirm-Choice -Prompt "REST API erişimi açılsın mı?" -Default "Y") {
        $selectedChannels += "api"
        Write-Success "REST API eklendi."
    }
    Write-Host ""
    
    $script:SelectedChannels = $selectedChannels
    
    if ($selectedChannels.Count -eq 0) {
        Write-Warning "Hiçbir kanal seçilmedi. Yerel kullanım için devam edilecek."
    } else {
        Write-Success "Seçilen kanallar: $($selectedChannels -join ', ')"
    }
}

# ─────────────────────────────────────────────────────────────────────────────
# STEP 4: INSTALLATION
# ─────────────────────────────────────────────────────────────────────────────

function Step-Installation {
    Write-Header -Title "KURULUM" -Step "4" -Total "4"
    
    Write-Host "Seçimleriniz kaydedildi. Şimdi kurulum başlıyor..." -ForegroundColor White
    Write-Host ""
    
    # Summary
    Write-Host "Kurulum Özeti:" -ForegroundColor White
    Write-Host ""
    Write-Host "  LLM:     $($script:SelectedLLM)" -ForegroundColor Cyan
    Write-Host "  Kanallar: $($script:SelectedChannels -join ', ')" -ForegroundColor Cyan
    Write-Host "  Dizin:   $script:InstallDir" -ForegroundColor Cyan
    Write-Host ""
    
    if (-not (Confirm-Choice -Prompt "Kuruluma devam edilsin mi?" -Default "Y")) {
        Write-Info "Kurulum iptal edildi."
        exit 0
    }
    
    Write-Host ""
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Magenta
    Write-Host ""
    
    # 1. System Check
    Write-Info "Sistem kontrol ediliyor..."
    
    # Check Rust
    $RustPath = "$env:USERPROFILE\.cargo\bin\rustc.exe"
    if (Test-Path $RustPath) {
        $RustVersion = & $RustPath --version 2>$null
        Write-Success "Rust mevcut: $RustVersion"
    } else {
        Write-Info "Rust kuruluyor..."
        
        $RustupUrl = "https://win.rustup.rs/x86_64"
        $RustupPath = "$env:TEMP\rustup-init.exe"
        
        try {
            Invoke-WebRequest -Uri $RustupUrl -OutFile $RustupPath -UseBasicParsing
            Start-Process -FilePath $RustupPath -ArgumentList "-y" -Wait -NoNewWindow
            Remove-Item $RustupPath -Force -ErrorAction SilentlyContinue
            
            # Refresh PATH
            $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "User") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "Machine")
            
            Write-Success "Rust kuruldu."
        } catch {
            Write-Warning "Rust kurulumu başarısız: $_"
            Write-Host "Lütfen manuel olarak https://rustup.rs adresinden kurun" -ForegroundColor Yellow
        }
    }
    
    # Check Git
    Write-Host ""
    Write-Info "Git kontrol ediliyor..."
    if (Get-Command git -ErrorAction SilentlyContinue) {
        Write-Success "Git mevcut."
    } else {
        Write-Warning "Git bulunamadı. https://git-scm.com/download/win adresinden kurun"
    }
    
    # 2. Clone Repository
    Write-Host ""
    Write-Info "SENTIENT OS indiriliyor..."
    
    if (Test-Path $script:InstallDir) {
        Write-Warning "$script:InstallDir zaten mevcut."
        Set-Location $script:InstallDir
        git pull origin main 2>$null
        Write-Success "Depo güncellendi."
    } else {
        git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git $script:InstallDir 2>$null
        if ($LASTEXITCODE -eq 0) {
            Write-Success "Depo klonlandı."
        } else {
            Write-Error "Klonlama başarısız!"
            exit 1
        }
    }
    
    Set-Location $script:InstallDir
    
    # 3. Install Ollama if local model selected
    if ($script:SelectedProvider -eq "ollama") {
        Write-Host ""
        Write-Info "Ollama kontrol ediliyor..."
        
        $OllamaPath = "$env:LOCALAPPDATA\Programs\Ollama\ollama.exe"
        
        if (Test-Path $OllamaPath) {
            Write-Success "Ollama mevcut."
        } else {
            Write-Info "Ollama kuruluyor..."
            
            $OllamaUrl = "https://ollama.com/download/OllamaSetup.exe"
            $OllamaSetup = "$env:TEMP\OllamaSetup.exe"
            
            try {
                Invoke-WebRequest -Uri $OllamaUrl -OutFile $OllamaSetup -UseBasicParsing
                Write-Host "Ollama kurulumu başlatılıyor..." -ForegroundColor Cyan
                Start-Process -FilePath $OllamaSetup -Wait
                Remove-Item $OllamaSetup -Force -ErrorAction SilentlyContinue
                
                Write-Success "Ollama kuruldu."
            } catch {
                Write-Warning "Ollama kurulumu başarısız. Lütfen https://ollama.com/download adresinden manuel kurun."
            }
        }
        
        # Pull model
        Write-Host ""
        Write-Info "Model indiriliyor: $script:SelectedLLM"
        
        $OllamaExe = Get-Command ollama -ErrorAction SilentlyContinue
        if ($OllamaExe) {
            & ollama pull $script:SelectedLLM
            Write-Success "Model hazır: $script:SelectedLLM"
        } else {
            Write-Warning "Ollama PATH'te bulunamadı. Modeli manuel olarak indirin: ollama pull $script:SelectedLLM"
        }
    }
    
    # 4. Build
    Write-Host ""
    Write-Info "SENTIENT derleniyor... (Bu işlem birkaç dakika sürebilir)"
    
    $CargoPath = "$env:USERPROFILE\.cargo\bin\cargo.exe"
    if (Test-Path $CargoPath) {
        & $CargoPath build --release
        if ($LASTEXITCODE -eq 0) {
            Write-Success "SENTIENT derlendi."
        } else {
            Write-Error "Derleme başarısız. Hataları kontrol edin."
            exit 1
        }
    } else {
        Write-Error "Cargo bulunamadı. Rust'ı kurun: https://rustup.rs"
        exit 1
    }
    
    # 5. Create .env
    Write-Host ""
    Write-Info "Yapılandırma dosyası oluşturuluyor..."
    
    $envContent = @"
# ═════════════════════════════════════════════════════════════
#  SENTIENT OS Yapılandırma
# ═════════════════════════════════════════════════════════════

# Model
SENTIENT_MODEL=$script:SelectedLLM
SENTIENT_PROVIDER=$script:SelectedProvider

# API Keys
"@

    foreach ($key in $script:ApiKeys.Keys) {
        $envContent += "`n$($key.ToUpper())=$($script:ApiKeys[$key])"
    }

    $envContent += @"


# Gateway
GATEWAY_HTTP_ADDR=0.0.0.0:8080
JWT_SECRET=change-this-in-production

# Memory
MEMORY_DB_PATH=data/sentient.db

# Logging
RUST_LOG=info
"@

    $envContent | Out-File -FilePath "$script:InstallDir\.env" -Encoding utf8
    Write-Success ".env dosyası oluşturuldu."
}

# ─────────────────────────────────────────────────────────────────────────────
# FINAL: SUCCESS SCREEN
# ─────────────────────────────────────────────────────────────────────────────

function Show-Success {
    Write-Header -Title "KURULUM TAMAMLANDI"
    
    Write-Host "╔═════════════════════════════════════════════════════════════════════════╗" -ForegroundColor Green
    Write-Host "║                                                                           ║" -ForegroundColor Green
    Write-Host "║              🎉 SENTIENT OS KURULUMU BAŞARIYLA TAMAMLANDI!               ║" -ForegroundColor Green
    Write-Host "║                                                                           ║" -ForegroundColor Green
    Write-Host "╚═════════════════════════════════════════════════════════════════════════╝" -ForegroundColor Green
    Write-Host ""
    
    Write-Host "📋 Kurulum Özeti:" -ForegroundColor White
    Write-Host ""
    Write-Host "  Model:    $script:SelectedLLM" -ForegroundColor Cyan
    Write-Host "  Provider: $script:SelectedProvider" -ForegroundColor Cyan
    Write-Host "  Kanallar: $($script:SelectedChannels -join ', ')" -ForegroundColor Cyan
    Write-Host "  Dizin:    $script:InstallDir" -ForegroundColor Cyan
    Write-Host ""
    
    Write-Host "🚀 Başlatmak için:" -ForegroundColor White
    Write-Host ""
    Write-Host "  cd $script:InstallDir" -ForegroundColor White
    Write-Host "  .\target\release\sentient-shell.exe" -ForegroundColor White
    Write-Host ""
    
    Write-Host "⚙️  Yapılandırma:" -ForegroundColor White
    Write-Host ""
    Write-Host "  notepad $script:InstallDir\.env" -ForegroundColor White
    Write-Host ""
    
    if ($script:SelectedProvider -eq "ollama") {
        Write-Host "🤖 Model Yönetimi:" -ForegroundColor White
        Write-Host ""
        Write-Host "  ollama list                  # Yüklü modeller" -ForegroundColor White
        Write-Host "  ollama run $script:SelectedLLM  # Modeli çalıştır" -ForegroundColor White
        Write-Host ""
    }
    
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Magenta
    Write-Host "🧠 SENTIENT OS - The Operating System That Thinks" -ForegroundColor Green
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Magenta
    Write-Host ""
    
    # Add to PATH
    $AddPath = Read-Host "SENTIENT'i PATH'e eklemek ister misiniz? (y/n)"
    if ($AddPath -eq "y") {
        $CurrentPath = [Environment]::GetEnvironmentVariable("Path", "User")
        $NewPath = "$script:InstallDir\target\release"
        
        if ($CurrentPath -notlike "*$NewPath*") {
            [Environment]::SetEnvironmentVariable("Path", "$CurrentPath;$NewPath", "User")
            Write-Success "PATH'e eklendi. Yeni terminalde 'sentient' komutu çalışacaktır."
        }
    }
    
    Write-Host ""
    Write-Host "Kurulum tamamlandı! İyi kullanımlar! 🚀" -ForegroundColor Green
}

# ─────────────────────────────────────────────────────────────────────────────
# MAIN ENTRY POINT
# ─────────────────────────────────────────────────────────────────────────────

function Main {
    Step-Welcome
    Step-LLMSelection
    Step-MessagingChannels
    Step-Installation
    Show-Success
}

# Run the wizard
Main
