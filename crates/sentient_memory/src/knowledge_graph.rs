//! ─── BİLGİ GRAFİĞİ ───
//!
//! Bellekler arası ilişkileri yöneten bilgi grafiği:
//! - İlişki çıkarımı
//! - Graf sorguları
//! - Ontoloji yönetimi

use crate::{MemoryEntry, MemoryType, RelationType, MemoryError, MemoryResult};
use rusqlite::{Connection, params};
use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;

// ─────────────────────────────────────────────────────────────────────────────
// GRAPH NODE
// ─────────────────────────────────────────────────────────────────────────────

/// Graf düğümü
#[derive(Debug, Clone)]
pub struct GraphNode {
    pub id: Uuid,
    pub label: String,
    pub memory_type: MemoryType,
    pub properties: HashMap<String, String>,
}

// ─────────────────────────────────────────────────────────────────────────────
// GRAPH EDGE
// ─────────────────────────────────────────────────────────────────────────────

/// Graf kenarı (ilişki)
#[derive(Debug, Clone)]
pub struct GraphEdge {
    pub id: i64,
    pub source: Uuid,
    pub target: Uuid,
    pub relation: RelationType,
    pub weight: f32,
    pub properties: HashMap<String, String>,
}

// ─────────────────────────────────────────────────────────────────────────────
// KNOWLEDGE GRAPH
// ─────────────────────────────────────────────────────────────────────────────

/// Bilgi grafiği
pub struct KnowledgeGraph {
    conn: Connection,
}

impl KnowledgeGraph {
    /// Yeni bilgi grafiği oluştur
    pub fn new(db_path: &str) -> MemoryResult<Self> {
        if let Some(parent) = Path::new(db_path).parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    MemoryError::DatabaseError(format!("Dizin oluşturulamadı: {}", e))
                })?;
            }
        }
        
        let conn = Connection::open(db_path).map_err(|e| {
            MemoryError::DatabaseError(format!("SQLite bağlantısı kurulamadı: {}", e))
        })?;
        
        let mut graph = Self { conn };
        graph.initialize_schema()?;
        
        log::info!("🕸️  GRAFİK: Bilgi grafiği başlatıldı");
        Ok(graph)
    }
    
    /// Şema oluştur
    fn initialize_schema(&mut self) -> MemoryResult<()> {
        self.conn.execute_batch(
            "
            -- Düğüm tablosu
            CREATE TABLE IF NOT EXISTS graph_nodes (
                id TEXT PRIMARY KEY,
                label TEXT NOT NULL,
                memory_type TEXT NOT NULL,
                properties TEXT DEFAULT '{}',
                created_at TEXT NOT NULL
            );
            
            -- Kenar tablosu (ilişkiler)
            CREATE TABLE IF NOT EXISTS graph_edges (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                source_id TEXT NOT NULL,
                target_id TEXT NOT NULL,
                relation TEXT NOT NULL,
                weight REAL DEFAULT 1.0,
                properties TEXT DEFAULT '{}',
                created_at TEXT NOT NULL,
                UNIQUE(source_id, target_id, relation)
            );
            
            -- İndeksler
            CREATE INDEX IF NOT EXISTS idx_nodes_type ON graph_nodes(memory_type);
            CREATE INDEX IF NOT EXISTS idx_edges_source ON graph_edges(source_id);
            CREATE INDEX IF NOT EXISTS idx_edges_target ON graph_edges(target_id);
            CREATE INDEX IF NOT EXISTS idx_edges_relation ON graph_edges(relation);
            "
        ).map_err(|e| MemoryError::DatabaseError(format!("Şema hatası: {}", e)))?;
        
        Ok(())
    }
    
    /// Düğüm ekle
    pub fn add_node(&self, memory: &MemoryEntry) -> MemoryResult<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO graph_nodes (id, label, memory_type, properties, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                memory.id.to_string(),
                memory.content.chars().take(100).collect::<String>(),
                serde_json::to_string(&memory.memory_type).unwrap_or_default(),
                serde_json::to_string(&memory.metadata).unwrap_or("{}".into()),
                memory.created_at.to_rfc3339(),
            ]
        ).map_err(|e| MemoryError::DatabaseError(format!("Düğüm ekleme hatası: {}", e)))?;
        
        Ok(())
    }
    
    /// İlişki ekle
    pub fn add_edge(
        &self,
        source: Uuid,
        target: Uuid,
        relation: RelationType,
        weight: Option<f32>,
    ) -> MemoryResult<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO graph_edges (source_id, target_id, relation, weight, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                source.to_string(),
                target.to_string(),
                serde_json::to_string(&relation).unwrap_or_default(),
                weight.unwrap_or(1.0),
                chrono::Utc::now().to_rfc3339(),
            ]
        ).map_err(|e| MemoryError::DatabaseError(format!("Kenar ekleme hatası: {}", e)))?;
        
        log::debug!("🕸️  Yeni ilişki: {} → {} ({})", source, target, relation);
        Ok(())
    }
    
    /// İlişkileri getir (giden)
    pub fn get_outgoing_edges(&self, node_id: Uuid) -> MemoryResult<Vec<GraphEdge>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, source_id, target_id, relation, weight, properties
             FROM graph_edges WHERE source_id = ?1"
        ).map_err(|e| MemoryError::DatabaseError(format!("Sorgu hatası: {}", e)))?;
        
        let edges = stmt.query_map(
            params![node_id.to_string()],
            |row| {
                Ok(GraphEdge {
                    id: row.get(0)?,
                    source: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap_or_default(),
                    target: Uuid::parse_str(&row.get::<_, String>(2)?).unwrap_or_default(),
                    relation: serde_json::from_str(&row.get::<_, String>(3)?).unwrap_or(RelationType::RelatedTo),
                    weight: row.get(4)?,
                    properties: serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_default(),
                })
            }
        ).map_err(|e| MemoryError::DatabaseError(format!("Kenar okuma hatası: {}", e)))?
        .filter_map(|e| e.ok())
        .collect();
        
        Ok(edges)
    }
    
    /// İlişkileri getir (gelen)
    pub fn get_incoming_edges(&self, node_id: Uuid) -> MemoryResult<Vec<GraphEdge>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, source_id, target_id, relation, weight, properties
             FROM graph_edges WHERE target_id = ?1"
        ).map_err(|e| MemoryError::DatabaseError(format!("Sorgu hatası: {}", e)))?;
        
        let edges = stmt.query_map(
            params![node_id.to_string()],
            |row| {
                Ok(GraphEdge {
                    id: row.get(0)?,
                    source: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap_or_default(),
                    target: Uuid::parse_str(&row.get::<_, String>(2)?).unwrap_or_default(),
                    relation: serde_json::from_str(&row.get::<_, String>(3)?).unwrap_or(RelationType::RelatedTo),
                    weight: row.get(4)?,
                    properties: serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_default(),
                })
            }
        ).map_err(|e| MemoryError::DatabaseError(format!("Kenar okuma hatası: {}", e)))?
        .filter_map(|e| e.ok())
        .collect();
        
        Ok(edges)
    }
    
    /// İlişkiye göre getir
    pub fn get_related_by_type(
        &self,
        node_id: Uuid,
        relation: RelationType,
    ) -> MemoryResult<Vec<GraphEdge>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, source_id, target_id, relation, weight, properties
             FROM graph_edges WHERE source_id = ?1 AND relation = ?2"
        ).map_err(|e| MemoryError::DatabaseError(format!("Sorgu hatası: {}", e)))?;
        
        let edges = stmt.query_map(
            params![node_id.to_string(), serde_json::to_string(&relation).unwrap_or_default()],
            |row| {
                Ok(GraphEdge {
                    id: row.get(0)?,
                    source: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap_or_default(),
                    target: Uuid::parse_str(&row.get::<_, String>(2)?).unwrap_or_default(),
                    relation: serde_json::from_str(&row.get::<_, String>(3)?).unwrap_or(RelationType::RelatedTo),
                    weight: row.get(4)?,
                    properties: serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_default(),
                })
            }
        ).map_err(|e| MemoryError::DatabaseError(format!("Kenar okuma hatası: {}", e)))?
        .filter_map(|e| e.ok())
        .collect();
        
        Ok(edges)
    }
    
    /// Komşuları getir
    pub fn get_neighbors(&self, node_id: Uuid) -> MemoryResult<Vec<Uuid>> {
        let mut stmt = self.conn.prepare(
            "SELECT target_id FROM graph_edges WHERE source_id = ?1
             UNION
             SELECT source_id FROM graph_edges WHERE target_id = ?1"
        ).map_err(|e| MemoryError::DatabaseError(format!("Sorgu hatası: {}", e)))?;
        
        let neighbors: Vec<Uuid> = stmt.query_map(
            params![node_id.to_string()],
            |row| {
                let id_str: String = row.get(0)?;
                Ok(Uuid::parse_str(&id_str).unwrap_or_default())
            }
        ).map_err(|e| MemoryError::DatabaseError(format!("Komşu okuma hatası: {}", e)))?
        .filter_map(|n| n.ok())
        .collect();
        
        Ok(neighbors)
    }
    
    /// Graf yürüme (BFS)
    pub fn traverse_bfs(&self, start: Uuid, max_depth: usize) -> MemoryResult<Vec<Uuid>> {
        use std::collections::VecDeque;
        
        let mut visited = std::collections::HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        
        queue.push_back((start, 0));
        visited.insert(start);
        
        while let Some((node, depth)) = queue.pop_front() {
            if depth > max_depth {
                continue;
            }
            
            result.push(node);
            
            let neighbors = self.get_neighbors(node)?;
            for neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back((neighbor, depth + 1));
                }
            }
        }
        
        Ok(result)
    }
    
    /// En kısa yol (BFS)
    pub fn shortest_path(&self, from: Uuid, to: Uuid) -> MemoryResult<Option<Vec<Uuid>>> {
        use std::collections::VecDeque;
        
        let mut visited = std::collections::HashMap::new();
        let mut queue = VecDeque::new();
        
        queue.push_back(from);
        visited.insert(from, None);
        
        while let Some(current) = queue.pop_front() {
            if current == to {
                // Yolu geri izle
                let mut path = vec![to];
                let mut node = to;
                while let Some(&Some(prev)) = visited.get(&node) {
                    path.push(prev);
                    node = prev;
                }
                path.reverse();
                return Ok(Some(path));
            }
            
            let neighbors = self.get_neighbors(current)?;
            for neighbor in neighbors {
                if !visited.contains_key(&neighbor) {
                    visited.insert(neighbor, Some(current));
                    queue.push_back(neighbor);
                }
            }
        }
        
        Ok(None)
    }
    
    /// İlişki sayısı
    pub fn edge_count(&self) -> MemoryResult<usize> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM graph_edges",
            [],
            |row| row.get(0)
        ).map_err(|e| MemoryError::DatabaseError(format!("Sayım hatası: {}", e)))?;
        
        Ok(count as usize)
    }
    
    /// Düğüm sayısı
    pub fn node_count(&self) -> MemoryResult<usize> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM graph_nodes",
            [],
            |row| row.get(0)
        ).map_err(|e| MemoryError::DatabaseError(format!("Sayım hatası: {}", e)))?;
        
        Ok(count as usize)
    }
    
    /// Düğümü sil
    pub fn delete_node(&self, node_id: Uuid) -> MemoryResult<bool> {
        // Önce ilişkileri sil
        self.conn.execute(
            "DELETE FROM graph_edges WHERE source_id = ?1 OR target_id = ?1",
            params![node_id.to_string()]
        ).map_err(|e| MemoryError::DatabaseError(format!("Kenar silme hatası: {}", e)))?;
        
        // Sonra düğümü sil
        let affected = self.conn.execute(
            "DELETE FROM graph_nodes WHERE id = ?1",
            params![node_id.to_string()]
        ).map_err(|e| MemoryError::DatabaseError(format!("Düğüm silme hatası: {}", e)))?;
        
        Ok(affected > 0)
    }
    
    /// İlişkiyi sil
    pub fn delete_edge(&self, source: Uuid, target: Uuid, relation: RelationType) -> MemoryResult<bool> {
        let affected = self.conn.execute(
            "DELETE FROM graph_edges WHERE source_id = ?1 AND target_id = ?2 AND relation = ?3",
            params![
                source.to_string(),
                target.to_string(),
                serde_json::to_string(&relation).unwrap_or_default()
            ]
        ).map_err(|e| MemoryError::DatabaseError(format!("İlişki silme hatası: {}", e)))?;
        
        Ok(affected > 0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ONTOLOGY MANAGER
// ─────────────────────────────────────────────────────────────────────────────

/// Ontoloji yöneticisi
pub struct OntologyManager {
    concepts: HashMap<String, Vec<String>>,
    hierarchies: HashMap<String, String>,
}

impl OntologyManager {
    pub fn new() -> Self {
        let mut concepts = HashMap::new();
        
        // Temel kavramlar
        concepts.insert("yapay_zeka".into(), vec![
            "makine_ogrenimi".into(),
            "derin_ogrenme".into(),
            "nlp".into(),
            "bilgisayarli_gorme".into(),
        ]);
        
        concepts.insert("programlama".into(), vec![
            "rust".into(),
            "python".into(),
            "javascript".into(),
        ]);
        
        concepts.insert("bilgi".into(), vec![
            "fact".into(),
            "teori".into(),
            "hipotez".into(),
        ]);
        
        Self {
            concepts,
            hierarchies: HashMap::new(),
        }
    }
    
    /// Kavram ekle
    pub fn add_concept(&mut self, concept: String, children: Vec<String>) {
        self.concepts.insert(concept, children);
    }
    
    /// Alt kavramları getir
    pub fn get_children(&self, concept: &str) -> Option<&Vec<String>> {
        self.concepts.get(concept)
    }
    
    /// Kavram var mı?
    pub fn has_concept(&self, concept: &str) -> bool {
        self.concepts.contains_key(concept)
    }
    
    /// Benzer kavramları bul
    pub fn find_similar(&self, concept: &str) -> Vec<&str> {
        self.concepts.keys()
            .filter(|k| k.contains(concept) || concept.contains(*k))
            .map(|s| s.as_str())
            .collect()
    }
}

impl Default for OntologyManager {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TESTLER
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    
    fn test_db(name: &str) -> String {
        format!("/tmp/test_graph_{}.db", name)
    }
    
    fn cleanup(path: &str) {
        let _ = fs::remove_file(path);
    }
    
    #[test]
    fn test_graph_creation() {
        let db = test_db("create");
        cleanup(&db);
        {
            let graph = KnowledgeGraph::new(&db).expect("operation failed");
            assert_eq!(graph.node_count().expect("operation failed"), 0);
            assert_eq!(graph.edge_count().expect("operation failed"), 0);
        }
        cleanup(&db);
    }
    
    #[test]
    fn test_add_and_traverse() {
        let db = test_db("traverse");
        cleanup(&db);
        {
            let graph = KnowledgeGraph::new(&db).expect("operation failed");
            
            let id1 = Uuid::new_v4();
            let id2 = Uuid::new_v4();
            let id3 = Uuid::new_v4();
            
            // Düğümler ekle
            let mem1 = MemoryEntry::from_input(
                crate::MemoryInput::new("Düğüm 1").with_type(MemoryType::Semantic)
            );
            let mem2 = MemoryEntry::from_input(
                crate::MemoryInput::new("Düğüm 2").with_type(MemoryType::Semantic)
            );
            let mem3 = MemoryEntry::from_input(
                crate::MemoryInput::new("Düğüm 3").with_type(MemoryType::Semantic)
            );
            
            // Önce düğümleri kaydet
            graph.add_node(&mem1).expect("operation failed");
            graph.add_node(&mem2).expect("operation failed");
            graph.add_node(&mem3).expect("operation failed");
            
            // Kenarlar ekle
            graph.add_edge(id1, id2, RelationType::Causes, None).expect("operation failed");
            graph.add_edge(id2, id3, RelationType::Causes, None).expect("operation failed");
            
            // Yürü
            let path = graph.shortest_path(id1, id3).expect("operation failed");
            assert!(path.is_some());
            let path = path.expect("operation failed");
            assert_eq!(path.len(), 3);
        }
        cleanup(&db);
    }
    
    #[test]
    fn test_ontology_manager() {
        let mut onto = OntologyManager::new();
        
        onto.add_concept("test".into(), vec!["alt1".into(), "alt2".into()]);
        
        assert!(onto.has_concept("test"));
        assert!(onto.get_children("test").is_some());
        
        let similar = onto.find_similar("yapay");
        assert!(!similar.is_empty());
    }
}
