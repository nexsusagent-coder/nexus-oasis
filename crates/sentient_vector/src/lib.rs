//! ─── SENTIENT VECTOR - Unified Vector Database Interface ───
//!
//! Multi-provider vector database support:
//! - Qdrant (high-performance, Rust-native)
//! - ChromaDB (open-source, Python-friendly)
//! - Weaviate (GraphQL-based, modules)
//! - Pinecone (managed, serverless)
//! - Milvus (scalable, distributed)
//! - Elasticsearch (hybrid search)
//!
//! # Architecture
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    VectorStore Trait                        │
//! │  - upsert, search, delete, get                              │
//! │  - create_collection, delete_collection                     │
//! └─────────────────────────────────────────────────────────────┘
//!                              │
//!      ┌───────────────────────┼───────────────────────┐
//!      │                       │                       │
//!      ▼                       ▼                       ▼
//! ┌──────────┐          ┌──────────┐          ┌──────────┐
//! │ Qdrant   │          │ ChromaDB │          │ Weaviate │
//! │ Client   │          │ Client   │          │ Client   │
//! └──────────┘          └──────────┘          └──────────┘
//!      │                       │                       │
//!      └───────────────────────┼───────────────────────┘
//!                              │
//!              ┌───────────────┼───────────────┐
//!              │               │               │
//!              ▼               ▼               ▼
//!         ┌────────┐      ┌────────┐      ┌────────┐
//!         │Pinecone│      │ Milvus │      │Elastic │
//!         │ Client │      │ Client │      │ Client │
//!         └────────┘      └────────┘      └────────┘
//! ```
//!
//! # Example
//! ```rust,ignore
//! use sentient_vector::{VectorStore, QdrantStore, VectorConfig};
//!
//! let store = QdrantStore::new(VectorConfig {
//!     host: "localhost".into(),
//!     port: 6333,
//!     collection: "memories".into(),
//!     embedding_dim: 1536,
//! }).await?;
//!
//! // Insert documents
//! store.upsert(vec![
//!     VectorDocument::new("doc1", "Hello world", vec![0.1, 0.2, ...]),
//! ]).await?;
//!
//! // Search
//! let results = store.search(&query_vector, 10, None).await?;
//! ```

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod qdrant;
pub mod chromadb;
pub mod weaviate;
pub mod pinecone;
pub mod milvus;
pub mod elastic;
pub mod hybrid;
pub mod index;

// ═══════════════════════════════════════════════════════════════════════════════
//  CORE TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Vector database type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VectorDbType {
    Qdrant,
    ChromaDB,
    Weaviate,
    Pinecone,
    Milvus,
    Elasticsearch,
}

impl std::fmt::Display for VectorDbType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Qdrant => write!(f, "Qdrant"),
            Self::ChromaDB => write!(f, "ChromaDB"),
            Self::Weaviate => write!(f, "Weaviate"),
            Self::Pinecone => write!(f, "Pinecone"),
            Self::Milvus => write!(f, "Milvus"),
            Self::Elasticsearch => write!(f, "Elasticsearch"),
        }
    }
}

/// Vector document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDocument {
    /// Unique identifier
    pub id: String,
    /// Text content
    pub content: String,
    /// Vector embedding
    pub vector: Vec<f32>,
    /// Metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Score (populated on search)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
}

impl VectorDocument {
    /// Create new document
    pub fn new(id: impl Into<String>, content: impl Into<String>, vector: Vec<f32>) -> Self {
        Self {
            id: id.into(),
            content: content.into(),
            vector,
            metadata: HashMap::new(),
            score: None,
        }
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }

    /// Set score
    pub fn with_score(mut self, score: f32) -> Self {
        self.score = Some(score);
        self
    }
}

/// Search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Document
    pub document: VectorDocument,
    /// Similarity score
    pub score: f32,
}

/// Vector store configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorConfig {
    /// Database type
    pub db_type: VectorDbType,
    /// Host
    pub host: String,
    /// Port
    pub port: u16,
    /// API key (for managed services)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// Collection/Index name
    pub collection: String,
    /// Embedding dimension
    pub embedding_dim: usize,
    /// Distance metric
    pub metric: DistanceMetric,
    /// Connection timeout (ms)
    pub timeout_ms: u64,
}

impl Default for VectorConfig {
    fn default() -> Self {
        Self {
            db_type: VectorDbType::Qdrant,
            host: "localhost".into(),
            port: 6333,
            api_key: None,
            collection: "sentient_vectors".into(),
            embedding_dim: 1536,
            metric: DistanceMetric::Cosine,
            timeout_ms: 30000,
        }
    }
}

impl VectorConfig {
    /// Create Qdrant config
    pub fn qdrant(host: &str, port: u16, collection: &str, dim: usize) -> Self {
        Self {
            db_type: VectorDbType::Qdrant,
            host: host.into(),
            port,
            collection: collection.into(),
            embedding_dim: dim,
            ..Default::default()
        }
    }

    /// Create ChromaDB config
    pub fn chromadb(host: &str, port: u16, collection: &str, dim: usize) -> Self {
        Self {
            db_type: VectorDbType::ChromaDB,
            host: host.into(),
            port,
            collection: collection.into(),
            embedding_dim: dim,
            ..Default::default()
        }
    }

    /// Create Pinecone config
    pub fn pinecone(api_key: &str, index: &str, dim: usize) -> Self {
        Self {
            db_type: VectorDbType::Pinecone,
            host: "api.pinecone.io".into(),
            port: 443,
            api_key: Some(api_key.into()),
            collection: index.into(),
            embedding_dim: dim,
            ..Default::default()
        }
    }
}

/// Distance metric
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DistanceMetric {
    /// Cosine similarity (normalized dot product)
    Cosine,
    /// Euclidean distance
    Euclidean,
    /// Dot product
    DotProduct,
    /// Manhattan distance
    Manhattan,
}

impl std::fmt::Display for DistanceMetric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cosine => write!(f, "cosine"),
            Self::Euclidean => write!(f, "euclidean"),
            Self::DotProduct => write!(f, "dot"),
            Self::Manhattan => write!(f, "manhattan"),
        }
    }
}

/// Search filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilter {
    /// Field name
    pub field: String,
    /// Filter condition
    pub condition: FilterCondition,
}

/// Filter condition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum FilterCondition {
    /// Equals
    Eq(serde_json::Value),
    /// Not equals
    Neq(serde_json::Value),
    /// Greater than
    Gt(serde_json::Value),
    /// Greater than or equal
    Gte(serde_json::Value),
    /// Less than
    Lt(serde_json::Value),
    /// Less than or equal
    Lte(serde_json::Value),
    /// In list
    In(Vec<serde_json::Value>),
    /// Not in list
    Nin(Vec<serde_json::Value>),
    /// Contains (for arrays)
    Contains(serde_json::Value),
    /// Text match
    Match(String),
}

impl SearchFilter {
    /// Create equality filter
    pub fn eq(field: impl Into<String>, value: serde_json::Value) -> Self {
        Self {
            field: field.into(),
            condition: FilterCondition::Eq(value),
        }
    }

    /// Create range filter
    pub fn gte(field: impl Into<String>, value: serde_json::Value) -> Self {
        Self {
            field: field.into(),
            condition: FilterCondition::Gte(value),
        }
    }

    /// Create match filter
    pub fn match_text(field: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            condition: FilterCondition::Match(text.into()),
        }
    }
}

/// Search options
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchOptions {
    /// Number of results
    pub limit: usize,
    /// Offset for pagination
    pub offset: usize,
    /// Filters
    pub filters: Vec<SearchFilter>,
    /// Include vectors in response
    pub include_vectors: bool,
    /// Include content in response
    pub include_content: bool,
    /// Minimum score threshold
    pub min_score: Option<f32>,
}

impl SearchOptions {
    /// Create with limit
    pub fn with_limit(limit: usize) -> Self {
        Self { limit, ..Default::default() }
    }

    /// Add filter
    pub fn with_filter(mut self, filter: SearchFilter) -> Self {
        self.filters.push(filter);
        self
    }

    /// Set minimum score
    pub fn with_min_score(mut self, score: f32) -> Self {
        self.min_score = Some(score);
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  VECTOR STORE TRAIT
// ═══════════════════════════════════════════════════════════════════════════════

/// Vector store trait - unified interface for all vector databases
#[async_trait]
pub trait VectorStore: Send + Sync {
    /// Get store type
    fn store_type(&self) -> VectorDbType;

    /// Create collection/index
    async fn create_collection(&self) -> Result<()>;

    /// Delete collection/index
    async fn delete_collection(&self) -> Result<()>;

    /// Check if collection exists
    async fn collection_exists(&self) -> Result<bool>;

    /// Insert or update documents
    async fn upsert(&self, documents: Vec<VectorDocument>) -> Result<usize>;

    /// Delete documents by IDs
    async fn delete(&self, ids: &[&str]) -> Result<usize>;

    /// Get document by ID
    async fn get(&self, id: &str) -> Result<Option<VectorDocument>>;

    /// Get multiple documents by IDs
    async fn get_batch(&self, ids: &[&str]) -> Result<Vec<VectorDocument>>;

    /// Vector similarity search
    async fn search(&self, vector: &[f32], limit: usize, options: Option<SearchOptions>) -> Result<Vec<SearchResult>>;

    /// Hybrid search (vector + keyword)
    async fn hybrid_search(&self, vector: &[f32], query: &str, limit: usize, options: Option<SearchOptions>) -> Result<Vec<SearchResult>>;

    /// Count documents in collection
    async fn count(&self) -> Result<usize>;

    /// Get collection stats
    async fn stats(&self) -> Result<CollectionStats>;

    /// Health check
    async fn health(&self) -> Result<bool>;
}

/// Collection statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionStats {
    /// Number of vectors
    pub vector_count: usize,
    /// Index size in bytes
    pub index_size_bytes: u64,
    /// Dimension
    pub dimension: usize,
    /// Distance metric
    pub metric: DistanceMetric,
    /// Status
    pub status: String,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ERROR TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Vector store error
#[derive(Debug, thiserror::Error)]
pub enum VectorError {
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Collection not found: {0}")]
    CollectionNotFound(String),

    #[error("Document not found: {0}")]
    DocumentNotFound(String),

    #[error("Invalid vector dimension: expected {expected}, got {actual}")]
    InvalidDimension { expected: usize, actual: usize },

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Request failed: {0}")]
    RequestFailed(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Timeout")]
    Timeout,

    #[error("Rate limited")]
    RateLimited,

    #[error("Not implemented: {0}")]
    NotImplemented(String),
}

pub type Result<T> = std::result::Result<T, VectorError>;

// ═══════════════════════════════════════════════════════════════════════════════
//  VECTOR UTILITIES
// ═══════════════════════════════════════════════════════════════════════════════

/// Calculate cosine similarity
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot / (norm_a * norm_b)
}

/// Calculate euclidean distance
pub fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return f32::MAX;
    }

    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f32>()
        .sqrt()
}

/// Normalize vector
pub fn normalize_vector(v: &[f32]) -> Vec<f32> {
    let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm == 0.0 {
        return v.to_vec();
    }
    v.iter().map(|x| x / norm).collect()
}

// ═══════════════════════════════════════════════════════════════════════════════
//  IN-Memory VECTOR STORE (FOR TESTING)
// ═══════════════════════════════════════════════════════════════════════════════

/// In-memory vector store for testing
pub struct InMemoryVectorStore {
    config: VectorConfig,
    documents: Arc<tokio::sync::RwLock<HashMap<String, VectorDocument>>>,
}

impl InMemoryVectorStore {
    /// Create new in-memory store
    pub fn new(config: VectorConfig) -> Self {
        Self {
            config,
            documents: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl VectorStore for InMemoryVectorStore {
    fn store_type(&self) -> VectorDbType {
        VectorDbType::Qdrant // Pretend to be Qdrant
    }

    async fn create_collection(&self) -> Result<()> {
        Ok(())
    }

    async fn delete_collection(&self) -> Result<()> {
        let mut docs = self.documents.write().await;
        docs.clear();
        Ok(())
    }

    async fn collection_exists(&self) -> Result<bool> {
        Ok(true)
    }

    async fn upsert(&self, documents: Vec<VectorDocument>) -> Result<usize> {
        let mut docs = self.documents.write().await;
        let count = documents.len();
        for doc in documents {
            docs.insert(doc.id.clone(), doc);
        }
        Ok(count)
    }

    async fn delete(&self, ids: &[&str]) -> Result<usize> {
        let mut docs = self.documents.write().await;
        let mut count = 0;
        for id in ids {
            if docs.remove(*id).is_some() {
                count += 1;
            }
        }
        Ok(count)
    }

    async fn get(&self, id: &str) -> Result<Option<VectorDocument>> {
        let docs = self.documents.read().await;
        Ok(docs.get(id).cloned())
    }

    async fn get_batch(&self, ids: &[&str]) -> Result<Vec<VectorDocument>> {
        let docs = self.documents.read().await;
        Ok(ids.iter().filter_map(|id| docs.get(*id).cloned()).collect())
    }

    async fn search(&self, vector: &[f32], limit: usize, options: Option<SearchOptions>) -> Result<Vec<SearchResult>> {
        let docs = self.documents.read().await;
        let min_score = options.as_ref().and_then(|o| o.min_score).unwrap_or(0.0);

        let mut results: Vec<SearchResult> = docs
            .values()
            .map(|doc| {
                let score = match self.config.metric {
                    DistanceMetric::Cosine => cosine_similarity(vector, &doc.vector),
                    DistanceMetric::Euclidean => 1.0 / (1.0 + euclidean_distance(vector, &doc.vector)),
                    DistanceMetric::DotProduct => vector.iter().zip(doc.vector.iter()).map(|(a, b)| a * b).sum(),
                    DistanceMetric::Manhattan => 1.0 / (1.0 + vector.iter().zip(doc.vector.iter()).map(|(a, b)| (a - b).abs()).sum::<f32>()),
                };
                SearchResult {
                    document: doc.clone(),
                    score,
                }
            })
            .filter(|r| r.score >= min_score)
            .collect();

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit);

        Ok(results)
    }

    async fn hybrid_search(&self, vector: &[f32], query: &str, limit: usize, options: Option<SearchOptions>) -> Result<Vec<SearchResult>> {
        // Simple hybrid: combine vector search with keyword matching
        let mut results = self.search(vector, limit * 2, options).await?;
        
        // Boost results that contain query terms
        let query_lower = query.to_lowercase();
        for result in &mut results {
            if result.document.content.to_lowercase().contains(&query_lower) {
                result.score *= 1.5; // Boost for keyword match
            }
        }

        // Re-sort and truncate
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit);

        Ok(results)
    }

    async fn count(&self) -> Result<usize> {
        let docs = self.documents.read().await;
        Ok(docs.len())
    }

    async fn stats(&self) -> Result<CollectionStats> {
        let docs = self.documents.read().await;
        Ok(CollectionStats {
            vector_count: docs.len(),
            index_size_bytes: 0,
            dimension: self.config.embedding_dim,
            metric: self.config.metric,
            status: "green".into(),
        })
    }

    async fn health(&self) -> Result<bool> {
        Ok(true)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  VECTOR STORE FACTORY
// ═══════════════════════════════════════════════════════════════════════════════

/// Create vector store from config
pub async fn create_store(config: VectorConfig) -> Result<Arc<dyn VectorStore>> {
    match config.db_type {
        VectorDbType::Qdrant => {
            let store = qdrant::QdrantStore::new(config).await?;
            Ok(Arc::new(store))
        }
        VectorDbType::ChromaDB => {
            let store = chromadb::ChromaStore::new(config).await?;
            Ok(Arc::new(store))
        }
        VectorDbType::Weaviate => {
            let store = weaviate::WeaviateStore::new(config).await?;
            Ok(Arc::new(store))
        }
        VectorDbType::Pinecone => {
            let store = pinecone::PineconeStore::new(config).await?;
            Ok(Arc::new(store))
        }
        VectorDbType::Milvus => {
            let store = milvus::MilvusStore::new(config).await?;
            Ok(Arc::new(store))
        }
        VectorDbType::Elasticsearch => {
            let store = elastic::ElasticStore::new(config).await?;
            Ok(Arc::new(store))
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 0.001);

        let c = vec![0.0, 1.0, 0.0];
        assert!(cosine_similarity(&a, &c).abs() < 0.001);

        let d = vec![-1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &d) - (-1.0)).abs() < 0.001);
    }

    #[test]
    fn test_normalize_vector() {
        let v = vec![3.0, 4.0];
        let n = normalize_vector(&v);
        assert!((n[0] - 0.6).abs() < 0.001);
        assert!((n[1] - 0.8).abs() < 0.001);
    }

    #[tokio::test]
    async fn test_in_memory_store() {
        let config = VectorConfig::default();
        let store = InMemoryVectorStore::new(config);

        // Create collection
        store.create_collection().await.unwrap();

        // Insert document
        let doc = VectorDocument::new("doc1", "Hello world", vec![0.1, 0.2, 0.3]);
        store.upsert(vec![doc]).await.unwrap();

        // Get document
        let result = store.get("doc1").await.unwrap();
        assert!(result.is_some());

        // Search
        let results = store.search(&[0.1, 0.2, 0.3], 10, None).await.unwrap();
        assert_eq!(results.len(), 1);
    }
}
