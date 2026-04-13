//! ─── SENTIENT OS Reranking Engine ───
//!
//! Search result reranking with multiple providers:
//! - Cohere Rerank (best quality)
//! - Jina Reranker (fast + cheap)
//! - LLM-based Reranking (GPT-4, Claude)
//! - Cross-Encoder (local models)
//! - ColBERT (late interaction)
//!
//! # Features
//! - Multiple reranking strategies
//! - Hybrid fusion (semantic + keyword)
//! - Diversity-aware reranking
//! - Query expansion
//!
//! # Example
//! ```rust,ignore
//! use sentient_rerank::{Reranker, RerankRequest};
//!
//! let reranker = Reranker::cohere("api-key");
//! let results = reranker.rerank("quantum computing", &documents).await?;
//! println!("Top result: {:?}", results[0]);
//! ```

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

pub mod error;
pub mod providers;
pub mod fusion;
pub mod diversity;

pub use error::{RerankError, RerankResult};
pub use fusion::{HybridFusion, FusionStrategy};
pub use diversity::{DiversityReranker, DiversityConfig};

// ═══════════════════════════════════════════════════════════════════════════════
//  RERANK DOCUMENT
// ═══════════════════════════════════════════════════════════════════════════════

/// Document for reranking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RerankDocument {
    /// Document ID (optional)
    pub id: Option<String>,
    /// Document text
    pub text: String,
    /// Original relevance score (optional)
    pub original_score: Option<f32>,
    /// Metadata (optional)
    pub metadata: HashMap<String, String>,
}

impl RerankDocument {
    /// Create new document
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: None,
            text: text.into(),
            original_score: None,
            metadata: HashMap::new(),
        }
    }

    /// With ID
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// With score
    pub fn with_score(mut self, score: f32) -> Self {
        self.original_score = Some(score);
        self
    }

    /// With metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

impl From<&str> for RerankDocument {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for RerankDocument {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  RERANK RESULT
// ═══════════════════════════════════════════════════════════════════════════════

/// Reranked result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RerankedDocument {
    /// Document index in original list
    pub index: usize,
    /// Document ID (if provided)
    pub id: Option<String>,
    /// Document text
    pub text: String,
    /// Relevance score (normalized 0-1)
    pub relevance_score: f32,
    /// Original score (if provided)
    pub original_score: Option<f32>,
    /// Score improvement
    pub score_delta: Option<f32>,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl RerankedDocument {
    /// Get relevance score
    pub fn score(&self) -> f32 {
        self.relevance_score
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  RERANK REQUEST
// ═══════════════════════════════════════════════════════════════════════════════

/// Reranking request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RerankRequest {
    /// Query text
    pub query: String,
    /// Documents to rerank
    pub documents: Vec<RerankDocument>,
    /// Model to use
    pub model: Option<String>,
    /// Number of top results to return
    pub top_n: usize,
    /// Maximum chunks per document
    pub max_chunks_per_doc: Option<usize>,
    /// Return documents in response
    pub return_documents: bool,
    /// Threshold for filtering
    pub threshold: Option<f32>,
}

impl RerankRequest {
    /// Create new request
    pub fn new(query: impl Into<String>, documents: Vec<RerankDocument>) -> Self {
        Self {
            query: query.into(),
            documents,
            model: None,
            top_n: 10,
            max_chunks_per_doc: None,
            return_documents: true,
            threshold: None,
        }
    }

    /// Set model
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    /// Set top_n
    pub fn top_n(mut self, n: usize) -> Self {
        self.top_n = n;
        self
    }

    /// Set threshold
    pub fn threshold(mut self, t: f32) -> Self {
        self.threshold = Some(t);
        self
    }
}

/// Reranking response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RerankResponse {
    /// Reranked results
    pub results: Vec<RerankedDocument>,
    /// Model used
    pub model: String,
    /// Total tokens used
    pub total_tokens: usize,
    /// Processing time in ms
    pub processing_time_ms: u64,
    /// Cost in USD
    pub cost: f64,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  RERANKING MODEL
// ═══════════════════════════════════════════════════════════════════════════════

/// Reranking model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RerankModel {
    /// Model ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Provider
    pub provider: String,
    /// Context length
    pub max_tokens: usize,
    /// Cost per 1K queries
    pub cost_per_1k: f64,
    /// Quality rating (1-10)
    pub quality: u8,
    /// Speed rating (1-10)
    pub speed: u8,
}

impl RerankModel {
    // ─── Cohere Models ───────────────────────────────────────────────────────

    pub fn cohere_rerank_english_v3() -> Self {
        Self {
            id: "rerank-english-v3.0".into(),
            name: "Cohere Rerank English v3".into(),
            provider: "cohere".into(),
            max_tokens: 4096,
            cost_per_1k: 2.0,
            quality: 10,
            speed: 9,
        }
    }

    pub fn cohere_rerank_multilingual_v3() -> Self {
        Self {
            id: "rerank-multilingual-v3.0".into(),
            name: "Cohere Rerank Multilingual v3".into(),
            provider: "cohere".into(),
            max_tokens: 4096,
            cost_per_1k: 2.0,
            quality: 9,
            speed: 9,
        }
    }

    // ─── Jina Models ──────────────────────────────────────────────────────────

    pub fn jina_reranker_v1() -> Self {
        Self {
            id: "jina-reranker-v1-base-en".into(),
            name: "Jina Reranker v1".into(),
            provider: "jina".into(),
            max_tokens: 8192,
            cost_per_1k: 0.02,
            quality: 8,
            speed: 10,
        }
    }

    pub fn jina_colbert_v2() -> Self {
        Self {
            id: "jina-colbert-v2".into(),
            name: "Jina ColBERT v2".into(),
            provider: "jina".into(),
            max_tokens: 8192,
            cost_per_1k: 0.02,
            quality: 9,
            speed: 8,
        }
    }

    // ─── Local Models ─────────────────────────────────────────────────────────

    pub fn cross_encoder_msmarco() -> Self {
        Self {
            id: "cross-encoder/ms-marco-MiniLM-L-6-v2".into(),
            name: "MS MARCO Cross-Encoder".into(),
            provider: "local".into(),
            max_tokens: 512,
            cost_per_1k: 0.0,
            quality: 7,
            speed: 6,
        }
    }

    pub fn bge_reranker_large() -> Self {
        Self {
            id: "BAAI/bge-reranker-large".into(),
            name: "BGE Reranker Large".into(),
            provider: "local".into(),
            max_tokens: 512,
            cost_per_1k: 0.0,
            quality: 8,
            speed: 5,
        }
    }

    // ─── LLM-based ────────────────────────────────────────────────────────────

    pub fn gpt4_rerank() -> Self {
        Self {
            id: "gpt-4-rerank".into(),
            name: "GPT-4 Rerank".into(),
            provider: "openai".into(),
            max_tokens: 128000,
            cost_per_1k: 30.0,
            quality: 10,
            speed: 3,
        }
    }

    pub fn claude_rerank() -> Self {
        Self {
            id: "claude-rerank".into(),
            name: "Claude Rerank".into(),
            provider: "anthropic".into(),
            max_tokens: 200000,
            cost_per_1k: 15.0,
            quality: 10,
            speed: 4,
        }
    }

    /// Get all models
    pub fn all() -> Vec<Self> {
        vec![
            Self::cohere_rerank_english_v3(),
            Self::cohere_rerank_multilingual_v3(),
            Self::jina_reranker_v1(),
            Self::jina_colbert_v2(),
            Self::cross_encoder_msmarco(),
            Self::bge_reranker_large(),
            Self::gpt4_rerank(),
            Self::claude_rerank(),
        ]
    }

    /// Get free models
    pub fn free() -> Vec<Self> {
        Self::all().into_iter().filter(|m| m.cost_per_1k == 0.0).collect()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  RERANKER TRAIT
// ═══════════════════════════════════════════════════════════════════════════════

/// Reranker provider trait
#[async_trait]
pub trait Reranker: Send + Sync {
    /// Provider name
    fn name(&self) -> &str;

    /// Available models
    fn models(&self) -> Vec<RerankModel>;

    /// Check if configured
    fn is_configured(&self) -> bool;

    /// Rerank documents
    async fn rerank(&self, request: RerankRequest) -> RerankResult<RerankResponse>;

    /// Rerank texts (convenience)
    async fn rerank_texts(&self, query: &str, texts: &[&str]) -> RerankResult<Vec<(usize, f32)>> {
        let documents: Vec<RerankDocument> = texts.iter().map(|&t| t.into()).collect();
        let request = RerankRequest::new(query, documents);
        
        let response = self.rerank(request).await?;
        
        Ok(response.results.into_iter()
            .map(|r| (r.index, r.relevance_score))
            .collect())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  RERANKER ENGINE
// ═══════════════════════════════════════════════════════════════════════════════

/// Reranking engine statistics
#[derive(Debug, Clone, Default)]
pub struct RerankStats {
    pub total_queries: u64,
    pub total_documents: u64,
    pub total_tokens: u64,
    pub total_cost: f64,
    pub avg_latency_ms: f64,
    pub cache_hits: u64,
}

/// Main reranking engine
pub struct RerankEngine {
    providers: HashMap<String, Arc<dyn Reranker>>,
    default_provider: String,
    stats: Arc<RwLock<RerankStats>>,
}

impl RerankEngine {
    /// Create new engine
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            default_provider: "cohere".into(),
            stats: Arc::new(RwLock::new(RerankStats::default())),
        }
    }

    /// Create from environment
    pub fn from_env() -> RerankResult<Self> {
        let mut engine = Self::new();
        
        if let Ok(key) = std::env::var("COHERE_API_KEY") {
            engine.add_provider("cohere", providers::CohereReranker::new(key));
        }
        
        if let Ok(key) = std::env::var("JINA_API_KEY") {
            engine.add_provider("jina", providers::JinaReranker::new(key));
        }
        
        // Local always available
        engine.add_provider("local", providers::LocalReranker::new());
        
        Ok(engine)
    }

    /// Add provider
    pub fn add_provider<R: Reranker + 'static>(&mut self, name: &str, provider: R) {
        self.providers.insert(name.to_string(), Arc::new(provider));
    }

    /// Set default provider
    pub fn set_default(&mut self, name: &str) {
        self.default_provider = name.to_string();
    }

    /// Rerank documents
    pub async fn rerank(&self, request: RerankRequest) -> RerankResult<RerankResponse> {
        let provider = self.providers.get(&self.default_provider)
            .ok_or_else(|| RerankError::ProviderNotFound(self.default_provider.clone()))?;
        
        let start = std::time::Instant::now();
        let response = provider.rerank(request).await?;
        
        // Update stats
        {
            let mut stats = self.stats.write();
            stats.total_queries += 1;
            stats.total_documents += response.results.len() as u64;
            stats.total_tokens += response.total_tokens as u64;
            stats.total_cost += response.cost;
            
            let latency = start.elapsed().as_millis() as f64;
            let n = stats.total_queries as f64;
            stats.avg_latency_ms = stats.avg_latency_ms * (n - 1.0) / n + latency / n;
        }
        
        Ok(response)
    }

    /// Rerank with specific provider
    pub async fn rerank_with(&self, provider: &str, request: RerankRequest) -> RerankResult<RerankResponse> {
        let provider = self.providers.get(provider)
            .ok_or_else(|| RerankError::ProviderNotFound(provider.to_string()))?;
        
        provider.rerank(request).await
    }

    /// Get statistics
    pub fn stats(&self) -> RerankStats {
        self.stats.read().clone()
    }

    /// List models
    pub fn list_models(&self) -> Vec<RerankModel> {
        RerankModel::all()
    }
}

impl Default for RerankEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = RerankDocument::new("Hello world")
            .with_id("doc-1")
            .with_score(0.95);
        
        assert_eq!(doc.id, Some("doc-1".to_string()));
        assert_eq!(doc.original_score, Some(0.95));
    }

    #[test]
    fn test_model_list() {
        let models = RerankModel::all();
        assert!(models.len() >= 6);
    }

    #[test]
    fn test_free_models() {
        let free = RerankModel::free();
        assert!(free.iter().all(|m| m.cost_per_1k == 0.0));
    }

    #[test]
    fn test_engine_creation() {
        let engine = RerankEngine::new();
        let stats = engine.stats();
        assert_eq!(stats.total_queries, 0);
    }
}
