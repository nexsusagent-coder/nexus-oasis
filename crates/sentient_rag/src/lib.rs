// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Advanced RAG (Retrieval Augmented Generation)
// ═══════════════════════════════════════════════════════════════════════════════
//  Advanced retrieval augmented generation
//  - Multiple chunking strategies
//  - Hybrid search (vector + keyword)
//  - Re-ranking
//  - Query expansion
//  - Context compression
// ═══════════════════════════════════════════════════════════════════════════════

pub mod chunking;
pub mod retrieval;
pub mod reranking;
pub mod embeddings;
pub mod pipeline;
pub mod error;

pub use chunking::{Chunker, Chunk, ChunkingStrategy};
pub use retrieval::{Retriever, RetrievalResult, SearchType};
pub use reranking::{Reranker, RerankedResult};
pub use embeddings::{EmbeddingModel, EmbeddingVector};
pub use pipeline::{RAGPipeline, RAGConfig, RAGResult};
pub use error::{RAGError, Result};

use serde::{Deserialize, Serialize};

/// Document for RAG
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Document ID
    pub id: String,
    /// Document content
    pub content: String,
    /// Metadata
    pub metadata: std::collections::HashMap<String, String>,
    /// Embedding (if computed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding: Option<EmbeddingVector>,
}

impl Document {
    pub fn new(id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            content: content.into(),
            metadata: std::collections::HashMap::new(),
            embedding: None,
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn with_embedding(mut self, embedding: EmbeddingVector) -> Self {
        self.embedding = Some(embedding);
        self
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

/// Query for retrieval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    /// Query text
    pub text: String,
    /// Number of results to retrieve
    pub top_k: usize,
    /// Search type
    pub search_type: SearchType,
    /// Filters
    pub filters: std::collections::HashMap<String, String>,
}

impl Query {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            top_k: 5,
            search_type: SearchType::Hybrid,
            filters: std::collections::HashMap::new(),
        }
    }

    pub fn with_top_k(mut self, k: usize) -> Self {
        self.top_k = k;
        self
    }

    pub fn with_search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = search_type;
        self
    }

    pub fn with_filter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.filters.insert(key.into(), value.into());
        self
    }
}

/// Context for generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    /// Retrieved documents
    pub documents: Vec<RetrievalResult>,
    /// Combined context text
    pub combined_text: String,
    /// Total tokens (approximate)
    pub total_tokens: usize,
}

impl Context {
    pub fn new(documents: Vec<RetrievalResult>) -> Self {
        let combined_text = documents.iter()
            .map(|d| d.chunk.content.as_str())
            .collect::<Vec<_>>()
            .join("\n\n");
        
        let total_tokens = combined_text.split_whitespace().count();

        Self {
            documents,
            combined_text,
            total_tokens,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }

    pub fn document_count(&self) -> usize {
        self.documents.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = Document::new("doc1", "Hello world")
            .with_metadata("author", "test");

        assert_eq!(doc.id, "doc1");
        assert_eq!(doc.content, "Hello world");
        assert_eq!(doc.metadata.get("author"), Some(&"test".to_string()));
    }

    #[test]
    fn test_document_length() {
        let doc = Document::new("doc1", "Hello world");
        assert_eq!(doc.len(), 11);
        assert!(!doc.is_empty());
    }

    #[test]
    fn test_query_creation() {
        let query = Query::new("test query")
            .with_top_k(10)
            .with_search_type(SearchType::Vector);

        assert_eq!(query.text, "test query");
        assert_eq!(query.top_k, 10);
    }

    #[test]
    fn test_context_creation() {
        let results = vec![
            RetrievalResult {
                chunk: Chunk::new("doc1", "First document", 0, 13),
                score: 0.9,
            },
            RetrievalResult {
                chunk: Chunk::new("doc2", "Second document", 0, 14),
                score: 0.8,
            },
        ];

        let context = Context::new(results);
        assert_eq!(context.document_count(), 2);
        assert!(!context.combined_text.is_empty());
    }
}
