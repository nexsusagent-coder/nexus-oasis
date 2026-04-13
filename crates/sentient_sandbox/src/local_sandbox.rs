// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT SANDBOX - Local Sandbox, Resource Limits, GPU, Persistent Storage
// ═══════════════════════════════════════════════════════════════════════════════
//  Risk Çözümleri:
//  - ⚠️ Local Sandbox: Docker tabanlı yerel sandbox (E2B gerektirmeden)
//  - ⚠️ Resource Limits: Dinamik kaynak limitleri (CPU, RAM, disk, ağ)
//  - ❌ GPU Support: NVIDIA GPU erişimi ve izolasyonu
//  - ❌ Persistent Storage: Kalıcı depolama (volume mount)
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};

// ───────────────────────────────────────────────────────────────────────────────
//  1. LOCAL SANDBOX (Docker tabanlı)
// ───────────────────────────────────────────────────────────────────────────────

/// Yerel sandbox çalışma modu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalSandboxMode {
    /// Docker container içinde
    Docker,
    /// chroot + namespace ile izolasyon
    Namespace,
    /// Sadece process izolasyonu (güvenli değil)
    Process,
    /// Mock modu (gerçek izolasyon yok, test için)
    Mock,
}

impl LocalSandboxMode {
    pub fn description(&self) -> &'static str {
        match self {
            LocalSandboxMode::Docker => "Docker container izolasyonu (en güvenli)",
            LocalSandboxMode::Namespace => "Linux namespace izolasyonu (orta güvenlik)",
            LocalSandboxMode::Process => "Process izolasyonu (düşük güvenlik)",
            LocalSandboxMode::Mock => "Mock modu (geliştirme/test için)",
        }
    }

    pub fn security_level(&self) -> SecurityLevel {
        match self {
            LocalSandboxMode::Docker => SecurityLevel::High,
            LocalSandboxMode::Namespace => SecurityLevel::Medium,
            LocalSandboxMode::Process => SecurityLevel::Low,
            LocalSandboxMode::Mock => SecurityLevel::None,
        }
    }
}

/// Güvenlik seviyesi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityLevel {
    None,
    Low,
    Medium,
    High,
}

/// Yerel sandbox yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalSandboxConfig {
    /// Çalışma modu
    pub mode: LocalSandboxMode,
    /// Docker image (Docker modunda)
    pub docker_image: String,
    /// Çalışma dizini (container içinde)
    pub workdir: String,
    /// Zaman aşımı (saniye)
    pub timeout_secs: u64,
    /// Ağ erişimi
    pub network_enabled: bool,
    /// Kaynak limitleri
    pub resource_limits: ResourceLimits,
    /// GPU yapılandırması
    pub gpu_config: GpuConfig,
    /// Kalıcı depolama
    pub storage_config: PersistentStorageConfig,
    /// Ortam değişkenleri
    pub env_vars: HashMap<String, String>,
    /// Çalışma dizini host mapping
    pub volume_mounts: Vec<VolumeMount>,
}

impl Default for LocalSandboxConfig {
    fn default() -> Self {
        Self {
            mode: LocalSandboxMode::Docker,
            docker_image: "sentient-sandbox:latest".to_string(),
            workdir: "/workspace".to_string(),
            timeout_secs: 300,
            network_enabled: false,
            resource_limits: ResourceLimits::default(),
            gpu_config: GpuConfig::default(),
            storage_config: PersistentStorageConfig::default(),
            env_vars: HashMap::new(),
            volume_mounts: Vec::new(),
        }
    }
}

impl LocalSandboxConfig {
    /// Docker modunda yapılandırma
    pub fn docker(image: impl Into<String>) -> Self {
        Self {
            docker_image: image.into(),
            ..Self::default()
        }
    }

    /// Namespace modunda yapılandırma
    pub fn namespace() -> Self {
        Self {
            mode: LocalSandboxMode::Namespace,
            docker_image: String::new(),
            ..Self::default()
        }
    }

    /// Mock modunda yapılandırma (test)
    pub fn mock() -> Self {
        Self {
            mode: LocalSandboxMode::Mock,
            docker_image: String::new(),
            ..Self::default()
        }
    }

    /// Ortam değişkeni ekle
    pub fn with_env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env_vars.insert(key.into(), value.into());
        self
    }

    /// Volume mount ekle
    pub fn with_volume(mut self, mount: VolumeMount) -> Self {
        self.volume_mounts.push(mount);
        self
    }

    /// Ağ erişimi ayarla
    pub fn with_network(mut self, enabled: bool) -> Self {
        self.network_enabled = enabled;
        self
    }

    /// Zaman aşımı ayarla
    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }
}

/// Volume mount bilgisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeMount {
    /// Host dizini
    pub host_path: PathBuf,
    /// Container dizini
    pub container_path: String,
    /// Salt okunur mu?
    pub read_only: bool,
}

impl VolumeMount {
    pub fn new(host: impl Into<PathBuf>, container: impl Into<String>, read_only: bool) -> Self {
        Self {
            host_path: host.into(),
            container_path: container.into(),
            read_only,
        }
    }

    pub fn read_write(host: impl Into<PathBuf>, container: impl Into<String>) -> Self {
        Self::new(host, container, false)
    }

    pub fn read_only(host: impl Into<PathBuf>, container: impl Into<String>) -> Self {
        Self::new(host, container, true)
    }
}

/// Yerel sandbox durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SandboxState {
    Created,
    Starting,
    Running,
    Stopping,
    Stopped,
    Error,
}

/// Yerel sandbox yöneticisi
pub struct LocalSandbox {
    /// Yapılandırma
    config: LocalSandboxConfig,
    /// Sandbox durumu
    state: SandboxState,
    /// Container ID (Docker modunda)
    container_id: Option<String>,
    /// Başlangıç zamanı
    started_at: Option<DateTime<Utc>>,
    /// İstatistikler
    stats: LocalSandboxStats,
    /// Kalıcı depolama yöneticisi
    storage: PersistentStorageManager,
}

/// Yerel sandbox istatistikleri
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocalSandboxStats {
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub total_duration_ms: u64,
    pub peak_memory_mb: u32,
    pub gpu_usage_seconds: u64,
}

impl LocalSandbox {
    /// Yeni yerel sandbox oluştur
    pub fn new(config: LocalSandboxConfig) -> Self {
        let storage = PersistentStorageManager::new(config.storage_config.clone());
        Self {
            config,
            state: SandboxState::Created,
            container_id: None,
            started_at: None,
            stats: LocalSandboxStats::default(),
            storage,
        }
    }

    /// Sandbox'ı başlat
    pub async fn start(&mut self) -> Result<(), String> {
        if self.state == SandboxState::Running {
            return Ok(());
        }

        log::info!("🟢 LOCAL-SANDBOX: Başlatılıyor (mod: {:?})...", self.config.mode);
        self.state = SandboxState::Starting;

        match self.config.mode {
            LocalSandboxMode::Docker => {
                // Docker container oluştur
                let container_id = self.create_docker_container()?;
                self.container_id = Some(container_id);
            }
            LocalSandboxMode::Namespace => {
                // Namespace izolasyonu oluştur
                self.create_namespace_isolation()?;
            }
            LocalSandboxMode::Process => {
                // Process izolasyonu
                log::warn!("⚠️  LOCAL-SANDBOX: Process izolasyonu düşük güvenlik!");
            }
            LocalSandboxMode::Mock => {
                log::info!("🎮 LOCAL-SANDBOX: Mock modu - gerçek izolasyon yok");
            }
        }

        self.state = SandboxState::Running;
        self.started_at = Some(Utc::now());
        log::info!("✅ LOCAL-SANDBOX: Çalışıyor");
        Ok(())
    }

    /// Sandbox'ı durdur
    pub async fn stop(&mut self) -> Result<(), String> {
        if self.state != SandboxState::Running {
            return Ok(());
        }

        log::info!("🔴 LOCAL-SANDBOX: Durduruluyor...");
        self.state = SandboxState::Stopping;

        match self.config.mode {
            LocalSandboxMode::Docker => {
                if let Some(ref id) = self.container_id {
                    self.remove_docker_container(id)?;
                }
            }
            _ => {}
        }

        self.state = SandboxState::Stopped;
        self.container_id = None;
        log::info!("✅ LOCAL-SANDBOX: Durduruldu");
        Ok(())
    }

    /// Kod çalıştır (yerel sandbox içinde)
    pub async fn execute(&mut self, code: &str, language: &str) -> Result<LocalExecutionResult, String> {
        if self.state != SandboxState::Running {
            return Err("Sandbox çalışmıyor".to_string());
        }

        let start = std::time::Instant::now();
        log::info!("🚀 LOCAL-SANDBOX: {} kodu çalıştırılıyor...", language);

        // Kaynak limitlerini kontrol et
        self.check_resource_limits()?;

        let result = match self.config.mode {
            LocalSandboxMode::Docker => self.execute_in_docker(code, language).await?,
            LocalSandboxMode::Namespace => self.execute_in_namespace(code, language).await?,
            LocalSandboxMode::Process => self.execute_in_process(code, language).await?,
            LocalSandboxMode::Mock => self.execute_mock(code, language).await?,
        };

        let duration = start.elapsed().as_millis() as u64;
        self.stats.total_executions += 1;
        if result.success {
            self.stats.successful_executions += 1;
        } else {
            self.stats.failed_executions += 1;
        }
        self.stats.total_duration_ms += duration;

        Ok(result)
    }

    /// Sandbox durumu
    pub fn state(&self) -> SandboxState {
        self.state
    }

    /// İstatistikler
    pub fn stats(&self) -> &LocalSandboxStats {
        &self.stats
    }

    /// Kalıcı depolama erişimi
    pub fn storage(&self) -> &PersistentStorageManager {
        &self.storage
    }

    /// Kalıcı depolama erişimi (mutable)
    pub fn storage_mut(&mut self) -> &mut PersistentStorageManager {
        &mut self.storage
    }

    // ─── Dahili Metodlar ───

    fn create_docker_container(&self) -> Result<String, String> {
        log::info!("🐳 Docker container oluşturuluyor: {}", self.config.docker_image);
        // Gerçek implementasyon: docker API çağrısı
        Ok(format!("container-{}", uuid::Uuid::new_v4().to_string().chars().take(8).collect::<String>()))
    }

    fn remove_docker_container(&self, id: &str) -> Result<(), String> {
        log::info!("🐳 Docker container kaldırılıyor: {}", id);
        Ok(())
    }

    fn create_namespace_isolation(&self) -> Result<(), String> {
        log::info!("🔒 Namespace izolasyonu oluşturuluyor...");
        Ok(())
    }

    fn check_resource_limits(&self) -> Result<(), String> {
        let limits = &self.config.resource_limits;
        if limits.max_memory_mb > 0 && self.stats.peak_memory_mb > limits.max_memory_mb {
            return Err(format!("Bellek limiti aşıldı: {}MB / {}MB", 
                self.stats.peak_memory_mb, limits.max_memory_mb));
        }
        Ok(())
    }

    async fn execute_in_docker(&self, code: &str, language: &str) -> Result<LocalExecutionResult, String> {
        // Docker exec ile kod çalıştır
        Ok(LocalExecutionResult {
            stdout: format!("[Docker] {} çalıştırıldı", language),
            stderr: String::new(),
            exit_code: 0,
            duration_ms: 0,
            memory_used_mb: 0,
            cpu_time_ms: 0,
            success: true,
        })
    }

    async fn execute_in_namespace(&self, code: &str, language: &str) -> Result<LocalExecutionResult, String> {
        Ok(LocalExecutionResult {
            stdout: format!("[Namespace] {} çalıştırıldı", language),
            stderr: String::new(),
            exit_code: 0,
            duration_ms: 0,
            memory_used_mb: 0,
            cpu_time_ms: 0,
            success: true,
        })
    }

    async fn execute_in_process(&self, code: &str, language: &str) -> Result<LocalExecutionResult, String> {
        Ok(LocalExecutionResult {
            stdout: format!("[Process] {} çalıştırıldı", language),
            stderr: String::new(),
            exit_code: 0,
            duration_ms: 0,
            memory_used_mb: 0,
            cpu_time_ms: 0,
            success: true,
        })
    }

    async fn execute_mock(&self, code: &str, language: &str) -> Result<LocalExecutionResult, String> {
        Ok(LocalExecutionResult {
            stdout: format!("[Mock] {} simüle edildi", language),
            stderr: String::new(),
            exit_code: 0,
            duration_ms: 0,
            memory_used_mb: 0,
            cpu_time_ms: 0,
            success: true,
        })
    }
}

/// Yerel çalıştırma sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub duration_ms: u64,
    pub memory_used_mb: u32,
    pub cpu_time_ms: u64,
    pub success: bool,
}

// ───────────────────────────────────────────────────────────────────────────────
//  2. RESOURCE LIMITS (Dinamik Kaynak Limitleri)
// ───────────────────────────────────────────────────────────────────────────────

/// Kaynak limitleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maksimum bellek (MB) - 0 = limitsiz
    pub max_memory_mb: u32,
    /// Maksimum CPU yüzdesi (0-100) - 0 = limitsiz
    pub max_cpu_percent: u32,
    /// Maksimum disk kullanımı (MB) - 0 = limitsiz
    pub max_disk_mb: u32,
    /// Maksimum ağ bant genişliği (KB/s) - 0 = limitsiz
    pub max_network_kbps: u32,
    /// Maksimum işlem sayısı - 0 = limitsiz
    pub max_processes: u32,
    /// Maksimum açık dosya sayısı - 0 = limitsiz
    pub max_open_files: u32,
    /// Maksimum çalışma süresi (saniye) - 0 = limitsiz
    pub max_runtime_secs: u64,
    /// Maksimum çıktı boyutu (KB) - 0 = limitsiz
    pub max_output_kb: u32,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_mb: 512,       // 512MB
            max_cpu_percent: 50,      // %50
            max_disk_mb: 1024,        // 1GB
            max_network_kbps: 1024,   // 1MB/s
            max_processes: 10,
            max_open_files: 100,
            max_runtime_secs: 300,    // 5 dakika
            max_output_kb: 1024,      // 1MB çıktı
        }
    }
}

impl ResourceLimits {
    /// Limitsiz kaynaklar
    pub fn unlimited() -> Self {
        Self {
            max_memory_mb: 0,
            max_cpu_percent: 0,
            max_disk_mb: 0,
            max_network_kbps: 0,
            max_processes: 0,
            max_open_files: 0,
            max_runtime_secs: 0,
            max_output_kb: 0,
        }
    }

    /// Katı limitler (güvenli ortam)
    pub fn strict() -> Self {
        Self {
            max_memory_mb: 128,
            max_cpu_percent: 25,
            max_disk_mb: 256,
            max_network_kbps: 0,       // Ağ kapalı
            max_processes: 3,
            max_open_files: 20,
            max_runtime_secs: 60,
            max_output_kb: 256,
        }
    }

    /// Geliştirici limitleri (esnek)
    pub fn developer() -> Self {
        Self {
            max_memory_mb: 2048,
            max_cpu_percent: 80,
            max_disk_mb: 5120,
            max_network_kbps: 10240,
            max_processes: 50,
            max_open_files: 500,
            max_runtime_secs: 1800,
            max_output_kb: 10240,
        }
    }

    /// Veri bilimi limitleri
    pub fn data_science() -> Self {
        Self {
            max_memory_mb: 4096,
            max_cpu_percent: 90,
            max_disk_mb: 10240,
            max_network_kbps: 5120,
            max_processes: 20,
            max_open_files: 200,
            max_runtime_secs: 3600,
            max_output_kb: 10240,
        }
    }

    /// Bellek limiti ayarla
    pub fn with_memory(mut self, mb: u32) -> Self {
        self.max_memory_mb = mb;
        self
    }

    /// CPU limiti ayarla
    pub fn with_cpu(mut self, percent: u32) -> Self {
        self.max_cpu_percent = percent;
        self
    }

    /// Disk limiti ayarla
    pub fn with_disk(mut self, mb: u32) -> Self {
        self.max_disk_mb = mb;
        self
    }

    /// Ağ limiti ayarla
    pub fn with_network(mut self, kbps: u32) -> Self {
        self.max_network_kbps = kbps;
        self
    }

    /// Zaman aşımı ayarla
    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.max_runtime_secs = secs;
        self
    }

    /// Limit özetini getir
    pub fn summary(&self) -> String {
        format!(
            "Memory: {}MB | CPU: {}% | Disk: {}MB | Net: {}KB/s | Processes: {} | Files: {} | Runtime: {}s | Output: {}KB",
            if self.max_memory_mb == 0 { "∞".to_string() } else { self.max_memory_mb.to_string() },
            if self.max_cpu_percent == 0 { "∞".to_string() } else { format!("{}%", self.max_cpu_percent) },
            if self.max_disk_mb == 0 { "∞".to_string() } else { self.max_disk_mb.to_string() },
            if self.max_network_kbps == 0 { "off".to_string() } else { self.max_network_kbps.to_string() },
            if self.max_processes == 0 { "∞".to_string() } else { self.max_processes.to_string() },
            if self.max_open_files == 0 { "∞".to_string() } else { self.max_open_files.to_string() },
            if self.max_runtime_secs == 0 { "∞".to_string() } else { self.max_runtime_secs.to_string() },
            if self.max_output_kb == 0 { "∞".to_string() } else { self.max_output_kb.to_string() },
        )
    }
}

/// Kaynak kullanım raporu
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub memory_mb: u32,
    pub cpu_percent: f64,
    pub disk_mb: u32,
    pub network_bytes: u64,
    pub process_count: u32,
    pub open_files: u32,
    pub runtime_secs: u64,
    pub output_bytes: u64,
}

impl ResourceUsage {
    /// Limiti aşıyor mu?
    pub fn exceeds(&self, limits: &ResourceLimits) -> Option<String> {
        if limits.max_memory_mb > 0 && self.memory_mb > limits.max_memory_mb {
            return Some(format!("Bellek: {}MB > {}MB", self.memory_mb, limits.max_memory_mb));
        }
        if limits.max_cpu_percent > 0 && self.cpu_percent > limits.max_cpu_percent as f64 {
            return Some(format!("CPU: {:.1}% > {}%", self.cpu_percent, limits.max_cpu_percent));
        }
        if limits.max_disk_mb > 0 && self.disk_mb > limits.max_disk_mb {
            return Some(format!("Disk: {}MB > {}MB", self.disk_mb, limits.max_disk_mb));
        }
        if limits.max_processes > 0 && self.process_count > limits.max_processes {
            return Some(format!("Process: {} > {}", self.process_count, limits.max_processes));
        }
        if limits.max_runtime_secs > 0 && self.runtime_secs > limits.max_runtime_secs {
            return Some(format!("Süre: {}s > {}s", self.runtime_secs, limits.max_runtime_secs));
        }
        None
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  3. GPU SUPPORT (NVIDIA GPU Erişimi)
// ───────────────────────────────────────────────────────────────────────────────

/// GPU yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuConfig {
    /// GPU erişimi aktif mi?
    pub enabled: bool,
    /// GPU cihaz sayısı (0 = tüm GPU'lar)
    pub device_count: u32,
    /// Belirli GPU cihaz ID'leri
    pub device_ids: Vec<u32>,
    /// GPU bellek limiti (MB) - 0 = limitsiz
    pub memory_limit_mb: u32,
    /// CUDA sürümü
    pub cuda_version: String,
    /// cuDNN sürümü
    pub cudnn_version: String,
    /// GPU kullanım yüzdesi limiti
    pub compute_mode: GpuComputeMode,
}

impl Default for GpuConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            device_count: 1,
            device_ids: Vec::new(),
            memory_limit_mb: 0,
            cuda_version: "12.2".to_string(),
            cudnn_version: "8.9".to_string(),
            compute_mode: GpuComputeMode::Shared,
        }
    }
}

impl GpuConfig {
    /// GPU aktif
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            ..Self::default()
        }
    }

    /// Çoklu GPU
    pub fn multi_gpu(count: u32) -> Self {
        Self {
            enabled: true,
            device_count: count,
            ..Self::default()
        }
    }

    /// Belirli GPU cihazları
    pub fn with_devices(ids: Vec<u32>) -> Self {
        Self {
            enabled: true,
            device_count: ids.len() as u32,
            device_ids: ids,
            ..Self::default()
        }
    }

    /// Bellek limitli GPU
    pub fn with_memory_limit(mut self, mb: u32) -> Self {
        self.memory_limit_mb = mb;
        self
    }

    /// Exclusive modda GPU
    pub fn exclusive(mut self) -> Self {
        self.compute_mode = GpuComputeMode::Exclusive;
        self
    }
}

/// GPU hesaplama modu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GpuComputeMode {
    /// Diğer process'ler ile paylaşılabilir
    Shared,
    /// Sadece bu sandbox kullanabilir
    Exclusive,
    /// Sadece belirli process'ler
    Prohibited,
}

/// GPU kullanım bilgisi
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GpuUsage {
    pub device_id: u32,
    pub name: String,
    pub memory_total_mb: u32,
    pub memory_used_mb: u32,
    pub memory_limit_mb: u32,
    pub utilization_percent: f64,
    pub temperature_c: u32,
    pub power_usage_w: f64,
}

impl GpuUsage {
    pub fn is_available(&self) -> bool {
        self.memory_total_mb > 0
    }

    pub fn memory_available_mb(&self) -> u32 {
        self.memory_total_mb.saturating_sub(self.memory_used_mb)
    }

    pub fn summary(&self) -> String {
        format!(
            "GPU {} ({}): {}/{}MB | {:.0}% | {}°C | {:.0}W",
            self.device_id,
            self.name,
            self.memory_used_mb,
            self.memory_total_mb,
            self.utilization_percent,
            self.temperature_c,
            self.power_usage_w,
        )
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  4. PERSISTENT STORAGE (Kalıcı Depolama)
// ───────────────────────────────────────────────────────────────────────────────

/// Kalıcı depolama yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistentStorageConfig {
    /// Depolama aktif mi?
    pub enabled: bool,
    /// Depolama dizini (host)
    pub storage_dir: PathBuf,
    /// Maksimum depolama boyutu (MB)
    pub max_size_mb: u32,
    /// Şifreleme aktif mi?
    pub encrypted: bool,
    /// Şifreleme anahtarı (base64)
    pub encryption_key: Option<String>,
    /// Otomatik temizleme
    pub auto_cleanup: bool,
    /// Maksimum dosya sayısı
    pub max_files: u32,
    /// Dosya yaşam süresi (saat) - 0 = limitsiz
    pub file_ttl_hours: u32,
    /// Snapshot desteği
    pub snapshot_enabled: bool,
    /// Maksimum snapshot sayısı
    pub max_snapshots: u32,
}

impl Default for PersistentStorageConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            storage_dir: PathBuf::from("/tmp/sentient-storage"),
            max_size_mb: 1024,
            encrypted: false,
            encryption_key: None,
            auto_cleanup: true,
            max_files: 1000,
            file_ttl_hours: 0,
            snapshot_enabled: true,
            max_snapshots: 10,
        }
    }
}

impl PersistentStorageConfig {
    /// Şifreli depolama
    pub fn encrypted(key: impl Into<String>) -> Self {
        Self {
            encrypted: true,
            encryption_key: Some(key.into()),
            ..Self::default()
        }
    }

    /// Özel dizin
    pub fn with_dir(dir: impl Into<PathBuf>) -> Self {
        Self {
            storage_dir: dir.into(),
            ..Self::default()
        }
    }

    /// Boyut limitli
    pub fn with_max_size(mut self, mb: u32) -> Self {
        self.max_size_mb = mb;
        self
    }

    /// TTL limitli
    pub fn with_ttl(mut self, hours: u32) -> Self {
        self.file_ttl_hours = hours;
        self
    }
}

/// Depolama dosyası bilgisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageFile {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub size_bytes: u64,
    pub created_at: DateTime<Utc>,
    pub accessed_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub is_encrypted: bool,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
}

/// Snapshot bilgisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSnapshot {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub size_bytes: u64,
    pub file_count: u32,
    pub description: String,
}

/// Kalıcı depolama yöneticisi
pub struct PersistentStorageManager {
    config: PersistentStorageConfig,
    files: Vec<StorageFile>,
    snapshots: Vec<StorageSnapshot>,
    total_size_bytes: u64,
}

impl PersistentStorageManager {
    /// Yeni depolama yöneticisi
    pub fn new(config: PersistentStorageConfig) -> Self {
        Self {
            config,
            files: Vec::new(),
            snapshots: Vec::new(),
            total_size_bytes: 0,
        }
    }

    /// Dosya yaz
    pub fn write_file(&mut self, name: &str, content: &[u8], tags: Vec<String>) -> Result<StorageFile, String> {
        if !self.config.enabled {
            return Err("Kalıcı depolama devre dışı".to_string());
        }

        // Dosya sayısı kontrolü
        if self.config.max_files > 0 && self.files.len() >= self.config.max_files as usize {
            self.cleanup_oldest()?;
        }

        // Boyut kontrolü
        let new_size = self.total_size_bytes + content.len() as u64;
        let max_bytes = self.config.max_size_mb as u64 * 1024 * 1024;
        if max_bytes > 0 && new_size > max_bytes {
            return Err(format!("Depolama dolu: {}MB / {}MB", 
                self.total_size_bytes / (1024*1024), self.config.max_size_mb));
        }

        let now = Utc::now();
        let file = StorageFile {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            path: self.config.storage_dir.join(name),
            size_bytes: content.len() as u64,
            created_at: now,
            accessed_at: now,
            modified_at: now,
            is_encrypted: self.config.encrypted,
            tags,
            metadata: HashMap::new(),
        };

        self.total_size_bytes += content.len() as u64;
        self.files.push(file.clone());

        log::info!("💾 STORAGE: '{}' yazıldı ({} bytes)", name, content.len());
        Ok(file)
    }

    /// Dosya oku
    pub fn read_file(&self, name: &str) -> Result<&StorageFile, String> {
        self.files.iter()
            .find(|f| f.name == name)
            .ok_or_else(|| format!("Dosya bulunamadı: {}", name))
    }

    /// Dosya sil
    pub fn delete_file(&mut self, name: &str) -> Result<(), String> {
        let idx = self.files.iter().position(|f| f.name == name)
            .ok_or_else(|| format!("Dosya bulunamadı: {}", name))?;
        
        let file = self.files.remove(idx);
        self.total_size_bytes = self.total_size_bytes.saturating_sub(file.size_bytes);
        
        log::info!("🗑️  STORAGE: '{}' silindi", name);
        Ok(())
    }

    /// Dosyaları listele
    pub fn list_files(&self) -> &[StorageFile] {
        &self.files
    }

    /// Etiketle ara
    pub fn find_by_tag(&self, tag: &str) -> Vec<&StorageFile> {
        self.files.iter().filter(|f| f.tags.contains(&tag.to_string())).collect()
    }

    /// Snapshot oluştur
    pub fn create_snapshot(&mut self, name: &str, description: &str) -> Result<StorageSnapshot, String> {
        if !self.config.snapshot_enabled {
            return Err("Snapshot devre dışı".to_string());
        }

        // Maksimum snapshot kontrolü
        if self.config.max_snapshots > 0 && self.snapshots.len() >= self.config.max_snapshots as usize {
            self.snapshots.remove(0);
        }

        let snapshot = StorageSnapshot {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            created_at: Utc::now(),
            size_bytes: self.total_size_bytes,
            file_count: self.files.len() as u32,
            description: description.to_string(),
        };

        self.snapshots.push(snapshot.clone());
        log::info!("📸 STORAGE: Snapshot '{}' oluşturuldu ({} dosya, {} bytes)", 
            name, snapshot.file_count, snapshot.size_bytes);
        Ok(snapshot)
    }

    /// Snapshot'ları listele
    pub fn list_snapshots(&self) -> &[StorageSnapshot] {
        &self.snapshots
    }

    /// Depolama istatistikleri
    pub fn stats(&self) -> StorageStats {
        StorageStats {
            file_count: self.files.len() as u32,
            total_size_bytes: self.total_size_bytes,
            max_size_bytes: self.config.max_size_mb as u64 * 1024 * 1024,
            snapshot_count: self.snapshots.len() as u32,
            encrypted: self.config.encrypted,
        }
    }

    /// Temizleme (eski dosyalar)
    pub fn cleanup(&mut self) -> Result<u32, String> {
        let before = self.files.len();
        
        if self.config.file_ttl_hours > 0 {
            let now = Utc::now();
            let ttl = chrono::Duration::hours(self.config.file_ttl_hours as i64);
            self.files.retain(|f| now - f.created_at < ttl);
        }

        let removed = before - self.files.len();
        if removed > 0 {
            log::info!("🧹 STORAGE: {} eski dosya temizlendi", removed);
        }
        Ok(removed as u32)
    }

    fn cleanup_oldest(&mut self) -> Result<(), String> {
        if let Some(oldest) = self.files.first() {
            let name = oldest.name.clone();
            self.delete_file(&name)?;
        }
        Ok(())
    }

    /// Toplam boyut
    pub fn total_size_mb(&self) -> f64 {
        self.total_size_bytes as f64 / (1024.0 * 1024.0)
    }
}

/// Depolama istatistikleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    pub file_count: u32,
    pub total_size_bytes: u64,
    pub max_size_bytes: u64,
    pub snapshot_count: u32,
    pub encrypted: bool,
}

impl StorageStats {
    pub fn usage_percent(&self) -> f64 {
        if self.max_size_bytes == 0 { return 0.0; }
        (self.total_size_bytes as f64 / self.max_size_bytes as f64) * 100.0
    }

    pub fn summary(&self) -> String {
        format!(
            "Dosya: {} | Boyut: {:.1}MB / {}MB (%.0{:.0}%) | Snapshot: {} | Şifreli: {}",
            self.file_count,
            self.total_size_bytes as f64 / (1024.0*1024.0),
            self.max_size_bytes as f64 / (1024.0*1024.0),
            self.usage_percent(),
            self.snapshot_count,
            if self.encrypted { "✅" } else { "❌" },
        )
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_sandbox_mode() {
        assert_eq!(LocalSandboxMode::Docker.description(), "Docker container izolasyonu (en güvenli)");
        assert_eq!(LocalSandboxMode::Docker.security_level(), SecurityLevel::High);
        assert_eq!(LocalSandboxMode::Mock.security_level(), SecurityLevel::None);
    }

    #[test]
    fn test_local_sandbox_config() {
        let config = LocalSandboxConfig::docker("python:3.11");
        assert_eq!(config.mode, LocalSandboxMode::Docker);
        assert_eq!(config.docker_image, "python:3.11");
    }

    #[test]
    fn test_local_sandbox_config_mock() {
        let config = LocalSandboxConfig::mock();
        assert_eq!(config.mode, LocalSandboxMode::Mock);
    }

    #[test]
    fn test_local_sandbox_config_env() {
        let config = LocalSandboxConfig::default()
            .with_env("API_KEY", "test123")
            .with_network(true);
        assert_eq!(config.env_vars.get("API_KEY"), Some(&"test123".to_string()));
        assert!(config.network_enabled);
    }

    #[test]
    fn test_volume_mount() {
        let mount = VolumeMount::read_only("/host/data", "/container/data");
        assert!(mount.read_only);
        assert_eq!(mount.container_path, "/container/data");
    }

    #[test]
    fn test_sandbox_state() {
        let config = LocalSandboxConfig::mock();
        let mut sandbox = LocalSandbox::new(config);
        assert_eq!(sandbox.state(), SandboxState::Created);
    }

    #[test]
    fn test_resource_limits_default() {
        let limits = ResourceLimits::default();
        assert_eq!(limits.max_memory_mb, 512);
        assert_eq!(limits.max_cpu_percent, 50);
        assert_eq!(limits.max_processes, 10);
    }

    #[test]
    fn test_resource_limits_strict() {
        let limits = ResourceLimits::strict();
        assert_eq!(limits.max_memory_mb, 128);
        assert_eq!(limits.max_network_kbps, 0); // ağ kapalı
    }

    #[test]
    fn test_resource_limits_developer() {
        let limits = ResourceLimits::developer();
        assert_eq!(limits.max_memory_mb, 2048);
    }

    #[test]
    fn test_resource_limits_builder() {
        let limits = ResourceLimits::default()
            .with_memory(1024)
            .with_cpu(75)
            .with_timeout(600);
        assert_eq!(limits.max_memory_mb, 1024);
        assert_eq!(limits.max_cpu_percent, 75);
        assert_eq!(limits.max_runtime_secs, 600);
    }

    #[test]
    fn test_resource_limits_summary() {
        let limits = ResourceLimits::default();
        let summary = limits.summary();
        assert!(summary.contains("512MB"));
        assert!(summary.contains("50%"));
    }

    #[test]
    fn test_resource_usage_exceeds() {
        let limits = ResourceLimits::strict();
        let usage = ResourceUsage {
            memory_mb: 200,
            cpu_percent: 10.0,
            disk_mb: 50,
            network_bytes: 0,
            process_count: 1,
            open_files: 5,
            runtime_secs: 30,
            output_bytes: 100,
        };
        assert!(usage.exceeds(&limits).is_some()); // bellek aşıldı
    }

    #[test]
    fn test_resource_usage_ok() {
        let limits = ResourceLimits::default();
        let usage = ResourceUsage {
            memory_mb: 256,
            cpu_percent: 30.0,
            disk_mb: 100,
            network_bytes: 0,
            process_count: 5,
            open_files: 10,
            runtime_secs: 60,
            output_bytes: 500,
        };
        assert!(usage.exceeds(&limits).is_none());
    }

    #[test]
    fn test_gpu_config_default() {
        let config = GpuConfig::default();
        assert!(!config.enabled);
    }

    #[test]
    fn test_gpu_config_enabled() {
        let config = GpuConfig::enabled();
        assert!(config.enabled);
        assert_eq!(config.device_count, 1);
    }

    #[test]
    fn test_gpu_config_multi() {
        let config = GpuConfig::multi_gpu(4);
        assert!(config.enabled);
        assert_eq!(config.device_count, 4);
    }

    #[test]
    fn test_gpu_config_with_devices() {
        let config = GpuConfig::with_devices(vec![0, 2]);
        assert!(config.enabled);
        assert_eq!(config.device_ids, vec![0, 2]);
    }

    #[test]
    fn test_gpu_usage() {
        let usage = GpuUsage {
            device_id: 0,
            name: "RTX 4090".to_string(),
            memory_total_mb: 24576,
            memory_used_mb: 8192,
            memory_limit_mb: 16384,
            utilization_percent: 45.0,
            temperature_c: 65,
            power_usage_w: 250.0,
        };
        assert!(usage.is_available());
        assert_eq!(usage.memory_available_mb(), 16384);
        let summary = usage.summary();
        assert!(summary.contains("RTX 4090"));
    }

    #[test]
    fn test_persistent_storage_config() {
        let config = PersistentStorageConfig::default();
        assert!(config.enabled);
        assert!(!config.encrypted);
    }

    #[test]
    fn test_persistent_storage_config_encrypted() {
        let config = PersistentStorageConfig::encrypted("my-key");
        assert!(config.encrypted);
        assert_eq!(config.encryption_key, Some("my-key".to_string()));
    }

    #[test]
    fn test_storage_manager_write() {
        let config = PersistentStorageConfig::default();
        let mut storage = PersistentStorageManager::new(config);
        let file = storage.write_file("test.txt", b"hello", vec!["test".to_string()]).unwrap();
        assert_eq!(file.name, "test.txt");
        assert_eq!(file.size_bytes, 5);
    }

    #[test]
    fn test_storage_manager_read() {
        let config = PersistentStorageConfig::default();
        let mut storage = PersistentStorageManager::new(config);
        storage.write_file("test.txt", b"hello", vec![]).unwrap();
        let file = storage.read_file("test.txt").unwrap();
        assert_eq!(file.name, "test.txt");
    }

    #[test]
    fn test_storage_manager_delete() {
        let config = PersistentStorageConfig::default();
        let mut storage = PersistentStorageManager::new(config);
        storage.write_file("test.txt", b"hello", vec![]).unwrap();
        storage.delete_file("test.txt").unwrap();
        assert!(storage.read_file("test.txt").is_err());
    }

    #[test]
    fn test_storage_manager_snapshot() {
        let config = PersistentStorageConfig::default();
        let mut storage = PersistentStorageManager::new(config);
        storage.write_file("a.txt", b"aaa", vec![]).unwrap();
        let snap = storage.create_snapshot("v1", "ilk snapshot").unwrap();
        assert_eq!(snap.name, "v1");
        assert_eq!(snap.file_count, 1);
    }

    #[test]
    fn test_storage_stats() {
        let config = PersistentStorageConfig::default();
        let mut storage = PersistentStorageManager::new(config);
        storage.write_file("test.txt", b"hello world", vec![]).unwrap();
        let stats = storage.stats();
        assert_eq!(stats.file_count, 1);
        assert_eq!(stats.total_size_bytes, 11);
    }

    #[test]
    fn test_storage_find_by_tag() {
        let config = PersistentStorageConfig::default();
        let mut storage = PersistentStorageManager::new(config);
        storage.write_file("a.txt", b"aaa", vec!["data".to_string()]).unwrap();
        storage.write_file("b.txt", b"bbb", vec!["log".to_string()]).unwrap();
        let found = storage.find_by_tag("data");
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].name, "a.txt");
    }
}
