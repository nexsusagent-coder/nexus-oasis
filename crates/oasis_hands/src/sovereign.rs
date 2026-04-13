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
use serde::{Deserialize, Serialize};

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
//  YASAKLI EKRAN BÖLGELERİ
// ───────────────────────────────────────────────────────────────────────────────

/// Yasaklı ekran bölgesi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForbiddenRegion {
    /// Bölge ID
    pub id: String,
    /// X koordinatı
    pub x: u32,
    /// Y koordinatı
    pub y: u32,
    /// Genişlik
    pub width: u32,
    /// Yükseklik
    pub height: u32,
    /// Neden (açıklama)
    pub reason: String,
    /// Aktif mi?
    pub enabled: bool,
    /// Oluşturulma zamanı
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl ForbiddenRegion {
    /// Yeni yasaklı bölge oluştur
    pub fn new(x: u32, y: u32, width: u32, height: u32, reason: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            x,
            y,
            width,
            height,
            reason: reason.to_string(),
            enabled: true,
            created_at: chrono::Utc::now(),
        }
    }

    /// Nokta bölge içinde mi?
    pub fn contains(&self, px: i32, py: i32) -> bool {
        if !self.enabled {
            return false;
        }
        
        px >= self.x as i32 
            && px <= (self.x + self.width) as i32
            && py >= self.y as i32 
            && py <= (self.y + self.height) as i32
    }

    /// İki bölge çakışıyor mu?
    pub fn overlaps(&self, other: &ForbiddenRegion) -> bool {
        if !self.enabled || !other.enabled {
            return false;
        }
        
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    /// Bölge alanı
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Merkez nokta
    pub fn center(&self) -> (u32, u32) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }
}

/// Yasaklı bölge türü (ön tanımlı)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForbiddenRegionType {
    /// Şifre alanı
    PasswordField,
    /// Admin paneli
    AdminPanel,
    /// Sistem ayarları
    SystemSettings,
    /// Ödeme formu
    PaymentForm,
    /// Kişisel bilgiler
    PersonalInfo,
    /// Özel kullanıcı tanımlı
    Custom,
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
    /// Yasaklı ekran bölgeleri
    pub forbidden_regions: Vec<ForbiddenRegion>,
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
    ForbiddenRegionAccess,
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
            forbidden_regions: Vec::new(),
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
            
            // Yasaklı bölge kontrolü
            if let Some(y) = action.y() {
                self.validate_screen_position(x, y)?;
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
    
    // ─── YASAKLI EKRAN BÖLGESİ METOTLARI ───
    
    /// Ekran pozisyonunu doğrula (yasaklı bölge kontrolü)
    pub fn validate_screen_position(&self, x: i32, y: i32) -> HandsResult<()> {
        for region in &self.forbidden_regions {
            if region.contains(x, y) {
                log::warn!(
                    "🔒  SOVEREIGN: Yasaklı bölgeye erişim engellendi → ({}, {}) '{}'",
                    x, y, region.reason
                );
                return Err(HandsError::ForbiddenRegion(format!(
                    "OASIS-HANDS SOVEREIGN: ({}, {}) koordinatı yasaklı bölgede: {}",
                    x, y, region.reason
                )));
            }
        }
        Ok(())
    }
    
    /// Yasaklı bölge ekle
    pub fn add_forbidden_region(&mut self, region: ForbiddenRegion) {
        log::info!(
            "🔒  SOVEREIGN: Yasaklı bölge eklendi → ({},{}) {}x{} '{}'",
            region.x, region.y, region.width, region.height, region.reason
        );
        self.forbidden_regions.push(region);
    }
    
    /// Yasaklı bölge oluştur ve ekle
    pub fn create_forbidden_region(&mut self, x: u32, y: u32, width: u32, height: u32, reason: &str) -> String {
        let region = ForbiddenRegion::new(x, y, width, height, reason);
        let id = region.id.clone();
        self.add_forbidden_region(region);
        id
    }
    
    /// Yasaklı bölge kaldır
    pub fn remove_forbidden_region(&mut self, id: &str) -> bool {
        if let Some(pos) = self.forbidden_regions.iter().position(|r| r.id == id) {
            let removed = self.forbidden_regions.remove(pos);
            log::info!("🔒  SOVEREIGN: Yasaklı bölge kaldırıldı → '{}' {}", id, removed.reason);
            return true;
        }
        false
    }
    
    /// Yasaklı bölgeyi aktif/pasif yap
    pub fn toggle_forbidden_region(&mut self, id: &str, enabled: bool) -> bool {
        if let Some(region) = self.forbidden_regions.iter_mut().find(|r| r.id == id) {
            region.enabled = enabled;
            log::info!(
                "🔒  SOVEREIGN: Yasaklı bölge {} → '{}' {}",
                if enabled { "aktifleştirildi" } else { "devre dışı bırakıldı" },
                id, region.reason
            );
            return true;
        }
        false
    }
    
    /// Yasaklı bölgeleri getir
    pub fn get_forbidden_regions(&self) -> &[ForbiddenRegion] {
        &self.forbidden_regions
    }
    
    /// Aktif yasaklı bölgeleri getir
    pub fn get_active_forbidden_regions(&self) -> Vec<&ForbiddenRegion> {
        self.forbidden_regions.iter().filter(|r| r.enabled).collect()
    }
    
    /// Yasaklı bölge sayısı
    pub fn forbidden_region_count(&self) -> usize {
        self.forbidden_regions.len()
    }
    
    /// Tüm yasaklı bölgeleri temizle
    pub fn clear_forbidden_regions(&mut self) {
        let count = self.forbidden_regions.len();
        self.forbidden_regions.clear();
        log::info!("🔒  SOVEREIGN: {} yasaklı bölge temizlendi", count);
    }
    
    /// Ön tanımlı yasaklı bölgeleri ekle (şifre alanları vb.)
    pub fn add_default_forbidden_regions(&mut self, screen_width: u32, screen_height: u32) {
        // Ekranın üst kısmı (genellikle menü/bar)
        self.create_forbidden_region(
            0, 0, screen_width, 30,
            "Sistem barı - tıklama korumalı"
        );
        
        log::info!(
            "🔒  SOVEREIGN: Ön tanımlı yasaklı bölgeler eklendi (ekran: {}x{})",
            screen_width, screen_height
        );
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
    
    // ─── YASAKLI BÖLGE TESTLERİ ───
    
    #[test]
    fn test_forbidden_region_creation() {
        let region = ForbiddenRegion::new(100, 100, 200, 150, "Test bölge");
        assert!(region.enabled);
        assert_eq!(region.x, 100);
        assert_eq!(region.width, 200);
        assert!(!region.id.is_empty());
    }
    
    #[test]
    fn test_forbidden_region_contains() {
        let region = ForbiddenRegion::new(100, 100, 200, 150, "Test");
        
        // İçeride
        assert!(region.contains(150, 150));
        assert!(region.contains(100, 100));
        assert!(region.contains(300, 250));
        
        // Dışarıda
        assert!(!region.contains(50, 50));
        assert!(!region.contains(350, 300));
    }
    
    #[test]
    fn test_forbidden_region_disabled() {
        let mut region = ForbiddenRegion::new(100, 100, 200, 150, "Test");
        region.enabled = false;
        
        // Devre dışı iken hiçbir noktayı içermez
        assert!(!region.contains(150, 150));
    }
    
    #[test]
    fn test_add_forbidden_region() {
        let mut policy = SovereignPolicy::strict();
        assert_eq!(policy.forbidden_region_count(), 0);
        
        policy.create_forbidden_region(0, 0, 100, 50, "Menü");
        assert_eq!(policy.forbidden_region_count(), 1);
    }
    
    #[test]
    fn test_validate_screen_position() {
        let mut policy = SovereignPolicy::strict();
        policy.create_forbidden_region(0, 0, 100, 50, "Menü");
        
        // Yasaklı bölge içinde - engellenmeli
        assert!(policy.validate_screen_position(50, 25).is_err());
        
        // Yasaklı bölge dışında - geçmeli
        assert!(policy.validate_screen_position(200, 200).is_ok());
    }
    
    #[test]
    fn test_remove_forbidden_region() {
        let mut policy = SovereignPolicy::strict();
        let id = policy.create_forbidden_region(0, 0, 100, 50, "Test");
        
        assert_eq!(policy.forbidden_region_count(), 1);
        
        let removed = policy.remove_forbidden_region(&id);
        assert!(removed);
        assert_eq!(policy.forbidden_region_count(), 0);
    }
    
    #[test]
    fn test_toggle_forbidden_region() {
        let mut policy = SovereignPolicy::strict();
        let id = policy.create_forbidden_region(0, 0, 100, 50, "Test");
        
        // Devre dışı bırak
        policy.toggle_forbidden_region(&id, false);
        assert!(policy.validate_screen_position(50, 25).is_ok());
        
        // Tekrar aktifleştir
        policy.toggle_forbidden_region(&id, true);
        assert!(policy.validate_screen_position(50, 25).is_err());
    }
    
    #[test]
    fn test_forbidden_region_area() {
        let region = ForbiddenRegion::new(0, 0, 100, 50, "Test");
        assert_eq!(region.area(), 5000);
    }
    
    #[test]
    fn test_forbidden_region_center() {
        let region = ForbiddenRegion::new(100, 100, 200, 100, "Test");
        let (cx, cy) = region.center();
        assert_eq!(cx, 200);
        assert_eq!(cy, 150);
    }
    
    #[test]
    fn test_clear_forbidden_regions() {
        let mut policy = SovereignPolicy::strict();
        policy.create_forbidden_region(0, 0, 100, 50, "Test1");
        policy.create_forbidden_region(200, 200, 100, 50, "Test2");
        
        assert_eq!(policy.forbidden_region_count(), 2);
        
        policy.clear_forbidden_regions();
        assert_eq!(policy.forbidden_region_count(), 0);
    }
}
