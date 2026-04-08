//! ─── TOPLU BELLEK VE BİLGİ SENKRONİZASYONU ───
//!
//! Tüm swarm ajanlarının paylaştığı merkezi bilgi deposu.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use parking_lot::RwLock;
use std::sync::Arc;

use super::blackboard::{KnowledgeEntry, Blackboard};
use super::{SwarmAgentId, SwarmTask};
use super::agent_type::AgentType;

/// ─── COLLECTIVE MEMORY ───
/// 
/// Swarm'ın toplu bilgi deposu.

pub struct CollectiveMemory {
    /// Blackboard referansı
    blackboard: Arc<Blackboard>,
    /// Ajan bazlı önbellek
    agent_cache: Arc<RwLock<HashMap<String, AgentMemoryCache>>>,
    /// Görev bazlı bilgi indeksi
    task_index: Arc<RwLock<HashMap<Uuid, Vec<String>>>>,
    /// Senkronizasyon durumu
    sync_state: Arc<RwLock<SyncState>>,
}

impl CollectiveMemory {
    pub fn new(blackboard: Arc<Blackboard>) -> Self {
        Self {
            blackboard,
            agent_cache: Arc::new(RwLock::new(HashMap::new())),
            task_index: Arc::new(RwLock::new(HashMap::new())),
            sync_state: Arc::new(RwLock::new(SyncState::new())),
        }
    }
    
    /// Bilgi sakla (ajandan)
    pub fn store(&self, agent_id: &SwarmAgentId, key: impl Into<String>, value: serde_json::Value) -> String {
        let key = key.into();
        
        // Blackboard'a yaz
        let entry_id = self.blackboard.write(&key, value.clone(), agent_id.clone());
        
        // Ajan önbelleğini güncelle
        self.agent_cache.write()
            .entry(agent_id.as_str().to_string())
            .or_insert_with(|| AgentMemoryCache::new(agent_id.clone()))
            .add_entry(entry_id.clone());
        
        // Senkronizasyon durumu güncelle
        self.sync_state.write().touch();
        
        log::debug!("💾 COLLECTIVE: {} stored {}", agent_id.as_str(), key);
        entry_id
    }
    
    /// Bilgi oku
    pub fn retrieve(&self, key: &str) -> Option<KnowledgeEntry> {
        self.blackboard.read(key)
    }
    
    /// Görev ile ilgili bilgi sakla
    pub fn store_for_task(&self, agent_id: &SwarmAgentId, task: &SwarmTask, key: impl Into<String>, value: serde_json::Value) -> String {
        let key = key.into();
        let entry_id = self.store(agent_id, &key, value);
        
        // Görev indeksine ekle
        self.task_index.write()
            .entry(task.id)
            .or_insert_with(Vec::new)
            .push(entry_id.clone());
        
        self.blackboard.add_task_knowledge(task.id, entry_id.clone());
        
        entry_id
    }
    
    /// Görev bilgilerini getir
    pub fn get_task_knowledge(&self, task_id: &Uuid) -> Vec<KnowledgeEntry> {
        self.blackboard.get_task_knowledge(task_id)
    }
    
    /// Ajanın katkılarını getir
    pub fn get_agent_contributions(&self, agent_id: &SwarmAgentId) -> Vec<KnowledgeEntry> {
        self.blackboard.get_agent_knowledge(agent_id)
    }
    
    /// Arama
    pub fn search(&self, query: &str) -> Vec<KnowledgeEntry> {
        self.blackboard.search_by_key(query)
    }
    
    /// Belirli bir türdeki bilgileri getir
    pub fn get_by_type(&self, type_name: &str) -> Vec<KnowledgeEntry> {
        self.blackboard.search_by_tag(type_name)
    }
    
    /// Senkronizasyon durumu
    pub fn sync_state(&self) -> SyncState {
        self.sync_state.read().clone()
    }
    
    /// Ajan önbelleğini senkronize et
    pub fn sync_agent_cache(&self, agent_id: &SwarmAgentId) -> KnowledgeSync {
        let mut cache = self.agent_cache.write();
        
        // Ajanın tüm girdilerini getir
        let entries = self.blackboard.get_agent_knowledge(agent_id);
        
        // Önbelleği güncelle
        cache.entry(agent_id.as_str().to_string())
            .and_modify(|c| {
                c.last_sync = Utc::now();
                c.entry_count = entries.len();
            })
            .or_insert_with(|| {
                let mut c = AgentMemoryCache::new(agent_id.clone());
                c.entry_count = entries.len();
                c
            });
        
        // Senkronizasyon kaydı oluştur
        KnowledgeSync {
            agent_id: agent_id.clone(),
            entries_synced: entries.len() as u64,
            timestamp: Utc::now(),
            status: SyncStatus::Complete,
        }
    }
    
    /// Toplu senkronizasyon
    pub fn sync_all(&self) -> Vec<KnowledgeSync> {
        let caches = self.agent_cache.read();
        caches.keys()
            .map(|id| {
                let agent_id = SwarmAgentId::new(); // TODO: Parse from string
                self.sync_agent_cache(&agent_id)
            })
            .collect()
    }
    
    /// Rapor
    pub fn report(&self) -> MemoryReport {
        let cache_count = self.agent_cache.read().len();
        let task_count = self.task_index.read().len();
        let bb_report = self.blackboard.report();
        
        MemoryReport {
            total_entries: bb_report.total_entries,
            agent_caches: cache_count,
            task_indexes: task_count,
            version: bb_report.version,
            last_sync: self.sync_state.read().last_sync,
        }
    }
    
    /// İstatistikler
    pub fn stats(&self) -> MemoryStats {
        let caches = self.agent_cache.read();
        let total_cached = caches.values().map(|c| c.entry_count).sum::<usize>();
        
        MemoryStats {
            total_entries: self.blackboard.count(),
            cached_entries: total_cached,
            agent_count: caches.len(),
            sync_count: self.sync_state.read().sync_count,
        }
    }
}

/// ─── AGENT MEMORY CACHE ───

#[derive(Debug, Clone)]
pub struct AgentMemoryCache {
    pub agent_id: SwarmAgentId,
    pub entries: Vec<String>,
    pub entry_count: usize,
    pub last_sync: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl AgentMemoryCache {
    pub fn new(agent_id: SwarmAgentId) -> Self {
        Self {
            agent_id,
            entries: Vec::new(),
            entry_count: 0,
            last_sync: Utc::now(),
            created_at: Utc::now(),
        }
    }
    
    pub fn add_entry(&mut self, entry_id: String) {
        self.entries.push(entry_id);
        self.entry_count = self.entries.len();
    }
}

/// ─── SYNC STATE ───

#[derive(Debug, Clone)]
pub struct SyncState {
    pub last_sync: DateTime<Utc>,
    pub sync_count: u64,
    pub pending_syncs: usize,
}

impl SyncState {
    pub fn new() -> Self {
        Self {
            last_sync: Utc::now(),
            sync_count: 0,
            pending_syncs: 0,
        }
    }
    
    pub fn touch(&mut self) {
        self.last_sync = Utc::now();
        self.sync_count += 1;
    }
}

impl Default for SyncState {
    fn default() -> Self {
        Self::new()
    }
}

/// ─── KNOWLEDGE SYNC ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeSync {
    pub agent_id: SwarmAgentId,
    pub entries_synced: u64,
    pub timestamp: DateTime<Utc>,
    pub status: SyncStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncStatus {
    Pending,
    InProgress,
    Complete,
    Failed,
}

/// ─── MEMORY REPORT ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryReport {
    pub total_entries: usize,
    pub agent_caches: usize,
    pub task_indexes: usize,
    pub version: u64,
    pub last_sync: DateTime<Utc>,
}

/// ─── MEMORY STATS ───

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryStats {
    pub total_entries: usize,
    pub cached_entries: usize,
    pub agent_count: usize,
    pub sync_count: u64,
}

impl std::fmt::Display for MemoryStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "💾 Memory: {} entries ({} cached) | {} agents | {} syncs",
            self.total_entries,
            self.cached_entries,
            self.agent_count,
            self.sync_count
        )
    }
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_collective_memory_store() {
        let bb = Arc::new(Blackboard::new());
        let memory = CollectiveMemory::new(bb);
        let agent = SwarmAgentId::new();
        
        let entry_id = memory.store(&agent, "test_key", serde_json::json!(42));
        assert!(!entry_id.is_empty());
        
        let entry = memory.retrieve("test_key").unwrap();
        assert_eq!(entry.value, 42);
    }
    
    #[test]
    fn test_collective_memory_search() {
        let bb = Arc::new(Blackboard::new());
        let memory = CollectiveMemory::new(bb);
        let agent = SwarmAgentId::new();
        
        memory.store(&agent, "python_code", serde_json::Value::Null);
        memory.store(&agent, "rust_code", serde_json::Value::Null);
        
        let results = memory.search("code");
        assert_eq!(results.len(), 2);
    }
    
    #[test]
    fn test_agent_cache() {
        let agent = SwarmAgentId::new();
        let cache = AgentMemoryCache::new(agent.clone());
        
        assert_eq!(cache.entry_count, 0);
    }
    
    #[test]
    fn test_sync_state() {
        let mut state = SyncState::new();
        state.touch();
        
        assert_eq!(state.sync_count, 1);
    }
}
