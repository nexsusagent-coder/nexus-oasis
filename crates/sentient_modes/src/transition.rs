//! ─── MODE TRANSITION ───
//!
//! Mod geçiş kuralları

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::ModeType;

/// Mod geçişi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeTransition {
    pub id: Uuid,
    pub from: ModeType,
    pub to: ModeType,
    pub timestamp: DateTime<Utc>,
    pub reason: String,
}

/// Geçiş kuralı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionRule {
    pub from: ModeType,
    pub to: ModeType,
    pub condition: Option<String>,
    pub auto_trigger: bool,
}

/// Geçiş yöneticisi
#[derive(Debug, Clone, Default)]
pub struct TransitionManager {
    rules: HashMap<ModeType, Vec<TransitionRule>>,
}

impl TransitionManager {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }
    
    /// Geçiş yapılabilir mi?
    pub fn can_transition(&self, from: ModeType, to: ModeType) -> bool {
        // Aynı moda geçiş her zaman geçerli
        if from == to {
            return true;
        }
        
        // Kuralları kontrol et
        self.rules.get(&from)
            .map(|rules| rules.iter().any(|r| r.to == to))
            .unwrap_or(true) // Kural yoksa izin ver
    }
    
    /// Kural ekle
    pub fn add_rule(&mut self, rule: TransitionRule) {
        self.rules.entry(rule.from)
            .or_insert_with(Vec::new)
            .push(rule);
    }
    
    /// Geçerli geçişleri getir
    pub fn get_valid_transitions(&self, from: ModeType) -> Vec<ModeType> {
        self.rules.get(&from)
            .map(|rules| rules.iter().map(|r| r.to).collect())
            .unwrap_or_default()
    }
}
