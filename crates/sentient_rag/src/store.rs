//! Vector store abstractions and implementations

use crate::types::*;
use crate::{RagError, Result};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Vector store trait
#[async_trait::async_trait]
pub trait VectorStore: Send + Sync {
    /// Add chunk with embedding
    async fn add(&self, chunk: Chunk) -> Result<()>;

    /// Add multiple chunks
    async fn add_batch(&self, chunks: Vec<Chunk>) -> Result<()>;

    /// Search by embedding
    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>>;

    /// Get chunk by ID
    async fn get(&self, id: &ChunkId) -> Result<Option<Chunk>>;

    /// Remove chunk by ID
    async fn remove(&self, id: &ChunkId) -> Result<bool>;

    /// Remove all chunks for a document
    async fn remove_document(&self, document_id: &DocumentId) -> Result<usize>;

    /// Get total chunk count
    async fn count(&self) -> usize;

    /// Clear all chunks
    async fn clear(&self) -> Result<()>;

    /// Get store statistics
    async fn stats(&self) -> StoreStats;
}

/// Store statistics
#[derive(Debug, Clone, Default)]
pub struct StoreStats {
    pub total_chunks: usize,
    pub total_documents: usize,
    pub total_embeddings: usize,
    pub size_bytes: usize,
}

/// In-memory vector store
pub struct MemoryStore {
    chunks: Arc<RwLock<HashMap<ChunkId, Chunk>>>,
    document_chunks: Arc<RwLock<HashMap<DocumentId, Vec<ChunkId>>>>,
    config: IndexConfig,
}

impl MemoryStore {
    /// Create new memory store
    pub fn new(config: IndexConfig) -> Self {
        Self {
            chunks: Arc::new(RwLock::new(HashMap::new())),
            document_chunks: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Create with default config
    pub fn default_config() -> Self {
        Self::new(IndexConfig::default())
    }

    /// Calculate distance based on metric
    fn calculate_distance(&self, a: &[f32], b: &[f32]) -> f32 {
        match self.config.metric {
            DistanceMetric::Cosine => {
                // Cosine similarity (1 - similarity for distance)
                1.0 - crate::embedder::cosine_similarity(a, b)
            }
            DistanceMetric::Euclidean => {
                crate::embedder::euclidean_distance(a, b)
            }
            DistanceMetric::DotProduct => {
                // Negative dot product (larger = more similar)
                -crate::embedder::dot_product(a, b)
            }
            DistanceMetric::Manhattan => {
                a.iter()
                    .zip(b.iter())
                    .map(|(x, y)| (x - y).abs())
                    .sum()
            }
        }
    }

    /// Convert distance to score (0-1)
    fn distance_to_score(&self, distance: f32) -> f32 {
        match self.config.metric {
            DistanceMetric::Cosine => 1.0 - distance, // similarity
            DistanceMetric::Euclidean => 1.0 / (1.0 + distance),
            DistanceMetric::DotProduct => 1.0 / (1.0 + (-distance).exp()),
            DistanceMetric::Manhattan => 1.0 / (1.0 + distance),
        }
    }
}

#[async_trait::async_trait]
impl VectorStore for MemoryStore {
    async fn add(&self, chunk: Chunk) -> Result<()> {
        // Validate embedding
        if let Some(ref embedding) = chunk.embedding {
            if embedding.len() != self.config.dimension {
                return Err(RagError::DimensionMismatch {
                    expected: self.config.dimension,
                    actual: embedding.len(),
                });
            }
        }

        let chunk_id = chunk.id.clone();
        let document_id = chunk.document_id.clone();

        // Add to chunks
        let mut chunks = self.chunks.write().await;
        chunks.insert(chunk_id.clone(), chunk);

        // Add to document index
        let mut doc_chunks = self.document_chunks.write().await;
        doc_chunks
            .entry(document_id)
            .or_default()
            .push(chunk_id);

        Ok(())
    }

    async fn add_batch(&self, chunks: Vec<Chunk>) -> Result<()> {
        for chunk in chunks {
            self.add(chunk).await?;
        }
        Ok(())
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let query_embedding = query.embedding.as_ref().ok_or_else(|| {
            RagError::invalid_input("Query embedding required for search")
        })?;

        let chunks = self.chunks.read().await;

        // Calculate scores for all chunks with embeddings
        let mut scored: Vec<(f32, &Chunk)> = chunks
            .values()
            .filter_map(|chunk| {
                chunk.embedding.as_ref().map(|emb| {
                    let distance = self.calculate_distance(query_embedding, emb);
                    let score = self.distance_to_score(distance);
                    (score, chunk)
                })
            })
            .collect();

        // Apply filters
        if !query.filters.is_empty() {
            scored.retain(|(_, chunk)| {
                query.filters.iter().all(|(k, v)| {
                    chunk.metadata.custom.get(k).map(|cv| cv == v).unwrap_or(false)
                })
            });
        }

        if !query.document_ids.is_empty() {
            scored.retain(|(_, chunk)| query.document_ids.contains(&chunk.document_id));
        }

        // Filter by minimum score
        scored.retain(|(score, _)| *score >= query.min_score);

        // Sort by score descending
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        // Take top_k
        scored.truncate(query.top_k);

        // Convert to results
        Ok(scored
            .into_iter()
            .map(|(score, chunk)| SearchResult::new(chunk.clone(), score))
            .collect())
    }

    async fn get(&self, id: &ChunkId) -> Result<Option<Chunk>> {
        let chunks = self.chunks.read().await;
        Ok(chunks.get(id).cloned())
    }

    async fn remove(&self, id: &ChunkId) -> Result<bool> {
        let mut chunks = self.chunks.write().await;
        if let Some(chunk) = chunks.remove(id) {
            // Remove from document index
            let mut doc_chunks = self.document_chunks.write().await;
            if let Some(doc_chunk_list) = doc_chunks.get_mut(&chunk.document_id) {
                doc_chunk_list.retain(|cid| cid != id);
                if doc_chunk_list.is_empty() {
                    doc_chunks.remove(&chunk.document_id);
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn remove_document(&self, document_id: &DocumentId) -> Result<usize> {
        let mut doc_chunks = self.document_chunks.write().await;
        let chunk_ids = doc_chunks.remove(document_id).unwrap_or_default();
        let count = chunk_ids.len();

        // Remove all chunks
        let mut chunks = self.chunks.write().await;
        for id in &chunk_ids {
            chunks.remove(id);
        }

        Ok(count)
    }

    async fn count(&self) -> usize {
        self.chunks.read().await.len()
    }

    async fn clear(&self) -> Result<()> {
        let mut chunks = self.chunks.write().await;
        chunks.clear();

        let mut doc_chunks = self.document_chunks.write().await;
        doc_chunks.clear();

        Ok(())
    }

    async fn stats(&self) -> StoreStats {
        let chunks = self.chunks.read().await;
        let doc_chunks = self.document_chunks.read().await;

        StoreStats {
            total_chunks: chunks.len(),
            total_documents: doc_chunks.len(),
            total_embeddings: chunks.values().filter(|c| c.embedding.is_some()).count(),
            size_bytes: chunks.values().map(|c| c.content.len()).sum(),
        }
    }
}

/// LanceDB vector store (persistent)
#[cfg(feature = "vector-store")]
pub struct LanceStore {
    db: std::sync::Mutex<Option<lancedb::Connection>>,
    table_name: String,
    config: IndexConfig,
}

#[cfg(feature = "vector-store")]
impl LanceStore {
    /// Create new LanceDB store
    pub async fn new(path: &str, config: IndexConfig) -> Result<Self> {
        let db = lancedb::connect(path)
            .execute()
            .await
            .map_err(|e| RagError::store(e.to_string()))?;

        Ok(Self {
            db: std::sync::Mutex::new(Some(db)),
            table_name: config.name.clone(),
            config,
        })
    }
}

#[cfg(feature = "vector-store")]
#[async_trait::async_trait]
impl VectorStore for LanceStore {
    async fn add(&self, _chunk: Chunk) -> Result<()> {
        // TODO: Implement LanceDB storage
        Err(RagError::store("LanceDB implementation pending"))
    }

    async fn add_batch(&self, _chunks: Vec<Chunk>) -> Result<()> {
        Err(RagError::store("LanceDB implementation pending"))
    }

    async fn search(&self, _query: &SearchQuery) -> Result<Vec<SearchResult>> {
        Err(RagError::store("LanceDB implementation pending"))
    }

    async fn get(&self, _id: &ChunkId) -> Result<Option<Chunk>> {
        Ok(None)
    }

    async fn remove(&self, _id: &ChunkId) -> Result<bool> {
        Ok(false)
    }

    async fn remove_document(&self, _document_id: &DocumentId) -> Result<usize> {
        Ok(0)
    }

    async fn count(&self) -> usize {
        0
    }

    async fn clear(&self) -> Result<()> {
        Ok(())
    }

    async fn stats(&self) -> StoreStats {
        StoreStats::default()
    }
}

/// Store builder
pub struct StoreBuilder {
    store_type: StoreType,
    config: IndexConfig,
    path: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreType {
    Memory,
    Lance,
}

impl StoreBuilder {
    pub fn new() -> Self {
        Self {
            store_type: StoreType::Memory,
            config: IndexConfig::default(),
            path: None,
        }
    }

    pub fn memory(mut self) -> Self {
        self.store_type = StoreType::Memory;
        self
    }

    pub fn lance(mut self, path: impl Into<String>) -> Self {
        self.store_type = StoreType::Lance;
        self.path = Some(path.into());
        self
    }

    pub fn config(mut self, config: IndexConfig) -> Self {
        self.config = config;
        self
    }

    pub fn dimension(mut self, dim: usize) -> Self {
        self.config.dimension = dim;
        self
    }

    pub fn metric(mut self, metric: DistanceMetric) -> Self {
        self.config.metric = metric;
        self
    }

    pub async fn build(self) -> Result<Arc<dyn VectorStore>> {
        match self.store_type {
            StoreType::Memory => Ok(Arc::new(MemoryStore::new(self.config))),
            StoreType::Lance => {
                #[cfg(feature = "vector-store")]
                {
                    let path = self.path.unwrap_or_else(|| "./data/lancedb".to_string());
                    Ok(Arc::new(LanceStore::new(&path, self.config).await?))
                }
                #[cfg(not(feature = "vector-store"))]
                {
                    Err(RagError::config("LanceDB requires 'vector-store' feature"))
                }
            }
        }
    }
}

impl Default for StoreBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_store_add() {
        let store = MemoryStore::default_config();
        let chunk = Chunk::new("doc-1", "test content", 0);

        store.add(chunk).await.unwrap();
        assert_eq!(store.count().await, 1);
    }

    #[tokio::test]
    async fn test_memory_store_get() {
        let store = MemoryStore::default_config();
        let chunk = Chunk::new("doc-1", "test content", 0);
        let id = chunk.id.clone();

        store.add(chunk).await.unwrap();

        let retrieved = store.get(&id).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().content, "test content");
    }

    #[tokio::test]
    async fn test_memory_store_remove() {
        let store = MemoryStore::default_config();
        let chunk = Chunk::new("doc-1", "test content", 0);
        let id = chunk.id.clone();

        store.add(chunk).await.unwrap();
        assert!(store.remove(&id).await.unwrap());
        assert_eq!(store.count().await, 0);
    }

    #[tokio::test]
    async fn test_memory_store_search() {
        let config = IndexConfig {
            dimension: 3,
            ..Default::default()
        };
        let store = MemoryStore::new(config);

        // Add chunks with embeddings
        let mut chunk1 = Chunk::new("doc-1", "first", 0);
        chunk1.embedding = Some(vec![1.0, 0.0, 0.0]);

        let mut chunk2 = Chunk::new("doc-1", "second", 1);
        chunk2.embedding = Some(vec![0.0, 1.0, 0.0]);

        store.add(chunk1).await.unwrap();
        store.add(chunk2).await.unwrap();

        // Search
        let query = SearchQuery::new("test")
            .with_embedding(vec![1.0, 0.0, 0.0])
            .with_top_k(1);

        let results = store.search(&query).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].chunk.content, "first");
    }

    #[tokio::test]
    async fn test_memory_store_dimension_mismatch() {
        let config = IndexConfig {
            dimension: 128,
            ..Default::default()
        };
        let store = MemoryStore::new(config);

        let mut chunk = Chunk::new("doc-1", "test", 0);
        chunk.embedding = Some(vec![0.0; 64]); // Wrong dimension

        let result = store.add(chunk).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_memory_store_remove_document() {
        let store = MemoryStore::default_config();

        let chunk1 = Chunk::new("doc-1", "content 1", 0);
        let chunk2 = Chunk::new("doc-1", "content 2", 1);
        let chunk3 = Chunk::new("doc-2", "content 3", 0);

        store.add(chunk1).await.unwrap();
        store.add(chunk2).await.unwrap();
        store.add(chunk3).await.unwrap();

        let doc_id: DocumentId = "doc-1".to_string();
        let removed = store.remove_document(&doc_id).await.unwrap();
        assert_eq!(removed, 2);
        assert_eq!(store.count().await, 1);
    }

    #[tokio::test]
    async fn test_store_builder() {
        let store = StoreBuilder::new()
            .memory()
            .dimension(256)
            .metric(DistanceMetric::Euclidean)
            .build()
            .await
            .unwrap();

        assert_eq!(store.count().await, 0);
    }

    #[tokio::test]
    async fn test_memory_store_stats() {
        let store = MemoryStore::default_config();

        let mut chunk = Chunk::new("doc-1", "test content", 0);
        chunk.embedding = Some(vec![0.0; 384]);

        store.add(chunk).await.unwrap();

        let stats = store.stats().await;
        assert_eq!(stats.total_chunks, 1);
        assert_eq!(stats.total_documents, 1);
        assert_eq!(stats.total_embeddings, 1);
    }

    #[tokio::test]
    async fn test_memory_store_clear() {
        let store = MemoryStore::default_config();

        store.add(Chunk::new("doc-1", "test", 0)).await.unwrap();
        store.add(Chunk::new("doc-2", "test", 0)).await.unwrap();

        store.clear().await.unwrap();
        assert_eq!(store.count().await, 0);
    }
}
