//! ─── Hybrid Fusion ───

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{RerankDocument, RerankedDocument};

// ═══════════════════════════════════════════════════════════════════════════════
//  FUSION STRATEGIES
// ═══════════════════════════════════════════════════════════════════════════════

/// Fusion strategy for combining multiple rankings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FusionStrategy {
    /// Reciprocal Rank Fusion (RRF)
    Rrf { k: u32 },
    /// Weighted combination
    Weighted { semantic_weight: f32, keyword_weight: f32 },
    /// Linear combination
    Linear { weights: Vec<f32> },
    /// Geometric mean
    GeometricMean,
    /// Harmonic mean
    HarmonicMean,
}

impl Default for FusionStrategy {
    fn default() -> Self {
        Self::Rrf { k: 60 }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  HYBRID FUSION
// ═══════════════════════════════════════════════════════════════════════════════

/// Hybrid fusion for combining semantic and keyword search
pub struct HybridFusion {
    strategy: FusionStrategy,
}

impl HybridFusion {
    /// Create new fusion with strategy
    pub fn new(strategy: FusionStrategy) -> Self {
        Self { strategy }
    }

    /// Create with RRF (default)
    pub fn rrf() -> Self {
        Self::new(FusionStrategy::Rrf { k: 60 })
    }

    /// Create with weighted combination
    pub fn weighted(semantic_weight: f32, keyword_weight: f32) -> Self {
        Self::new(FusionStrategy::Weighted { semantic_weight, keyword_weight })
    }

    /// Fuse semantic and keyword results
    pub fn fuse(
        &self,
        semantic_results: &[(usize, f32)],
        keyword_results: &[(usize, f32)],
        documents: &[RerankDocument],
    ) -> Vec<RerankedDocument> {
        match &self.strategy {
            FusionStrategy::Rrf { k } => self.rrf_fusion(semantic_results, keyword_results, documents, *k),
            FusionStrategy::Weighted { semantic_weight, keyword_weight } => {
                self.weighted_fusion(semantic_results, keyword_results, documents, *semantic_weight, *keyword_weight)
            }
            _ => self.rrf_fusion(semantic_results, keyword_results, documents, 60),
        }
    }

    /// Reciprocal Rank Fusion
    fn rrf_fusion(
        &self,
        semantic_results: &[(usize, f32)],
        keyword_results: &[(usize, f32)],
        documents: &[RerankDocument],
        k: u32,
    ) -> Vec<RerankedDocument> {
        let mut rrf_scores: HashMap<usize, f32> = HashMap::new();

        // Add semantic rankings
        for (rank, (idx, _)) in semantic_results.iter().enumerate() {
            let rrf = 1.0 / (k as f32 + (rank + 1) as f32);
            *rrf_scores.entry(*idx).or_insert(0.0) += rrf;
        }

        // Add keyword rankings
        for (rank, (idx, _)) in keyword_results.iter().enumerate() {
            let rrf = 1.0 / (k as f32 + (rank + 1) as f32);
            *rrf_scores.entry(*idx).or_insert(0.0) += rrf;
        }

        // Convert to results
        let mut results: Vec<RerankedDocument> = rrf_scores.into_iter()
            .map(|(idx, score)| {
                let doc = documents.get(idx);
                RerankedDocument {
                    index: idx,
                    id: doc.and_then(|d| d.id.clone()),
                    text: doc.map(|d| d.text.clone()).unwrap_or_default(),
                    relevance_score: score,
                    original_score: doc.and_then(|d| d.original_score),
                    score_delta: None,
                    metadata: doc.map(|d| d.metadata.clone()).unwrap_or_default(),
                }
            })
            .collect();

        // Sort by score
        results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
        
        results
    }

    /// Weighted fusion
    fn weighted_fusion(
        &self,
        semantic_results: &[(usize, f32)],
        keyword_results: &[(usize, f32)],
        documents: &[RerankDocument],
        semantic_weight: f32,
        keyword_weight: f32,
    ) -> Vec<RerankedDocument> {
        let mut scores: HashMap<usize, (f32, f32)> = HashMap::new();

        // Collect semantic scores
        for (idx, score) in semantic_results {
            scores.entry(*idx).or_insert((0.0, 0.0)).0 = *score;
        }

        // Collect keyword scores
        for (idx, score) in keyword_results {
            scores.entry(*idx).or_insert((0.0, 0.0)).1 = *score;
        }

        // Combine with weights
        let mut results: Vec<RerankedDocument> = scores.into_iter()
            .map(|(idx, (sem_score, kw_score))| {
                let combined = sem_score * semantic_weight + kw_score * keyword_weight;
                let doc = documents.get(idx);
                RerankedDocument {
                    index: idx,
                    id: doc.and_then(|d| d.id.clone()),
                    text: doc.map(|d| d.text.clone()).unwrap_or_default(),
                    relevance_score: combined,
                    original_score: doc.and_then(|d| d.original_score),
                    score_delta: doc.and_then(|d| d.original_score)
                        .map(|s| combined - s),
                    metadata: doc.map(|d| d.metadata.clone()).unwrap_or_default(),
                }
            })
            .collect();

        // Sort by score
        results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
        
        results
    }
}

impl Default for HybridFusion {
    fn default() -> Self {
        Self::rrf()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rrf_fusion() {
        let fusion = HybridFusion::rrf();
        let docs = vec![
            RerankDocument::new("doc1"),
            RerankDocument::new("doc2"),
            RerankDocument::new("doc3"),
        ];
        
        let semantic = vec![(0, 0.9), (1, 0.8), (2, 0.7)];
        let keyword = vec![(1, 1.0), (2, 0.9), (0, 0.8)];
        
        let results = fusion.fuse(&semantic, &keyword, &docs);
        
        assert_eq!(results.len(), 3);
        // Should be sorted by combined RRF score
    }

    #[test]
    fn test_weighted_fusion() {
        let fusion = HybridFusion::weighted(0.7, 0.3);
        let docs = vec![
            RerankDocument::new("doc1"),
            RerankDocument::new("doc2"),
        ];
        
        let semantic = vec![(0, 1.0), (1, 0.5)];
        let keyword = vec![(1, 1.0), (0, 0.5)];
        
        let results = fusion.fuse(&semantic, &keyword, &docs);
        
        assert_eq!(results.len(), 2);
    }
}
