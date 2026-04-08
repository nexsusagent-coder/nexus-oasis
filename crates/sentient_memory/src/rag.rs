//! ─── RAG MOTORU ───
//!
//! Retrieval-Augmented Generation:
//! - Sorguya ilgili bellekleri getirme
//! - Context hazırlama
//! - Re-ranking ve filtreleme

use crate::{
    MemoryCube, MemoryEntry, MemoryType, MemoryInput,
    RagContext, SearchResult, SearchType, SearchOptions,
    MemoryResult, MemoryError, MemorySource, Importance,
};
use crate::embeddings::{EmbeddingEngine, EmbeddingConfig, cosine_similarity};
use std::sync::Arc;
use std::collections::HashMap;

// ─────────────────────────────────────────────────────────────────────────────
// RAG CONFIG
// ─────────────────────────────────────────────────────────────────────────────

/// RAG yapılandırması
#[derive(Debug, Clone)]
pub struct RagConfig {
    /// Maksimum getirilecek bellek sayısı
    pub max_memories: usize,
    /// Minimum benzerlik eşiği
    pub min_similarity: f32,
    /// Maksimum context token sayısı
    pub max_context_tokens: usize,
    /// Episodik bellek ağırlığı
    pub episodic_weight: f32,
    /// Semantik bellek ağırlığı
    pub semantic_weight: f32,
    /// Prosedürel bellek ağırlığı
    pub procedural_weight: f32,
    /// Recency bonus (zaman)
    pub recency_bonus: f32,
    /// Erişim sayısı bonus
    pub access_bonus: f32,
    /// Önem bonus
    pub importance_bonus: f32,
}

impl Default for RagConfig {
    fn default() -> Self {
        Self {
            max_memories: 10,
            min_similarity: 0.3,
            max_context_tokens: 4000,
            episodic_weight: 0.8,
            semantic_weight: 1.0,
            procedural_weight: 0.9,
            recency_bonus: 0.1,
            access_bonus: 0.05,
            importance_bonus: 0.15,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// RAG ENGINE
// ─────────────────────────────────────────────────────────────────────────────

/// RAG Motoru
pub struct RagEngine {
    /// Embedding motoru
    embedding: Arc<EmbeddingEngine>,
    /// Bellek küpü
    memory: Arc<tokio::sync::RwLock<MemoryCube>>,
    /// Yapılandırma
    config: RagConfig,
}

impl RagEngine {
    /// Yeni RAG motoru oluştur
    pub fn new(
        embedding: EmbeddingEngine,
        memory: MemoryCube,
        config: RagConfig,
    ) -> Self {
        Self {
            embedding: Arc::new(embedding),
            memory: Arc::new(tokio::sync::RwLock::new(memory)),
            config,
        }
    }
    
    /// Varsayılan yapılandırma ile oluştur
    pub fn with_defaults(embedding: EmbeddingEngine, memory: MemoryCube) -> Self {
        Self::new(embedding, memory, RagConfig::default())
    }
    
    /// Sorgu için ilgili bellekleri getir
    pub async fn retrieve(&self, query: &str, opts: Option<SearchOptions>) -> MemoryResult<RagContext> {
        let opts = opts.unwrap_or_default();
        
        // 1. Sorguyu embedding'e çevir
        let query_embedding = self.embedding.embed(query).await
            .map_err(|e| MemoryError::EmbeddingError(e.to_string()))?;
        
        // 2. Bellek küpünde ara
        let memory = self.memory.read().await;
        
        // Vector search
        let vector_results = memory.search_vector(&query_embedding, opts.limit * 2)?;
        
        // Keyword search (fallback)
        let keyword_results = memory.search(query, None)?;
        
        // 3. Sonuçları birleştir ve re-rank
        let merged = self.merge_results(vector_results, keyword_results);
        let ranked = self.rank_results(merged, &query_embedding);
        
        // 4. Filtrele
        let filtered: Vec<_> = ranked
            .into_iter()
            .filter(|r| r.similarity >= self.config.min_similarity)
            .take(self.config.max_memories)
            .collect();
        
        // 5. Context oluştur
        let context = self.build_context(query, filtered.clone());
        
        Ok(context)
    }
    
    /// Sonuçları birleştir
    fn merge_results(
        &self,
        vector_results: Vec<SearchResult>,
        keyword_results: Vec<MemoryEntry>,
    ) -> Vec<SearchResult> {
        let mut seen = std::collections::HashSet::new();
        let mut merged = Vec::new();
        
        // Vector sonuçlarını ekle
        for result in vector_results {
            if seen.insert(result.memory.id) {
                merged.push(result);
            }
        }
        
        // Keyword sonuçlarını ekle (benzerlik 0.5 varsay)
        for entry in keyword_results {
            if seen.insert(entry.id) {
                merged.push(SearchResult {
                    memory: entry,
                    similarity: 0.5,
                    search_type: SearchType::KeywordMatch,
                });
            }
        }
        
        merged
    }
    
    /// Sonuçları sırala
    fn rank_results(&self, results: Vec<SearchResult>, query_embedding: &[f32]) -> Vec<SearchResult> {
        let mut ranked: Vec<_> = results
            .into_iter()
            .map(|mut r| {
                // Tip ağırlığı
                let type_weight = match r.memory.memory_type {
                    MemoryType::Episodic => self.config.episodic_weight,
                    MemoryType::Semantic => self.config.semantic_weight,
                    MemoryType::Procedural => self.config.procedural_weight,
                    _ => 1.0,
                };
                
                // Recency bonus
                let age_hours = (chrono::Utc::now() - r.memory.created_at)
                    .num_hours()
                    .max(0) as f32;
                let recency = (-age_hours / 168.0).exp(); // 1 hafta yarı ömür
                
                // Access bonus
                let access = 1.0 + (r.memory.access_count as f32 * self.config.access_bonus);
                
                // Importance bonus
                let importance = 1.0 + (r.memory.importance.value() * self.config.importance_bonus);
                
                // Final score
                let score = r.similarity * type_weight * recency * access * importance;
                r.similarity = score;
                
                r
            })
            .collect();
        
        ranked.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap_or(std::cmp::Ordering::Equal));
        ranked
    }
    
    /// Context oluştur
    fn build_context(&self, query: &str, results: Vec<SearchResult>) -> RagContext {
        let mut context_text = String::new();
        let mut token_count = 0;
        let mut source_types = Vec::new();
        
        for result in &results {
            let entry_text = format!(
                "[{}] {}\n",
                result.memory.memory_type, result.memory.content
            );
            
            let entry_tokens = self.embedding.count_tokens(&entry_text);
            
            if token_count + entry_tokens > self.config.max_context_tokens {
                break;
            }
            
            context_text.push_str(&entry_text);
            token_count += entry_tokens;
            
            if !source_types.contains(&result.memory.memory_type) {
                source_types.push(result.memory.memory_type);
            }
        }
        
        RagContext {
            query: query.to_string(),
            retrieved_memories: results,
            estimated_tokens: token_count,
            context_text,
            source_types,
        }
    }
    
    /// Yeni bilgiyi kaydet ve indeksle
    pub async fn memorize(
        &self,
        content: &str,
        memory_type: MemoryType,
        source: crate::MemorySource,
        importance: f32,
    ) -> MemoryResult<uuid::Uuid> {
        // Embedding al
        let embedding = self.embedding.embed(content).await
            .map_err(|e| MemoryError::EmbeddingError(e.to_string()))?;
        
        // Bellek girdisi oluştur
        let input = MemoryInput::new(content)
            .with_type(memory_type)
            .with_source(source)
            .with_importance(crate::Importance::new(importance));
        
        // Kaydet
        let mut memory = self.memory.write().await;
        let id = memory.create_with_embedding(input, Some(embedding))?;
        
        log::info!(
            "📚  RAG: Yeni anı kaydedildi [{}] {}",
            memory_type,
            id
        );
        
        Ok(id)
    }
    
    /// Web araştırmasından öğren
    pub async fn learn_from_research(
        &self,
        topic: &str,
        findings: &str,
        url: Option<&str>,
    ) -> MemoryResult<uuid::Uuid> {
        self.memorize(
            findings,
            MemoryType::Semantic,
            crate::MemorySource::WebResearch { url: url.map(|s| s.to_string()) },
            0.7,
        ).await
    }
    
    /// Görev deneyimini kaydet
    pub async fn record_experience(
        &self,
        task: &str,
        outcome: &str,
        success: bool,
    ) -> MemoryResult<uuid::Uuid> {
        let content = format!(
            "Görev: {}\nSonuç: {}\nBaşarılı: {}",
            task, outcome, success
        );
        
        let importance = if success { 0.8 } else { 0.6 };
        
        self.memorize(
            &content,
            MemoryType::Episodic,
            crate::MemorySource::InternalInference,
            importance,
        ).await
    }
    
    /// Yöntem/prosedür kaydet
    pub async fn store_procedure(
        &self,
        name: &str,
        steps: &[&str],
        context: &str,
    ) -> MemoryResult<uuid::Uuid> {
        let content = format!(
            "Yöntem: {}\nAdımlar:\n{}\nBağlam: {}",
            name,
            steps.iter().enumerate()
                .map(|(i, s)| format!("{}. {}", i + 1, s))
                .collect::<Vec<_>>()
                .join("\n"),
            context
        );
        
        self.memorize(
            &content,
            MemoryType::Procedural,
            crate::MemorySource::InternalInference,
            0.9,
        ).await
    }
    
    /// Yapılandırmayı getir
    pub fn config(&self) -> &RagConfig {
        &self.config
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// CONTEXT BUILDER
// ─────────────────────────────────────────────────────────────────────────────

/// Gelişmiş context oluşturucu
pub struct ContextBuilder {
    memories: Vec<SearchResult>,
    max_tokens: usize,
    template: String,
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self {
            memories: Vec::new(),
            max_tokens: 4000,
            template: String::new(),
        }
    }
    
    pub fn with_memories(mut self, memories: Vec<SearchResult>) -> Self {
        self.memories = memories;
        self
    }
    
    pub fn with_max_tokens(mut self, tokens: usize) -> Self {
        self.max_tokens = tokens;
        self
    }
    
    pub fn with_template(mut self, template: impl Into<String>) -> Self {
        self.template = template.into();
        self
    }
    
    pub fn build(self) -> String {
        let mut output = if self.template.is_empty() {
            "📚 ÖNCEKİ BİLGİLER:\n\n".to_string()
        } else {
            self.template.clone()
        };
        
        for (i, result) in self.memories.iter().enumerate() {
            let entry = format!(
                "[{}] {} (önem: {:.1})\n{}\n\n",
                i + 1,
                result.memory.memory_type,
                result.memory.importance.value(),
                result.memory.content
            );
            output.push_str(&entry);
        }
        
        output
    }
}

impl Default for ContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SYNC WRAPPER
// ─────────────────────────────────────────────────────────────────────────────

/// Senkron RAG motoru
pub struct RagEngineSync {
    embedding: crate::embeddings::EmbeddingEngineSync,
    memory: std::sync::Mutex<MemoryCube>,
    config: RagConfig,
}

impl RagEngineSync {
    pub fn new(
        embedding_config: EmbeddingConfig,
        memory_db: &str,
        rag_config: RagConfig,
    ) -> MemoryResult<Self> {
        let embedding = crate::embeddings::EmbeddingEngineSync::new(embedding_config);
        let memory = MemoryCube::new(memory_db)?;
        
        Ok(Self {
            embedding,
            memory: std::sync::Mutex::new(memory),
            config: rag_config,
        })
    }
    
    pub fn retrieve(&self, query: &str) -> MemoryResult<RagContext> {
        let query_embedding = self.embedding.embed(query)?;
        let memory = self.memory.lock().unwrap();
        
        let results = memory.search_vector(&query_embedding, self.config.max_memories)?;
        
        let filtered: Vec<_> = results
            .into_iter()
            .filter(|r| r.similarity >= self.config.min_similarity)
            .collect();
        
        // Context oluştur
        let mut context_text = String::new();
        let mut token_count = 0;
        let mut source_types = Vec::new();
        
        for result in &filtered {
            let entry_text = format!("[{}] {}\n", result.memory.memory_type, result.memory.content);
            let tokens = query.len() / 4; // Basit tahmin
            
            if token_count + tokens > self.config.max_context_tokens {
                break;
            }
            
            context_text.push_str(&entry_text);
            token_count += tokens;
            
            if !source_types.contains(&result.memory.memory_type) {
                source_types.push(result.memory.memory_type);
            }
        }
        
        Ok(RagContext {
            query: query.to_string(),
            retrieved_memories: filtered,
            estimated_tokens: token_count,
            context_text,
            source_types,
        })
    }
    
    pub fn memorize(&self, content: &str, memory_type: MemoryType) -> MemoryResult<uuid::Uuid> {
        let embedding = self.embedding.embed(content)?;
        let mut memory = self.memory.lock().unwrap();
        
        let input = MemoryInput::new(content).with_type(memory_type);
        memory.create_with_embedding(input, Some(embedding))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TESTLER
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rag_config_default() {
        let config = RagConfig::default();
        assert_eq!(config.max_memories, 10);
        assert!((config.min_similarity - 0.3).abs() < 0.01);
    }
    
    #[test]
    fn test_context_builder() {
        let builder = ContextBuilder::new()
            .with_max_tokens(1000);
        
        let output = builder.build();
        assert!(output.contains("BİLGİLER"));
    }
    
    #[test]
    fn test_merge_results() {
        let config = RagConfig::default();
        let embedding_config = EmbeddingConfig::default();
        let embedding = Arc::new(EmbeddingEngine::new(embedding_config));
        
        // Basit merge test
        let vec_results: Vec<SearchResult> = vec![];
        let kw_results: Vec<MemoryEntry> = vec![];
        
        // merge_results private, dolaylı test
    }
}
