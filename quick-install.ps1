#!/usr/bin/env pwsh
# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT NEXUS OS - Quick Install Script v1.0.0
#  One-command installation: irm https://get.sentient.ai | iex
# ═══════════════════════════════════════════════════════════════════════════════

$ErrorActionPreference = "Continue"

# Colors
$Red = "`e[31m"
$Green = "`e[32m"
$Yellow = "`e[33m"
$Cyan = "`e[36m"
$Bold = "`e[1m"
$NC = "`e[0m"

# Installation directory
$InstallDir = if ($env:SENTIENT_HOME) { $env:SENTIENT_HOME } else { Join-Path $env:USERPROFILE ".sentient" }
$RepoUrl = "https://github.com/nexsusagent-coder/SENTIENT_CORE.git"

function Print-Banner {
    Write-Host ""
    Write-Host "  ╔═══════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "  ║                                                               ║" -ForegroundColor Cyan
    Write-Host "  ║   ███████╗██╗███████╗███╗   ██╗██████╗ ███████╗██████╗        ║" -ForegroundColor Cyan
    Write-Host "  ║   ██╔════╝██║██╔════╝████╗  ██║██╔══██╗██╔════╝██╔══██╗       ║" -ForegroundColor Cyan
    Write-Host "  ║   ███████╗██║█████╗  ██╔██╗ ██║██║  ██║█████╗  ██████╔╝       ║" -ForegroundColor Cyan
    Write-Host "  ║   ╚════██║██║██╔══╝  ██║╚██╗██║██║  ██║██╔══╝  ██╔══██╗       ║" -ForegroundColor Cyan
    Write-Host "  ║   ███████║██║███████╗██║ ╚████║██████╔╝███████╗██║  ██║       ║" -ForegroundColor Cyan
    Write-Host "  ║   ╚══════╝╚═╝╚══════╝╚═╝  ╚═══╝╚═════╝ ╚══════╝╚═╝  ╚═╝       ║" -ForegroundColor Cyan
    Write-Host "  ║                                                               ║" -ForegroundColor Cyan
    Write-Host "  ║              SENTIENT NEXUS OS v7.0.0                        ║" -ForegroundColor Cyan
    Write-Host "  ║              Professional AI Agent Framework                 ║" -ForegroundColor Cyan
    Write-Host "  ║                                                               ║" -ForegroundColor Cyan
    Write-Host "  ╚═══════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
    Write-Host ""
}

function Install-Rust {
    $CargoPath = Join-Path $env:USERPROFILE ".cargo\bin\cargo.exe"
    
    if (Test-Path $CargoPath) {
        $Version = & "$env:USERPROFILE\.cargo\bin\rustc.exe" --version 2>$null
        Write-Host "[OK] Rust: $Version" -ForegroundColor Green
        return
    }
    
    Write-Host "[...] Installing Rust..." -ForegroundColor Yellow
    
    $RustupUrl = "https://win.rustup.rs/x86_64"
    $RustupPath = Join-Path $env:TEMP "rustup-init.exe"
    
    try {
        Invoke-WebRequest -Uri $RustupUrl -OutFile $RustupPath -UseBasicParsing
        Start-Process -FilePath $RustupPath -ArgumentList "-y" -Wait -NoNewWindow
        Remove-Item $RustupPath -Force -ErrorAction SilentlyContinue
        
        # Refresh PATH
        $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "User") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "Machine")
        
        Write-Host "[OK] Rust installed" -ForegroundColor Green
    } catch {
        Write-Host "[ERR] Rust installation failed: $_" -ForegroundColor Red
        Write-Host "Install manually: https://rustup.rs" -ForegroundColor Yellow
        exit 1
    }
}

function Install-Git {
    if (Get-Command git -ErrorAction SilentlyContinue) {
        $Version = git --version
        Write-Host "[OK] Git: $Version" -ForegroundColor Green
        return
    }
    
    Write-Host "[ERR] Git not found. Install from: https://git-scm.com/download/win" -ForegroundColor Red
    exit 1
}

function Clone-Repo {
    if (Test-Path $InstallDir) {
        Write-Host "[...] Updating existing installation..." -ForegroundColor Yellow
        Set-Location $InstallDir
        git pull origin main 2>$null
    } else {
        Write-Host "[...] Cloning SENTIENT..." -ForegroundColor Yellow
        git clone $RepoUrl $InstallDir
    }
    
    Write-Host "[OK] Repository ready: $InstallDir" -ForegroundColor Green
}

function Build-Project {
    Set-Location $InstallDir
    
    Write-Host "[...] Building SENTIENT (this may take a few minutes)..." -ForegroundColor Yellow
    Write-Host ""
    
    $CargoPath = Join-Path $env:USERPROFILE ".cargo\bin\cargo.exe"
    
    if (-not (Test-Path $CargoPath)) {
        $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "User") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "Machine")
        $CargoPath = "cargo"
    }
    
    & $CargoPath build --release --bin sentient-setup --bin sentient-shell 2>&1 | ForEach-Object {
        if ($_ -match "Compiling") {
            Write-Host "`r   $_                              " -NoNewline -ForegroundColor Cyan
        }
    }
    
    Write-Host ""
    Write-Host "[OK] Build complete" -ForegroundColor Green
}

function Create-Launcher {
    $LauncherDir = Join-Path $env:LOCALAPPDATA "sentient"
    $LauncherPath = Join-Path $LauncherDir "sentient.ps1"
    
    New-Item -ItemType Directory -Force -Path $LauncherDir | Out-Null
    
    $LauncherContent = @'
param([string]$Command = "dashboard")

$SentientDir = if ($env:SENTIENT_HOME) { $env:SENTIENT_HOME } else { Join-Path $env:USERPROFILE ".sentient" }
$SentientBin = Join-Path $SentientDir "target\release"

switch ($Command) {
    { $_ -in @("dashboard", "ui", "") } {
        $Shell = Join-Path $SentientBin "sentient-shell.exe"
        if (Test-Path $Shell) { & $Shell } else { Write-Host "Run: sentient setup" -ForegroundColor Yellow }
    }
    "setup" {
        $Setup = Join-Path $SentientBin "sentient-setup.exe"
        if (Test-Path $Setup) { & $Setup } else { Write-Host "Build required" -ForegroundColor Yellow }
    }
    "status" {
        Write-Host "SENTIENT: $SentientDir"
        $Shell = Join-Path $SentientBin "sentient-shell.exe"
        if (Test-Path $Shell) { Write-Host "  Status: Installed" -ForegroundColor Green } else { Write-Host "  Status: Not built" -ForegroundColor Yellow }
    }
    default { Write-Host "Usage: sentient [dashboard|setup|status]" }
}
'@
    
    Set-Content -Path $LauncherPath -Value $LauncherContent -Force
    
    # Add to PATH
    $UserPath = [System.Environment]::GetEnvironmentVariable("Path", "User")
    if ($UserPath -notlike "*$LauncherDir*") {
        [System.Environment]::SetEnvironmentVariable("Path", "$UserPath;$LauncherDir", "User")
    }
    
    Write-Host "[OK] Launcher created: $LauncherPath" -ForegroundColor Green
}

# Main
Clear-Host
Print-Banner

Write-Host "Installation starting..." -ForegroundColor Bold
Write-Host ""

Install-Git
Install-Rust
Clone-Repo
Build-Project
Create-Launcher

Write-Host ""
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Green
Write-Host "  INSTALLATION COMPLETE!" -ForegroundColor Green
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Green
Write-Host ""
Write-Host "  Start:    sentient" -ForegroundColor White
Write-Host "  Setup:    sentient setup" -ForegroundColor White
Write-Host "  Status:   sentient status" -ForegroundColor White
Write-Host ""
Write-Host "  Restart your terminal, then run: sentient setup" -ForegroundColor Cyan
Write-Host ""
