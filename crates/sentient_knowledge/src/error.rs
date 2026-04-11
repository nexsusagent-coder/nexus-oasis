//! ─── Error Module ───
//!
//! Error types for Knowledge Graph operations.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

// ─── Knowledge Error ───

#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum KnowledgeError {
    #[error("Entity not found: {0}")]
    EntityNotFound(Uuid),

    #[error("Relation not found: {0}")]
    RelationNotFound(Uuid),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Query error: {0}")]
    QueryError(String),

    #[error("Not connected to database")]
    NotConnected,

    #[error("Invalid entity type: {0}")]
    InvalidEntityType(String),

    #[error("Invalid relation type: {0}")]
    InvalidRelationType(String),

    #[error("Entity already exists: {0}")]
    EntityAlreadyExists(Uuid),

    #[error("Relation already exists: {0}")]
    RelationAlreadyExists(Uuid),

    #[error("Cycle detected in graph")]
    CycleDetected,

    #[error("Maximum depth exceeded: {0}")]
    MaxDepthExceeded(u32),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

// ─── Result Type ───

pub type KnowledgeResult<T> = Result<T, KnowledgeError>;

// ─── Implementations ───

impl From<serde_json::Error> for KnowledgeError {
    fn from(e: serde_json::Error) -> Self {
        KnowledgeError::SerializationError(e.to_string())
    }
}

impl From<std::io::Error> for KnowledgeError {
    fn from(e: std::io::Error) -> Self {
        KnowledgeError::IoError(e.to_string())
    }
}

impl KnowledgeError {
    /// Check if this is a not found error
    pub fn is_not_found(&self) -> bool {
        matches!(
            self,
            KnowledgeError::EntityNotFound(_) | KnowledgeError::RelationNotFound(_)
        )
    }

    /// Check if this is a connection error
    pub fn is_connection_error(&self) -> bool {
        matches!(
            self,
            KnowledgeError::ConnectionError(_) | KnowledgeError::NotConnected
        )
    }

    /// Get error code for categorization
    pub fn error_code(&self) -> &'static str {
        match self {
            KnowledgeError::EntityNotFound(_) => "ENTITY_NOT_FOUND",
            KnowledgeError::RelationNotFound(_) => "RELATION_NOT_FOUND",
            KnowledgeError::ConnectionError(_) => "CONNECTION_ERROR",
            KnowledgeError::QueryError(_) => "QUERY_ERROR",
            KnowledgeError::NotConnected => "NOT_CONNECTED",
            KnowledgeError::InvalidEntityType(_) => "INVALID_ENTITY_TYPE",
            KnowledgeError::InvalidRelationType(_) => "INVALID_RELATION_TYPE",
            KnowledgeError::EntityAlreadyExists(_) => "ENTITY_EXISTS",
            KnowledgeError::RelationAlreadyExists(_) => "RELATION_EXISTS",
            KnowledgeError::CycleDetected => "CYCLE_DETECTED",
            KnowledgeError::MaxDepthExceeded(_) => "MAX_DEPTH_EXCEEDED",
            KnowledgeError::SerializationError(_) => "SERIALIZATION_ERROR",
            KnowledgeError::DeserializationError(_) => "DESERIALIZATION_ERROR",
            KnowledgeError::IoError(_) => "IO_ERROR",
            KnowledgeError::InternalError(_) => "INTERNAL_ERROR",
        }
    }

    /// Get a summary of the error (for logging)
    pub fn summary(&self) -> String {
        match self {
            KnowledgeError::EntityNotFound(id) => format!("Entity {} not found", id),
            KnowledgeError::RelationNotFound(id) => format!("Relation {} not found", id),
            KnowledgeError::ConnectionError(msg) => format!("Connection failed: {}", msg),
            KnowledgeError::QueryError(msg) => format!("Query failed: {}", msg),
            KnowledgeError::NotConnected => "Not connected to database".to_string(),
            KnowledgeError::InvalidEntityType(t) => format!("Invalid entity type: {}", t),
            KnowledgeError::InvalidRelationType(t) => format!("Invalid relation type: {}", t),
            KnowledgeError::EntityAlreadyExists(id) => format!("Entity {} already exists", id),
            KnowledgeError::RelationAlreadyExists(id) => format!("Relation {} already exists", id),
            KnowledgeError::CycleDetected => "Cycle detected in graph".to_string(),
            KnowledgeError::MaxDepthExceeded(d) => format!("Maximum depth {} exceeded", d),
            KnowledgeError::SerializationError(msg) => format!("Serialization error: {}", msg),
            KnowledgeError::DeserializationError(msg) => format!("Deserialization error: {}", msg),
            KnowledgeError::IoError(msg) => format!("IO error: {}", msg),
            KnowledgeError::InternalError(msg) => format!("Internal error: {}", msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes() {
        let err = KnowledgeError::EntityNotFound(Uuid::nil());
        assert_eq!(err.error_code(), "ENTITY_NOT_FOUND");
        
        let err = KnowledgeError::NotConnected;
        assert_eq!(err.error_code(), "NOT_CONNECTED");
    }

    #[test]
    fn test_is_not_found() {
        let err = KnowledgeError::EntityNotFound(Uuid::nil());
        assert!(err.is_not_found());
        
        let err = KnowledgeError::ConnectionError("test".to_string());
        assert!(!err.is_not_found());
    }

    #[test]
    fn test_is_connection_error() {
        let err = KnowledgeError::NotConnected;
        assert!(err.is_connection_error());
        
        let err = KnowledgeError::QueryError("test".to_string());
        assert!(!err.is_connection_error());
    }

    #[test]
    fn test_error_summary() {
        let id = Uuid::new_v4();
        let err = KnowledgeError::EntityNotFound(id);
        assert!(err.summary().contains(&id.to_string()));
    }
}
