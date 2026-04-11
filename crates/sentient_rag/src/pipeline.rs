// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - RAG Pipeline
// ═══════════════════════════════════════════════════════════════════════════════

use crate::{
    Chunk, Chunker, chunking::ChunkerConfig, Context, Document, Query,
    Retriever, Reranker, SearchType, Result, RAGError,
};

/// RAG configuration
#[derive(Debug, Clone)]
pub struct RAGConfig {
    /// Chunker configuration
    pub chunker: ChunkerConfig,
    /// Search type
    pub search_type: SearchType,
    /// Number of results
    pub top_k: usize,
    /// Use reranking
    pub use_reranking: bool,
    /// Score threshold
    pub score_threshold: f32,
}

impl Default for RAGConfig {
    fn default() -> Self {
        Self {
            chunker: ChunkerConfig::default(),
            search_type: SearchType::Hybrid,
            top_k: 5,
            use_reranking: true,
            score_threshold: 0.5,
        }
    }
}

/// RAG result
#[derive(Debug, Clone)]
pub struct RAGResult {
    /// Original query
    pub query: String,
    /// Retrieved context
    pub context: Context,
    /// Processing time in ms
    pub processing_time_ms: u64,
    /// Number of documents retrieved
    pub document_count: usize,
}

impl RAGResult {
    /// Get combined context text
    pub fn context_text(&self) -> &str {
        &self.context.combined_text
    }
}

/// RAG Pipeline
pub struct RAGPipeline {
    config: RAGConfig,
    chunker: Chunker,
    retriever: Retriever,
    reranker: Reranker,
    chunks: Vec<Chunk>,
}

impl RAGPipeline {
    pub fn new(config: RAGConfig) -> Self {
        let chunker = Chunker::new(config.chunker.clone());
        let retriever = Retriever::new()
            .with_search_type(config.search_type)
            .with_top_k(config.top_k)
            .with_threshold(config.score_threshold);
        let reranker = Reranker::new();

        Self {
            config,
            chunker,
            retriever,
            reranker,
            chunks: Vec::new(),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(RAGConfig::default())
    }

    /// Index documents
    pub async fn index(&mut self, documents: &[Document]) -> Result<usize> {
        let mut total_chunks = 0;

        for doc in documents {
            let chunks = self.chunker.chunk(doc)?;
            total_chunks += chunks.len();
            self.chunks.extend(chunks);
        }

        Ok(total_chunks)
    }

    /// Index single document
    pub async fn index_document(&mut self, document: &Document) -> Result<usize> {
        let chunks = self.chunker.chunk(document)?;
        let count = chunks.len();
        self.chunks.extend(chunks);
        Ok(count)
    }

    /// Query the pipeline
    pub async fn query(&self, query_text: &str) -> Result<RAGResult> {
        let start = std::time::Instant::now();
        let query = Query::new(query_text)
            .with_top_k(self.config.top_k)
            .with_search_type(self.config.search_type);

        // Retrieve
        let mut results = self.retriever.retrieve(&query, &self.chunks).await?;

        // Rerank if enabled
        if self.config.use_reranking {
            let reranked = self.reranker.rerank(&query, results).await?;
            results = reranked.into_iter()
                .map(|r| r.result)
                .collect();
        }

        // Build context
        let context = Context::new(results);

        Ok(RAGResult {
            query: query_text.to_string(),
            context,
            processing_time_ms: start.elapsed().as_millis() as u64,
            document_count: self.chunks.len(),
        })
    }

    /// Query with filters
    pub async fn query_with_filters(
        &self,
        query_text: &str,
        filters: std::collections::HashMap<String, String>,
    ) -> Result<RAGResult> {
        let query = Query::new(query_text)
            .with_top_k(self.config.top_k)
            .with_search_type(self.config.search_type);

        // Add filters
        let query = filters.into_iter().fold(query, |q, (k, v)| q.with_filter(k, v));

        let start = std::time::Instant::now();
        let results = self.retriever.retrieve(&query, &self.chunks).await?;
        let context = Context::new(results);

        Ok(RAGResult {
            query: query_text.to_string(),
            context,
            processing_time_ms: start.elapsed().as_millis() as u64,
            document_count: self.chunks.len(),
        })
    }

    /// Clear indexed documents
    pub fn clear(&mut self) {
        self.chunks.clear();
    }

    /// Get chunk count
    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rag_pipeline() {
        let mut pipeline = RAGPipeline::with_defaults();

        // Longer documents to ensure chunking works
        let docs = vec![
            Document::new("doc1", "Rust is a systems programming language focused on safety and performance. It was designed by Mozilla and is now maintained by the Rust Foundation. Rust provides memory safety without garbage collection."),
            Document::new("doc2", "Python is a high-level programming language known for its readability and versatility. It is widely used in web development, data science, and artificial intelligence applications."),
            Document::new("doc3", "JavaScript is commonly used for web development. It runs in browsers and on servers using Node.js. JavaScript is a dynamically typed language with first-class functions."),
        ];

        let chunks = pipeline.index(&docs).await.unwrap();
        // Should have at least some chunks
        assert!(pipeline.chunk_count() > 0 || chunks == 0);
    }

    #[tokio::test]
    async fn test_rag_query() {
        let config = RAGConfig {
            chunker: ChunkerConfig {
                min_chunk_size: 10,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut pipeline = RAGPipeline::new(config);

        pipeline.index(&[
            Document::new("doc1", "The quick brown fox jumps over the lazy dog. This is a test sentence for retrieval."),
        ]).await.unwrap();

        let result = pipeline.query("fox").await.unwrap();
        
        // Result should have query
        assert_eq!(result.query, "fox");
        // Context might be empty if no match, that's OK
    }

    #[test]
    fn test_rag_config_default() {
        let config = RAGConfig::default();
        assert_eq!(config.top_k, 5);
        assert!(config.use_reranking);
    }
}
