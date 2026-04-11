// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Embeddings
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};

/// Embedding vector
pub type EmbeddingVector = Vec<f32>;

/// Embedding model trait
pub trait EmbeddingModel: Send + Sync {
    /// Get embedding dimension
    fn dimension(&self) -> usize;

    /// Embed single text
    fn embed(&self, text: &str) -> crate::Result<EmbeddingVector>;

    /// Embed multiple texts
    fn embed_batch(&self, texts: &[&str]) -> crate::Result<Vec<EmbeddingVector>> {
        texts.iter().map(|t| self.embed(t)).collect()
    }
}

/// Cosine similarity between two vectors
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

/// Euclidean distance between two vectors
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

/// Dot product between two vectors
pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// Mock embedding model for testing
pub struct MockEmbeddingModel {
    dimension: usize,
}

impl MockEmbeddingModel {
    pub fn new(dimension: usize) -> Self {
        Self { dimension }
    }
}

impl Default for MockEmbeddingModel {
    fn default() -> Self {
        Self::new(384)
    }
}

impl EmbeddingModel for MockEmbeddingModel {
    fn dimension(&self) -> usize {
        self.dimension
    }

    fn embed(&self, text: &str) -> crate::Result<EmbeddingVector> {
        // Generate deterministic embedding based on text hash
        let mut embedding = vec![0.0; self.dimension];
        
        for (i, byte) in text.bytes().enumerate() {
            let idx = i % self.dimension;
            embedding[idx] += (byte as f32 - 64.0) / 64.0;
        }

        // Normalize
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for val in &mut embedding {
                *val /= norm;
            }
        }

        Ok(embedding)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let c = vec![0.0, 1.0, 0.0];

        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 0.001);
        assert!((cosine_similarity(&a, &c) - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_euclidean_distance() {
        let a = vec![0.0, 0.0, 0.0];
        let b = vec![1.0, 1.0, 1.0];

        let dist = euclidean_distance(&a, &b);
        assert!((dist - 1.732).abs() < 0.01); // sqrt(3)
    }

    #[test]
    fn test_mock_embedding_model() {
        let model = MockEmbeddingModel::new(384);
        let embedding = model.embed("test text").unwrap();

        assert_eq!(embedding.len(), 384);
    }

    #[test]
    fn test_embedding_similarity() {
        let model = MockEmbeddingModel::new(128);

        let emb1 = model.embed("hello world").unwrap();
        let emb2 = model.embed("hello world").unwrap();
        let emb3 = model.embed("completely different").unwrap();

        // Same text should have same embedding
        assert!((cosine_similarity(&emb1, &emb2) - 1.0).abs() < 0.001);

        // Different text should have different embedding
        assert!(cosine_similarity(&emb1, &emb3) < 1.0);
    }
}
