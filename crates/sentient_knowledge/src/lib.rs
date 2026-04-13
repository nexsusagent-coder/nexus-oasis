//! ─── SENTIENT KNOWLEDGE ───
//!
//! Knowledge Graph Engine with Neo4j and Graph RAG support.
//!
//! Features:
//! - Entity management (create, read, update, delete)
//! - Relation management with types and properties
//! - Graph traversal and pathfinding
//! - Graph RAG for contextual retrieval
//! - Neo4j backend support

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
//! - In-memory backend for testing

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

pub mod backend;
pub mod entity;
pub mod relation;
pub mod query;
pub mod rag;
pub mod error;

pub use entity::*;
pub use relation::*;
pub use query::*;
pub use rag::*;
pub use error::*;
pub use backend::{KnowledgeBackend, InMemoryBackend, Neo4jBackend};

// ─── Knowledge Graph ───

/// Main Knowledge Graph structure
pub struct KnowledgeGraph {
    backend: Arc<dyn KnowledgeBackend>,
    name: String,
    stats: RwLock<GraphStats>,
}

use parking_lot::RwLock;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GraphStats {
    pub total_entities: u64,
    pub total_relations: u64,
    pub queries_executed: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

impl KnowledgeGraph {
    /// Create a new Knowledge Graph with the given backend
    pub fn new(name: impl Into<String>, backend: Arc<dyn KnowledgeBackend>) -> Self {
        Self {
            backend,
            name: name.into(),
            stats: RwLock::new(GraphStats::default()),
        }
    }

    /// Create an in-memory Knowledge Graph (for testing)
    pub fn in_memory(name: impl Into<String>) -> Self {
        Self::new(name, Arc::new(InMemoryBackend::new()))
    }

    /// Connect to Neo4j database
    pub async fn neo4j(
        name: impl Into<String>,
        uri: &str,
        username: &str,
        password: &str,
    ) -> Result<Self, KnowledgeError> {
        let backend = Neo4jBackend::connect(uri, username, password).await?;
        Ok(Self::new(name, Arc::new(backend)))
    }

    // ─── Entity Operations ───

    /// Add an entity to the graph
    pub async fn add_entity(&self, entity: Entity) -> Result<Uuid, KnowledgeError> {
        let id = entity.id;
        let name = entity.name.clone();
        self.backend.create_entity(entity).await?;
        
        let mut stats = self.stats.write();
        stats.total_entities += 1;
        
        log::info!("🧠  KNOWLEDGE: Entity created → {} ({})", id, name);
        Ok(id)
    }

    /// Get an entity by ID
    pub async fn get_entity(&self, id: Uuid) -> Result<Option<Entity>, KnowledgeError> {
        self.backend.read_entity(id).await
    }

    /// Update an entity
    pub async fn update_entity(&self, entity: Entity) -> Result<(), KnowledgeError> {
        self.backend.update_entity(entity.clone()).await?;
        log::info!("🧠  KNOWLEDGE: Entity updated → {} ({})", entity.id, entity.name);
        Ok(())
    }

    /// Delete an entity
    pub async fn delete_entity(&self, id: Uuid) -> Result<(), KnowledgeError> {
        self.backend.delete_entity(id).await?;
        
        let mut stats = self.stats.write();
        stats.total_entities = stats.total_entities.saturating_sub(1);
        
        log::info!("🧠  KNOWLEDGE: Entity deleted → {}", id);
        Ok(())
    }

    /// Find entities by type
    pub async fn find_entities_by_type(&self, entity_type: &str) -> Result<Vec<Entity>, KnowledgeError> {
        self.backend.find_entities_by_type(entity_type).await
    }

    /// Find entities by property
    pub async fn find_entities(&self, query: EntityQuery) -> Result<Vec<Entity>, KnowledgeError> {
        let results = self.backend.query_entities(query).await?;
        
        let mut stats = self.stats.write();
        stats.queries_executed += 1;
        
        Ok(results)
    }

    // ─── Relation Operations ───

    /// Add a relation between entities
    pub async fn add_relation(&self, relation: Relation) -> Result<Uuid, KnowledgeError> {
        let id = relation.id;
        self.backend.create_relation(relation.clone()).await?;
        
        let mut stats = self.stats.write();
        stats.total_relations += 1;
        
        log::info!(
            "🧠  KNOWLEDGE: Relation created → {} -[{}]-> {}",
            relation.from_id,
            relation.relation_type,
            relation.to_id
        );
        Ok(id)
    }

    /// Get a relation by ID
    pub async fn get_relation(&self, id: Uuid) -> Result<Option<Relation>, KnowledgeError> {
        self.backend.read_relation(id).await
    }

    /// Delete a relation
    pub async fn delete_relation(&self, id: Uuid) -> Result<(), KnowledgeError> {
        self.backend.delete_relation(id).await?;
        
        let mut stats = self.stats.write();
        stats.total_relations = stats.total_relations.saturating_sub(1);
        
        Ok(())
    }

    /// Find relations from an entity
    pub async fn find_relations_from(&self, entity_id: Uuid) -> Result<Vec<Relation>, KnowledgeError> {
        self.backend.find_relations_from(entity_id).await
    }

    /// Find relations to an entity
    pub async fn find_relations_to(&self, entity_id: Uuid) -> Result<Vec<Relation>, KnowledgeError> {
        self.backend.find_relations_to(entity_id).await
    }

    /// Find relations by type
    pub async fn find_relations_by_type(&self, relation_type: &str) -> Result<Vec<Relation>, KnowledgeError> {
        self.backend.find_relations_by_type(relation_type).await
    }

    // ─── Graph Operations ───

    /// Get the subgraph around an entity (up to depth levels)
    pub async fn subgraph(&self, entity_id: Uuid, depth: u32) -> Result<Subgraph, KnowledgeError> {
        let mut entities = vec![];
        let mut relations = vec![];
        let mut visited = std::collections::HashSet::new();
        
        self.collect_subgraph(entity_id, depth, &mut entities, &mut relations, &mut visited)
            .await?;
        
        Ok(Subgraph { entities, relations })
    }

    fn collect_subgraph<'a>(
        &'a self,
        entity_id: Uuid,
        depth: u32,
        entities: &'a mut Vec<Entity>,
        relations: &'a mut Vec<Relation>,
        visited: &'a mut std::collections::HashSet<Uuid>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), KnowledgeError>> + 'a>> {
        Box::pin(async move {
            if depth == 0 || visited.contains(&entity_id) {
                return Ok(());
            }
            
            visited.insert(entity_id);
            
            // Get entity
            if let Some(entity) = self.get_entity(entity_id).await? {
                entities.push(entity);
            }
            
            // Get outgoing relations
            let outgoing = self.find_relations_from(entity_id).await?;
            for rel in outgoing {
                relations.push(rel.clone());
                Box::pin(self.collect_subgraph(rel.to_id, depth - 1, entities, relations, visited)).await?;
            }
            
            // Get incoming relations
            let incoming = self.find_relations_to(entity_id).await?;
            for rel in incoming {
                relations.push(rel.clone());
                Box::pin(self.collect_subgraph(rel.from_id, depth - 1, entities, relations, visited)).await?;
            }
            
            Ok(())
        })
    }

    /// Find path between two entities
    pub async fn find_path(
        &self,
        from_id: Uuid,
        to_id: Uuid,
        max_depth: u32,
    ) -> Result<Option<Vec<Uuid>>, KnowledgeError> {
        // BFS pathfinding
        let mut queue = vec![(from_id, vec![from_id])];
        let mut visited = std::collections::HashSet::new();
        visited.insert(from_id);
        
        while let Some((current, path)) = queue.pop() {
            if current == to_id {
                return Ok(Some(path));
            }
            
            if path.len() > max_depth as usize {
                continue;
            }
            
            let relations = self.find_relations_from(current).await?;
            for rel in relations {
                if !visited.contains(&rel.to_id) {
                    visited.insert(rel.to_id);
                    let mut new_path = path.clone();
                    new_path.push(rel.to_id);
                    queue.push((rel.to_id, new_path));
                }
            }
        }
        
        Ok(None)
    }

    // ─── Graph RAG ───

    /// Create a Graph RAG engine
    pub fn rag(&self) -> GraphRAG {
        GraphRAG::new(Arc::new(self.clone()))
    }

    // ─── Stats ───

    /// Get graph statistics
    pub fn stats(&self) -> GraphStats {
        self.stats.read().clone()
    }

    /// Get graph name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Count entities
    pub async fn entity_count(&self) -> Result<u64, KnowledgeError> {
        self.backend.entity_count().await
    }

    /// Count relations
    pub async fn relation_count(&self) -> Result<u64, KnowledgeError> {
        self.backend.relation_count().await
    }
}

impl Clone for KnowledgeGraph {
    fn clone(&self) -> Self {
        Self {
            backend: self.backend.clone(),
            name: self.name.clone(),
            stats: RwLock::new(self.stats.read().clone()),
        }
    }
}

// ─── Subgraph ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subgraph {
    pub entities: Vec<Entity>,
    pub relations: Vec<Relation>,
}

impl Subgraph {
    /// Convert to context string for LLM
    pub fn to_context(&self) -> String {
        let mut context = String::new();
        
        context.push_str("=== ENTITIES ===\n");
        for entity in &self.entities {
            context.push_str(&format!("- {} ({:?}): {}\n", entity.name, entity.entity_type, entity.description));
        }
        
        context.push_str("\n=== RELATIONS ===\n");
        for rel in &self.relations {
            context.push_str(&format!("- {} -[{}]-> {}\n", rel.from_id, rel.relation_type, rel.to_id));
        }
        
        context
    }
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_entity() {
        let kg = KnowledgeGraph::in_memory("test");
        
        let entity = Entity::new("Test Entity")
            .with_type(EntityType::Concept)
            .with_description("A test entity");
        
        let id = kg.add_entity(entity).await.expect("Failed to add entity");
        
        let retrieved = kg.get_entity(id).await.expect("Failed to get entity");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Test Entity");
    }

    #[tokio::test]
    async fn test_create_relation() {
        let kg = KnowledgeGraph::in_memory("test");
        
        let entity1 = Entity::new("Entity 1").with_type(EntityType::Concept);
        let entity2 = Entity::new("Entity 2").with_type(EntityType::Concept);
        
        let id1 = kg.add_entity(entity1).await.expect("Failed to add entity");
        let id2 = kg.add_entity(entity2).await.expect("Failed to add entity");
        
        let relation = Relation::new(id1, id2, "relates_to");
        let rel_id = kg.add_relation(relation).await.expect("Failed to add relation");
        
        let retrieved = kg.get_relation(rel_id).await.expect("Failed to get relation");
        assert!(retrieved.is_some());
    }

    #[tokio::test]
    async fn test_subgraph() {
        let kg = KnowledgeGraph::in_memory("test");
        
        let e1 = Entity::new("A").with_type(EntityType::Concept);
        let e2 = Entity::new("B").with_type(EntityType::Concept);
        let e3 = Entity::new("C").with_type(EntityType::Concept);
        
        let id1 = kg.add_entity(e1).await.unwrap();
        let id2 = kg.add_entity(e2).await.unwrap();
        let id3 = kg.add_entity(e3).await.unwrap();
        
        kg.add_relation(Relation::new(id1, id2, "connects")).await.unwrap();
        kg.add_relation(Relation::new(id2, id3, "connects")).await.unwrap();
        
        // Depth 2 should reach all entities: id1 -> id2 -> id3
        let subgraph = kg.subgraph(id1, 3).await.expect("Failed to get subgraph");
        
        assert!(subgraph.entities.len() >= 2, "Expected at least 2 entities, got {}", subgraph.entities.len());
        assert!(subgraph.relations.len() >= 1, "Expected at least 1 relation, got {}", subgraph.relations.len());
    }

    #[tokio::test]
    async fn test_find_path() {
        let kg = KnowledgeGraph::in_memory("test");
        
        let e1 = Entity::new("Start").with_type(EntityType::Concept);
        let e2 = Entity::new("Middle").with_type(EntityType::Concept);
        let e3 = Entity::new("End").with_type(EntityType::Concept);
        
        let id1 = kg.add_entity(e1).await.unwrap();
        let id2 = kg.add_entity(e2).await.unwrap();
        let id3 = kg.add_entity(e3).await.unwrap();
        
        kg.add_relation(Relation::new(id1, id2, "connects")).await.unwrap();
        kg.add_relation(Relation::new(id2, id3, "connects")).await.unwrap();
        
        let path = kg.find_path(id1, id3, 5).await.expect("Failed to find path");
        
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path.len(), 3);
        assert_eq!(path[0], id1);
        assert_eq!(path[2], id3);
    }

    #[tokio::test]
    async fn test_stats() {
        let kg = KnowledgeGraph::in_memory("test");
        
        let e1 = Entity::new("Entity 1").with_type(EntityType::Concept);
        let e2 = Entity::new("Entity 2").with_type(EntityType::Concept);
        
        let id1 = kg.add_entity(e1).await.unwrap();
        let id2 = kg.add_entity(e2).await.unwrap();
        
        kg.add_relation(Relation::new(id1, id2, "relates")).await.unwrap();
        
        let stats = kg.stats();
        assert_eq!(stats.total_entities, 2);
        assert_eq!(stats.total_relations, 1);
    }
}
