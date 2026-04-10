//! Research Bellek Köprüsü
//! Araştırma sonuçlarını sentient_memory'ye kaydetme/yükleme

use crate::error::{ResearchError, ResearchResult};
use crate::graph::SearchGraph;
use serde::{Deserialize, Serialize};

/// Research bellek köprüsü
pub struct ResearchMemoryBridge {
    /// Aktif oturumlar
    sessions: std::collections::HashMap<String, ResearchSession>,
}

/// Araştırma oturumu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchSession {
    /// Oturum ID
    pub id: String,
    /// Konu
    pub topic: String,
    /// Arama grafiği
    pub graph: SearchGraph,
    /// Oluşturulma zamanı
    pub created_at: String,
    /// Son güncelleme
    pub updated_at: String,
    /// Etiketler
    pub tags: Vec<String>,
    /// Notlar
    pub notes: Vec<String>,
}

/// Bellek girişi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    /// Giriş ID
    pub id: String,
    /// Anahtar
    pub key: String,
    /// Değer
    pub value: String,
    /// Tür
    pub entry_type: MemoryType,
    /// Zaman damgası
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryType {
    Research,
    Citation,
    Summary,
    Graph,
    Note,
}

impl ResearchMemoryBridge {
    /// Yeni bellek köprüsü oluştur
    pub fn new() -> Self {
        Self {
            sessions: std::collections::HashMap::new(),
        }
    }
    
    /// Araştırmayı kaydet
    pub async fn save_research(&mut self, session_id: &str, graph: &SearchGraph) -> ResearchResult<()> {
        log::info!("💾 RESEARCH-MEMORY: Araştırma kaydediliyor → {}", session_id);
        
        let session = ResearchSession {
            id: session_id.to_string(),
            topic: graph.root()
                .map(|n| n.query.clone())
                .unwrap_or_else(|| "Unknown".into()),
            graph: graph.clone(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            tags: vec![],
            notes: vec![],
        };
        
        self.sessions.insert(session_id.to_string(), session);
        
        log::info!("✅ RESEARCH-MEMORY: Araştırma kaydedildi");
        Ok(())
    }
    
    /// Araştırmayı yükle
    pub async fn load_research(&self, session_id: &str) -> ResearchResult<Option<SearchGraph>> {
        log::info!("💾 RESEARCH-MEMORY: Araştırma yükleniyor → {}", session_id);
        
        Ok(self.sessions.get(session_id).map(|s| s.graph.clone()))
    }
    
    /// Tüm oturumları listele
    pub fn list_sessions(&self) -> Vec<&ResearchSession> {
        self.sessions.values().collect()
    }
    
    /// Oturumu sil
    pub fn delete_session(&mut self, session_id: &str) -> ResearchResult<()> {
        self.sessions.remove(session_id);
        log::info!("💾 RESEARCH-MEMORY: Oturum silindi → {}", session_id);
        Ok(())
    }
    
    /// Etiket ekle
    pub fn add_tag(&mut self, session_id: &str, tag: &str) -> ResearchResult<()> {
        if let Some(session) = self.sessions.get_mut(session_id) {
            if !session.tags.contains(&tag.to_string()) {
                session.tags.push(tag.to_string());
                session.updated_at = chrono::Utc::now().to_rfc3339();
            }
        }
        Ok(())
    }
    
    /// Not ekle
    pub fn add_note(&mut self, session_id: &str, note: &str) -> ResearchResult<()> {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.notes.push(note.to_string());
            session.updated_at = chrono::Utc::now().to_rfc3339();
        }
        Ok(())
    }
    
    /// Oturumu JSON olarak dışa aktar
    pub fn export_session(&self, session_id: &str) -> ResearchResult<Option<String>> {
        Ok(self.sessions.get(session_id).map(|s| {
            serde_json::to_string_pretty(s).unwrap_or_default()
        }))
    }
    
    /// JSON'dan oturum içe aktar
    pub fn import_session(&mut self, json: &str) -> ResearchResult<String> {
        let session: ResearchSession = serde_json::from_str(json)?;
        let id = session.id.clone();
        self.sessions.insert(id.clone(), session);
        Ok(id)
    }
}

impl Default for ResearchMemoryBridge {
    fn default() -> Self {
        Self::new()
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_bridge_creation() {
        let bridge = ResearchMemoryBridge::new();
        assert!(bridge.sessions.is_empty());
    }
    
    #[tokio::test]
    async fn test_save_and_load_research() {
        let mut bridge = ResearchMemoryBridge::new();
        let graph = SearchGraph::new("Test query");
        
        bridge.save_research("test-session", &graph).await.expect("operation failed");
        
        let loaded = bridge.load_research("test-session").await.expect("operation failed");
        assert!(loaded.is_some());
    }
    
    #[test]
    fn test_add_tag() {
        let mut bridge = ResearchMemoryBridge::new();
        let graph = SearchGraph::new("Test");
        
        // Manuel ekleme (async olmadan)
        bridge.sessions.insert("test".into(), ResearchSession {
            id: "test".into(),
            topic: "Test".into(),
            graph,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            tags: vec![],
            notes: vec![],
        });
        
        bridge.add_tag("test", "important").expect("operation failed");
        
        let session = bridge.sessions.get("test").expect("operation failed");
        assert!(session.tags.contains(&"important".to_string()));
    }
}
