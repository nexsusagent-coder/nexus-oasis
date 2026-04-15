//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT RAG Vector Store - Retrieval-Augmented Generation
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  High-performance vector storage for semantic search:
//!  - Multiple backends (InMemory, Qdrant, Pinecone, Weaviate)
//!  - Embedding generation (OpenAI, Sentence Transformers)
//!  - Hybrid search (vector + keyword)
//!  - Metadata filtering
//!  - Document chunking

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// ═══════════════════════════════════════════════════════════════════════════════
//  VECTOR TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Vector embedding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector {
    /// Vector ID
    pub id: String,
    /// Embedding values
    pub embedding: Vec<f32>,
    /// Dimension
    pub dimension: usize,
    /// Metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Document text
    pub text: Option<String>,
    /// Source document ID
    pub document_id: Option<String>,
    /// Chunk index in source document
    pub chunk_index: Option<usize>,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Vector {
    pub fn new(id: String, embedding: Vec<f32>) -> Self {
        let dimension = embedding.len();
        Self {
            id,
            embedding,
            dimension,
            metadata: HashMap::new(),
            text: None,
            document_id: None,
            chunk_index: None,
            created_at: chrono::Utc::now(),
        }
    }
    
    pub fn with_metadata(mut self, metadata: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = metadata;
        self
    }
    
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }
    
    pub fn with_document(mut self, doc_id: String, chunk_idx: usize) -> Self {
        self.document_id = Some(doc_id);
        self.chunk_index = Some(chunk_idx);
        self
    }
}

/// Document for indexing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Document ID
    pub id: String,
    /// Document content
    pub content: String,
    /// Metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Source URL or path
    pub source: Option<String>,
    /// Document title
    pub title: Option<String>,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Document {
    pub fn new(id: String, content: String) -> Self {
        Self {
            id,
            content,
            metadata: HashMap::new(),
            source: None,
            title: None,
            created_at: chrono::Utc::now(),
        }
    }
    
    pub fn with_source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }
}

/// Search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Matched vector
    pub vector: Vector,
    /// Similarity score (0.0 - 1.0)
    pub score: f32,
    /// Highlighted text snippet
    pub highlight: Option<String>,
}

/// Search query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    /// Query text
    pub query: String,
    /// Number of results
    pub limit: usize,
    /// Minimum similarity threshold
    pub min_score: f32,
    /// Metadata filters
    pub filters: Option<MetadataFilter>,
    /// Include text in results
    pub include_text: bool,
    /// Hybrid search weight (0.0 = pure vector, 1.0 = pure keyword)
    pub hybrid_weight: Option<f32>,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            query: String::new(),
            limit: 10,
            min_score: 0.0,
            filters: None,
            include_text: true,
            hybrid_weight: None,
        }
    }
}

impl SearchQuery {
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            ..Default::default()
        }
    }
    
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
    
    pub fn with_min_score(mut self, score: f32) -> Self {
        self.min_score = score;
        self
    }
    
    pub fn with_filter(mut self, filter: MetadataFilter) -> Self {
        self.filters = Some(filter);
        self
    }
}

/// Metadata filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataFilter {
    /// Must match all conditions
    pub must: Vec<FilterCondition>,
    /// Must match any condition
    pub should: Vec<FilterCondition>,
    /// Must not match any condition
    pub must_not: Vec<FilterCondition>,
}

impl MetadataFilter {
    pub fn new() -> Self {
        Self {
            must: Vec::new(),
            should: Vec::new(),
            must_not: Vec::new(),
        }
    }
    
    pub fn must(mut self, condition: FilterCondition) -> Self {
        self.must.push(condition);
        self
    }
    
    pub fn should(mut self, condition: FilterCondition) -> Self {
        self.should.push(condition);
        self
    }
    
    pub fn must_not(mut self, condition: FilterCondition) -> Self {
        self.must_not.push(condition);
        self
    }
    
    /// Check if a document matches the filter
    pub fn matches(&self, metadata: &HashMap<String, serde_json::Value>) -> bool {
        // All must conditions must match
        for condition in &self.must {
            if !condition.matches(metadata) {
                return false;
            }
        }
        
        // None of must_not conditions should match
        for condition in &self.must_not {
            if condition.matches(metadata) {
                return false;
            }
        }
        
        // At least one should condition must match (if any exist)
        if !self.should.is_empty() {
            let any_match = self.should.iter().any(|c| c.matches(metadata));
            if !any_match {
                return false;
            }
        }
        
        true
    }
}

/// Filter condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCondition {
    /// Field name
    pub field: String,
    /// Operator
    pub operator: FilterOperator,
    /// Value to compare
    pub value: serde_json::Value,
}

impl FilterCondition {
    pub fn new(field: impl Into<String>, operator: FilterOperator, value: serde_json::Value) -> Self {
        Self {
            field: field.into(),
            operator,
            value,
        }
    }
    
    pub fn eq(field: impl Into<String>, value: serde_json::Value) -> Self {
        Self::new(field, FilterOperator::Eq, value)
    }
    
    pub fn ne(field: impl Into<String>, value: serde_json::Value) -> Self {
        Self::new(field, FilterOperator::Ne, value)
    }
    
    pub fn gt(field: impl Into<String>, value: serde_json::Value) -> Self {
        Self::new(field, FilterOperator::Gt, value)
    }
    
    pub fn lt(field: impl Into<String>, value: serde_json::Value) -> Self {
        Self::new(field, FilterOperator::Lt, value)
    }
    
    pub fn in_list(field: impl Into<String>, values: Vec<serde_json::Value>) -> Self {
        Self::new(field, FilterOperator::In, serde_json::Value::Array(values))
    }
    
    pub fn matches(&self, metadata: &HashMap<String, serde_json::Value>) -> bool {
        let field_value = metadata.get(&self.field);
        
        match &self.operator {
            FilterOperator::Eq => field_value == Some(&self.value),
            FilterOperator::Ne => field_value != Some(&self.value),
            FilterOperator::Gt => {
                if let (Some(fv), Some(v)) = (field_value, self.value.as_f64()) {
                    fv.as_f64().map(|f| f > v).unwrap_or(false)
                } else {
                    false
                }
            }
            FilterOperator::Lt => {
                if let (Some(fv), Some(v)) = (field_value, self.value.as_f64()) {
                    fv.as_f64().map(|f| f < v).unwrap_or(false)
                } else {
                    false
                }
            }
            FilterOperator::Gte => {
                if let (Some(fv), Some(v)) = (field_value, self.value.as_f64()) {
                    fv.as_f64().map(|f| f >= v).unwrap_or(false)
                } else {
                    false
                }
            }
            FilterOperator::Lte => {
                if let (Some(fv), Some(v)) = (field_value, self.value.as_f64()) {
                    fv.as_f64().map(|f| f <= v).unwrap_or(false)
                } else {
                    false
                }
            }
            FilterOperator::In => {
                if let Some(arr) = self.value.as_array() {
                    field_value.map(|fv| arr.contains(fv)).unwrap_or(false)
                } else {
                    false
                }
            }
            FilterOperator::Contains => {
                if let (Some(fv), Some(v)) = (field_value.and_then(|f| f.as_str()), self.value.as_str()) {
                    fv.contains(v)
                } else {
                    false
                }
            }
        }
    }
}

/// Filter operators
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FilterOperator {
    Eq,
    Ne,
    Gt,
    Lt,
    Gte,
    Lte,
    In,
    Contains,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  EMBEDDING GENERATOR
// ═══════════════════════════════════════════════════════════════════════════════

/// Embedding model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingConfig {
    /// Model type
    pub model: EmbeddingModel,
    /// Output dimension
    pub dimension: usize,
    /// Batch size for embedding
    pub batch_size: usize,
}

impl Default for EmbeddingConfig {
    fn default() -> Self {
        Self {
            model: EmbeddingModel::SentenceTransformerAllMiniLmL6V2,
            dimension: 384,
            batch_size: 32,
        }
    }
}

/// Supported embedding models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmbeddingModel {
    /// OpenAI text-embedding-ada-002
    OpenAIAda002,
    /// OpenAI text-embedding-3-small
    OpenAI3Small,
    /// OpenAI text-embedding-3-large
    OpenAI3Large,
    /// Sentence Transformers all-MiniLM-L6-v2
    SentenceTransformerAllMiniLmL6V2,
    /// Sentence Transformers all-mpnet-base-v2
    SentenceTransformerAllMpnetBaseV2,
    /// Custom model path
    Custom(String),
}

/// Embedding generator trait
#[async_trait::async_trait]
pub trait EmbeddingGenerator: Send + Sync {
    /// Generate embedding for text
    async fn embed(&self, text: &str) -> Result<Vec<f32>, VectorStoreError>;
    
    /// Generate embeddings for multiple texts
    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, VectorStoreError>;
    
    /// Get embedding dimension
    fn dimension(&self) -> usize;
}

// ═══════════════════════════════════════════════════════════════════════════════
//  VECTOR STORE BACKEND
// ═══════════════════════════════════════════════════════════════════════════════

/// Vector store backend trait
#[async_trait::async_trait]
pub trait VectorStoreBackend: Send + Sync {
    /// Add a vector
    async fn add(&self, vector: Vector) -> Result<(), VectorStoreError>;
    
    /// Add multiple vectors
    async fn add_batch(&self, vectors: Vec<Vector>) -> Result<(), VectorStoreError>;
    
    /// Search for similar vectors
    async fn search(&self, query: Vec<f32>, limit: usize, filter: Option<&MetadataFilter>) -> Result<Vec<SearchResult>, VectorStoreError>;
    
    /// Delete a vector by ID
    async fn delete(&self, id: &str) -> Result<(), VectorStoreError>;
    
    /// Delete vectors by document ID
    async fn delete_by_document(&self, document_id: &str) -> Result<usize, VectorStoreError>;
    
    /// Get vector by ID
    async fn get(&self, id: &str) -> Result<Option<Vector>, VectorStoreError>;
    
    /// Get vector count
    async fn count(&self) -> Result<usize, VectorStoreError>;
    
    /// Clear all vectors
    async fn clear(&self) -> Result<(), VectorStoreError>;
    
    /// Get backend name
    fn name(&self) -> &str;
}

// ═══════════════════════════════════════════════════════════════════════════════
//  IN-MEMORY VECTOR STORE
// ═══════════════════════════════════════════════════════════════════════════════

/// In-memory vector store
pub struct InMemoryVectorStore {
    vectors: Arc<RwLock<HashMap<String, Vector>>>,
    dimension: usize,
}

impl InMemoryVectorStore {
    pub fn new(dimension: usize) -> Self {
        Self {
            vectors: Arc::new(RwLock::new(HashMap::new())),
            dimension,
        }
    }
    
    /// Calculate cosine similarity
    fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() {
            return 0.0;
        }
        
        let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let mag_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let mag_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
        
        if mag_a == 0.0 || mag_b == 0.0 {
            return 0.0;
        }
        
        dot / (mag_a * mag_b)
    }
}

#[async_trait::async_trait]
impl VectorStoreBackend for InMemoryVectorStore {
    async fn add(&self, vector: Vector) -> Result<(), VectorStoreError> {
        let mut vectors = self.vectors.write().await;
        vectors.insert(vector.id.clone(), vector);
        Ok(())
    }
    
    async fn add_batch(&self, vectors_batch: Vec<Vector>) -> Result<(), VectorStoreError> {
        let mut vectors = self.vectors.write().await;
        for vector in vectors_batch {
            vectors.insert(vector.id.clone(), vector);
        }
        Ok(())
    }
    
    async fn search(&self, query: Vec<f32>, limit: usize, filter: Option<&MetadataFilter>) -> Result<Vec<SearchResult>, VectorStoreError> {
        let vectors = self.vectors.read().await;
        
        let mut results: Vec<SearchResult> = vectors
            .values()
            .filter(|v| {
                if let Some(f) = filter {
                    f.matches(&v.metadata)
                } else {
                    true
                }
            })
            .map(|v| {
                let score = Self::cosine_similarity(&query, &v.embedding);
                SearchResult {
                    vector: v.clone(),
                    score,
                    highlight: v.text.as_ref().map(|t| {
                        // Simple highlight - first 200 chars
                        if t.len() > 200 {
                            format!("{}...", &t[..200])
                        } else {
                            t.clone()
                        }
                    }),
                }
            })
            .filter(|r| r.score > 0.0)
            .collect();
        
        // Sort by score descending
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit);
        
        Ok(results)
    }
    
    async fn delete(&self, id: &str) -> Result<(), VectorStoreError> {
        let mut vectors = self.vectors.write().await;
        vectors.remove(id);
        Ok(())
    }
    
    async fn delete_by_document(&self, document_id: &str) -> Result<usize, VectorStoreError> {
        let mut vectors = self.vectors.write().await;
        let ids_to_remove: Vec<String> = vectors
            .values()
            .filter(|v| v.document_id.as_deref() == Some(document_id))
            .map(|v| v.id.clone())
            .collect();
        
        let count = ids_to_remove.len();
        for id in ids_to_remove {
            vectors.remove(&id);
        }
        
        Ok(count)
    }
    
    async fn get(&self, id: &str) -> Result<Option<Vector>, VectorStoreError> {
        let vectors = self.vectors.read().await;
        Ok(vectors.get(id).cloned())
    }
    
    async fn count(&self) -> Result<usize, VectorStoreError> {
        let vectors = self.vectors.read().await;
        Ok(vectors.len())
    }
    
    async fn clear(&self) -> Result<(), VectorStoreError> {
        let mut vectors = self.vectors.write().await;
        vectors.clear();
        Ok(())
    }
    
    fn name(&self) -> &str {
        "in-memory"
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DOCUMENT CHUNKER
// ═══════════════════════════════════════════════════════════════════════════════

/// Chunking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkingConfig {
    /// Chunk size in characters
    pub chunk_size: usize,
    /// Overlap between chunks
    pub overlap: usize,
    /// Separator for splitting
    pub separator: String,
    /// Respect sentence boundaries
    pub respect_sentences: bool,
}

impl Default for ChunkingConfig {
    fn default() -> Self {
        Self {
            chunk_size: 1000,
            overlap: 200,
            separator: "\n\n".into(),
            respect_sentences: true,
        }
    }
}

/// Document chunker
pub struct DocumentChunker {
    config: ChunkingConfig,
}

impl DocumentChunker {
    pub fn new(config: ChunkingConfig) -> Self {
        Self { config }
    }
    
    /// Split document into chunks
    pub fn chunk(&self, document: &Document) -> Vec<Chunk> {
        let text = &document.content;
        
        if text.len() <= self.config.chunk_size {
            return vec![Chunk {
                id: format!("{}-0", document.id),
                document_id: document.id.clone(),
                content: text.clone(),
                index: 0,
                metadata: document.metadata.clone(),
            }];
        }
        
        let mut chunks = Vec::new();
        let mut start = 0;
        let mut index = 0;
        
        while start < text.len() {
            let end = (start + self.config.chunk_size).min(text.len());
            
            // Try to find sentence boundary
            let actual_end = if self.config.respect_sentences && end < text.len() {
                // Look for sentence ending
                let substr = &text[start..end];
                if let Some(pos) = substr.rfind(|c: char| c == '.' || c == '!' || c == '?') {
                    start + pos + 1
                } else {
                    end
                }
            } else {
                end
            };
            
            chunks.push(Chunk {
                id: format!("{}-{}", document.id, index),
                document_id: document.id.clone(),
                content: text[start..actual_end].to_string(),
                index,
                metadata: document.metadata.clone(),
            });
            
            start = actual_end.saturating_sub(self.config.overlap);
            index += 1;
            
            // Prevent infinite loop
            if actual_end >= text.len() {
                break;
            }
        }
        
        chunks
    }
}

/// Document chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub id: String,
    pub document_id: String,
    pub content: String,
    pub index: usize,
    pub metadata: HashMap<String, serde_json::Value>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  RAG STORE
// ═══════════════════════════════════════════════════════════════════════════════

/// RAG Store - combines embedding + storage
pub struct RagStore {
    backend: Arc<dyn VectorStoreBackend>,
    embedder: Arc<dyn EmbeddingGenerator>,
    chunker: DocumentChunker,
}

impl RagStore {
    pub fn new(
        backend: Arc<dyn VectorStoreBackend>,
        embedder: Arc<dyn EmbeddingGenerator>,
        chunking_config: ChunkingConfig,
    ) -> Self {
        Self {
            backend,
            embedder,
            chunker: DocumentChunker::new(chunking_config),
        }
    }
    
    /// Add a document
    pub async fn add_document(&self, document: Document) -> Result<usize, VectorStoreError> {
        log::info!("📚 Adding document: {}", document.id);
        
        let chunks = self.chunker.chunk(&document);
        let chunk_count = chunks.len();
        
        // Generate embeddings for all chunks
        let texts: Vec<String> = chunks.iter().map(|c| c.content.clone()).collect();
        let embeddings = self.embedder.embed_batch(&texts).await?;
        
        // Create vectors
        let vectors: Vec<Vector> = chunks
            .into_iter()
            .zip(embeddings.into_iter())
            .map(|(chunk, embedding)| {
                Vector::new(chunk.id, embedding)
                    .with_text(chunk.content)
                    .with_metadata(chunk.metadata)
                    .with_document(chunk.document_id, chunk.index)
            })
            .collect();
        
        // Store vectors
        self.backend.add_batch(vectors).await?;
        
        log::info!("📚 Document {} indexed with {} chunks", document.id, chunk_count);
        Ok(chunk_count)
    }
    
    /// Search for similar content
    pub async fn search(&self, query: SearchQuery) -> Result<Vec<SearchResult>, VectorStoreError> {
        log::debug!("🔍 Searching: {} (limit: {})", query.query, query.limit);
        
        // Generate query embedding
        let query_embedding = self.embedder.embed(&query.query).await?;
        
        // Search in backend
        let mut results = self.backend
            .search(query_embedding, query.limit, query.filters.as_ref())
            .await?;
        
        // Filter by min_score
        results.retain(|r| r.score >= query.min_score);
        
        log::debug!("🔍 Found {} results", results.len());
        Ok(results)
    }
    
    /// Delete document and all its chunks
    pub async fn delete_document(&self, document_id: &str) -> Result<usize, VectorStoreError> {
        self.backend.delete_by_document(document_id).await
    }
    
    /// Get vector count
    pub async fn count(&self) -> Result<usize, VectorStoreError> {
        self.backend.count().await
    }
    
    /// Clear all data
    pub async fn clear(&self) -> Result<(), VectorStoreError> {
        self.backend.clear().await
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ERROR TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Vector store error
#[derive(Debug, Clone)]
pub enum VectorStoreError {
    EmbeddingError(String),
    StorageError(String),
    NotFound(String),
    InvalidDimension { expected: usize, actual: usize },
    SerializationError(String),
}

impl std::fmt::Display for VectorStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmbeddingError(e) => write!(f, "Embedding error: {}", e),
            Self::StorageError(e) => write!(f, "Storage error: {}", e),
            Self::NotFound(id) => write!(f, "Not found: {}", id),
            Self::InvalidDimension { expected, actual } => {
                write!(f, "Invalid dimension: expected {}, got {}", expected, actual)
            }
            Self::SerializationError(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

impl std::error::Error for VectorStoreError {}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vector_creation() {
        let vec = Vector::new("v1".into(), vec![0.1, 0.2, 0.3]);
        assert_eq!(vec.id, "v1");
        assert_eq!(vec.dimension, 3);
    }
    
    #[test]
    fn test_metadata_filter() {
        let mut metadata = HashMap::new();
        metadata.insert("category".into(), serde_json::json!("docs"));
        metadata.insert("score".into(), serde_json::json!(0.95));
        
        let filter = MetadataFilter::new()
            .must(FilterCondition::eq("category", serde_json::json!("docs")));
        
        assert!(filter.matches(&metadata));
    }
    
    #[test]
    fn test_document_chunker() {
        let chunker = DocumentChunker::new(ChunkingConfig {
            chunk_size: 50,
            overlap: 10,
            ..Default::default()
        });
        
        let doc = Document::new("doc1".into(), "This is a test document with some content.".into());
        let chunks = chunker.chunk(&doc);
        
        assert!(!chunks.is_empty());
    }
    
    #[test]
    fn test_search_query() {
        let query = SearchQuery::new("test query")
            .with_limit(5)
            .with_min_score(0.5);
        
        assert_eq!(query.limit, 5);
        assert!((query.min_score - 0.5).abs() < 0.001);
    }
    
    #[tokio::test]
    async fn test_in_memory_store() {
        let store = InMemoryVectorStore::new(3);
        
        let vector = Vector::new("v1".into(), vec![1.0, 0.0, 0.0]);
        store.add(vector).await.unwrap();
        
        let count = store.count().await.unwrap();
        assert_eq!(count, 1);
        
        let results = store.search(vec![1.0, 0.0, 0.0], 10, None).await.unwrap();
        assert_eq!(results.len(), 1);
        assert!((results[0].score - 1.0).abs() < 0.001);
    }
    
    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let sim = InMemoryVectorStore::cosine_similarity(&a, &b);
        assert!((sim - 1.0).abs() < 0.001);
        
        let c = vec![0.0, 1.0, 0.0];
        let sim2 = InMemoryVectorStore::cosine_similarity(&a, &c);
        assert!(sim2.abs() < 0.001);
    }
}
