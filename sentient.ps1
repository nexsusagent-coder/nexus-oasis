#!/usr/bin/env pwsh
# ╔════════════════════════════════════════════════════════════════════════════════╗
# ║  SENTIENT NEXUS OS - Global Launcher v7.0.0                                   ║
# ║  Professional AI Agent Framework                                              ║
# ║  PowerShell Launcher                                                          ║
# ╚════════════════════════════════════════════════════════════════════════════════╝

param(
    [Parameter(Position=0)]
    [string]$Command = "dashboard"
)

# Colors
$Red = "`e[31m"
$Green = "`e[32m"
$Cyan = "`e[36m"
$Yellow = "`e[33m"
$NC = "`e[0m"

# Find installation directory
$SentientHome = if ($env:SENTIENT_HOME) { $env:SENTIENT_HOME } else { Join-Path $env:USERPROFILE ".sentient" }
$InstallDir = $null

if (Test-Path "C:\opt\sentient") {
    $InstallDir = "C:\opt\sentient"
} elseif (Test-Path $SentientHome) {
    $InstallDir = $SentientHome
} elseif (Test-Path (Join-Path $PSScriptRoot "Cargo.toml")) {
    $InstallDir = $PSScriptRoot
}

if (-not $InstallDir) {
    Write-Host "${Red}ERROR: SENTIENT installation not found.${NC}" -ForegroundColor Red
    Write-Host "Please run setup.ps1 first to install SENTIENT."
    exit 1
}

Set-Location $InstallDir

function Show-Help {
    Write-Host "${Cyan}SENTIENT NEXUS OS - Global Launcher${NC}" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Usage: sentient [command]"
    Write-Host ""
    Write-Host "Commands:"
    Write-Host "  dashboard, ui    Launch the web dashboard (default)"
    Write-Host "  shell, cli       Launch the interactive CLI"
    Write-Host "  setup            Run the setup wizard"
    Write-Host "  config           Open configuration"
    Write-Host "  status           Show system status"
    Write-Host "  update           Update to latest version"
    Write-Host "  logs             View system logs"
    Write-Host "  help, --help     Show this help message"
    Write-Host ""
    Write-Host "Examples:"
    Write-Host "  sentient              # Launch dashboard"
    Write-Host "  sentient shell        # Launch CLI"
    Write-Host "  sentient setup        # Reconfigure"
    Write-Host ""
}

function Show-Status {
    Write-Host "${Cyan}SENTIENT System Status${NC}" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Installation: $InstallDir"
    
    # Check if dashboard is running
    $dashboard = Get-Process -Name "sentient-dashboard" -ErrorAction SilentlyContinue
    if ($dashboard) {
        Write-Host "Dashboard: ${Green}Running${NC}"
    } else {
        Write-Host "Dashboard: ${Yellow}Not running${NC}"
    }
    
    # Check config
    $configPath = Join-Path $env:APPDATA "sentient\config.toml"
    if (Test-Path $configPath) {
        Write-Host "Config: ${Green}Configured${NC}"
    } else {
        Write-Host "Config: ${Yellow}Not configured (run 'sentient setup')${NC}"
    }
    
    # Check Ollama
    $ollama = Get-Command ollama -ErrorAction SilentlyContinue
    if ($ollama) {
        Write-Host "Ollama: ${Green}Installed${NC}"
    } else {
        Write-Host "Ollama: ${Yellow}Not installed${NC}"
    }
    Write-Host ""
}

# Main execution
switch ($Command) {
    { $_ -in @("dashboard", "ui", "") } {
        Write-Host "${Cyan}Starting SENTIENT Dashboard...${NC}" -ForegroundColor Cyan
        Write-Host ""
        
        $releaseBin = Join-Path $InstallDir "target\release\sentient-dashboard.exe"
        $debugBin = Join-Path $InstallDir "target\debug\sentient-dashboard.exe"
        $dashboardDir = Join-Path $InstallDir "dashboard"
        
        if (Test-Path $releaseBin) {
            & $releaseBin
        } elseif (Test-Path $debugBin) {
            & $debugBin
        } elseif (Test-Path (Join-Path $dashboardDir "package.json")) {
            Set-Location $dashboardDir
            if (Get-Command npm -ErrorAction SilentlyContinue) {
                npm run dev 2>$null
                if ($LASTEXITCODE -ne 0) { npm start }
            } else {
                Write-Host "${Red}ERROR: npm not found. Please install Node.js.${NC}" -ForegroundColor Red
                exit 1
            }
        } elseif (Test-Path (Join-Path $InstallDir "start-dashboard.bat")) {
            & (Join-Path $InstallDir "start-dashboard.bat")
        } else {
            Write-Host "${Red}ERROR: No dashboard binary found.${NC}" -ForegroundColor Red
            Write-Host "Please build the project first: cargo build --release"
            exit 1
        }
    }
    
    { $_ -in @("shell", "cli", "repl") } {
        Write-Host "${Cyan}Starting SENTIENT Shell...${NC}" -ForegroundColor Cyan
        Write-Host ""
        
        $releaseBin = Join-Path $InstallDir "target\release\sentient-shell.exe"
        $debugBin = Join-Path $InstallDir "target\debug\sentient-shell.exe"
        
        if (Test-Path $releaseBin) {
            & $releaseBin
        } elseif (Test-Path $debugBin) {
            & $debugBin
        } else {
            Write-Host "${Red}ERROR: sentient-shell binary not found.${NC}" -ForegroundColor Red
            Write-Host "Please build the project first: cargo build --release"
            exit 1
        }
    }
    
    { $_ -in @("setup", "configure", "init") } {
        Write-Host "${Cyan}Starting SENTIENT Setup Wizard...${NC}" -ForegroundColor Cyan
        Write-Host ""
        
        $releaseBin = Join-Path $InstallDir "target\release\sentient-setup.exe"
        $debugBin = Join-Path $InstallDir "target\debug\sentient-setup.exe"
        $setupScript = Join-Path $InstallDir "setup.ps1"
        
        if (Test-Path $releaseBin) {
            & $releaseBin
        } elseif (Test-Path $debugBin) {
            & $debugBin
        } elseif (Test-Path $setupScript) {
            & $setupScript
        } else {
            Write-Host "${Red}ERROR: Setup wizard not found.${NC}" -ForegroundColor Red
            exit 1
        }
    }
    
    "config" {
        $configPath = Join-Path $env:APPDATA "sentient\config.toml"
        if (Test-Path $configPath) {
            if ($env:EDITOR) {
                & $env:EDITOR $configPath
            } else {
                notepad $configPath
            }
        } else {
            Write-Host "${Yellow}Config file not found. Run 'sentient setup' first.${NC}" -ForegroundColor Yellow
        }
    }
    
    "status" {
        Show-Status
    }
    
    { $_ -in @("update", "upgrade") } {
        Write-Host "${Cyan}Updating SENTIENT...${NC}" -ForegroundColor Cyan
        Set-Location $InstallDir
        if (Test-Path (Join-Path $InstallDir ".git")) {
            git pull origin main
            cargo build --release
            Write-Host "${Green}Update complete!${NC}" -ForegroundColor Green
        } else {
            Write-Host "${Yellow}Not a git repository. Manual update required.${NC}" -ForegroundColor Yellow
        }
    }
    
    "logs" {
        $logDir = Join-Path $env:LOCALAPPDATA "sentient\logs"
        if (Test-Path $logDir) {
            Get-ChildItem $logDir
            $logFile = Read-Host "Enter log file to view"
            $logPath = Join-Path $logDir $logFile
            if (Test-Path $logPath) {
                Get-Content $logPath
            }
        } else {
            Write-Host "${Yellow}No logs found.${NC}" -ForegroundColor Yellow
        }
    }
    
    { $_ -in @("help", "--help", "-h") } {
        Show-Help
    }
    
    default {
        Write-Host "${Red}Unknown command: $Command${NC}" -ForegroundColor Red
        Write-Host ""
        Show-Help
        exit 1
    }
}
