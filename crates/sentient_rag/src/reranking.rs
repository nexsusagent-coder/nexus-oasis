// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Reranking
// ═══════════════════════════════════════════════════════════════════════════════

use crate::{Query, RetrievalResult, Result};

/// Reranked result
#[derive(Debug, Clone)]
pub struct RerankedResult {
    /// Original result
    pub result: RetrievalResult,
    /// Reranked score
    pub reranked_score: f32,
    /// Original rank
    pub original_rank: usize,
    /// New rank
    pub new_rank: usize,
}

/// Reranker
pub struct Reranker {
    /// Use cross-encoder
    use_cross_encoder: bool,
    /// Diversity penalty
    diversity_penalty: f32,
}

impl Reranker {
    pub fn new() -> Self {
        Self {
            use_cross_encoder: false,
            diversity_penalty: 0.0,
        }
    }

    pub fn with_cross_encoder(mut self) -> Self {
        self.use_cross_encoder = true;
        self
    }

    pub fn with_diversity(mut self, penalty: f32) -> Self {
        self.diversity_penalty = penalty;
        self
    }

    /// Rerank results
    pub async fn rerank(&self, query: &Query, results: Vec<RetrievalResult>) -> Result<Vec<RerankedResult>> {
        if self.use_cross_encoder {
            self.cross_encoder_rerank(query, results).await
        } else {
            self.simple_rerank(query, results).await
        }
    }

    /// Simple reranking based on query-document similarity
    async fn simple_rerank(&self, query: &Query, results: Vec<RetrievalResult>) -> Result<Vec<RerankedResult>> {
        let query_lower = query.text.to_lowercase();
        let query_terms: std::collections::HashSet<&str> = query_lower
            .split_whitespace()
            .collect();

        let mut reranked: Vec<RerankedResult> = results
            .into_iter()
            .enumerate()
            .map(|(rank, result)| {
                let doc_lower = result.chunk.content.to_lowercase();
                let doc_terms: std::collections::HashSet<&str> = doc_lower
                    .split_whitespace()
                    .collect();

                let overlap = query_terms.intersection(&doc_terms).count() as f32;
                let coverage = overlap / query_terms.len().max(1) as f32;
                
                let reranked_score = (result.score + coverage) / 2.0;

                RerankedResult {
                    result,
                    reranked_score,
                    original_rank: rank,
                    new_rank: 0,
                }
            })
            .collect();

        // Apply diversity penalty
        if self.diversity_penalty > 0.0 {
            self.apply_diversity(&mut reranked);
        }

        // Sort by reranked score
        reranked.sort_by(|a, b| b.reranked_score.partial_cmp(&a.reranked_score).unwrap());

        // Assign new ranks
        for (i, result) in reranked.iter_mut().enumerate() {
            result.new_rank = i + 1;
        }

        Ok(reranked)
    }

    /// Cross-encoder reranking (placeholder)
    async fn cross_encoder_rerank(&self, query: &Query, results: Vec<RetrievalResult>) -> Result<Vec<RerankedResult>> {
        // Would use actual cross-encoder model
        self.simple_rerank(query, results).await
    }

    /// Apply diversity penalty to avoid similar results
    fn apply_diversity(&self, results: &mut [RerankedResult]) {
        for i in 1..results.len() {
            for j in 0..i {
                let similarity = self.text_similarity(
                    &results[i].result.chunk.content,
                    &results[j].result.chunk.content,
                );

                if similarity > 0.8 {
                    results[i].reranked_score -= self.diversity_penalty * similarity;
                }
            }
        }
    }

    /// Calculate text similarity (Jaccard)
    fn text_similarity(&self, text1: &str, text2: &str) -> f32 {
        let set1: std::collections::HashSet<&str> = text1.split_whitespace().collect();
        let set2: std::collections::HashSet<&str> = text2.split_whitespace().collect();

        let intersection = set1.intersection(&set2).count();
        let union = set1.union(&set2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f32 / union as f32
        }
    }
}

impl Default for Reranker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Chunk;

    #[test]
    fn test_reranker_creation() {
        let reranker = Reranker::new()
            .with_diversity(0.1);

        assert_eq!(reranker.diversity_penalty, 0.1);
    }

    #[tokio::test]
    async fn test_simple_rerank() {
        let reranker = Reranker::new();
        let query = Query::new("test query");

        let results = vec![
            RetrievalResult::new(Chunk::new("doc1", "test query matches", 0, 18), 0.7),
            RetrievalResult::new(Chunk::new("doc2", "no match", 0, 8), 0.3),
        ];

        let reranked = reranker.rerank(&query, results).await.unwrap();
        
        assert_eq!(reranked.len(), 2);
        assert!(reranked[0].reranked_score >= reranked[1].reranked_score);
    }
}
