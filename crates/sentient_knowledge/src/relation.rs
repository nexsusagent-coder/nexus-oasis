//! ─── Relation Module ───
//!
//! Relation definitions for Knowledge Graph edges.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ─── Common Relation Types ───

pub const RELATES_TO: &str = "relates_to";
pub const PART_OF: &str = "part_of";
pub const HAS_PART: &str = "has_part";
pub const DEPENDS_ON: &str = "depends_on";
pub const CAUSED_BY: &str = "caused_by";
pub const CAUSES: &str = "causes";
pub const SIMILAR_TO: &str = "similar_to";
pub const DIFFERENT_FROM: &str = "different_from";
pub const INSTANCE_OF: &str = "instance_of";
pub const HAS_INSTANCE: &str = "has_instance";
pub const CREATED_BY: &str = "created_by";
pub const CREATES: &str = "creates";
pub const USED_BY: &str = "used_by";
pub const USES: &str = "uses";
pub const LOCATED_IN: &str = "located_in";
pub const CONTAINS: &str = "contains";
pub const KNOWS: &str = "knows";
pub const WORKS_FOR: &str = "works_for";
pub const EMPLOYS: &str = "employs";
pub const MENTIONED_IN: &str = "mentioned_in";
pub const MENTIONS: &str = "mentions";

// ─── Relation ───

/// A relation between two entities in the knowledge graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    /// Unique identifier
    pub id: Uuid,
    /// Source entity ID
    pub from_id: Uuid,
    /// Target entity ID
    pub to_id: Uuid,
    /// Relation type
    pub relation_type: String,
    /// Additional properties
    pub properties: HashMap<String, serde_json::Value>,
    /// Weight/strength of the relation (0.0 - 1.0)
    pub weight: f32,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
    /// Source of the relation
    pub source: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl Relation {
    /// Create a new relation between two entities
    pub fn new(from_id: Uuid, to_id: Uuid, relation_type: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            from_id,
            to_id,
            relation_type: relation_type.into(),
            properties: HashMap::new(),
            weight: 1.0,
            confidence: 1.0,
            source: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Set weight
    pub fn with_weight(mut self, weight: f32) -> Self {
        self.weight = weight.clamp(0.0, 1.0);
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

    /// Add a property
    pub fn with_property(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.properties.insert(key.into(), value);
        self
    }

    /// Update the relation
    pub fn update(&mut self) {
        self.updated_at = Utc::now();
    }

    /// Check if this is a bidirectional relation
    pub fn is_bidirectional(&self) -> bool {
        matches!(
            self.relation_type.as_str(),
            RELATES_TO | SIMILAR_TO | DIFFERENT_FROM | KNOWS
        )
    }

    /// Get the inverse relation type
    pub fn inverse_type(&self) -> Option<String> {
        match self.relation_type.as_str() {
            PART_OF => Some(HAS_PART.to_string()),
            HAS_PART => Some(PART_OF.to_string()),
            DEPENDS_ON => Some("depended_by".to_string()),
            CAUSED_BY => Some(CAUSES.to_string()),
            CAUSES => Some(CAUSED_BY.to_string()),
            INSTANCE_OF => Some(HAS_INSTANCE.to_string()),
            HAS_INSTANCE => Some(INSTANCE_OF.to_string()),
            CREATED_BY => Some(CREATES.to_string()),
            CREATES => Some(CREATED_BY.to_string()),
            USED_BY => Some(USES.to_string()),
            USES => Some(USED_BY.to_string()),
            LOCATED_IN => Some(CONTAINS.to_string()),
            CONTAINS => Some(LOCATED_IN.to_string()),
            WORKS_FOR => Some(EMPLOYS.to_string()),
            EMPLOYS => Some(WORKS_FOR.to_string()),
            MENTIONED_IN => Some(MENTIONS.to_string()),
            MENTIONS => Some(MENTIONED_IN.to_string()),
            _ => None,
        }
    }

    /// Create the inverse relation
    pub fn inverse(&self) -> Option<Relation> {
        self.inverse_type().map(|inv_type| {
            let mut inv = Relation::new(self.to_id, self.from_id, inv_type);
            inv.weight = self.weight;
            inv.confidence = self.confidence;
            inv.source = self.source.clone();
            inv.properties = self.properties.clone();
            inv
        })
    }
}

// ─── Relation Query ───

/// Query for finding relations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RelationQuery {
    /// Filter by source entity
    pub from_id: Option<Uuid>,
    /// Filter by target entity
    pub to_id: Option<Uuid>,
    /// Filter by relation type
    pub relation_types: Option<Vec<String>>,
    /// Filter by properties
    pub properties: HashMap<String, serde_json::Value>,
    /// Minimum weight
    pub min_weight: Option<f32>,
    /// Minimum confidence
    pub min_confidence: Option<f32>,
    /// Limit results
    pub limit: Option<usize>,
    /// Skip results (offset)
    pub skip: Option<usize>,
}

impl RelationQuery {
    /// Create a new relation query
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by source entity
    pub fn from_entity(mut self, id: Uuid) -> Self {
        self.from_id = Some(id);
        self
    }

    /// Filter by target entity
    pub fn to_entity(mut self, id: Uuid) -> Self {
        self.to_id = Some(id);
        self
    }

    /// Filter by relation type
    pub fn with_type(mut self, relation_type: impl Into<String>) -> Self {
        self.relation_types
            .get_or_insert_with(Vec::new)
            .push(relation_type.into());
        self
    }

    /// Filter by property
    pub fn with_property(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.properties.insert(key.into(), value);
        self
    }

    /// Set minimum weight
    pub fn with_min_weight(mut self, weight: f32) -> Self {
        self.min_weight = Some(weight);
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

    /// Check if a relation matches this query
    pub fn matches(&self, relation: &Relation) -> bool {
        // From filter
        if let Some(from_id) = self.from_id {
            if relation.from_id != from_id {
                return false;
            }
        }

        // To filter
        if let Some(to_id) = self.to_id {
            if relation.to_id != to_id {
                return false;
            }
        }

        // Type filter
        if let Some(ref types) = self.relation_types {
            if !types.contains(&relation.relation_type) {
                return false;
            }
        }

        // Weight filter
        if let Some(min_weight) = self.min_weight {
            if relation.weight < min_weight {
                return false;
            }
        }

        // Confidence filter
        if let Some(min_confidence) = self.min_confidence {
            if relation.confidence < min_confidence {
                return false;
            }
        }

        // Property filter
        for (key, value) in &self.properties {
            if let Some(rel_value) = relation.properties.get(key) {
                if rel_value != value {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

// ─── Relation Builder ───

/// Builder for creating relations
pub struct RelationBuilder {
    from_id: Uuid,
    to_id: Uuid,
    relation_type: String,
    weight: f32,
    confidence: f32,
    source: Option<String>,
    properties: HashMap<String, serde_json::Value>,
}

impl RelationBuilder {
    pub fn new(from_id: Uuid, to_id: Uuid, relation_type: impl Into<String>) -> Self {
        Self {
            from_id,
            to_id,
            relation_type: relation_type.into(),
            weight: 1.0,
            confidence: 1.0,
            source: None,
            properties: HashMap::new(),
        }
    }

    pub fn weight(mut self, weight: f32) -> Self {
        self.weight = weight.clamp(0.0, 1.0);
        self
    }

    pub fn confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn property(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.properties.insert(key.into(), value);
        self
    }

    pub fn build(self) -> Relation {
        let mut relation = Relation::new(self.from_id, self.to_id, self.relation_type);
        relation.weight = self.weight;
        relation.confidence = self.confidence;
        relation.source = self.source;
        relation.properties = self.properties;
        relation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_relation() {
        let from = Uuid::new_v4();
        let to = Uuid::new_v4();
        
        let relation = Relation::new(from, to, "relates_to")
            .with_weight(0.8)
            .with_confidence(0.9);

        assert_eq!(relation.from_id, from);
        assert_eq!(relation.to_id, to);
        assert_eq!(relation.relation_type, "relates_to");
        assert_eq!(relation.weight, 0.8);
        assert_eq!(relation.confidence, 0.9);
    }

    #[test]
    fn test_inverse_relation() {
        let from = Uuid::new_v4();
        let to = Uuid::new_v4();
        
        // part_of -> has_part
        let relation = Relation::new(from, to, PART_OF);
        let inverse = relation.inverse().unwrap();
        
        assert_eq!(inverse.from_id, to);
        assert_eq!(inverse.to_id, from);
        assert_eq!(inverse.relation_type, HAS_PART);

        // works_for -> employs
        let relation2 = Relation::new(from, to, WORKS_FOR);
        let inverse2 = relation2.inverse().unwrap();
        
        assert_eq!(inverse2.relation_type, EMPLOYS);
    }

    #[test]
    fn test_bidirectional() {
        let from = Uuid::new_v4();
        let to = Uuid::new_v4();
        
        let rel1 = Relation::new(from, to, RELATES_TO);
        assert!(rel1.is_bidirectional());

        let rel2 = Relation::new(from, to, PART_OF);
        assert!(!rel2.is_bidirectional());
    }

    #[test]
    fn test_relation_query() {
        let from = Uuid::new_v4();
        let to = Uuid::new_v4();
        
        let relation = Relation::new(from, to, "relates_to")
            .with_weight(0.8);

        // Should match
        let query1 = RelationQuery::new()
            .from_entity(from);
        assert!(query1.matches(&relation));

        // Should match
        let query2 = RelationQuery::new()
            .with_type("relates_to");
        assert!(query2.matches(&relation));

        // Should not match
        let query3 = RelationQuery::new()
            .with_min_weight(0.9);
        assert!(!query3.matches(&relation));
    }

    #[test]
    fn test_relation_builder() {
        let from = Uuid::new_v4();
        let to = Uuid::new_v4();
        
        let relation = RelationBuilder::new(from, to, "depends_on")
            .weight(0.7)
            .confidence(0.85)
            .source("test")
            .property("context", serde_json::json!("test_context"))
            .build();

        assert_eq!(relation.from_id, from);
        assert_eq!(relation.to_id, to);
        assert_eq!(relation.weight, 0.7);
        assert_eq!(relation.confidence, 0.85);
        assert_eq!(relation.source, Some("test".to_string()));
    }
}
