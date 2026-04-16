# SENTIENT OS Installer for Windows
# Usage: iwr -useb https://sentientos.ai/install.ps1 | iex

param(
    [string]$InstallDir = "",
    [switch]$NoOnboard,
    [switch]$Uninstall
)

$ErrorActionPreference = "Stop"

# Colors
$ACCENT = "`e[38;2;0;229;204m"
$SUCCESS = "`e[38;2;0;229;204m"
$WARN = "`e[38;2;255;176;32m"
$ERROR = "`e[38;2;230;57;70m"
$MUTED = "`e[38;2;90;100;128m"
$NC = "`e[0m"

function Write-Step { param($msg) Write-Host "${ACCENT}►${NC} $msg" }
function Write-OK { param($msg) Write-Host "${SUCCESS}✓${NC} $msg" }
function Write-Warn { param($msg) Write-Host "${WARN}!${NC} $msg" }
function Write-Err { param($msg) Write-Host "${ERROR}✗${NC} $msg" }
function Write-Info { param($msg) Write-Host "${MUTED}·${NC} $msg" }

function Write-Banner {
    Clear-Host
    Write-Host ""
    Write-Host "${ACCENT}  ███████╗███████╗███╗   ██╗████████╗███╗   ██╗███████╗██╗${NC}"
    Write-Host "${ACCENT}  ██╔════╝██╔════╝████╗  ██║╚══██╔══╝████╗  ██║██╔════╝██║${NC}"
    Write-Host "${ACCENT}  ███████╗█████╗  ██╔██╗ ██║   ██║   ██╔██╗ ██║███████╗██║${NC}"
    Write-Host "${ACCENT}  ╚════██║██╔══╝  ██║╚██╗██║   ██║   ██║╚██╗██║╚════██║██║${NC}"
    Write-Host "${ACCENT}  ███████║███████╗██║ ╚████║   ██║   ██║ ╚████║███████║██║${NC}"
    Write-Host "${ACCENT}  ╚══════╝╚══════╝╚═╝  ╚═══╝   ╚═╝   ╚═╝  ╚═══╝╚══════╝╚═╝${NC}"
    Write-Host ""
    Write-Host "${MUTED}  OS - The Operating System That Thinks${NC}"
    Write-Host ""
}

function Test-Command {
    param($cmd)
    return [bool](Get-Command $cmd -ErrorAction SilentlyContinue)
}

function Get-RustVersion {
    try { return (rustc --version 2>$null).Split()[1] } catch { return $null }
}

function Get-GitVersion {
    try { return (git --version 2>$null).Split()[2] } catch { return $null }
}

function Get-CargoVersion {
    try { return (cargo --version 2>$null).Split()[1] } catch { return $null }
}

function Install-Rust {
    Write-Info "Rust not found - installing..."
    
    # Download rustup-init
    $rustupUrl = "https://win.rustup.rs/x86_64"
    $rustupExe = "$env:TEMP\rustup-init.exe"
    
    try {
        Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupExe -UseBasicParsing
        & $rustupExe -y 2>&1 | Out-Null
        
        # Refresh PATH
        $env:Path = [System.Environment]::GetEnvironmentVariable("Path","User") + ";" + [System.Environment]::GetEnvironmentVariable("Path","Machine")
        
        Write-OK "Rust installed"
        return $true
    } catch {
        Write-Err "Failed to install Rust: $_"
        Write-Info "Install manually from: https://rustup.rs"
        return $false
    }
}

function Install-Git {
    Write-Info "Git not found - installing..."
    
    if (Test-Command winget) {
        try {
            winget install Git.Git --accept-package-agreements --accept-source-agreements 2>&1 | Out-Null
            $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
            Write-OK "Git installed"
            return $true
        } catch { }
    }
    
    Write-Err "Could not install Git automatically"
    Write-Info "Install from: https://git-scm.com"
    return $false
}

function Install-BuildTools {
    Write-Info "Checking Visual Studio Build Tools..."
    
    $vsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
    $hasBuildTools = if (Test-Path $vsWhere) {
        & $vsWhere -latest -requires Microsoft.VisualStudio.Workload.VCTools 2>$null
    } else { $false }
    
    if (-not $hasBuildTools) {
        Write-Info "Installing Build Tools (this may take a few minutes)..."
        if (Test-Command winget) {
            try {
                winget install Microsoft.VisualStudio.2022.BuildTools --override "--add Microsoft.VisualStudio.Workload.VCTools --passive" --accept-source-agreements 2>&1 | Out-Null
                Write-OK "Build Tools installed"
            } catch {
                Write-Warn "Build Tools installation may need manual intervention"
            }
        }
    } else {
        Write-OK "Build Tools already installed"
    }
    return $true
}

function Install-Sentient {
    param($TargetDir)
    
    Write-Step "Cloning SENTIENT OS..."
    
    # Clone
    if (Test-Path "$TargetDir\.git") {
        Write-Info "Updating existing installation..."
        git -C $TargetDir pull 2>&1 | Out-Null
    } else {
        if (Test-Path $TargetDir) {
            Remove-Item $TargetDir -Recurse -Force
        }
        git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git $TargetDir 2>&1 | Out-Null
    }
    Write-OK "Repository ready"
    
    # Build
    Write-Step "Building SENTIENT OS (5-15 minutes)..."
    Set-Location $TargetDir
    
    $env:PYTHON_SYS_EXECUTABLE = if (Test-Command python) { (Get-Command python).Source } else { "" }
    
    # Sadece CLI crate'ini derle (tüm workspace değil!)
    cargo build --release -p sentient_cli 2>&1 | ForEach-Object {
        if ($_ -match "Compiling|Building|Finished") {
            Write-Info $_
        }
    }
    
    if (-not (Test-Path "target\release\sentient.exe")) {
        Write-Err "Build failed!"
        return $false
    }
    
    $size = [math]::Round((Get-Item "target\release\sentient.exe").Length / 1MB, 1)
    Write-OK "Built successfully ($size MB)"
    
    # Create .env if not exists
    if (-not (Test-Path ".env")) {
        Write-Step "Creating configuration..."
        
        @"
# SENTIENT OS Configuration
# Generated: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')

# Default: Use Ollama (local, free)
OLLAMA_HOST=http://localhost:11434
DEFAULT_MODEL=ollama/gemma3:12b

# Alternative providers (uncomment and add your API key):
# OPENROUTER_API_KEY=sk-or-xxx
# OPENAI_API_KEY=sk-xxx
# ANTHROPIC_API_KEY=sk-ant-xxx
# DEEPSEEK_API_KEY=sk-xxx
# GROQ_API_KEY=gsk_xxx
# GOOGLE_AI_API_KEY=xxx

# To switch provider, change DEFAULT_MODEL to:
# DEFAULT_MODEL=openrouter/anthropic/claude-4-sonnet
# DEFAULT_MODEL=openai/gpt-4o
# DEFAULT_MODEL=anthropic/claude-4-sonnet
# DEFAULT_MODEL=deepseek/deepseek-chat
# DEFAULT_MODEL=groq/llama-3.3-70b-versatile
# DEFAULT_MODEL=google/gemini-2.0-flash

RUST_LOG=info
"@ | Out-File -FilePath ".env" -Encoding UTF8
        Write-OK "Configuration created"
    }
    
    # Add to PATH
    $binPath = "$TargetDir\target\release"
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($currentPath -notlike "*$binPath*") {
        [Environment]::SetEnvironmentVariable("Path", "$currentPath;$binPath", "User")
        Write-OK "Added to PATH"
    }
    
    return $true
}

function Uninstall-Sentient {
    param($TargetDir)
    
    Write-Banner
    Write-Host "  Uninstalling SENTIENT OS..."
    Write-Host ""
    
    if (Test-Path $TargetDir) {
        Remove-Item $TargetDir -Recurse -Force
        Write-OK "Removed $TargetDir"
    }
    
    # Remove from PATH
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $newPath = ($currentPath -split ';' | Where-Object { $_ -notlike "*sentient*" }) -join ';'
    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
    Write-OK "Removed from PATH"
    
    Write-Host ""
    Write-OK "SENTIENT OS uninstalled"
}

# ═══════════════════════════════════════════════════════════════════════════════
# MAIN
# ═══════════════════════════════════════════════════════════════════════════════

if ($Uninstall) {
    $dir = if ($InstallDir) { $InstallDir } else { "$env:USERPROFILE\.sentient" }
    Uninstall-Sentient -TargetDir $dir
    exit 0
}

Write-Banner

# Determine install directory
$installDir = if ($InstallDir) { $InstallDir } else { "$env:USERPROFILE\.sentient" }
Write-Info "Installing to: $installDir"
Write-Host ""

# Check prerequisites
Write-Step "Checking prerequisites..."

# Rust
$rustVersion = Get-RustVersion
if ($rustVersion) {
    Write-OK "Rust $rustVersion"
} else {
    if (-not (Install-Rust)) { exit 1 }
}

# Cargo
$cargoVersion = Get-CargoVersion
if ($cargoVersion) {
    Write-OK "Cargo $cargoVersion"
} else {
    Write-Err "Cargo not found after Rust installation"
    Write-Info "Please restart your terminal and try again"
    exit 1
}

# Git
$gitVersion = Get-GitVersion
if ($gitVersion) {
    Write-OK "Git $gitVersion"
} else {
    if (-not (Install-Git)) { exit 1 }
}

# Build Tools
Install-BuildTools | Out-Null

Write-Host ""

# Install
if (-not (Install-Sentient -TargetDir $installDir)) {
    Write-Host ""
    Write-Err "Installation failed"
    exit 1
}

Write-Host ""
Write-Host "${ACCENT}════════════════════════════════════════════════════════════════${NC}"
Write-Host "${SUCCESS}  ✓ SENTIENT OS installed successfully!${NC}"
Write-Host "${ACCENT}════════════════════════════════════════════════════════════════${NC}"
Write-Host ""
Write-Host "  ${MUTED}Quick Start:${NC}"
Write-Host ""
Write-Host "    ${ACCENT}sentient --version${NC}"
Write-Host "    ${ACCENT}sentient chat${NC}"
Write-Host ""
Write-Host "  ${MUTED}To use Ollama (free, local):${NC}"
Write-Host "    ${ACCENT}ollama pull gemma3:12b${NC}"
Write-Host "    ${ACCENT}sentient chat${NC}"
Write-Host ""
Write-Host "  ${MUTED}To use cloud AI (OpenRouter, OpenAI, etc.):${NC}"
Write-Host "    ${ACCENT}notepad $installDir\.env${NC}"
Write-Host "    ${MUTED}# Uncomment your preferred provider and add API key${NC}"
Write-Host ""
Write-Host "  ${MUTED}Documentation: https://github.com/nexsusagent-coder/SENTIENT_CORE${NC}"
Write-Host ""
