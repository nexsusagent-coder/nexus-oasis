//! ─── A5 SIX OPERATION MODES ───
//!
//! SENTIENT'nın çalışma modları sistemi.
//! Farklı görev tipleri için özelleştirilmiş davranış.
//!
//! Modlar:
//! 1. ReAct - Standart ajan döngüsü
//! 2. Plan - Planlama modu
//! 3. Research - Araştırma modu
//! 4. Development - Geliştirme modu
//! 5. Interactive - İnteraktif mod
//! 6. Autonomous - Otonom mod

pub mod modes;
pub mod transition;
pub mod config;
pub mod mode_ext;

pub use modes::{OperationMode, ModeBehavior, ModeRegistry};
pub use transition::{ModeTransition, TransitionManager, TransitionRule};
pub use config::{ModeConfig, ModeSettings};
pub use mode_ext::{
    CustomMode, CustomModeBehavior, CustomModeBuilder, ErrorBehavior,
    ModeLearningEngine, ModeLearningEntry, ModeLearningStats,
    ModePlugin, ModePluginType, ModePluginManager, HookPoint, PluginResult, PluginStats,
};

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════════════════════
// MODE ERROR
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, thiserror::Error)]
pub enum ModeError {
    #[error("Mod bulunamadı: {0}")]
    NotFound(String),
    
    #[error("Geçersiz geçiş: {0}")]
    InvalidTransition(String),
    
    #[error("Mod aktif değil: {0}")]
    NotActive(String),
}

pub type ModeResult<T> = Result<T, ModeError>;

// ═══════════════════════════════════════════════════════════════════════════════
// MODE ENGINE
// ═══════════════════════════════════════════════════════════════════════════════

/// Mod motoru
pub struct ModeEngine {
    /// Aktif mod
    active_mode: Arc<RwLock<Option<OperationMode>>>,
    /// Mod kayıt defteri
    registry: ModeRegistry,
    /// Geçiş yöneticisi
    transition_manager: TransitionManager,
    /// Mod geçmişi
    history: Arc<RwLock<Vec<ModeTransition>>>,
}

impl ModeEngine {
    pub fn new() -> Self {
        Self {
            active_mode: Arc::new(RwLock::new(None)),
            registry: ModeRegistry::new(),
            transition_manager: TransitionManager::new(),
            history: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Modu başlat
    pub async fn start_mode(&self, mode_type: ModeType) -> ModeResult<OperationMode> {
        let mode = self.registry.get(mode_type)?;
        
        // Geçiş kontrolü
        if let Some(current) = self.active_mode.read().await.as_ref() {
            if !self.transition_manager.can_transition(current.mode_type, mode_type) {
                return Err(ModeError::InvalidTransition(format!(
                    "{:?} -> {:?}", current.mode_type, mode_type
                )));
            }
            
            // Geçşi kaydet
            let transition = ModeTransition {
                id: Uuid::new_v4(),
                from: current.mode_type,
                to: mode_type,
                timestamp: chrono::Utc::now(),
                reason: String::new(),
            };
            self.history.write().await.push(transition);
        }
        
        // Aktif moda ayarla
        *self.active_mode.write().await = Some(mode.clone());
        
        Ok(mode)
    }
    
    /// Aktif modu getir
    pub async fn get_active(&self) -> Option<OperationMode> {
        self.active_mode.read().await.clone()
    }
    
    /// Modu sonlandır
    pub async fn end_mode(&self) -> ModeResult<()> {
        let mut active = self.active_mode.write().await;
        if active.is_none() {
            return Err(ModeError::NotActive("Hiçbir mod aktif değil".into()));
        }
        *active = None;
        Ok(())
    }
    
    /// Mod geçmişini getir
    pub async fn get_history(&self) -> Vec<ModeTransition> {
        self.history.read().await.clone()
    }
    
    /// Önerilen mod
    pub fn suggest_mode(&self, task_description: &str) -> ModeType {
        let desc = task_description.to_lowercase();
        
        if desc.contains("araştır") || desc.contains("research") || desc.contains("bul") {
            ModeType::Research
        } else if desc.contains("kod") || desc.contains("geliştir") || desc.contains("develop") {
            ModeType::Development
        } else if desc.contains("plan") || desc.contains("tasarla") {
            ModeType::Plan
        } else if desc.contains("otonom") || desc.contains("auto") {
            ModeType::Autonomous
        } else if desc.contains("sor") || desc.contains("yardım") {
            ModeType::Interactive
        } else {
            ModeType::ReAct
        }
    }
}

impl Default for ModeEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Mod tipi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModeType {
    ReAct,
    Plan,
    Research,
    Development,
    Interactive,
    Autonomous,
}

impl ModeType {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::ReAct => "ReAct Döngüsü",
            Self::Plan => "Planlama Modu",
            Self::Research => "Araştırma Modu",
            Self::Development => "Geliştirme Modu",
            Self::Interactive => "İnteraktif Mod",
            Self::Autonomous => "Otonom Mod",
        }
    }
    
    pub fn icon(&self) -> &'static str {
        match self {
            Self::ReAct => "🔄",
            Self::Plan => "📋",
            Self::Research => "🔍",
            Self::Development => "💻",
            Self::Interactive => "💬",
            Self::Autonomous => "🤖",
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_mode_start() {
        let engine = ModeEngine::new();
        let mode = engine.start_mode(ModeType::ReAct).await.ok();
        
        assert!(mode.is_some());
    }
    
    #[tokio::test]
    async fn test_mode_suggest() {
        let engine = ModeEngine::new();
        
        assert_eq!(engine.suggest_mode("veritabanı araştır"), ModeType::Research);
        assert_eq!(engine.suggest_mode("kod yaz"), ModeType::Development);
        assert_eq!(engine.suggest_mode("plan yap"), ModeType::Plan);
    }
    
    #[tokio::test]
    async fn test_mode_history() {
        let engine = ModeEngine::new();
        
        engine.start_mode(ModeType::ReAct).await.ok();
        engine.start_mode(ModeType::Development).await.ok();
        
        let history = engine.get_history().await;
        assert!(history.len() >= 1); // En az bir geçiş
    }
}
