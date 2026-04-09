# ═══════════════════════════════════════════════════════════════════════════════
#  🧠 SENTIENT OS - Setup Bootstrap Script (Windows)
#  Interactive TUI Wizard with Arrow-Key Navigation
# ═══════════════════════════════════════════════════════════════════════════════

$ErrorActionPreference = "Continue"

# ─────────────────────────────────────────────────────────────────────────────
# WELCOME
# ─────────────────────────────────────────────────────────────────────────────

Clear-Host

Write-Host ""
Write-Host "    ╔══════════════════════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "    ║                                                                              ║" -ForegroundColor Cyan
Write-Host "    ║   ███████╗██╗███████╗███╗   ██╗██████╗ ███████╗██████╗ ██████╗ ██╗███╗   ██╗ ║" -ForegroundColor Cyan
Write-Host "    ║   ██╔════╝██║██╔════╝████╗  ██║██╔══██╗██╔════╝██╔══██╗██╔══██╗██║████╗  ██║ ║" -ForegroundColor Cyan
Write-Host "    ║   ███████╗██║█████╗  ██╔██╗ ██║██║  ██║█████╗  ██████╔╝██████╔╝██║██╔██╗ ██║ ║" -ForegroundColor Cyan
Write-Host "    ║   ╚════██║██║██╔══╝  ██║╚██╗██║██║  ██║██╔══╝  ██╔══██╗██╔══██╗██║██║╚██╗██║ ║" -ForegroundColor Cyan
Write-Host "    ║   ███████║██║███████╗██║ ╚████║██████╔╝███████╗██║  ██║██║  ██║██║██║ ╚████║ ║" -ForegroundColor Cyan
Write-Host "    ║   ╚══════╝╚═╝╚══════╝╚═╝  ╚═══╝╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝ ║" -ForegroundColor Cyan
Write-Host "    ║                                                                              ║" -ForegroundColor Cyan
Write-Host "    ║                     🧠 The Operating System That Thinks                     ║" -ForegroundColor Cyan
Write-Host "    ║                                                                              ║" -ForegroundColor Cyan
Write-Host "    ║                     🎮 Interactive TUI Setup Wizard                         ║" -ForegroundColor Cyan
Write-Host "    ║                     ↑↓ Navigate    Space: Select    Enter: Confirm         ║" -ForegroundColor Cyan
Write-Host "    ║                                                                              ║" -ForegroundColor Cyan
Write-Host "    ╚══════════════════════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# ─────────────────────────────────────────────────────────────────────────────
# DEPENDENCY CHECK
# ─────────────────────────────────────────────────────────────────────────────

Write-Host "📋 Sistem Kontrolü..." -ForegroundColor White
Write-Host ""

# Check Rust
$RustPath = "$env:USERPROFILE\.cargo\bin\rustc.exe"
if (Test-Path $RustPath) {
    $RustVersion = & $RustPath --version 2>$null
    Write-Host "✅ Rust: $RustVersion" -ForegroundColor Green
} else {
    Write-Host "⚠️  Rust bulunamadı. Kuruluyor..." -ForegroundColor Yellow
    
    $RustupUrl = "https://win.rustup.rs/x86_64"
    $RustupPath = "$env:TEMP\rustup-init.exe"
    
    try {
        Invoke-WebRequest -Uri $RustupUrl -OutFile $RustupPath -UseBasicParsing
        Start-Process -FilePath $RustupPath -ArgumentList "-y" -Wait -NoNewWindow
        Remove-Item $RustupPath -Force -ErrorAction SilentlyContinue
        
        # Refresh PATH
        $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "User") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "Machine")
        
        Write-Host "✅ Rust kuruldu" -ForegroundColor Green
    } catch {
        Write-Host "❌ Rust kurulumu başarısız: $_" -ForegroundColor Red
        Write-Host "Lütfen manuel olarak https://rustup.rs adresinden kurun" -ForegroundColor Yellow
        exit 1
    }
}

# Check Git
if (Get-Command git -ErrorAction SilentlyContinue) {
    Write-Host "✅ Git: $(git --version)" -ForegroundColor Green
} else {
    Write-Host "⚠️  Git bulunamadı. https://git-scm.com/download/win adresinden kurun" -ForegroundColor Yellow
    exit 1
}

Write-Host ""

# ─────────────────────────────────────────────────────────────────────────────
# CLONE REPO (if needed)
# ─────────────────────────────────────────────────────────────────────────────

$InstallDir = if ($env:SENTIENT_DIR) { $env:SENTIENT_DIR } else { "$env:USERPROFILE\sentient" }

if (-not (Test-Path $InstallDir)) {
    Write-Host "📥 SENTIENT indiriliyor..." -ForegroundColor White
    git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git $InstallDir
    Write-Host "✅ Depo klonlandı: $InstallDir" -ForegroundColor Green
} else {
    Write-Host "✅ Depo mevcut: $InstallDir" -ForegroundColor Green
    Set-Location $InstallDir
    Write-Host "📥 Güncellemeler kontrol ediliyor..." -ForegroundColor White
    git pull
}

Set-Location $InstallDir

# ─────────────────────────────────────────────────────────────────────────────
# BUILD TUI WIZARD
# ─────────────────────────────────────────────────────────────────────────────

Write-Host ""
Write-Host "🔨 TUI Sihirbazı derleniyor..." -ForegroundColor White
Write-Host "   (Bu işlem birkaç dakika sürebilir...)" -ForegroundColor Gray
Write-Host ""

$CargoPath = "$env:USERPROFILE\.cargo\bin\cargo.exe"

if (Test-Path $CargoPath) {
    # Show errors for debugging
    & $CargoPath build --release --bin sentient-setup
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "✅ TUI Sihirbazı derlendi!" -ForegroundColor Green
    } else {
        Write-Host ""
        Write-Host "⚠️  Derleme başarısız oldu!" -ForegroundColor Yellow
        Write-Host ""
        Write-Host "Alternatif kurulum yöntemleri:" -ForegroundColor Cyan
        Write-Host ""
        Write-Host "1. Docker ile:" -ForegroundColor White
        Write-Host "   docker run -it ghcr.io/nexsusagent-coder/sentient:latest" -ForegroundColor Gray
        Write-Host ""
        Write-Host "2. Binary indir:" -ForegroundColor White
        Write-Host "   https://github.com/nexsusagent-coder/SENTIENT_CORE/releases" -ForegroundColor Gray
        Write-Host ""
        Write-Host "3. Manuel derleme:" -ForegroundColor White
        Write-Host "   cargo install sentient-cli" -ForegroundColor Gray
        Write-Host ""
        exit 1
    }
} else {
    Write-Host "❌ Cargo bulunamadı. Rust'ı kurun: https://rustup.rs" -ForegroundColor Red
    exit 1
}

# ─────────────────────────────────────────────────────────────────────────────
# RUN TUI WIZARD
# ─────────────────────────────────────────────────────────────────────────────

Write-Host ""
Write-Host "🎮 Interactive TUI Sihirbazı başlatılıyor..." -ForegroundColor Cyan
Write-Host "   ↑↓ Ok tuşlarıyla gezinin" -ForegroundColor Cyan
Write-Host "   Space ile çoklu seçim yapın" -ForegroundColor Cyan
Write-Host "   Enter ile onaylayın" -ForegroundColor Cyan
Write-Host ""

Start-Sleep -Seconds 1

# Run the interactive TUI wizard
$SetupExe = Join-Path $InstallDir "target\release\sentient-setup.exe"
if (Test-Path $SetupExe) {
    & $SetupExe
} else {
    Write-Host "⚠️  Setup executable bulunamadı" -ForegroundColor Yellow
}

# ─────────────────────────────────────────────────────────────────────────────
# DONE
# ─────────────────────────────────────────────────────────────────────────────

Write-Host ""
Write-Host "╔══════════════════════════════════════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║  🎉 SENTIENT kurulumu tamamlandı!                                            ║" -ForegroundColor Green
Write-Host "╚══════════════════════════════════════════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""

Write-Host "🚀 Başlatmak için:" -ForegroundColor White
Write-Host "   cd $InstallDir" -ForegroundColor Cyan
Write-Host "   .\target\release\sentient-shell.exe" -ForegroundColor Cyan
Write-Host ""

Write-Host "🌐 Dashboard:" -ForegroundColor White
Write-Host "   http://localhost:8080" -ForegroundColor Cyan
Write-Host ""
