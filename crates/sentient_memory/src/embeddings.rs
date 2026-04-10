//! ─── EMBEDDING MOTORU ───
//!
//! Metinleri vektörlere dönüştürme:
//! - V-GATE API üzerinden embedding
//! - Yerel fallback (hash-based)
//! - Batch processing

use crate::MemoryError;
use crate::MemoryResult;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// ─────────────────────────────────────────────────────────────────────────────
// EMBEDDING CONFIG
// ─────────────────────────────────────────────────────────────────────────────

/// Embedding motoru yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingConfig {
    /// V-GATE sunucu URL
    pub vgate_url: String,
    /// Kullanılacak model
    pub model: String,
    /// Vektör boyutu
    pub dimension: usize,
    /// Batch boyutu
    pub batch_size: usize,
    /// Timeout (saniye)
    pub timeout_secs: u64,
    /// Yerel fallback kullan
    pub use_local_fallback: bool,
}

impl Default for EmbeddingConfig {
    fn default() -> Self {
        Self {
            vgate_url: "http://127.0.0.1:1071".into(),
            model: "text-embedding-3-small".into(),
            dimension: 1536,
            batch_size: 32,
            timeout_secs: 30,
            use_local_fallback: true,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// EMBEDDING RESPONSE
// ─────────────────────────────────────────────────────────────────────────────

/// V-GATE embedding yanıtı
#[derive(Debug, Clone, Serialize, Deserialize)]
struct EmbeddingResponse {
    object: String,
    data: Vec<EmbeddingData>,
    model: String,
    usage: EmbeddingUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EmbeddingData {
    object: String,
    embedding: Vec<f32>,
    index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EmbeddingUsage {
    prompt_tokens: usize,
    total_tokens: usize,
}

// ─────────────────────────────────────────────────────────────────────────────
// EMBEDDING ENGINE
// ─────────────────────────────────────────────────────────────────────────────

/// Embedding motoru
pub struct EmbeddingEngine {
    config: EmbeddingConfig,
    client: reqwest::Client,
}

impl EmbeddingEngine {
    /// Yeni embedding motoru oluştur
    pub fn new(config: EmbeddingConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        
        Self { config, client }
    }
    
    /// Varsayılan yapılandırma ile oluştur
    pub fn with_defaults() -> Self {
        Self::new(EmbeddingConfig::default())
    }
    
    /// Tek metni embedding'e çevir
    pub async fn embed(&self, text: &str) -> MemoryResult<Vec<f32>> {
        let embeddings = self.embed_batch(&[text.to_string()]).await?;
        embeddings.into_iter().next()
            .ok_or_else(|| MemoryError::EmbeddingError("Embedding sonucu boş".into()))
    }
    
    /// Çoklu metin embedding
    pub async fn embed_batch(&self, texts: &[String]) -> MemoryResult<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }
        
        // V-GATE üzerinden dene
        match self.embed_via_vgate(texts).await {
            Ok(embeddings) => {
                log::debug!("embed  V-GATE üzerinden {} vektör alındı", embeddings.len());
                return Ok(embeddings);
            }
            Err(e) => {
                log::warn!("embed  V-GATE başarısız: {}, yerel fallback kullanılıyor", e);
            }
        }
        
        // Yerel fallback
        if self.config.use_local_fallback {
            return self.embed_local_fallback(texts);
        }
        
        Err(MemoryError::EmbeddingError("Embedding alınamadı".into()))
    }
    
    /// V-GATE API üzerinden embedding
    async fn embed_via_vgate(&self, texts: &[String]) -> MemoryResult<Vec<Vec<f32>>> {
        let url = format!("{}/v1/embeddings", self.config.vgate_url);
        
        let body = serde_json::json!({
            "model": self.config.model,
            "input": texts,
            "encoding_format": "float"
        });
        
        let response = self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| MemoryError::EmbeddingError(format!("V-GATE bağlantı hatası: {}", e)))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(MemoryError::EmbeddingError(
                format!("V-GATE hata {}: {}", status, text)
            ));
        }
        
        let embedding_response: EmbeddingResponse = response
            .json()
            .await
            .map_err(|e| MemoryError::EmbeddingError(format!("JSON parse hatası: {}", e)))?;
        
        // Index'e göre sırala
        let mut embeddings = vec![Vec::new(); texts.len()];
        for data in embedding_response.data {
            if data.index < embeddings.len() {
                embeddings[data.index] = data.embedding;
            }
        }
        
        // Boyut kontrolü
        for (i, emb) in embeddings.iter().enumerate() {
            if emb.len() != self.config.dimension {
                return Err(MemoryError::EmbeddingError(
                    format!("Embedding boyutu hatalı: beklenen {}, alınan {}", 
                            self.config.dimension, emb.len())
                ));
            }
        }
        
        Ok(embeddings)
    }
    
    /// Yerel hash-based fallback embedding
    /// Not: Gerçek semantik anlam yok, sadece benzersizlik için
    fn embed_local_fallback(&self, texts: &[String]) -> MemoryResult<Vec<Vec<f32>>> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let dim = self.config.dimension;
        
        let embeddings: Vec<Vec<f32>> = texts
            .par_iter()
            .map(|text| {
                let mut embedding = vec![0.0f32; dim];
                
                // Hash'ten deterministic vektör üret
                let mut hasher = DefaultHasher::new();
                text.hash(&mut hasher);
                let hash = hasher.finish();
                
                // Hash'i vektöre dağıt
                for i in 0..dim {
                    let idx = (hash as usize + i) % dim;
                    let val = ((hash.wrapping_add(i as u64)) as f32 / u64::MAX as f32) * 2.0 - 1.0;
                    embedding[idx] = val;
                }
                
                // Normalize et
                let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
                if norm > 0.0 {
                    for x in &mut embedding {
                        *x /= norm;
                    }
                }
                
                embedding
            })
            .collect();
        
        log::debug!("embed  Yerel fallback: {} vektör üretildi", embeddings.len());
        Ok(embeddings)
    }
    
    /// Token'ları say (yaklaşık)
    pub fn count_tokens(&self, text: &str) -> usize {
        // Basit whitespace tabanlı tahmin
        // Gerçek tokenizer için tokenizers crate kullanılabilir
        text.split_whitespace().count()
    }
    
    /// Toplam token sayısı
    pub fn count_tokens_batch(&self, texts: &[String]) -> usize {
        texts.iter().map(|t| self.count_tokens(t)).sum()
    }
    
    /// Yapılandırma
    pub fn config(&self) -> &EmbeddingConfig {
        &self.config
    }
    
    /// Vektör boyutu
    pub fn dimension(&self) -> usize {
        self.config.dimension
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// COSINE SIMILARITY
// ─────────────────────────────────────────────────────────────────────────────

/// İki vektör arası kosinüs benzerliği
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    
    (dot / (norm_a * norm_b)).clamp(-1.0, 1.0)
}

/// Top-N en benzer vektörleri bul
pub fn find_top_similar(query: &[f32], candidates: &[(usize, Vec<f32>)], top_k: usize) -> Vec<(usize, f32)> {
    let mut similarities: Vec<_> = candidates
        .iter()
        .map(|(idx, vec)| (*idx, cosine_similarity(query, vec)))
        .filter(|(_, sim)| *sim > 0.0)
        .collect();
    
    similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    similarities.truncate(top_k);
    similarities
}

// ─────────────────────────────────────────────────────────────────────────────
// SYNC WRAPPER
// ─────────────────────────────────────────────────────────────────────────────

/// Senkron wrapper (blokluyor!)
pub struct EmbeddingEngineSync {
    inner: Arc<EmbeddingEngine>,
    runtime: tokio::runtime::Runtime,
}

impl EmbeddingEngineSync {
    pub fn new(config: EmbeddingConfig) -> Self {
        Self {
            inner: Arc::new(EmbeddingEngine::new(config)),
            runtime: tokio::runtime::Runtime::new().expect("operation failed"),
        }
    }
    
    pub fn embed(&self, text: &str) -> MemoryResult<Vec<f32>> {
        self.runtime.block_on(self.inner.embed(text))
    }
    
    pub fn embed_batch(&self, texts: &[String]) -> MemoryResult<Vec<Vec<f32>>> {
        self.runtime.block_on(self.inner.embed_batch(texts))
    }
    
    pub fn dimension(&self) -> usize {
        self.inner.dimension()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TESTLER
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim - 1.0).abs() < 0.01);
        
        let c = vec![0.0, 1.0, 0.0];
        let sim = cosine_similarity(&a, &c);
        assert!(sim.abs() < 0.01);
        
        let d = vec![-1.0, 0.0, 0.0];
        let sim = cosine_similarity(&a, &d);
        assert!((sim - (-1.0)).abs() < 0.01);
    }
    
    #[test]
    fn test_local_fallback() {
        let config = EmbeddingConfig {
            dimension: 128,
            ..Default::default()
        };
        let engine = EmbeddingEngine::new(config);
        
        let texts = vec!["Test metin".into(), "Başka metin".into()];
        let embeddings = engine.embed_local_fallback(&texts).expect("operation failed");
        
        assert_eq!(embeddings.len(), 2);
        assert_eq!(embeddings[0].len(), 128);
        
        // Normalize edilmiş mi?
        let norm: f32 = embeddings[0].iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 0.01);
    }
    
    #[test]
    fn test_find_top_similar() {
        let query = vec![1.0, 0.0, 0.0];
        let candidates = vec![
            (0, vec![0.9, 0.1, 0.0]),
            (1, vec![0.0, 1.0, 0.0]),
            (2, vec![0.95, 0.05, 0.0]),
            (3, vec![0.0, 0.0, 1.0]),
        ];
        
        let top = find_top_similar(&query, &candidates, 2);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].0, 2); // En benzer
        assert_eq!(top[1].0, 0); // İkinci
    }
    
    #[test]
    fn test_embedding_config_default() {
        let config = EmbeddingConfig::default();
        assert_eq!(config.dimension, 1536);
        assert_eq!(config.model, "text-embedding-3-small");
    }
}
