//! ─── Query Module ───
//!
//! Query DSL for Knowledge Graph operations.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Entity, EntityType, EntityQuery, Relation, RelationQuery};

// ─── Graph Query ───

/// A query against the knowledge graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GraphQuery {
    /// Find entities matching criteria
    FindEntities(EntityQuery),
    /// Find relations matching criteria
    FindRelations(RelationQuery),
    /// Find a path between two entities
    FindPath {
        from: Uuid,
        to: Uuid,
        max_depth: u32,
    },
    /// Get subgraph around an entity
    Subgraph {
        entity: Uuid,
        depth: u32,
    },
    /// Find neighbors of an entity
    Neighbors {
        entity: Uuid,
        relation_type: Option<String>,
        direction: Direction,
    },
    /// Count entities or relations
    Count {
        target: CountTarget,
    },
    /// Custom Cypher query (Neo4j)
    Custom(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Direction {
    Outgoing,
    Incoming,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CountTarget {
    Entities,
    Relations,
    EntitiesOfType(EntityType),
    RelationsOfType(String),
}

// ─── Query Builder ───

/// Builder for constructing graph queries
pub struct QueryBuilder {
    query: Option<GraphQuery>,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self { query: None }
    }

    /// Find entities
    pub fn find_entities() -> EntityQueryBuilder {
        EntityQueryBuilder::new()
    }

    /// Find relations
    pub fn find_relations() -> RelationQueryBuilder {
        RelationQueryBuilder::new()
    }

    /// Find path between entities
    pub fn find_path(from: Uuid, to: Uuid) -> PathQueryBuilder {
        PathQueryBuilder::new(from, to)
    }

    /// Get subgraph
    pub fn subgraph(entity: Uuid) -> SubgraphQueryBuilder {
        SubgraphQueryBuilder::new(entity)
    }

    /// Find neighbors
    pub fn neighbors(entity: Uuid) -> NeighborQueryBuilder {
        NeighborQueryBuilder::new(entity)
    }

    /// Count entities
    pub fn count_entities() -> CountBuilder {
        CountBuilder::new(CountTarget::Entities)
    }

    /// Count relations
    pub fn count_relations() -> CountBuilder {
        CountBuilder::new(CountTarget::Relations)
    }

    /// Custom query
    pub fn custom(query: impl Into<String>) -> Self {
        Self {
            query: Some(GraphQuery::Custom(query.into())),
        }
    }

    pub fn build(self) -> Option<GraphQuery> {
        self.query
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Entity Query Builder ───

pub struct EntityQueryBuilder {
    query: EntityQuery,
}

impl EntityQueryBuilder {
    pub fn new() -> Self {
        Self {
            query: EntityQuery::new(),
        }
    }

    pub fn of_type(mut self, entity_type: EntityType) -> Self {
        self.query = self.query.with_type(entity_type);
        self
    }

    pub fn with_name(mut self, pattern: impl Into<String>) -> Self {
        self.query = self.query.with_name(pattern);
        self
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.query = self.query.with_tag(tag);
        self
    }

    pub fn with_property(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.query = self.query.with_property(key, value);
        self
    }

    pub fn min_confidence(mut self, confidence: f32) -> Self {
        self.query = self.query.with_min_confidence(confidence);
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.query = self.query.with_limit(limit);
        self
    }

    pub fn skip(mut self, skip: usize) -> Self {
        self.query = self.query.with_skip(skip);
        self
    }

    pub fn build(self) -> GraphQuery {
        GraphQuery::FindEntities(self.query)
    }
}

impl Default for EntityQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Relation Query Builder ───

pub struct RelationQueryBuilder {
    query: RelationQuery,
}

impl RelationQueryBuilder {
    pub fn new() -> Self {
        Self {
            query: RelationQuery::new(),
        }
    }

    pub fn from(mut self, entity_id: Uuid) -> Self {
        self.query = self.query.from_entity(entity_id);
        self
    }

    pub fn to(mut self, entity_id: Uuid) -> Self {
        self.query = self.query.to_entity(entity_id);
        self
    }

    pub fn of_type(mut self, relation_type: impl Into<String>) -> Self {
        self.query = self.query.with_type(relation_type);
        self
    }

    pub fn min_weight(mut self, weight: f32) -> Self {
        self.query = self.query.with_min_weight(weight);
        self
    }

    pub fn min_confidence(mut self, confidence: f32) -> Self {
        self.query = self.query.with_min_confidence(confidence);
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.query = self.query.with_limit(limit);
        self
    }

    pub fn build(self) -> GraphQuery {
        GraphQuery::FindRelations(self.query)
    }
}

impl Default for RelationQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Path Query Builder ───

pub struct PathQueryBuilder {
    from: Uuid,
    to: Uuid,
    max_depth: u32,
    relation_types: Option<Vec<String>>,
}

impl PathQueryBuilder {
    pub fn new(from: Uuid, to: Uuid) -> Self {
        Self {
            from,
            to,
            max_depth: 10,
            relation_types: None,
        }
    }

    pub fn max_depth(mut self, depth: u32) -> Self {
        self.max_depth = depth;
        self
    }

    pub fn via_relation_type(mut self, relation_type: impl Into<String>) -> Self {
        self.relation_types
            .get_or_insert_with(Vec::new)
            .push(relation_type.into());
        self
    }

    pub fn build(self) -> GraphQuery {
        GraphQuery::FindPath {
            from: self.from,
            to: self.to,
            max_depth: self.max_depth,
        }
    }
}

// ─── Subgraph Query Builder ───

pub struct SubgraphQueryBuilder {
    entity: Uuid,
    depth: u32,
    relation_types: Option<Vec<String>>,
}

impl SubgraphQueryBuilder {
    pub fn new(entity: Uuid) -> Self {
        Self {
            entity,
            depth: 2,
            relation_types: None,
        }
    }

    pub fn depth(mut self, depth: u32) -> Self {
        self.depth = depth;
        self
    }

    pub fn via_relation_type(mut self, relation_type: impl Into<String>) -> Self {
        self.relation_types
            .get_or_insert_with(Vec::new)
            .push(relation_type.into());
        self
    }

    pub fn build(self) -> GraphQuery {
        GraphQuery::Subgraph {
            entity: self.entity,
            depth: self.depth,
        }
    }
}

// ─── Neighbor Query Builder ───

pub struct NeighborQueryBuilder {
    entity: Uuid,
    relation_type: Option<String>,
    direction: Direction,
}

impl NeighborQueryBuilder {
    pub fn new(entity: Uuid) -> Self {
        Self {
            entity,
            relation_type: None,
            direction: Direction::Both,
        }
    }

    pub fn via(mut self, relation_type: impl Into<String>) -> Self {
        self.relation_type = Some(relation_type.into());
        self
    }

    pub fn outgoing(mut self) -> Self {
        self.direction = Direction::Outgoing;
        self
    }

    pub fn incoming(mut self) -> Self {
        self.direction = Direction::Incoming;
        self
    }

    pub fn both_directions(mut self) -> Self {
        self.direction = Direction::Both;
        self
    }

    pub fn build(self) -> GraphQuery {
        GraphQuery::Neighbors {
            entity: self.entity,
            relation_type: self.relation_type,
            direction: self.direction,
        }
    }
}

// ─── Count Builder ───

pub struct CountBuilder {
    target: CountTarget,
}

impl CountBuilder {
    pub fn new(target: CountTarget) -> Self {
        Self { target }
    }

    pub fn entities() -> Self {
        Self::new(CountTarget::Entities)
    }

    pub fn relations() -> Self {
        Self::new(CountTarget::Relations)
    }

    pub fn entities_of_type(entity_type: EntityType) -> Self {
        Self::new(CountTarget::EntitiesOfType(entity_type))
    }

    pub fn relations_of_type(relation_type: impl Into<String>) -> Self {
        Self::new(CountTarget::RelationsOfType(relation_type.into()))
    }

    pub fn build(self) -> GraphQuery {
        GraphQuery::Count { target: self.target }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_query_builder() {
        let query = QueryBuilder::find_entities()
            .of_type(EntityType::Topic)
            .with_name("rust")
            .with_tag("programming")
            .min_confidence(0.8)
            .limit(10)
            .build();

        match query {
            GraphQuery::FindEntities(q) => {
                assert!(q.entity_types.is_some());
                assert_eq!(q.name_pattern, Some("rust".to_string()));
                assert_eq!(q.limit, Some(10));
            }
            _ => panic!("Wrong query type"),
        }
    }

    #[test]
    fn test_relation_query_builder() {
        let from = Uuid::new_v4();
        let query = QueryBuilder::find_relations()
            .from(from)
            .of_type("relates_to")
            .min_weight(0.5)
            .build();

        match query {
            GraphQuery::FindRelations(q) => {
                assert_eq!(q.from_id, Some(from));
                assert!(q.relation_types.is_some());
            }
            _ => panic!("Wrong query type"),
        }
    }

    #[test]
    fn test_path_query_builder() {
        let from = Uuid::new_v4();
        let to = Uuid::new_v4();

        let query = QueryBuilder::find_path(from, to)
            .max_depth(5)
            .build();

        match query {
            GraphQuery::FindPath { from: f, to: t, max_depth } => {
                assert_eq!(f, from);
                assert_eq!(t, to);
                assert_eq!(max_depth, 5);
            }
            _ => panic!("Wrong query type"),
        }
    }

    #[test]
    fn test_subgraph_query_builder() {
        let entity = Uuid::new_v4();

        let query = QueryBuilder::subgraph(entity)
            .depth(3)
            .build();

        match query {
            GraphQuery::Subgraph { entity: e, depth } => {
                assert_eq!(e, entity);
                assert_eq!(depth, 3);
            }
            _ => panic!("Wrong query type"),
        }
    }
}
