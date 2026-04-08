//! ═══════════════════════════════════════════════════════════════════════════════
//!  MANUS SOVEREIGN - L1 Sandbox Politikaları
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Kod çalıştırma için güvenlik kuralları.
//! Ana sisteme ASLA erişim yok!

use crate::error::{ManusError, ManusResult, detect_malicious_code};
use std::collections::HashMap;

/// ─── SOVEREIGN SANDBOX ───

#[derive(Debug, Clone)]
pub struct SovereignSandbox {
    /// Sandbox politikası
    policy: SandboxPolicy,
    /// Aktif mi?
    active: bool,
    /// İhlal geçmişi
    violations: Vec<SandboxViolation>,
}

/// Sandbox politikası
#[derive(Debug, Clone)]
pub struct SandboxPolicy {
    /// Kaynak limitleri
    pub resources: ResourceLimits,
    /// Ağ erişimi
    pub network_enabled: bool,
    /// İzin verilen çıkış portları
    pub allowed_ports: Vec<u16>,
    /// İzin verilen domainler (boş = hiçbiri)
    pub allowed_domains: Vec<String>,
    /// Maksimum dosya boyutu (bytes)
    pub max_file_size: u64,
    /// Maksimum süreç sayısı
    pub max_processes: u32,
    /// Maksimum çalışma süresi (saniye)
    pub max_timeout_secs: u64,
    /// Ortam değişkenleri
    pub env_vars: HashMap<String, String>,
    /// Read-only dosya sistemi
    pub read_only_fs: bool,
    /// İzin verilen sistem çağrıları (seccomp)
    pub seccomp_profile: SeccompProfile,
}

/// Kaynak limitleri
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// Maksimum bellek (MB)
    pub memory_mb: u32,
    /// CPU kotası (1.0 = 1 CPU)
    pub cpu_quota: f32,
    /// Disk I/O limiti (MB/s)
    pub disk_io_limit: u32,
    /// PIDs limiti
    pub pids_limit: u64,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            memory_mb: 256,
            cpu_quota: 0.5,
            disk_io_limit: 10,
            pids_limit: 64,
        }
    }
}

/// Seccomp profil türü
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeccompProfile {
    /// En kısıtlı - sadece temel çağrılar
    Strict,
    /// Orta - çoğu uygulama için
    Moderate,
    /// Gevşek - geliştirme için
    Relaxed,
}

impl SandboxPolicy {
    /// L1 Sovereign politikası - En katı
    pub fn sovereign() -> Self {
        Self {
            resources: ResourceLimits {
                memory_mb: 128,
                cpu_quota: 0.25,
                disk_io_limit: 5,
                pids_limit: 32,
            },
            network_enabled: false,
            allowed_ports: vec![],
            allowed_domains: vec![],
            max_file_size: 10 * 1024 * 1024, // 10 MB
            max_processes: 16,
            max_timeout_secs: 60,
            env_vars: HashMap::new(),
            read_only_fs: true,
            seccomp_profile: SeccompProfile::Strict,
        }
    }
    
    /// Standart politikası
    pub fn standard() -> Self {
        Self {
            resources: ResourceLimits {
                memory_mb: 512,
                cpu_quota: 1.0,
                disk_io_limit: 50,
                pids_limit: 128,
            },
            network_enabled: false,
            allowed_ports: vec![],
            allowed_domains: vec![],
            max_file_size: 100 * 1024 * 1024, // 100 MB
            max_processes: 64,
            max_timeout_secs: 180,
            env_vars: HashMap::new(),
            read_only_fs: false,
            seccomp_profile: SeccompProfile::Moderate,
        }
    }
    
    /// Geliştirme politikası - Daha esnek
    pub fn development() -> Self {
        let mut env = HashMap::new();
        env.insert("RUST_LOG".into(), "debug".into());
        
        Self {
            resources: ResourceLimits {
                memory_mb: 2048,
                cpu_quota: 2.0,
                disk_io_limit: 100,
                pids_limit: 256,
            },
            network_enabled: true,
            allowed_ports: vec![80, 443],
            allowed_domains: vec!["github.com".into(), "pypi.org".into()],
            max_file_size: 1024 * 1024 * 1024, // 1 GB
            max_processes: 256,
            max_timeout_secs: 300,
            env_vars: env,
            read_only_fs: false,
            seccomp_profile: SeccompProfile::Relaxed,
        }
    }
}

/// Sandbox ihlali kaydı
#[derive(Debug, Clone)]
pub struct SandboxViolation {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub violation_type: ViolationType,
    pub resource: String,
    pub blocked: bool,
}

/// İhlal türleri
#[derive(Debug, Clone)]
pub enum ViolationType {
    MemoryLimitExceeded,
    CpuLimitExceeded,
    TimeoutExceeded,
    NetworkAccessBlocked,
    FileSystemAccessBlocked,
    MaliciousCodeDetected,
    ForbiddenSyscall,
    ProcessLimitExceeded,
}

impl SovereignSandbox {
    /// Yeni sandbox oluştur
    pub fn new(policy: SandboxPolicy) -> Self {
        log::info!("🔒  MANUS-SOVEREIGN: Sandbox oluşturuluyor...");
        log::info!("🔒  MANUS-SOVEREIGN: Bellek limiti: {} MB", policy.resources.memory_mb);
        log::info!("🔒  MANUS-SOVEREIGN: CPU kotası: {:.2}", policy.resources.cpu_quota);
        log::info!("🔒  MANUS-SOVEREIGN: Ağ erişimi: {}", if policy.network_enabled { "AÇIK" } else { "KAPALI" });
        
        Self {
            policy,
            active: false,
            violations: Vec::new(),
        }
    }
    
    /// Sovereign sandbox oluştur
    pub fn sovereign() -> Self {
        Self::new(SandboxPolicy::sovereign())
    }
    
    /// Sandbox'ı aktifleştir
    pub fn activate(&mut self) -> ManusResult<()> {
        self.active = true;
        log::info!("🔒  MANUS-SOVEREIGN: Aktif - Kaynak limitleri zorunlu");
        Ok(())
    }
    
    /// Sandbox'ı deaktif et
    pub fn deactivate(&mut self) -> ManusResult<()> {
        self.active = false;
        log::info!("🔒  MANUS-SOVEREIGN: Deaktif");
        Ok(())
    }
    
    /// Aktif mi?
    pub fn is_active(&self) -> bool {
        self.active
    }
    
    /// Kod doğrula
    pub fn validate_code(&self, code: &str) -> ManusResult<()> {
        log::debug!("🔒  MANUS-SOVEREIGN: Kod doğrulanıyor...");
        
        // Zararlı kod kontrolü
        if let Some(reason) = detect_malicious_code(code) {
            log::warn!("🔒  MANUS-SOVEREIGN: Güvenlik ihlali engellendi → {:?}", ViolationType::MaliciousCodeDetected);
            return Err(ManusError::MaliciousCodeDetected(reason));
        }
        
        // Uzunluk kontrolü
        if code.len() > 1024 * 1024 {
            log::warn!("🔒  MANUS-SOVEREIGN: Güvenlik ihlali engellendi → {:?}", ViolationType::FileSystemAccessBlocked);
            return Err(ManusError::InvalidCode("Kod boyutu 1 MB'ı aşamaz".into()));
        }
        
        log::debug!("🔒  MANUS-SOVEREIGN: Kod doğrulandı");
        Ok(())
    }
    
    /// Ağ erişimi kontrolü
    pub fn check_network_access(&self, host: &str) -> ManusResult<()> {
        if !self.policy.network_enabled {
            log::warn!("🔒  MANUS-SOVEREIGN: Ağ erişimi engellendi → {}", host);
            return Err(ManusError::NetworkBlocked(host.into()));
        }
        
        // Domain kontrolü
        if !self.policy.allowed_domains.is_empty() {
            let allowed = self.policy.allowed_domains.iter()
                .any(|d| host.ends_with(d));
            
            if !allowed {
                log::warn!("🔒  MANUS-SOVEREIGN: Domain engellendi → {}", host);
                return Err(ManusError::NetworkBlocked(format!(
                    "Domain '{}' izin verilen listede değil",
                    host
                )));
            }
        }
        
        Ok(())
    }
    
    /// Bellek limiti kontrolü
    pub fn check_memory_limit(&self, requested_mb: u32) -> ManusResult<()> {
        if requested_mb > self.policy.resources.memory_mb {
            log::warn!("🔒  MANUS-SOVEREIGN: Bellek limiti aşıldı → {} MB > {} MB", requested_mb, self.policy.resources.memory_mb);
            return Err(ManusError::MemoryLimitExceeded(self.policy.resources.memory_mb));
        }
        Ok(())
    }
    
    /// Timeout kontrolü
    pub fn check_timeout(&self, timeout_secs: u64) -> ManusResult<()> {
        if timeout_secs > self.policy.max_timeout_secs {
            log::warn!("🔒  MANUS-SOVEREIGN: Timeout aşıldı → {} sn > {} sn", timeout_secs, self.policy.max_timeout_secs);
            return Err(ManusError::Timeout(self.policy.max_timeout_secs));
        }
        Ok(())
    }
    
    /// Politikayı al
    pub fn policy(&self) -> &SandboxPolicy {
        &self.policy
    }
    
    /// İhlal kaydı
    fn record_violation(&mut self, vtype: ViolationType, resource: &str) {
        self.violations.push(SandboxViolation {
            timestamp: chrono::Utc::now(),
            violation_type: vtype,
            resource: resource.into(),
            blocked: true,
        });
    }
    
    /// İhlal sayısı
    pub fn violation_count(&self) -> usize {
        self.violations.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereign_policy() {
        let policy = SandboxPolicy::sovereign();
        assert!(!policy.network_enabled);
        assert!(policy.read_only_fs);
        assert_eq!(policy.resources.memory_mb, 128);
    }

    #[test]
    fn test_standard_policy() {
        let policy = SandboxPolicy::standard();
        assert!(!policy.network_enabled);
        assert_eq!(policy.resources.memory_mb, 512);
    }

    #[test]
    fn test_development_policy() {
        let policy = SandboxPolicy::development();
        assert!(policy.network_enabled);
        assert!(policy.allowed_domains.contains(&"github.com".into()));
    }

    #[test]
    fn test_validate_safe_code() {
        let sandbox = SovereignSandbox::sovereign();
        let code = "print('Hello, World!')";
        assert!(sandbox.validate_code(code).is_ok());
    }

    #[test]
    fn test_validate_malicious_code() {
        let sandbox = SovereignSandbox::sovereign();
        let code = "import os; os.system('rm -rf /')";
        assert!(sandbox.validate_code(code).is_err());
    }

    #[test]
    fn test_network_blocked() {
        let sandbox = SovereignSandbox::sovereign();
        assert!(sandbox.check_network_access("example.com").is_err());
    }

    #[test]
    fn test_memory_limit() {
        let sandbox = SovereignSandbox::sovereign();
        assert!(sandbox.check_memory_limit(256).is_err());
        assert!(sandbox.check_memory_limit(64).is_ok());
    }

    #[test]
    fn test_timeout_limit() {
        let sandbox = SovereignSandbox::sovereign();
        assert!(sandbox.check_timeout(120).is_err());
        assert!(sandbox.check_timeout(30).is_ok());
    }
}
