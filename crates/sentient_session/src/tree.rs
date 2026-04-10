//! ─── SESSION TREE ───
//!
//! Hiyerarşik oturum ağacı

use crate::{Session, SessionError, SessionResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Node ID
pub type NodeId = Uuid;

/// Oturum ağacı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionTree {
    /// Kök düğümler
    roots: Vec<NodeId>,
    /// Tüm düğümler
    nodes: HashMap<NodeId, SessionNode>,
}

impl SessionTree {
    /// Yeni ağaç oluştur
    pub fn new() -> Self {
        Self {
            roots: Vec::new(),
            nodes: HashMap::new(),
        }
    }
    
    /// Düğüm ekle
    pub fn add_node(&mut self, id: NodeId, parent_id: Option<NodeId>) -> SessionResult<()> {
        let node = SessionNode::new(id);
        
        if let Some(pid) = parent_id {
            // Ebeveyn bul ve çocuk ekle
            if let Some(parent) = self.nodes.get_mut(&pid) {
                parent.children.push(id);
            } else {
                return Err(SessionError::NotFound(format!("Parent node: {}", pid)));
            }
            self.nodes.insert(id, node);
        } else {
            // Kök düğüm
            self.roots.push(id);
            self.nodes.insert(id, node);
        }
        
        Ok(())
    }
    
    /// Düğüm getir
    pub fn get_node(&self, id: &NodeId) -> Option<&SessionNode> {
        self.nodes.get(id)
    }
    
    /// Oturumu güncelle
    pub fn update_session(&mut self, session: Session) -> SessionResult<()> {
        if let Some(node) = self.nodes.get_mut(&session.id) {
            node.session = session;
            Ok(())
        } else {
            Err(SessionError::NotFound(session.id.to_string()))
        }
    }
    
    /// Alt düğümleri getir
    pub fn get_children(&self, id: &NodeId) -> Vec<&SessionNode> {
        self.nodes.get(id)
            .map(|node| node.children.iter().filter_map(|cid| self.nodes.get(cid)).collect())
            .unwrap_or_default()
    }
    
    /// Ebeveyn getir
    pub fn get_parent(&self, id: &NodeId) -> Option<&SessionNode> {
        for node in self.nodes.values() {
            if node.children.contains(id) {
                return Some(node);
            }
        }
        None
    }
    
    /// Aktif oturumları getir
    pub fn get_active_sessions(&self) -> Vec<Session> {
        self.nodes.values()
            .filter(|n| n.session.is_active())
            .map(|n| n.session.clone())
            .collect()
    }
    
    /// Kök düğümleri getir
    pub fn get_roots(&self) -> Vec<&SessionNode> {
        self.roots.iter().filter_map(|id| self.nodes.get(id)).collect()
    }
    
    /// Derinlik hesapla
    pub fn depth(&self, id: &NodeId) -> usize {
        let mut depth = 0;
        let mut current_id = *id;
        
        while let Some(parent) = self.get_parent(&current_id) {
            depth += 1;
            current_id = parent.session.id;
        }
        
        depth
    }
    
    /// Toplam düğüm sayısı
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    
    /// Ağacı düzleştir (DFS sırası)
    pub fn flatten(&self) -> Vec<&SessionNode> {
        let mut result = Vec::new();
        
        for root_id in &self.roots {
            self.flatten_recursive(root_id, &mut result);
        }
        
        result
    }
    
    fn flatten_recursive<'a>(&'a self, id: &NodeId, result: &mut Vec<&'a SessionNode>) {
        if let Some(node) = self.nodes.get(id) {
            result.push(node);
            for child_id in &node.children {
                self.flatten_recursive(child_id, result);
            }
        }
    }
}

impl Default for SessionTree {
    fn default() -> Self {
        Self::new()
    }
}

/// Oturum düğümü
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionNode {
    /// Oturum
    pub session: Session,
    /// Alt düğümler
    pub children: Vec<NodeId>,
    /// Oluşturulma zamanı
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl SessionNode {
    pub fn new(_id: NodeId) -> Self {
        Self {
            session: Session::new(crate::session::SessionConfig::default()),
            children: Vec::new(),
            created_at: chrono::Utc::now(),
        }
    }
    
    pub fn with_session(session: Session) -> Self {
        Self {
            session,
            children: Vec::new(),
            created_at: chrono::Utc::now(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::SessionConfig;
    
    #[test]
    fn test_tree_creation() {
        let tree = SessionTree::new();
        assert_eq!(tree.node_count(), 0);
    }
    
    #[test]
    fn test_add_root_node() {
        let mut tree = SessionTree::new();
        let id = Uuid::new_v4();
        
        tree.add_node(id, None).expect("operation failed");
        assert_eq!(tree.node_count(), 1);
        assert_eq!(tree.get_roots().len(), 1);
    }
    
    #[test]
    fn test_add_child_node() {
        let mut tree = SessionTree::new();
        let parent_id = Uuid::new_v4();
        let child_id = Uuid::new_v4();
        
        tree.add_node(parent_id, None).expect("operation failed");
        tree.add_node(child_id, Some(parent_id)).expect("operation failed");
        
        assert_eq!(tree.node_count(), 2);
        assert_eq!(tree.get_children(&parent_id).len(), 1);
        assert_eq!(tree.depth(&child_id), 1);
    }
}
