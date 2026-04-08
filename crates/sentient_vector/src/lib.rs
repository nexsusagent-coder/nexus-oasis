//! SENTIENT Vector Database Integration Module
//! 
//! Unified interface for vector databases:
//! - **ChromaDB**: Open-source embedding database
//! - **Qdrant**: High-performance vector search
//! - **Weaviate**: GraphQL-based vector search
//! 
//! Sources loaded from integrations/memory/

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

pub mod chromadb;
pub mod qdrant;
pub mod weaviate;

/// Vector Database Type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VectorDb {
    ChromaDB,
    Qdrant,
    Weaviate,
}

/// Vector Document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDocument {
    pub id: String,
    pub content: String,
    pub embedding: Option<Vec<f32>>,
    pub metadata: HashMap<String, String>,
}

/// Search Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub document: VectorDocument,
    pub score: f32,
}

/// Vector Store Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorConfig {
    pub db_type: VectorDb,
    pub host: String,
    pub port: u16,
    pub collection: String,
    pub embedding_dim: usize,
}

impl Default for VectorConfig {
    fn default() -> Self {
        Self {
            db_type: VectorDb::Qdrant,
            host: "localhost".to_string(),
            port: 6333,
            collection: "sentient_memory".to_string(),
            embedding_dim: 1536,
        }
    }
}

/// Available Vector Databases
pub fn available_databases() -> Vec<VectorDbInfo> {
    vec![
        VectorDbInfo {
            db_type: VectorDb::ChromaDB,
            name: "ChromaDB".to_string(),
            source: "integrations/memory/chromadb".to_string(),
            status: "READY".to_string(),
            features: vec!["Embeddings".to_string(), "Collections".to_string(), "Metadata Filtering".to_string()],
        },
        VectorDbInfo {
            db_type: VectorDb::Qdrant,
            name: "Qdrant".to_string(),
            source: "integrations/memory/qdrant".to_string(),
            status: "READY".to_string(),
            features: vec!["High Performance".to_string(), "Filtering".to_string(), "Payload".to_string()],
        },
        VectorDbInfo {
            db_type: VectorDb::Weaviate,
            name: "Weaviate".to_string(),
            source: "integrations/memory/weaviate".to_string(),
            status: "READY".to_string(),
            features: vec!["GraphQL".to_string(), "Schema".to_string(), "Modules".to_string()],
        },
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDbInfo {
    pub db_type: VectorDb,
    pub name: String,
    pub source: String,
    pub status: String,
    pub features: Vec<String>,
}
