//! ─── Embedding Engine ───

use crate::{MemoryError, Result};
use serde::{Deserialize, Serialize};

/// Embedding configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingConfig {
    /// Model name
    pub model: String,
    
    /// Embedding dimension
    pub dimension: usize,
    
    /// Batch size
    pub batch_size: usize,
}

impl Default for EmbeddingConfig {
    fn default() -> Self {
        Self {
            model: "all-MiniLM-L6-v2".into(),
            dimension: 384,
            batch_size: 32,
        }
    }
}

/// Embedding engine
#[cfg(feature = "embeddings")]
pub struct EmbeddingEngine {
    model: fastembed::TextEmbedding,
    config: EmbeddingConfig,
}

#[cfg(feature = "embeddings")]
impl EmbeddingEngine {
    /// Create new embedding engine
    pub fn new(config: EmbeddingConfig) -> Result<Self> {
        let model = fastembed::TextEmbedding::try_new(
            fastembed::InitOptions::new(fastembed::EmbeddingModel::AllMiniLML6V2)
        ).map_err(|e| MemoryError::Embedding(e.to_string()))?;
        
        Ok(Self { model, config })
    }
    
    /// Embed single text
    pub async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let embeddings = self.model.embed(vec![text], None)
            .map_err(|e| MemoryError::Embedding(e.to_string()))?;
        
        embeddings.into_iter()
            .next()
            .map(|e| e.to_vec())
            .ok_or_else(|| MemoryError::Embedding("No embedding generated".into()))
    }
    
    /// Embed batch
    pub async fn embed_batch(&self, texts: Vec<&str>) -> Result<Vec<Vec<f32>>> {
        let embeddings = self.model.embed(texts, None)
            .map_err(|e| MemoryError::Embedding(e.to_string()))?;
        
        Ok(embeddings.into_iter().map(|e| e.to_vec()).collect())
    }
    
    /// Get embedding dimension
    pub fn dimension(&self) -> usize {
        self.config.dimension
    }
}

#[cfg(not(feature = "embeddings"))]
pub struct EmbeddingEngine;

#[cfg(not(feature = "embeddings"))]
impl EmbeddingEngine {
    pub fn new(_config: EmbeddingConfig) -> Result<Self> {
        Ok(Self)
    }
    
    pub async fn embed(&self, _text: &str) -> Result<Vec<f32>> {
        Err(MemoryError::Embedding(
            "Embeddings disabled. Enable 'embeddings' feature.".into()
        ))
    }
    
    pub async fn embed_batch(&self, _texts: Vec<&str>) -> Result<Vec<Vec<f32>>> {
        Err(MemoryError::Embedding(
            "Embeddings disabled. Enable 'embeddings' feature.".into()
        ))
    }
    
    pub fn dimension(&self) -> usize {
        384
    }
}
