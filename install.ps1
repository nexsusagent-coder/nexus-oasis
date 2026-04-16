# ═══════════════════════════════════════════════════════════════════════════════
#  ███████╗███████╗███╗   ██╗████████╗███╗   ██╗███████╗██╗
#  ██╔════╝██╔════╝████╗  ██║╚══██╔══╝████╗  ██║██╔════╝██║
#  ███████╗█████╗  ██╔██╗ ██║   ██║   ██╔██╗ ██║███████╗██║
#  ╚════██║██╔══╝  ██║╚██╗██║   ██║   ██║╚██╗██║╚════██║██║
#  ███████║███████╗██║ ╚████║   ██║   ██║ ╚████║███████║██║
#  ╚══════╝╚══════╝╚═╝  ╚═══╝   ╚═╝   ╚═╝  ╚═══╝╚══════╝╚═╝
#
#  OS - The Operating System That Thinks
#  Windows Installer v4.0.0
# ═══════════════════════════════════════════════════════════════════════════════
#
#  KULLANIM:
#    irm https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.ps1 | iex
#
#  PARAMETRELER:
#    -Mode <quick|standard|full|custom>   : Kurulum modu
#    -Provider <name>                     : LLM provider
#    -Model <name>                        : Model adı
#    -ApiKey <key>                        : API key
#    -SkipPrompts                         : Tüm soruları atla
#    -Uninstall                           : Kaldır
# ═══════════════════════════════════════════════════════════════════════════════

[CmdletBinding()]
param(
    [ValidateSet("quick", "standard", "full", "custom")]
    [string]$Mode = "",
    
    [ValidateSet("ollama", "openrouter", "openai", "anthropic", "deepseek", "groq", "google", "unify", "lmstudio", "vllm")]
    [string]$Provider = "",
    
    [string]$Model = "",
    [string]$ApiKey = "",
    [switch]$SkipPrompts,
    [switch]$Uninstall,
    [switch]$Silent
)

# ═══════════════════════════════════════════════════════════════════════════════
#  ANSI RENKLERİ VE YARDIMCI FONKSİYONLAR
# ═══════════════════════════════════════════════════════════════════════════════

$ESC = [char]27

function Color { param($c) "$ESC[$c" }
function Reset { Color "0m" }

# Renk kodları
$RED = Color "91m"
$GREEN = Color "92m"
$YELLOW = Color "93m"
$BLUE = Color "94m"
$MAGENTA = Color "95m"
$CYAN = Color "96m"
$WHITE = Color "97m"
$BOLD = Color "1m"
$DIM = Color "2m"
$UNDERLINE = Color "4m"

# Log fonksiyonları
function Write-Step { param($msg) Write-Host "${CYAN}━━━${RESET} $msg" }
function Write-Info { param($msg) Write-Host "  ${BLUE}ℹ${RESET}  $msg" }
function Write-OK { param($msg) Write-Host "  ${GREEN}✓${RESET}  $msg" }
function Write-Warn { param($msg) Write-Host "  ${YELLOW}⚠${RESET}  $msg" }
function Write-Err { param($msg) Write-Host "  ${RED}✗${RESET}  $msg" }
function Write-Menu { param($num, $text) Write-Host "    ${WHITE}[${CYAN}$num${WHITE}]${RESET} $text" }
function Write-Separator { Write-Host "${DIM}  ═════════════════════════════════════════════════════════════${RESET}" }

# Progress bar
function Show-Progress {
    param($Step, $Total, $Message)
    $percent = [math]::Round(($Step / $Total) * 100)
    $filled = [math]::Round($percent / 5)
    $empty = 20 - $filled
    $bar = "${GREEN}" + ("█" * $filled) + "${DIM}" + ("░" * $empty) + "${RESET}"
    Write-Host "`r  $bar ${WHITE}$percent%${RESET} - $Message" -NoNewline
}

# Clear line
function Clear-Line { Write-Host "`r$(' ' * 80)`r" -NoNewline }

# ═══════════════════════════════════════════════════════════════════════════════
#  GLOBAL YAPILANDIRMA
# ═══════════════════════════════════════════════════════════════════════════════

$script:Config = @{
    Mode = "standard"
    Provider = "ollama"
    Model = "gemma3:27b"
    InstallDir = "$env:USERPROFILE\.sentient"
    InstallOllama = $true
    InstallDocker = $false
    InstallVoice = $false
    InstallDashboard = $false
    InstallDevTools = $false
    InstallPython = $true
    InstallRust = $true
    DownloadModel = $true
    StartServices = $true
    AddToPath = $true
    ApiKeys = @{}
    SystemRAM = 0
    SystemVRAM = 0
    HasNvidia = $false
    CpuCores = 0
    GpuName = ""
    OsVersion = ""
    RisCpu = $false
}

$script:Steps = @(
    @{Name = "Hoş Geldiniz"; Fn = "Show-Welcome" }
    @{Name = "Lisans Sözleşmesi"; Fn = "Show-License" }
    @{Name = "Sistem Analizi"; Fn = "Analyze-System" }
    @{Name = "Kurulum Modu"; Fn = "Select-Mode" }
    @{Name = "LLM Provider"; Fn = "Select-Provider" }
    @{Name = "Model Seçimi"; Fn = "Select-Model" }
    @{Name = "Bileşenler"; Fn = "Select-Components" }
    @{Name = "Ön Koşullar"; Fn = "Install-Prerequisites" }
    @{Name = "Kaynak İndirme"; Fn = "Download-Source" }
    @{Name = "Derleme"; Fn = "Build-Project" }
    @{Name = "Yapılandırma"; Fn = "Configure-Environment" }
    @{Name = "Doğrulama"; Fn = "Validate-Installation" }
)

$script:CurrentStep = 0

# ═══════════════════════════════════════════════════════════════════════════════
#  BANNER VE HOŞGELDİNİZ
# ═══════════════════════════════════════════════════════════════════════════════

function Show-Banner {
    Clear-Host
    
    # Ana banner
    Write-Host ""
    Write-Host "${CYAN}╔═══════════════════════════════════════════════════════════════════════════╗${RESET}"
    Write-Host "${CYAN}║${RESET}                                                                           ${CYAN}║${RESET}"
    Write-Host "${CYAN}║${RESET}   ${BOLD}${WHITE}███████╗███████╗███╗   ██╗████████╗███╗   ██╗███████╗██╗${RESET}               ${CYAN}║${RESET}"
    Write-Host "${CYAN}║${RESET}   ${BOLD}${WHITE}██╔════╝██╔════╝████╗  ██║╚══██╔══╝████╗  ██║██╔════╝██║${RESET}               ${CYAN}║${RESET}"
    Write-Host "${CYAN}║${RESET}   ${BOLD}${WHITE}███████╗█████╗  ██╔██╗ ██║   ██║   ██╔██╗ ██║███████╗██║${RESET}               ${CYAN}║${RESET}"
    Write-Host "${CYAN}║${RESET}   ${BOLD}${WHITE}╚════██║██╔══╝  ██║╚██╗██║   ██║   ██║╚██╗██║╚════██║██║${RESET}               ${CYAN}║${RESET}"
    Write-Host "${CYAN}║${RESET}   ${BOLD}${WHITE}███████║███████╗██║ ╚████║   ██║   ██║ ╚████║███████║██║${RESET}               ${CYAN}║${RESET}"
    Write-Host "${CYAN}║${RESET}   ${BOLD}${WHITE}╚══════╝╚══════╝╚═╝  ╚═══╝   ╚═╝   ╚═╝  ╚═══╝╚══════╝╚═╝${RESET}               ${CYAN}║${RESET}"
    Write-Host "${CYAN}║${RESET}                                                                           ${CYAN}║${RESET}"
    Write-Host "${CYAN}║${RESET}              ${MAGENTA}OS${RESET} ${DIM}-${RESET} ${YELLOW}The Operating System That Thinks${RESET}                          ${CYAN}║${RESET}"
    Write-Host "${CYAN}║${RESET}                                                                           ${CYAN}║${RESET}"
    Write-Host "${CYAN}║${RESET}   ${DIM}Version 4.0.0  •  AGPL v3 License  •  Made with${RESET} ${RED}❤${RESET} ${DIM}by Community${RESET}            ${CYAN}║${RESET}"
    Write-Host "${CYAN}║${RESET}                                                                           ${CYAN}║${RESET}"
    Write-Host "${CYAN}╚═══════════════════════════════════════════════════════════════════════════╝${RESET}"
    Write-Host ""
}

function Show-Welcome {
    Show-Banner
    
    Write-Host "${WHITE}  SENTIENT OS'e hoş geldiniz!${RESET}"
    Write-Host ""
    Write-Host "  Bu sihirbaz size adım adım rehberlik edecek:"
    Write-Host ""
    Write-Host "    ${CYAN}◆${RESET} Sistem gereksinimlerinizi analiz edecek"
    Write-Host "    ${CYAN}◆${RESET} Size en uygun kurulum modunu önerecek"
    Write-Host "    ${CYAN}◆${RESET} Donanımınıza göre model seçimi yapacak"
    Write-Host "    ${CYAN}◆${RESET} Gerekli tüm bağımlılıkları kuracak"
    Write-Host "    ${CYAN}◆${RESET} İlk yapılandırmanızı otomatik oluşturacak"
    Write-Host ""
    Write-Separator
    Write-Host ""
    
    # Kurulum yollarını göster
    Write-Host "${WHITE}  Kurulum Seçenekleri:${RESET}"
    Write-Host ""
    Write-Host "    ${GREEN}QUICK${RESET}    ${DIM}→${RESET} Hazır profil, hızlı kurulum (5 dk)"
    Write-Host "             ${DIM}CLI + Ollama + Küçük model${RESET}"
    Write-Host ""
    Write-Host "    ${YELLOW}STANDARD${RESET} ${DIM}→${RESET} Dengeli kurulum (15 dk)"
    Write-Host "             ${DIM}CLI + Tools + Orta boy model${RESET}"
    Write-Host ""
    Write-Host "    ${MAGENTA}FULL${RESET}     ${DIM}→${RESET} Tam kurulum (30 dk)"
    Write-Host "             ${DIM}Docker + Voice + Dashboard + Büyük model${RESET}"
    Write-Host ""
    Write-Host "    ${BLUE}CUSTOM${RESET}    ${DIM}→${RESET} Özelleştirilmiş kurulum"
    Write-Host "             ${DIM}Her bileşeni kendiniz seçin${RESET}"
    Write-Host ""
    Write-Separator
    Write-Host ""
    
    if (-not $SkipPrompts) {
        Write-Host "  ${YELLOW}Enter${RESET} tuşuna basarak devam edin..."
        $null = Read-Host
    }
    
    return $true
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 1: LİSANS SÖZLEŞMESİ
# ═══════════════════════════════════════════════════════════════════════════════

function Show-License {
    Show-Progress -Step 1 -Total 12 -Message "Lisans"
    Start-Sleep -Milliseconds 300
    Clear-Line
    
    Write-Host ""
    Write-Host "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "${WHITE}  │${RESET} ${BOLD}LİSANS SÖZLEŞMESİ${RESET}                                                   ${WHITE}│${RESET}"
    Write-Host "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    Write-Host "  ${CYAN}◆${RESET} ${WHITE}Lisans Türü:${RESET} AGPL v3 (Affero GNU General Public License)"
    Write-Host "  ${CYAN}◆${RESET} ${WHITE}Kaynak Kod:${RESET} https://github.com/nexsusagent-coder/SENTIENT_CORE"
    Write-Host ""
    
    # Lisans kutusu
    Write-Host "${DIM}  ┌───────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "${DIM}  │${RESET}  ${WHITE}ÖNEMLİ NOKTALAR:${RESET}                                                 ${DIM}│${RESET}"
    Write-Host "${DIM}  │${RESET}                                                                   ${DIM}│${RESET}"
    Write-Host "${DIM}  │${RESET}  ${GREEN}✓${RESET} Özgür ve açık kaynak yazılım                            ${DIM}│${RESET}"
    Write-Host "${DIM}  │${RESET}  ${GREEN}✓${RESET} Kişisel ve ticari kullanım serbest                       ${DIM}│${RESET}"
    Write-Host "${DIM}  │${RESET}  ${GREEN}✓${RESET} Değişiklik yapma hakkı                                    ${DIM}│${RESET}"
    Write-Host "${DIM}  │${RESET}  ${GREEN}✓${RESET} Dağıtma hakkı                                              ${DIM}│${RESET}"
    Write-Host "${DIM}  │${RESET}                                                                   ${DIM}│${RESET}"
    Write-Host "${DIM}  │${RESET}  ${YELLOW}⚠${RESET} Değişiklik yaparsanız kaynak kodunu paylaşmalısınız       ${DIM}│${RESET}"
    Write-Host "${DIM}  │${RESET}  ${YELLOW}⚠${RESET} AGPL lisansı network kullanımını kapsar                   ${DIM}│${RESET}"
    Write-Host "${DIM}  │${RESET}                                                                   ${DIM}│${RESET}"
    Write-Host "${DIM}  │${RESET}  ${RED}✗${RESET} Kapalı kaynak türev ürünler yasak                         ${DIM}│${RESET}"
    Write-Host "${DIM}  │${RESET}  ${RED}✗${RESET} Lisans bildirimlerini kaldırmak yasak                     ${DIM}│${RESET}"
    Write-Host "${DIM}  │${RESET}                                                                   ${DIM}│${RESET}"
    Write-Host "${DIM}  └───────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    # AI Uyarısı
    Write-Host "  ${WHITE}┌─────────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "  ${WHITE}│${RESET} ${YELLOW}⚠ AI SİSTEMİ UYARISI${RESET}                                              ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}                                                                     ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}  SENTIENT bir AI asistanıdır. Ürettiği içerikler:                  ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}  ${DIM}• Hatalı veya yanıltıcı olabilir${RESET}                              ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}  ${DIM}• Güncel olmayabilir${RESET}                                          ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}  ${DIM}• Profesyonel tavsiye yerine geçmez${RESET}                            ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}                                                                     ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}  ${RED}Kritik kararlar için her zaman doğrulama yapın!${RESET}                ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}└─────────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    if ($SkipPrompts) {
        Write-OK "Lisans otomatik kabul edildi"
        return $true
    }
    
    Write-Host "  Lisans sözleşmesini kabul ediyor musunuz?"
    Write-Host ""
    Write-Host "    ${WHITE}[${GREEN}Y${WHITE}]${RESET} Evet, kabul ediyorum ve devam et"
    Write-Host "    ${WHITE}[${RED}N${WHITE}]${RESET} Hayır, kurulumdan çık"
    Write-Host "    ${WHITE}[${BLUE}R${WHITE}]${RESET} Lisansın tamamını oku"
    Write-Host ""
    
    while ($true) {
        $choice = Read-Host "  Seçiminiz [Y/N/R]"
        
        switch -Regex ($choice) {
            "^[Yy]$|^$" {
                Write-OK "Lisans kabul edildi"
                return $true
            }
            "^[Nn]$" {
                Write-Err "Kurulum iptal edildi"
                exit 0
            }
            "^[Rr]$" {
                Write-Host ""
                Write-Host "  ${CYAN}AGPL v3 Lisans Özeti:${RESET}"
                Write-Host "  ${DIM}────────────────────────────────────────────────────────────${RESET}"
                Write-Host "  Bu program özgür yazılımdır; dağıtabilir ve/veya değiştirebilirsiniz."
                Write-Host "  GNU Affero General Public License koşulları altında yayımlanmıştır."
                Write-Host "  Lisansın 3. sürümü veya (isteğe bağlı) daha yeni sürümü geçerlidir."
                Write-Host ""
                Write-Host "  Tam lisans metni için: https://www.gnu.org/licenses/agpl-3.0.html"
                Write-Host "  ${DIM}────────────────────────────────────────────────────────────${RESET}"
                Write-Host ""
            }
        }
    }
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 2: SİSTEM ANALİZİ
# ═══════════════════════════════════════════════════════════════════════════════

function Analyze-System {
    Write-Host ""
    Write-Host "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "${WHITE}  │${RESET} ${BOLD}SİSTEM ANALİZİ${RESET}                                                     ${WHITE}│${RESET}"
    Write-Host "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    # OS
    $osInfo = Get-CimInstance Win32_OperatingSystem
    $script:Config.OsVersion = "$($osInfo.Caption) $($osInfo.Version)"
    Write-Info "İşletim Sistemi: $($script:Config.OsVersion)"
    
    # CPU
    $cpu = Get-CimInstance Win32_Processor
    $script:Config.CpuCores = $cpu.NumberOfLogicalProcessors
    Write-OK "CPU: $($cpu.Name) ($($script:Config.CpuCores) çekirdek)"
    
    # ARM kontrolü
    if ($cpu.Name -match "ARM|Snapdragon|Apple") {
        $script:Config.RisCpu = $true
        Write-Warn "ARM mimarisi tespit edildi - bazı özellikler sınırlı olabilir"
    }
    
    # RAM
    $ram = (Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory / 1GB
    $script:Config.SystemRAM = [math]::Round($ram, 1)
    if ($ram -ge 32) {
        Write-OK "RAM: $($script:Config.SystemRAM) GB ${GREEN}(Mükemmel)${RESET}"
    } elseif ($ram -ge 16) {
        Write-OK "RAM: $($script:Config.SystemRAM) GB ${GREEN}(İyi)${RESET}"
    } elseif ($ram -ge 8) {
        Write-Warn "RAM: $($script:Config.SystemRAM) GB ${YELLOW}(Minimum)${RESET}"
    } else {
        Write-Err "RAM: $($script:Config.SystemRAM) GB ${RED}(Yetersiz)${RESET}"
    }
    
    # GPU
    $gpus = Get-CimInstance Win32_VideoController
    $nvidiaGpu = $gpus | Where-Object { $_.Name -match "NVIDIA|GeForce|RTX|GTX|Quadro" }
    $amdGpu = $gpus | Where-Object { $_.Name -match "AMD|Radeon|RX" }
    $intelGpu = $gpus | Where-Object { $_.Name -match "Intel|Arc|UHD|Iris" }
    
    if ($nvidiaGpu) {
        $script:Config.HasNvidia = $true
        $gpu = $nvidiaGpu[0]
        $script:Config.GpuName = $gpu.Name
        
        # VRAM tahmini
        $vramBytes = $gpu.AdapterRAM
        if ($vramBytes) {
            $vramGB = [math]::Round($vramBytes / 1GB, 0)
            $script:Config.SystemVRAM = $vramGB
            Write-OK "GPU: $($gpu.Name) ${GREEN}(${vramGB}GB VRAM)${RESET}"
        } else {
            # VRAM bilinmiyorsa model adından tahmin et
            if ($gpu.Name -match "4090|3090") { $vramGB = 24 }
            elseif ($gpu.Name -match "4080|3080") { $vramGB = 16 }
            elseif ($gpu.Name -match "4070|3070|4060 Ti") { $vramGB = 12 }
            elseif ($gpu.Name -match "4060|3060|2080") { $vramGB = 8 }
            elseif ($gpu.Name -match "3050|2060|1660") { $vramGB = 6 }
            else { $vramGB = 4 }
            $script:Config.SystemVRAM = $vramGB
            Write-OK "GPU: $($gpu.Name) ${YELLOW}(~${vramGB}GB VRAM)${RESET}"
        }
    } elseif ($amdGpu) {
        Write-OK "GPU: $($amdGpu[0].Name) ${YELLOW}(ROCm desteği sınırlı)${RESET}"
    } elseif ($intelGpu) {
        Write-Warn "GPU: $($intelGpu[0].Name) ${YELLOW}(Intel Arc destekleniyor)${RESET}"
    } else {
        Write-Warn "GPU: NVIDIA GPU bulunamadı - CPU inference kullanılacak"
    }
    
    # Disk alanı
    $disk = (Get-CimInstance Win32_LogicalDisk -Filter "DeviceID='C:'").FreeSpace / 1GB
    Write-Info "Disk (C:): $([math]::Round($disk, 1)) GB boş"
    
    if ($disk -lt 20) {
        Write-Err "Yetersiz disk alanı! En az 20 GB gerekli."
        return $false
    }
    
    # Sistem profili
    Write-Host ""
    Write-Separator
    Write-Host ""
    Write-Host "  ${WHITE}Sistem Profiliniz:${RESET}"
    Write-Host ""
    
    $profile = ""
    $recommendation = ""
    
    if ($script:Config.SystemRAM -ge 64 -and $script:Config.SystemVRAM -ge 24) {
        $profile = "${GREEN}WORKSTATION${RESET}"
        $recommendation = "Büyük modeller (70B+) için uygun"
        $script:Config.Model = "llama3.3:70b"
    } elseif ($script:Config.SystemRAM -ge 32 -and $script:Config.SystemVRAM -ge 16) {
        $profile = "${GREEN}HIGH-END${RESET}"
        $recommendation = "Orta-büyük modeller (27B-70B) için uygun"
        $script:Config.Model = "gemma3:27b"
    } elseif ($script:Config.SystemRAM -ge 16 -and $script:Config.SystemVRAM -ge 8) {
        $profile = "${YELLOW}MID-RANGE${RESET}"
        $recommendation = "Orta boy modeller (8B-27B) için uygun"
        $script:Config.Model = "gemma3:12b"
    } elseif ($script:Config.SystemRAM -ge 8) {
        $profile = "${YELLOW}ENTRY-LEVEL${RESET}"
        $recommendation = "Küçük modeller veya API kullanımı önerilir"
        $script:Config.Model = "qwen3:30b-a3b"
    } else {
        $profile = "${RED}MINIMAL${RESET}"
        $recommendation = "API modu önerilir (Cloud LLM)"
        $script:Config.Provider = "openrouter"
        $script:Config.InstallOllama = $false
        $script:Config.DownloadModel = $false
    }
    
    Write-Host "    ${CYAN}◆${RESET} Profil:    $profile"
    Write-Host "    ${CYAN}◆${RESET} Öneri:     $recommendation"
    Write-Host "    ${CYAN}◆${RESET} Model:     $($script:Config.Model)"
    Write-Host ""
    Write-Separator
    
    return $true
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 3: KURULUM MODU SEÇİMİ
# ═══════════════════════════════════════════════════════════════════════════════

function Select-Mode {
    Write-Host ""
    Write-Host "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "${WHITE}  │${RESET} ${BOLD}KURULUM MODU SEÇİMİ${RESET}                                                ${WHITE}│${RESET}"
    Write-Host "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    # Parametre ile verildiyse
    if ($Mode -ne "") {
        $script:Config.Mode = $Mode
        Write-Info "Mod parametreden: $($Mode.ToUpper())"
        Apply-ModeDefaults
        return $true
    }
    
    if ($SkipPrompts) {
        $script:Config.Mode = "standard"
        Apply-ModeDefaults
        return $true
    }
    
    Write-Host "  Kurulum modunu seçin:"
    Write-Host ""
    
    # Quick
    Write-Host "  ${WHITE}┌─────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "  ${WHITE}│${RESET} ${GREEN}${BOLD}QUICK${RESET} ${DIM}- Hızlı Başlangıç${RESET}                                         ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}                                                                 ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}   ${DIM}Süre: ~5 dakika${RESET}                                              ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}   ${DIM}• CLI + temel araçlar${RESET}                                      ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}   ${DIM}• Ollama + küçük model (qwen3:30b-a3b)${RESET}                       ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}   ${DIM}• Minimum yapılandırma${RESET}                                     ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}   ${DIM}• Yeni başlayanlar için ideal${RESET}                                ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}└─────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    # Standard
    Write-Host "  ${WHITE}┌─────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "  ${WHITE}│${RESET} ${YELLOW}${BOLD}STANDARD${RESET} ${DIM}- Önerilen${RESET} ${GREEN}★${RESET}                                        ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}                                                                 ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}   ${DIM}Süre: ~15 dakika${RESET}                                             ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}   ${DIM}• CLI + araçlar + Python entegrasyonu${RESET}                        ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}   ${DIM}• Ollama + donanımınıza uygun model${RESET}                          ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}   ${DIM}• Tam yapılandırma${RESET}                                          ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}   ${DIM}• Çoğu kullanıcı için en iyi seçenek${RESET}                         ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}└─────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    # Full
    Write-Host "  ${WHITE}┌─────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "  ${WHITE}│${RESET} ${MAGENTA}${BOLD}FULL${RESET} ${DIM}- Tam Kurulum${RESET}                                            ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}                                                                 ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}   ${DIM}Süre: ~30 dakika${RESET}                                             ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}   ${DIM}• Tüm STANDARD özellikler +${RESET}                                  ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}   ${DIM}• Docker servisleri (PostgreSQL, Redis, Qdrant...)${RESET}            ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}   ${DIM}• Voice (Whisper + Piper)${RESET}                                   ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}   ${DIM}• Dashboard (Web UI)${RESET}                                        ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}   ${DIM}• Geliştiriciler ve power users için${RESET}                         ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}└─────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    # Custom
    Write-Host "  ${WHITE}┌─────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "  ${WHITE}│${RESET} ${BLUE}${BOLD}CUSTOM${RESET} ${DIM}- Özelleştirilmiş${RESET}                                        ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}                                                                 ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}   ${DIM}Süre: Değişken${RESET}                                               ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}   ${DIM}• Her bileşeni kendiniz seçin${RESET}                                ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}   ${DIM}• Provider ve model seçimi${RESET}                                  ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}   ${DIM}• Modül ekleme/çıkarma${RESET}                                      ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}│${RESET}   ${DIM}• Deneyimli kullanıcılar için${RESET}                                ${WHITE}│${RESET}"
    Write-Host "  ${WHITE}└─────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    Write-Host "  Seçiminiz: ${WHITE}[${GREEN}1${WHITE}]${RESET} Quick  ${WHITE}[${GREEN}2${WHITE}]${RESET} Standard  ${WHITE}[${GREEN}3${WHITE}]${RESET} Full  ${WHITE}[${GREEN}4${WHITE}]${RESET} Custom"
    Write-Host ""
    
    while ($true) {
        $choice = Read-Host "  [1-4]"
        
        switch ($choice) {
            "1" {
                $script:Config.Mode = "quick"
                break
            }
            "2" {
                $script:Config.Mode = "standard"
                break
            }
            "3" {
                $script:Config.Mode = "full"
                break
            }
            "4" {
                $script:Config.Mode = "custom"
                break
            }
            "" {
                $script:Config.Mode = "standard"
                break
            }
            default {
                continue
            }
        }
        break
    }
    
    Write-OK "$($script:Config.Mode.ToUpper()) modu seçildi"
    Apply-ModeDefaults
    return $true
}

function Apply-ModeDefaults {
    switch ($script:Config.Mode) {
        "quick" {
            $script:Config.InstallOllama = $true
            $script:Config.InstallDocker = $false
            $script:Config.InstallVoice = $false
            $script:Config.InstallDashboard = $false
            $script:Config.InstallDevTools = $false
            $script:Config.Model = "qwen3:30b-a3b"
        }
        "standard" {
            $script:Config.InstallOllama = $true
            $script:Config.InstallDocker = $false
            $script:Config.InstallVoice = $false
            $script:Config.InstallDashboard = $false
            $script:Config.InstallDevTools = $false
            # Model sistem analizinden gelir
        }
        "full" {
            $script:Config.InstallOllama = $true
            $script:Config.InstallDocker = $true
            $script:Config.InstallVoice = $true
            $script:Config.InstallDashboard = $true
            $script:Config.InstallDevTools = $true
        }
        "custom" {
            # Custom modda seçimler sonraki adımlarda yapılır
        }
    }
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 4: LLM PROVIDER SEÇİMİ
# ═══════════════════════════════════════════════════════════════════════════════

function Select-Provider {
    Write-Host ""
    Write-Host "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "${WHITE}  │${RESET} ${BOLD}LLM PROVIDER SEÇİMİ${RESET}                                                ${WHITE}│${RESET}"
    Write-Host "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    # Parametre ile verildiyse
    if ($Provider -ne "") {
        $script:Config.Provider = $Provider
        Write-Info "Provider parametreden: $($Provider.ToUpper())"
        if ($ApiKey -ne "") {
            $script:Config.ApiKeys[$Provider] = $ApiKey
        }
        return $true
    }
    
    if ($SkipPrompts -or $script:Config.Mode -ne "custom") {
        Write-Info "Varsayılan provider: OLLAMA (lokal)"
        return $true
    }
    
    Write-Host "  AI modelinizi nasıl çalıştırmak istersiniz?"
    Write-Host ""
    
    # Lokal
    Write-Host "  ${GREEN}╔═══════════════════════════════════════════════════════════════════╗${RESET}"
    Write-Host "  ${GREEN}║${RESET}              ${WHITE}${BOLD}LOKAL MODELLER${RESET} ${DIM}(Ücretsiz)${RESET}                           ${GREEN}║${RESET}"
    Write-Host "  ${GREEN}╠═══════════════════════════════════════════════════════════════════╣${RESET}"
    Write-Host "  ${GREEN}║${RESET}                                                                   ${GREEN}║${RESET}"
    Write-Host "  ${GREEN}║${RESET}  ${WHITE}[1]${RESET} Ollama ${GREEN}★${RESET}        En popüler, 50K+ model             ${GREEN}║${RESET}"
    Write-Host "  ${GREEN}║${RESET}      ${DIM}Kolay kullanım, otomatik GPU desteği${RESET}                        ${GREEN}║${RESET}"
    Write-Host "  ${GREEN}║${RESET}                                                                   ${GREEN}║${RESET}"
    Write-Host "  ${GREEN}║${RESET}  ${WHITE}[2]${RESET} LM Studio       GUI ile model yönetimi               ${GREEN}║${RESET}"
    Write-Host "  ${GREEN}║${RESET}      ${DIM}Model indirme, parametre ayarı${RESET}                               ${GREEN}║${RESET}"
    Write-Host "  ${GREEN}║${RESET}                                                                   ${GREEN}║${RESET}"
    Write-Host "  ${GREEN}║${reset}  ${WHITE}[3]${RESET} vLLM            Yüksek performans server            ${GREEN}║${RESET}"
    Write-Host "  ${GREEN}║${RESET}      ${DIM}Production-grade, batch inference${RESET}                           ${GREEN}║${RESET}"
    Write-Host "  ${GREEN}║${RESET}                                                                   ${GREEN}║${RESET}"
    Write-Host "  ${GREEN}╚═══════════════════════════════════════════════════════════════════╝${RESET}"
    Write-Host ""
    
    # Cloud API
    Write-Host "  ${YELLOW}╔═══════════════════════════════════════════════════════════════════╗${RESET}"
    Write-Host "  ${YELLOW}║${RESET}              ${WHITE}${BOLD}CLOUD API${RESET} ${DIM}(API Key Gerekli)${RESET}                          ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}╠═══════════════════════════════════════════════════════════════════╣${RESET}"
    Write-Host "  ${YELLOW}║${RESET}                                                                   ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}║${RESET}  ${WHITE}[4]${RESET} OpenRouter ${GREEN}★${RESET}    200+ model, \$5 ücretsiz kredi      ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}║${RESET}      ${DIM}openrouter.ai${RESET}                                               ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}║${RESET}                                                                   ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}║${RESET}  ${WHITE}[5]${RESET} OpenAI          GPT-4o, o1, o3, o4-mini               ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}║${RESET}      ${DIM}platform.openai.com${RESET}                                         ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}║${RESET}                                                                   ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}║${RESET}  ${WHITE}[6]${RESET} Anthropic       Claude 4 Sonnet, Opus 4.1             ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}║${RESET}      ${DIM}console.anthropic.com${RESET}                                       ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}║${RESET}                                                                   ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}║${RESET}  ${WHITE}[7]${RESET} DeepSeek ${GREEN}★${RESET}      ${BOLD}EN UCUZ${RESET} - V3, R1 reasoning           ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}║${RESET}      ${DIM}platform.deepseek.com${RESET}                                        ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}║${RESET}                                                                   ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}║${RESET}  ${WHITE}[8]${RESET} Google AI        Gemini Flash ${GREEN}(FREE tier!)${RESET}          ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}║${RESET}      ${DIM}aistudio.google.com${RESET}                                          ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}║${RESET}                                                                   ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}║${RESET}  ${WHITE}[9]${RESET} Groq ${GREEN}★${RESET}          ${BOLD}EN HIZLI${RESET} - Llama 3.3 70B           ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}║${RESET}      ${DIM}console.groq.com${RESET}                                              ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}║${RESET}                                                                   ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}╚═══════════════════════════════════════════════════════════════════╝${RESET}"
    Write-Host ""
    
    # Gateway
    Write-Host "  ${MAGENTA}╔═══════════════════════════════════════════════════════════════════╗${RESET}"
    Write-Host "  ${MAGENTA}║${RESET}              ${WHITE}${BOLD}AI GATEWAY / ROUTER${RESET}                                       ${MAGENTA}║${RESET}"
    Write-Host "  ${MAGENTA}╠═══════════════════════════════════════════════════════════════════╣${RESET}"
    Write-Host "  ${MAGENTA}║${RESET}                                                                   ${MAGENTA}║${RESET}"
    Write-Host "  ${MAGENTA}║${RESET}  ${WHITE}[10]${RESET} Unify AI        Akıllı routing (kalite+maliyet)    ${MAGENTA}║${RESET}"
    Write-Host "  ${MAGENTA}║${RESET}       ${DIM}unify.ai${RESET}                                                    ${MAGENTA}║${RESET}"
    Write-Host "  ${MAGENTA}║${RESET}                                                                   ${MAGENTA}║${RESET}"
    Write-Host "  ${MAGENTA}║${RESET}  ${WHITE}[11]${RESET} LiteLLM         Self-hosted proxy server          ${MAGENTA}║${RESET}"
    Write-Host "  ${MAGENTA}║${RESET}       ${DIM}github.com/BerriAI/litellm${RESET}                                 ${MAGENTA}║${RESET}"
    Write-Host "  ${MAGENTA}║${RESET}                                                                   ${MAGENTA}║${RESET}"
    Write-Host "  ${MAGENTA}╚═══════════════════════════════════════════════════════════════════╝${RESET}"
    Write-Host ""
    
    Write-Host "  ${WHITE}[0]${RESET} API Key olmadan devam et (daha sonra yapılandır)"
    Write-Host ""
    
    while ($true) {
        $choice = Read-Host "  Provider seçiniz [1-11]"
        
        $providerMap = @{
            "1" = "ollama"
            "2" = "lmstudio"
            "3" = "vllm"
            "4" = "openrouter"
            "5" = "openai"
            "6" = "anthropic"
            "7" = "deepseek"
            "8" = "google"
            "9" = "groq"
            "10" = "unify"
            "11" = "litellm"
            "0" = "none"
        }
        
        if ($providerMap.ContainsKey($choice)) {
            $script:Config.Provider = $providerMap[$choice]
            break
        }
    }
    
    # API Key gerektiren provider'lar
    $needsApiKey = @("openrouter", "openai", "anthropic", "deepseek", "google", "groq", "unify")
    
    if ($needsApiKey -contains $script:Config.Provider) {
        Write-Host ""
        
        $keyNames = @{
            "openrouter" = "OPENROUTER_API_KEY"
            "openai" = "OPENAI_API_KEY"
            "anthropic" = "ANTHROPIC_API_KEY"
            "deepseek" = "DEEPSEEK_API_KEY"
            "google" = "GOOGLE_AI_API_KEY"
            "groq" = "GROQ_API_KEY"
            "unify" = "UNIFY_API_KEY"
        }
        
        $urls = @{
            "openrouter" = "https://openrouter.ai/keys"
            "openai" = "https://platform.openai.com/api-keys"
            "anthropic" = "https://console.anthropic.com/settings/keys"
            "deepseek" = "https://platform.deepseek.com/api_keys"
            "google" = "https://aistudio.google.com/apikey"
            "groq" = "https://console.groq.com/keys"
            "unify" = "https://unify.ai/keys"
        }
        
        Write-Host "  ${YELLOW}API Key Gerekli!${RESET}"
        Write-Host "  Almak için: ${CYAN}$($urls[$script:Config.Provider])${RESET}"
        Write-Host ""
        
        if ($ApiKey -ne "") {
            $script:Config.ApiKeys[$script:Config.Provider] = $ApiKey
            Write-OK "API Key parametreden alındı"
        } else {
            $key = Read-Host "  $($keyNames[$script:Config.Provider])"
            if ($key -ne "") {
                $script:Config.ApiKeys[$script:Config.Provider] = $key
            } else {
                Write-Warn "API Key girilmedi - .env dosyasından ekleyebilirsiniz"
            }
        }
        
        # Cloud provider ise Ollama kurulumuna gerek yok
        $script:Config.InstallOllama = $false
        $script:Config.DownloadModel = $false
    }
    
    Write-OK "Provider: $($script:Config.Provider.ToUpper())"
    return $true
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 5: MODEL SEÇİMİ
# ═══════════════════════════════════════════════════════════════════════════════

function Select-Model {
    # Cloud provider ise model seçimi atlanır
    if ($script:Config.Provider -notin @("ollama", "lmstudio", "vllm")) {
        Write-Info "Cloud provider seçildi - model .env'de yapılandırılacak"
        return $true
    }
    
    # Parametre ile verildiyse
    if ($Model -ne "") {
        $script:Config.Model = $Model
        Write-Info "Model parametreden: $Model"
        return $true
    }
    
    if ($SkipPrompts -or $script:Config.Mode -ne "custom") {
        Write-Info "Önerilen model: $($script:Config.Model)"
        return $true
    }
    
    Write-Host ""
    Write-Host "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "${WHITE}  │${RESET} ${BOLD}MODEL SEÇİMİ${RESET} ${DIM}(VRAM: $($script:Config.SystemVRAM) GB)${RESET}                              ${WHITE}│${RESET}"
    Write-Host "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    $vram = $script:Config.SystemVRAM
    
    # 24GB+ VRAM
    if ($vram -ge 24) {
        Write-Host "  ${GREEN}╔═══════════════════════════════════════════════════════════════════╗${RESET}"
        Write-Host "  ${GREEN}║${RESET}              ${WHITE}${BOLD}24GB+ VRAM - BÜYÜK MODELLER${RESET}                            ${GREEN}║${RESET}"
        Write-Host "  ${GREEN}╠═══════════════════════════════════════════════════════════════════╣${RESET}"
        Write-Host "  ${GREEN}║${RESET}                                                                   ${GREEN}║${RESET}"
        Write-Host "  ${GREEN}║${RESET}  ${WHITE}[1]${RESET} llama3.3:70b       70B parametre, güçlü reasoning    ${GREEN}║${RESET}"
        Write-Host "  ${GREEN}║${RESET}  ${WHITE}[2]${RESET} deepseek-r1:67b    67B, mükemmel matematik/kod       ${GREEN}║${RESET}"
        Write-Host "  ${GREEN}║${RESET}  ${WHITE}[3]${RESET} llama4:scout       109B MoE, 10M context            ${GREEN}║${RESET}"
        Write-Host "  ${GREEN}║${RESET}  ${WHITE}[4]${RESET} gemma3:27b ${GREEN}★${RESET}       27B, dengeli performans         ${GREEN}║${RESET}"
        Write-Host "  ${GREEN}║${RESET}                                                                   ${GREEN}║${RESET}"
        Write-Host "  ${GREEN}╚═══════════════════════════════════════════════════════════════════╝${RESET}"
    }
    # 16GB VRAM
    elseif ($vram -ge 16) {
        Write-Host "  ${YELLOW}╔═══════════════════════════════════════════════════════════════════╗${RESET}"
        Write-Host "  ${YELLOW}║${RESET}              ${WHITE}${BOLD}16GB VRAM - ORTA-BÜYÜK MODELLER${RESET}                        ${YELLOW}║${RESET}"
        Write-Host "  ${YELLOW}╠═══════════════════════════════════════════════════════════════════╣${RESET}"
        Write-Host "  ${YELLOW}║${RESET}                                                                   ${YELLOW}║${RESET}"
        Write-Host "  ${YELLOW}║${RESET}  ${WHITE}[1]${RESET} gemma3:27b ${GREEN}★${RESET}       27B, dengeli performans         ${YELLOW}║${RESET}"
        Write-Host "  ${YELLOW}║${RESET}  ${WHITE}[2]${RESET} gemma3:12b         12B, hızlı inference             ${YELLOW}║${RESET}"
        Write-Host "  ${YELLOW}║${RESET}  ${WHITE}[3]${RESET} mistral-small3.1   24B, Avrupa yapımı               ${YELLOW}║${RESET}"
        Write-Host "  ${YELLOW}║${RESET}  ${WHITE}[4]${RESET} pixtral:12b        12B, multimodal (görüntü)        ${YELLOW}║${RESET}"
        Write-Host "  ${YELLOW}║${RESET}                                                                   ${YELLOW}║${RESET}"
        Write-Host "  ${YELLOW}╚═══════════════════════════════════════════════════════════════════╝${RESET}"
    }
    # 8GB VRAM
    elseif ($vram -ge 8) {
        Write-Host "  ${YELLOW}╔═══════════════════════════════════════════════════════════════════╗${RESET}"
        Write-Host "  ${YELLOW}║${RESET}              ${WHITE}${BOLD}8GB VRAM - ORTA MODELLER${RESET}                               ${YELLOW}║${RESET}"
        Write-Host "  ${YELLOW}╠═══════════════════════════════════════════════════════════════════╣${RESET}"
        Write-Host "  ${YELLOW}║${RESET}                                                                   ${YELLOW}║${RESET}"
        Write-Host "  ${YELLOW}║${RESET}  ${WHITE}[1]${RESET} deepseek-r1:8b     8B, iyi reasoning               ${YELLOW}║${RESET}"
        Write-Host "  ${YELLOW}║${RESET}  ${WHITE}[2]${RESET} mistral-small3.1   24B quantized                   ${YELLOW}║${RESET}"
        Write-Host "  ${YELLOW}║${RESET}  ${WHITE}[3]${RESET} qwen2.5-coder:7b   7B, coding optimize             ${YELLOW}║${RESET}"
        Write-Host "  ${YELLOW}║${RESET}  ${WHITE}[4]${RESET} gemma3:12b ${GREEN}★${RESET}       12B, dengeli                    ${YELLOW}║${RESET}"
        Write-Host "  ${YELLOW}║${RESET}                                                                   ${YELLOW}║${RESET}"
        Write-Host "  ${YELLOW}╚═══════════════════════════════════════════════════════════════════╝${RESET}"
    }
    # 4GB veya daha az
    else {
        Write-Host "  ${RED}╔═══════════════════════════════════════════════════════════════════╗${RESET}"
        Write-Host "  ${RED}║${RESET}              ${WHITE}${BOLD}DÜŞÜK VRAM - KÜÇÜK MODELLER${RESET}                               ${RED}║${RESET}"
        Write-Host "  ${RED}╠═══════════════════════════════════════════════════════════════════╣${RESET}"
        Write-Host "  ${RED}║${RESET}                                                                   ${RED}║${RESET}"
        Write-Host "  ${RED}║${RESET}  ${WHITE}[1]${RESET} qwen3:30b-a3b ${GREEN}★${RESET}   30B MoE (3B aktif) - ÖNERİLEN  ${RED}║${RESET}"
        Write-Host "  ${RED}║${RESET}  ${WHITE}[2]${RESET} phi4-mini          3.8B, Microsoft                 ${RED}║${RESET}"
        Write-Host "  ${RED}║${RESET}  ${WHITE}[3]${RESET} llama3.2:3b       3B, Meta                        ${RED}║${RESET}"
        Write-Host "  ${RED}║${RESET}  ${WHITE}[4]${RESET} llama3.2:1b       1.2B, çok hızlı                 ${RED}║${RESET}"
        Write-Host "  ${RED}║${RESET}                                                                   ${RED}║${RESET}"
        Write-Host "  ${RED}║${RESET}  ${YELLOW}⚠ Düşük VRAM: Cloud API kullanımı önerilir${RESET}                  ${RED}║${RESET}"
        Write-Host "  ${RED}║${RESET}                                                                   ${RED}║${RESET}"
        Write-Host "  ${RED}║${RESET}  ${WHITE}[5]${RESET} Cloud API kullan                                  ${RED}║${RESET}"
        Write-Host "  ${RED}║${RESET}                                                                   ${RED}║${RESET}"
        Write-Host "  ${RED}╚═══════════════════════════════════════════════════════════════════╝${RESET}"
    }
    
    Write-Host ""
    Write-Host "  ${WHITE}[0]${RESET} Model indirmeden devam et"
    Write-Host ""
    
    $choice = Read-Host "  Seçiminiz"
    
    # Seçime göre model ata
    switch ($choice) {
        "0" {
            $script:Config.DownloadModel = $false
            Write-Info "Model indirme atlanıyor"
        }
        "5" {
            # Cloud'a dön
            return Select-Provider
        }
        default {
            # VRAM'a göre seçenekleri map'le
            if ($vram -ge 24) {
                $models = @("llama3.3:70b", "deepseek-r1:67b", "llama4:scout", "gemma3:27b")
            } elseif ($vram -ge 16) {
                $models = @("gemma3:27b", "gemma3:12b", "mistral-small3.1", "pixtral:12b")
            } elseif ($vram -ge 8) {
                $models = @("deepseek-r1:8b", "mistral-small3.1", "qwen2.5-coder:7b", "gemma3:12b")
            } else {
                $models = @("qwen3:30b-a3b", "phi4-mini", "llama3.2:3b", "llama3.2:1b")
            }
            
            $idx = [int]$choice - 1
            if ($idx -ge 0 -and $idx -lt $models.Count) {
                $script:Config.Model = $models[$idx]
            }
        }
    }
    
    Write-OK "Model: $($script:Config.Model)"
    return $true
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 6: BİLEŞEN SEÇİMİ
# ═══════════════════════════════════════════════════════════════════════════════

function Select-Components {
    if ($script:Config.Mode -ne "custom") {
        return $true
    }
    
    Write-Host ""
    Write-Host "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "${WHITE}  │${RESET} ${BOLD}BİLEŞEN SEÇİMİ${RESET}                                                    ${WHITE}│${RESET}"
    Write-Host "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    # Ollama
    Write-Host "  ${WHITE}╔═══════════════════════════════════════════════════════════════════╗${RESET}"
    Write-Host "  ${WHITE}║${RESET}  ${CYAN}🤖 OLLAMA${RESET} ${DIM}(Lokal LLM Runtime)${RESET}                                ${WHITE}║${RESET}"
    Write-Host "  ${WHITE}║${RESET}                                                                   ${WHITE}║${RESET}"
    Write-Host "  ${WHITE}║${RESET}  ${DIM}Lokal AI modelleri çalıştırmak için gereklidir.${RESET}                 ${WHITE}║${RESET}"
    Write-Host "  ${WHITE}║${RESET}  ${DIM}Cloud API kullanacaksanız kurmanıza gerek yok.${RESET}                 ${WHITE}║${RESET}"
    Write-Host "  ${WHITE}║${RESET}                                                                   ${WHITE}║${RESET}"
    if ($script:Config.InstallOllama) {
        Write-Host "  ${WHITE}║${RESET}  ${GREEN}✓ Kurulacak${RESET}                                                    ${WHITE}║${RESET}"
    } else {
        Write-Host "  ${WHITE}║${RESET}  ${RED}✗ Kurulmayacak${RESET}                                                  ${WHITE}║${RESET}"
    }
    Write-Host "  ${WHITE}╚═══════════════════════════════════════════════════════════════════╝${RESET}"
    
    $ollama = Read-Host "  Kurulsun mu? [Y/n]"
    $script:Config.InstallOllama = ($ollama -ne "n" -and $ollama -ne "N")
    
    Write-Host ""
    
    # Docker
    Write-Host "  ${WHITE}╔═══════════════════════════════════════════════════════════════════╗${RESET}"
    Write-Host "  ${WHITE}║${RESET}  ${CYAN}🐳 DOCKER SERVİSLERİ${RESET}                                              ${WHITE}║${RESET}"
    Write-Host "  ${WHITE}║${RESET}                                                                   ${WHITE}║${RESET}"
    Write-Host "  ${WHITE}║${RESET}  ${DIM}PostgreSQL, Redis, Qdrant, MinIO, Prometheus, Grafana${RESET}        ${WHITE}║${RESET}"
    Write-Host "  ${WHITE}║${RESET}  ${DIM}Production ortamı için önerilir.${RESET}                               ${WHITE}║${RESET}"
    Write-Host "  ${WHITE}║${RESET}                                                                   ${WHITE}║${RESET}"
    if ($script:Config.InstallDocker) {
        Write-Host "  ${WHITE}║${RESET}  ${GREEN}✓ Kurulacak${RESET}                                                    ${WHITE}║${RESET}"
    } else {
        Write-Host "  ${WHITE}║${RESET}  ${YELLOW}○ Kurulmayacak${RESET}                                                  ${WHITE}║${RESET}"
    }
    Write-Host "  ${WHITE}╚═══════════════════════════════════════════════════════════════════╝${RESET}"
    
    $docker = Read-Host "  Kurulsun mu? [y/N]"
    $script:Config.InstallDocker = ($docker -eq "y" -or $docker -eq "Y")
    
    Write-Host ""
    
    # Voice
    Write-Host "  ${WHITE}╔═══════════════════════════════════════════════════════════════════╗${RESET}"
    Write-Host "  ${WHITE}║${RESET}  ${CYAN}🎤 VOICE${RESET} ${DIM}(Sesli Asistan)${RESET}                                        ${WHITE}║${RESET}"
    Write-Host "  ${WHITE}║${RESET}                                                                   ${WHITE}║${RESET}"
    Write-Host "  ${WHITE}║${RESET}  ${DIM}Whisper.cpp ile Speech-to-Text${RESET}                                 ${WHITE}║${RESET}"
    Write-Host "  ${WHITE}║${RESET}  ${DIM}Piper ile Text-to-Speech${RESET}                                      ${WHITE}║${RESET}"
    Write-Host "  ${WHITE}║${RESET}  ${DIM}Wake word desteği${RESET}                                             ${WHITE}║${RESET}"
    Write-Host "  ${WHITE}║${RESET}                                                                   ${WHITE}║${RESET}"
    if ($script:Config.InstallVoice) {
        Write-Host "  ${WHITE}║${RESET}  ${GREEN}✓ Kurulacak${RESET}                                                    ${WHITE}║${RESET}"
    } else {
        Write-Host "  ${WHITE}║${RESET}  ${YELLOW}○ Kurulmayacak${RESET}                                                  ${WHITE}║${RESET}"
    }
    Write-Host "  ${WHITE}╚═══════════════════════════════════════════════════════════════════╝${RESET}"
    
    $voice = Read-Host "  Kurulsun mu? [y/N]"
    $script:Config.InstallVoice = ($voice -eq "y" -or $voice -eq "Y")
    
    Write-Host ""
    
    # Dashboard
    Write-Host "  ${WHITE}╔═══════════════════════════════════════════════════════════════════╗${RESET}"
    Write-Host "  ${WHITE}║${RESET}  ${CYAN}📊 DASHBOARD${RESET} ${DIM}(Web Arayüzü)${RESET}                                      ${WHITE}║${RESET}"
    Write-Host "  ${WHITE}║${RESET}                                                                   ${WHITE}║${RESET}"
    Write-Host "  ${WHITE}║${RESET}  ${DIM}Tauri tabanlı masaüstü uygulaması${RESET}                               ${WHITE}║${RESET}"
    Write-Host "  ${WHITE}║${RESET}  ${DIM}WebSocket ile gerçek zamanlı iletişim${RESET}                           ${WHITE}║${RESET}"
    Write-Host "  ${WHITE}║${RESET}  ${DIM}Skill ve tool yönetimi${RESET}                                        ${WHITE}║${RESET}"
    Write-Host "  ${WHITE}║${RESET}                                                                   ${WHITE}║${RESET}"
    if ($script:Config.InstallDashboard) {
        Write-Host "  ${WHITE}║${RESET}  ${GREEN}✓ Kurulacak${RESET}                                                    ${WHITE}║${RESET}"
    } else {
        Write-Host "  ${WHITE}║${RESET}  ${YELLOW}○ Kurulmayacak${RESET}                                                  ${WHITE}║${RESET}"
    }
    Write-Host "  ${WHITE}╚═══════════════════════════════════════════════════════════════════╝${RESET}"
    
    $dashboard = Read-Host "  Kurulsun mu? [y/N]"
    $script:Config.InstallDashboard = ($dashboard -eq "y" -or $dashboard -eq "Y")
    
    Write-Host ""
    Write-Separator
    Write-Host ""
    Write-Host "  ${WHITE}Seçilen Bileşenler:${RESET}"
    if ($script:Config.InstallOllama) { Write-OK "Ollama (Lokal LLM)" }
    if ($script:Config.InstallDocker) { Write-OK "Docker Servisleri" }
    if ($script:Config.InstallVoice) { Write-OK "Voice (Sesli Asistan)" }
    if ($script:Config.InstallDashboard) { Write-OK "Dashboard (Web UI)" }
    if ($script:Config.Provider -ne "ollama") { Write-OK "$($script:Config.Provider.ToUpper()) API" }
    
    return $true
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 7: ÖN KOŞULLAR KURULUMU
# ═══════════════════════════════════════════════════════════════════════════════

function Install-Prerequisites {
    Write-Host ""
    Write-Host "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "${WHITE}  │${RESET} ${BOLD}ÖN KOŞULLAR KURULUYOR...${RESET}                                         ${WHITE}│${RESET}"
    Write-Host "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    # Git
    Write-Step "Git kontrol ediliyor..."
    if (Get-Command git -ErrorAction SilentlyContinue) {
        Write-OK "Git: $(git --version)"
    } else {
        Write-Info "Git kuruluyor..."
        winget install Git.Git --accept-source-agreements --accept-package-agreements
        $env:Path += ";$env:ProgramFiles\Git\cmd"
        Write-OK "Git kuruldu"
    }
    
    # Rust
    Write-Step "Rust kontrol ediliyor..."
    if (Get-Command rustc -ErrorAction SilentlyContinue) {
        Write-OK "Rust: $(rustc --version)"
    } else {
        Write-Info "Rust kuruluyor..."
        winget install Rustlang.Rustup --accept-source-agreements --accept-package-agreements
        $env:Path += ";$env:USERPROFILE\.cargo\bin"
        Write-OK "Rust kuruldu"
    }
    
    # Python
    if ($script:Config.InstallPython) {
        Write-Step "Python kontrol ediliyor..."
        if (Get-Command python -ErrorAction SilentlyContinue) {
            Write-OK "Python: $(python --version 2>&1)"
        } else {
            Write-Info "Python kuruluyor..."
            winget install Python.Python.3.12 --accept-source-agreements --accept-package-agreements
            $pythonPath = "$env:LOCALAPPDATA\Programs\Python\Python312"
            if (Test-Path $pythonPath) {
                $env:Path += ";$pythonPath;$pythonPath\Scripts"
            }
            Write-OK "Python kuruldu"
        }
    }
    
    # Visual Studio Build Tools
    Write-Step "Build Tools kontrol ediliyor..."
    $vsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
    $hasBuildTools = $false
    if (Test-Path $vsWhere) {
        $vsInstall = & $vsWhere -latest -property installationPath 2>$null
        if ($vsInstall) { $hasBuildTools = $true }
    }
    
    if (-not $hasBuildTools) {
        Write-Info "Visual Studio Build Tools kuruluyor (5-10 dk)..."
        winget install Microsoft.VisualStudio.2022.BuildTools --override "--add Microsoft.VisualStudio.Workload.VCTools --passive" --accept-source-agreements
        Write-OK "Build Tools kuruldu"
    } else {
        Write-OK "Build Tools mevcut"
    }
    
    # FFmpeg
    Write-Step "FFmpeg kontrol ediliyor..."
    if (Get-Command ffmpeg -ErrorAction SilentlyContinue) {
        Write-OK "FFmpeg: mevcut"
    } else {
        Write-Info "FFmpeg kuruluyor..."
        winget install Gyan.FFmpeg --accept-source-agreements --accept-package-agreements
        $ffmpegPath = "$env:ProgramFiles\ffmpeg\bin"
        if (Test-Path $ffmpegPath) {
            $env:Path += ";$ffmpegPath"
        }
        Write-OK "FFmpeg kuruldu"
    }
    
    # Ollama
    if ($script:Config.InstallOllama) {
        Write-Step "Ollama kontrol ediliyor..."
        if (Get-Command ollama -ErrorAction SilentlyContinue) {
            Write-OK "Ollama: mevcut"
            
            # Servis kontrol
            $ollamaRunning = Get-Process -Name "ollama" -ErrorAction SilentlyContinue
            if (-not $ollamaRunning) {
                Write-Info "Ollama servisi başlatılıyor..."
                Start-Process "ollama" -ArgumentList "serve" -WindowStyle Hidden
                Start-Sleep -Seconds 3
            }
        } else {
            Write-Info "Ollama kuruluyor..."
            winget install Ollama.Ollama --accept-source-agreements --accept-package-agreements
            $ollamaPath = "$env:LOCALAPPDATA\Programs\Ollama"
            if (Test-Path $ollamaPath) {
                $env:Path += ";$ollamaPath"
            }
            Start-Process "ollama" -ArgumentList "serve" -WindowStyle Hidden
            Start-Sleep -Seconds 5
            Write-OK "Ollama kuruldu"
        }
    }
    
    # Docker Desktop
    if ($script:Config.InstallDocker) {
        Write-Step "Docker kontrol ediliyor..."
        if (Get-Command docker -ErrorAction SilentlyContinue) {
            Write-OK "Docker: mevcut"
        } else {
            Write-Info "Docker Desktop kuruluyor..."
            winget install Docker.DockerDesktop --accept-source-agreements --accept-package-agreements
            Write-Warn "Docker kurulumu tamamlandı - bilgisayarı yeniden başlatmanız gerekebilir"
        }
    }
    
    Write-Host ""
    Write-OK "Tüm ön koşullar hazır!"
    return $true
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 8: KAYNAK İNDİRME
# ═══════════════════════════════════════════════════════════════════════════════

function Download-Source {
    Write-Host ""
    Write-Host "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "${WHITE}  │${RESET} ${BOLD}KAYNAK İNDİRİLİYOR...${RESET}                                              ${WHITE}│${RESET}"
    Write-Host "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    $installDir = $script:Config.InstallDir
    
    if (Test-Path "$installDir\Cargo.toml") {
        Write-Info "Mevcut kurulum bulundu, güncelleniyor..."
        Set-Location $installDir
        git pull 2>$null
        Write-OK "Repository güncellendi"
    } else {
        Write-Info "SENTIENT repository'si klonlanıyor..."
        git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git $installDir
        Set-Location $installDir
        Write-OK "Repository klonlandı"
    }
    
    return $true
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 9: DERLEME
# ═══════════════════════════════════════════════════════════════════════════════

function Build-Project {
    Write-Host ""
    Write-Host "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "${WHITE}  │${RESET} ${BOLD}SENTIENT DERLENİYOR...${RESET}                                             ${WHITE}│${RESET}"
    Write-Host "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    Write-Info "Bu işlem 5-15 dakika sürebilir..."
    Write-Info "İlk derleme uzun sürer, lütfen bekleyin..."
    Write-Host ""
    
    # Python path
    $python = Get-Command python -ErrorAction SilentlyContinue
    if ($python) {
        $env:PYTHON_SYS_EXECUTABLE = $python.Source
    }
    
    # Build
    $buildLog = "$env:TEMP\sentient-build.log"
    cargo build --release 2>&1 | Tee-Object -FilePath $buildLog
    
    if (Test-Path "target\release\sentient.exe") {
        $size = (Get-Item "target\release\sentient.exe").Length / 1MB
        Write-Host ""
        Write-OK "SENTIENT derlendi! ($([math]::Round($size, 1)) MB)"
        return $true
    }
    
    Write-Err "Derleme başarısız!"
    Write-Info "Log: $buildLog"
    return $false
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 10: YAPILANDIRMA
# ═══════════════════════════════════════════════════════════════════════════════

function Configure-Environment {
    Write-Host ""
    Write-Host "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "${WHITE}  │${RESET} ${BOLD}YAPILANDIRMA OLUŞTURULUYOR...${RESET}                                     ${WHITE}│${RESET}"
    Write-Host "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    # .env dosyası
    if (-not (Test-Path ".env")) {
        $envContent = @"
# ════════════════════════════════════════════════════════════════
#  SENTIENT OS - Yapılandırma Dosyası
#  Oluşturulma: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")
# ════════════════════════════════════════════════════════════════

# LLM PROVIDER: $($script:Config.Provider)
"@
        
        switch ($script:Config.Provider) {
            "ollama" {
                $envContent += @"


# Ollama (Lokal - Ücretsiz)
OLLAMA_HOST=http://localhost:11434
OPENAI_API_BASE=http://localhost:11434/v1
OPENAI_API_KEY=ollama
DEFAULT_MODEL=ollama/$($script:Config.Model)
"@
            }
            "openrouter" {
                $key = $script:Config.ApiKeys["openrouter"]
                $envContent += @"


# OpenRouter (200+ Model)
OPENROUTER_API_KEY=$key
DEFAULT_MODEL=openrouter/auto
"@
            }
            "openai" {
                $key = $script:Config.ApiKeys["openai"]
                $envContent += @"


# OpenAI
OPENAI_API_KEY=$key
DEFAULT_MODEL=openai/gpt-4o
"@
            }
            "anthropic" {
                $key = $script:Config.ApiKeys["anthropic"]
                $envContent += @"


# Anthropic
ANTHROPIC_API_KEY=$key
DEFAULT_MODEL=anthropic/claude-4-sonnet
"@
            }
            "deepseek" {
                $key = $script:Config.ApiKeys["deepseek"]
                $envContent += @"


# DeepSeek (EN UCUZ)
DEEPSEEK_API_KEY=$key
DEFAULT_MODEL=deepseek/deepseek-chat
"@
            }
            "groq" {
                $key = $script:Config.ApiKeys["groq"]
                $envContent += @"


# Groq (EN HIZLI)
GROQ_API_KEY=$key
DEFAULT_MODEL=groq/llama-3.3-70b-versatile
"@
            }
            "google" {
                $key = $script:Config.ApiKeys["google"]
                $envContent += @"


# Google AI (Gemini)
GOOGLE_AI_API_KEY=$key
DEFAULT_MODEL=google/gemini-2.0-flash
"@
            }
            default {
                $envContent += @"


# Provider yapılandırması gerekli
# Diğer provider'lar için .env.template dosyasına bakın
"@
            }
        }
        
        $envContent += @"


# ════════════════════════════════════════════════════════════════
#  OPSİYONEL YAPILANDIRMA
# ════════════════════════════════════════════════════════════════

# Voice
VOICE_ENABLED=$($script:Config.InstallVoice)

# Dashboard
DASHBOARD_ENABLED=$($script:Config.InstallDashboard)

# Logging
RUST_LOG=info
"@
        
        Set-Content -Path ".env" -Value $envContent -Encoding UTF8
        Write-OK ".env dosyası oluşturuldu"
    } else {
        Write-OK ".env dosyası zaten mevcut"
    }
    
    # Model indir
    if ($script:Config.DownloadModel -and $script:Config.Provider -eq "ollama") {
        Write-Host ""
        Write-Step "$($script:Config.Model) modeli indiriliyor..."
        ollama pull $script:Config.Model
        Write-OK "Model hazır: $($script:Config.Model)"
    }
    
    return $true
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 11: DOĞRULAMA
# ═══════════════════════════════════════════════════════════════════════════════

function Validate-Installation {
    Write-Host ""
    Write-Host "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "${WHITE}  │${RESET} ${BOLD}KURULUM DOĞRULANIYOR...${RESET}                                          ${WHITE}│${RESET}"
    Write-Host "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    $allOk = $true
    
    # Binary
    if (Test-Path "target\release\sentient.exe") {
        Write-OK "sentient.exe"
    } else {
        Write-Err "sentient.exe bulunamadı"
        $allOk = $false
    }
    
    # .env
    if (Test-Path ".env") {
        Write-OK ".env yapılandırması"
    } else {
        Write-Err ".env bulunamadı"
        $allOk = $false
    }
    
    # Ollama
    if ($script:Config.InstallOllama) {
        if (Get-Command ollama -ErrorAction SilentlyContinue) {
            Write-OK "Ollama"
        } else {
            Write-Warn "Ollama (PATH'e eklenmeli)"
        }
    }
    
    return $allOk
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 12: TAMAMLANDI
# ═══════════════════════════════════════════════════════════════════════════════

function Show-Complete {
    Show-Banner
    
    Write-Host "${GREEN}  ╔═══════════════════════════════════════════════════════════════════╗${RESET}"
    Write-Host "${GREEN}  ║${RESET}                                                                   ${GREEN}║${RESET}"
    Write-Host "${GREEN}  ║${RESET}          ${WHITE}${BOLD}🎉 KURULUM BAŞARIYLA TAMAMLANDI! 🎉${RESET}                       ${GREEN}║${RESET}"
    Write-Host "${GREEN}  ║${RESET}                                                                   ${GREEN}║${RESET}"
    Write-Host "${GREEN}  ╚═══════════════════════════════════════════════════════════════════╝${RESET}"
    Write-Host ""
    
    Write-Separator
    Write-Host ""
    Write-Host "  ${WHITE}KURULUM ÖZETİ${RESET}"
    Write-Host ""
    Write-Host "    ${CYAN}Mod:${RESET}         $($script:Config.Mode.ToUpper())"
    Write-Host "    ${CYAN}Provider:${RESET}    $($script:Config.Provider.ToUpper())"
    Write-Host "    ${CYAN}Model:${RESET}       $($script:Config.Model)"
    Write-Host "    ${CYAN}Dizin:${RESET}       $($script:Config.InstallDir)"
    Write-Host ""
    
    Write-Host "    ${CYAN}Ollama:${RESET}      $(if($script:Config.InstallOllama){'✓'}else{'✗'})"
    Write-Host "    ${CYAN}Docker:${RESET}      $(if($script:Config.InstallDocker){'✓'}else{'✗'})"
    Write-Host "    ${CYAN}Voice:${RESET}       $(if($script:Config.InstallVoice){'✓'}else{'✗'})"
    Write-Host "    ${CYAN}Dashboard:${RESET}   $(if($script:Config.InstallDashboard){'✓'}else{'✗'})"
    Write-Host ""
    Write-Separator
    Write-Host ""
    
    Write-Host "  ${WHITE}KULLANIM${RESET}"
    Write-Host ""
    Write-Host "    ${DIM}# Versiyon kontrolü${RESET}"
    Write-Host "    ${GREEN}.\target\release\sentient.exe --version${RESET}"
    Write-Host ""
    Write-Host "    ${DIM}# Sohbet başlat${RESET}"
    Write-Host "    ${GREEN}.\target\release\sentient.exe chat${RESET}"
    Write-Host ""
    Write-Host "    ${DIM}# Web dashboard${RESET}"
    Write-Host "    ${GREEN}.\target\release\sentient.exe web${RESET}"
    Write-Host ""
    
    if ($script:Config.Provider -eq "ollama") {
        Write-Host "    ${DIM}# Model yönetimi${RESET}"
        Write-Host "    ${GREEN}ollama list${RESET}              ${DIM}# Yüklü modeller${RESET}"
        Write-Host "    ${GREEN}ollama pull <model>${RESET}      ${DIM}# Model indir${RESET}"
        Write-Host ""
    }
    
    Write-Separator
    Write-Host ""
    Write-Host "  ${WHITE}SONRAKİ ADIMLAR${RESET}"
    Write-Host ""
    Write-Host "    1. API key ekleyin:    ${CYAN}notepad .env${RESET}"
    Write-Host "    2. Farklı model indir: ${CYAN}ollama pull deepseek-r1:8b${RESET}"
    Write-Host "    3. Dokümantasyon:      ${CYAN}README.md${RESET}"
    Write-Host ""
    
    Write-Host "${MAGENTA}  ╔═══════════════════════════════════════════════════════════════════╗${RESET}"
    Write-Host "${MAGENTA}  ║${RESET}                                                                   ${MAGENTA}║${RESET}"
    Write-Host "${MAGENTA}  ║${RESET}        ${WHITE}SENTIENT OS${RESET} ${DIM}-${RESET} ${YELLOW}The Operating System That Thinks${RESET}              ${MAGENTA}║${RESET}"
    Write-Host "${MAGENTA}  ║${RESET}                                                                   ${MAGENTA}║${RESET}"
    Write-Host "${MAGENTA}  ╚═══════════════════════════════════════════════════════════════════╝${RESET}"
    Write-Host ""
    
    # PATH'e ekle
    if ($script:Config.AddToPath) {
        $sentientPath = "$($script:Config.InstallDir)\target\release"
        $currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
        if ($currentPath -notlike "*$sentientPath*") {
            [Environment]::SetEnvironmentVariable("PATH", "$currentPath;$sentientPath", "User")
            Write-OK "SENTIENT PATH'e eklendi"
        }
    }
}

# ═══════════════════════════════════════════════════════════════════════════════
#  KALDIRMA
# ═══════════════════════════════════════════════════════════════════════════════

function Uninstall-Sentient {
    Show-Banner
    
    Write-Host "  ${RED}KALDIRMA İŞLEMİ${RESET}"
    Write-Host ""
    
    $confirm = Read-Host "  SENTIENT OS'i kaldırmak istediğinizden emin misiniz? [y/N]"
    if ($confirm -ne "y" -and $confirm -ne "Y") {
        Write-Info "İptal edildi"
        return
    }
    
    $installDir = $script:Config.InstallDir
    
    # Dizini sil
    if (Test-Path $installDir) {
        Write-Info "$installDir siliniyor..."
        Remove-Item -Path $installDir -Recurse -Force
        Write-OK "Dizin silindi"
    }
    
    # PATH'ten kaldır
    $currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    $newPath = ($currentPath -split ';' | Where-Object { $_ -notlike "*sentient*" }) -join ';'
    [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
    Write-OK "PATH'ten kaldırıldı"
    
    Write-Host ""
    Write-OK "SENTIENT OS başarıyla kaldırıldı"
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ANA FONKSİYON
# ═══════════════════════════════════════════════════════════════════════════════

function Main {
    # Kaldırma modu
    if ($Uninstall) {
        Uninstall-Sentient
        return
    }
    
    # Adımları çalıştır
    foreach ($step in $script:Steps) {
        $script:CurrentStep++
        $fn = $step.Fn
        
        Write-Host ""
        Write-Host "${DIM}  ─────────────────────────────────────────────────────────────────────${RESET}"
        Write-Host "${WHITE}  ADIM $($script:CurrentStep)/$($script:Steps.Count): $($step.Name)${RESET}"
        Write-Host "${DIM}  ─────────────────────────────────────────────────────────────────────${RESET}"
        
        $result = & $fn
        
        if (-not $result) {
            Write-Err "$($step.Name) başarısız!"
            Write-Info "Kurulum durduruldu."
            return
        }
    }
    
    # Tamamlandı
    Show-Complete
}

# Script'i çalıştır
Main
