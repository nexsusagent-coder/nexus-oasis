//! ─── SENTIENT OS Embedding Hub ───
//!
//! Multi-provider embedding engine supporting:
//! - OpenAI (text-embedding-3-large, text-embedding-3-small, ada-002)
//! - Cohere (embed-english-v3, embed-multilingual-v3)
//! - HuggingFace (300+ models)
//! - Voyage AI (voyage-large-2)
//! - Jina AI (jina-embeddings-v2)
//! - Local Models (sentence-transformers, llama.cpp)
//!
//! # Features
//! - Batch processing
//! - Semantic caching
//! - Dimension reduction
//! - MMR diversity
//!
//! # Example
//! ```rust,ignore
//! use sentient_embed::{EmbeddingHub, EmbeddingRequest};
//!
//! let hub = EmbeddingHub::from_env().unwrap();
//! let embedding = hub.embed("Hello world").await?;
//! println!("Vector: {:?}", embedding.vector);
//! ```

use std::collections::HashMap;
use std::sync::Arc;
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;

use async_trait::async_trait;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

pub mod error;
pub mod providers;
pub mod cache;
pub mod batch;
pub mod reduce;

pub use error::{EmbedError, EmbedResult};
pub use cache::{EmbeddingCache, CacheConfig};
pub use batch::{BatchProcessor, BatchConfig};
pub use reduce::{DimensionReducer, ReductionMethod};

// ═══════════════════════════════════════════════════════════════════════════════
//  EMBEDDING TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Embedding model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingModel {
    /// Model ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Provider
    pub provider: String,
    /// Vector dimensions
    pub dimensions: usize,
    /// Max input tokens
    pub max_tokens: usize,
    /// Cost per 1K tokens
    pub cost_per_1k: f64,
    /// Supports batch
    pub supports_batch: bool,
    /// Multilingual
    pub multilingual: bool,
    /// Quality rating (1-10)
    pub quality: u8,
    /// Speed rating (1-10)
    pub speed: u8,
}

impl EmbeddingModel {
    // ─── OpenAI Models ─────────────────────────────────────────────────────────
    
    pub fn text_embedding_3_large() -> Self {
        Self {
            id: "text-embedding-3-large".into(),
            name: "OpenAI Text Embedding 3 Large".into(),
            provider: "openai".into(),
            dimensions: 3072,
            max_tokens: 8191,
            cost_per_1k: 0.00013,
            supports_batch: true,
            multilingual: true,
            quality: 10,
            speed: 9,
        }
    }

    pub fn text_embedding_3_small() -> Self {
        Self {
            id: "text-embedding-3-small".into(),
            name: "OpenAI Text Embedding 3 Small".into(),
            provider: "openai".into(),
            dimensions: 1536,
            max_tokens: 8191,
            cost_per_1k: 0.00002,
            supports_batch: true,
            multilingual: true,
            quality: 8,
            speed: 10,
        }
    }

    pub fn ada_002() -> Self {
        Self {
            id: "text-embedding-ada-002".into(),
            name: "OpenAI Ada 002".into(),
            provider: "openai".into(),
            dimensions: 1536,
            max_tokens: 8191,
            cost_per_1k: 0.0001,
            supports_batch: true,
            multilingual: true,
            quality: 7,
            speed: 9,
        }
    }

    // ─── Cohere Models ─────────────────────────────────────────────────────────

    pub fn cohere_english_v3() -> Self {
        Self {
            id: "embed-english-v3.0".into(),
            name: "Cohere Embed English v3".into(),
            provider: "cohere".into(),
            dimensions: 1024,
            max_tokens: 512,
            cost_per_1k: 0.0001,
            supports_batch: true,
            multilingual: false,
            quality: 9,
            speed: 9,
        }
    }

    pub fn cohere_multilingual_v3() -> Self {
        Self {
            id: "embed-multilingual-v3.0".into(),
            name: "Cohere Embed Multilingual v3".into(),
            provider: "cohere".into(),
            dimensions: 1024,
            max_tokens: 512,
            cost_per_1k: 0.0001,
            supports_batch: true,
            multilingual: true,
            quality: 9,
            speed: 8,
        }
    }

    // ─── Voyage AI ─────────────────────────────────────────────────────────────

    pub fn voyage_large_2() -> Self {
        Self {
            id: "voyage-large-2".into(),
            name: "Voyage Large 2".into(),
            provider: "voyage".into(),
            dimensions: 1536,
            max_tokens: 16000,
            cost_per_1k: 0.00012,
            supports_batch: true,
            multilingual: true,
            quality: 9,
            speed: 8,
        }
    }

    pub fn voyage_code_2() -> Self {
        Self {
            id: "voyage-code-2".into(),
            name: "Voyage Code 2".into(),
            provider: "voyage".into(),
            dimensions: 1536,
            max_tokens: 16000,
            cost_per_1k: 0.00012,
            supports_batch: true,
            multilingual: false,
            quality: 10,
            speed: 8,
        }
    }

    // ─── Jina AI ───────────────────────────────────────────────────────────────

    pub fn jina_v2_base() -> Self {
        Self {
            id: "jina-embeddings-v2-base-en".into(),
            name: "Jina Embeddings v2 Base".into(),
            provider: "jina".into(),
            dimensions: 768,
            max_tokens: 8192,
            cost_per_1k: 0.00001,
            supports_batch: true,
            multilingual: false,
            quality: 8,
            speed: 9,
        }
    }

    // ─── HuggingFace ───────────────────────────────────────────────────────────

    pub fn e5_large_v2() -> Self {
        Self {
            id: "intfloat/e5-large-v2".into(),
            name: "E5 Large v2".into(),
            provider: "huggingface".into(),
            dimensions: 1024,
            max_tokens: 512,
            cost_per_1k: 0.0,
            supports_batch: true,
            multilingual: true,
            quality: 9,
            speed: 7,
        }
    }

    pub fn bge_large_en() -> Self {
        Self {
            id: "BAAI/bge-large-en".into(),
            name: "BGE Large English".into(),
            provider: "huggingface".into(),
            dimensions: 1024,
            max_tokens: 512,
            cost_per_1k: 0.0,
            supports_batch: true,
            multilingual: false,
            quality: 9,
            speed: 7,
        }
    }

    pub fn sentence_transformers_mpnet() -> Self {
        Self {
            id: "sentence-transformers/all-mpnet-base-v2".into(),
            name: "Sentence Transformers MPNet".into(),
            provider: "local".into(),
            dimensions: 768,
            max_tokens: 384,
            cost_per_1k: 0.0,
            supports_batch: true,
            multilingual: true,
            quality: 8,
            speed: 8,
        }
    }

    // ─── Nomic ─────────────────────────────────────────────────────────────────

    pub fn nomic_embed_v1() -> Self {
        Self {
            id: "nomic-ai/nomic-embed-text-v1".into(),
            name: "Nomic Embed v1".into(),
            provider: "local".into(),
            dimensions: 768,
            max_tokens: 8192,
            cost_per_1k: 0.0,
            supports_batch: true,
            multilingual: false,
            quality: 8,
            speed: 9,
        }
    }

    /// Get all available models
    pub fn all() -> Vec<Self> {
        vec![
            Self::text_embedding_3_large(),
            Self::text_embedding_3_small(),
            Self::ada_002(),
            Self::cohere_english_v3(),
            Self::cohere_multilingual_v3(),
            Self::voyage_large_2(),
            Self::voyage_code_2(),
            Self::jina_v2_base(),
            Self::e5_large_v2(),
            Self::bge_large_en(),
            Self::sentence_transformers_mpnet(),
            Self::nomic_embed_v1(),
        ]
    }

    /// Get free models
    pub fn free() -> Vec<Self> {
        Self::all().into_iter().filter(|m| m.cost_per_1k == 0.0).collect()
    }

    /// Get best quality
    pub fn best_quality() -> Self {
        Self::text_embedding_3_large()
    }

    /// Get fastest
    pub fn fastest() -> Self {
        Self::text_embedding_3_small()
    }

    /// Get cheapest
    pub fn cheapest() -> Self {
        Self::nomic_embed_v1()
    }
}

/// Single embedding result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedding {
    /// The embedding vector
    pub vector: Vec<f32>,
    /// Model used
    pub model: String,
    /// Number of tokens
    pub tokens: usize,
    /// Index in batch
    pub index: usize,
    /// Original text (optional)
    pub text: Option<String>,
}

impl Embedding {
    /// Get dimensions
    pub fn dimensions(&self) -> usize {
        self.vector.len()
    }

    /// Calculate cosine similarity with another embedding
    pub fn cosine_similarity(&self, other: &Embedding) -> f32 {
        let dot: f32 = self.vector.iter()
            .zip(other.vector.iter())
            .map(|(a, b)| a * b)
            .sum();
        
        let norm_a: f32 = self.vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = other.vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        
        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot / (norm_a * norm_b)
        }
    }

    /// Calculate euclidean distance
    pub fn euclidean_distance(&self, other: &Embedding) -> f32 {
        self.vector.iter()
            .zip(other.vector.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f32>()
            .sqrt()
    }

    /// Calculate dot product
    pub fn dot_product(&self, other: &Embedding) -> f32 {
        self.vector.iter()
            .zip(other.vector.iter())
            .map(|(a, b)| a * b)
            .sum()
    }

    /// Normalize vector to unit length
    pub fn normalize(&self) -> Self {
        let norm: f32 = self.vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        let vector = if norm > 0.0 {
            self.vector.iter().map(|x| x / norm).collect()
        } else {
            self.vector.clone()
        };
        
        Self {
            vector,
            model: self.model.clone(),
            tokens: self.tokens,
            index: self.index,
            text: self.text.clone(),
        }
    }
}

/// Embedding request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingRequest {
    /// Input text(s) to embed
    pub input: EmbeddingInput,
    /// Model to use
    pub model: String,
    /// Target dimensions (for truncation)
    pub dimensions: Option<usize>,
    /// Encoding format
    pub encoding_format: Option<EncodingFormat>,
    /// User identifier
    pub user: Option<String>,
}

/// Input types for embedding
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmbeddingInput {
    /// Single text
    Single(String),
    /// Multiple texts
    Multiple(Vec<String>),
}

impl EmbeddingInput {
    /// Get number of texts
    pub fn len(&self) -> usize {
        match self {
            Self::Single(_) => 1,
            Self::Multiple(v) => v.len(),
        }
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get texts as slice
    pub fn texts(&self) -> Vec<&str> {
        match self {
            Self::Single(s) => vec![s.as_str()],
            Self::Multiple(v) => v.iter().map(|s| s.as_str()).collect(),
        }
    }
}

impl From<String> for EmbeddingInput {
    fn from(s: String) -> Self {
        Self::Single(s)
    }
}

impl From<&str> for EmbeddingInput {
    fn from(s: &str) -> Self {
        Self::Single(s.to_string())
    }
}

impl From<Vec<String>> for EmbeddingInput {
    fn from(v: Vec<String>) -> Self {
        Self::Multiple(v)
    }
}

impl From<Vec<&str>> for EmbeddingInput {
    fn from(v: Vec<&str>) -> Self {
        Self::Multiple(v.into_iter().map(|s| s.to_string()).collect())
    }
}

/// Encoding format
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EncodingFormat {
    /// Float array
    Float,
    /// Base64 encoded
    Base64,
}

/// Embedding response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingResponse {
    /// Embeddings
    pub embeddings: Vec<Embedding>,
    /// Model used
    pub model: String,
    /// Total tokens used
    pub total_tokens: usize,
    /// Cost in USD
    pub cost: f64,
    /// Processing time in ms
    pub processing_time_ms: u64,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  EMBEDDING PROVIDER TRAIT
// ═══════════════════════════════════════════════════════════════════════════════

/// Embedding provider trait
#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    /// Provider name
    fn name(&self) -> &str;

    /// List available models
    fn models(&self) -> Vec<EmbeddingModel>;

    /// Check if configured
    fn is_configured(&self) -> bool;

    /// Get embeddings
    async fn embed(&self, request: EmbeddingRequest) -> EmbedResult<EmbeddingResponse>;

    /// Get single embedding
    async fn embed_one(&self, text: &str, model: &str) -> EmbedResult<Embedding> {
        let request = EmbeddingRequest {
            input: text.into(),
            model: model.to_string(),
            dimensions: None,
            encoding_format: None,
            user: None,
        };
        
        let response = self.embed(request).await?;
        Ok(response.embeddings.into_iter().next()
            .unwrap_or(Embedding {
                vector: vec![],
                model: model.to_string(),
                tokens: 0,
                index: 0,
                text: Some(text.to_string()),
            }))
    }

    /// Get batch embeddings
    async fn embed_batch(&self, texts: &[&str], model: &str) -> EmbedResult<Vec<Embedding>> {
        let request = EmbeddingRequest {
            input: texts.to_vec().into(),
            model: model.to_string(),
            dimensions: None,
            encoding_format: None,
            user: None,
        };
        
        let response = self.embed(request).await?;
        Ok(response.embeddings)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  EMBEDDING HUB
// ═══════════════════════════════════════════════════════════════════════════════

/// Embedding routing strategy
#[derive(Debug, Clone, Copy)]
pub enum EmbeddingStrategy {
    /// Use default model
    Default,
    /// Use best quality
    BestQuality,
    /// Use fastest
    Fastest,
    /// Use cheapest
    Cheapest,
    /// Prefer local/free
    FreeFirst,
}

/// Embedding hub statistics
#[derive(Debug, Clone, Default)]
pub struct EmbeddingStats {
    pub total_requests: u64,
    pub total_embeddings: u64,
    pub total_tokens: u64,
    pub total_cost: f64,
    pub cache_hits: u64,
    pub average_latency_ms: f64,
}

/// Central embedding hub
pub struct EmbeddingHub {
    providers: HashMap<String, Arc<dyn EmbeddingProvider>>,
    default_model: String,
    strategy: EmbeddingStrategy,
    cache: Option<EmbeddingCache>,
    stats: Arc<RwLock<EmbeddingStats>>,
}

impl EmbeddingHub {
    /// Create new embedding hub
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            default_model: "text-embedding-3-small".into(),
            strategy: EmbeddingStrategy::Default,
            cache: None,
            stats: Arc::new(RwLock::new(EmbeddingStats::default())),
        }
    }

    /// Create from environment variables
    pub fn from_env() -> EmbedResult<Self> {
        let mut hub = Self::new();
        
        // Add OpenAI if configured
        if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
            hub.add_provider("openai", providers::OpenAIEmbedding::new(api_key));
        }
        
        // Add Cohere if configured
        if let Ok(api_key) = std::env::var("COHERE_API_KEY") {
            hub.add_provider("cohere", providers::CohereEmbedding::new(api_key));
        }
        
        // Add Voyage if configured
        if let Ok(api_key) = std::env::var("VOYAGE_API_KEY") {
            hub.add_provider("voyage", providers::VoyageEmbedding::new(api_key));
        }
        
        // Add Jina if configured
        if let Ok(api_key) = std::env::var("JINA_API_KEY") {
            hub.add_provider("jina", providers::JinaEmbedding::new(api_key));
        }
        
        // Add HuggingFace if configured
        if let Ok(api_key) = std::env::var("HUGGINGFACE_API_KEY") {
            hub.add_provider("huggingface", providers::HuggingFaceEmbedding::new(api_key));
        }
        
        // Add local provider (always available)
        hub.add_provider("local", providers::LocalEmbedding::new());
        
        // Enable caching
        hub.cache = Some(EmbeddingCache::new(CacheConfig::default()));
        
        Ok(hub)
    }

    /// Add provider
    pub fn add_provider<P: EmbeddingProvider + 'static>(&mut self, name: &str, provider: P) {
        self.providers.insert(name.to_string(), Arc::new(provider));
    }

    /// Set default model
    pub fn set_default_model(&mut self, model: impl Into<String>) {
        self.default_model = model.into();
    }

    /// Set routing strategy
    pub fn set_strategy(&mut self, strategy: EmbeddingStrategy) {
        self.strategy = strategy;
    }

    /// Enable cache
    pub fn enable_cache(&mut self, config: CacheConfig) {
        self.cache = Some(EmbeddingCache::new(config));
    }

    /// Get single embedding
    pub async fn embed(&self, text: &str) -> EmbedResult<Embedding> {
        self.embed_with_model(text, &self.default_model).await
    }

    /// Get single embedding with specific model
    pub async fn embed_with_model(&self, text: &str, model: &str) -> EmbedResult<Embedding> {
        // Check cache first
        if let Some(cache) = &self.cache {
            if let Some(cached) = cache.get(text, model) {
                let mut stats = self.stats.write();
                stats.cache_hits += 1;
                return Ok(cached);
            }
        }
        
        let request = EmbeddingRequest {
            input: text.into(),
            model: model.to_string(),
            dimensions: None,
            encoding_format: None,
            user: None,
        };
        
        let provider = self.resolve_provider(model)?;
        let response = provider.embed(request).await?;
        
        // Update stats
        {
            let mut stats = self.stats.write();
            stats.total_requests += 1;
            stats.total_embeddings += response.embeddings.len() as u64;
            stats.total_tokens += response.total_tokens as u64;
            stats.total_cost += response.cost;
        }
        
        let embedding = response.embeddings.into_iter().next()
            .ok_or(EmbedError::NoEmbeddings)?;
        
        // Cache result
        if let Some(cache) = &self.cache {
            cache.put(text, model, embedding.clone());
        }
        
        Ok(embedding)
    }

    /// Get batch embeddings
    pub async fn embed_batch(&self, texts: &[&str]) -> EmbedResult<Vec<Embedding>> {
        self.embed_batch_with_model(texts, &self.default_model).await
    }

    /// Get batch embeddings with specific model
    pub async fn embed_batch_with_model(&self, texts: &[&str], model: &str) -> EmbedResult<Vec<Embedding>> {
        let request = EmbeddingRequest {
            input: texts.to_vec().into(),
            model: model.to_string(),
            dimensions: None,
            encoding_format: None,
            user: None,
        };
        
        let provider = self.resolve_provider(model)?;
        let response = provider.embed(request).await?;
        
        // Update stats
        {
            let mut stats = self.stats.write();
            stats.total_requests += 1;
            stats.total_embeddings += response.embeddings.len() as u64;
            stats.total_tokens += response.total_tokens as u64;
            stats.total_cost += response.cost;
        }
        
        Ok(response.embeddings)
    }

    /// Calculate similarity between two texts
    pub async fn similarity(&self, text1: &str, text2: &str) -> EmbedResult<f32> {
        let emb1 = self.embed(text1).await?;
        let emb2 = self.embed(text2).await?;
        Ok(emb1.cosine_similarity(&emb2))
    }

    /// Find most similar texts from candidates
    pub async fn find_similar(&self, query: &str, candidates: &[&str], top_k: usize) -> EmbedResult<Vec<(usize, f32)>> {
        let query_emb = self.embed(query).await?;
        let candidate_embs = self.embed_batch(candidates).await?;
        
        let mut scores: Vec<_> = candidate_embs.iter()
            .enumerate()
            .map(|(i, emb)| (i, query_emb.cosine_similarity(emb)))
            .collect();
        
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        scores.truncate(top_k);
        
        Ok(scores)
    }

    /// MMR diversity search
    pub async fn mmr_search(
        &self,
        query: &str,
        candidates: &[&str],
        top_k: usize,
        diversity: f32,
    ) -> EmbedResult<Vec<(usize, f32)>> {
        let query_emb = self.embed(query).await?;
        let candidate_embs = self.embed_batch(candidates).await?;
        
        let mut selected: Vec<(usize, f32)> = Vec::new();
        let mut remaining: Vec<usize> = (0..candidates.len()).collect();
        
        while selected.len() < top_k && !remaining.is_empty() {
            let mut best_score = f32::NEG_INFINITY;
            let mut best_idx = 0;
            
            for &idx in &remaining {
                let relevance = query_emb.cosine_similarity(&candidate_embs[idx]);
                
                let mut max_similarity: f32 = 0.0;
                for (sel_idx, _) in &selected {
                    let sim = candidate_embs[idx].cosine_similarity(&candidate_embs[*sel_idx]);
                    max_similarity = max_similarity.max(sim);
                }
                
                let mmr_score = relevance * (1.0 - diversity) - max_similarity * diversity;
                
                if mmr_score > best_score {
                    best_score = mmr_score;
                    best_idx = idx;
                }
            }
            
            selected.push((best_idx, candidate_embs[best_idx].cosine_similarity(&query_emb)));
            remaining.retain(|&x| x != best_idx);
        }
        
        Ok(selected)
    }

    /// Get statistics
    pub fn stats(&self) -> EmbeddingStats {
        self.stats.read().clone()
    }

    /// Clear cache
    pub fn clear_cache(&self) {
        if let Some(cache) = &self.cache {
            cache.clear();
        }
    }

    /// List available models
    pub fn available_models(&self) -> Vec<EmbeddingModel> {
        EmbeddingModel::all()
    }

    /// Resolve provider for model
    fn resolve_provider(&self, model: &str) -> EmbedResult<Arc<dyn EmbeddingProvider>> {
        // Find provider by model prefix
        let provider_name = if model.starts_with("text-embedding") || model.starts_with("ada") {
            "openai"
        } else if model.starts_with("embed-") {
            "cohere"
        } else if model.starts_with("voyage-") {
            "voyage"
        } else if model.starts_with("jina-") {
            "jina"
        } else if model.contains('/') {
            "huggingface"
        } else {
            "local"
        };
        
        self.providers.get(provider_name)
            .cloned()
            .ok_or_else(|| EmbedError::ProviderNotFound(provider_name.to_string()))
    }
}

impl Default for EmbeddingHub {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  EMBEDDING UTILITIES
// ═══════════════════════════════════════════════════════════════════════════════

/// Calculate cosine similarity between two vectors
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }
    
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a * norm_b)
    }
}

/// Normalize a vector to unit length
pub fn normalize_vector(v: &mut [f32]) {
    let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for x in v.iter_mut() {
            *x /= norm;
        }
    }
}

/// Compute cosine similarity matrix
pub fn similarity_matrix(embeddings: &[Embedding]) -> Vec<Vec<f32>> {
    let n = embeddings.len();
    let mut matrix = vec![vec![0.0f32; n]; n];
    
    for i in 0..n {
        for j in i..n {
            let sim = embeddings[i].cosine_similarity(&embeddings[j]);
            matrix[i][j] = sim;
            matrix[j][i] = sim;
        }
    }
    
    matrix
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_model_list() {
        let models = EmbeddingModel::all();
        assert!(models.len() >= 10);
    }

    #[test]
    fn test_free_models() {
        let free = EmbeddingModel::free();
        assert!(!free.is_empty());
        assert!(free.iter().all(|m| m.cost_per_1k == 0.0));
    }

    #[test]
    fn test_embedding_similarity() {
        let emb1 = Embedding {
            vector: vec![1.0, 0.0, 0.0],
            model: "test".into(),
            tokens: 1,
            index: 0,
            text: None,
        };
        
        let emb2 = Embedding {
            vector: vec![0.0, 1.0, 0.0],
            model: "test".into(),
            tokens: 1,
            index: 1,
            text: None,
        };
        
        let sim = emb1.cosine_similarity(&emb2);
        assert!((sim - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_embedding_normalize() {
        let emb = Embedding {
            vector: vec![3.0, 4.0],
            model: "test".into(),
            tokens: 1,
            index: 0,
            text: None,
        };
        
        let normalized = emb.normalize();
        let norm: f32 = normalized.vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        
        let sim = cosine_similarity(&a, &b);
        assert!((sim - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_embedding_hub_creation() {
        let hub = EmbeddingHub::new();
        let stats = hub.stats();
        assert_eq!(stats.total_requests, 0);
    }

    #[test]
    fn test_embedding_input() {
        let single: EmbeddingInput = "hello".into();
        assert_eq!(single.len(), 1);
        
        let multi: EmbeddingInput = vec!["hello", "world"].into();
        assert_eq!(multi.len(), 2);
    }
}
