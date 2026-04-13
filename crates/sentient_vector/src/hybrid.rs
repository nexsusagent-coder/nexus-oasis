//! ─── Hybrid Search Engine ───
//!
//! Combines vector similarity search with keyword/BM25 search.
//!
//! # Strategies
//! - **RRF**: Reciprocal Rank Fusion
//! - **Weighted**: Linear combination of scores
//! - **Geometric**: Geometric mean of scores
//! - **CombSUM**: Sum of normalized scores
//! - **CombMNZ**: Count-based fusion

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{SearchResult, VectorDocument, SearchFilter, SearchOptions};

// ═══════════════════════════════════════════════════════════════════════════════
//  HYBRID FUSION STRATEGIES
// ═══════════════════════════════════════════════════════════════════════════════

/// Hybrid search strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HybridStrategy {
    /// Reciprocal Rank Fusion
    Rrf { k: u32 },
    /// Weighted combination
    Weighted { vector_weight: f32, keyword_weight: f32 },
    /// Geometric mean of scores
    GeometricMean,
    /// Sum of normalized scores
    CombSum,
    /// Count-based fusion
    CombMnz { threshold: f32 },
}

impl Default for HybridStrategy {
    fn default() -> Self {
        Self::Rrf { k: 60 }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  HYBRID SEARCH ENGINE
// ═══════════════════════════════════════════════════════════════════════════════

/// Hybrid search engine
pub struct HybridSearchEngine {
    strategy: HybridStrategy,
}

impl HybridSearchEngine {
    /// Create new hybrid search engine
    pub fn new(strategy: HybridStrategy) -> Self {
        Self { strategy }
    }

    /// Fuse vector and keyword search results
    pub fn fuse(&self, vector_results: Vec<SearchResult>, keyword_results: Vec<SearchResult>) -> Vec<SearchResult> {
        match &self.strategy {
            HybridStrategy::Rrf { k } => self.rrf_fusion(vector_results, keyword_results, *k),
            HybridStrategy::Weighted { vector_weight, keyword_weight } => {
                self.weighted_fusion(vector_results, keyword_results, *vector_weight, *keyword_weight)
            }
            HybridStrategy::GeometricMean => self.geometric_fusion(vector_results, keyword_results),
            HybridStrategy::CombSum => self.combsum_fusion(vector_results, keyword_results),
            HybridStrategy::CombMnz { threshold } => self.combmnz_fusion(vector_results, keyword_results, *threshold),
        }
    }

    /// Reciprocal Rank Fusion
    fn rrf_fusion(&self, vector_results: Vec<SearchResult>, keyword_results: Vec<SearchResult>, k: u32) -> Vec<SearchResult> {
        let mut scores: HashMap<String, f32> = HashMap::new();
        let mut documents: HashMap<String, VectorDocument> = HashMap::new();

        // Add vector results
        for (rank, result) in vector_results.iter().enumerate() {
            let rrf_score = 1.0 / (k as f32 + (rank + 1) as f32);
            *scores.entry(result.document.id.clone()).or_insert(0.0) += rrf_score;
            documents.entry(result.document.id.clone()).or_insert_with(|| result.document.clone());
        }

        // Add keyword results
        for (rank, result) in keyword_results.iter().enumerate() {
            let rrf_score = 1.0 / (k as f32 + (rank + 1) as f32);
            *scores.entry(result.document.id.clone()).or_insert(0.0) += rrf_score;
            documents.entry(result.document.id.clone()).or_insert_with(|| result.document.clone());
        }

        // Sort by score
        let mut results: Vec<SearchResult> = scores
            .into_iter()
            .map(|(id, score)| SearchResult {
                document: documents.remove(&id).unwrap(),
                score,
            })
            .collect();

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results
    }

    /// Weighted fusion
    fn weighted_fusion(
        &self,
        vector_results: Vec<SearchResult>,
        keyword_results: Vec<SearchResult>,
        vector_weight: f32,
        keyword_weight: f32,
    ) -> Vec<SearchResult> {
        // Normalize scores
        let vector_results = self.normalize_scores(vector_results);
        let keyword_results = self.normalize_scores(keyword_results);

        let mut scores: HashMap<String, f32> = HashMap::new();
        let mut documents: HashMap<String, VectorDocument> = HashMap::new();

        // Add weighted vector results
        for result in vector_results {
            *scores.entry(result.document.id.clone()).or_insert(0.0) += result.score * vector_weight;
            documents.entry(result.document.id.clone()).or_insert_with(|| result.document.clone());
        }

        // Add weighted keyword results
        for result in keyword_results {
            *scores.entry(result.document.id.clone()).or_insert(0.0) += result.score * keyword_weight;
            documents.entry(result.document.id.clone()).or_insert_with(|| result.document.clone());
        }

        // Sort by score
        let mut results: Vec<SearchResult> = scores
            .into_iter()
            .map(|(id, score)| SearchResult {
                document: documents.remove(&id).unwrap(),
                score,
            })
            .collect();

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results
    }

    /// Geometric mean fusion
    fn geometric_fusion(&self, vector_results: Vec<SearchResult>, keyword_results: Vec<SearchResult>) -> Vec<SearchResult> {
        let vector_results = self.normalize_scores(vector_results);
        let keyword_results = self.normalize_scores(keyword_results);

        let mut scores: HashMap<String, (f32, f32)> = HashMap::new(); // (vector_score, keyword_score)
        let mut documents: HashMap<String, VectorDocument> = HashMap::new();

        // Collect vector scores
        for result in vector_results {
            scores.entry(result.document.id.clone()).or_insert((0.001, 0.001)).0 = result.score.max(0.001);
            documents.entry(result.document.id.clone()).or_insert_with(|| result.document.clone());
        }

        // Collect keyword scores
        for result in keyword_results {
            scores.entry(result.document.id.clone()).or_insert((0.001, 0.001)).1 = result.score.max(0.001);
            documents.entry(result.document.id.clone()).or_insert_with(|| result.document.clone());
        }

        // Calculate geometric mean
        let results: Vec<SearchResult> = scores
            .into_iter()
            .map(|(id, (vs, ks))| SearchResult {
                document: documents.remove(&id).unwrap(),
                score: (vs * ks).sqrt(),
            })
            .collect();

        let mut results = results;
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results
    }

    /// CombSUM fusion
    fn combsum_fusion(&self, vector_results: Vec<SearchResult>, keyword_results: Vec<SearchResult>) -> Vec<SearchResult> {
        let vector_results = self.normalize_scores(vector_results);
        let keyword_results = self.normalize_scores(keyword_results);

        let mut scores: HashMap<String, f32> = HashMap::new();
        let mut documents: HashMap<String, VectorDocument> = HashMap::new();

        for result in vector_results {
            *scores.entry(result.document.id.clone()).or_insert(0.0) += result.score;
            documents.entry(result.document.id.clone()).or_insert_with(|| result.document.clone());
        }

        for result in keyword_results {
            *scores.entry(result.document.id.clone()).or_insert(0.0) += result.score;
            documents.entry(result.document.id.clone()).or_insert_with(|| result.document.clone());
        }

        let mut results: Vec<SearchResult> = scores
            .into_iter()
            .map(|(id, score)| SearchResult {
                document: documents.remove(&id).unwrap(),
                score,
            })
            .collect();

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results
    }

    /// CombMNZ fusion
    fn combmnz_fusion(&self, vector_results: Vec<SearchResult>, keyword_results: Vec<SearchResult>, threshold: f32) -> Vec<SearchResult> {
        let vector_results = self.normalize_scores(vector_results);
        let keyword_results = self.normalize_scores(keyword_results);

        let mut scores: HashMap<String, f32> = HashMap::new();
        let mut counts: HashMap<String, usize> = HashMap::new();
        let mut documents: HashMap<String, VectorDocument> = HashMap::new();

        for result in vector_results {
            if result.score >= threshold {
                *scores.entry(result.document.id.clone()).or_insert(0.0) += result.score;
                *counts.entry(result.document.id.clone()).or_insert(0) += 1;
                documents.entry(result.document.id.clone()).or_insert_with(|| result.document.clone());
            }
        }

        for result in keyword_results {
            if result.score >= threshold {
                *scores.entry(result.document.id.clone()).or_insert(0.0) += result.score;
                *counts.entry(result.document.id.clone()).or_insert(0) += 1;
                documents.entry(result.document.id.clone()).or_insert_with(|| result.document.clone());
            }
        }

        let mut results: Vec<SearchResult> = scores
            .into_iter()
            .map(|(id, score)| SearchResult {
                document: documents.remove(&id).unwrap(),
                score: score * counts.get(&id).copied().unwrap_or(0) as f32,
            })
            .collect();

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results
    }

    /// Normalize scores to [0, 1]
    fn normalize_scores(&self, results: Vec<SearchResult>) -> Vec<SearchResult> {
        if results.is_empty() {
            return results;
        }

        let max_score = results.iter().map(|r| r.score).fold(0.0_f32, f32::max);
        if max_score == 0.0 {
            return results;
        }

        results
            .into_iter()
            .map(|mut r| {
                r.score /= max_score;
                r.document.score = Some(r.score);
                r
            })
            .collect()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  KEYWORD SEARCH
// ═══════════════════════════════════════════════════════════════════════════════

/// Simple BM25 keyword search
pub struct Bm25Search {
    documents: Vec<VectorDocument>,
    k1: f32,
    b: f32,
}

impl Bm25Search {
    /// Create new BM25 search engine
    pub fn new(documents: Vec<VectorDocument>) -> Self {
        Self {
            documents,
            k1: 1.5,
            b: 0.75,
        }
    }

    /// Set BM25 parameters
    pub fn with_params(mut self, k1: f32, b: f32) -> Self {
        self.k1 = k1;
        self.b = b;
        self
    }

    /// Search documents
    pub fn search(&self, query: &str, limit: usize) -> Vec<SearchResult> {
        let query_terms: Vec<String> = query
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        if query_terms.is_empty() || self.documents.is_empty() {
            return vec![];
        }

        // Calculate average document length
        let avg_dl: f32 = self.documents.iter()
            .map(|d| d.content.split_whitespace().count() as f32)
            .sum::<f32>() / self.documents.len() as f32;

        // Calculate IDF for each term
        let n = self.documents.len() as f32;
        let mut idf: HashMap<String, f32> = HashMap::new();

        for term in &query_terms {
            let df = self.documents.iter()
                .filter(|d| d.content.to_lowercase().contains(term))
                .count() as f32;
            let idf_val = ((n - df + 0.5) / (df + 0.5) + 1.0).ln();
            idf.insert(term.clone(), idf_val);
        }

        // Calculate BM25 scores
        let mut results: Vec<SearchResult> = self.documents.iter()
            .map(|doc| {
                let dl = doc.content.split_whitespace().count() as f32;
                let mut score = 0.0_f32;

                for term in &query_terms {
                    let tf = doc.content.to_lowercase()
                        .split_whitespace()
                        .filter(|w| w == term)
                        .count() as f32;

                    let idf_val = idf.get(term).copied().unwrap_or(0.0);
                    let numerator = tf * (self.k1 + 1.0);
                    let denominator = tf + self.k1 * (1.0 - self.b + self.b * dl / avg_dl);
                    score += idf_val * numerator / denominator;
                }

                SearchResult {
                    document: doc.clone(),
                    score,
                }
            })
            .filter(|r| r.score > 0.0)
            .collect();

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit);
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rrf_fusion() {
        let engine = HybridSearchEngine::new(HybridStrategy::Rrf { k: 60 });

        let vector_results = vec![
            SearchResult {
                document: VectorDocument::new("1", "doc 1", vec![]),
                score: 0.9,
            },
            SearchResult {
                document: VectorDocument::new("2", "doc 2", vec![]),
                score: 0.8,
            },
        ];

        let keyword_results = vec![
            SearchResult {
                document: VectorDocument::new("2", "doc 2", vec![]),
                score: 0.95,
            },
            SearchResult {
                document: VectorDocument::new("3", "doc 3", vec![]),
                score: 0.7,
            },
        ];

        let fused = engine.fuse(vector_results, keyword_results);
        assert!(!fused.is_empty());
        // Doc 2 should be ranked high as it appears in both lists
        assert_eq!(fused[0].document.id, "2");
    }

    #[test]
    fn test_bm25_search() {
        let docs = vec![
            VectorDocument::new("1", "The quick brown fox jumps over the lazy dog", vec![]),
            VectorDocument::new("2", "A lazy cat sleeps all day", vec![]),
            VectorDocument::new("3", "The dog barks at the fox", vec![]),
        ];

        let bm25 = Bm25Search::new(docs);
        let results = bm25.search("lazy dog", 10);

        assert!(!results.is_empty());
        // Doc 1 and 2 should match
    }
}
