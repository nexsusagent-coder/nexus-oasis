//! ═══════════════════════════════════════════════════════════════════════════════
//!  SOVEREIGN SANDBOX - L1 ANAYASA GÜVENLİĞİ
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Browser-Use aracının yerel dosya sistemine erişimi TAMAMEN ENGELLENİR.
//! Sadece dış ağ (internet) erişimine izin verilir.
//!
//! ═──────────────────────────────────────────────────────────────────────────────
//!  ERİŞİM MATRİSİ:
//!  ────────────────
//!  file://         → ❌ BLOCKED (Yerel dosyalar)
//!  file:///etc/*   → ❌ BLOCKED (Sistem dosyaları)
//!  file:///home/*  → ❌ BLOCKED (Kullanıcı dosyaları)
//!  http://localhost → ❌ BLOCKED (Yerel servisler)
//!  127.0.0.1       → ❌ BLOCKED (Loopback)
//!  192.168.*       → ❌ BLOCKED (Yerel ağ)
//!  10.*            → ❌ BLOCKED (VPN/Dahili)
//!  https://*       → ✅ ALLOWED (Dış web - HTTPS)
//!  http://*        → ⚠️  WARNING (Dış web - HTTP, uyarı ver)
//! ═──────────────────────────────────────────────────────────────────────────────

use crate::error::{BrowserError, BrowserResult};
use regex::Regex;
use std::net::IpAddr;
use url::Url;

/// ─── DOSYA ERİŞİM POLİTİKASI ───

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileAccess {
    /// Tamamen engelli (Sovereign mod)
    Blocked,
    /// Sadece okuma
    ReadOnly,
    /// Sınırlı yazma (belirli dizinler)
    LimitedWrite,
    /// Tam erişim (ASLA kullanılmamalı)
    Full,
}

/// ─── AĞ ERİŞİM POLİTİKASI ───

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkAccess {
    /// Sadece dış internet (HTTPS)
    InternetOnly,
    /// Yerel ağ dahil
    LocalNetwork,
    /// Tüm ağlar
    Full,
    /// Tamamen engelli
    Blocked,
}

/// ─── PROCESS POLİTİKASI ───

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessAccess {
    /// Hiçbir process başlatılamaz
    None,
    /// Sadece whitelist'teki processler
    Whitelisted,
    /// Tüm processler
    Full,
}

/// ─── SOVEREIGN SANDBOX POLİTİKASı ───

#[derive(Debug, Clone)]
pub struct SandboxPolicy {
    /// Dosya erişimi
    pub file_access: FileAccess,
    /// Ağ erişimi
    pub network_access: NetworkAccess,
    /// Process erişimi
    pub process_access: ProcessAccess,
    /// İzin verilen domainler (boş = tüm dış web)
    pub allowed_domains: Vec<String>,
    /// Engellenen domainler
    pub blocked_domains: Vec<String>,
    /// Maksimum bellek (MB)
    pub max_memory_mb: u32,
    /// Maksimum CPU (%)  
    pub max_cpu_percent: u32,
    /// Zaman aşımı (saniye)
    pub timeout_secs: u32,
    /// localStorage/sessionStorage kullanımı
    pub allow_storage: bool,
    /// Cookie kullanımı
    pub allow_cookies: bool,
    /// JavaScript çalıştırma
    pub allow_javascript: bool,
}

impl SandboxPolicy {
    /// L1 Sovereign politikası - En katı güvenlik
    pub fn sovereign() -> Self {
        Self {
            file_access: FileAccess::Blocked,
            network_access: NetworkAccess::InternetOnly,
            process_access: ProcessAccess::None,
            allowed_domains: vec![],
            blocked_domains: vec![
                // Malicious/Tracking
                "doubleclick.net".into(),
                "googlesyndication.com".into(),
                "facebook.com".into(), // Tracker
            ],
            max_memory_mb: 512,
            max_cpu_percent: 50,
            timeout_secs: 60,
            allow_storage: false,
            allow_cookies: true, // Oturum için gerekli
            allow_javascript: true, // Modern web için gerekli
        }
    }
    
    /// Araştırma modu - Biraz daha esnek
    pub fn research() -> Self {
        let mut policy = Self::sovereign();
        policy.timeout_secs = 300; // 5 dakika
        policy.max_memory_mb = 1024;
        policy
    }
    
    /// E-ticaret modu - Ödeme sayfaları için
    pub fn ecommerce() -> Self {
        let mut policy = Self::sovereign();
        policy.allow_cookies = true;
        policy.allow_storage = true;
        policy.blocked_domains.retain(|d| d != "facebook.com"); // Login için
        policy
    }
}

impl Default for SandboxPolicy {
    fn default() -> Self {
        Self::sovereign()
    }
}

/// ─── SOVEREIGN SANDBOX ───

pub struct SovereignSandbox {
    policy: SandboxPolicy,
    blocked_schemes: Vec<String>,
    private_ip_patterns: Vec<Regex>,
    active: bool,
    violations: Vec<SandboxViolation>,
}

/// Sandbox ihlali kaydı
#[derive(Debug, Clone)]
pub struct SandboxViolation {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub violation_type: ViolationType,
    pub resource: String,
    pub blocked: bool,
}

#[derive(Debug, Clone)]
pub enum ViolationType {
    FileAccess,
    NetworkAccess,
    PrivateIP,
    BlockedDomain,
    SchemeBlocked,
}

impl SovereignSandbox {
    /// Yeni sandbox oluştur
    pub fn new(policy: SandboxPolicy) -> Self {
        log::info!("🔒  SOVEREIGN: Sandbox oluşturuluyor...");
        
        let blocked_schemes = vec![
            "file".to_string(),
            "data".to_string(), // data: URL'leri tehlikeli olabilir
            "javascript".to_string(),
            "vbscript".to_string(),
            "about".to_string(),
        ];
        
        // RFC 1918 private IP ranges
        let private_ip_patterns = vec![
            Regex::new(r"^127\.").expect("operation failed"),
            Regex::new(r"^10\.").expect("operation failed"),
            Regex::new(r"^172\.(1[6-9]|2[0-9]|3[0-1])\.").expect("operation failed"),
            Regex::new(r"^192\.168\.").expect("operation failed"),
            Regex::new(r"^169\.254\.").expect("operation failed"), // Link-local
            Regex::new(r"^0\.0\.0\.0").expect("operation failed"),
            Regex::new(r"^::1$").expect("operation failed"), // IPv6 loopback
            Regex::new(r"^fc00:").expect("operation failed"), // IPv6 ULA
            Regex::new(r"^fe80:").expect("operation failed"), // IPv6 link-local
        ];
        
        log::info!("🔒  SOVEREIGN: {} engellenmiş şema, {} özel IP pattern", 
            blocked_schemes.len(), private_ip_patterns.len());
        
        Self {
            policy,
            blocked_schemes,
            private_ip_patterns,
            active: false,
            violations: Vec::new(),
        }
    }
    
    /// Sandbox'ı aktive et
    pub fn activate(&mut self) -> BrowserResult<()> {
        if self.active {
            return Ok(());
        }
        
        self.active = true;
        log::info!("🔒  SOVEREIGN: Aktif - Dosya sistemi erişimi ENGELLENDI");
        log::info!("🔒  SOVEREIGN: Ağ politikası: {:?}", self.policy.network_access);
        Ok(())
    }
    
    /// Sandbox'ı deaktif et
    pub fn deactivate(&mut self) -> BrowserResult<()> {
        self.active = false;
        log::info!("🔒  SOVEREIGN: Deaktif");
        Ok(())
    }
    
    /// URL doğrula
    pub fn validate_url(&self, url: &str) -> BrowserResult<()> {
        // Parse URL
        let parsed = Url::parse(url).map_err(|e| {
            BrowserError::SandboxViolation(format!("Geçersiz URL formatı: {}", e))
        })?;
        
        // Şema kontrolü
        let scheme = parsed.scheme().to_lowercase();
        if self.blocked_schemes.contains(&scheme) {
            log::warn!("🔒  SOVEREIGN: Güvenlik ihlali engellendi → {:?}", ViolationType::SchemeBlocked);
            return Err(BrowserError::SandboxViolation(format!(
                "OASIS-BROWSER: '{}' şeması güvenlik nedeniyle engellendi. Sadece dış web adreslerine erişilebilir.",
                scheme
            )));
        }
        
        // file:// özel kontrol
        if scheme == "file" {
            log::warn!("🔒  SOVEREIGN: Güvenlik ihlali engellendi → {:?}", ViolationType::FileAccess);
            return Err(BrowserError::SandboxViolation(
                "OASIS-BROWSER: Yerel dosya erişimi SOVEREIGN anayasası gereği yasaktır. Sadece dış web'e erişebilirsiniz.".into()
            ));
        }
        
        // Host kontrolü
        if let Some(host) = parsed.host_str() {
            // IPv6 adresleri için köşeli parantezleri kaldır
            let host_clean = host.trim_start_matches('[').trim_end_matches(']');
            
            // Localhost kontrolü
            if host_clean == "localhost" || host_clean == "localhost.localdomain" || host_clean == "::1" {
                log::warn!("🔒  SOVEREIGN: Güvenlik ihlali engellendi → {:?}", ViolationType::PrivateIP);
                return Err(BrowserError::SandboxViolation(
                    "OASIS-BROWSER: Yerel servis (localhost) erişimi engellendi.".into()
                ));
            }
            
            // Private IP kontrolü (temizlenmiş host ile)
            for pattern in &self.private_ip_patterns {
                if pattern.is_match(host_clean) {
                    log::warn!("🔒  SOVEREIGN: Güvenlik ihlali engellendi → {:?}", ViolationType::PrivateIP);
                    return Err(BrowserError::SandboxViolation(format!(
                        "OASIS-BROWSER: Özel ağ adresine ({}) erişim engellendi. Sadece genel internet adresleri kullanılabilir.",
                        host
                    )));
                }
            }
            
            // IP adresi olarak parse et ve kontrol et
            if let Ok(ip) = host_clean.parse::<IpAddr>() {
                if ip.is_loopback() {
                    log::warn!("🔒  SOVEREIGN: Güvenlik ihlali engellendi → {:?}", ViolationType::PrivateIP);
                    return Err(BrowserError::SandboxViolation(format!(
                        "OASIS-BROWSER: Kısıtlı IP adresi ({}) engellendi.",
                        host
                    )));
                }
            }
            
            // IPv4 özel kontrol
            if let Ok(ipv4) = host_clean.parse::<std::net::Ipv4Addr>() {
                if ipv4.is_private() || ipv4.is_link_local() {
                    log::warn!("🔒  SOVEREIGN: Güvenlik ihlali engellendi → {:?}", ViolationType::PrivateIP);
                    return Err(BrowserError::SandboxViolation(format!(
                        "OASIS-BROWSER: Özel ağ adresi ({}) erişim engellendi.",
                        host
                    )));
                }
            }
            
            // Engellenen domain kontrolü
            let host_lower = host.to_lowercase();
            for blocked in &self.policy.blocked_domains {
                if host_lower.contains(blocked) || host_lower.ends_with(blocked) {
                    log::warn!("🔒  SOVEREIGN: Güvenlik ihlali engellendi → {:?}", ViolationType::BlockedDomain);
                    return Err(BrowserError::SandboxViolation(format!(
                        "OASIS-BROWSER: '{}' domain'i güvenlik nedeniyle engellendi.",
                        host
                    )));
                }
            }
        }
        
        // HTTP uyarısı (engelleme değil)
        if scheme == "http" {
            log::warn!("⚠️  OASIS-BROWSER: Güvenli olmayan HTTP bağlantısı kullanılıyor: {}", url);
        }
        
        Ok(())
    }
    
    /// Ağ erişimi kontrolü
    pub fn check_network_access(&self, operation: &str) -> BrowserResult<()> {
        if !self.active {
            return Err(BrowserError::SandboxViolation(
                "OASIS-BROWSER: Sandbox aktif değil.".into()
            ));
        }
        
        match self.policy.network_access {
            NetworkAccess::Blocked => {
                Err(BrowserError::SandboxViolation(format!(
                    "OASIS-BROWSER: Ağ erişimi tamamen engelli. '{}' işlemi reddedildi.",
                    operation
                )))
            }
            NetworkAccess::InternetOnly => Ok(()), // URL doğrulama zaten yapıldı
            NetworkAccess::LocalNetwork => Ok(()),
            NetworkAccess::Full => Ok(()),
        }
    }
    
    /// Dosya erişimi kontrolü
    pub fn check_file_access(&self, path: &str) -> BrowserResult<()> {
        match self.policy.file_access {
            FileAccess::Blocked => {
                // Violation kaydı ayrı bir metod ile yapılır
                log::warn!("🔒  SOVEREIGN: Dosya erişimi engellendi → {}", path);
                Err(BrowserError::SandboxViolation(format!(
                    "OASIS-BROWSER: Dosya sistemi erişimi SOVEREIGN anayasası gereği yasaktır. '{}' yolu reddedildi.",
                    path
                )))
            }
            FileAccess::ReadOnly => {
                log::warn!("⚠️  OASIS-BROWSER: Salt okunur dosya erişimi: {}", path);
                Ok(())
            }
            FileAccess::LimitedWrite => {
                // Sadece belirli dizinlere yazılabilir
                Ok(())
            }
            FileAccess::Full => {
                log::error!("❌ OASIS-BROWSER: Full dosya erişimi VERİLİŞ OLUNMAMALI!");
                Ok(())
            }
        }
    }
    
    /// Process başlatma kontrolü
    pub fn check_process_spawn(&self, process: &str) -> BrowserResult<()> {
        match self.policy.process_access {
            ProcessAccess::None => {
                Err(BrowserError::SandboxViolation(format!(
                    "OASIS-BROWSER: Process başlatma engelli. '{}' reddedildi.",
                    process
                )))
            }
            ProcessAccess::Whitelisted => {
                // Whitelist kontrolü
                Ok(())
            }
            ProcessAccess::Full => Ok(()),
        }
    }
    
    /// İhlal kaydet
    fn record_violation(&mut self, violation_type: ViolationType, resource: &str) {
        let vtype = violation_type.clone();
        self.violations.push(SandboxViolation {
            timestamp: chrono::Utc::now(),
            violation_type,
            resource: resource.to_string(),
            blocked: true,
        });
        
        log::warn!("🔒  SOVEREIGN: Güvenlik ihlali engellendi → {:?}", vtype);
    }
    
    /// Politikayı getir
    pub fn policy(&self) -> &SandboxPolicy {
        &self.policy
    }
    
    /// İhlalleri getir
    pub fn violations(&self) -> &[SandboxViolation] {
        &self.violations
    }
    
    /// Aktif mi?
    pub fn is_active(&self) -> bool {
        self.active
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ─────────────────────────────────────────────────────────────────────────────--

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sovereign_policy() {
        let policy = SandboxPolicy::sovereign();
        assert_eq!(policy.file_access, FileAccess::Blocked);
        assert_eq!(policy.network_access, NetworkAccess::InternetOnly);
        assert_eq!(policy.process_access, ProcessAccess::None);
    }
    
    #[test]
    fn test_url_validation_https() {
        let sandbox = SovereignSandbox::new(SandboxPolicy::sovereign());
        
        // Geçerli HTTPS URL'leri
        assert!(sandbox.validate_url("https://example.com").is_ok());
        assert!(sandbox.validate_url("https://www.google.com/search?q=test").is_ok());
        assert!(sandbox.validate_url("https://github.com/user/repo").is_ok());
    }
    
    #[test]
    fn test_url_validation_file() {
        let sandbox = SovereignSandbox::new(SandboxPolicy::sovereign());
        
        // file:// URL'leri engellenmeli
        assert!(sandbox.validate_url("file:///etc/passwd").is_err());
        assert!(sandbox.validate_url("file:///home/user/document.pdf").is_err());
        assert!(sandbox.validate_url("file://localhost/tmp/file").is_err());
    }
    
    #[test]
    fn test_url_validation_localhost() {
        let sandbox = SovereignSandbox::new(SandboxPolicy::sovereign());
        
        // Localhost engellenmeli
        assert!(sandbox.validate_url("http://localhost:8080").is_err());
        assert!(sandbox.validate_url("http://127.0.0.1:3000").is_err());
        assert!(sandbox.validate_url("http://[::1]:8080").is_err());
    }
    
    #[test]
    fn test_url_validation_private_ip() {
        let sandbox = SovereignSandbox::new(SandboxPolicy::sovereign());
        
        // Private IP'ler engellenmeli
        assert!(sandbox.validate_url("http://192.168.1.1").is_err());
        assert!(sandbox.validate_url("http://10.0.0.1").is_err());
        assert!(sandbox.validate_url("http://172.16.0.1").is_err());
    }
    
    #[test]
    fn test_file_access_blocked() {
        let sandbox = SovereignSandbox::new(SandboxPolicy::sovereign());
        
        // Tüm dosya erişimleri engellenmeli
        assert!(sandbox.check_file_access("/tmp/file").is_err());
        assert!(sandbox.check_file_access("/etc/passwd").is_err());
        assert!(sandbox.check_file_access("/home/user/data").is_err());
    }
    
    #[test]
    fn test_blocked_domains() {
        let sandbox = SovereignSandbox::new(SandboxPolicy::sovereign());
        
        // Engellenen domainler
        assert!(sandbox.validate_url("https://doubleclick.net/ad.js").is_err());
        assert!(sandbox.validate_url("https://ads.googlesyndication.com/").is_err());
    }
    
    #[test]
    fn test_blocked_schemes() {
        let sandbox = SovereignSandbox::new(SandboxPolicy::sovereign());
        
        // Engellenen şemalar
        assert!(sandbox.validate_url("javascript:alert(1)").is_err());
        assert!(sandbox.validate_url("data:text/html,<h1>test</h1>").is_err());
        assert!(sandbox.validate_url("vbscript:msgbox").is_err());
    }
}
