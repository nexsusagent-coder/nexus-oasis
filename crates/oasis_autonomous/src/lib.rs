//! ═══════════════════════════════════════════════════════════════════════════════
//!  OASIS AUTONOMOUS - Fully Autonomous Desktop Agent
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! İnsan gibi bilgisayar kullanabilen tam otonom agent sistemi.
//!
//! MODÜLLER:
//! ─────────
//! 1. agent_loop       - Desktop Agent Loop (Perception → Decision → Action)
//! 2. screen           - Screen Understanding (Ekran anlama)

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
//! 3. safety           - Safety System (Güvenlik katmanı)
//! 4. planner          - Task Planner (Görev planlama)
//! 5. vision           - Enhanced Vision (Gelişmiş görü)
//! 6. memory           - Advanced Memory (Öğrenen bellek)
//! 7. tools            - Tool Chaining (Araç zincirleri)
//! 8. orchestrator     - Multi-Agent Orchestrator
//! 9. healing          - Self-Healing System
//!
//! SİSTEM MİMARİSİ:
//! ─────────────────
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                         ORCHESTRATOR                                    │
//! │  ┌─────────────────────────────────────────────────────────────────┐   │
//! │  │                    AGENT LOOP                                    │   │
//! │  │  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐    │   │
//! │  │  │ PERCEIVE │ → │  DECIDE  │ → │   ACT    │ → │  LEARN   │    │   │
//! │  │  └────┬─────┘   └────┬─────┘   └────┬─────┘   └────┬─────┘    │   │
//! │  │       │              │              │              │          │   │
//! │  │  ┌────▼─────┐   ┌────▼─────┐   ┌────▼─────┐   ┌────▼─────┐    │   │
//! │  │  │  SCREEN  │   │ PLANNER  │   │  TOOLS   │   │  MEMORY  │    │   │
//! │  │  │  VISION  │   │  SAFETY  │   │ CHAINING │   │ HEALING  │    │   │
//! │  │  └──────────┘   └──────────┘   └──────────┘   └──────────┘    │   │
//! │  └─────────────────────────────────────────────────────────────────┘   │
//! └─────────────────────────────────────────────────────────────────────────┘
//!
//! SOVEREIGN CONSTITUTION L1:
//! ───────────────────────────
//! - Tüm aksiyonlar Safety System'den geçer
//! - İnsan onayı kritik aksiyonlar için zorunlu
//! - Self-healing ile otomatik kurtarma
//! - Memory ile öğrenme ve iyileştirme

pub mod agent_loop;
pub mod screen;
pub mod safety;
pub mod planner;
pub mod vision;
pub mod memory;
pub mod tools;
pub mod orchestrator;
pub mod healing;
pub mod error;
pub mod voice_control;

// ═══════════════════════════════════════════════════════════════════════════════
//  RE-EXPORTS
// ═══════════════════════════════════════════════════════════════════════════════

pub use agent_loop::{AutonomousAgent, AgentState, AgentConfig};
pub use screen::{ScreenUnderstanding, ScreenRegion, WindowInfo};
pub use safety::{SafetySystem, SafetyConfig, StopCondition};
pub use planner::{TaskPlanner, Task, TaskStep, Target};
pub use vision::{EnhancedVision, VisionResult, UIElement, Observation};
pub use memory::{AdvancedMemory, MemoryType, Episode};
pub use tools::{ToolChain, ChainStep, ChainResult};
pub use orchestrator::{MultiAgentOrchestrator, AgentMessage};
pub use healing::{SelfHealing, HealthStatus, RecoveryAction};
pub use voice_control::{
    VoiceControlEngine, VoiceControlConfig, VoiceControlState,
    VoiceCommand, CommandResult, CommandParser, ScrollDirection,
};

pub use error::{AutonomousError, AutonomousResult};

// ═══════════════════════════════════════════════════════════════════════════════
//  CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Maksimum iterasyon sayısı
pub const MAX_ITERATIONS: usize = 100;

/// Varsayılan timeout (saniye)
pub const DEFAULT_TIMEOUT_SECS: u64 = 300;

/// Maksimum action kaydı
pub const MAX_ACTION_HISTORY: usize = 1000;

/// Token limiti (LLM için)
pub const MAX_CONTEXT_TOKENS: usize = 16000;

/// Minimum güven skor
pub const MIN_CONFIDENCE: f32 = 0.7;

/// Human approval threshold
pub const HUMAN_APPROVAL_THRESHOLD: f32 = 0.9;

// ═══════════════════════════════════════════════════════════════════════════════
//  CORE TYPES
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Agent kimliği
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(pub String);

impl Default for AgentId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl std::fmt::Display for AgentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Aksiyon tipi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    /// Mouse hareketi
    MouseMove { x: i32, y: i32 },
    /// Mouse tıklama
    MouseClick { button: MouseButton, x: i32, y: i32 },
    /// Mouse sürükleme
    MouseDrag { from: (i32, i32), to: (i32, i32) },
    /// Mouse scroll
    MouseScroll { delta_x: i32, delta_y: i32 },
    
    /// Klavye tuş basma
    KeyPress { key: Key },
    /// Klavye kısayol
    KeyShortcut { modifiers: Vec<Key>, key: Key },
    /// Metin yazma
    TypeText { text: String, human_like: bool },
    
    /// Browser aksiyonu
    BrowserNavigate { url: String },
    BrowserClick { selector: String },
    BrowserType { selector: String, text: String },
    
    /// Kompozit aksiyon
    Composite { actions: Vec<Action> },
    
    /// Özel aksiyon
    Custom { name: String, params: HashMap<String, serde_json::Value> },
    
    /// Hiçbir şey
    NoOp,
    
    /// Durdur
    Stop { reason: String },
}

/// Mouse butonları
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

impl Default for MouseButton {
    fn default() -> Self {
        MouseButton::Left
    }
}

/// Klavye tuşları
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Key {
    Char(char),
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    ArrowUp, ArrowDown, ArrowLeft, ArrowRight,
    Escape, Enter, Tab, Backspace, Delete, Insert,
    Home, End, PageUp, PageDown,
    Shift, Ctrl, Alt, Super,
    Space,
}

/// Aksiyon sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    /// Başarılı mı?
    pub success: bool,
    /// Mesaj
    pub message: String,
    /// Güven skoru
    pub confidence: f32,
    /// Ek veri
    pub data: Option<serde_json::Value>,
    /// Süre (ms)
    pub duration_ms: u64,
}

impl ActionResult {
    pub fn success(message: &str) -> Self {
        Self {
            success: true,
            message: message.into(),
            confidence: 1.0,
            data: None,
            duration_ms: 0,
        }
    }
    
    pub fn failure(message: &str) -> Self {
        Self {
            success: false,
            message: message.into(),
            confidence: 0.0,
            data: None,
            duration_ms: 0,
        }
    }
    
    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence;
        self
    }
    
    pub fn with_data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
}

/// Görev sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    /// Görev ID
    pub task_id: String,
    /// Başarılı mı?
    pub success: bool,
    /// Mesaj
    pub message: String,
    /// Toplam süre
    pub total_duration_ms: u64,
    /// Aksiyon sayısı
    pub action_count: usize,
    /// Hata sayısı
    pub error_count: usize,
    /// Sonuç verisi
    pub result_data: Option<serde_json::Value>,
}

impl TaskResult {
    pub fn success(task_id: &str) -> Self {
        Self {
            task_id: task_id.into(),
            success: true,
            message: "Görev başarıyla tamamlandı".into(),
            total_duration_ms: 0,
            action_count: 0,
            error_count: 0,
            result_data: None,
        }
    }
    
    pub fn failure(task_id: &str, reason: &str) -> Self {
        Self {
            task_id: task_id.into(),
            success: false,
            message: reason.into(),
            total_duration_ms: 0,
            action_count: 0,
            error_count: 0,
            result_data: None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_agent_id_creation() {
        let id = AgentId::default();
        assert!(!id.0.is_empty());
    }
    
    #[test]
    fn test_action_result_success() {
        let result = ActionResult::success("Test");
        assert!(result.success);
        assert_eq!(result.confidence, 1.0);
    }
    
    #[test]
    fn test_action_result_failure() {
        let result = ActionResult::failure("Test failed");
        assert!(!result.success);
        assert_eq!(result.confidence, 0.0);
    }
    
    #[test]
    fn test_observation_default() {
        let obs = Observation::default();
        assert!(!obs.id.is_empty());
        assert_eq!(obs.screen_size, (1920, 1080));
    }
}
