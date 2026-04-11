//! ─── Backend Module ───
//!
//! Backend implementations for Knowledge Graph storage.

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::{Entity, EntityQuery, KnowledgeError, Relation};

// ─── Knowledge Backend Trait ───

/// Trait for knowledge graph backends
#[async_trait]
pub trait KnowledgeBackend: Send + Sync {
    // ─── Entity Operations ───

    /// Create an entity
    async fn create_entity(&self, entity: Entity) -> Result<(), KnowledgeError>;

    /// Read an entity by ID
    async fn read_entity(&self, id: Uuid) -> Result<Option<Entity>, KnowledgeError>;

    /// Update an entity
    async fn update_entity(&self, entity: Entity) -> Result<(), KnowledgeError>;

    /// Delete an entity
    async fn delete_entity(&self, id: Uuid) -> Result<(), KnowledgeError>;

    /// Find entities by type
    async fn find_entities_by_type(&self, entity_type: &str) -> Result<Vec<Entity>, KnowledgeError>;

    /// Query entities
    async fn query_entities(&self, query: EntityQuery) -> Result<Vec<Entity>, KnowledgeError>;

    /// Count entities
    async fn entity_count(&self) -> Result<u64, KnowledgeError>;

    // ─── Relation Operations ───

    /// Create a relation
    async fn create_relation(&self, relation: Relation) -> Result<(), KnowledgeError>;

    /// Read a relation by ID
    async fn read_relation(&self, id: Uuid) -> Result<Option<Relation>, KnowledgeError>;

    /// Delete a relation
    async fn delete_relation(&self, id: Uuid) -> Result<(), KnowledgeError>;

    /// Find relations from an entity
    async fn find_relations_from(&self, entity_id: Uuid) -> Result<Vec<Relation>, KnowledgeError>;

    /// Find relations to an entity
    async fn find_relations_to(&self, entity_id: Uuid) -> Result<Vec<Relation>, KnowledgeError>;

    /// Find relations by type
    async fn find_relations_by_type(&self, relation_type: &str) -> Result<Vec<Relation>, KnowledgeError>;

    /// Count relations
    async fn relation_count(&self) -> Result<u64, KnowledgeError>;

    // ─── Graph Operations ───

    /// Clear all data
    async fn clear(&self) -> Result<(), KnowledgeError>;
}

// ─── In-Memory Backend ───

/// In-memory backend for testing and development
pub struct InMemoryBackend {
    entities: parking_lot::RwLock<HashMap<Uuid, Entity>>,
    relations: parking_lot::RwLock<HashMap<Uuid, Relation>>,
    entity_type_index: parking_lot::RwLock<HashMap<String, Vec<Uuid>>>,
    relation_type_index: parking_lot::RwLock<HashMap<String, Vec<Uuid>>>,
    from_index: parking_lot::RwLock<HashMap<Uuid, Vec<Uuid>>>,
    to_index: parking_lot::RwLock<HashMap<Uuid, Vec<Uuid>>>,
}

impl InMemoryBackend {
    pub fn new() -> Self {
        Self {
            entities: parking_lot::RwLock::new(HashMap::new()),
            relations: parking_lot::RwLock::new(HashMap::new()),
            entity_type_index: parking_lot::RwLock::new(HashMap::new()),
            relation_type_index: parking_lot::RwLock::new(HashMap::new()),
            from_index: parking_lot::RwLock::new(HashMap::new()),
            to_index: parking_lot::RwLock::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl KnowledgeBackend for InMemoryBackend {
    // ─── Entity Operations ───

    async fn create_entity(&self, entity: Entity) -> Result<(), KnowledgeError> {
        let id = entity.id;
        let entity_type = entity.entity_type.to_string();
        
        // Add to main storage
        self.entities.write().insert(id, entity);
        
        // Add to type index
        self.entity_type_index
            .write()
            .entry(entity_type)
            .or_insert_with(Vec::new)
            .push(id);
        
        Ok(())
    }

    async fn read_entity(&self, id: Uuid) -> Result<Option<Entity>, KnowledgeError> {
        Ok(self.entities.read().get(&id).cloned())
    }

    async fn update_entity(&self, entity: Entity) -> Result<(), KnowledgeError> {
        let id = entity.id;
        if self.entities.read().contains_key(&id) {
            self.entities.write().insert(id, entity);
            Ok(())
        } else {
            Err(KnowledgeError::EntityNotFound(id))
        }
    }

    async fn delete_entity(&self, id: Uuid) -> Result<(), KnowledgeError> {
        // Remove from main storage
        if self.entities.write().remove(&id).is_none() {
            return Err(KnowledgeError::EntityNotFound(id));
        }
        
        // Remove from type index
        let mut type_index = self.entity_type_index.write();
        for (_, ids) in type_index.iter_mut() {
            ids.retain(|&x| x != id);
        }
        
        // Remove related relations
        let mut relations = self.relations.write();
        let rel_ids_to_remove: Vec<Uuid> = relations
            .iter()
            .filter(|(_, r)| r.from_id == id || r.to_id == id)
            .map(|(&rid, _)| rid)
            .collect();
        
        for rid in rel_ids_to_remove {
            relations.remove(&rid);
        }
        
        Ok(())
    }

    async fn find_entities_by_type(&self, entity_type: &str) -> Result<Vec<Entity>, KnowledgeError> {
        let ids = self
            .entity_type_index
            .read()
            .get(entity_type)
            .cloned()
            .unwrap_or_default();
        
        let entities = self.entities.read();
        Ok(ids.iter().filter_map(|id| entities.get(id).cloned()).collect())
    }

    async fn query_entities(&self, query: EntityQuery) -> Result<Vec<Entity>, KnowledgeError> {
        let entities = self.entities.read();
        let mut results: Vec<Entity> = entities
            .values()
            .filter(|e| e.matches(&query))
            .cloned()
            .collect();
        
        // Apply skip
        if let Some(skip) = query.skip {
            results = results.into_iter().skip(skip).collect();
        }
        
        // Apply limit
        if let Some(limit) = query.limit {
            results = results.into_iter().take(limit).collect();
        }
        
        Ok(results)
    }

    async fn entity_count(&self) -> Result<u64, KnowledgeError> {
        Ok(self.entities.read().len() as u64)
    }

    // ─── Relation Operations ───

    async fn create_relation(&self, relation: Relation) -> Result<(), KnowledgeError> {
        let id = relation.id;
        let from_id = relation.from_id;
        let to_id = relation.to_id;
        let relation_type = relation.relation_type.clone();
        
        // Verify entities exist
        if !self.entities.read().contains_key(&from_id) {
            return Err(KnowledgeError::EntityNotFound(from_id));
        }
        if !self.entities.read().contains_key(&to_id) {
            return Err(KnowledgeError::EntityNotFound(to_id));
        }
        
        // Add to main storage
        self.relations.write().insert(id, relation);
        
        // Add to type index
        self.relation_type_index
            .write()
            .entry(relation_type)
            .or_insert_with(Vec::new)
            .push(id);
        
        // Add to from index
        self.from_index
            .write()
            .entry(from_id)
            .or_insert_with(Vec::new)
            .push(id);
        
        // Add to to index
        self.to_index
            .write()
            .entry(to_id)
            .or_insert_with(Vec::new)
            .push(id);
        
        Ok(())
    }

    async fn read_relation(&self, id: Uuid) -> Result<Option<Relation>, KnowledgeError> {
        Ok(self.relations.read().get(&id).cloned())
    }

    async fn delete_relation(&self, id: Uuid) -> Result<(), KnowledgeError> {
        if let Some(relation) = self.relations.write().remove(&id) {
            // Remove from type index
            if let Some(ids) = self.relation_type_index.write().get_mut(&relation.relation_type) {
                ids.retain(|&x| x != id);
            }
            
            // Remove from from index
            if let Some(ids) = self.from_index.write().get_mut(&relation.from_id) {
                ids.retain(|&x| x != id);
            }
            
            // Remove from to index
            if let Some(ids) = self.to_index.write().get_mut(&relation.to_id) {
                ids.retain(|&x| x != id);
            }
            
            Ok(())
        } else {
            Err(KnowledgeError::RelationNotFound(id))
        }
    }

    async fn find_relations_from(&self, entity_id: Uuid) -> Result<Vec<Relation>, KnowledgeError> {
        let ids = self
            .from_index
            .read()
            .get(&entity_id)
            .cloned()
            .unwrap_or_default();
        
        let relations = self.relations.read();
        Ok(ids.iter().filter_map(|id| relations.get(id).cloned()).collect())
    }

    async fn find_relations_to(&self, entity_id: Uuid) -> Result<Vec<Relation>, KnowledgeError> {
        let ids = self
            .to_index
            .read()
            .get(&entity_id)
            .cloned()
            .unwrap_or_default();
        
        let relations = self.relations.read();
        Ok(ids.iter().filter_map(|id| relations.get(id).cloned()).collect())
    }

    async fn find_relations_by_type(&self, relation_type: &str) -> Result<Vec<Relation>, KnowledgeError> {
        let ids = self
            .relation_type_index
            .read()
            .get(relation_type)
            .cloned()
            .unwrap_or_default();
        
        let relations = self.relations.read();
        Ok(ids.iter().filter_map(|id| relations.get(id).cloned()).collect())
    }

    async fn relation_count(&self) -> Result<u64, KnowledgeError> {
        Ok(self.relations.read().len() as u64)
    }

    async fn clear(&self) -> Result<(), KnowledgeError> {
        self.entities.write().clear();
        self.relations.write().clear();
        self.entity_type_index.write().clear();
        self.relation_type_index.write().clear();
        self.from_index.write().clear();
        self.to_index.write().clear();
        Ok(())
    }
}

// ─── Neo4j Backend ───

/// Neo4j backend for production use
pub struct Neo4jBackend {
    client: Option<neo4rs::Graph>,
    config: Neo4jConfig,
}

#[derive(Debug, Clone)]
pub struct Neo4jConfig {
    pub uri: String,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl Default for Neo4jConfig {
    fn default() -> Self {
        Self {
            uri: "bolt://localhost:7687".to_string(),
            username: "neo4j".to_string(),
            password: "password".to_string(),
            database: "neo4j".to_string(),
        }
    }
}

impl Neo4jBackend {
    pub fn new(config: Neo4jConfig) -> Self {
        Self {
            client: None,
            config,
        }
    }

    pub async fn connect(
        uri: &str,
        username: &str,
        password: &str,
    ) -> Result<Self, KnowledgeError> {
        let config = Neo4jConfig {
            uri: uri.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            database: "neo4j".to_string(),
        };

        let client = neo4rs::Graph::new(&config.uri, &config.username, &config.password)
            .await
            .map_err(|e| KnowledgeError::ConnectionError(e.to_string()))?;

        Ok(Self {
            client: Some(client),
            config,
        })
    }

    pub async fn is_connected(&self) -> bool {
        self.client.is_some()
    }
}

#[async_trait]
impl KnowledgeBackend for Neo4jBackend {
    async fn create_entity(&self, entity: Entity) -> Result<(), KnowledgeError> {
        let client = self.client.as_ref()
            .ok_or_else(|| KnowledgeError::NotConnected)?;

        let query = neo4rs::query(
            "CREATE (e:Entity {id: $id, name: $name, type: $type, description: $description, properties: $properties, tags: $tags, confidence: $confidence, source: $source, created_at: $created_at, updated_at: $updated_at})"
        )
        .param("id", entity.id.to_string())
        .param("name", entity.name.clone())
        .param("type", entity.entity_type.to_string())
        .param("description", entity.description.clone())
        .param("properties", serde_json::to_string(&entity.properties).unwrap_or_default())
        .param("tags", serde_json::to_string(&entity.tags).unwrap_or_default())
        .param("confidence", entity.confidence)
        .param("source", entity.source.clone().unwrap_or_default())
        .param("created_at", entity.created_at.to_rfc3339())
        .param("updated_at", entity.updated_at.to_rfc3339());

        client.run(query).await
            .map_err(|e| KnowledgeError::QueryError(e.to_string()))?;

        Ok(())
    }

    async fn read_entity(&self, id: Uuid) -> Result<Option<Entity>, KnowledgeError> {
        // For now, return None - full implementation would query Neo4j
        log::debug!("Neo4j read_entity called for {}", id);
        Ok(None)
    }

    async fn update_entity(&self, entity: Entity) -> Result<(), KnowledgeError> {
        let client = self.client.as_ref()
            .ok_or_else(|| KnowledgeError::NotConnected)?;

        let query = neo4rs::query(
            "MATCH (e:Entity {id: $id}) SET e.name = $name, e.description = $description, e.updated_at = $updated_at"
        )
        .param("id", entity.id.to_string())
        .param("name", entity.name.clone())
        .param("description", entity.description.clone())
        .param("updated_at", entity.updated_at.to_rfc3339());

        client.run(query).await
            .map_err(|e| KnowledgeError::QueryError(e.to_string()))?;

        Ok(())
    }

    async fn delete_entity(&self, id: Uuid) -> Result<(), KnowledgeError> {
        let client = self.client.as_ref()
            .ok_or_else(|| KnowledgeError::NotConnected)?;

        let query = neo4rs::query(
            "MATCH (e:Entity {id: $id}) DETACH DELETE e"
        )
        .param("id", id.to_string());

        client.run(query).await
            .map_err(|e| KnowledgeError::QueryError(e.to_string()))?;

        Ok(())
    }

    async fn find_entities_by_type(&self, entity_type: &str) -> Result<Vec<Entity>, KnowledgeError> {
        let client = self.client.as_ref()
            .ok_or_else(|| KnowledgeError::NotConnected)?;

        let query = neo4rs::query(
            "MATCH (e:Entity {type: $type}) RETURN e"
        )
        .param("type", entity_type.to_string());

        let mut result = client.execute(query).await
            .map_err(|e| KnowledgeError::QueryError(e.to_string()))?;

        let mut entities = Vec::new();
        while let Ok(Some(row)) = result.next().await {
            // Parse entity from row
            // This is simplified - full implementation would parse all fields
            log::debug!("Found entity row: {:?}", row);
        }

        Ok(entities)
    }

    async fn query_entities(&self, _query: EntityQuery) -> Result<Vec<Entity>, KnowledgeError> {
        // Simplified - full implementation would translate EntityQuery to Cypher
        Ok(Vec::new())
    }

    async fn entity_count(&self) -> Result<u64, KnowledgeError> {
        let client = self.client.as_ref()
            .ok_or_else(|| KnowledgeError::NotConnected)?;

        let query = neo4rs::query("MATCH (e:Entity) RETURN count(e) as count");
        let mut result = client.execute(query).await
            .map_err(|e| KnowledgeError::QueryError(e.to_string()))?;

        if let Ok(Some(row)) = result.next().await {
            let count: i64 = row.get("count").unwrap_or(0);
            return Ok(count as u64);
        }

        Ok(0)
    }

    async fn create_relation(&self, relation: Relation) -> Result<(), KnowledgeError> {
        let client = self.client.as_ref()
            .ok_or_else(|| KnowledgeError::NotConnected)?;

        // Create relation with dynamic type
        let query_str = format!(
            "MATCH (from:Entity {{id: $from_id}}), (to:Entity {{id: $to_id}}) CREATE (from)-[r:{} {{id: $id, weight: $weight, confidence: $confidence, source: $source}}]->(to)",
            relation.relation_type
        );

        let query = neo4rs::query(&query_str)
            .param("from_id", relation.from_id.to_string())
            .param("to_id", relation.to_id.to_string())
            .param("id", relation.id.to_string())
            .param("weight", relation.weight)
            .param("confidence", relation.confidence)
            .param("source", relation.source.clone().unwrap_or_default());

        client.run(query).await
            .map_err(|e| KnowledgeError::QueryError(e.to_string()))?;

        Ok(())
    }

    async fn read_relation(&self, id: Uuid) -> Result<Option<Relation>, KnowledgeError> {
        log::debug!("Neo4j read_relation called for {}", id);
        Ok(None)
    }

    async fn delete_relation(&self, id: Uuid) -> Result<(), KnowledgeError> {
        let client = self.client.as_ref()
            .ok_or_else(|| KnowledgeError::NotConnected)?;

        let query = neo4rs::query(
            "MATCH ()-[r {id: $id}]->() DELETE r"
        )
        .param("id", id.to_string());

        client.run(query).await
            .map_err(|e| KnowledgeError::QueryError(e.to_string()))?;

        Ok(())
    }

    async fn find_relations_from(&self, entity_id: Uuid) -> Result<Vec<Relation>, KnowledgeError> {
        let client = self.client.as_ref()
            .ok_or_else(|| KnowledgeError::NotConnected)?;

        let query = neo4rs::query(
            "MATCH (from:Entity {id: $from_id})-[r]->(to) RETURN r, to.id as to_id"
        )
        .param("from_id", entity_id.to_string());

        let mut result = client.execute(query).await
            .map_err(|e| KnowledgeError::QueryError(e.to_string()))?;

        let mut relations = Vec::new();
        while let Ok(Some(row)) = result.next().await {
            log::debug!("Found relation row: {:?}", row);
        }

        Ok(relations)
    }

    async fn find_relations_to(&self, entity_id: Uuid) -> Result<Vec<Relation>, KnowledgeError> {
        let client = self.client.as_ref()
            .ok_or_else(|| KnowledgeError::NotConnected)?;

        let query = neo4rs::query(
            "MATCH (from)-[r]->(to:Entity {id: $to_id}) RETURN r, from.id as from_id"
        )
        .param("to_id", entity_id.to_string());

        let mut result = client.execute(query).await
            .map_err(|e| KnowledgeError::QueryError(e.to_string()))?;

        let mut relations = Vec::new();
        while let Ok(Some(row)) = result.next().await {
            log::debug!("Found relation row: {:?}", row);
        }

        Ok(relations)
    }

    async fn find_relations_by_type(&self, relation_type: &str) -> Result<Vec<Relation>, KnowledgeError> {
        let client = self.client.as_ref()
            .ok_or_else(|| KnowledgeError::NotConnected)?;

        let query_str = format!(
            "MATCH ()-[r:{}]->() RETURN r",
            relation_type
        );

        let query = neo4rs::query(&query_str);
        let mut result = client.execute(query).await
            .map_err(|e| KnowledgeError::QueryError(e.to_string()))?;

        let mut relations = Vec::new();
        while let Ok(Some(row)) = result.next().await {
            log::debug!("Found relation row: {:?}", row);
        }

        Ok(relations)
    }

    async fn relation_count(&self) -> Result<u64, KnowledgeError> {
        let client = self.client.as_ref()
            .ok_or_else(|| KnowledgeError::NotConnected)?;

        let query = neo4rs::query("MATCH ()-[r]->() RETURN count(r) as count");
        let mut result = client.execute(query).await
            .map_err(|e| KnowledgeError::QueryError(e.to_string()))?;

        if let Ok(Some(row)) = result.next().await {
            let count: i64 = row.get("count").unwrap_or(0);
            return Ok(count as u64);
        }

        Ok(0)
    }

    async fn clear(&self) -> Result<(), KnowledgeError> {
        let client = self.client.as_ref()
            .ok_or_else(|| KnowledgeError::NotConnected)?;

        let query = neo4rs::query("MATCH (n) DETACH DELETE n");
        client.run(query).await
            .map_err(|e| KnowledgeError::QueryError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EntityType, RelationQuery};

    #[tokio::test]
    async fn test_in_memory_backend_entity() {
        let backend = InMemoryBackend::new();
        
        let entity = Entity::new("Test Entity")
            .with_type(EntityType::Concept)
            .with_description("Test description");

        // Create
        backend.create_entity(entity.clone()).await.unwrap();
        
        // Read
        let retrieved = backend.read_entity(entity.id).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Test Entity");

        // Count
        let count = backend.entity_count().await.unwrap();
        assert_eq!(count, 1);

        // Delete
        backend.delete_entity(entity.id).await.unwrap();
        
        let count = backend.entity_count().await.unwrap();
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn test_in_memory_backend_relation() {
        let backend = InMemoryBackend::new();
        
        let entity1 = Entity::new("Entity 1").with_type(EntityType::Concept);
        let entity2 = Entity::new("Entity 2").with_type(EntityType::Concept);
        
        backend.create_entity(entity1.clone()).await.unwrap();
        backend.create_entity(entity2.clone()).await.unwrap();
        
        let relation = Relation::new(entity1.id, entity2.id, "relates_to");
        
        // Create
        backend.create_relation(relation.clone()).await.unwrap();
        
        // Find from
        let from_relations = backend.find_relations_from(entity1.id).await.unwrap();
        assert_eq!(from_relations.len(), 1);

        // Find to
        let to_relations = backend.find_relations_to(entity2.id).await.unwrap();
        assert_eq!(to_relations.len(), 1);

        // Count
        let count = backend.relation_count().await.unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_in_memory_backend_query() {
        let backend = InMemoryBackend::new();
        
        let entity1 = Entity::new("Rust Programming")
            .with_type(EntityType::Topic)
            .with_tag("programming");
        
        let entity2 = Entity::new("Python Programming")
            .with_type(EntityType::Topic)
            .with_tag("programming");
        
        let entity3 = Entity::new("John Doe")
            .with_type(EntityType::Person);

        backend.create_entity(entity1.clone()).await.unwrap();
        backend.create_entity(entity2.clone()).await.unwrap();
        backend.create_entity(entity3.clone()).await.unwrap();

        // Query by type
        let query = EntityQuery::new().with_type(EntityType::Topic);
        let results = backend.query_entities(query).await.unwrap();
        assert_eq!(results.len(), 2);

        // Query by name
        let query = EntityQuery::new().with_name("Rust");
        let results = backend.query_entities(query).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Rust Programming");
    }
}
