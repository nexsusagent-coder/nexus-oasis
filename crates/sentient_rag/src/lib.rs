//! # Sentient RAG
//!
//! Native RAG (Retrieval-Augmented Generation) Engine for SENTIENT OS.
//!
//! ## Features
//!
//! - **Document Chunking**: Multiple strategies (fixed, sentence, paragraph, recursive, code)
//! - **Embeddings**: Local (fastembed) and remote (OpenAI, etc.) embedding generation
//! - **Vector Store**: In-memory and LanceDB for persistent storage
//! - **Semantic Search**: Cosine, Euclidean, Dot Product distance metrics
//! - **RAG Pipeline**: Complete end-to-end RAG workflow
//!
//! ## Example
//!
//! ```rust
//! use sentient_rag::{RagPipeline, Document, ChunkingConfig, ChunkingStrategy};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create pipeline with defaults
//! let pipeline = RagPipeline::default_pipeline()?;
//!
//! // Ingest documents
//! let doc = Document::new("The quick brown fox jumps over the lazy dog.");
//! pipeline.ingest(&doc).await?;
//!
//! // Query
//! let response = pipeline.query("What does the fox do?").await?;
//! println!("Response: {}", response.response);
//!
//! // Advanced configuration
//! let pipeline = RagPipeline::builder()
//!     .chunk_size(512)
//!     .chunk_overlap(50)
//!     .chunking_strategy(ChunkingStrategy::Recursive)
//!     .embedding_dimension(384)
//!     .distance_metric(sentient_rag::DistanceMetric::Cosine)
//!     .build()
//!     .await?;
//!
//! # Ok(())
//! # }
//! ```

pub mod error;
pub mod types;
pub mod chunker;
pub mod embedder;
pub mod store;
pub mod retriever;
pub mod pipeline;

pub use error::{RagError, Result};
pub use types::*;
pub use chunker::Chunker;
pub use embedder::{Embedder, cosine_similarity, euclidean_distance, dot_product, normalize_embedding};
pub use store::{VectorStore, MemoryStore, StoreBuilder, StoreStats};
pub use retriever::{Retriever, RetrievalResult, RetrieverConfig};
pub use pipeline::{RagPipeline, RagPipelineBuilder, IngestResult};

/// RAG version
pub const RAG_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!RAG_VERSION.is_empty());
    }

    #[tokio::test]
    async fn test_full_workflow() {
        // Create pipeline
        let pipeline = RagPipeline::builder()
            .chunk_size(100)
            .embedding_dimension(32)
            .build()
            .await
            .unwrap();

        // Ingest documents
        let doc1 = Document::new("Rust is a systems programming language.");

        pipeline.ingest(&doc1).await.unwrap();

        // Query
        let response = pipeline.query("programming").await.unwrap();

        assert!(!response.response.is_empty());
    }

    #[tokio::test]
    async fn test_chunking_strategies() {
        let text = "First paragraph.\n\nSecond paragraph.";

        // Test different strategies
        let strategies = [
            ChunkingStrategy::FixedSize,
            ChunkingStrategy::Paragraph,
            ChunkingStrategy::Recursive,
        ];

        for strategy in strategies {
            let config = ChunkingConfig {
                strategy,
                chunk_size: 50,
                ..Default::default()
            };
            let chunker = Chunker::new(config);
            let chunks = chunker.chunk_text(text).unwrap();
            assert!(!chunks.is_empty(), "Failed for {:?}", strategy);
        }
    }

    #[tokio::test]
    async fn test_search_with_filters() {
        let pipeline = RagPipeline::builder()
            .embedding_dimension(32)
            .build()
            .await
            .unwrap();

        let doc = Document::new("Test content");
        pipeline.ingest(&doc).await.unwrap();

        let mut filters = std::collections::HashMap::new();
        filters.insert("category".to_string(), "test".to_string());

        let response = pipeline.query_filtered("test", filters).await.unwrap();
        assert!(response.processing_time_ms >= 0);
    }
}
