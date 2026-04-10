//! RAG types and data structures

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Document ID
pub type DocumentId = String;

/// Chunk ID
pub type ChunkId = String;

/// Embedding vector
pub type EmbeddingVector = Vec<f32>;

/// Document for RAG
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Unique document ID
    pub id: DocumentId,
    /// Document content
    pub content: String,
    /// Metadata
    pub metadata: DocumentMetadata,
    /// Source information
    pub source: Option<DocumentSource>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Updated timestamp
    pub updated_at: DateTime<Utc>,
}

impl Document {
    /// Create new document
    pub fn new(content: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            content: content.into(),
            metadata: DocumentMetadata::default(),
            source: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Create document with ID
    pub fn with_id(id: impl Into<String>, content: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: id.into(),
            content: content.into(),
            metadata: DocumentMetadata::default(),
            source: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.custom.insert(key.into(), value.into());
        self
    }

    /// Set source
    pub fn with_source(mut self, source: DocumentSource) -> Self {
        self.source = Some(source);
        self
    }

    /// Set title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.metadata.title = Some(title.into());
        self
    }

    /// Get content length
    pub fn len(&self) -> usize {
        self.content.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

/// Document metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocumentMetadata {
    /// Document title
    pub title: Option<String>,
    /// Author
    pub author: Option<String>,
    /// Language
    pub language: Option<String>,
    /// Document type
    pub doc_type: Option<String>,
    /// Tags
    pub tags: Vec<String>,
    /// Custom metadata
    #[serde(default)]
    pub custom: HashMap<String, String>,
}

/// Document source information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSource {
    /// Source type
    pub source_type: SourceType,
    /// Source URI/URL/path
    pub uri: String,
    /// Page number (for PDFs)
    pub page: Option<u32>,
    /// Line number
    pub line: Option<u32>,
}

/// Source type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    File,
    Url,
    Database,
    Api,
    Stream,
    Memory,
}

/// Document chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    /// Unique chunk ID
    pub id: ChunkId,
    /// Parent document ID
    pub document_id: DocumentId,
    /// Chunk content
    pub content: String,
    /// Chunk index in document
    pub index: usize,
    /// Start position in original document
    pub start_char: usize,
    /// End position in original document
    pub end_char: usize,
    /// Token count (approximate)
    pub token_count: Option<usize>,
    /// Embedding (if computed)
    pub embedding: Option<EmbeddingVector>,
    /// Chunk metadata
    pub metadata: ChunkMetadata,
}

impl Chunk {
    /// Create new chunk
    pub fn new(document_id: impl Into<String>, content: impl Into<String>, index: usize) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            document_id: document_id.into(),
            content: content.into(),
            index,
            start_char: 0,
            end_char: 0,
            token_count: None,
            embedding: None,
            metadata: ChunkMetadata::default(),
        }
    }

    /// Set position
    pub fn with_position(mut self, start: usize, end: usize) -> Self {
        self.start_char = start;
        self.end_char = end;
        self
    }

    /// Set embedding
    pub fn with_embedding(mut self, embedding: EmbeddingVector) -> Self {
        self.embedding = Some(embedding);
        self
    }

    /// Get content length
    pub fn len(&self) -> usize {
        self.content.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

/// Chunk metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChunkMetadata {
    /// Overlap with previous chunk
    pub overlap_previous: usize,
    /// Overlap with next chunk
    pub overlap_next: usize,
    /// Chunk type
    pub chunk_type: ChunkType,
    /// Heading (if chunk starts with a heading)
    pub heading: Option<String>,
    /// Custom metadata
    #[serde(default)]
    pub custom: HashMap<String, String>,
}

/// Chunk type
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChunkType {
    #[default]
    Text,
    Code,
    Heading,
    List,
    Table,
    Image,
    Quote,
}

/// Search query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    /// Query text
    pub text: String,
    /// Query embedding (optional, will be computed if not provided)
    pub embedding: Option<EmbeddingVector>,
    /// Number of results
    #[serde(default = "default_top_k")]
    pub top_k: usize,
    /// Minimum similarity score (0.0 - 1.0)
    #[serde(default)]
    pub min_score: f32,
    /// Filter by metadata
    #[serde(default)]
    pub filters: HashMap<String, String>,
    /// Filter by document IDs
    #[serde(default)]
    pub document_ids: Vec<DocumentId>,
}

fn default_top_k() -> usize { 5 }

impl SearchQuery {
    /// Create new query
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            embedding: None,
            top_k: default_top_k(),
            min_score: 0.0,
            filters: HashMap::new(),
            document_ids: Vec::new(),
        }
    }

    /// Set top_k
    pub fn with_top_k(mut self, k: usize) -> Self {
        self.top_k = k;
        self
    }

    /// Set minimum score
    pub fn with_min_score(mut self, score: f32) -> Self {
        self.min_score = score;
        self
    }

    /// Add filter
    pub fn with_filter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.filters.insert(key.into(), value.into());
        self
    }

    /// Set embedding
    pub fn with_embedding(mut self, embedding: EmbeddingVector) -> Self {
        self.embedding = Some(embedding);
        self
    }
}

/// Search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Chunk
    pub chunk: Chunk,
    /// Similarity score (0.0 - 1.0)
    pub score: f32,
    /// Document metadata
    pub document_metadata: Option<DocumentMetadata>,
}

impl SearchResult {
    /// Create new result
    pub fn new(chunk: Chunk, score: f32) -> Self {
        Self {
            chunk,
            score,
            document_metadata: None,
        }
    }
}

/// RAG response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagResponse {
    /// Generated response
    pub response: String,
    /// Source chunks used
    pub sources: Vec<SearchResult>,
    /// Total chunks retrieved
    pub retrieved_count: usize,
    /// Processing time in ms
    pub processing_time_ms: u64,
    /// Model used
    pub model: Option<String>,
}

/// Chunking strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChunkingStrategy {
    /// Fixed character size
    FixedSize,
    /// Sentence-based
    Sentence,
    /// Paragraph-based
    Paragraph,
    /// Recursive (try smaller chunks)
    Recursive,
    /// Semantic (requires embeddings)
    Semantic,
    /// Code-aware
    Code,
}

/// Chunking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkingConfig {
    /// Chunking strategy
    pub strategy: ChunkingStrategy,
    /// Target chunk size (characters)
    pub chunk_size: usize,
    /// Overlap between chunks
    pub overlap: usize,
    /// Minimum chunk size
    pub min_chunk_size: usize,
    /// Maximum chunk size
    pub max_chunk_size: usize,
    /// Respect sentence boundaries
    pub respect_sentence_boundary: bool,
    /// Respect paragraph boundaries
    pub respect_paragraph_boundary: bool,
    /// Separator characters
    pub separators: Vec<String>,
}

impl Default for ChunkingConfig {
    fn default() -> Self {
        Self {
            strategy: ChunkingStrategy::Recursive,
            chunk_size: 512,
            overlap: 50,
            min_chunk_size: 100,
            max_chunk_size: 1024,
            respect_sentence_boundary: true,
            respect_paragraph_boundary: true,
            separators: vec![
                "\n\n".to_string(),
                "\n".to_string(),
                ". ".to_string(),
                "! ".to_string(),
                "? ".to_string(),
                " ".to_string(),
            ],
        }
    }
}

impl ChunkingConfig {
    /// Create fixed size config
    pub fn fixed_size(size: usize) -> Self {
        Self {
            strategy: ChunkingStrategy::FixedSize,
            chunk_size: size,
            ..Default::default()
        }
    }

    /// Create sentence config
    pub fn sentence() -> Self {
        Self {
            strategy: ChunkingStrategy::Sentence,
            chunk_size: 256,
            overlap: 20,
            ..Default::default()
        }
    }

    /// Create code config
    pub fn code() -> Self {
        Self {
            strategy: ChunkingStrategy::Code,
            chunk_size: 512,
            overlap: 50,
            separators: vec![
                "\n\n".to_string(),
                "\n".to_string(),
                "fn ".to_string(),
                "struct ".to_string(),
                "impl ".to_string(),
                "class ".to_string(),
                "def ".to_string(),
                "function ".to_string(),
            ],
            ..Default::default()
        }
    }
}

/// Embedding model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingConfig {
    /// Model name
    pub model: String,
    /// Embedding dimension
    pub dimension: usize,
    /// Batch size
    pub batch_size: usize,
    /// Use cache
    pub use_cache: bool,
    /// Cache size
    pub cache_size: usize,
    /// Normalize embeddings
    pub normalize: bool,
}

impl Default for EmbeddingConfig {
    fn default() -> Self {
        Self {
            model: "all-MiniLM-L6-v2".to_string(),
            dimension: 384,
            batch_size: 32,
            use_cache: true,
            cache_size: 10000,
            normalize: true,
        }
    }
}

/// Index configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexConfig {
    /// Index name
    pub name: String,
    /// Embedding dimension
    pub dimension: usize,
    /// Distance metric
    pub metric: DistanceMetric,
    /// Index type
    pub index_type: IndexType,
    /// Number of lists for IVF index
    pub nlist: usize,
}

impl Default for IndexConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            dimension: 384,
            metric: DistanceMetric::Cosine,
            index_type: IndexType::Flat,
            nlist: 100,
        }
    }
}

/// Distance metric
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DistanceMetric {
    #[default]
    Cosine,
    Euclidean,
    DotProduct,
    Manhattan,
}

/// Index type
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IndexType {
    #[default]
    Flat,
    Ivf,
    Hnsw,
    Lsh,
}

/// RAG pipeline configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagConfig {
    /// Chunking configuration
    pub chunking: ChunkingConfig,
    /// Embedding configuration
    pub embedding: EmbeddingConfig,
    /// Index configuration
    pub index: IndexConfig,
    /// Enable caching
    pub enable_cache: bool,
    /// Maximum documents to process in parallel
    pub max_parallel: usize,
}

impl Default for RagConfig {
    fn default() -> Self {
        Self {
            chunking: ChunkingConfig::default(),
            embedding: EmbeddingConfig::default(),
            index: IndexConfig::default(),
            enable_cache: true,
            max_parallel: 10,
        }
    }
}

/// RAG statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RagStats {
    /// Total documents
    pub total_documents: usize,
    /// Total chunks
    pub total_chunks: usize,
    /// Total embeddings
    pub total_embeddings: usize,
    /// Index size in bytes
    pub index_size_bytes: usize,
    /// Cache hits
    pub cache_hits: u64,
    /// Cache misses
    pub cache_misses: u64,
    /// Average retrieval time ms
    pub avg_retrieval_time_ms: f64,
    /// Total queries
    pub total_queries: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = Document::new("Test content");
        assert!(!doc.id.is_empty());
        assert_eq!(doc.content, "Test content");
    }

    #[test]
    fn test_document_with_metadata() {
        let doc = Document::new("Test")
            .with_title("My Doc")
            .with_metadata("author", "Test Author");

        assert_eq!(doc.metadata.title, Some("My Doc".to_string()));
        assert_eq!(doc.metadata.custom.get("author"), Some(&"Test Author".to_string()));
    }

    #[test]
    fn test_chunk_creation() {
        let chunk = Chunk::new("doc-1", "Chunk content", 0)
            .with_position(0, 13);

        assert_eq!(chunk.document_id, "doc-1");
        assert_eq!(chunk.index, 0);
        assert_eq!(chunk.start_char, 0);
        assert_eq!(chunk.end_char, 13);
    }

    #[test]
    fn test_search_query() {
        let query = SearchQuery::new("test query")
            .with_top_k(10)
            .with_min_score(0.5);

        assert_eq!(query.text, "test query");
        assert_eq!(query.top_k, 10);
        assert_eq!(query.min_score, 0.5);
    }

    #[test]
    fn test_chunking_config_default() {
        let config = ChunkingConfig::default();
        assert_eq!(config.chunk_size, 512);
        assert_eq!(config.overlap, 50);
    }

    #[test]
    fn test_chunking_config_presets() {
        let fixed = ChunkingConfig::fixed_size(1024);
        assert_eq!(fixed.strategy, ChunkingStrategy::FixedSize);
        assert_eq!(fixed.chunk_size, 1024);

        let code = ChunkingConfig::code();
        assert_eq!(code.strategy, ChunkingStrategy::Code);
    }
}
