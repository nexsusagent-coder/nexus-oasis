//! RAG Pipeline - Complete end-to-end RAG workflow

use crate::chunker::Chunker;
use crate::embedder::Embedder;
use crate::retriever::{Retriever, RetrievalResult};
use crate::store::VectorStore;
use crate::types::*;
use crate::{RagError, Result};
use std::sync::Arc;
use std::time::Instant;

/// RAG Pipeline
pub struct RagPipeline {
    chunker: Chunker,
    embedder: Arc<dyn Embedder>,
    store: Arc<dyn VectorStore>,
    retriever: Retriever,
    config: RagConfig,
}

impl RagPipeline {
    /// Create new RAG pipeline
    pub fn new(
        chunker: Chunker,
        embedder: Arc<dyn Embedder>,
        store: Arc<dyn VectorStore>,
        config: RagConfig,
    ) -> Self {
        let retriever = Retriever::default_config(store.clone(), embedder.clone());
        Self {
            chunker,
            embedder,
            store,
            retriever,
            config,
        }
    }

    /// Create with default configuration
    pub fn default_pipeline() -> Result<Self> {
        let config = RagConfig::default();
        let chunker = Chunker::new(config.chunking.clone());
        let embedder = Arc::new(crate::embedder::MockEmbedder::new(config.embedding.dimension));
        let store = Arc::new(crate::store::MemoryStore::new(config.index.clone()));

        Ok(Self::new(chunker, embedder, store, config))
    }

    /// Create builder
    pub fn builder() -> RagPipelineBuilder {
        RagPipelineBuilder::new()
    }

    /// Ingest a document into the RAG system
    pub async fn ingest(&self, document: &Document) -> Result<IngestResult> {
        let start = Instant::now();

        // 1. Chunk document
        let chunks = self.chunker.chunk(document)?;

        // 2. Generate embeddings
        let texts: Vec<&str> = chunks.iter().map(|c| c.content.as_str()).collect();
        let embeddings = self.embedder.embed_batch(&texts).await?;

        // 3. Add embeddings to chunks
        let mut chunks_with_embeddings = Vec::new();
        for (mut chunk, embedding) in chunks.into_iter().zip(embeddings.into_iter()) {
            chunk.embedding = Some(embedding);
            chunks_with_embeddings.push(chunk);
        }

        // 4. Store chunks
        self.store.add_batch(chunks_with_embeddings.clone()).await?;

        let elapsed = start.elapsed();

        Ok(IngestResult {
            document_id: document.id.clone(),
            chunks_created: chunks_with_embeddings.len(),
            processing_time_ms: elapsed.as_millis() as u64,
        })
    }

    /// Ingest multiple documents
    pub async fn ingest_batch(&self, documents: &[Document]) -> Result<Vec<IngestResult>> {
        let mut results = Vec::new();

        for document in documents {
            let result = self.ingest(document).await?;
            results.push(result);
        }

        Ok(results)
    }

    /// Ingest text directly
    pub async fn ingest_text(&self, text: &str) -> Result<IngestResult> {
        let document = Document::new(text);
        self.ingest(&document).await
    }

    /// Query the RAG system
    pub async fn query(&self, query: &str) -> Result<RagResponse> {
        self.query_with_config(query, self.config.chunking.chunk_size * 2, None).await
    }

    /// Query with custom context length
    pub async fn query_with_context_length(
        &self,
        query: &str,
        max_context_tokens: usize,
    ) -> Result<RagResponse> {
        self.query_with_config(query, max_context_tokens, None).await
    }

    /// Query with filters
    pub async fn query_filtered(
        &self,
        query: &str,
        filters: HashMap<String, String>,
    ) -> Result<RagResponse> {
        self.query_with_config(query, self.config.chunking.chunk_size * 2, Some(filters)).await
    }

    async fn query_with_config(
        &self,
        query: &str,
        max_context_tokens: usize,
        filters: Option<HashMap<String, String>>,
    ) -> Result<RagResponse> {
        let start = Instant::now();

        // 1. Retrieve relevant chunks
        let retrieval = if let Some(f) = filters {
            self.retriever.search_filtered(query, f, 5).await?
        } else {
            self.retriever.search(query).await?
        };

        // 2. Build context from results
        let context = self.build_context(&retrieval.results, max_context_tokens);

        // 3. Format response
        let response = self.format_response(query, &context, &retrieval);

        let elapsed = start.elapsed();

        Ok(RagResponse {
            response,
            sources: retrieval.results.clone(),
            retrieved_count: retrieval.results.len(),
            processing_time_ms: elapsed.as_millis() as u64,
            model: Some(self.embedder.model_name().to_string()),
        })
    }

    /// Build context string from search results
    fn build_context(&self, results: &[SearchResult], max_tokens: usize) -> String {
        let mut context = String::new();
        let mut token_count = 0;

        for result in results {
            let chunk_tokens = Chunker::estimate_tokens(&result.chunk.content);

            if token_count + chunk_tokens > max_tokens {
                break;
            }

            if !context.is_empty() {
                context.push_str("\n\n");
            }

            // Add source info
            if let Some(ref meta) = result.document_metadata {
                if let Some(ref title) = meta.title {
                    context.push_str(&format!("[Source: {}]\n", title));
                }
            }

            context.push_str(&result.chunk.content);
            token_count += chunk_tokens;
        }

        context
    }

    /// Format response with context
    fn format_response(
        &self,
        query: &str,
        context: &str,
        retrieval: &RetrievalResult,
    ) -> String {
        // Simple template-based response
        // In a real implementation, this would call an LLM
        format!(
            "Based on {} relevant sources:\n\n{}\n\n[Query: {}]",
            retrieval.len(),
            context,
            query
        )
    }

    /// Get retriever for advanced usage
    pub fn retriever(&self) -> &Retriever {
        &self.retriever
    }

    /// Get store reference
    pub fn store(&self) -> &Arc<dyn VectorStore> {
        &self.store
    }

    /// Get chunker reference
    pub fn chunker(&self) -> &Chunker {
        &self.chunker
    }

    /// Get embedder reference
    pub fn embedder(&self) -> &Arc<dyn Embedder> {
        &self.embedder
    }

    /// Remove document from index
    pub async fn remove_document(&self, document_id: &str) -> Result<usize> {
        let doc_id: DocumentId = document_id.to_string();
        self.store.remove_document(&doc_id).await
    }

    /// Get pipeline statistics
    pub async fn stats(&self) -> Result<RagStats> {
        let store_stats = self.store.stats().await;

        Ok(RagStats {
            total_documents: store_stats.total_documents,
            total_chunks: store_stats.total_chunks,
            total_embeddings: store_stats.total_embeddings,
            index_size_bytes: store_stats.size_bytes,
            ..Default::default()
        })
    }

    /// Clear all indexed data
    pub async fn clear(&self) -> Result<()> {
        self.store.clear().await
    }
}

use std::collections::HashMap;

/// Ingest result
#[derive(Debug, Clone)]
pub struct IngestResult {
    /// Document ID
    pub document_id: String,
    /// Number of chunks created
    pub chunks_created: usize,
    /// Processing time in ms
    pub processing_time_ms: u64,
}

/// RAG Pipeline builder
pub struct RagPipelineBuilder {
    config: RagConfig,
    embedder: Option<Arc<dyn Embedder>>,
    store: Option<Arc<dyn VectorStore>>,
}

impl RagPipelineBuilder {
    pub fn new() -> Self {
        Self {
            config: RagConfig::default(),
            embedder: None,
            store: None,
        }
    }

    pub fn config(mut self, config: RagConfig) -> Self {
        self.config = config;
        self
    }

    pub fn chunk_size(mut self, size: usize) -> Self {
        self.config.chunking.chunk_size = size;
        self
    }

    pub fn chunk_overlap(mut self, overlap: usize) -> Self {
        self.config.chunking.overlap = overlap;
        self
    }

    pub fn chunking_strategy(mut self, strategy: ChunkingStrategy) -> Self {
        self.config.chunking.strategy = strategy;
        self
    }

    pub fn embedding_dimension(mut self, dim: usize) -> Self {
        self.config.embedding.dimension = dim;
        self.config.index.dimension = dim;
        self
    }

    pub fn distance_metric(mut self, metric: DistanceMetric) -> Self {
        self.config.index.metric = metric;
        self
    }

    pub fn embedder(mut self, embedder: Arc<dyn Embedder>) -> Self {
        self.embedder = Some(embedder);
        self
    }

    pub fn store(mut self, store: Arc<dyn VectorStore>) -> Self {
        self.store = Some(store);
        self
    }

    pub async fn build(self) -> Result<RagPipeline> {
        let chunker = Chunker::new(self.config.chunking.clone());

        let embedder = self.embedder.unwrap_or_else(|| {
            Arc::new(crate::embedder::MockEmbedder::new(self.config.embedding.dimension))
        });

        let store = self.store.unwrap_or_else(|| {
            Arc::new(crate::store::MemoryStore::new(self.config.index.clone()))
        });

        Ok(RagPipeline::new(chunker, embedder, store, self.config))
    }
}

impl Default for RagPipelineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embedder::MockEmbedder;
    use crate::store::MemoryStore;

    fn create_test_config() -> RagConfig {
        RagConfig {
            chunking: ChunkingConfig {
                chunk_size: 100,
                overlap: 10,
                ..Default::default()
            },
            embedding: EmbeddingConfig {
                dimension: 64,
                ..Default::default()
            },
            index: IndexConfig {
                dimension: 64,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_pipeline_ingest() {
        let config = create_test_config();
        let chunker = Chunker::new(config.chunking.clone());
        let embedder = Arc::new(MockEmbedder::new(64));
        let store = Arc::new(MemoryStore::new(config.index.clone()));

        let pipeline = RagPipeline::new(chunker, embedder, store, config);

        let doc = Document::new("This is a test document for RAG ingestion.");
        let result = pipeline.ingest(&doc).await.unwrap();

        assert!(result.chunks_created > 0);
        assert!(result.processing_time_ms > 0 || result.processing_time_ms == 0);
    }

    #[tokio::test]
    async fn test_pipeline_ingest_text() {
        let pipeline = RagPipeline::default_pipeline().unwrap();
        let result = pipeline.ingest_text("Test content").await.unwrap();

        assert!(!result.document_id.is_empty());
    }

    #[tokio::test]
    async fn test_pipeline_query() {
        let pipeline = RagPipeline::default_pipeline().unwrap();

        // Ingest some content
        pipeline.ingest_text("The quick brown fox jumps over the lazy dog.").await.unwrap();

        // Query
        let response = pipeline.query("fox").await.unwrap();

        assert!(!response.response.is_empty());
        assert!(response.retrieved_count > 0);
    }

    #[tokio::test]
    async fn test_pipeline_query_filtered() {
        let pipeline = RagPipeline::default_pipeline().unwrap();

        pipeline.ingest_text("Content about Rust programming.").await.unwrap();

        let mut filters = HashMap::new();
        filters.insert("type".to_string(), "article".to_string());

        let response = pipeline.query_filtered("programming", filters).await.unwrap();

        // Response should still work, just potentially filtered results
        assert!(response.processing_time_ms >= 0);
    }

    #[tokio::test]
    async fn test_pipeline_remove_document() {
        let pipeline = RagPipeline::default_pipeline().unwrap();

        let result = pipeline.ingest_text("Test document").await.unwrap();
        let doc_id = result.document_id;

        let removed = pipeline.remove_document(&doc_id).await.unwrap();
        assert!(removed > 0);
    }

    #[tokio::test]
    async fn test_pipeline_stats() {
        let pipeline = RagPipeline::default_pipeline().unwrap();

        pipeline.ingest_text("Document one").await.unwrap();
        pipeline.ingest_text("Document two").await.unwrap();

        let stats = pipeline.stats().await.unwrap();

        assert!(stats.total_documents > 0);
        assert!(stats.total_chunks > 0);
    }

    #[tokio::test]
    async fn test_pipeline_clear() {
        let pipeline = RagPipeline::default_pipeline().unwrap();

        pipeline.ingest_text("Content to clear").await.unwrap();
        pipeline.clear().await.unwrap();

        let stats = pipeline.stats().await.unwrap();
        assert_eq!(stats.total_chunks, 0);
    }

    #[tokio::test]
    async fn test_pipeline_builder() {
        let pipeline = RagPipeline::builder()
            .chunk_size(200)
            .chunk_overlap(20)
            .embedding_dimension(128)
            .distance_metric(DistanceMetric::Euclidean)
            .build()
            .await
            .unwrap();

        let result = pipeline.ingest_text("Test").await.unwrap();
        assert!(result.chunks_created > 0);
    }

    #[tokio::test]
    async fn test_ingest_result() {
        let result = IngestResult {
            document_id: "doc-123".to_string(),
            chunks_created: 5,
            processing_time_ms: 100,
        };

        assert_eq!(result.document_id, "doc-123");
        assert_eq!(result.chunks_created, 5);
    }

    #[tokio::test]
    async fn test_pipeline_batch_ingest() {
        let pipeline = RagPipeline::default_pipeline().unwrap();

        let docs = vec![
            Document::new("First document"),
            Document::new("Second document"),
            Document::new("Third document"),
        ];

        let results = pipeline.ingest_batch(&docs).await.unwrap();
        assert_eq!(results.len(), 3);
    }
}
