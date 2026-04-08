//! Arama Graph Yapısı
//! MindSearch ve AutoResearch için ortak graph modeli

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Arama grafiği
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchGraph {
    /// Tüm node'lar
    pub nodes: Vec<GraphNode>,
    /// Kenarlar
    pub edges: Vec<GraphEdge>,
    /// Root node ID
    pub root_id: String,
    /// Oluşturulma zamanı
    pub created_at: String,
}

/// Graph node'u
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    /// Node ID
    pub id: String,
    /// Arama sorgusu
    pub query: String,
    /// Sonuç/yanıt
    pub response: Option<String>,
    /// Alt node'lar
    pub children: Vec<String>,
    /// Referanslar
    pub references: HashMap<String, String>,
}

/// Graph kenarı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    /// Kaynak node
    pub from: String,
    /// Hedef node
    pub to: String,
    /// Etiket
    pub label: Option<String>,
}

impl SearchGraph {
    /// Yeni graph oluştur
    pub fn new(root_query: &str) -> Self {
        let root_id = uuid::Uuid::new_v4().to_string();
        
        let root_node = GraphNode {
            id: root_id.clone(),
            query: root_query.to_string(),
            response: None,
            children: vec![],
            references: HashMap::new(),
        };
        
        Self {
            nodes: vec![root_node],
            edges: vec![],
            root_id,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
    
    /// Node ekle
    pub fn add_node(&mut self, parent_id: &str, query: &str) -> String {
        let node_id = uuid::Uuid::new_v4().to_string();
        
        let node = GraphNode {
            id: node_id.clone(),
            query: query.to_string(),
            response: None,
            children: vec![],
            references: HashMap::new(),
        };
        
        self.nodes.push(node);
        
        // Edge ekle
        self.edges.push(GraphEdge {
            from: parent_id.to_string(),
            to: node_id.clone(),
            label: None,
        });
        
        // Parent'ın children listesini güncelle
        if let Some(parent) = self.nodes.iter_mut().find(|n| n.id == parent_id) {
            parent.children.push(node_id.clone());
        }
        
        node_id
    }
    
    /// Node yanıtını güncelle
    pub fn set_response(&mut self, node_id: &str, response: &str) {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == node_id) {
            node.response = Some(response.to_string());
        }
    }
    
    /// Referans ekle
    pub fn add_reference(&mut self, node_id: &str, url: &str, title: &str) {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == node_id) {
            node.references.insert(url.to_string(), title.to_string());
        }
    }
    
    /// Node'u getir
    pub fn get_node(&self, node_id: &str) -> Option<&GraphNode> {
        self.nodes.iter().find(|n| n.id == node_id)
    }
    
    /// Root'u getir
    pub fn root(&self) -> Option<&GraphNode> {
        self.get_node(&self.root_id)
    }
    
    /// Tüm node'ları derinlik sırasına göre getir
    pub fn nodes_by_depth(&self) -> Vec<&GraphNode> {
        let mut result = Vec::new();
        let mut visited = std::collections::HashSet::new();
        
        self.collect_children(&self.root_id, &mut result, &mut visited);
        
        result
    }
    
    fn collect_children<'a>(
        &'a self,
        node_id: &str,
        result: &mut Vec<&'a GraphNode>,
        visited: &mut std::collections::HashSet<String>,
    ) {
        if visited.contains(node_id) {
            return;
        }
        visited.insert(node_id.to_string());
        
        if let Some(node) = self.get_node(node_id) {
            result.push(node);
            
            for child_id in &node.children {
                self.collect_children(child_id, result, visited);
            }
        }
    }
    
    /// Toplam node sayısı
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    
    /// Toplam kenar sayısı
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
    
    /// Graph'ı JSON string'e dönüştür
    pub fn to_json(&self) -> crate::ResearchResult<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
    
    /// JSON string'den graph oluştur
    pub fn from_json(json: &str) -> crate::ResearchResult<Self> {
        Ok(serde_json::from_str(json)?)
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_graph_creation() {
        let graph = SearchGraph::new("Test query");
        assert_eq!(graph.node_count(), 1);
        assert!(graph.root().is_some());
    }
    
    #[test]
    fn test_node_addition() {
        let mut graph = SearchGraph::new("Root query");
        let root_id = graph.root_id.clone();
        let child_id = graph.add_node(&root_id, "Child query");
        
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
        
        let root = graph.root().unwrap();
        assert!(root.children.contains(&child_id));
    }
    
    #[test]
    fn test_response_setting() {
        let mut graph = SearchGraph::new("Test");
        let root_id = graph.root_id.clone();
        graph.set_response(&root_id, "Test response");
        
        let root = graph.root().unwrap();
        assert!(root.response.is_some());
    }
    
    #[test]
    fn test_reference_addition() {
        let mut graph = SearchGraph::new("Test");
        let root_id = graph.root_id.clone();
        graph.add_reference(&root_id, "https://example.com", "Example");
        
        let root = graph.root().unwrap();
        assert!(root.references.contains_key("https://example.com"));
    }
    
    #[test]
    fn test_json_serialization() {
        let graph = SearchGraph::new("Test");
        let json = graph.to_json().unwrap();
        
        let parsed = SearchGraph::from_json(&json).unwrap();
        assert_eq!(parsed.node_count(), graph.node_count());
    }
    
    #[test]
    fn test_depth_ordering() {
        let mut graph = SearchGraph::new("Root");
        let root_id = graph.root_id.clone();
        let child1 = graph.add_node(&root_id, "Child 1");
        let _grandchild = graph.add_node(&child1, "Grandchild");
        let _child2 = graph.add_node(&root_id, "Child 2");
        
        let by_depth = graph.nodes_by_depth();
        assert_eq!(by_depth.len(), 4);
        assert_eq!(by_depth[0].id, graph.root_id);
    }
}
