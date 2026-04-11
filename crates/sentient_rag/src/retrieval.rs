// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Retrieval
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use crate::{Chunk, Query, Result, RAGError};

/// Search type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchType {
    /// Vector similarity search
    Vector,
    /// Keyword/BM25 search
    Keyword,
    /// Hybrid (vector + keyword)
    Hybrid,
}

/// Retrieval result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievalResult {
    /// Retrieved chunk
    pub chunk: Chunk,
    /// Similarity/relevance score
    pub score: f32,
}

impl RetrievalResult {
    pub fn new(chunk: Chunk, score: f32) -> Self {
        Self { chunk, score }
    }
}

/// Retriever
pub struct Retriever {
    /// Search type
    search_type: SearchType,
    /// Number of results
    top_k: usize,
    /// Score threshold
    score_threshold: f32,
}

impl Retriever {
    pub fn new() -> Self {
        Self {
            search_type: SearchType::Hybrid,
            top_k: 5,
            score_threshold: 0.5,
        }
    }

    pub fn with_search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = search_type;
        self
    }

    pub fn with_top_k(mut self, k: usize) -> Self {
        self.top_k = k;
        self
    }

    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.score_threshold = threshold;
        self
    }

    /// Retrieve relevant chunks
    pub async fn retrieve(&self, query: &Query, chunks: &[Chunk]) -> Result<Vec<RetrievalResult>> {
        match self.search_type {
            SearchType::Vector => self.vector_search(query, chunks).await,
            SearchType::Keyword => self.keyword_search(query, chunks).await,
            SearchType::Hybrid => self.hybrid_search(query, chunks).await,
        }
    }

    /// Vector similarity search
    async fn vector_search(&self, query: &Query, chunks: &[Chunk]) -> Result<Vec<RetrievalResult>> {
        // Placeholder - would use actual embeddings
        let mut results: Vec<RetrievalResult> = chunks
            .iter()
            .enumerate()
            .map(|(i, chunk)| {
                // Simulate similarity based on word overlap
                let query_words: std::collections::HashSet<&str> = query.text.split_whitespace().collect();
                let chunk_words: std::collections::HashSet<&str> = chunk.content.split_whitespace().collect();
                let overlap = query_words.intersection(&chunk_words).count();
                let score = overlap as f32 / query_words.len().max(1) as f32;
                
                RetrievalResult::new(chunk.clone(), score)
            })
            .filter(|r| r.score >= self.score_threshold)
            .collect();

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results.truncate(self.top_k);

        Ok(results)
    }

    /// Keyword/BM25 search
    async fn keyword_search(&self, query: &Query, chunks: &[Chunk]) -> Result<Vec<RetrievalResult>> {
        // Placeholder - would use BM25 or similar
        self.vector_search(query, chunks).await
    }

    /// Hybrid search (combines vector and keyword)
    async fn hybrid_search(&self, query: &Query, chunks: &[Chunk]) -> Result<Vec<RetrievalResult>> {
        let vector_results = self.vector_search(query, chunks).await?;
        let keyword_results = self.keyword_search(query, chunks).await?;

        // Combine and deduplicate
        let mut combined: std::collections::HashMap<String, RetrievalResult> = std::collections::HashMap::new();

        for result in vector_results {
            let key = format!("{}_{}", result.chunk.doc_id, result.chunk.start);
            combined.insert(key.clone(), result);
        }

        for result in keyword_results {
            let key = format!("{}_{}", result.chunk.doc_id, result.chunk.start);
            combined.entry(key)
                .and_modify(|existing| {
                    existing.score = (existing.score + result.score) / 2.0;
                })
                .or_insert(result);
        }

        let mut results: Vec<RetrievalResult> = combined.into_values().collect();
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results.truncate(self.top_k);

        Ok(results)
    }
}

impl Default for Retriever {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retriever_creation() {
        let retriever = Retriever::new()
            .with_top_k(10)
            .with_threshold(0.7);

        assert_eq!(retriever.top_k, 10);
    }

    #[test]
    fn test_retrieval_result() {
        let chunk = Chunk::new("doc1", "Test content", 0, 12);
        let result = RetrievalResult::new(chunk, 0.85);

        assert_eq!(result.score, 0.85);
    }

    #[tokio::test]
    async fn test_vector_search() {
        let retriever = Retriever::new()
            .with_search_type(SearchType::Vector);

        let query = Query::new("test query");
        let chunks = vec![
            Chunk::new("doc1", "test query matches", 0, 18),
            Chunk::new("doc2", "no match here", 0, 13),
        ];

        let results = retriever.retrieve(&query, &chunks).await.unwrap();
        assert!(!results.is_empty());
    }
}
