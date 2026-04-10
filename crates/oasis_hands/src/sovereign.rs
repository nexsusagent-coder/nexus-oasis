//! ═══════════════════════════════════════════════════════════════════════════════
//!  SOVEREIGN POLICY - L1 ANAYASA GÜVENLİĞİ
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Masaüstü kontrolü için L1 Sovereign Anayasası kısıtlamaları.
//! 
//! ═──────────────────────────────────────────────────────────────────────────────
//!  ERİŞİM MATRİSİ:
//!  ────────────────
//!  Dosya Sistemi   → ⚠️  WHITELIST (sadece izin verilen dizinler)
//!  Process Başlat  → ⚠️  WHITELIST (sadece izin verilen uygulamalar)
//!  Ağ              → ❌ BLOCKED (Agent-S masaüstü için ağ erişimi yok)
//!  Tehlikeli Komut → ❌ BLOCKED (rm -rf, format, dd, etc.)
//!  GUI Kontrol     → ✅ ALLOWED (mesai sınırları ile)
//! ═──────────────────────────────────────────────────────────────────────────────

use crate::error::{HandsError, HandsResult};
use crate::{ALLOWED_APPS, BLOCKED_COMMANDS, ALLOWED_PATHS, BLOCKED_PATHS};
use std::collections::HashSet;

// ───────────────────────────────────────────────────────────────────────────────
//  POLİTİKA TİPLERİ
// ─────────────────────────────────────────────────────────────────────────────--

/// Dosya erişim politikası
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileAccessPolicy {
    /// Tamamen engelli
    Blocked,
    /// Sadece whitelist dizinler
    Whitelist,
    /// Salt okunur
    ReadOnly,
    /// Sınırlı yazma
    LimitedWrite,
}

/// Process politikası
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessPolicy {
    /// Hiçbir process başlatılamaz
    Blocked,
    /// Sadece whitelist uygulamalar
    Whitelist,
    /// Hepsi serbest
    Full,
}

/// Ağ politikası
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkPolicy {
    /// Ağ erişimi yok
    Blocked,
    /// Sadece localhost
    Localhost,
    /// Tam erişim
    Full,
}

// ───────────────────────────────────────────────────────────────────────────────
//  SOVEREIGN POLİTİKA
// ─────────────────────────────────────────────────────────────────────────────--

#[derive(Debug, Clone)]
pub struct SovereignPolicy {
    /// Dosya erişim politikası
    pub file_access: FileAccessPolicy,
    /// Process politikası
    pub process: ProcessPolicy,
    /// Ağ politikası
    pub network: NetworkPolicy,
    /// İzin verilen dizinler
    pub allowed_paths: HashSet<String>,
    /// Engellenen dizinler
    pub blocked_paths: HashSet<String>,
    /// İzin verilen uygulamalar
    pub allowed_apps: HashSet<String>,
    /// Engellenen komutlar
    pub blocked_commands: HashSet<String>,
    /// Maksimum işlem süresi (saniye)
    pub max_duration_secs: u64,
    /// Maksimum fare hareketi (px)
    pub max_mouse_distance: u32,
    /// GUI kontrolüne izin var mı?
    pub gui_control_allowed: bool,
    /// Onay gerekli mi?
    pub require_confirmation: bool,
    /// Aktif mi?
    active: bool,
    /// İhlaller
    violations: Vec<PolicyViolation>,
}

/// Politika ihlali kaydı
#[derive(Debug, Clone)]
pub struct PolicyViolation {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub violation_type: ViolationType,
    pub resource: String,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub enum ViolationType {
    FileAccess,
    ProcessSpawn,
    BlockedCommand,
    BlockedPath,
    BlockedApp,
    NetworkAccess,
    Timeout,
    MouseLimitExceeded,
}

impl SovereignPolicy {
    /// Katı Sovereign politika - En yüksek güvenlik
    pub fn strict() -> Self {
        let mut allowed_paths = HashSet::new();
        for path in ALLOWED_PATHS {
            allowed_paths.insert(path.to_string());
        }
        
        let mut blocked_paths = HashSet::new();
        for path in BLOCKED_PATHS {
            blocked_paths.insert(path.to_string());
        }
        
        let mut allowed_apps = HashSet::new();
        for app in ALLOWED_APPS {
            allowed_apps.insert(app.to_string());
        }
        
        let mut blocked_commands = HashSet::new();
        for cmd in BLOCKED_COMMANDS {
            blocked_commands.insert(cmd.to_string());
        }
        
        Self {
            file_access: FileAccessPolicy::Whitelist,
            process: ProcessPolicy::Whitelist,
            network: NetworkPolicy::Blocked,
            allowed_paths,
            blocked_paths,
            allowed_apps,
            blocked_commands,
            max_duration_secs: 300,
            max_mouse_distance: 10000,
            gui_control_allowed: true,
            require_confirmation: true,
            active: false,
            violations: Vec::new(),
        }
    }
    
    /// Geliştirici modu - Daha esnek ama yine de güvenli
    pub fn developer() -> Self {
        let mut policy = Self::strict();
        policy.require_confirmation = false;
        policy.max_duration_secs = 600; // 10 dakika
        policy
    }
    
    /// Demo modu - Sadece gözlem
    pub fn demo() -> Self {
        let mut policy = Self::strict();
        policy.gui_control_allowed = false; // Sadece izle, dokunma
        policy.require_confirmation = false;
        policy
    }
    
    /// Politika'yı aktive et
    pub fn activate(&mut self) -> HandsResult<()> {
        if self.active {
            return Ok(());
        }
        
        self.active = true;
        log::info!("🔒  SOVEREIGN: Masaüstü kontrol politikası aktif");
        log::info!("🔒  SOVEREIGN: Dosya erişimi: {:?}", self.file_access);
        log::info!("🔒  SOVEREIGN: Process politikası: {:?}", self.process);
        log::info!("🔒  SOVEREIGN: Ağ erişimi: {:?}", self.network);
        Ok(())
    }
    
    /// Politika'yı deaktif et
    pub fn deactivate(&mut self) -> HandsResult<()> {
        self.active = false;
        log::info!("🔒  SOVEREIGN: Politika deaktif");
        Ok(())
    }
    
    /// Aktif mi?
    pub fn is_active(&self) -> bool {
        self.active
    }
    
    // ─── DOĞRULAMA METODLARI ───
    
    /// Dosya erişimini doğrula
    pub fn validate_file_access(&self, path: &str, write: bool) -> HandsResult<()> {
        // Önce engellenen yolları kontrol et
        let path_lower = path.to_lowercase();
        for blocked in &self.blocked_paths {
            if path_lower.starts_with(&blocked.to_lowercase()) {
                log::warn!("🔒  SOVEREIGN: Güvenlik ihlali → Engellenmiş yola erişim: {}", path);
                return Err(HandsError::SovereignViolation(format!(
                    "OASIS-HANDS SOVEREIGN: '{}' yolu sistem koruması altındadır. Erişim engellendi.",
                    path
                )));
            }
        }
        
        // Politikaya göre kontrol
        match self.file_access {
            FileAccessPolicy::Blocked => {
                log::warn!("🔒  SOVEREIGN: Dosya erişimi engelli → {}", path);
                Err(HandsError::FileAccess(
                    "OASIS-HANDS SOVEREIGN: Dosya erişimi bu politikada tamamen engellidir.".into()
                ))
            }
            FileAccessPolicy::Whitelist => {
                // Sadece izin verilen dizinler
                for allowed in &self.allowed_paths {
                    if path_lower.starts_with(&allowed.to_lowercase()) {
                        return Ok(());
                    }
                }
                log::warn!("🔒  SOVEREIGN: Whitelist dışı dizine erişim → {}", path);
                Err(HandsError::FileAccess(format!(
                    "OASIS-HANDS SOVEREIGN: '{}' yolu izin verilen dizinler dışındadır.",
                    path
                )))
            }
            FileAccessPolicy::ReadOnly => {
                if write {
                    Err(HandsError::FileAccess(
                        "OASIS-HANDS SOVEREIGN: Salt okunur modda yazma işlemi yapılamaz.".into()
                    ))
                } else {
                    Ok(())
                }
            }
            FileAccessPolicy::LimitedWrite => Ok(()),
        }
    }
    
    /// Komut doğrula
    pub fn validate_command(&self, command: &str) -> HandsResult<()> {
        let command_lower = command.to_lowercase();
        
        // Tehlikeli komutları kontrol et
        for blocked in &self.blocked_commands {
            if command_lower.contains(&blocked.to_lowercase()) {
                log::warn!("🔒  SOVEREIGN: Tehlikeli komut engellendi → {}", blocked);
                return Err(HandsError::CommandBlocked(format!(
                    "OASIS-HANDS SOVEREIGN: Tehlikeli komut engellendi! '{}' komutu sistem güvenliği için yasaktır.",
                    blocked
                )));
            }
        }
        
        // Sudo kontrolü
        if command_lower.contains("sudo") {
            log::warn!("🔒  SOVEREIGN: sudo kullanımı engellendi");
            return Err(HandsError::CommandBlocked(
                "OASIS-HANDS SOVEREIGN: 'sudo' ile komut çalıştırma izni yoktur.".into()
            ));
        }
        
        // Pipe zincirleme kontrolü
        if command.matches('|').count() > 3 {
            log::warn!("⚠️  OASIS-HANDS SOVEREIGN: Karmaşık pipe zinciri tespit edildi: {}", command);
        }
        
        Ok(())
    }
    
    /// Uygulama doğrula
    pub fn validate_application(&self, app_name: &str) -> HandsResult<()> {
        match self.process {
            ProcessPolicy::Blocked => {
                Err(HandsError::AppBlocked(
                    "OASIS-HANDS SOVEREIGN: Process başlatma tamamen engelli.".into()
                ))
            }
            ProcessPolicy::Whitelist => {
                let app_lower = app_name.to_lowercase();
                for allowed in &self.allowed_apps {
                    if app_lower.contains(&allowed.to_lowercase()) || 
                       app_lower.starts_with(&allowed.to_lowercase()) {
                        return Ok(());
                    }
                }
                log::warn!("🔒  SOVEREIGN: Whitelist dışı uygulama → {}", app_name);
                Err(HandsError::AppBlocked(format!(
                    "OASIS-HANDS SOVEREIGN: '{}' uygulaması izin verilenler listesinde değil.",
                    app_name
                )))
            }
            ProcessPolicy::Full => Ok(()),
        }
    }
    
    /// Fare aksiyonunu doğrula
    pub fn validate_mouse_action(&self, action: &crate::input::MouseAction) -> HandsResult<()> {
        if !self.gui_control_allowed {
            return Err(HandsError::SovereignViolation(
                "OASIS-HANDS SOVEREIGN: GUI kontrolü bu politikada izin verilmemiş.".into()
            ));
        }
        
        // Ekran sınırları kontrolü
        if let Some(x) = action.x() {
            if x < 0 || x as u32 > crate::MAX_SCREEN_WIDTH {
                return Err(HandsError::InputError(format!(
                    "OASIS-HANDS SOVEREIGN: X koordinatı ekran dışında: {}", x
                )));
            }
        }
        
        if let Some(y) = action.y() {
            if y < 0 || y as u32 > crate::MAX_SCREEN_HEIGHT {
                return Err(HandsError::InputError(format!(
                    "OASIS-HANDS SOVEREIGN: Y koordinatı ekran dışında: {}", y
                )));
            }
        }
        
        Ok(())
    }
    
    /// Klavye aksiyonunu doğrula
    pub fn validate_keyboard_action(&self, action: &crate::input::KeyboardAction) -> HandsResult<()> {
        if !self.gui_control_allowed {
            return Err(HandsError::SovereignViolation(
                "OASIS-HANDS SOVEREIGN: GUI kontrolü bu politikada izin verilmemiş.".into()
            ));
        }
        
        // Tehlikeli kısayol kombinasyonları
        let text = action.text().unwrap_or("");
        let dangerous_shortcuts = [
            "ctrl+alt+delete",
            "ctrl+alt+f1", "ctrl+alt+f2",
            "alt+f4", // Pencere kapatma
        ];
        
        let text_lower = text.to_lowercase();
        for shortcut in dangerous_shortcuts {
            if text_lower.contains(shortcut) {
                return Err(HandsError::InputError(format!(
                    "OASIS-HANDS SOVEREIGN: '{}' kısayolu güvenlik nedeniyle engellendi.",
                    shortcut
                )));
            }
        }
        
        Ok(())
    }
    
    /// İhlal kaydet
    fn record_violation(&mut self, vtype: ViolationType, resource: &str, reason: &str) {
        self.violations.push(PolicyViolation {
            timestamp: chrono::Utc::now(),
            violation_type: vtype.clone(),
            resource: resource.to_string(),
            reason: reason.to_string(),
        });
        
        log::warn!("🔒  SOVEREIGN: Güvenlik ihlali kaydedildi → {:?}", vtype);
    }
    
    /// İhlalleri getir
    pub fn violations(&self) -> &[PolicyViolation] {
        &self.violations
    }
    
    /// İhlal sayısı
    pub fn violation_count(&self) -> usize {
        self.violations.len()
    }
    
    /// Whitelist'e dizin ekle
    pub fn add_allowed_path(&mut self, path: &str) {
        self.allowed_paths.insert(path.to_string());
        log::info!("🔒  SOVEREIGN: '{}' dizini whitelist'e eklendi", path);
    }
    
    /// Whitelist'ten dizin kaldır
    pub fn remove_allowed_path(&mut self, path: &str) {
        self.allowed_paths.remove(path);
        log::info!("🔒  SOVEREIGN: '{}' dizini whitelist'ten kaldırıldı", path);
    }
    
    /// Blacklist'e komut ekle
    pub fn add_blocked_command(&mut self, command: &str) {
        self.blocked_commands.insert(command.to_string());
        log::info!("🔒  SOVEREIGN: '{}' komutu blacklist'e eklendi", command);
    }
}

impl Default for SovereignPolicy {
    fn default() -> Self {
        Self::strict()
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ─────────────────────────────────────────────────────────────────────────────--

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_strict_policy_creation() {
        let policy = SovereignPolicy::strict();
        assert_eq!(policy.file_access, FileAccessPolicy::Whitelist);
        assert_eq!(policy.process, ProcessPolicy::Whitelist);
        assert_eq!(policy.network, NetworkPolicy::Blocked);
        assert!(policy.require_confirmation);
    }
    
    #[test]
    fn test_policy_activation() {
        let mut policy = SovereignPolicy::strict();
        assert!(!policy.is_active());
        policy.activate().expect("operation failed");
        assert!(policy.is_active());
    }
    
    #[test]
    fn test_blocked_command() {
        let policy = SovereignPolicy::strict();
        
        // Tehlikeli komutlar engellenmeli
        assert!(policy.validate_command("rm -rf /home").is_err());
        assert!(policy.validate_command("sudo rm -rf /").is_err());
        assert!(policy.validate_command("dd if=/dev/zero of=/dev/sda").is_err());
        assert!(policy.validate_command("shutdown -h now").is_err());
    }
    
    #[test]
    fn test_allowed_command() {
        let policy = SovereignPolicy::strict();
        
        // Normal komutlar geçmeli
        assert!(policy.validate_command("ls -la").is_ok());
        assert!(policy.validate_command("cat file.txt").is_ok());
        assert!(policy.validate_command("echo hello").is_ok());
    }
    
    #[test]
    fn test_blocked_path() {
        let policy = SovereignPolicy::strict();
        
        // Sistem dizinleri engellenmeli
        assert!(policy.validate_file_access("/etc/shadow", false).is_err());
        assert!(policy.validate_file_access("/root/.bashrc", false).is_err());
        assert!(policy.validate_file_access("/proc/self/cmdline", false).is_err());
    }
    
    #[test]
    fn test_allowed_path() {
        let policy = SovereignPolicy::strict();
        
        // İzin verilen dizinler
        assert!(policy.validate_file_access("/home/sentient/workspace/test.py", false).is_ok());
        assert!(policy.validate_file_access("/tmp/sentient/temp.txt", false).is_ok());
    }
    
    #[test]
    fn test_allowed_app() {
        let policy = SovereignPolicy::strict();
        
        assert!(policy.validate_application("firefox").is_ok());
        assert!(policy.validate_application("libreoffice").is_ok());
        assert!(policy.validate_application("gedit").is_ok());
    }
    
    #[test]
    fn test_blocked_app() {
        let policy = SovereignPolicy::strict();
        
        // Bilinmeyen uygulamalar engellenmeli
        assert!(policy.validate_application("malware").is_err());
        assert!(policy.validate_application("unknown_app").is_err());
    }
    
    #[test]
    fn test_developer_policy() {
        let policy = SovereignPolicy::developer();
        assert!(!policy.require_confirmation);
        assert_eq!(policy.max_duration_secs, 600);
    }
    
    #[test]
    fn test_demo_policy() {
        let policy = SovereignPolicy::demo();
        assert!(!policy.gui_control_allowed);
    }
    
    #[test]
    fn test_add_allowed_path() {
        let mut policy = SovereignPolicy::strict();
        policy.add_allowed_path("/tmp/custom");
        
        assert!(policy.validate_file_access("/tmp/custom/file.txt", false).is_ok());
    }
    
    #[test]
    fn test_violation_recording() {
        let mut policy = SovereignPolicy::strict();
        policy.activate().expect("operation failed");
        
        // Tehlikeli komut engellenmeli
        let result = policy.validate_command("rm -rf /");
        assert!(result.is_err());
        
        // Politika aktif olmalı
        assert!(policy.is_active());
    }
}
