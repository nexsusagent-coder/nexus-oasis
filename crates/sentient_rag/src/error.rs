// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - RAG Errors
// ═══════════════════════════════════════════════════════════════════════════════

use thiserror::Error;

pub type Result<T> = std::result::Result<T, RAGError>;

/// RAG errors
#[derive(Debug, Error)]
pub enum RAGError {
    #[error("Chunking failed: {0}")]
    ChunkingFailed(String),

    #[error("Embedding failed: {0}")]
    EmbeddingFailed(String),

    #[error("Retrieval failed: {0}")]
    RetrievalFailed(String),

    #[error("Index not found: {0}")]
    IndexNotFound(String),

    #[error("Document not found: {0}")]
    DocumentNotFound(String),

    #[error("Query too long: {0} characters")]
    QueryTooLong(usize),

    #[error("No documents indexed")]
    NoDocumentsIndexed,

    #[error("Vector dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(String),
}
