//! ═══════════════════════════════════════════════════════════════════════════════
//!  SANDBOX MODE - GÜVENLİ TEST ORTAMI
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Masaüstü kontrolü için güvenli test/simülasyon ortamı.
//! 
//! ═──────────────────────────────────────────────────────────────────────────────
//!  MODLAR:
//!  ────────────────
//!  SimulateOnly   → Hiçbir gerçek aksiyon çalışmaz, sadece simüle
//!  DryRun         → Log'lar ama gerçek işlem yapmaz
//!  Preview        → Ekran görüntüsü alır ama tıklamaz
//!  FakeResponses  → Simüle edilmiş sonuçlar döner
//! ═──────────────────────────────────────────────────────────────────────────────

use crate::error::{HandsError, HandsResult};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// ───────────────────────────────────────────────────────────────────────────────
//  SANDBOX MODU
// ───────────────────────────────────────────────────────────────────────────────

/// Sandbox çalışma modu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SandboxModeType {
    /// Hiçbir gerçek aksiyon çalışmaz, sadece simüle eder
    SimulateOnly,
    /// Log'lar ama gerçek işlem yapmaz
    DryRun,
    /// Ekran görüntüsü alır ama tıklamaz
    Preview,
    /// Simüle edilmiş sonuçlar döner
    FakeResponses,
    /// Normal mod (sandbox devre dışı)
    Normal,
}

impl SandboxModeType {
    /// Mod açıklayıcı metni
    pub fn description(&self) -> &'static str {
        match self {
            SandboxModeType::SimulateOnly => "Simülasyon - gerçek işlem yok",
            SandboxModeType::DryRun => "Dry-run - sadece log",
            SandboxModeType::Preview => "Preview - görüntüleme modu",
            SandboxModeType::FakeResponses => "Fake responses - sahte yanıtlar",
            SandboxModeType::Normal => "Normal mod - sandbox devre dışı",
        }
    }
    
    /// Bu modda gerçek işlem yapılıyor mu?
    pub fn is_real_execution(&self) -> bool {
        matches!(self, SandboxModeType::Normal)
    }
    
    /// Bu mod sandbox aktif mi?
    pub fn is_sandbox(&self) -> bool {
        !self.is_real_execution()
    }
    
    /// Bu modda log tutuluyor mu?
    pub fn should_log(&self) -> bool {
        matches!(self, SandboxModeType::DryRun | SandboxModeType::SimulateOnly)
    }
}

impl Default for SandboxModeType {
    fn default() -> Self {
        Self::Normal
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  SANDBOX AYARLARI
// ───────────────────────────────────────────────────────────────────────────────

/// Sandbox ayarları
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    /// Sandbox modu
    pub mode: SandboxModeType,
    /// Simülasyon gecikmesi (ms)
    pub simulation_delay_ms: u64,
    /// Sahte yanıt şablonu
    pub fake_response_template: String,
    /// Maksimum simüle edilen aksiyon sayısı
    pub max_simulated_actions: usize,
    /// Detaylı log tut
    pub verbose_logging: bool,
    /// Ekran görüntüsü kaydet
    pub capture_screenshots: bool,
    /// Otomatik geri al (test sonrası)
    pub auto_rollback: bool,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            mode: SandboxModeType::Normal,
            simulation_delay_ms: 100,
            fake_response_template: "OK: {action} simulated".to_string(),
            max_simulated_actions: 1000,
            verbose_logging: false,
            capture_screenshots: false,
            auto_rollback: false,
        }
    }
}

impl SandboxConfig {
    /// Yeni sandbox config oluştur
    pub fn new(mode: SandboxModeType) -> Self {
        Self {
            mode,
            ..Default::default()
        }
    }
    
    /// SimulateOnly modunda config
    pub fn simulate_only() -> Self {
        Self {
            mode: SandboxModeType::SimulateOnly,
            verbose_logging: true,
            capture_screenshots: true,
            ..Default::default()
        }
    }
    
    /// DryRun modunda config
    pub fn dry_run() -> Self {
        Self {
            mode: SandboxModeType::DryRun,
            verbose_logging: true,
            ..Default::default()
        }
    }
    
    /// Preview modunda config
    pub fn preview() -> Self {
        Self {
            mode: SandboxModeType::Preview,
            capture_screenshots: true,
            ..Default::default()
        }
    }
    
    /// FakeResponses modunda config
    pub fn fake_responses() -> Self {
        Self {
            mode: SandboxModeType::FakeResponses,
            ..Default::default()
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  AKSİYON SONUCU
// ───────────────────────────────────────────────────────────────────────────────

/// Simüle edilmiş aksiyon sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimulatedResult {
    /// Başarılı simülasyon
    Success {
        action: String,
        duration_ms: u64,
        message: String,
    },
    /// Simülasyon başarısız (senaryo)
    SimulatedFailure {
        action: String,
        error: String,
    },
    /// Dry-run sonucu
    DryRunResult {
        action: String,
        would_execute: bool,
        log: String,
    },
    /// Preview sonucu
    PreviewResult {
        action: String,
        screenshot_path: Option<String>,
    },
    /// Sahte yanıt
    FakeResult {
        action: String,
        response: String,
    },
}

impl SimulatedResult {
    /// Başarılı mı?
    pub fn is_success(&self) -> bool {
        matches!(self, SimulatedResult::Success { .. } 
            | SimulatedResult::DryRunResult { .. }
            | SimulatedResult::PreviewResult { .. }
            | SimulatedResult::FakeResult { .. })
    }
    
    /// Aksiyon adını getir
    pub fn action_name(&self) -> &str {
        match self {
            SimulatedResult::Success { action, .. } => action,
            SimulatedResult::SimulatedFailure { action, .. } => action,
            SimulatedResult::DryRunResult { action, .. } => action,
            SimulatedResult::PreviewResult { action, .. } => action,
            SimulatedResult::FakeResult { action, .. } => action,
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  KAYITLI AKSİYON
// ───────────────────────────────────────────────────────────────────────────────

/// Kayıtlı aksiyon (geri alma için)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordedAction {
    /// Aksiyon ID
    pub id: String,
    /// Aksiyon türü
    pub action_type: String,
    /// Zaman damgası
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Parametreler (JSON)
    pub parameters: String,
    /// Önceki durum (geri alma için)
    pub previous_state: Option<String>,
    /// Simüle mi?
    pub was_simulated: bool,
}

impl RecordedAction {
    pub fn new(action_type: &str, parameters: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            action_type: action_type.to_string(),
            timestamp: chrono::Utc::now(),
            parameters: parameters.to_string(),
            previous_state: None,
            was_simulated: true,
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  SANDBOX YÖNETİCİSİ
// ───────────────────────────────────────────────────────────────────────────────

/// Sandbox yöneticisi
pub struct SandboxManager {
    /// Yapılandırma
    config: SandboxConfig,
    /// Aktif mi?
    active: Arc<AtomicBool>,
    /// Simüle edilen aksiyonlar
    simulated_actions: VecDeque<SimulatedResult>,
    /// Kayıtlı aksiyonlar (geri alma için)
    recorded_actions: Vec<RecordedAction>,
    /// İstatistikler
    stats: SandboxStats,
}

/// Sandbox istatistikleri
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SandboxStats {
    /// Toplam simüle edilen aksiyon
    pub total_simulated: u64,
    /// Başarılı simülasyonlar
    pub successful_simulations: u64,
    /// Başarısız simülasyonlar
    pub failed_simulations: u64,
    /// Toplam süre (ms)
    pub total_duration_ms: u64,
    /// Dry-run sayısı
    pub dry_run_count: u64,
    /// Preview sayısı
    pub preview_count: u64,
}

impl SandboxManager {
    /// Yeni sandbox yöneticisi oluştur
    pub fn new(config: SandboxConfig) -> Self {
        let active = config.mode.is_sandbox();
        Self {
            config,
            active: Arc::new(AtomicBool::new(active)),
            simulated_actions: VecDeque::new(),
            recorded_actions: Vec::new(),
            stats: SandboxStats::default(),
        }
    }
    
    /// Varsayılan yönetici (sandbox devre dışı)
    pub fn disabled() -> Self {
        Self::new(SandboxConfig::default())
    }
    
    /// SimulateOnly modunda
    pub fn simulate_only() -> Self {
        Self::new(SandboxConfig::simulate_only())
    }
    
    /// DryRun modunda
    pub fn dry_run() -> Self {
        Self::new(SandboxConfig::dry_run())
    }
    
    /// Preview modunda
    pub fn preview() -> Self {
        Self::new(SandboxConfig::preview())
    }
    
    /// FakeResponses modunda
    pub fn fake_responses() -> Self {
        Self::new(SandboxConfig::fake_responses())
    }
    
    // ─── KONTROL METOTLARI ───
    
    /// Sandbox aktif mi?
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::SeqCst)
    }
    
    /// Mevcut mod
    pub fn mode(&self) -> SandboxModeType {
        self.config.mode
    }
    
    /// Sandbox'ı etkinleştir
    pub fn enable(&mut self, mode: SandboxModeType) {
        self.config.mode = mode;
        self.active.store(true, Ordering::SeqCst);
        log::info!("🎮 SANDBOX: Etkinleştirildi → {}", mode.description());
    }
    
    /// Sandbox'ı devre dışı bırak
    pub fn disable(&mut self) {
        self.config.mode = SandboxModeType::Normal;
        self.active.store(false, Ordering::SeqCst);
        log::info!("🎮 SANDBOX: Devre dışı bırakıldı → Normal mod");
    }
    
    /// Sandbox modunu değiştir
    pub fn set_mode(&mut self, mode: SandboxModeType) {
        if mode.is_sandbox() {
            self.enable(mode);
        } else {
            self.disable();
        }
    }
    
    // ─── AKSİYON İŞLEME ───
    
    /// Aksiyonu sandbox modunda işle
    pub fn process_action(&mut self, action_type: &str, params: &str) -> HandsResult<SimulatedResult> {
        if !self.is_active() {
            return Err(HandsError::SandboxError(
                "OASIS-HANDS SANDBOX: Sandbox aktif değil".to_string()
            ));
        }
        
        // Maksimum aksiyon kontrolü
        if self.simulated_actions.len() >= self.config.max_simulated_actions {
            self.simulated_actions.pop_front();
        }
        
        let start = std::time::Instant::now();
        
        // Mod bazlı işlem
        let result = match self.config.mode {
            SandboxModeType::SimulateOnly => {
                self.process_simulate_only(action_type, params)
            }
            SandboxModeType::DryRun => {
                self.process_dry_run(action_type, params)
            }
            SandboxModeType::Preview => {
                self.process_preview(action_type, params)
            }
            SandboxModeType::FakeResponses => {
                self.process_fake_response(action_type, params)
            }
            SandboxModeType::Normal => {
                return Err(HandsError::SandboxError(
                    "OASIS-HANDS SANDBOX: Normal modda sandbox işlemi yapılamaz".to_string()
                ));
            }
        };
        
        // İstatistikleri güncelle
        let duration = start.elapsed().as_millis() as u64;
        self.stats.total_simulated += 1;
        self.stats.total_duration_ms += duration;
        
        if result.is_success() {
            self.stats.successful_simulations += 1;
        } else {
            self.stats.failed_simulations += 1;
        }
        
        // Kaydet
        self.simulated_actions.push_back(result.clone());
        
        // Aksiyonu kaydet (geri alma için)
        let recorded = RecordedAction::new(action_type, params);
        self.recorded_actions.push(recorded);
        
        // Simülasyon gecikmesi
        if self.config.simulation_delay_ms > 0 {
            std::thread::sleep(std::time::Duration::from_millis(self.config.simulation_delay_ms));
        }
        
        Ok(result)
    }
    
    /// SimulateOnly modu işlemi
    fn process_simulate_only(&mut self, action_type: &str, params: &str) -> SimulatedResult {
        if self.config.verbose_logging {
            log::info!("🎮 SANDBOX [SimulateOnly]: {} -> {}", action_type, params);
        }
        
        SimulatedResult::Success {
            action: action_type.to_string(),
            duration_ms: self.config.simulation_delay_ms,
            message: format!("Simulated: {} with params: {}", action_type, params),
        }
    }
    
    /// DryRun modu işlemi
    fn process_dry_run(&mut self, action_type: &str, params: &str) -> SimulatedResult {
        self.stats.dry_run_count += 1;
        
        log::info!("🎮 SANDBOX [DryRun]: Would execute {} -> {}", action_type, params);
        
        SimulatedResult::DryRunResult {
            action: action_type.to_string(),
            would_execute: true,
            log: format!("DRY-RUN: {} ({})", action_type, params),
        }
    }
    
    /// Preview modu işlemi
    fn process_preview(&mut self, action_type: &str, params: &str) -> SimulatedResult {
        self.stats.preview_count += 1;
        
        log::info!("🎮 SANDBOX [Preview]: {} -> {}", action_type, params);
        
        SimulatedResult::PreviewResult {
            action: action_type.to_string(),
            screenshot_path: if self.config.capture_screenshots {
                Some(format!("/tmp/sandbox_preview_{}.png", chrono::Utc::now().timestamp()))
            } else {
                None
            },
        }
    }
    
    /// FakeResponse modu işlemi
    fn process_fake_response(&mut self, action_type: &str, _params: &str) -> SimulatedResult {
        let response = self.config.fake_response_template
            .replace("{action}", action_type);
        
        if self.config.verbose_logging {
            log::info!("🎮 SANDBOX [FakeResponse]: {} -> {}", action_type, response);
        }
        
        SimulatedResult::FakeResult {
            action: action_type.to_string(),
            response,
        }
    }
    
    // ─── RAPORLAMA ───
    
    /// Simüle edilen aksiyonları getir
    pub fn get_simulated_actions(&self) -> &VecDeque<SimulatedResult> {
        &self.simulated_actions
    }
    
    /// Kayıtlı aksiyonları getir
    pub fn get_recorded_actions(&self) -> &[RecordedAction] {
        &self.recorded_actions
    }
    
    /// İstatistikleri getir
    pub fn stats(&self) -> &SandboxStats {
        &self.stats
    }
    
    /// İstatistikleri sıfırla
    pub fn reset_stats(&mut self) {
        self.stats = SandboxStats::default();
        self.simulated_actions.clear();
        log::info!("🎮 SANDBOX: İstatistikler sıfırlandı");
    }
    
    /// Tüm kayıtları temizle
    pub fn clear_all(&mut self) {
        self.simulated_actions.clear();
        self.recorded_actions.clear();
        self.stats = SandboxStats::default();
        log::info!("🎮 SANDBOX: Tüm kayıtlar temizlendi");
    }
    
    /// Sandbox raporu oluştur
    pub fn report(&self) -> SandboxReport {
        SandboxReport {
            is_active: self.is_active(),
            mode: self.config.mode,
            mode_description: self.config.mode.description().to_string(),
            total_simulated: self.stats.total_simulated,
            successful: self.stats.successful_simulations,
            failed: self.stats.failed_simulations,
            total_duration_ms: self.stats.total_duration_ms,
            dry_run_count: self.stats.dry_run_count,
            preview_count: self.stats.preview_count,
            recorded_actions_count: self.recorded_actions.len(),
        }
    }
    
    /// Config'i getir
    pub fn config(&self) -> &SandboxConfig {
        &self.config
    }
    
    /// Config'i değiştir
    pub fn set_config(&mut self, config: SandboxConfig) {
        self.config = config;
    }
    
    // ─── GÜVENLİ ÇALIŞTIRMA ───
    
    /// Sandbox içinde güvenli çalıştır
    /// 
    /// Eğer sandbox aktifse simüle eder, değilse gerçek işlemi yapar.
    pub fn safe_execute<F, T>(&mut self, action_type: &str, params: &str, real_action: F) -> HandsResult<T>
    where
        F: FnOnce() -> HandsResult<T>,
        T: Clone,
    {
        if self.is_active() {
            // Sandbox modunda - simüle et
            let result = self.process_action(action_type, params)?;
            
            // Simülasyon başarılı ama gerçek değer döndüremeyiz
            // Bu durumda default değer veya hata döndürmeliyiz
            return Err(HandsError::SandboxError(format!(
                "Sandbox aktif - gerçek işlem yapılamaz: {}",
                result.action_name()
            )));
        }
        
        // Gerçek işlemi yap
        real_action()
    }
}

/// Sandbox raporu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxReport {
    pub is_active: bool,
    pub mode: SandboxModeType,
    pub mode_description: String,
    pub total_simulated: u64,
    pub successful: u64,
    pub failed: u64,
    pub total_duration_ms: u64,
    pub dry_run_count: u64,
    pub preview_count: u64,
    pub recorded_actions_count: usize,
}

impl std::fmt::Display for SandboxReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "╔════════════════════════════════════════════╗")?;
        writeln!(f, "║         SANDBOX MODE RAPORU                ║")?;
        writeln!(f, "╠════════════════════════════════════════════╣")?;
        writeln!(f, "║ Aktif: {:<36} ║", if self.is_active { "Evet" } else { "Hayır" })?;
        writeln!(f, "║ Mod: {:<38} ║", self.mode_description)?;
        writeln!(f, "╠════════════════════════════════════════════╣")?;
        writeln!(f, "║ Toplam Simülasyon: {:<23} ║", self.total_simulated)?;
        writeln!(f, "║ Başarılı: {:<33} ║", self.successful)?;
        writeln!(f, "║ Başarısız: {:<32} ║", self.failed)?;
        writeln!(f, "║ Toplam Süre: {:<30} ║", format!("{}ms", self.total_duration_ms))?;
        writeln!(f, "║ Dry-Run: {:<34} ║", self.dry_run_count)?;
        writeln!(f, "║ Preview: {:<34} ║", self.preview_count)?;
        writeln!(f, "║ Kayıtlı Aksiyon: {:<25} ║", self.recorded_actions_count)?;
        writeln!(f, "╚════════════════════════════════════════════╝")
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sandbox_mode_type_description() {
        assert_eq!(SandboxModeType::SimulateOnly.description(), "Simülasyon - gerçek işlem yok");
        assert_eq!(SandboxModeType::Normal.description(), "Normal mod - sandbox devre dışı");
    }
    
    #[test]
    fn test_sandbox_mode_type_flags() {
        assert!(SandboxModeType::SimulateOnly.is_sandbox());
        assert!(SandboxModeType::DryRun.is_sandbox());
        assert!(!SandboxModeType::Normal.is_sandbox());
        
        assert!(SandboxModeType::Normal.is_real_execution());
        assert!(!SandboxModeType::SimulateOnly.is_real_execution());
    }
    
    #[test]
    fn test_sandbox_config_default() {
        let config = SandboxConfig::default();
        assert_eq!(config.mode, SandboxModeType::Normal);
        assert_eq!(config.simulation_delay_ms, 100);
    }
    
    #[test]
    fn test_sandbox_config_presets() {
        let simulate = SandboxConfig::simulate_only();
        assert_eq!(simulate.mode, SandboxModeType::SimulateOnly);
        assert!(simulate.verbose_logging);
        
        let dry_run = SandboxConfig::dry_run();
        assert_eq!(dry_run.mode, SandboxModeType::DryRun);
        
        let preview = SandboxConfig::preview();
        assert_eq!(preview.mode, SandboxModeType::Preview);
        assert!(preview.capture_screenshots);
        
        let fake = SandboxConfig::fake_responses();
        assert_eq!(fake.mode, SandboxModeType::FakeResponses);
    }
    
    #[test]
    fn test_simulated_result_is_success() {
        let success = SimulatedResult::Success {
            action: "click".to_string(),
            duration_ms: 100,
            message: "OK".to_string(),
        };
        assert!(success.is_success());
        
        let failure = SimulatedResult::SimulatedFailure {
            action: "click".to_string(),
            error: "Failed".to_string(),
        };
        assert!(!failure.is_success());
    }
    
    #[test]
    fn test_simulated_result_action_name() {
        let result = SimulatedResult::DryRunResult {
            action: "mouse_move".to_string(),
            would_execute: true,
            log: "test".to_string(),
        };
        assert_eq!(result.action_name(), "mouse_move");
    }
    
    #[test]
    fn test_recorded_action_creation() {
        let action = RecordedAction::new("click", r#"{"x": 100, "y": 200}"#);
        assert_eq!(action.action_type, "click");
        assert!(action.was_simulated);
        assert!(!action.id.is_empty());
    }
    
    #[test]
    fn test_sandbox_manager_disabled() {
        let manager = SandboxManager::disabled();
        assert!(!manager.is_active());
        assert_eq!(manager.mode(), SandboxModeType::Normal);
    }
    
    #[test]
    fn test_sandbox_manager_simulate_only() {
        let manager = SandboxManager::simulate_only();
        assert!(manager.is_active());
        assert_eq!(manager.mode(), SandboxModeType::SimulateOnly);
    }
    
    #[test]
    fn test_sandbox_manager_enable_disable() {
        let mut manager = SandboxManager::disabled();
        assert!(!manager.is_active());
        
        manager.enable(SandboxModeType::DryRun);
        assert!(manager.is_active());
        assert_eq!(manager.mode(), SandboxModeType::DryRun);
        
        manager.disable();
        assert!(!manager.is_active());
        assert_eq!(manager.mode(), SandboxModeType::Normal);
    }
    
    #[test]
    fn test_sandbox_manager_set_mode() {
        let mut manager = SandboxManager::disabled();
        
        manager.set_mode(SandboxModeType::Preview);
        assert!(manager.is_active());
        assert_eq!(manager.mode(), SandboxModeType::Preview);
        
        manager.set_mode(SandboxModeType::Normal);
        assert!(!manager.is_active());
    }
    
    #[test]
    fn test_sandbox_manager_process_simulate_only() {
        let mut manager = SandboxManager::simulate_only();
        let result = manager.process_action("click", r#"{"x": 100}"#).unwrap();
        
        assert!(result.is_success());
        assert_eq!(manager.stats().total_simulated, 1);
        assert_eq!(manager.stats().successful_simulations, 1);
    }
    
    #[test]
    fn test_sandbox_manager_process_dry_run() {
        let mut manager = SandboxManager::dry_run();
        let result = manager.process_action("type", r#"{"text": "hello"}"#).unwrap();
        
        assert!(result.is_success());
        assert_eq!(manager.stats().dry_run_count, 1);
    }
    
    #[test]
    fn test_sandbox_manager_process_preview() {
        let mut manager = SandboxManager::preview();
        let result = manager.process_action("screenshot", "{}").unwrap();
        
        assert!(result.is_success());
        assert_eq!(manager.stats().preview_count, 1);
    }
    
    #[test]
    fn test_sandbox_manager_process_fake_response() {
        let mut manager = SandboxManager::fake_responses();
        let result = manager.process_action("click", r#"{"x": 100}"#).unwrap();
        
        assert!(result.is_success());
        match result {
            SimulatedResult::FakeResult { response, .. } => {
                assert!(response.contains("click"));
            }
            _ => panic!("Expected FakeResult"),
        }
    }
    
    #[test]
    fn test_sandbox_manager_process_disabled() {
        let mut manager = SandboxManager::disabled();
        let result = manager.process_action("click", "{}");
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_sandbox_manager_reset_stats() {
        let mut manager = SandboxManager::simulate_only();
        manager.process_action("click", "{}").unwrap();
        
        assert_eq!(manager.stats().total_simulated, 1);
        
        manager.reset_stats();
        assert_eq!(manager.stats().total_simulated, 0);
        assert!(manager.get_simulated_actions().is_empty());
    }
    
    #[test]
    fn test_sandbox_manager_clear_all() {
        let mut manager = SandboxManager::simulate_only();
        manager.process_action("click", "{}").unwrap();
        manager.process_action("type", "{}").unwrap();
        
        assert_eq!(manager.get_recorded_actions().len(), 2);
        
        manager.clear_all();
        assert!(manager.get_recorded_actions().is_empty());
        assert!(manager.get_simulated_actions().is_empty());
    }
    
    #[test]
    fn test_sandbox_report() {
        let manager = SandboxManager::simulate_only();
        let report = manager.report();
        
        assert!(report.is_active);
        assert_eq!(report.mode, SandboxModeType::SimulateOnly);
    }
    
    #[test]
    fn test_sandbox_report_display() {
        let manager = SandboxManager::simulate_only();
        let report = manager.report();
        let output = format!("{}", report);
        
        assert!(output.contains("SANDBOX MODE RAPORU"));
        assert!(output.contains("Aktif:"));
    }
}
