//! ─── A3 SESSION TREE / COMPACTION ───
//!
//! SENTIENT'nın oturum yönetim sistemi.
//! Hiyerarşik oturum ağacı ve bağlam sıkıştırma.
//!
//! Özellikler:
//! - Tree yapısında oturum hiyerarşisi
//! - Bağlam sıkıştırma (compaction)
//! - Oturum devam ettirme (resume)
//! - Checkpoint oluşturma

pub mod session;
pub mod tree;
pub mod compaction;
pub mod checkpoint;
pub mod history;

pub use session::{Session, SessionConfig, SessionState, SessionType};
pub use tree::{SessionTree, SessionNode, NodeId};
pub use compaction::{Compactor, CompactionConfig, CompactionResult};
pub use checkpoint::{Checkpoint, CheckpointManager};
pub use history::{SessionHistory, HistoryEntry};

use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use lru::LruCache;
use std::num::NonZeroUsize;

// ═══════════════════════════════════════════════════════════════════════════════
// SESSION ERROR
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("Oturum bulunamadı: {0}")]
    NotFound(String),
    
    #[error("Oturum geçersiz: {0}")]
    Invalid(String),
    
    #[error("Sıkıştırma hatası: {0}")]
    CompactionError(String),
    
    #[error("Checkpoint hatası: {0}")]
    CheckpointError(String),
    
    #[error("I/O hatası: {0}")]
    IoError(String),
}

pub type SessionResult<T> = Result<T, SessionError>;

// ═══════════════════════════════════════════════════════════════════════════════
// SESSION MANAGER
// ═══════════════════════════════════════════════════════════════════════════════

/// Oturum yöneticisi
pub struct SessionManager {
    /// Aktif oturum ağacı
    tree: Arc<RwLock<SessionTree>>,
    /// Oturum önbelleği
    cache: Arc<RwLock<LruCache<Uuid, Session>>>,
    /// Sıkıştırma motoru
    compactor: Compactor,
    /// Checkpoint yöneticisi
    checkpoint_manager: CheckpointManager,
    /// Geçmiş
    history: Arc<RwLock<SessionHistory>>,
}

impl SessionManager {
    /// Yeni oturum yöneticisi oluştur
    pub fn new() -> Self {
        Self {
            tree: Arc::new(RwLock::new(SessionTree::new())),
            cache: Arc::new(RwLock::new(LruCache::new(NonZeroUsize::new(100).expect("operation failed")))),
            compactor: Compactor::new(CompactionConfig::default()),
            checkpoint_manager: CheckpointManager::new("data/sessions"),
            history: Arc::new(RwLock::new(SessionHistory::new())),
        }
    }
    
    /// Yeni oturum başlat
    pub async fn create_session(&self, config: SessionConfig) -> SessionResult<Session> {
        let mut session = Session::new(config);
        session.start()?;
        
        // Ağaca ekle
        self.tree.write().await.add_node(session.id, None)?;
        
        // Önbelleğe al
        self.cache.write().await.put(session.id, session.clone());
        
        // Geçmişe kaydet
        self.history.write().await.add_entry(session.id, "created", &session.config.name);
        
        Ok(session)
    }
    
    /// Oturum getir
    pub async fn get_session(&self, id: Uuid) -> SessionResult<Session> {
        // Önce önbellekte ara
        if let Some(session) = self.cache.write().await.get(&id) {
            return Ok(session.clone());
        }
        
        // Ağaçta ara
        let tree = self.tree.read().await;
        if let Some(node) = tree.get_node(&id) {
            // Önbelleğe al
            let session = node.session.clone();
            self.cache.write().await.put(id, session.clone());
            return Ok(session);
        }
        
        Err(SessionError::NotFound(id.to_string()))
    }
    
    /// Alt oturum oluştur
    pub async fn create_child(&self, parent_id: Uuid, config: SessionConfig) -> SessionResult<Session> {
        let mut session = Session::new(config);
        session.parent_id = Some(parent_id);
        session.start()?;
        
        // Ağaca ekle
        self.tree.write().await.add_node(session.id, Some(parent_id))?;
        
        // Önbelleğe al
        self.cache.write().await.put(session.id, session.clone());
        
        Ok(session)
    }
    
    /// Oturumu güncelle
    pub async fn update_session(&self, session: Session) -> SessionResult<()> {
        let id = session.id;
        
        // Ağacı güncelle
        self.tree.write().await.update_session(session.clone())?;
        
        // Önbelleği güncelle
        self.cache.write().await.put(id, session);
        
        Ok(())
    }
    
    /// Oturumu sonlandır
    pub async fn end_session(&self, id: Uuid) -> SessionResult<Session> {
        let mut session = self.get_session(id).await?;
        session.end()?;
        
        // Checkpoint al
        self.checkpoint_manager.save(&session).await?;
        
        // Ağacı güncelle
        self.tree.write().await.update_session(session.clone())?;
        
        // Önbellekten kaldır
        self.cache.write().await.pop(&id);
        
        Ok(session)
    }
    
    /// Bağlam sıkıştır
    pub async fn compact(&self, id: Uuid) -> SessionResult<CompactionResult> {
        let session = self.get_session(id).await?;
        let result = self.compactor.compact(&session)?;
        
        // Sıkıştırılmış bağlamı güncelle
        let mut session = session;
        session.compacted_context = Some(result.summary.clone());
        self.update_session(session).await?;
        
        Ok(result)
    }
    
    /// Oturumu devam ettir
    pub async fn resume(&self, id: Uuid) -> SessionResult<Session> {
        let session = self.checkpoint_manager.load(id).await?;
        let mut session = session;
        session.resume()?;
        
        // Ağaca yeniden ekle
        self.tree.write().await.add_node(session.id, session.parent_id)?;
        
        // Önbelleğe al
        self.cache.write().await.put(session.id, session.clone());
        
        Ok(session)
    }
    
    /// Aktif oturumları listele
    pub async fn list_active(&self) -> Vec<Session> {
        self.tree.read().await.get_active_sessions()
    }
    
    /// Oturum ağacını getir
    pub async fn get_tree(&self) -> SessionTree {
        self.tree.read().await.clone()
    }
    
    /// Oturum geçmişini getir
    pub async fn get_history(&self) -> Vec<HistoryEntry> {
        self.history.read().await.entries().to_vec()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_session_creation() {
        let manager = SessionManager::new();
        let config = SessionConfig::default();
        
        let session = manager.create_session(config).await.expect("operation failed");
        assert!(session.is_active());
    }
    
    #[tokio::test]
    async fn test_child_session() {
        let manager = SessionManager::new();
        
        let parent = manager.create_session(SessionConfig::default()).await.expect("operation failed");
        let child = manager.create_child(parent.id, SessionConfig::default()).await.expect("operation failed");
        
        assert_eq!(child.parent_id, Some(parent.id));
    }
    
    #[tokio::test]
    async fn test_session_compaction() {
        let manager = SessionManager::new();
        let session = manager.create_session(SessionConfig::default()).await.expect("operation failed");
        
        let result = manager.compact(session.id).await.expect("operation failed");
        assert!(!result.summary.is_empty());
    }
}
