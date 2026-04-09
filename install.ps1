# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT - Windows Quick Install Script
#  https://get.sentient.ai/ps
# ═══════════════════════════════════════════════════════════════════════════════
#
#  Usage (PowerShell):
#    irm https://get.sentient.ai/ps | iex
#    irm https://get.sentient.ai/ps | iex -Version "4.0.0"
#
#  Options:
#    -Version VERSION    Specify version (default: latest)
#    -Prefix PATH        Install directory (default: C:\Users\You\.sentient)
#    -NoConfirm          Skip confirmation prompts
#    -Uninstall          Remove SENTIENT
# ═══════════════════════════════════════════════════════════════════════════════

param(
    [string]$Version = "latest",
    [string]$Prefix = "$env:USERPROFILE\.sentient",
    [switch]$NoConfirm,
    [switch]$Uninstall
)

$ErrorActionPreference = "Stop"
$Repo = "nexsusagent-coder/SENTIENT_CORE"

# Colors
function Write-ColorOutput($ForegroundColor) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    if ($args) {
        Write-Output $args
    }
    $host.UI.RawUI.ForegroundColor = $fc
}

# ═══════════════════════════════════════════════════════════════════════════════
#  UNINSTALL
# ═══════════════════════════════════════════════════════════════════════════════
if ($Uninstall) {
    Write-ColorOutput Yellow "Uninstalling SENTIENT..."
    
    # Remove from PATH
    $path = [Environment]::GetEnvironmentVariable("PATH", "User")
    $newPath = ($path -split ';' | Where-Object { $_ -notlike "*sentient*" }) -join ';'
    [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
    
    # Remove directory
    if (Test-Path $Prefix) {
        Remove-Item -Recurse -Force $Prefix
    }
    
    Write-ColorOutput Green "✓ SENTIENT uninstalled"
    exit 0
}

# ═══════════════════════════════════════════════════════════════════════════════
#  DETECT ARCHITECTURE
# ═══════════════════════════════════════════════════════════════════════════════
$Arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "x86" }

# ═══════════════════════════════════════════════════════════════════════════════
#  BANNER
# ═══════════════════════════════════════════════════════════════════════════════
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
  ║          NEXUS OASIS — AI Operating System                 ║
  ╚════════════════════════════════════════════════════════════╝
"@
Write-Output ""

# ═══════════════════════════════════════════════════════════════════════════════
#  GET LATEST VERSION
# ═══════════════════════════════════════════════════════════════════════════════
if ($Version -eq "latest") {
    Write-ColorOutput Cyan "🔍  Fetching latest version..."
    try {
        $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
        $Version = $release.tag_name -replace '^v', ''
    } catch {
        Write-ColorOutput Yellow "⚠ Could not fetch latest version, using default"
        $Version = "4.0.0"
    }
}

Write-ColorOutput Green "📦  Version: $Version"
Write-ColorOutput Blue "🖥️  Platform: Windows-$Arch"
Write-ColorOutput Magenta "📁  Install location: $Prefix"
Write-Output ""

# ═══════════════════════════════════════════════════════════════════════════════
#  CONFIRM
# ═══════════════════════════════════════════════════════════════════════════════
if (-not $NoConfirm) {
    Write-ColorOutput Yellow "Continue with installation? [Y/n]"
    $confirm = Read-Host
    if ($confirm -eq "n" -or $confirm -eq "N") {
        Write-Output "Installation cancelled."
        exit 0
    }
}

# ═══════════════════════════════════════════════════════════════════════════════
#  CREATE DIRECTORIES
# ═══════════════════════════════════════════════════════════════════════════════
Write-Output ""
Write-ColorOutput Cyan "📁  Creating directories..."
New-Item -ItemType Directory -Force -Path "$Prefix\bin" | Out-Null
New-Item -ItemType Directory -Force -Path "$Prefix\data" | Out-Null
New-Item -ItemType Directory -Force -Path "$Prefix\config" | Out-Null
New-Item -ItemType Directory -Force -Path "$Prefix\logs" | Out-Null

# ═══════════════════════════════════════════════════════════════════════════════
#  DOWNLOAD BINARY
# ═══════════════════════════════════════════════════════════════════════════════
$DownloadUrl = "https://github.com/$Repo/releases/download/v$Version/sentient-windows-$Arch.zip"
$TempFile = "$env:TEMP\sentient-$Version.zip"

Write-ColorOutput Cyan "📥  Downloading SENTIENT v$Version..."
Write-Output "    $DownloadUrl"

try {
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempFile -UseBasicParsing
} catch {
    Write-ColorOutput Red "❌  Download failed!"
    Write-Output ""
    Write-Output "Possible reasons:"
    Write-Output "  1. Version $Version doesn't exist"
    Write-Output "  2. Binary for Windows-$Arch not available"
    Write-Output ""
    Write-Output "Available versions: https://github.com/$Repo/releases"
    exit 1
}

# ═══════════════════════════════════════════════════════════════════════════════
#  EXTRACT
# ═══════════════════════════════════════════════════════════════════════════════
Write-ColorOutput Cyan "📦  Extracting..."
Expand-Archive -Path $TempFile -DestinationPath "$Prefix\bin" -Force
Remove-Item $TempFile

# ═══════════════════════════════════════════════════════════════════════════════
#  ADD TO PATH
# ═══════════════════════════════════════════════════════════════════════════════
Write-ColorOutput Cyan "🔧  Adding to PATH..."

$path = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($path -notlike "*$Prefix\bin*") {
    [Environment]::SetEnvironmentVariable("PATH", "$path;$Prefix\bin", "User")
    Write-ColorOutput Green "✓  Added to PATH"
} else {
    Write-ColorOutput Yellow "⚠  Already in PATH"
}

# ═══════════════════════════════════════════════════════════════════════════════
#  VERIFY INSTALLATION
# ═══════════════════════════════════════════════════════════════════════════════
Write-Output ""
Write-ColorOutput Cyan "🔍  Verifying installation..."

$sentientExe = "$Prefix\bin\sentient.exe"
if (Test-Path $sentientExe) {
    $installedVersion = & $sentientExe --version 2>$null
    if (-not $installedVersion) { $installedVersion = "v$Version" }
    Write-ColorOutput Green "✓  SENTIENT $installedVersion installed successfully!"
} else {
    Write-ColorOutput Red "❌  Installation failed - binary not found"
    exit 1
}

# ═══════════════════════════════════════════════════════════════════════════════
#  NEXT STEPS
# ═══════════════════════════════════════════════════════════════════════════════
Write-Output ""
Write-ColorOutput Green "══════════════════════════════════════════════════════════════"
Write-ColorOutput Green "  🎉  INSTALLATION COMPLETE!"
Write-ColorOutput Green "══════════════════════════════════════════════════════════════"
Write-Output ""
Write-ColorOutput Yellow "Next steps:"
Write-Output ""
Write-Output "  1. Restart your terminal or run:"
Write-Output "     `$env:PATH += ';$Prefix\bin'"
Write-Output ""
Write-Output "  2. Run setup wizard:"
Write-Output "     sentient setup"
Write-Output ""
Write-Output "  3. Start interactive REPL:"
Write-Output "     sentient repl"
Write-Output ""
Write-Output "  4. Run autonomous agent:"
Write-Output '     sentient agent --goal "Your task description"'
Write-Output ""
Write-ColorOutput Blue "Documentation: https://github.com/$Repo"
Write-Output ""
