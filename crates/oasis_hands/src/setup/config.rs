//! Setup Config - Yapılandırma

use serde::{Deserialize, Serialize};
use crate::setup::config_path;
use super::permissions::Permission;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupConfig {
    pub version: String,
    pub profile_name: String,
    pub platform: Platform,
    pub state: SetupState,
    pub permissions: PermissionsConfig,
    pub security: SecurityConfig,
    pub human: HumanConfig,
    pub forbidden: ForbiddenConfig,
    pub approval: ApprovalConfig,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform { Linux, Windows, MacOS, Unknown }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SetupState { NotSetup, Configured, Active, Paused, Error }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionsConfig {
    pub screen_capture: bool,
    pub mouse_control: bool,
    pub keyboard_control: bool,
    pub window_management: bool,
    pub file_access: bool,
    pub process_spawn: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub mode: SecurityMode,
    pub require_approval: bool,
    pub max_actions: u32,
    pub emergency_stop: bool,
    pub timeout_secs: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityMode { Strict, Normal, Relaxed }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanConfig {
    // --- Mouse Ayarları ---
    pub mouse: bool,
    pub mouse_pattern: String,           // linear, curved, wavy, natural, adaptive
    pub bezier_quality: u32,             // 20-100 nokta
    pub tremor: bool,
    pub tremor_intensity: f64,          // 0.0-0.20
    
    // --- Klavye Ayarları ---
    pub typing: bool,
    pub typing_profile: String,         // beginner, intermediate, expert, custom, robot
    pub wpm: u32,
    pub errors: bool,
    pub error_rate: f64,                // 0.0-0.20
    
    // --- Davranış Modeli ---
    pub use_rnn_model: bool,
    pub best_of_n: u32,                 // 1-10
    pub exploration_rate: f64,          // 0.0-0.30
    
    // --- Dikkat ve Yorgunluk ---
    pub simulate_attention: bool,
    pub attention_span_sec: u32,        // 30-300 saniye
    pub distraction_rate: f64,          // 0.0-0.30
    pub simulate_fatigue: bool,
    pub fatigue_rate: f64,              // 0.0-0.50 /saat
    
    // --- Karar Verme ---
    pub decision_delay_min_ms: u32,     // 50-500 ms
    pub decision_delay_max_ms: u32,     // 100-2000 ms
    pub hesitation_rate: f64,           // 0.0-0.20
    
    // --- Fiziksel Özellikler ---
    pub hand_preference: String,        // right, left, both
    
    // --- Genel ---
    pub level: f64,                     // 0.0-1.0 (insan benzerliği)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForbiddenConfig {
    pub regions: Vec<Rect>,
    pub apps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rect { pub x: u32, pub y: u32, pub width: u32, pub height: u32 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalConfig {
    pub remember: bool,
    pub timeout: u64,
    pub auto_safe: bool,
}

impl Default for SetupConfig {
    fn default() -> Self {
        Self {
            version: "1.0".into(),
            profile_name: "default".into(),
            platform: Platform::Unknown,
            state: SetupState::NotSetup,
            permissions: PermissionsConfig {
                screen_capture: true, mouse_control: true,
                keyboard_control: true, window_management: true,
                file_access: false, process_spawn: false,
            },
            security: SecurityConfig {
                mode: SecurityMode::Normal, require_approval: true,
                max_actions: 60, emergency_stop: true, timeout_secs: 300,
            },
            human: HumanConfig {
                mouse: true,
                mouse_pattern: "natural".into(),
                bezier_quality: 50,
                tremor: true,
                tremor_intensity: 0.05,
                typing: true,
                typing_profile: "intermediate".into(),
                wpm: 45,
                errors: false,
                error_rate: 0.03,
                use_rnn_model: true,
                best_of_n: 5,
                exploration_rate: 0.10,
                simulate_attention: true,
                attention_span_sec: 120,
                distraction_rate: 0.05,
                simulate_fatigue: true,
                fatigue_rate: 0.10,
                decision_delay_min_ms: 100,
                decision_delay_max_ms: 500,
                hesitation_rate: 0.05,
                hand_preference: "right".into(),
                level: 0.85,
            },
            forbidden: ForbiddenConfig { regions: vec![], apps: vec![] },
            approval: ApprovalConfig { remember: true, timeout: 30, auto_safe: true },
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}

impl SetupConfig {
    pub fn with_platform(mut self, p: Platform) -> Self { self.platform = p; self }
    
    pub fn with_permissions(mut self, perms: Vec<Permission>) -> Self {
        for p in perms {
            match p {
                Permission::ScreenCapture => self.permissions.screen_capture = true,
                Permission::MouseControl => self.permissions.mouse_control = true,
                Permission::KeyboardControl => self.permissions.keyboard_control = true,
                Permission::WindowManagement => self.permissions.window_management = true,
                Permission::FileAccess => self.permissions.file_access = true,
                Permission::ProcessSpawn => self.permissions.process_spawn = true,
            }
        }
        self
    }
    
    pub fn with_security_mode(mut self, mode: SecurityMode) -> Self { self.security.mode = mode; self }
    pub fn with_strict_security(mut self) -> Self {
        self.security.mode = SecurityMode::Strict;
        self.security.require_approval = true;
        self
    }
    
    pub fn with_approval_settings(mut self, req: bool, rem: bool, timeout: u64, auto: bool) -> Self {
        self.security.require_approval = req;
        self.approval.remember = rem;
        self.approval.timeout = timeout;
        self.approval.auto_safe = auto;
        self
    }
    
    pub fn with_human_settings(
        mut self, 
        mouse: bool, mouse_pattern: &str, bezier_quality: u32, tremor: bool, tremor_intensity: f64,
        typing: bool, typing_profile: &str, wpm: u32, errors: bool, error_rate: f64,
        use_rnn_model: bool, best_of_n: u32, exploration_rate: f64,
        simulate_attention: bool, attention_span_sec: u32, distraction_rate: f64,
        simulate_fatigue: bool, fatigue_rate: f64,
        decision_delay_min_ms: u32, decision_delay_max_ms: u32, hesitation_rate: f64,
        hand_preference: &str,
        level: f64
    ) -> Self {
        self.human.mouse = mouse;
        self.human.mouse_pattern = mouse_pattern.into();
        self.human.bezier_quality = bezier_quality;
        self.human.tremor = tremor;
        self.human.tremor_intensity = tremor_intensity;
        self.human.typing = typing;
        self.human.typing_profile = typing_profile.into();
        self.human.wpm = wpm;
        self.human.errors = errors;
        self.human.error_rate = error_rate;
        self.human.use_rnn_model = use_rnn_model;
        self.human.best_of_n = best_of_n;
        self.human.exploration_rate = exploration_rate;
        self.human.simulate_attention = simulate_attention;
        self.human.attention_span_sec = attention_span_sec;
        self.human.distraction_rate = distraction_rate;
        self.human.simulate_fatigue = simulate_fatigue;
        self.human.fatigue_rate = fatigue_rate;
        self.human.decision_delay_min_ms = decision_delay_min_ms;
        self.human.decision_delay_max_ms = decision_delay_max_ms;
        self.human.hesitation_rate = hesitation_rate;
        self.human.hand_preference = hand_preference.into();
        self.human.level = level;
        self
    }
    
    pub fn with_forbidden_areas(mut self, regions: Vec<Rect>, apps: Vec<String>) -> Self {
        self.forbidden.regions = regions;
        self.forbidden.apps = apps;
        self
    }
    
    pub fn with_emergency_stop(mut self, enabled: bool) -> Self { self.security.emergency_stop = enabled; self }
    pub fn with_max_actions(mut self, max: u64) -> Self { self.security.max_actions = max as u32; self }
    
    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = config_path();
        if let Some(p) = path.parent() { std::fs::create_dir_all(p)?; }
        let s = toml::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write(&path, s)
    }
    
    pub fn load() -> Result<Self, std::io::Error> {
        let path = config_path();
        let s = std::fs::read_to_string(&path)?;
        toml::from_str(&s).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }
}
