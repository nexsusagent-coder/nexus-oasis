//! Retrieval and search functionality

use crate::types::*;
use crate::embedder::Embedder;
use crate::store::VectorStore;
use crate::{RagError, Result};
use std::sync::Arc;
use std::time::Instant;

/// Retriever for semantic search
pub struct Retriever {
    store: Arc<dyn VectorStore>,
    embedder: Arc<dyn Embedder>,
    config: RetrieverConfig,
}

/// Retriever configuration
#[derive(Debug, Clone)]
pub struct RetrieverConfig {
    /// Default number of results
    pub default_top_k: usize,
    /// Minimum similarity score
    pub min_score: f32,
    /// Enable query caching
    pub cache_queries: bool,
    /// Maximum query length
    pub max_query_length: usize,
}

impl Default for RetrieverConfig {
    fn default() -> Self {
        Self {
            default_top_k: 5,
            min_score: 0.0,
            cache_queries: true,
            max_query_length: 10000,
        }
    }
}

impl Retriever {
    /// Create new retriever
    pub fn new(
        store: Arc<dyn VectorStore>,
        embedder: Arc<dyn Embedder>,
        config: RetrieverConfig,
    ) -> Self {
        Self { store, embedder, config }
    }

    /// Create with default config
    pub fn default_config(store: Arc<dyn VectorStore>, embedder: Arc<dyn Embedder>) -> Self {
        Self::new(store, embedder, RetrieverConfig::default())
    }

    /// Search for similar chunks
    pub async fn search(&self, query: &str) -> Result<RetrievalResult> {
        self.search_with_config(query, None, self.config.default_top_k).await
    }

    /// Search with custom top_k
    pub async fn search_top_k(&self, query: &str, top_k: usize) -> Result<RetrievalResult> {
        self.search_with_config(query, None, top_k).await
    }

    /// Search with filters
    pub async fn search_filtered(
        &self,
        query: &str,
        filters: HashMap<String, String>,
        top_k: usize,
    ) -> Result<RetrievalResult> {
        self.search_with_config(query, Some(filters), top_k).await
    }

    async fn search_with_config(
        &self,
        query: &str,
        filters: Option<HashMap<String, String>>,
        top_k: usize,
    ) -> Result<RetrievalResult> {
        let start = Instant::now();

        // Validate query length
        if query.len() > self.config.max_query_length {
            return Err(RagError::invalid_input(format!(
                "Query too long: {} > {}",
                query.len(),
                self.config.max_query_length
            )));
        }

        // Generate query embedding
        let embedding = self.embedder.embed(query).await?;

        // Build search query
        let mut search_query = SearchQuery::new(query)
            .with_embedding(embedding)
            .with_top_k(top_k)
            .with_min_score(self.config.min_score);

        if let Some(f) = filters {
            search_query.filters = f;
        }

        // Search
        let results = self.store.search(&search_query).await?;

        let elapsed = start.elapsed();

        Ok(RetrievalResult {
            query: query.to_string(),
            results,
            retrieval_time_ms: elapsed.as_millis() as u64,
        })
    }

    /// Hybrid search (keyword + semantic)
    pub async fn hybrid_search(
        &self,
        query: &str,
        keyword_weight: f32,
        top_k: usize,
    ) -> Result<RetrievalResult> {
        let start = Instant::now();

        // Semantic search
        let semantic_results = self.search_top_k(query, top_k * 2).await?;

        // Keyword matching (simple)
        let query_lower = query.to_lowercase();
        let query_words: Vec<&str> = query_lower.split_whitespace().collect();

        // Re-rank by keyword match
        let mut reranked: Vec<SearchResult> = semantic_results
            .results
            .into_iter()
            .map(|mut result| {
                let content_lower = result.chunk.content.to_lowercase();
                let keyword_matches = query_words
                    .iter()
                    .filter(|word| content_lower.contains(*word))
                    .count();

                let keyword_score = keyword_matches as f32 / query_words.len().max(1) as f32;
                let semantic_score = result.score;

                // Combine scores
                result.score = (1.0 - keyword_weight) * semantic_score + keyword_weight * keyword_score;
                result
            })
            .collect();

        // Sort by combined score
        reranked.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        reranked.truncate(top_k);

        let elapsed = start.elapsed();

        Ok(RetrievalResult {
            query: query.to_string(),
            results: reranked,
            retrieval_time_ms: elapsed.as_millis() as u64,
        })
    }

    /// Multi-query retrieval
    pub async fn multi_query(
        &self,
        queries: &[&str],
        top_k_per_query: usize,
    ) -> Result<MultiQueryResult> {
        let start = Instant::now();
        let mut all_results = Vec::new();

        for query in queries {
            let result = self.search_top_k(query, top_k_per_query).await?;
            all_results.push(result);
        }

        // Deduplicate and merge
        let merged = self.merge_results(all_results, top_k_per_query * queries.len());

        let elapsed = start.elapsed();

        Ok(MultiQueryResult {
            queries: queries.iter().map(|s| s.to_string()).collect(),
            results: merged,
            total_time_ms: elapsed.as_millis() as u64,
        })
    }

    fn merge_results(&self, results: Vec<RetrievalResult>, top_k: usize) -> Vec<SearchResult> {
        let mut seen = std::collections::HashSet::new();
        let mut merged = Vec::new();

        for result in results {
            for search_result in result.results {
                if seen.insert(search_result.chunk.id.clone()) {
                    merged.push(search_result);
                }
            }
        }

        // Sort by score
        merged.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        merged.truncate(top_k);

        merged
    }

    /// Get context for query (formatted context string)
    pub async fn get_context(&self, query: &str, max_tokens: usize) -> Result<String> {
        let results = self.search(query).await?;

        let mut context = String::new();
        let mut token_count = 0;

        for result in results.results {
            let chunk_tokens = crate::chunker::Chunker::estimate_tokens(&result.chunk.content);

            if token_count + chunk_tokens > max_tokens {
                break;
            }

            context.push_str(&result.chunk.content);
            context.push_str("\n\n");
            token_count += chunk_tokens;
        }

        Ok(context.trim().to_string())
    }

    /// Get store reference
    pub fn store(&self) -> &Arc<dyn VectorStore> {
        &self.store
    }

    /// Get embedder reference
    pub fn embedder(&self) -> &Arc<dyn Embedder> {
        &self.embedder
    }
}

use std::collections::HashMap;

/// Retrieval result
#[derive(Debug, Clone)]
pub struct RetrievalResult {
    /// Original query
    pub query: String,
    /// Search results
    pub results: Vec<SearchResult>,
    /// Retrieval time in ms
    pub retrieval_time_ms: u64,
}

impl RetrievalResult {
    /// Get result count
    pub fn len(&self) -> usize {
        self.results.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    /// Get best result
    pub fn best(&self) -> Option<&SearchResult> {
        self.results.first()
    }

    /// Get contexts as string
    pub fn to_context(&self) -> String {
        self.results
            .iter()
            .map(|r| r.chunk.content.as_str())
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

/// Multi-query result
#[derive(Debug, Clone)]
pub struct MultiQueryResult {
    /// Original queries
    pub queries: Vec<String>,
    /// Merged results
    pub results: Vec<SearchResult>,
    /// Total time in ms
    pub total_time_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embedder::MockEmbedder;
    use crate::store::MemoryStore;

    async fn create_test_retriever() -> Retriever {
        let store = Arc::new(MemoryStore::new(IndexConfig {
            dimension: 32,
            ..Default::default()
        }));
        let embedder = Arc::new(MockEmbedder::new(32));

        // Add some test chunks
        let mut chunk1 = Chunk::new("doc-1", "The quick brown fox jumps.", 0);
        chunk1.embedding = Some(embedder.embed(&chunk1.content).await.unwrap());

        let mut chunk2 = Chunk::new("doc-1", "A fast animal runs.", 1);
        chunk2.embedding = Some(embedder.embed(&chunk2.content).await.unwrap());

        let mut chunk3 = Chunk::new("doc-2", "Programming in Rust.", 0);
        chunk3.embedding = Some(embedder.embed(&chunk3.content).await.unwrap());

        store.add(chunk1).await.unwrap();
        store.add(chunk2).await.unwrap();
        store.add(chunk3).await.unwrap();

        Retriever::default_config(store, embedder)
    }

    #[tokio::test]
    async fn test_retriever_search() {
        let retriever = create_test_retriever().await;
        let result = retriever.search("fox animal").await.unwrap();

        assert!(!result.is_empty());
        assert!(result.retrieval_time_ms >= 0);
    }

    #[tokio::test]
    async fn test_retriever_search_top_k() {
        let retriever = create_test_retriever().await;
        let result = retriever.search_top_k("animal", 2).await.unwrap();

        assert!(result.len() <= 2);
    }

    #[tokio::test]
    async fn test_retriever_hybrid_search() {
        let retriever = create_test_retriever().await;
        let result = retriever.hybrid_search("fox quick", 0.5, 2).await.unwrap();

        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn test_retriever_get_context() {
        let retriever = create_test_retriever().await;
        let context = retriever.get_context("animal", 100).await.unwrap();

        assert!(!context.is_empty());
    }

    #[tokio::test]
    async fn test_retrieval_result_methods() {
        let result = RetrievalResult {
            query: "test".to_string(),
            results: vec![
                SearchResult::new(Chunk::new("doc", "content 1", 0), 0.9),
                SearchResult::new(Chunk::new("doc", "content 2", 1), 0.8),
            ],
            retrieval_time_ms: 10,
        };

        assert_eq!(result.len(), 2);
        assert!(!result.is_empty());
        assert!(result.best().is_some());
        assert_eq!(result.best().unwrap().score, 0.9);
    }

    #[tokio::test]
    async fn test_retriever_multi_query() {
        let retriever = create_test_retriever().await;
        let result = retriever
            .multi_query(&["fox", "programming", "animal"], 2)
            .await
            .unwrap();

        assert_eq!(result.queries.len(), 3);
        assert!(result.total_time_ms >= 0);
    }

    #[tokio::test]
    async fn test_retriever_filtered_search() {
        let retriever = create_test_retriever().await;

        let mut filters = HashMap::new();
        filters.insert("type".to_string(), "text".to_string());

        let result = retriever
            .search_filtered("animal", filters, 5)
            .await
            .unwrap();

        // Results may be empty if no chunks match filter
        assert!(result.results.len() <= 5);
    }

    #[tokio::test]
    async fn test_empty_query() {
        let store = Arc::new(MemoryStore::new(IndexConfig {
            dimension: 32,
            ..Default::default()
        }));
        let embedder = Arc::new(MockEmbedder::new(32));
        let retriever = Retriever::default_config(store, embedder);

        let result = retriever.search("test").await.unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_to_context() {
        let result = RetrievalResult {
            query: "test".to_string(),
            results: vec![
                SearchResult::new(Chunk::new("doc", "First chunk", 0), 0.9),
                SearchResult::new(Chunk::new("doc", "Second chunk", 1), 0.8),
            ],
            retrieval_time_ms: 10,
        };

        let context = result.to_context();
        assert!(context.contains("First chunk"));
        assert!(context.contains("Second chunk"));
    }
}
