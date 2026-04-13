//! ─── Diversity Reranking ───

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{RerankDocument, RerankedDocument};

// ═══════════════════════════════════════════════════════════════════════════════
//  DIVERSITY CONFIG
// ═══════════════════════════════════════════════════════════════════════════════

/// Diversity configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiversityConfig {
    /// Diversity factor (0.0 = none, 1.0 = maximum)
    pub diversity_factor: f32,
    /// Minimum similarity threshold for deduplication
    pub similarity_threshold: f32,
    /// Number of diversity buckets (for categorical diversity)
    pub num_buckets: usize,
    /// Bucket assignment function (metadata key)
    pub bucket_key: Option<String>,
}

impl Default for DiversityConfig {
    fn default() -> Self {
        Self {
            diversity_factor: 0.3,
            similarity_threshold: 0.95,
            num_buckets: 5,
            bucket_key: None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DIVERSITY RERANKER
// ═══════════════════════════════════════════════════════════════════════════════

/// Diversity-aware reranking
pub struct DiversityReranker {
    config: DiversityConfig,
}

impl DiversityReranker {
    /// Create new diversity reranker
    pub fn new(config: DiversityConfig) -> Self {
        Self { config }
    }

    /// Apply diversity to results
    pub fn rerank(&self, results: &[RerankedDocument]) -> Vec<RerankedDocument> {
        if self.config.diversity_factor == 0.0 {
            return results.to_vec();
        }

        let mut selected: Vec<RerankedDocument> = Vec::new();
        let mut seen_texts: HashSet<String> = HashSet::new();

        for result in results {
            // Skip duplicates
            let text_normalized = result.text.to_lowercase()
                .chars()
                .filter(|c: &char| c.is_alphanumeric() || c.is_whitespace())
                .collect::<String>();
            
            if seen_texts.contains(&text_normalized) {
                continue;
            }

            // Check if too similar to already selected
            let mut too_similar = false;
            for selected_result in &selected {
                let sim = self.text_similarity(&selected_result.text, &result.text);
                if sim > self.config.similarity_threshold {
                    too_similar = true;
                    break;
                }
            }

            if too_similar && self.config.diversity_factor > 0.5 {
                continue;
            }

            seen_texts.insert(text_normalized);
            selected.push(result.clone());

            // Apply bucket-based diversity if configured
            if let Some(bucket_key) = &self.config.bucket_key {
                self.apply_bucket_diversity(&mut selected, bucket_key);
            }
        }

        selected
    }

    /// Calculate text similarity (simple Jaccard)
    fn text_similarity(&self, text1: &str, text2: &str) -> f32 {
        let text1_lower = text1.to_lowercase();
        let text2_lower = text2.to_lowercase();
        let words1: HashSet<&str> = text1_lower.split_whitespace().collect();
        let words2: HashSet<&str> = text2_lower.split_whitespace().collect();
        
        if words1.is_empty() || words2.is_empty() {
            return 0.0;
        }

        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();
        
        intersection as f32 / union as f32
    }

    /// Apply bucket-based diversity
    fn apply_bucket_diversity(&self, selected: &mut Vec<RerankedDocument>, _bucket_key: &str) {
        // Group results by bucket (e.g., category, source)
        // Limit results per bucket for diversity
        // This is a placeholder for the actual implementation
    }

    /// MMR (Maximal Marginal Relevance) selection
    pub fn mmr_select(
        &self,
        query_embedding: &[f32],
        doc_embeddings: &[Vec<f32>],
        documents: &[RerankDocument],
        top_k: usize,
        lambda: f32,
    ) -> Vec<RerankedDocument> {
        let mut selected_indices: Vec<usize> = Vec::new();
        let mut remaining: Vec<usize> = (0..documents.len()).collect();

        while selected_indices.len() < top_k && !remaining.is_empty() {
            let mut best_score = f32::NEG_INFINITY;
            let mut best_idx = 0;

            for &idx in &remaining {
                // Relevance to query
                let relevance = self.cosine_similarity(query_embedding, &doc_embeddings[idx]);

                // Max similarity to already selected
                let mut max_sim: f32 = 0.0;
                for &sel_idx in &selected_indices {
                    let sim = self.cosine_similarity(&doc_embeddings[sel_idx], &doc_embeddings[idx]);
                    max_sim = max_sim.max(sim);
                }

                // MMR score
                let mmr = lambda * relevance - (1.0 - lambda) * max_sim;

                if mmr > best_score {
                    best_score = mmr;
                    best_idx = idx;
                }
            }

            selected_indices.push(best_idx);
            remaining.retain(|&x| x != best_idx);
        }

        selected_indices.into_iter()
            .map(|idx| RerankedDocument {
                index: idx,
                id: documents[idx].id.clone(),
                text: documents[idx].text.clone(),
                relevance_score: self.cosine_similarity(query_embedding, &doc_embeddings[idx]),
                original_score: documents[idx].original_score,
                score_delta: None,
                metadata: documents[idx].metadata.clone(),
            })
            .collect()
    }

    /// Cosine similarity
    fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f32 {
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
}

impl Default for DiversityReranker {
    fn default() -> Self {
        Self::new(DiversityConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diversity_reranker() {
        let config = DiversityConfig {
            diversity_factor: 0.5,
            similarity_threshold: 0.9,
            ..Default::default()
        };
        let reranker = DiversityReranker::new(config);
        
        let results = vec![
            RerankedDocument {
                index: 0,
                id: None,
                text: "hello world".into(),
                relevance_score: 0.9,
                original_score: None,
                score_delta: None,
                metadata: Default::default(),
            },
            RerankedDocument {
                index: 1,
                id: None,
                text: "hello world similar".into(),
                relevance_score: 0.85,
                original_score: None,
                score_delta: None,
                metadata: Default::default(),
            },
        ];
        
        let reranked = reranker.rerank(&results);
        assert!(reranked.len() <= results.len());
    }
}
