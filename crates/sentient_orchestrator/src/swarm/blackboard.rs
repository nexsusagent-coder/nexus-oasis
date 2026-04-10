//! ─── ORTAK BİLGİ ALANI (BLACKBOARD) ───
//!
//! Ajanların bilgi paylaşabileceği merkezi depolama alanı.
//! Blackboard pattern - tüm ajanlar okuyabilir ve yazabilir.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use parking_lot::RwLock;
use std::sync::Arc;

use super::SwarmAgentId;

/// ─── BLACKBOARD ───
/// 
/// Ortak bilgi alanı - tüm ajanlar erişebilir.

pub struct Blackboard {
    /// Bilgi girdileri
    entries: Arc<RwLock<HashMap<String, KnowledgeEntry>>>,
    /// Görev bazlı bilgi
    task_knowledge: Arc<RwLock<HashMap<Uuid, Vec<String>>>>,
    /// Ajan bazlı bilgi
    agent_knowledge: Arc<RwLock<HashMap<String, Vec<String>>>>,
    /// Global bağlam
    shared_context: Arc<RwLock<SharedContext>>,
    /// Versiyon (optimistic locking için)
    version: Arc<RwLock<u64>>,
}

impl Blackboard {
    /// Yeni blackboard oluştur
    pub fn new() -> Self {
        Self {
            entries: Arc::new(RwLock::new(HashMap::new())),
            task_knowledge: Arc::new(RwLock::new(HashMap::new())),
            agent_knowledge: Arc::new(RwLock::new(HashMap::new())),
            shared_context: Arc::new(RwLock::new(SharedContext::new())),
            version: Arc::new(RwLock::new(1)),
        }
    }
    
    /// Bilgi yaz
    pub fn write(&self, key: impl Into<String>, value: serde_json::Value, author: SwarmAgentId) -> String {
        let key = key.into();
        let entry_id = format!("kb_{}", Uuid::new_v4());
        
        let entry = KnowledgeEntry {
            id: entry_id.clone(),
            key: key.clone(),
            value,
            author: author.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            version: 1,
            tags: Vec::new(),
            confidence: 1.0,
            expires_at: None,
        };
        
        self.entries.write().insert(entry_id.clone(), entry);
        self.bump_version();
        
        log::debug!("📝 BLACKBOARD: {} = {:?} (by {})", key, entry_id, author.as_str());
        entry_id
    }
    
    /// Bilgi oku
    pub fn read(&self, key: &str) -> Option<KnowledgeEntry> {
        self.entries.read().values()
            .find(|e| e.key == key)
            .cloned()
    }
    
    /// ID ile oku
    pub fn read_by_id(&self, id: &str) -> Option<KnowledgeEntry> {
        self.entries.read().get(id).cloned()
    }
    
    /// Bilgi güncelle
    pub fn update(&self, entry_id: &str, value: serde_json::Value) -> bool {
        let mut entries = self.entries.write();
        
        if let Some(entry) = entries.get_mut(entry_id) {
            entry.value = value;
            entry.updated_at = Utc::now();
            entry.version += 1;
            self.bump_version();
            true
        } else {
            false
        }
    }
    
    /// Bilgi sil
    pub fn delete(&self, entry_id: &str) -> bool {
        let removed = self.entries.write().remove(entry_id).is_some();
        if removed {
            self.bump_version();
        }
        removed
    }
    
    /// Etiket ile ara
    pub fn search_by_tag(&self, tag: &str) -> Vec<KnowledgeEntry> {
        self.entries.read().values()
            .filter(|e| e.tags.iter().any(|t| t.contains(tag)))
            .cloned()
            .collect()
    }
    
    /// Anahtar deseni ile ara
    pub fn search_by_key(&self, pattern: &str) -> Vec<KnowledgeEntry> {
        let pattern = pattern.to_lowercase();
        self.entries.read().values()
            .filter(|e| e.key.to_lowercase().contains(&pattern))
            .cloned()
            .collect()
    }
    
    /// Görev bazlı bilgi ekle
    pub fn add_task_knowledge(&self, task_id: Uuid, entry_id: String) {
        self.task_knowledge.write()
            .entry(task_id)
            .or_insert_with(Vec::new)
            .push(entry_id);
    }
    
    /// Görev bilgilerini getir
    pub fn get_task_knowledge(&self, task_id: &Uuid) -> Vec<KnowledgeEntry> {
        let task_map = self.task_knowledge.read();
        let entries = self.entries.read();
        
        task_map.get(task_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| entries.get(id).cloned())
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Ajan bazlı bilgi ekle
    pub fn add_agent_knowledge(&self, agent_id: &SwarmAgentId, entry_id: String) {
        self.agent_knowledge.write()
            .entry(agent_id.as_str().to_string())
            .or_insert_with(Vec::new)
            .push(entry_id);
    }
    
    /// Ajan bilgilerini getir
    pub fn get_agent_knowledge(&self, agent_id: &SwarmAgentId) -> Vec<KnowledgeEntry> {
        let agent_map = self.agent_knowledge.read();
        let entries = self.entries.read();
        
        agent_map.get(agent_id.as_str())
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| entries.get(id).cloned())
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Paylaşılan bağlam
    pub fn context(&self) -> SharedContext {
        self.shared_context.read().clone()
    }
    
    /// Bağlam güncelle
    pub fn update_context(&self, key: impl Into<String>, value: serde_json::Value) {
        self.shared_context.write().set(key, value);
        self.bump_version();
    }
    
    /// Tüm girdileri getir
    pub fn all_entries(&self) -> Vec<KnowledgeEntry> {
        self.entries.read().values().cloned().collect()
    }
    
    /// Toplam giriş sayısı
    pub fn count(&self) -> usize {
        self.entries.read().len()
    }
    
    /// Versiyon
    pub fn version(&self) -> u64 {
        *self.version.read()
    }
    
    /// Versiyon artır
    fn bump_version(&self) {
        *self.version.write() += 1;
    }
    
    /// Temizle (eski girdileri sil)
    pub fn cleanup_expired(&self) -> usize {
        let mut entries = self.entries.write();
        let now = Utc::now();
        
        let expired: Vec<String> = entries.values()
            .filter(|e| e.expires_at.map(|exp| exp < now).unwrap_or(false))
            .map(|e| e.id.clone())
            .collect();
        
        let count = expired.len();
        for id in expired {
            entries.remove(&id);
        }
        
        if count > 0 {
            self.bump_version();
        }
        
        count
    }
    
    /// Rapor
    pub fn report(&self) -> BlackboardReport {
        let entries = self.entries.read();
        let task_knowledge = self.task_knowledge.read();
        let agent_knowledge = self.agent_knowledge.read();
        
        BlackboardReport {
            total_entries: entries.len(),
            task_count: task_knowledge.len(),
            agent_count: agent_knowledge.len(),
            version: *self.version.read(),
            oldest_entry: entries.values().map(|e| e.created_at).min(),
            newest_entry: entries.values().map(|e| e.created_at).max(),
        }
    }
}

impl Default for Blackboard {
    fn default() -> Self {
        Self::new()
    }
}

/// ─── KNOWLEDGE ENTRY ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEntry {
    /// Giriş ID
    pub id: String,
    /// Anahtar
    pub key: String,
    /// Değer
    pub value: serde_json::Value,
    /// Yazar ajan
    pub author: SwarmAgentId,
    /// Oluşturulma zamanı
    pub created_at: DateTime<Utc>,
    /// Güncelleme zamanı
    pub updated_at: DateTime<Utc>,
    /// Versiyon
    pub version: u64,
    /// Etiketler
    pub tags: Vec<String>,
    /// Güven skor
    pub confidence: f32,
    /// Son kullanma
    pub expires_at: Option<DateTime<Utc>>,
}

impl KnowledgeEntry {
    /// Yaşı (saniye)
    pub fn age_secs(&self) -> i64 {
        (Utc::now() - self.created_at).num_seconds()
    }
    
    /// Süre (güncellemeden beri)
    pub fn since_update_secs(&self) -> i64 {
        (Utc::now() - self.updated_at).num_seconds()
    }
    
    /// Etiket ekle
    pub fn add_tag(&mut self, tag: impl Into<String>) {
        self.tags.push(tag.into());
    }
    
    /// Süre belirle
    pub fn expires_in(&mut self, secs: i64) {
        self.expires_at = Some(Utc::now() + chrono::Duration::seconds(secs));
    }
}

/// ─── SHARED CONTEXT ───

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SharedContext {
    /// Global değişkenler
    variables: HashMap<String, serde_json::Value>,
    /// Aktif hedef
    current_goal: Option<String>,
    /// Mevcut strateji
    strategy: Option<String>,
    /// Swarm modu
    mode: SwarmMode,
    /// Son güncelleme
    last_updated: DateTime<Utc>,
}

impl SharedContext {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            current_goal: None,
            strategy: None,
            mode: SwarmMode::Normal,
            last_updated: Utc::now(),
        }
    }
    
    pub fn set(&mut self, key: impl Into<String>, value: serde_json::Value) {
        self.variables.insert(key.into(), value);
        self.last_updated = Utc::now();
    }
    
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.variables.get(key)
    }
    
    pub fn set_goal(&mut self, goal: impl Into<String>) {
        self.current_goal = Some(goal.into());
        self.last_updated = Utc::now();
    }
    
    pub fn set_strategy(&mut self, strategy: impl Into<String>) {
        self.strategy = Some(strategy.into());
        self.last_updated = Utc::now();
    }
    
    pub fn set_mode(&mut self, mode: SwarmMode) {
        self.mode = mode;
        self.last_updated = Utc::now();
    }
    
    pub fn goal(&self) -> Option<&str> {
        self.current_goal.as_deref()
    }
    
    pub fn strategy(&self) -> Option<&str> {
        self.strategy.as_deref()
    }
    
    pub fn mode(&self) -> SwarmMode {
        self.mode
    }
}

/// ─── SWARM MODE ───

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SwarmMode {
    /// Normal çalışma
    Normal,
    /// Acil mod - kritik görevler öncelikli
    Emergency,
    /// Öğrenme modu - yeni bilgileri kaydet
    Learning,
    /// Bakım modu - temizlik ve optimizasyon
    Maintenance,
}

impl Default for SwarmMode {
    fn default() -> Self {
        Self::Normal
    }
}

/// ─── BLACKBOARD REPORT ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlackboardReport {
    pub total_entries: usize,
    pub task_count: usize,
    pub agent_count: usize,
    pub version: u64,
    pub oldest_entry: Option<DateTime<Utc>>,
    pub newest_entry: Option<DateTime<Utc>>,
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_blackboard_write_read() {
        let bb = Blackboard::new();
        let agent = SwarmAgentId::new();
        
        let entry_id = bb.write("test_key", serde_json::json!({"value": 42}), agent);
        
        let entry = bb.read("test_key").expect("operation failed");
        assert_eq!(entry.value["value"], 42);
        assert_eq!(entry.id, entry_id);
    }
    
    #[test]
    fn test_blackboard_update() {
        let bb = Blackboard::new();
        let agent = SwarmAgentId::new();
        
        let entry_id = bb.write("key", serde_json::json!(1), agent);
        bb.update(&entry_id, serde_json::json!(2));
        
        let entry = bb.read_by_id(&entry_id).expect("operation failed");
        assert_eq!(entry.value, 2);
        assert_eq!(entry.version, 2);
    }
    
    #[test]
    fn test_blackboard_delete() {
        let bb = Blackboard::new();
        let agent = SwarmAgentId::new();
        
        let entry_id = bb.write("key", serde_json::Value::Null, agent);
        assert!(bb.delete(&entry_id));
        assert!(bb.read_by_id(&entry_id).is_none());
    }
    
    #[test]
    fn test_blackboard_search() {
        let bb = Blackboard::new();
        let agent = SwarmAgentId::new();
        
        bb.write("python_code", serde_json::Value::Null, agent.clone());
        bb.write("rust_code", serde_json::Value::Null, agent);
        
        let results = bb.search_by_key("code");
        assert_eq!(results.len(), 2);
    }
    
    #[test]
    fn test_shared_context() {
        let mut ctx = SharedContext::new();
        
        ctx.set_goal("Test hedefi");
        ctx.set("var1", serde_json::json!(100));
        
        assert_eq!(ctx.goal(), Some("Test hedefi"));
        assert_eq!(ctx.get("var1"), Some(&serde_json::json!(100)));
    }
    
    #[test]
    fn test_knowledge_entry_tags() {
        let mut entry = KnowledgeEntry {
            id: "test".into(),
            key: "test".into(),
            value: serde_json::Value::Null,
            author: SwarmAgentId::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            version: 1,
            tags: Vec::new(),
            confidence: 1.0,
            expires_at: None,
        };
        
        entry.add_tag("important");
        entry.expires_in(3600);
        
        assert!(entry.tags.contains(&"important".to_string()));
        assert!(entry.expires_at.is_some());
    }
}
