//! ═══════════════════════════════════════════════════════════════════════════════
//!  OASIS HANDS - L6: EXECUTION KATMANI
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Agent-S3 aracının SENTIENT'ya tam asimilasyonu.
//! Masaüstü GUI kontrolü, fare/klavye otomasyonu, ekran analizi.
//! 
//! ═──────────────────────────────────────────────────────────────────────────────
//!  L1 SOVEREIGN ANAYASASI:
//!  ───────────────────────

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(private_interfaces)]
//!  ✓ GUI kontrolü ANCAK izin verilen uygulamalarla
//!  ✓ Dosya sistemi ERİŞİMİ KISITLANMIŞ (whitelist dizinler)
//!  ✓ Process başlatma WHITELIST ile kontrol edilir
//!  ✓ Tehlikeli komutlar ENGELLENİR (rm -rf, format, dd, etc.)
//!  ✓ Tüm aksiyonlar V-GATE üzerinden loglanır
//!  ✓ Tüm hatalar SENTIENT diline çevrilir
//! ═──────────────────────────────────────────────────────────────────────────────
//!
//! MİMARİ:
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │                         OASIS HANDS                                          │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │                                                                             │
//! │  ┌─────────────────────────────────────────────────────────────────────┐   │
//! │  │                    SOVEREIGN POLICY (L1)                             │   │
//! │  │  ┌───────────────┐  ┌───────────────┐  ┌───────────────┐           │   │
//! │  │  │ FileSystem    │  │ Process       │  │ Network       │           │   │
//! │  │  │   Whitelist   │  │   Whitelist   │  │   Restricted  │           │   │
//! │  │  └───────────────┘  └───────────────┘  └───────────────┘           │   │
//! │  └─────────────────────────────────────────────────────────────────────┘   │
//! │                                    │                                        │
//! │                                    ▼                                        │
//! │  ┌─────────────────────────────────────────────────────────────────────┐   │
//! │  │                    DESKTOP AGENT                                    │   │
//! │  │  ┌───────────┐  ┌───────────┐  ┌───────────┐  ┌───────────┐        │   │
//! │  │  │ Screen    │  │ Mouse     │  │ Keyboard  │  │ OCR/Vision│        │   │
//! │  │  │ Capture   │  │ Control   │  │ Control   │  │ Analysis  │        │   │
//! │  │  └───────────┘  └───────────┘  └───────────┘  └───────────┘        │   │
//! │  └─────────────────────────────────────────────────────────────────────┘   │
//! │                                    │                                        │
//! │                                    ▼                                        │
//! │  ┌─────────────────────────────────────────────────────────────────────┐   │
//! │  │                    V-GATE (L2)                                       │   │
//! │  │  Action Request → Guardrails → Audit Log → LLM Decision             │   │
//! │  └─────────────────────────────────────────────────────────────────────┘   │
//! │                                                                             │
//! └─────────────────────────────────────────────────────────────────────────────┘
//! ```

pub mod error;
pub mod sovereign;
pub mod screen;
pub mod input;
pub mod vision;
pub mod agent;
pub mod vgate;
pub mod session;
pub mod tools;
pub mod sentient_tool;
pub mod sentient_tools;
pub mod executor;
pub mod skill_loader;
pub mod emergency;
pub mod rate_limiter;
pub mod time_rules;
pub mod sandbox;
pub mod alert;
pub mod history;
pub mod recorder;

// Human Mimicry Modülü (Bumblebee + typerr asimilasyonu)
pub mod human_mimicry;

// Setup Wizard Modülü
pub mod setup;

// Re-exports
pub use error::{HandsError, HandsResult};
pub use sovereign::{SovereignPolicy, FileAccessPolicy, ProcessPolicy, NetworkPolicy, ForbiddenRegion, ForbiddenRegionType};
pub use screen::{ScreenCapture, ScreenInfo, CaptureConfig};
pub use input::{InputController, MouseAction, KeyboardAction, MouseButton};
pub use vision::{VisionEngine, VisionResult, UIElement};
pub use agent::{DesktopAgent, AgentConfig, AgentState, DesktopTask};
pub use vgate::HandsVGate;
pub use session::{HandsSession, SessionConfig, SessionStats};
pub use tools::{ToolRegistry, ToolResult};
pub use skill_loader::{SkillLoader, LoadedSkill, SkillMetadata, SkillExecutionContext};
pub use emergency::{EmergencyStop, EmergencyManager, Hotkey, EmergencyLevel};
pub use rate_limiter::{RateLimiter, RateLimitConfig, RateLimitReport, MouseRateLimits, KeyboardRateLimits, GeneralRateLimits};
pub use time_rules::{TimeRules, TimeRulesManager, TimeRulesReport, WorkMode, BlockedPeriod};
pub use sandbox::{SandboxManager, SandboxConfig, SandboxModeType, SandboxReport, SimulatedResult, RecordedAction};
pub use alert::{AlertSystem, AlertConfig, AlertLevel, AlertStats, AlertRecord, AlertChannel, ViolationType};
pub use history::{ActionHistory, HistoryConfig, HistoryStats, HistoryReport, HistoricalAction, StateSnapshot, UndoableActionType, UndoRedoResult, UndoRedoOperation, HistoryBranch};
pub use recorder::{MacroRecorder, RecorderConfig, RecorderStats, RecorderReport, Macro, MacroSummary, TimedAction, PlaybackSettings, PlaybackResult, RecordingState, StopReason};

// Sentient Tools Re-exports
pub use sentient_tool::{SentientTool, SentientToolResult, ToolCategory, RiskLevel, ToolParameter};
pub use sentient_tools::{
    GitTool, GrepTool, SedTool, BrowserTool, ScreenshotTool,
    McpTool, MemoryTool, TaskTool, N8nTool, EmailTool,
    NotifyTool, PdfTool, TranslateTool, CalendarTool, AgentTool,
};
pub use executor::{SENTIENTToolExecutor, ToolInfo};

// Human Mimicry Re-Exports
pub use human_mimicry::{
    HumanMimicryEngine, HumanMimicryConfig,
    BezierCurve, BezierPoint, CubicBezier,
    TypingDynamics, TypingProfile, KeyDistance,
    MouseDynamics, MouseProfile, TremorConfig,
    BehaviorModel, BehaviorProfile, ActionType,
    BumblebeeEngine, BumblebeeConfig, MovementPattern,
};

// Public exports

/// Agent-S3 asimilasyon sürümü
pub const OASIS_HANDS_VERSION: &str = "0.1.0-sentient";

/// Maksimum aksiyon süresi (saniye)
pub const MAX_ACTION_DURATION_SECS: u64 = 300; // 5 dakika

/// Maksimum ekran boyutu
pub const MAX_SCREEN_WIDTH: u32 = 3840;
pub const MAX_SCREEN_HEIGHT: u32 = 2160;

/// Varsayılan fare hızı (px/ms)
pub const DEFAULT_MOUSE_SPEED: f32 = 1000.0;

/// ─── İZİN VERİLEN UYGULAMALAR (WHITELIST) ───
pub const ALLOWED_APPS: &[&str] = &[
    // Ofis uygulamaları
    "libreoffice",
    "soffice",
    "calc",
    "writer",
    "impress",
    
    // Tarayıcılar (kontrollü)
    "firefox",
    "chromium",
    "chrome",
    
    // Terminal (kısıtlı)
    "gnome-terminal",
    "xterm",
    "konsole",
    
    // Dosya yöneticisi (salt okunur)
    "nautilus",
    "dolphin",
    "thunar",
    
    // Metin editörleri
    "gedit",
    "kate",
    "code",  // VS Code
    "nano",
    "vim",
];

/// ─── YASAKLI KOMUTLAR (BLACKLIST) ───
pub const BLOCKED_COMMANDS: &[&str] = &[
    // Dosya sistemi tehlikeleri
    "rm -rf",
    "rm -r /",
    "rm -rf /",
    "format",
    "mkfs",
    "dd if=",
    "shred",
    "wipe",
    
    // Sistem tehlikeleri
    "init 0",
    "shutdown",
    "reboot",
    "poweroff",
    "halt",
    
    // Ağ tehlikeleri
    "iptables -F",
    "ip route del",
    "ifconfig down",
    
    // Kullanıcı tehlikeleri
    "userdel",
    "passwd",
    "chmod 777 /",
    "chown -R",
    
    // Süreç tehlikeleri
    "killall",
    "pkill -9",
    "kill -9 1",
];

/// ─── İZİN VERİLEN DİZİNLER ───
pub const ALLOWED_PATHS: &[&str] = &[
    "/home/sentient/workspace",
    "/home/sentient/documents",
    "/home/sentient/downloads",
    "/tmp/sentient",
    "/var/log/sentient",
];

/// ─── YASAKLI DİZİNLER ───
pub const BLOCKED_PATHS: &[&str] = &[
    "/etc/shadow",
    "/etc/passwd",
    "/etc/sudoers",
    "/root",
    "/var/run",
    "/proc",
    "/sys",
    "/dev",
    "/boot",
    "/usr/bin",
    "/usr/sbin",
    "/bin",
    "/sbin",
];

// ───────────────────────────────────────────────────────────────────────────────
//  OASIS HANDS MANAGER
// ─────────────────────────────────────────────────────────────────────────────--

/// Oasis Hands yöneticisi - Ana giriş noktası
pub struct OasisHands {
    /// Sovereign policy
    policy: sovereign::SovereignPolicy,
    /// Desktop agent
    agent: Option<agent::DesktopAgent>,
    /// V-GATE köprüsü
    vgate: vgate::HandsVGate,
    /// Yapılandırma
    config: HandsConfig,
    /// Başlatıldı mı?
    initialized: bool,
    /// Aktif oturum
    session: Option<session::HandsSession>,
}

/// Hands yapılandırması
#[derive(Debug, Clone)]
pub struct HandsConfig {
    /// V-GATE URL
    pub vgate_url: String,
    /// Ekran çözünürlüğü (opsiyonel)
    pub screen_resolution: Option<(u32, u32)>,
    /// Mouse hızı
    pub mouse_speed: f32,
    /// OCR aktif mi?
    pub ocr_enabled: bool,
    /// Güvenlik modu
    pub security_mode: SecurityMode,
    /// Maksimum aksiyon süresi (saniye)
    pub max_action_duration: u64,
    /// Whitelist modu aktif mi?
    pub whitelist_mode: bool,
    /// Otomatik onay gerekli mi?
    pub require_confirmation: bool,
}

/// Güvenlik modları
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SecurityMode {
    /// En katı - her aksiyon onay gerekir
    Strict,
    /// Normal - tehlikeli aksiyonlar onay gerekir
    Normal,
    /// Rahat - sadece yasaklı komutlar engellenir
    Relaxed,
}

impl Default for HandsConfig {
    fn default() -> Self {
        Self {
            vgate_url: "http://127.0.0.1:1071".into(),
            screen_resolution: None,
            mouse_speed: DEFAULT_MOUSE_SPEED,
            ocr_enabled: true,
            security_mode: SecurityMode::Normal,
            max_action_duration: MAX_ACTION_DURATION_SECS,
            whitelist_mode: true,
            require_confirmation: true,
        }
    }
}

impl OasisHands {
    /// Yeni Oasis Hands oluştur
    pub fn new(config: HandsConfig) -> Self {
        log::info!("🖐️  OASIS-HANDS: L6 EXECUTION katmanı başlatılıyor...");
        
        let policy = sovereign::SovereignPolicy::strict();
        let vgate = vgate::HandsVGate::new(&config.vgate_url);
        
        log::info!("🖐️  OASIS-HANDS: Sovereign Policy aktif - Güvenlik modu: {:?}", config.security_mode);
        
        Self {
            policy,
            agent: None,
            vgate,
            config,
            initialized: false,
            session: None,
        }
    }
    
    /// Masaüstü kontrolünü başlat
    pub async fn initialize(&mut self) -> HandsResult<()> {
        if self.initialized {
            return Ok(());
        }
        
        log::info!("🖐️  OASIS-HANDS: Masaüstü kontrol sistemi başlatılıyor...");
        
        // Policy'yi aktive et
        self.policy.activate()?;
        
        // Desktop agent oluştur
        let agent_config = agent::AgentConfig {
            mouse_speed: self.config.mouse_speed,
            ocr_enabled: self.config.ocr_enabled,
            security_mode: self.config.security_mode,
            require_confirmation: self.config.require_confirmation,
            max_action_duration: self.config.max_action_duration,
            max_iterations: 50,
            target_resolution: self.config.screen_resolution,
        };
        
        self.agent = Some(agent::DesktopAgent::new(agent_config, self.policy.clone()).await?);
        
        // Yeni oturum başlat
        self.session = Some(session::HandsSession::new());
        
        self.initialized = true;
        
        log::info!("✅  OASIS-HANDS: Hazır - Masaüstü kontrolü aktif");
        log::info!("🖐️  OASIS-HANDS: İzin verilen uygulamalar: {} adet", ALLOWED_APPS.len());
        log::info!("🖐️  OASIS-HANDS: Yasaklı komutlar: {} adet", BLOCKED_COMMANDS.len());
        
        Ok(())
    }
    
    /// Görev çalıştır (ana giriş noktası)
    pub async fn execute_task(&mut self, task: &str) -> HandsResult<String> {
        self.ensure_initialized()?;
        
        log::info!("🖐️  OASIS-HANDS: Görev alındı → {}", task.chars().take(80).collect::<String>());
        
        // V-GATE üzerinden yetkilendirme al
        self.vgate.authorize_action(task).await?;
        
        let agent = self.agent.as_mut().ok_or_else(|| {
            HandsError::NotInitialized("Agent başlatılmadı".into())
        })?;
        
        // Task'ı çalıştır
        let result = agent.execute_task(task).await?;
        
        // Session'ı güncelle
        if let Some(ref mut session) = self.session {
            session.record_action(task, &result);
        }
        
        Ok(result)
    }
    
    /// Ekran görüntüsü al
    pub async fn capture_screen(&mut self) -> HandsResult<screen::ScreenCapture> {
        self.ensure_initialized()?;
        
        let agent = self.agent.as_mut().ok_or_else(|| {
            HandsError::NotInitialized("Agent başlatılmadı".into())
        })?;
        
        agent.capture_screen().await
    }
    
    /// UI element bul
    pub async fn find_element(&mut self, description: &str) -> HandsResult<Option<vision::UIElement>> {
        self.ensure_initialized()?;
        
        let agent = self.agent.as_mut().ok_or_else(|| {
            HandsError::NotInitialized("Agent başlatılmadı".into())
        })?;
        
        agent.find_element(description).await
    }
    
    /// Fare aksiyonu çalıştır
    pub async fn mouse_action(&mut self, action: input::MouseAction) -> HandsResult<()> {
        self.ensure_initialized()?;
        
        // Sovereign kontrol
        self.policy.validate_mouse_action(&action)?;
        
        let agent = self.agent.as_mut().ok_or_else(|| {
            HandsError::NotInitialized("Agent başlatılmadı".into())
        })?;
        
        agent.mouse_action(action).await
    }
    
    /// Klavye aksiyonu çalıştır
    pub async fn keyboard_action(&mut self, action: input::KeyboardAction) -> HandsResult<()> {
        self.ensure_initialized()?;
        
        // Sovereign kontrol
        self.policy.validate_keyboard_action(&action)?;
        
        let agent = self.agent.as_mut().ok_or_else(|| {
            HandsError::NotInitialized("Agent başlatılmadı".into())
        })?;
        
        agent.keyboard_action(action).await
    }
    
    /// Dosya oku (kısıtlı)
    pub async fn read_file(&mut self, path: &str) -> HandsResult<String> {
        self.ensure_initialized()?;
        
        // Sovereign kontrol - dosya erişimi kısıtlı
        self.policy.validate_file_access(path, false)?;
        
        let agent = self.agent.as_mut().ok_or_else(|| {
            HandsError::NotInitialized("Agent başlatılmadı".into())
        })?;
        
        agent.read_file(path).await
    }
    
    /// Dosya yaz (kısıtlı)
    pub async fn write_file(&mut self, path: &str, content: &str) -> HandsResult<()> {
        self.ensure_initialized()?;
        
        // Sovereign kontrol - yazma daha katı
        self.policy.validate_file_access(path, true)?;
        
        let agent = self.agent.as_mut().ok_or_else(|| {
            HandsError::NotInitialized("Agent başlatılmadı".into())
        })?;
        
        agent.write_file(path, content).await
    }
    
    /// Komut çalıştır (whitelist kontrolü)
    pub async fn execute_command(&mut self, command: &str) -> HandsResult<String> {
        self.ensure_initialized()?;
        
        // Sovereign kontrol - tehlikeli komutlar engellenir
        self.policy.validate_command(command)?;
        
        let agent = self.agent.as_mut().ok_or_else(|| {
            HandsError::NotInitialized("Agent başlatılmadı".into())
        })?;
        
        agent.execute_command(command).await
    }
    
    /// Masaüstü kontrolünü kapat
    pub async fn close(&mut self) -> HandsResult<()> {
        if let Some(ref mut agent) = self.agent {
            agent.close().await?;
        }
        
        self.policy.deactivate()?;
        self.initialized = false;
        
        if let Some(ref session) = self.session {
            log::info!("🖐️  OASIS-HANDS: Oturum kapatıldı - {} aksiyon gerçekleştirildi", session.action_count());
        }
        
        log::info!("🖐️  OASIS-HANDS: Masaüstü kontrolü kapatıldı");
        Ok(())
    }
    
    /// Oturum istatistikleri
    pub fn stats(&self) -> Option<&session::HandsSession> {
        self.session.as_ref()
    }
    
    /// Başlatıldı mı?
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    /// Politikayı getir
    pub fn policy(&self) -> &sovereign::SovereignPolicy {
        &self.policy
    }
    
    // ─── Yardımcı Metodlar ───
    
    fn ensure_initialized(&self) -> HandsResult<()> {
        if !self.initialized {
            Err(HandsError::NotInitialized(
                "Oasis Hands başlatılmadı. Önce initialize() çağırın.".into()
            ))
        } else {
            Ok(())
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ─────────────────────────────────────────────────────────────────────────────--

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_default() {
        let config = HandsConfig::default();
        assert!(config.ocr_enabled);
        assert!(config.whitelist_mode);
        assert_eq!(config.security_mode, SecurityMode::Normal);
    }
    
    #[test]
    fn test_policy_creation() {
        let policy = SovereignPolicy::strict();
        assert!(policy.is_active() == false); // Henüz aktive edilmedi
    }
    
    #[test]
    fn test_oasis_hands_creation() {
        let hands = OasisHands::new(HandsConfig::default());
        assert!(!hands.is_initialized());
    }
    
    #[test]
    fn test_blocked_commands() {
        assert!(BLOCKED_COMMANDS.contains(&"rm -rf"));
        assert!(BLOCKED_COMMANDS.contains(&"shutdown"));
        assert!(BLOCKED_COMMANDS.contains(&"dd if="));
    }
    
    #[test]
    fn test_allowed_apps() {
        assert!(ALLOWED_APPS.contains(&"firefox"));
        assert!(ALLOWED_APPS.contains(&"libreoffice"));
        assert!(ALLOWED_APPS.contains(&"gedit"));
    }
    
    #[test]
    fn test_blocked_paths() {
        assert!(BLOCKED_PATHS.contains(&"/etc/shadow"));
        assert!(BLOCKED_PATHS.contains(&"/root"));
        assert!(BLOCKED_PATHS.contains(&"/proc"));
    }
    
    #[test]
    fn test_allowed_paths() {
        assert!(ALLOWED_PATHS.iter().any(|p| p.contains("sentient")));
    }
    
    #[test]
    fn test_security_mode_values() {
        assert_eq!(SecurityMode::Strict as i32, 0);
        assert_eq!(SecurityMode::Normal as i32, 1);
        assert_eq!(SecurityMode::Relaxed as i32, 2);
    }
}
