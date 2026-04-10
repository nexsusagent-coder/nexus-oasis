//! Embedding generation

use crate::types::*;
use crate::{RagError, Result};
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Embedder trait for embedding generation
#[async_trait::async_trait]
pub trait Embedder: Send + Sync {
    /// Embed a single text
    async fn embed(&self, text: &str) -> Result<EmbeddingVector>;

    /// Embed multiple texts
    async fn embed_batch(&self, texts: &[&str]) -> Result<Vec<EmbeddingVector>>;

    /// Get embedding dimension
    fn dimension(&self) -> usize;

    /// Get model name
    fn model_name(&self) -> &str;
}

/// Local embedder using fastembed
#[cfg(feature = "embeddings")]
pub struct LocalEmbedder {
    model: fastembed::TextEmbedding,
    config: EmbeddingConfig,
    cache: Arc<RwLock<LruCache<String, EmbeddingVector>>>,
}

#[cfg(feature = "embeddings")]
impl LocalEmbedder {
    /// Create new local embedder
    pub fn new(config: EmbeddingConfig) -> Result<Self> {
        let model = fastembed::TextEmbedding::try_new(
            fastembed::InitOptions::new(fastembed::EmbeddingModel::AllMiniLML6V2)
        ).map_err(|e| RagError::embedding(e.to_string()))?;

        let cache = Arc::new(RwLock::new(
            LruCache::new(NonZeroUsize::new(config.cache_size).unwrap())
        ));

        Ok(Self { model, config, cache })
    }

    /// Create with default config
    pub fn default_config() -> Result<Self> {
        Self::new(EmbeddingConfig::default())
    }

    /// Hash text for cache
    fn hash_text(&self, text: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        format!("{}:{:x}", self.config.model, hasher.finish())
    }
}

#[cfg(feature = "embeddings")]
#[async_trait::async_trait]
impl Embedder for LocalEmbedder {
    async fn embed(&self, text: &str) -> Result<EmbeddingVector> {
        // Check cache
        if self.config.use_cache {
            let hash = self.hash_text(text);
            let cache = self.cache.read().await;
            if let Some(embedding) = cache.get(&hash) {
                return Ok(embedding.clone());
            }
        }

        // Generate embedding
        let embeddings = self.model
            .embed(vec![text], None)
            .map_err(|e| RagError::embedding(e.to_string()))?;

        let mut embedding = embeddings
            .into_iter()
            .next()
            .ok_or_else(|| RagError::embedding("No embedding generated"))?
            .to_vec();

        // Normalize if configured
        if self.config.normalize {
            normalize_embedding(&mut embedding);
        }

        // Cache
        if self.config.use_cache {
            let hash = self.hash_text(text);
            let mut cache = self.cache.write().await;
            cache.put(hash, embedding.clone());
        }

        Ok(embedding)
    }

    async fn embed_batch(&self, texts: &[&str]) -> Result<Vec<EmbeddingVector>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        // Check cache for all
        let mut results = vec![None; texts.len()];
        let mut uncached_indices = Vec::new();
        let mut uncached_texts = Vec::new();

        if self.config.use_cache {
            let cache = self.cache.read().await;
            for (i, text) in texts.iter().enumerate() {
                let hash = self.hash_text(text);
                if let Some(embedding) = cache.get(&hash) {
                    results[i] = Some(embedding.clone());
                } else {
                    uncached_indices.push(i);
                    uncached_texts.push(*text);
                }
            }
        } else {
            uncached_indices = (0..texts.len()).collect();
            uncached_texts = texts.to_vec();
        }

        // Generate embeddings for uncached
        if !uncached_texts.is_empty() {
            let embeddings = self.model
                .embed(uncached_texts.clone(), None)
                .map_err(|e| RagError::embedding(e.to_string()))?;

            for (i, embedding_array) in embeddings.into_iter().enumerate() {
                let mut embedding = embedding_array.to_vec();

                if self.config.normalize {
                    normalize_embedding(&mut embedding);
                }

                // Cache
                if self.config.use_cache {
                    let hash = self.hash_text(uncached_texts[i]);
                    let mut cache = self.cache.write().await;
                    cache.put(hash, embedding.clone());
                }

                results[uncached_indices[i]] = Some(embedding);
            }
        }

        // Collect results
        Ok(results.into_iter().map(|o| o.unwrap()).collect())
    }

    fn dimension(&self) -> usize {
        self.config.dimension
    }

    fn model_name(&self) -> &str {
        &self.config.model
    }
}

/// Remote embedder using HTTP API
#[cfg(feature = "remote-embeddings")]
pub struct RemoteEmbedder {
    client: reqwest::Client,
    endpoint: String,
    api_key: Option<String>,
    config: EmbeddingConfig,
    cache: Arc<RwLock<LruCache<String, EmbeddingVector>>>,
}

#[cfg(feature = "remote-embeddings")]
impl RemoteEmbedder {
    /// Create new remote embedder
    pub fn new(endpoint: impl Into<String>, config: EmbeddingConfig) -> Self {
        let client = reqwest::Client::new();
        let cache = Arc::new(RwLock::new(
            LruCache::new(NonZeroUsize::new(config.cache_size).unwrap())
        ));

        Self {
            client,
            endpoint: endpoint.into(),
            api_key: None,
            config,
            cache,
        }
    }

    /// Set API key
    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Create OpenAI embedder
    pub fn openai(api_key: impl Into<String>) -> Self {
        Self::new(
            "https://api.openai.com/v1/embeddings",
            EmbeddingConfig {
                model: "text-embedding-3-small".to_string(),
                dimension: 1536,
                ..Default::default()
            },
        ).with_api_key(api_key)
    }

    /// Hash text for cache
    fn hash_text(&self, text: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        format!("{}:{:x}", self.config.model, hasher.finish())
    }
}

#[cfg(feature = "remote-embeddings")]
#[async_trait::async_trait]
impl Embedder for RemoteEmbedder {
    async fn embed(&self, text: &str) -> Result<EmbeddingVector> {
        // Check cache
        if self.config.use_cache {
            let hash = self.hash_text(text);
            let cache = self.cache.read().await;
            if let Some(embedding) = cache.get(&hash) {
                return Ok(embedding.clone());
            }
        }

        // Call API
        let mut request = self.client
            .post(&self.endpoint)
            .json(&serde_json::json!({
                "model": self.config.model,
                "input": text
            }));

        if let Some(key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request
            .send()
            .await
            .map_err(|e| RagError::Http(e.to_string()))?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(RagError::Http(format!("API error: {}", error)));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| RagError::Http(e.to_string()))?;

        let embedding = json["data"][0]["embedding"]
            .as_array()
            .ok_or_else(|| RagError::embedding("Invalid embedding response"))?
            .iter()
            .filter_map(|v| v.as_f64().map(|f| f as f32))
            .collect();

        // Cache
        if self.config.use_cache {
            let hash = self.hash_text(text);
            let mut cache = self.cache.write().await;
            cache.put(hash, embedding.clone());
        }

        Ok(embedding)
    }

    async fn embed_batch(&self, texts: &[&str]) -> Result<Vec<EmbeddingVector>> {
        // TODO: Implement batch API call
        let mut embeddings = Vec::new();
        for text in texts {
            embeddings.push(self.embed(text).await?);
        }
        Ok(embeddings)
    }

    fn dimension(&self) -> usize {
        self.config.dimension
    }

    fn model_name(&self) -> &str {
        &self.config.model
    }
}

/// Mock embedder for testing
pub struct MockEmbedder {
    dimension: usize,
}

impl MockEmbedder {
    pub fn new(dimension: usize) -> Self {
        Self { dimension }
    }
}

#[async_trait::async_trait]
impl Embedder for MockEmbedder {
    async fn embed(&self, text: &str) -> Result<EmbeddingVector> {
        // Generate deterministic but pseudo-random embedding
        let mut embedding = vec![0.0; self.dimension];
        let bytes = text.as_bytes();
        for (i, value) in embedding.iter_mut().enumerate() {
            let byte = bytes[i % bytes.len()];
            *value = (byte as f32 / 255.0) * 2.0 - 1.0;
        }
        normalize_embedding(&mut embedding);
        Ok(embedding)
    }

    async fn embed_batch(&self, texts: &[&str]) -> Result<Vec<EmbeddingVector>> {
        let mut embeddings = Vec::new();
        for text in texts {
            embeddings.push(self.embed(text).await?);
        }
        Ok(embeddings)
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn model_name(&self) -> &str {
        "mock"
    }
}

/// Normalize embedding vector
pub fn normalize_embedding(embedding: &mut [f32]) {
    let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for value in embedding.iter_mut() {
            *value /= norm;
        }
    }
}

/// Calculate cosine similarity between two embeddings
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }

    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a > 0.0 && norm_b > 0.0 {
        dot / (norm_a * norm_b)
    } else {
        0.0
    }
}

/// Calculate Euclidean distance between two embeddings
pub fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return f32::MAX;
    }

    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f32>()
        .sqrt()
}

/// Calculate dot product between two embeddings
pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }

    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_embedding() {
        let mut embedding = vec![3.0, 4.0];
        normalize_embedding(&mut embedding);

        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 0.0001);

        let c = vec![0.0, 1.0, 0.0];
        assert!(cosine_similarity(&a, &c).abs() < 0.0001);

        let d = vec![-1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &d) - (-1.0)).abs() < 0.0001);
    }

    #[test]
    fn test_euclidean_distance() {
        let a = vec![0.0, 0.0];
        let b = vec![3.0, 4.0];
        assert!((euclidean_distance(&a, &b) - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_dot_product() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        assert!((dot_product(&a, &b) - 32.0).abs() < 0.0001);
    }

    #[tokio::test]
    async fn test_mock_embedder() {
        let embedder = MockEmbedder::new(384);
        let embedding = embedder.embed("test").await.unwrap();

        assert_eq!(embedding.len(), 384);
        assert_eq!(embedder.dimension(), 384);
        assert_eq!(embedder.model_name(), "mock");
    }

    #[tokio::test]
    async fn test_mock_embedder_batch() {
        let embedder = MockEmbedder::new(128);
        let embeddings = embedder.embed_batch(&["test1", "test2", "test3"]).await.unwrap();

        assert_eq!(embeddings.len(), 3);
        assert_eq!(embeddings[0].len(), 128);
    }

    #[test]
    fn test_different_lengths() {
        let a = vec![1.0, 2.0];
        let b = vec![1.0, 2.0, 3.0];

        assert_eq!(cosine_similarity(&a, &b), 0.0);
        assert_eq!(euclidean_distance(&a, &b), f32::MAX);
        assert_eq!(dot_product(&a, &b), 0.0);
    }
}
