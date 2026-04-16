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
#  PARAMETRELER (Opsiyonel):
#    -Quick        : Hızlı kurulum
#    -Standard     : Standart kurulum (varsayılan)
#    -Full         : Tam kurulum
#    -Custom       : Özelleştirilmiş kurulum
#    -Provider     : LLM provider (ollama, openrouter, openai, vs.)
#    -Model        : Model adı
#    -ApiKey       : API key
#    -Uninstall    : Kaldır
# ═══════════════════════════════════════════════════════════════════════════════

param(
    [switch]$Quick,
    [switch]$Standard,
    [switch]$Full,
    [switch]$Custom,
    [string]$Provider,
    [string]$Model,
    [string]$ApiKey,
    [switch]$Uninstall
)

# ═══════════════════════════════════════════════════════════════════════════════
#  ANSI RENKLERİ
# ═══════════════════════════════════════════════════════════════════════════════

$ESC = [char]27
$RED = "$ESC[91m"
$GREEN = "$ESC[92m"
$YELLOW = "$ESC[93m"
$BLUE = "$ESC[94m"
$MAGENTA = "$ESC[95m"
$CYAN = "$ESC[96m"
$WHITE = "$ESC[97m"
$BOLD = "$ESC[1m"
$DIM = "$ESC[2m"
$RESET = "$ESC[0m"

# ═══════════════════════════════════════════════════════════════════════════════
#  LOG FONKSİYONLARI
# ═══════════════════════════════════════════════════════════════════════════════

function Write-Step { param($msg) Write-Host "${CYAN}━━━${RESET} $msg" }
function Write-Info { param($msg) Write-Host "  ${BLUE}ℹ${RESET}  $msg" }
function Write-OK { param($msg) Write-Host "  ${GREEN}✓${RESET}  $msg" }
function Write-Warn { param($msg) Write-Host "  ${YELLOW}⚠${RESET}  $msg" }
function Write-Err { param($msg) Write-Host "  ${RED}✗${RESET}  $msg" }
function Write-Sep { Write-Host "${DIM}  ═════════════════════════════════════════════════════════════${RESET}" }

# ═══════════════════════════════════════════════════════════════════════════════
#  GLOBAL YAPILANDIRMA
# ═══════════════════════════════════════════════════════════════════════════════

$Config = @{
    Mode = "standard"
    Provider = "ollama"
    Model = "gemma3:27b"
    InstallDir = "$env:USERPROFILE\.sentient"
    InstallOllama = $true
    InstallDocker = $false
    InstallVoice = $false
    InstallDashboard = $false
    DownloadModel = $true
    SystemRAM = 0
    SystemVRAM = 0
    HasNvidia = $false
    CpuCores = 0
    GpuName = ""
    ApiKeys = @{}
}

# ═══════════════════════════════════════════════════════════════════════════════
#  BANNER
# ═══════════════════════════════════════════════════════════════════════════════

function Show-Banner {
    Clear-Host
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

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 1: HOŞGELDİNİZ
# ═══════════════════════════════════════════════════════════════════════════════

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
    Write-Host ""
    Write-Sep
    Write-Host ""
    Write-Host "  ${YELLOW}Enter${RESET} tuşuna basarak devam edin..."
    $null = Read-Host
    return $true
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 2: LİSANS
# ═══════════════════════════════════════════════════════════════════════════════

function Show-License {
    Write-Host ""
    Write-Host "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "${WHITE}  │${RESET} ${BOLD}LİSANS SÖZLEŞMESİ${RESET}                                                   ${WHITE}│${RESET}"
    Write-Host "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    Write-Host "  ${CYAN}◆${RESET} ${WHITE}Lisans Türü:${RESET} AGPL v3 (Affero GNU General Public License)"
    Write-Host "  ${CYAN}◆${RESET} ${WHITE}Kaynak Kod:${RESET} https://github.com/nexsusagent-coder/SENTIENT_CORE"
    Write-Host ""
    
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
    
    Write-Host "  Lisans sözleşmesini kabul ediyor musunuz?"
    Write-Host ""
    Write-Host "    ${WHITE}[${GREEN}Y${WHITE}]${RESET} Evet, kabul ediyorum"
    Write-Host "    ${WHITE}[${RED}N${WHITE}]${RESET} Hayır, çık"
    Write-Host ""
    
    while ($true) {
        $choice = Read-Host "  Seçiminiz [Y/N]"
        if ($choice -eq "Y" -or $choice -eq "y" -or $choice -eq "") {
            Write-OK "Lisans kabul edildi"
            return $true
        }
        if ($choice -eq "N" -or $choice -eq "n") {
            Write-Err "Kurulum iptal edildi"
            exit 0
        }
    }
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 3: SİSTEM ANALİZİ
# ═══════════════════════════════════════════════════════════════════════════════

function Analyze-System {
    Write-Host ""
    Write-Host "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "${WHITE}  │${RESET} ${BOLD}SİSTEM ANALİZİ${RESET}                                                     ${WHITE}│${RESET}"
    Write-Host "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    # OS
    $osInfo = Get-CimInstance Win32_OperatingSystem
    Write-Info "İşletim Sistemi: $($osInfo.Caption)"
    
    # CPU
    $cpu = Get-CimInstance Win32_Processor
    $Config.CpuCores = $cpu.NumberOfLogicalProcessors
    Write-OK "CPU: $($cpu.Name) ($($Config.CpuCores) çekirdek)"
    
    # RAM
    $ram = (Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory / 1GB
    $Config.SystemRAM = [math]::Round($ram, 1)
    if ($ram -ge 32) {
        Write-OK "RAM: $($Config.SystemRAM) GB ${GREEN}(Mükemmel)${RESET}"
    } elseif ($ram -ge 16) {
        Write-OK "RAM: $($Config.SystemRAM) GB ${GREEN}(İyi)${RESET}"
    } elseif ($ram -ge 8) {
        Write-Warn "RAM: $($Config.SystemRAM) GB ${YELLOW}(Minimum)${RESET}"
    } else {
        Write-Err "RAM: $($Config.SystemRAM) GB ${RED}(Yetersiz)${RESET}"
    }
    
    # GPU
    $gpus = Get-CimInstance Win32_VideoController
    $nvidiaGpu = $gpus | Where-Object { $_.Name -match "NVIDIA|GeForce|RTX|GTX|Quadro" }
    
    if ($nvidiaGpu) {
        $Config.HasNvidia = $true
        $gpu = $nvidiaGpu[0]
        $Config.GpuName = $gpu.Name
        
        $vramBytes = $gpu.AdapterRAM
        if ($vramBytes) {
            $Config.SystemVRAM = [math]::Round($vramBytes / 1GB, 0)
            Write-OK "GPU: $($gpu.Name) ${GREEN}($($Config.SystemVRAM)GB VRAM)${RESET}"
        } else {
            # Model adından tahmin
            if ($gpu.Name -match "4090|3090") { $Config.SystemVRAM = 24 }
            elseif ($gpu.Name -match "4080|3080") { $Config.SystemVRAM = 16 }
            elseif ($gpu.Name -match "4070|3070|4060 Ti") { $Config.SystemVRAM = 12 }
            elseif ($gpu.Name -match "4060|3060|2080") { $Config.SystemVRAM = 8 }
            else { $Config.SystemVRAM = 4 }
            Write-OK "GPU: $($gpu.Name) ${YELLOW}(~$($Config.SystemVRAM)GB VRAM)${RESET}"
        }
    } else {
        Write-Warn "GPU: NVIDIA GPU bulunamadı - CPU inference kullanılacak"
    }
    
    # Disk
    $disk = (Get-CimInstance Win32_LogicalDisk -Filter "DeviceID='C:'").FreeSpace / 1GB
    Write-Info "Disk (C:): $([math]::Round($disk, 1)) GB boş"
    
    if ($disk -lt 20) {
        Write-Err "Yetersiz disk alanı! En az 20 GB gerekli."
        return $false
    }
    
    # Sistem profili ve model önerisi
    Write-Host ""
    Write-Sep
    Write-Host ""
    Write-Host "  ${WHITE}Sistem Profiliniz:${RESET}"
    Write-Host ""
    
    if ($Config.SystemRAM -ge 64 -and $Config.SystemVRAM -ge 24) {
        Write-Host "    ${CYAN}◆${RESET} Profil:    ${GREEN}WORKSTATION${RESET}"
        Write-Host "    ${CYAN}◆${RESET} Öneri:     Büyük modeller (70B+) için uygun"
        $Config.Model = "llama3.3:70b"
    } elseif ($Config.SystemRAM -ge 32 -and $Config.SystemVRAM -ge 16) {
        Write-Host "    ${CYAN}◆${RESET} Profil:    ${GREEN}HIGH-END${RESET}"
        Write-Host "    ${CYAN}◆${RESET} Öneri:     Orta-büyük modeller (27B-70B) için uygun"
        $Config.Model = "gemma3:27b"
    } elseif ($Config.SystemRAM -ge 16 -and $Config.SystemVRAM -ge 8) {
        Write-Host "    ${CYAN}◆${RESET} Profil:    ${YELLOW}MID-RANGE${RESET}"
        Write-Host "    ${CYAN}◆${RESET} Öneri:     Orta boy modeller (8B-27B) için uygun"
        $Config.Model = "gemma3:12b"
    } elseif ($Config.SystemRAM -ge 8) {
        Write-Host "    ${CYAN}◆${RESET} Profil:    ${YELLOW}ENTRY-LEVEL${RESET}"
        Write-Host "    ${CYAN}◆${RESET} Öneri:     Küçük modeller veya API kullanımı önerilir"
        $Config.Model = "qwen3:30b-a3b"
    } else {
        Write-Host "    ${CYAN}◆${RESET} Profil:    ${RED}MINIMAL${RESET}"
        Write-Host "    ${CYAN}◆${RESET} Öneri:     API modu önerilir (Cloud LLM)"
        $Config.Provider = "openrouter"
        $Config.InstallOllama = $false
        $Config.DownloadModel = $false
    }
    
    Write-Host "    ${CYAN}◆${RESET} Model:     $($Config.Model)"
    Write-Host ""
    Write-Sep
    
    return $true
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 4: KURULUM MODU SEÇİMİ
# ═══════════════════════════════════════════════════════════════════════════════

function Select-Mode {
    Write-Host ""
    Write-Host "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "${WHITE}  │${RESET} ${BOLD}KURULUM MODU SEÇİMİ${RESET}                                                ${WHITE}│${RESET}"
    Write-Host "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    # Komut satırından mod belirlendiyse
    if ($Quick) { $Config.Mode = "quick" }
    elseif ($Full) { $Config.Mode = "full" }
    elseif ($Custom) { $Config.Mode = "custom" }
    elseif ($Standard) { $Config.Mode = "standard" }
    else {
        # İnteraktif seçim
        Write-Host "  Kurulum modunu seçin:"
        Write-Host ""
        
        # Quick
        Write-Host "  ${WHITE}┌─────────────────────────────────────────────────────────────────┐${RESET}"
        Write-Host "  ${WHITE}│${RESET} ${GREEN}${BOLD}[1] QUICK${RESET} ${DIM}- Hızlı Başlangıç${RESET}                                    ${WHITE}│${RESET}"
        Write-Host "  ${WHITE}│${RESET}   ${DIM}Süre: ~5 dk • CLI + Ollama + Küçük model${RESET}                  ${WHITE}│${RESET}"
        Write-Host "  ${WHITE}│${RESET}   ${DIM}Yeni başlayanlar için ideal${RESET}                                  ${WHITE}│${RESET}"
        Write-Host "  ${WHITE}└─────────────────────────────────────────────────────────────────┘${RESET}"
        Write-Host ""
        
        # Standard
        Write-Host "  ${WHITE}┌─────────────────────────────────────────────────────────────────┐${RESET}"
        Write-Host "  ${WHITE}│${RESET} ${YELLOW}${BOLD}[2] STANDARD${RESET} ${DIM}- Önerilen${RESET} ${GREEN}★${RESET}                                   ${WHITE}│${RESET}"
        Write-Host "  ${WHITE}│${RESET}   ${DIM}Süre: ~15 dk • CLI + Araçlar + Uygun model${RESET}               ${WHITE}│${RESET}"
        Write-Host "  ${WHITE}│${RESET}   ${DIM}Çoğu kullanıcı için en iyi seçenek${RESET}                           ${WHITE}│${RESET}"
        Write-Host "  ${WHITE}└─────────────────────────────────────────────────────────────────┘${RESET}"
        Write-Host ""
        
        # Full
        Write-Host "  ${WHITE}┌─────────────────────────────────────────────────────────────────┐${RESET}"
        Write-Host "  ${WHITE}│${RESET} ${MAGENTA}${BOLD}[3] FULL${RESET} ${DIM}- Tam Kurulum${RESET}                                        ${WHITE}│${RESET}"
        Write-Host "  ${WHITE}│${RESET}   ${DIM}Süre: ~30 dk • Docker + Voice + Dashboard${RESET}                 ${WHITE}│${RESET}"
        Write-Host "  ${WHITE}│${RESET}   ${DIM}Geliştiriciler ve power users için${RESET}                           ${WHITE}│${RESET}"
        Write-Host "  ${WHITE}└─────────────────────────────────────────────────────────────────┘${RESET}"
        Write-Host ""
        
        # Custom
        Write-Host "  ${WHITE}┌─────────────────────────────────────────────────────────────────┐${RESET}"
        Write-Host "  ${WHITE}│${RESET} ${BLUE}${BOLD}[4] CUSTOM${RESET} ${DIM}- Özelleştirilmiş${RESET}                                      ${WHITE}│${RESET}"
        Write-Host "  ${WHITE}│${RESET}   ${DIM}Süre: Değişken • Her bileşeni kendiniz seçin${RESET}                 ${WHITE}│${RESET}"
        Write-Host "  ${WHITE}│${RESET}   ${DIM}Deneyimli kullanıcılar için${RESET}                                    ${WHITE}│${RESET}"
        Write-Host "  ${WHITE}└─────────────────────────────────────────────────────────────────┘${RESET}"
        Write-Host ""
        
        while ($true) {
            $choice = Read-Host "  Seçiminiz [1-4]"
            switch ($choice) {
                "1" { $Config.Mode = "quick"; break }
                "2" { $Config.Mode = "standard"; break }
                "3" { $Config.Mode = "full"; break }
                "4" { $Config.Mode = "custom"; break }
                "" { $Config.Mode = "standard"; break }
            }
            if ($Config.Mode) { break }
        }
    }
    
    # Mod ayarlarını uygula
    switch ($Config.Mode) {
        "quick" {
            $Config.InstallOllama = $true
            $Config.InstallDocker = $false
            $Config.InstallVoice = $false
            $Config.InstallDashboard = $false
            $Config.Model = "qwen3:30b-a3b"
        }
        "standard" {
            $Config.InstallOllama = $true
            $Config.InstallDocker = $false
            $Config.InstallVoice = $false
            $Config.InstallDashboard = $false
        }
        "full" {
            $Config.InstallOllama = $true
            $Config.InstallDocker = $true
            $Config.InstallVoice = $true
            $Config.InstallDashboard = $true
        }
    }
    
    Write-OK "$($Config.Mode.ToUpper()) modu seçildi"
    return $true
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 5: PROVIDER SEÇİMİ
# ═══════════════════════════════════════════════════════════════════════════════

function Select-Provider {
    # Komut satırından provider belirlendiyse
    if ($Provider -ne "") {
        $Config.Provider = $Provider.ToLower()
        Write-Info "Provider: $($Config.Provider.ToUpper())"
        if ($ApiKey -ne "") {
            $Config.ApiKeys[$Config.Provider] = $ApiKey
        }
        if ($Config.Provider -ne "ollama") {
            $Config.InstallOllama = $false
            $Config.DownloadModel = $false
        }
        return $true
    }
    
    # Custom mod değilse varsayılan
    if ($Config.Mode -ne "custom") {
        Write-Info "Provider: OLLAMA (lokal)"
        return $true
    }
    
    Write-Host ""
    Write-Host "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "${WHITE}  │${RESET} ${BOLD}LLM PROVIDER SEÇİMİ${RESET}                                                ${WHITE}│${RESET}"
    Write-Host "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    Write-Host "  ${GREEN}╔═══════════════════════════════════════════════════════════════════╗${RESET}"
    Write-Host "  ${GREEN}║${RESET}              ${WHITE}${BOLD}LOKAL MODELLER${RESET} ${DIM}(Ücretsiz)${RESET}                           ${GREEN}║${RESET}"
    Write-Host "  ${GREEN}╠═══════════════════════════════════════════════════════════════════╣${RESET}"
    Write-Host "  ${GREEN}║${RESET}  ${WHITE}[1]${RESET} Ollama        En popüler, 50K+ model               ${GREEN}║${RESET}"
    Write-Host "  ${GREEN}║${RESET}  ${WHITE}[2]${RESET} LM Studio     GUI ile model yönetimi               ${GREEN}║${RESET}"
    Write-Host "  ${GREEN}║${RESET}  ${WHITE}[3]${RESET} vLLM          Yüksek performans server            ${GREEN}║${RESET}"
    Write-Host "  ${GREEN}╚═══════════════════════════════════════════════════════════════════╝${RESET}"
    Write-Host ""
    
    Write-Host "  ${YELLOW}╔═══════════════════════════════════════════════════════════════════╗${RESET}"
    Write-Host "  ${YELLOW}║${RESET}              ${WHITE}${BOLD}CLOUD API${RESET} ${DIM}(API Key Gerekli)${RESET}                          ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}╠═══════════════════════════════════════════════════════════════════╣${RESET}"
    Write-Host "  ${YELLOW}║${RESET}  ${WHITE}[4]${RESET} OpenRouter   200+ model, \$5 ücretsiz                ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}║${RESET}  ${WHITE}[5]${RESET} OpenAI        GPT-4o, o1, o3                       ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}║${RESET}  ${WHITE}[6]${RESET} Anthropic     Claude 4 Sonnet, Opus 4.1           ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}║${RESET}  ${WHITE}[7]${RESET} DeepSeek      EN UCUZ - V3, R1                    ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}║${RESET}  ${WHITE}[8]${RESET} Google AI     Gemini Flash (FREE tier!)           ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}║${RESET}  ${WHITE}[9]${RESET} Groq          EN HIZLI - Llama 3.3 70B            ${YELLOW}║${RESET}"
    Write-Host "  ${YELLOW}╚═══════════════════════════════════════════════════════════════════╝${RESET}"
    Write-Host ""
    
    $choice = Read-Host "  Provider seçiniz [1-9]"
    
    $providers = @("ollama", "lmstudio", "vllm", "openrouter", "openai", "anthropic", "deepseek", "google", "groq")
    $idx = [int]$choice - 1
    if ($idx -ge 0 -and $idx -lt $providers.Count) {
        $Config.Provider = $providers[$idx]
    }
    
    # API Key gerekli mi?
    $needsKey = @("openrouter", "openai", "anthropic", "deepseek", "google", "groq")
    if ($needsKey -contains $Config.Provider) {
        Write-Host ""
        $urls = @{
            "openrouter" = "https://openrouter.ai/keys"
            "openai" = "https://platform.openai.com/api-keys"
            "anthropic" = "https://console.anthropic.com/settings/keys"
            "deepseek" = "https://platform.deepseek.com/api_keys"
            "google" = "https://aistudio.google.com/apikey"
            "groq" = "https://console.groq.com/keys"
        }
        Write-Host "  ${YELLOW}API Key Gerekli!${RESET}"
        Write-Host "  Almak için: ${CYAN}$($urls[$Config.Provider])${RESET}"
        Write-Host ""
        $key = Read-Host "  API Key"
        if ($key -ne "") {
            $Config.ApiKeys[$Config.Provider] = $key
        }
        $Config.InstallOllama = $false
        $Config.DownloadModel = $false
    }
    
    Write-OK "Provider: $($Config.Provider.ToUpper())"
    return $true
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 6: MODEL SEÇİMİ
# ═══════════════════════════════════════════════════════════════════════════════

function Select-Model {
    if ($Model -ne "") {
        $Config.Model = $Model
        Write-Info "Model: $Model"
        return $true
    }
    
    if ($Config.Provider -notin @("ollama", "lmstudio", "vllm")) {
        Write-Info "Cloud provider - model .env'de yapılandırılacak"
        return $true
    }
    
    if ($Config.Mode -ne "custom") {
        Write-Info "Model: $($Config.Model)"
        return $true
    }
    
    Write-Host ""
    Write-Host "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "${WHITE}  │${RESET} ${BOLD}MODEL SEÇİMİ${RESET} ${DIM}(VRAM: $($Config.SystemVRAM) GB)${RESET}                              ${WHITE}│${RESET}"
    Write-Host "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    $vram = $Config.SystemVRAM
    
    if ($vram -ge 24) {
        Write-Host "  ${GREEN}[1]${RESET} llama3.3:70b     70B, güçlü reasoning"
        Write-Host "  ${GREEN}[2]${RESET} deepseek-r1:67b  67B, matematik/kod"
        Write-Host "  ${GREEN}[3]${RESET} gemma3:27b       27B, dengeli"
        $models = @("llama3.3:70b", "deepseek-r1:67b", "gemma3:27b")
    } elseif ($vram -ge 16) {
        Write-Host "  ${YELLOW}[1]${RESET} gemma3:27b       27B, dengeli"
        Write-Host "  ${YELLOW}[2]${RESET} gemma3:12b       12B, hızlı"
        Write-Host "  ${YELLOW}[3]${RESET} mistral-small    24B, Avrupa"
        $models = @("gemma3:27b", "gemma3:12b", "mistral-small3.1")
    } elseif ($vram -ge 8) {
        Write-Host "  ${YELLOW}[1]${RESET} deepseek-r1:8b   8B, reasoning"
        Write-Host "  ${YELLOW}[2]${RESET} qwen2.5-coder    7B, kod"
        Write-Host "  ${YELLOW}[3]${RESET} gemma3:12b       12B, dengeli"
        $models = @("deepseek-r1:8b", "qwen2.5-coder:7b", "gemma3:12b")
    } else {
        Write-Host "  ${RED}[1]${RESET} qwen3:30b-a3b   30B MoE (3B aktif) - ÖNERİLEN"
        Write-Host "  ${RED}[2]${RESET} phi4-mini       3.8B, Microsoft"
        Write-Host "  ${RED}[3]${RESET} llama3.2:3b     3B, Meta"
        $models = @("qwen3:30b-a3b", "phi4-mini", "llama3.2:3b")
    }
    
    Write-Host "  ${DIM}[0]${RESET} Model indirmeden devam et"
    Write-Host ""
    
    $choice = Read-Host "  Seçiminiz"
    
    if ($choice -eq "0") {
        $Config.DownloadModel = $false
    } else {
        $idx = [int]$choice - 1
        if ($idx -ge 0 -and $idx -lt $models.Count) {
            $Config.Model = $models[$idx]
        }
    }
    
    Write-OK "Model: $($Config.Model)"
    return $true
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 7: BİLEŞEN SEÇİMİ (CUSTOM)
# ═══════════════════════════════════════════════════════════════════════════════

function Select-Components {
    if ($Config.Mode -ne "custom") {
        return $true
    }
    
    Write-Host ""
    Write-Host "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "${WHITE}  │${RESET} ${BOLD}BİLEŞEN SEÇİMİ${RESET}                                                    ${WHITE}│${RESET}"
    Write-Host "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    # Ollama
    $choice = Read-Host "  Ollama kurulsun mu? [Y/n]"
    $Config.InstallOllama = ($choice -ne "n" -and $choice -ne "N")
    
    # Docker
    $choice = Read-Host "  Docker servisleri kurulsun mu? [y/N]"
    $Config.InstallDocker = ($choice -eq "y" -or $choice -eq "Y")
    
    # Voice
    $choice = Read-Host "  Voice (sesli asistan) kurulsun mu? [y/N]"
    $Config.InstallVoice = ($choice -eq "y" -or $choice -eq "Y")
    
    # Dashboard
    $choice = Read-Host "  Dashboard (Web UI) kurulsun mu? [y/N]"
    $Config.InstallDashboard = ($choice -eq "y" -or $choice -eq "Y")
    
    Write-Host ""
    Write-Host "  ${WHITE}Seçilenler:${RESET}"
    if ($Config.InstallOllama) { Write-OK "Ollama" }
    if ($Config.InstallDocker) { Write-OK "Docker Servisleri" }
    if ($Config.InstallVoice) { Write-OK "Voice" }
    if ($Config.InstallDashboard) { Write-OK "Dashboard" }
    
    return $true
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 8: ÖN KOŞULLAR
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
        winget install Git.Git --accept-source-agreements --accept-package-agreements 2>$null
        $env:Path += ";$env:ProgramFiles\Git\cmd"
        Write-OK "Git kuruldu"
    }
    
    # Rust
    Write-Step "Rust kontrol ediliyor..."
    if (Get-Command rustc -ErrorAction SilentlyContinue) {
        Write-OK "Rust: $(rustc --version)"
    } else {
        Write-Info "Rust kuruluyor..."
        winget install Rustlang.Rustup --accept-source-agreements --accept-package-agreements 2>$null
        $env:Path += ";$env:USERPROFILE\.cargo\bin"
        Write-OK "Rust kuruldu"
    }
    
    # Build Tools
    Write-Step "Build Tools kontrol ediliyor..."
    $vsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
    if (-not (Test-Path $vsWhere)) {
        Write-Info "Visual Studio Build Tools kuruluyor..."
        winget install Microsoft.VisualStudio.2022.BuildTools --override "--add Microsoft.VisualStudio.Workload.VCTools --passive" --accept-source-agreements 2>$null
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
        winget install Gyan.FFmpeg --accept-source-agreements --accept-package-agreements 2>$null
        Write-OK "FFmpeg kuruldu"
    }
    
    # Ollama
    if ($Config.InstallOllama) {
        Write-Step "Ollama kontrol ediliyor..."
        if (Get-Command ollama -ErrorAction SilentlyContinue) {
            Write-OK "Ollama: mevcut"
        } else {
            Write-Info "Ollama kuruluyor..."
            winget install Ollama.Ollama --accept-source-agreements --accept-package-agreements 2>$null
            Start-Process "ollama" -ArgumentList "serve" -WindowStyle Hidden
            Start-Sleep -Seconds 5
            Write-OK "Ollama kuruldu"
        }
    }
    
    # Docker
    if ($Config.InstallDocker) {
        Write-Step "Docker kontrol ediliyor..."
        if (Get-Command docker -ErrorAction SilentlyContinue) {
            Write-OK "Docker: mevcut"
        } else {
            Write-Info "Docker Desktop kuruluyor..."
            winget install Docker.DockerDesktop --accept-source-agreements --accept-package-agreements 2>$null
            Write-Warn "Docker kurulumu tamamlandı - yeniden başlatma gerekebilir"
        }
    }
    
    Write-Host ""
    Write-OK "Tüm ön koşullar hazır!"
    return $true
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 9: KAYNAK İNDİRME
# ═══════════════════════════════════════════════════════════════════════════════

function Download-Source {
    Write-Host ""
    Write-Host "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "${WHITE}  │${RESET} ${BOLD}KAYNAK İNDİRİLİYOR...${RESET}                                              ${WHITE}│${RESET}"
    Write-Host "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    $installDir = $Config.InstallDir
    
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
#  ADIM 10: DERLEME
# ═══════════════════════════════════════════════════════════════════════════════

function Build-Project {
    Write-Host ""
    Write-Host "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "${WHITE}  │${RESET} ${BOLD}SENTIENT DERLENİYOR...${RESET}                                             ${WHITE}│${RESET}"
    Write-Host "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    Write-Info "Bu işlem 5-15 dakika sürebilir..."
    Write-Host ""
    
    $python = Get-Command python -ErrorAction SilentlyContinue
    if ($python) {
        $env:PYTHON_SYS_EXECUTABLE = $python.Source
    }
    
    cargo build --release 2>&1
    
    if (Test-Path "target\release\sentient.exe") {
        $size = (Get-Item "target\release\sentient.exe").Length / 1MB
        Write-Host ""
        Write-OK "SENTIENT derlendi! ($([math]::Round($size, 1)) MB)"
        return $true
    }
    
    Write-Err "Derleme başarısız!"
    return $false
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 11: YAPILANDIRMA
# ═══════════════════════════════════════════════════════════════════════════════

function Configure-Environment {
    Write-Host ""
    Write-Host "${WHITE}  ┌─────────────────────────────────────────────────────────────────────┐${RESET}"
    Write-Host "${WHITE}  │${RESET} ${BOLD}YAPILANDIRMA OLUŞTURULUYOR...${RESET}                                     ${WHITE}│${RESET}"
    Write-Host "${WHITE}  └─────────────────────────────────────────────────────────────────────┘${RESET}"
    Write-Host ""
    
    if (-not (Test-Path ".env")) {
        $envContent = "# SENTIENT OS - Yapılandırma`n# Oluşturulma: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')`n`n"
        
        switch ($Config.Provider) {
            "ollama" {
                $envContent += "OLLAMA_HOST=http://localhost:11434`nDEFAULT_MODEL=ollama/$($Config.Model)`n"
            }
            "openrouter" {
                $envContent += "OPENROUTER_API_KEY=$($Config.ApiKeys['openrouter'])`nDEFAULT_MODEL=openrouter/auto`n"
            }
            "openai" {
                $envContent += "OPENAI_API_KEY=$($Config.ApiKeys['openai'])`nDEFAULT_MODEL=openai/gpt-4o`n"
            }
            "anthropic" {
                $envContent += "ANTHROPIC_API_KEY=$($Config.ApiKeys['anthropic'])`nDEFAULT_MODEL=anthropic/claude-4-sonnet`n"
            }
            "deepseek" {
                $envContent += "DEEPSEEK_API_KEY=$($Config.ApiKeys['deepseek'])`nDEFAULT_MODEL=deepseek/deepseek-chat`n"
            }
            "groq" {
                $envContent += "GROQ_API_KEY=$($Config.ApiKeys['groq'])`nDEFAULT_MODEL=groq/llama-3.3-70b-versatile`n"
            }
            "google" {
                $envContent += "GOOGLE_AI_API_KEY=$($Config.ApiKeys['google'])`nDEFAULT_MODEL=google/gemini-2.0-flash`n"
            }
        }
        
        $envContent += "`nVOICE_ENABLED=$($Config.InstallVoice)`nDASHBOARD_ENABLED=$($Config.InstallDashboard)`nRUST_LOG=info`n"
        
        Set-Content -Path ".env" -Value $envContent -Encoding UTF8
        Write-OK ".env dosyası oluşturuldu"
    } else {
        Write-OK ".env dosyası zaten mevcut"
    }
    
    # Model indir
    if ($Config.DownloadModel -and $Config.Provider -eq "ollama") {
        Write-Host ""
        Write-Step "$($Config.Model) modeli indiriliyor..."
        ollama pull $Config.Model
        Write-OK "Model hazır: $($Config.Model)"
    }
    
    return $true
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ADIM 12: TAMAMLAMA
# ═══════════════════════════════════════════════════════════════════════════════

function Show-Complete {
    Show-Banner
    
    Write-Host "${GREEN}  ╔═══════════════════════════════════════════════════════════════════╗${RESET}"
    Write-Host "${GREEN}  ║${RESET}          ${WHITE}${BOLD}🎉 KURULUM BAŞARIYLA TAMAMLANDI! 🎉${RESET}                       ${GREEN}║${RESET}"
    Write-Host "${GREEN}  ╚═══════════════════════════════════════════════════════════════════╝${RESET}"
    Write-Host ""
    
    Write-Sep
    Write-Host ""
    Write-Host "  ${WHITE}KURULUM ÖZETİ${RESET}"
    Write-Host ""
    Write-Host "    ${CYAN}Mod:${RESET}         $($Config.Mode.ToUpper())"
    Write-Host "    ${CYAN}Provider:${RESET}    $($Config.Provider.ToUpper())"
    Write-Host "    ${CYAN}Model:${RESET}       $($Config.Model)"
    Write-Host "    ${CYAN}Dizin:${RESET}       $($Config.InstallDir)"
    Write-Host ""
    Write-Host "    ${CYAN}Ollama:${RESET}      $(if($Config.InstallOllama){'✓'}else{'✗'})"
    Write-Host "    ${CYAN}Docker:${RESET}      $(if($Config.InstallDocker){'✓'}else{'✗'})"
    Write-Host "    ${CYAN}Voice:${RESET}       $(if($Config.InstallVoice){'✓'}else{'✗'})"
    Write-Host "    ${CYAN}Dashboard:${RESET}   $(if($Config.InstallDashboard){'✓'}else{'✗'})"
    Write-Host ""
    Write-Sep
    Write-Host ""
    
    Write-Host "  ${WHITE}KULLANIM${RESET}"
    Write-Host ""
    Write-Host "    ${GREEN}.\target\release\sentient.exe --version${RESET}"
    Write-Host "    ${GREEN}.\target\release\sentient.exe chat${RESET}"
    Write-Host "    ${GREEN}.\target\release\sentient.exe web${RESET}"
    Write-Host ""
    Write-Sep
    Write-Host ""
    
    # PATH'e ekle
    $sentientPath = "$($Config.InstallDir)\target\release"
    $currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    if ($currentPath -notlike "*$sentientPath*") {
        [Environment]::SetEnvironmentVariable("PATH", "$currentPath;$sentientPath", "User")
        Write-OK "SENTIENT PATH'e eklendi"
    }
    
    Write-Host ""
    Write-Host "${MAGENTA}  ╔═══════════════════════════════════════════════════════════════════╗${RESET}"
    Write-Host "${MAGENTA}  ║${NC}        ${WHITE}SENTIENT OS${NC} - ${YELLOW}The Operating System That Thinks${NC}              ${MAGENTA}║${NC}"
    Write-Host "${MAGENTA}  ╚═══════════════════════════════════════════════════════════════════╝${RESET}"
    Write-Host ""
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
    
    $installDir = $Config.InstallDir
    
    if (Test-Path $installDir) {
        Remove-Item -Path $installDir -Recurse -Force
        Write-OK "Dizin silindi"
    }
    
    $currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    $newPath = ($currentPath -split ';' | Where-Object { $_ -notlike "*sentient*" }) -join ';'
    [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
    Write-OK "PATH'ten kaldırıldı"
    
    Write-Host ""
    Write-OK "SENTIENT OS başarıyla kaldırıldı"
}

# ═══════════════════════════════════════════════════════════════════════════════
#  ANA AKIŞ
# ═══════════════════════════════════════════════════════════════════════════════

# Kaldırma modu
if ($Uninstall) {
    Uninstall-Sentient
    exit 0
}

# Kurulum adımları - sırayla çalıştır
try {
    Write-Host ""
    Write-Host "${DIM}  ─────────────────────────────────────────────────────────────────────${RESET}"
    Write-Host "${WHITE}  ADIM 1/11: Hoş Geldiniz${RESET}"
    Write-Host "${DIM}  ─────────────────────────────────────────────────────────────────────${RESET}"
    Show-Welcome | Out-Null
    
    Write-Host ""
    Write-Host "${DIM}  ─────────────────────────────────────────────────────────────────────${RESET}"
    Write-Host "${WHITE}  ADIM 2/11: Lisans${RESET}"
    Write-Host "${DIM}  ─────────────────────────────────────────────────────────────────────${RESET}"
    Show-License | Out-Null
    
    Write-Host ""
    Write-Host "${DIM}  ─────────────────────────────────────────────────────────────────────${RESET}"
    Write-Host "${WHITE}  ADIM 3/11: Sistem Analizi${RESET}"
    Write-Host "${DIM}  ─────────────────────────────────────────────────────────────────────${RESET}"
    Analyze-System | Out-Null
    
    Write-Host ""
    Write-Host "${DIM}  ─────────────────────────────────────────────────────────────────────${RESET}"
    Write-Host "${WHITE}  ADIM 4/11: Kurulum Modu${RESET}"
    Write-Host "${DIM}  ─────────────────────────────────────────────────────────────────────${RESET}"
    Select-Mode | Out-Null
    
    Write-Host ""
    Write-Host "${DIM}  ─────────────────────────────────────────────────────────────────────${RESET}"
    Write-Host "${WHITE}  ADIM 5/11: Provider${RESET}"
    Write-Host "${DIM}  ─────────────────────────────────────────────────────────────────────${RESET}"
    Select-Provider | Out-Null
    
    Write-Host ""
    Write-Host "${DIM}  ─────────────────────────────────────────────────────────────────────${RESET}"
    Write-Host "${WHITE}  ADIM 6/11: Model${RESET}"
    Write-Host "${DIM}  ─────────────────────────────────────────────────────────────────────${RESET}"
    Select-Model | Out-Null
    
    Write-Host ""
    Write-Host "${DIM}  ─────────────────────────────────────────────────────────────────────${RESET}"
    Write-Host "${WHITE}  ADIM 7/11: Bileşenler${RESET}"
    Write-Host "${DIM}  ─────────────────────────────────────────────────────────────────────${RESET}"
    Select-Components | Out-Null
    
    Write-Host ""
    Write-Host "${DIM}  ─────────────────────────────────────────────────────────────────────${RESET}"
    Write-Host "${WHITE}  ADIM 8/11: Ön Koşullar${RESET}"
    Write-Host "${DIM}  ─────────────────────────────────────────────────────────────────────${RESET}"
    Install-Prerequisites | Out-Null
    
    Write-Host ""
    Write-Host "${DIM}  ─────────────────────────────────────────────────────────────────────${RESET}"
    Write-Host "${WHITE}  ADIM 9/11: Kaynak${RESET}"
    Write-Host "${DIM}  ─────────────────────────────────────────────────────────────────────${RESET}"
    Download-Source | Out-Null
    
    Write-Host ""
    Write-Host "${DIM}  ─────────────────────────────────────────────────────────────────────${RESET}"
    Write-Host "${WHITE}  ADIM 10/11: Derleme${RESET}"
    Write-Host "${DIM}  ─────────────────────────────────────────────────────────────────────${RESET}"
    Build-Project | Out-Null
    
    Write-Host ""
    Write-Host "${DIM}  ─────────────────────────────────────────────────────────────────────${RESET}"
    Write-Host "${WHITE}  ADIM 11/11: Yapılandırma${RESET}"
    Write-Host "${DIM}  ─────────────────────────────────────────────────────────────────────${RESET}"
    Configure-Environment | Out-Null
    
    Show-Complete
    
} catch {
    Write-Err "Hata: $_"
    exit 1
}
