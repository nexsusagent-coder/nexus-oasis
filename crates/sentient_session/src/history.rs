//! ─── SESSION HISTORY ───
//!
//! Oturum geçmişi

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Geçmiş girdisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: Uuid,
    pub session_id: Uuid,
    pub action: String,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

/// Oturum geçmişi
#[derive(Debug, Clone, Default)]
pub struct SessionHistory {
    entries: Vec<HistoryEntry>,
}

impl SessionHistory {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    
    /// Geçmişe kaydet
    pub fn add_entry(&mut self, session_id: Uuid, action: &str, description: &str) {
        self.entries.push(HistoryEntry {
            id: Uuid::new_v4(),
            session_id,
            action: action.into(),
            description: description.into(),
            timestamp: Utc::now(),
            metadata: std::collections::HashMap::new(),
        });
    }
    
    pub fn entries(&self) -> &[HistoryEntry] {
        &self.entries
    }
    
    pub fn for_session(&self, session_id: Uuid) -> Vec<&HistoryEntry> {
        self.entries.iter().filter(|e| e.session_id == session_id).collect()
    }
    
    pub fn recent(&self, limit: usize) -> Vec<&HistoryEntry> {
        self.entries.iter().rev().take(limit).collect()
    }
}
