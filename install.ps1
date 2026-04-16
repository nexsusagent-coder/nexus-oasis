# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS - Windows Universal Installer
#  One command: irm https://get.sentient.ai/ps | iex
# ═══════════════════════════════════════════════════════════════════════════════

param(
    [string]$Version = "latest",
    [string]$Prefix = "$env:USERPROFILE\.sentient",
    [switch]$NoConfirm,
    [switch]$Uninstall,
    [switch]$Help
)

$ErrorActionPreference = "Stop"
$Repo = "nexsusagent-coder/SENTIENT_CORE"

# ─────────────────────────────────────────────────────────────────────────────
#  Help
# ─────────────────────────────────────────────────────────────────────────────
if ($Help) {
    Write-Host @"
SENTIENT OS Installer

Usage:
  irm https://get.sentient.ai/ps | iex

Options:
  -Version VERSION    Version to install (default: latest)
  -Prefix PATH        Install directory (default: ~/.sentient)
  -NoConfirm          Skip confirmation
  -Uninstall          Remove SENTIENT

Examples:
  irm https://get.sentient.ai/ps | iex
  irm https://get.sentient.ai/ps | iex -Version "v4.0.0"
  irm https://get.sentient.ai/ps | iex -Uninstall
"@
    exit 0
}

# ─────────────────────────────────────────────────────────────────────────────
#  Colors
# ─────────────────────────────────────────────────────────────────────────────
function Write-ColorOutput {
    param([string]$Color, [string]$Message)
    $colors = @{
        "Red" = "Red"; "Green" = "Green"; "Yellow" = "Yellow"
        "Blue" = "Blue"; "Cyan" = "Cyan"; "White" = "White"
    }
    Write-Host $Message -ForegroundColor $colors[$Color]
}

# ─────────────────────────────────────────────────────────────────────────────
#  Banner
# ─────────────────────────────────────────────────────────────────────────────
function Print-Banner {
    Clear-Host
    Write-ColorOutput Cyan @"
  ╔════════════════════════════════════════════════════════════╗
  ║     █████╗ ███╗   ██╗███████╗██╗      ██████╗ ██╗   ██╗    ║
  ║    ██╔══██╗████╗  ██║██╔════╝██║     ██╔═══██╗██║   ██║    ║
  ║    ███████║██╔██╗ ██║█████╗  ██║     ██║   ██║██║   ██║    ║
  ║    ██╔══██║██║╚██╗██║██╔══╝  ██║     ██║   ██║██║   ██║    ║
  ║    ██║  ██║██║ ╚████║███████╗███████╗╚██████╔╝╚██████╔╝    ║
  ║    ╚═╝  ╚═╝╚═╝  ╚═══╝╚══════╝╚══════╝ ╚═════╝  ╚═════╝     ║
  ║                                                            ║
  ║          SENTIENT OS - AI Operating System                 ║
  ╚════════════════════════════════════════════════════════════╝
"@
    Write-Host ""
}

# ─────────────────────────────────────────────────────────────────────────────
#  Uninstall
# ─────────────────────────────────────────────────────────────────────────────
if ($Uninstall) {
    Write-ColorOutput Yellow "Uninstalling SENTIENT..."
    
    # Remove from PATH
    $userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    $newPath = ($userPath -split ';' | Where-Object { $_ -notlike "*sentient*" }) -join ';'
    [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
    
    # Remove SENTIENT_HOME
    [Environment]::SetEnvironmentVariable("SENTIENT_HOME", $null, "User")
    
    # Remove directory
    if (Test-Path $Prefix) {
        Remove-Item -Recurse -Force $Prefix
    }
    
    Write-ColorOutput Green "SENTIENT uninstalled"
    exit 0
}

# ─────────────────────────────────────────────────────────────────────────────
#  Detect Architecture
# ─────────────────────────────────────────────────────────────────────────────
$Arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }
$Target = "$Arch-pc-windows-msvc"

# ─────────────────────────────────────────────────────────────────────────────
#  Check Dependencies
# ─────────────────────────────────────────────────────────────────────────────
function Check-Dependencies {
    Write-ColorOutput Blue "[INFO] Checking dependencies..."
    
    # Check for Visual C++ Redistributable (required for Rust binaries)
    $vcInstalled = Get-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\VisualStudio\*\VC\Runtimes\x64" -ErrorAction SilentlyContinue
    if (-not $vcInstalled) {
        Write-ColorOutput Yellow "[WARN] Visual C++ Redistributable not found"
        Write-ColorOutput Yellow "       Some features may not work. Install from:"
        Write-ColorOutput Yellow "       https://aka.ms/vs/17/release/vc_redist.x64.exe"
    } else {
        Write-ColorOutput Green "[OK] Visual C++ Redistributable found"
    }
    
    # Check Python (optional)
    $python = Get-Command python -ErrorAction SilentlyContinue
    if ($python) {
        $pyVersion = & python --version 2>&1
        Write-ColorOutput Green "[OK] $pyVersion"
    } else {
        Write-ColorOutput Yellow "[INFO] Python not found (optional for some features)"
    }
}

# ─────────────────────────────────────────────────────────────────────────────
#  Get Latest Version
# ─────────────────────────────────────────────────────────────────────────────
if ($Version -eq "latest") {
    Write-ColorOutput Cyan "[INFO] Fetching latest version..."
    try {
        $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
        $Version = $release.tag_name
    } catch {
        Write-ColorOutput Yellow "[WARN] Could not fetch latest version, using default"
        $Version = "v4.0.0"
    }
}

Write-ColorOutput Green "[INFO] Version: $Version"
Write-ColorOutput Blue "[INFO] Platform: Windows-$Arch"
Write-ColorOutput Magenta "[INFO] Install location: $Prefix"
Write-Host ""

# ─────────────────────────────────────────────────────────────────────────────
#  Confirm
# ─────────────────────────────────────────────────────────────────────────────
if (-not $NoConfirm) {
    Write-ColorOutput Yellow "Continue with installation? [Y/n]"
    $confirm = Read-Host
    if ($confirm -eq "n" -or $confirm -eq "N") {
        Write-Host "Installation cancelled."
        exit 0
    }
}

# ─────────────────────────────────────────────────────────────────────────────
#  Create Directories
# ─────────────────────────────────────────────────────────────────────────────
Write-Host ""
Write-ColorOutput Cyan "[INFO] Creating directories..."
New-Item -ItemType Directory -Force -Path "$Prefix\bin" | Out-Null
New-Item -ItemType Directory -Force -Path "$Prefix\data" | Out-Null
New-Item -ItemType Directory -Force -Path "$Prefix\config" | Out-Null
New-Item -ItemType Directory -Force -Path "$Prefix\logs" | Out-Null

# ─────────────────────────────────────────────────────────────────────────────
#  Download Binary
# ─────────────────────────────────────────────────────────────────────────────
$DownloadUrl = "https://github.com/$Repo/releases/download/$Version/sentient-$Target.zip"
$TempFile = "$env:TEMP\sentient-$Version.zip"

Write-ColorOutput Cyan "[INFO] Downloading SENTIENT $Version..."
Write-Host "       $DownloadUrl"

try {
    # Use progress-free download for speed
    $ProgressPreference = 'SilentlyContinue'
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempFile -UseBasicParsing
    $ProgressPreference = 'Continue'
} catch {
    Write-ColorOutput Red "[ERROR] Download failed!"
    Write-Host ""
    Write-Host "Possible reasons:"
    Write-Host "  1. Version $Version doesn't exist"
    Write-Host "  2. Binary for Windows-$Arch not available"
    Write-Host ""
    Write-Host "Available versions: https://github.com/$Repo/releases"
    exit 1
}

# ─────────────────────────────────────────────────────────────────────────────
#  Extract
# ─────────────────────────────────────────────────────────────────────────────
Write-ColorOutput Cyan "[INFO] Extracting..."
Expand-Archive -Path $TempFile -DestinationPath "$Prefix" -Force
Remove-Item $TempFile -Force

# Move files from bin subdirectory if present
if (Test-Path "$Prefix\bin\bin") {
    Move-Item "$Prefix\bin\bin\*" "$Prefix\bin\" -Force
    Remove-Item "$Prefix\bin\bin" -Force
}

# ─────────────────────────────────────────────────────────────────────────────
#  Add to PATH
# ─────────────────────────────────────────────────────────────────────────────
Write-ColorOutput Cyan "[INFO] Configuring environment..."

# Set SENTIENT_HOME
[Environment]::SetEnvironmentVariable("SENTIENT_HOME", $Prefix, "User")
$env:SENTIENT_HOME = $Prefix

# Add to PATH
$userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($userPath -notlike "*$Prefix\bin*") {
    [Environment]::SetEnvironmentVariable("PATH", "$userPath;$Prefix\bin", "User")
    $env:PATH = "$env:PATH;$Prefix\bin"
    Write-ColorOutput Green "[OK] Added to PATH"
} else {
    Write-ColorOutput Yellow "[WARN] Already in PATH"
}

# ─────────────────────────────────────────────────────────────────────────────
#  Verify Installation
# ─────────────────────────────────────────────────────────────────────────────
Write-Host ""
Write-ColorOutput Cyan "[INFO] Verifying installation..."

$sentientExe = "$Prefix\bin\sentient.exe"
if (Test-Path $sentientExe) {
    $installedVersion = & $sentientExe --version 2>$null
    if (-not $installedVersion) { $installedVersion = $Version }
    Write-ColorOutput Green "[OK] SENTIENT $installedVersion installed!"
} else {
    Write-ColorOutput Red "[ERROR] Installation failed - binary not found"
    exit 1
}

Check-Dependencies

# ─────────────────────────────────────────────────────────────────────────────
#  Next Steps
# ─────────────────────────────────────────────────────────────────────────────
Write-Host ""
Write-ColorOutput Green "══════════════════════════════════════════════════════════════"
Write-ColorOutput Green "  INSTALLATION COMPLETE!"
Write-ColorOutput Green "══════════════════════════════════════════════════════════════"
Write-Host ""
Write-ColorOutput Yellow "Next steps:"
Write-Host ""
Write-Host "  1. Restart your terminal or run:"
Write-Host "     `$env:PATH += ';$Prefix\bin'"
Write-Host ""
Write-Host "  2. Run setup wizard:"
Write-Host "     sentient setup"
Write-Host ""
Write-Host "  3. Start interactive session:"
Write-Host "     sentient"
Write-Host ""
Write-Host "  4. Start web dashboard:"
Write-Host "     sentient-web"
Write-Host ""
Write-ColorOutput Blue "Documentation: https://github.com/$Repo"
Write-Host ""
