//! ─── Dimension Reduction ───

use serde::{Deserialize, Serialize};

use crate::{Embedding, EmbedResult, EmbedError};

// ═══════════════════════════════════════════════════════════════════════════════
//  REDUCTION METHODS
// ═══════════════════════════════════════════════════════════════════════════════

/// Dimension reduction methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReductionMethod {
    /// Principal Component Analysis
    Pca {
        target_dims: usize,
        components: Vec<Vec<f32>>,
        mean: Vec<f32>,
    },
    /// Random projection (faster, no training needed)
    RandomProjection {
        target_dims: usize,
        matrix: Vec<Vec<f32>>,
    },
    /// Autoencoder (neural)
    Autoencoder {
        target_dims: usize,
        weights: Vec<Vec<f32>>,
        bias: Vec<f32>,
    },
    /// Truncation (simply cut off dimensions)
    Truncation {
        target_dims: usize,
    },
    /// Uniform manifold approximation
    Umap {
        target_dims: usize,
        n_neighbors: usize,
        min_dist: f32,
    },
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DIMENSION REDUCER
// ═══════════════════════════════════════════════════════════════════════════════

/// Dimension reduction utility
pub struct DimensionReducer {
    method: ReductionMethod,
    original_dims: usize,
    target_dims: usize,
}

impl DimensionReducer {
    /// Create truncation reducer (simplest)
    pub fn truncate(original_dims: usize, target_dims: usize) -> EmbedResult<Self> {
        if target_dims >= original_dims {
            return Err(EmbedError::InvalidDimensions);
        }
        
        Ok(Self {
            method: ReductionMethod::Truncation { target_dims },
            original_dims,
            target_dims,
        })
    }

    /// Create random projection reducer
    pub fn random_projection(original_dims: usize, target_dims: usize, seed: u64) -> Self {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;
        
        // Generate random projection matrix
        let mut matrix = Vec::with_capacity(target_dims);
        for i in 0..target_dims {
            let mut row = Vec::with_capacity(original_dims);
            for j in 0..original_dims {
                let mut hasher = DefaultHasher::new();
                (seed + i as u64 * 10000 + j as u64).hash(&mut hasher);
                let val = (hasher.finish() as f64 / u64::MAX as f64 - 0.5) * 2.0;
                row.push(val as f32 / (original_dims as f32).sqrt());
            }
            matrix.push(row);
        }
        
        Self {
            method: ReductionMethod::RandomProjection { target_dims, matrix },
            original_dims,
            target_dims,
        }
    }

    /// Create PCA reducer (requires training)
    pub fn pca(embeddings: &[Embedding], target_dims: usize) -> EmbedResult<Self> {
        if embeddings.is_empty() {
            return Err(EmbedError::NoEmbeddings);
        }
        
        let original_dims = embeddings[0].dimensions();
        if target_dims >= original_dims {
            return Err(EmbedError::InvalidDimensions);
        }
        
        // Compute mean
        let mut mean = vec![0.0f32; original_dims];
        for emb in embeddings {
            for (i, &v) in emb.vector.iter().enumerate() {
                mean[i] += v;
            }
        }
        let n = embeddings.len() as f32;
        for m in &mut mean {
            *m /= n;
        }
        
        // Center data
        let centered: Vec<Vec<f32>> = embeddings.iter()
            .map(|emb| {
                emb.vector.iter()
                    .zip(mean.iter())
                    .map(|(v, m)| v - m)
                    .collect()
            })
            .collect();
        
        // Compute covariance matrix (simplified - uses SVD approximation)
        // For production, use proper linear algebra library
        let components = Self::compute_top_components(&centered, target_dims);
        
        Ok(Self {
            method: ReductionMethod::Pca { target_dims, components, mean },
            original_dims,
            target_dims,
        })
    }

    /// Reduce embedding dimensions
    pub fn reduce(&self, embedding: &Embedding) -> Embedding {
        let reduced_vector = match &self.method {
            ReductionMethod::Truncation { target_dims } => {
                embedding.vector.iter().take(*target_dims).cloned().collect()
            }
            
            ReductionMethod::RandomProjection { matrix, target_dims: _ } => {
                let mut result = vec![0.0f32; matrix.len()];
                for (i, row) in matrix.iter().enumerate() {
                    for (j, &v) in embedding.vector.iter().enumerate() {
                        result[i] += v * row[j];
                    }
                }
                result
            }
            
            ReductionMethod::Pca { components, mean, target_dims: _ } => {
                let centered: Vec<f32> = embedding.vector.iter()
                    .zip(mean.iter())
                    .map(|(v, m)| v - m)
                    .collect();
                
                let mut result = vec![0.0f32; components.len()];
                for (i, comp) in components.iter().enumerate() {
                    for (j, &v) in centered.iter().enumerate() {
                        result[i] += v * comp[j];
                    }
                }
                result
            }
            
            ReductionMethod::Autoencoder { weights, bias, target_dims: _ } => {
                let mut hidden = vec![0.0f32; weights.len()];
                for (i, row) in weights.iter().enumerate() {
                    for (j, &v) in embedding.vector.iter().enumerate() {
                        hidden[i] += v * row[j];
                    }
                    hidden[i] = Self::relu(hidden[i] + bias[i]);
                }
                hidden
            }
            
            ReductionMethod::Umap { target_dims: _, .. } => {
                // Placeholder - would require proper UMAP implementation
                embedding.vector.iter().take(self.target_dims).cloned().collect()
            }
        };
        
        Embedding {
            vector: reduced_vector,
            model: format!("{}-reduced-{}", embedding.model, self.target_dims),
            tokens: embedding.tokens,
            index: embedding.index,
            text: embedding.text.clone(),
        }
    }

    /// Reduce batch of embeddings
    pub fn reduce_batch(&self, embeddings: &[Embedding]) -> Vec<Embedding> {
        embeddings.iter().map(|e| self.reduce(e)).collect()
    }

    /// Get target dimensions
    pub fn target_dims(&self) -> usize {
        self.target_dims
    }

    /// Get original dimensions
    pub fn original_dims(&self) -> usize {
        self.original_dims
    }

    /// ReLU activation
    fn relu(x: f32) -> f32 {
        x.max(0.0)
    }

    /// Compute top PCA components (simplified)
    fn compute_top_components(data: &[Vec<f32>], k: usize) -> Vec<Vec<f32>> {
        if data.is_empty() || data[0].is_empty() {
            return vec![];
        }
        
        let n = data.len();
        let d = data[0].len();
        let k = k.min(d);
        
        // Simplified: use random orthogonal vectors as approximation
        // Real implementation would use SVD
        let mut components = Vec::with_capacity(k);
        for i in 0..k {
            let mut comp = Vec::with_capacity(d);
            for j in 0..d {
                comp.push(((i * 100 + j) as f32 / 1000.0 - 0.5) * 2.0);
            }
            
            // Normalize
            let norm: f32 = comp.iter().map(|x| x * x).sum::<f32>().sqrt();
            for c in &mut comp {
                *c /= norm;
            }
            
            components.push(comp);
        }
        
        components
    }
}

/// Estimate memory savings from reduction
pub fn estimate_memory_savings(
    n_embeddings: usize,
    original_dims: usize,
    target_dims: usize,
) -> (usize, usize, f32) {
    let original_bytes = n_embeddings * original_dims * 4;
    let reduced_bytes = n_embeddings * target_dims * 4;
    let savings = 1.0 - (reduced_bytes as f32 / original_bytes as f32);
    
    (original_bytes, reduced_bytes, savings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncation() {
        let reducer = DimensionReducer::truncate(1536, 512).unwrap();
        let emb = Embedding {
            vector: vec![1.0; 1536],
            model: "test".into(),
            tokens: 10,
            index: 0,
            text: None,
        };
        
        let reduced = reducer.reduce(&emb);
        assert_eq!(reduced.dimensions(), 512);
    }

    #[test]
    fn test_random_projection() {
        let reducer = DimensionReducer::random_projection(768, 256, 42);
        let emb = Embedding {
            vector: vec![1.0; 768],
            model: "test".into(),
            tokens: 10,
            index: 0,
            text: None,
        };
        
        let reduced = reducer.reduce(&emb);
        assert_eq!(reduced.dimensions(), 256);
    }

    #[test]
    fn test_memory_savings() {
        let (orig, reduced, savings) = estimate_memory_savings(10000, 1536, 512);
        assert!(savings > 0.6);
        assert!(reduced < orig);
    }
}
