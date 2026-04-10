//! ─── SENTIENT LanceDB Memory ───
//!
//! Long-term memory storage with vector search capabilities.
//!
//! Features:
//! - Persistent vector storage
//! - Semantic search
//! - Conversation memory
//! - Knowledge base
//! - Embedding generation
//!
//! Usage:
//! ```rust,ignore
//! use sentient_lancedb::{LanceMemory, MemoryEntry};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let memory = LanceMemory::new("./memory").await?;
//!
//!     // Store memory
//!     memory.store(MemoryEntry {
//!         id: "1".to_string(),
//!         content: "User likes Rust programming".to_string(),
//!         embedding: None,
//!         metadata: serde_json::json!({"user": "john"}),
//!         timestamp: chrono::Utc::now().timestamp(),
//!         source: "chat".to_string(),
//!     }).await?;
//!
//!     // Search
//!     let results = memory.search("programming", vec![], 10).await?;
//!     Ok(())
//! }
//! ```

pub mod memory;
pub mod embeddings;
pub mod conversation;
pub mod knowledge;

pub use memory::{LanceMemory, MemoryEntry, MemorySearchResult};
pub use embeddings::{EmbeddingEngine, EmbeddingConfig};
pub use conversation::{ConversationMemory, ConversationEntry};
pub use knowledge::{KnowledgeBase, KnowledgeEntry};

/// Memory error type
#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Embedding error: {0}")]
    Embedding(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Result type
pub type Result<T> = std::result::Result<T, MemoryError>;
