//! ─── Graph RAG Module ───
//!
//! Graph-based Retrieval Augmented Generation.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use uuid::Uuid;

use crate::{Entity, EntityType, KnowledgeGraph, Relation, Subgraph};

// ─── Graph RAG ───

/// Graph RAG engine for contextual retrieval
pub struct GraphRAG {
    graph: Arc<KnowledgeGraph>,
    config: RAGConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RAGConfig {
    /// Maximum depth for subgraph extraction
    pub max_depth: u32,
    /// Minimum confidence for entities
    pub min_confidence: f32,
    /// Maximum entities in context
    pub max_entities: usize,
    /// Maximum relations in context
    pub max_relations: usize,
    /// Include entity descriptions
    pub include_descriptions: bool,
    /// Include relation properties
    pub include_properties: bool,
}

impl Default for RAGConfig {
    fn default() -> Self {
        Self {
            max_depth: 3,
            min_confidence: 0.5,
            max_entities: 50,
            max_relations: 100,
            include_descriptions: true,
            include_properties: false,
        }
    }
}

impl GraphRAG {
    pub fn new(graph: Arc<KnowledgeGraph>) -> Self {
        Self {
            graph,
            config: RAGConfig::default(),
        }
    }

    pub fn with_config(mut self, config: RAGConfig) -> Self {
        self.config = config;
        self
    }

    /// Retrieve context for a query
    pub async fn retrieve(&self, query: &str) -> Result<RAGContext, crate::KnowledgeError> {
        // 1. Extract entities from query (simple keyword matching for now)
        let entities = self.extract_entities_from_query(query).await?;
        
        // 2. Get subgraphs for each entity
        let mut all_entities = Vec::new();
        let mut all_relations = Vec::new();
        let mut seen_entities = HashSet::new();
        let mut seen_relations = HashSet::new();
        
        for entity in &entities {
            let subgraph = self.graph.subgraph(entity.id, self.config.max_depth).await?;
            
            for e in subgraph.entities {
                if !seen_entities.contains(&e.id) && e.confidence >= self.config.min_confidence {
                    seen_entities.insert(e.id);
                    all_entities.push(e);
                }
            }
            
            for r in subgraph.relations {
                if !seen_relations.contains(&r.id) && r.confidence >= self.config.min_confidence {
                    seen_relations.insert(r.id);
                    all_relations.push(r);
                }
            }
        }
        
        // 3. Limit results
        all_entities.truncate(self.config.max_entities);
        all_relations.truncate(self.config.max_relations);
        
        // 4. Build context
        let context = self.build_context(all_entities, all_relations, query);
        
        Ok(context)
    }

    /// Retrieve context starting from specific entities
    pub async fn retrieve_from_entities(
        &self,
        entity_ids: &[Uuid],
    ) -> Result<RAGContext, crate::KnowledgeError> {
        let mut all_entities = Vec::new();
        let mut all_relations = Vec::new();
        let mut seen_entities = HashSet::new();
        let mut seen_relations = HashSet::new();
        
        for &entity_id in entity_ids {
            let subgraph = self.graph.subgraph(entity_id, self.config.max_depth).await?;
            
            for e in subgraph.entities {
                if !seen_entities.contains(&e.id) && e.confidence >= self.config.min_confidence {
                    seen_entities.insert(e.id);
                    all_entities.push(e);
                }
            }
            
            for r in subgraph.relations {
                if !seen_relations.contains(&r.id) && r.confidence >= self.config.min_confidence {
                    seen_relations.insert(r.id);
                    all_relations.push(r);
                }
            }
        }
        
        all_entities.truncate(self.config.max_entities);
        all_relations.truncate(self.config.max_relations);
        
        let context = self.build_context(all_entities, all_relations, "");
        
        Ok(context)
    }

    /// Extract entities from query text
    async fn extract_entities_from_query(
        &self,
        query: &str,
    ) -> Result<Vec<Entity>, crate::KnowledgeError> {
        // Simple keyword-based entity matching
        // In production, this would use NER or LLM
        
        let words: Vec<&str> = query
            .split_whitespace()
            .filter(|w| w.len() > 3) // Skip short words
            .collect();
        
        let mut matched_entities = Vec::new();
        
        for word in words {
            let search_query = crate::EntityQuery::new()
                .with_name(word)
                .with_min_confidence(self.config.min_confidence);
            
            let entities = self.graph.find_entities(search_query).await?;
            matched_entities.extend(entities);
        }
        
        // Deduplicate
        let mut seen = HashSet::new();
        matched_entities.retain(|e| seen.insert(e.id));
        
        Ok(matched_entities)
    }

    /// Build context string from entities and relations
    fn build_context(
        &self,
        entities: Vec<Entity>,
        relations: Vec<Relation>,
        query: &str,
    ) -> RAGContext {
        let mut context_str = String::new();
        
        // Add query context
        if !query.is_empty() {
            context_str.push_str(&format!("Query: {}\n\n", query));
        }
        
        // Add entities section
        context_str.push_str("=== KNOWLEDGE ENTITIES ===\n\n");
        
        // Group by type
        let mut by_type: HashMap<EntityType, Vec<&Entity>> = HashMap::new();
        for entity in &entities {
            by_type.entry(entity.entity_type).or_default().push(entity);
        }
        
        for (entity_type, type_entities) in by_type {
            context_str.push_str(&format!("## {}s\n", entity_type));
            
            for entity in type_entities {
                context_str.push_str(&format!("- **{}**", entity.name));
                
                if self.config.include_descriptions && !entity.description.is_empty() {
                    context_str.push_str(&format!(": {}", entity.description));
                }
                
                if !entity.tags.is_empty() {
                    context_str.push_str(&format!(" [{}]", entity.tags.join(", ")));
                }
                
                context_str.push('\n');
            }
            
            context_str.push('\n');
        }
        
        // Add relations section
        context_str.push_str("=== KNOWLEDGE RELATIONS ===\n\n");
        
        // Create entity name lookup
        let entity_names: HashMap<Uuid, &str> = entities
            .iter()
            .map(|e| (e.id, e.name.as_str()))
            .collect();
        
        // Group by relation type
        let mut by_type: HashMap<String, Vec<&Relation>> = HashMap::new();
        for relation in &relations {
            by_type
                .entry(relation.relation_type.clone())
                .or_default()
                .push(relation);
        }
        
        for (relation_type, type_relations) in by_type {
            context_str.push_str(&format!("## {} relations\n", relation_type));
            
            for relation in type_relations {
                let from_name = entity_names
                    .get(&relation.from_id)
                    .map(|s| *s)
                    .unwrap_or("Unknown");
                let to_name = entity_names
                    .get(&relation.to_id)
                    .map(|s| *s)
                    .unwrap_or("Unknown");
                
                context_str.push_str(&format!("- {} → **{}** → {}\n", from_name, relation.relation_type, to_name));
            }
            
            context_str.push('\n');
        }
        
        // Add triplets section (for LLM consumption)
        context_str.push_str("=== KNOWLEDGE TRIPLETS ===\n\n");
        
        for relation in &relations {
            let from_name = entity_names
                .get(&relation.from_id)
                .map(|s| *s)
                .unwrap_or("Unknown");
            let to_name = entity_names
                .get(&relation.to_id)
                .map(|s| *s)
                .unwrap_or("Unknown");
            
            context_str.push_str(&format!(
                "({}, {}, {})\n",
                from_name, relation.relation_type, to_name
            ));
        }
        
        RAGContext {
            text: context_str,
            entities,
            relations,
            query: query.to_string(),
        }
    }
}

// ─── RAG Context ───

/// Result of Graph RAG retrieval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RAGContext {
    /// Context text for LLM
    pub text: String,
    /// Retrieved entities
    pub entities: Vec<Entity>,
    /// Retrieved relations
    pub relations: Vec<Relation>,
    /// Original query
    pub query: String,
}

impl RAGContext {
    /// Get the context text for LLM
    pub fn as_text(&self) -> &str {
        &self.text
    }

    /// Get entity count
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    /// Get relation count
    pub fn relation_count(&self) -> usize {
        self.relations.len()
    }

    /// Get entities of a specific type
    pub fn entities_of_type(&self, entity_type: EntityType) -> Vec<&Entity> {
        self.entities
            .iter()
            .filter(|e| e.entity_type == entity_type)
            .collect()
    }

    /// Get relations of a specific type
    pub fn relations_of_type(&self, relation_type: &str) -> Vec<&Relation> {
        self.relations
            .iter()
            .filter(|r| r.relation_type == relation_type)
            .collect()
    }

    /// Convert to JSON for LLM
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "query": self.query,
            "entities": self.entities.iter().map(|e| serde_json::json!({
                "id": e.id.to_string(),
                "name": e.name,
                "type": e.entity_type.to_string(),
                "description": e.description,
                "tags": e.tags,
            })).collect::<Vec<_>>(),
            "relations": self.relations.iter().map(|r| serde_json::json!({
                "from": r.from_id.to_string(),
                "to": r.to_id.to_string(),
                "type": r.relation_type,
                "weight": r.weight,
            })).collect::<Vec<_>>(),
        })
    }
}

// ─── Entity Extractor ───

/// Entity extractor for building knowledge graph from text
pub struct EntityExtractor {
    patterns: Vec<(EntityType, Vec<&'static str>)>,
}

impl EntityExtractor {
    pub fn new() -> Self {
        Self {
            patterns: vec![
                (EntityType::Person, vec!["person", "user", "developer", "engineer", "manager"]),
                (EntityType::Organization, vec!["company", "team", "organization", "department"]),
                (EntityType::Tool, vec!["tool", "library", "framework", "api", "service"]),
                (EntityType::Concept, vec!["concept", "idea", "theory", "principle"]),
                (EntityType::Topic, vec!["topic", "subject", "category", "tag"]),
            ],
        }
    }

    /// Extract entities from text (simple keyword-based)
    pub fn extract(&self, text: &str) -> Vec<ExtractedEntity> {
        let mut entities = Vec::new();
        let text_lower = text.to_lowercase();
        
        for (entity_type, keywords) in &self.patterns {
            for keyword in keywords {
                if text_lower.contains(keyword) {
                    // Find the word after the keyword
                    if let Some(pos) = text_lower.find(keyword) {
                        let after = &text[pos + keyword.len()..].trim();
                        if let Some(name) = after.split_whitespace().next() {
                            if name.len() > 2 {
                                entities.push(ExtractedEntity {
                                    name: name.to_string(),
                                    entity_type: *entity_type,
                                    confidence: 0.7,
                                });
                            }
                        }
                    }
                }
            }
        }
        
        entities
    }
}

impl Default for EntityExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedEntity {
    pub name: String,
    pub entity_type: EntityType,
    pub confidence: f32,
}

// ─── Relation Extractor ───

/// Relation extractor for building knowledge graph from text
pub struct RelationExtractor {
    patterns: Vec<(&'static str, &'static str)>,
}

impl RelationExtractor {
    pub fn new() -> Self {
        Self {
            patterns: vec![
                ("uses", "uses"),
                ("depends on", "depends_on"),
                ("created by", "created_by"),
                ("part of", "part_of"),
                ("similar to", "similar_to"),
                ("relates to", "relates_to"),
                ("knows", "knows"),
                ("works for", "works_for"),
            ],
        }
    }

    /// Extract relations from text (simple pattern-based)
    pub fn extract(&self, text: &str) -> Vec<ExtractedRelation> {
        let mut relations = Vec::new();
        let text_lower = text.to_lowercase();
        
        for (pattern, relation_type) in &self.patterns {
            if text_lower.contains(pattern) {
                // Find entities before and after the pattern
                if let Some(pos) = text_lower.find(pattern) {
                    let before = &text[..pos].trim();
                    let after = &text[pos + pattern.len()..].trim();
                    
                    if let (Some(from), Some(to)) = (
                        before.split_whitespace().last(),
                        after.split_whitespace().next(),
                    ) {
                        if from.len() > 2 && to.len() > 2 {
                            relations.push(ExtractedRelation {
                                from: from.to_string(),
                                to: to.to_string(),
                                relation_type: relation_type.to_string(),
                                confidence: 0.6,
                            });
                        }
                    }
                }
            }
        }
        
        relations
    }
}

impl Default for RelationExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedRelation {
    pub from: String,
    pub to: String,
    pub relation_type: String,
    pub confidence: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_graph_rag_context() {
        let kg = KnowledgeGraph::in_memory("test");
        
        // Add some entities
        let rust = Entity::new("Rust")
            .with_type(EntityType::Topic)
            .with_description("A systems programming language")
            .with_tag("programming");
        
        let python = Entity::new("Python")
            .with_type(EntityType::Topic)
            .with_description("A high-level programming language")
            .with_tag("programming");
        
        let id1 = kg.add_entity(rust).await.unwrap();
        let id2 = kg.add_entity(python).await.unwrap();
        
        // Add relation
        kg.add_relation(Relation::new(id1, id2, "similar_to")).await.unwrap();
        
        // Create RAG
        let rag = GraphRAG::new(Arc::new(kg));
        
        // Retrieve context
        let context = rag.retrieve("Rust programming").await.unwrap();
        
        assert!(context.entity_count() > 0);
        assert!(!context.text.is_empty());
    }

    #[test]
    fn test_entity_extractor() {
        let extractor = EntityExtractor::new();
        
        let text = "The developer Alice uses the tool Rust";
        let entities = extractor.extract(text);
        
        // Should extract "Alice" as Person and "Rust" as Tool
        assert!(!entities.is_empty());
    }

    #[test]
    fn test_relation_extractor() {
        let extractor = RelationExtractor::new();
        
        let text = "Alice works for the company Acme";
        let relations = extractor.extract(text);
        
        // Should extract "Alice works_for Acme"
        assert!(!relations.is_empty());
    }

    #[test]
    fn test_rag_context_json() {
        let context = RAGContext {
            text: "Test context".to_string(),
            entities: vec![Entity::new("Test").with_type(EntityType::Concept)],
            relations: vec![],
            query: "test query".to_string(),
        };
        
        let json = context.to_json();
        assert_eq!(json["query"], "test query");
        assert!(json["entities"].is_array());
    }
}
