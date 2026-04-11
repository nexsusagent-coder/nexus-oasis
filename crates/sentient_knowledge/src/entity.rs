//! ─── Entity Module ───
//!
//! Entity definitions and management for Knowledge Graph.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ─── Entity Type ───

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    /// A concept or idea
    Concept,
    /// A person
    Person,
    /// An organization
    Organization,
    /// A location
    Location,
    /// An event
    Event,
    /// A document or resource
    Document,
    /// A skill or capability
    Skill,
    /// A topic or subject
    Topic,
    /// A tool or software
    Tool,
    /// Custom entity type
    Custom,
}

impl Default for EntityType {
    fn default() -> Self {
        Self::Concept
    }
}

impl std::fmt::Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityType::Concept => write!(f, "Concept"),
            EntityType::Person => write!(f, "Person"),
            EntityType::Organization => write!(f, "Organization"),
            EntityType::Location => write!(f, "Location"),
            EntityType::Event => write!(f, "Event"),
            EntityType::Document => write!(f, "Document"),
            EntityType::Skill => write!(f, "Skill"),
            EntityType::Topic => write!(f, "Topic"),
            EntityType::Tool => write!(f, "Tool"),
            EntityType::Custom => write!(f, "Custom"),
        }
    }
}

// ─── Entity ───

/// An entity in the knowledge graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// Unique identifier
    pub id: Uuid,
    /// Entity name
    pub name: String,
    /// Entity type
    pub entity_type: EntityType,
    /// Description
    pub description: String,
    /// Additional properties
    pub properties: HashMap<String, serde_json::Value>,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
    /// Source of the entity
    pub source: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl Entity {
    /// Create a new entity with the given name
    pub fn new(name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            entity_type: EntityType::default(),
            description: String::new(),
            properties: HashMap::new(),
            tags: Vec::new(),
            confidence: 1.0,
            source: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Set entity type
    pub fn with_type(mut self, entity_type: EntityType) -> Self {
        self.entity_type = entity_type;
        self
    }

    /// Set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Add a property
    pub fn with_property(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.properties.insert(key.into(), value);
        self
    }

    /// Add a tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Set confidence
    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Set source
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Update the entity
    pub fn update(&mut self) {
        self.updated_at = Utc::now();
    }

    /// Check if entity matches a query
    pub fn matches(&self, query: &EntityQuery) -> bool {
        // Type filter
        if let Some(ref types) = query.entity_types {
            if !types.contains(&self.entity_type) {
                return false;
            }
        }

        // Name filter
        if let Some(ref name_pattern) = query.name_pattern {
            if !self.name.to_lowercase().contains(&name_pattern.to_lowercase()) {
                return false;
            }
        }

        // Tags filter
        if let Some(ref required_tags) = query.tags {
            for tag in required_tags {
                if !self.tags.contains(tag) {
                    return false;
                }
            }
        }

        // Property filter
        for (key, value) in &query.properties {
            if let Some(entity_value) = self.properties.get(key) {
                if entity_value != value {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Confidence filter
        if let Some(min_confidence) = query.min_confidence {
            if self.confidence < min_confidence {
                return false;
            }
        }

        true
    }
}

// ─── Entity Query ───

/// Query for finding entities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntityQuery {
    /// Filter by entity types
    pub entity_types: Option<Vec<EntityType>>,
    /// Filter by name pattern (case-insensitive contains)
    pub name_pattern: Option<String>,
    /// Filter by tags (all must match)
    pub tags: Option<Vec<String>>,
    /// Filter by properties
    pub properties: HashMap<String, serde_json::Value>,
    /// Minimum confidence
    pub min_confidence: Option<f32>,
    /// Limit results
    pub limit: Option<usize>,
    /// Skip results (offset)
    pub skip: Option<usize>,
}

impl EntityQuery {
    /// Create a new entity query
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by entity type
    pub fn with_type(mut self, entity_type: EntityType) -> Self {
        self.entity_types
            .get_or_insert_with(Vec::new)
            .push(entity_type);
        self
    }

    /// Filter by name pattern
    pub fn with_name(mut self, pattern: impl Into<String>) -> Self {
        self.name_pattern = Some(pattern.into());
        self
    }

    /// Filter by tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.get_or_insert_with(Vec::new).push(tag.into());
        self
    }

    /// Filter by property
    pub fn with_property(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.properties.insert(key.into(), value);
        self
    }

    /// Set minimum confidence
    pub fn with_min_confidence(mut self, confidence: f32) -> Self {
        self.min_confidence = Some(confidence);
        self
    }

    /// Set limit
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set skip
    pub fn with_skip(mut self, skip: usize) -> Self {
        self.skip = Some(skip);
        self
    }
}

// ─── Entity Builder ───

/// Builder for creating entities
pub struct EntityBuilder {
    entity: Entity,
}

impl EntityBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            entity: Entity::new(name),
        }
    }

    pub fn entity_type(mut self, entity_type: EntityType) -> Self {
        self.entity.entity_type = entity_type;
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.entity.description = description.into();
        self
    }

    pub fn property(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.entity.properties.insert(key.into(), value);
        self
    }

    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.entity.tags.push(tag.into());
        self
    }

    pub fn confidence(mut self, confidence: f32) -> Self {
        self.entity.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.entity.source = Some(source.into());
        self
    }

    pub fn build(self) -> Entity {
        self.entity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_entity() {
        let entity = Entity::new("Test Entity")
            .with_type(EntityType::Concept)
            .with_description("A test entity");

        assert_eq!(entity.name, "Test Entity");
        assert_eq!(entity.entity_type, EntityType::Concept);
        assert_eq!(entity.description, "A test entity");
    }

    #[test]
    fn test_entity_properties() {
        let entity = Entity::new("Test")
            .with_property("url", serde_json::json!("https://example.com"))
            .with_property("count", serde_json::json!(42))
            .with_tag("test")
            .with_tag("example");

        assert_eq!(entity.properties.len(), 2);
        assert_eq!(entity.tags.len(), 2);
    }

    #[test]
    fn test_entity_query_matching() {
        let entity = Entity::new("Rust Programming")
            .with_type(EntityType::Topic)
            .with_tag("programming")
            .with_confidence(0.9);

        // Should match
        let query1 = EntityQuery::new()
            .with_type(EntityType::Topic);
        assert!(entity.matches(&query1));

        // Should not match
        let query2 = EntityQuery::new()
            .with_type(EntityType::Person);
        assert!(!entity.matches(&query2));

        // Should match by name
        let query3 = EntityQuery::new()
            .with_name("rust");
        assert!(entity.matches(&query3));

        // Should match by tag
        let query4 = EntityQuery::new()
            .with_tag("programming");
        assert!(entity.matches(&query4));

        // Should not match by confidence
        let query5 = EntityQuery::new()
            .with_min_confidence(0.95);
        assert!(!entity.matches(&query5));
    }

    #[test]
    fn test_entity_builder() {
        let entity = EntityBuilder::new("Built Entity")
            .entity_type(EntityType::Skill)
            .description("Built with builder")
            .tag("builder")
            .confidence(0.8)
            .build();

        assert_eq!(entity.name, "Built Entity");
        assert_eq!(entity.entity_type, EntityType::Skill);
        assert_eq!(entity.confidence, 0.8);
    }
}
