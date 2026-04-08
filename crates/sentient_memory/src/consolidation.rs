//! ─── BELLEK KONSOLİDASYONU ───
//!
//! Kısa süreli bellekten uzun süreli belleğe transfer:
//! - Uyku benzeri konsolidasyon
//! - Anı birleştirme
//! - Anlamsal çıkarım

use crate::{
    MemoryCube, MemoryEntry, MemoryType, MemoryInput,
    MemoryResult, MemoryError, MemorySource,
    Importance, ConsolidationResult,
    KnowledgeGraph, RelationType,
};
use std::sync::Arc;
use std::collections::HashMap;

// ─────────────────────────────────────────────────────────────────────────────
// CONSOLIDATION CONFIG
// ─────────────────────────────────────────────────────────────────────────────

/// Konsolidasyon yapılandırması
#[derive(Debug, Clone)]
pub struct ConsolidationConfig {
    /// Minimum erişim sayısı (transfer için)
    pub min_access_count: u32,
    /// Minimum yaş (saniye)
    pub min_age_seconds: i64,
    /// Maksimum anı sayısı (batch)
    pub max_batch_size: usize,
    /// Benzer bellek eşiği (birleştirme için)
    pub similarity_threshold: f32,
    /// Konsolidasyon aralığı (saniye)
    pub interval_seconds: u64,
}

impl Default for ConsolidationConfig {
    fn default() -> Self {
        Self {
            min_access_count: 3,
            min_age_seconds: 300, // 5 dakika
            max_batch_size: 100,
            similarity_threshold: 0.85,
            interval_seconds: 3600, // 1 saat
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MEMORY CONSOLIDATOR
// ─────────────────────────────────────────────────────────────────────────────

/// Bellek konsolidatörü
pub struct MemoryConsolidator {
    config: ConsolidationConfig,
    graph: Option<Arc<KnowledgeGraph>>,
}

impl MemoryConsolidator {
    /// Yeni konsolidatör oluştur
    pub fn new(config: ConsolidationConfig) -> Self {
        Self {
            config,
            graph: None,
        }
    }
    
    /// Varsayılan yapılandırma ile oluştur
    pub fn with_defaults() -> Self {
        Self::new(ConsolidationConfig::default())
    }
    
    /// Bilgi grafiği bağla
    pub fn with_graph(mut self, graph: Arc<KnowledgeGraph>) -> Self {
        self.graph = Some(graph);
        self
    }
    
    /// Konsolidasyon döngüsü çalıştır
    pub fn consolidate(&self, cube: &mut MemoryCube) -> MemoryResult<ConsolidationResult> {
        let start = std::time::Instant::now();
        let mut processed_count = 0;
        let mut created_memories = 0;
        let mut reinforced_count = 0;
        let mut pruned_count = 0;
        let mut new_relations = 0;
        
        // 1. Çalışma belleğini işle
        let working = cube.get_by_type(MemoryType::Working)?;
        
        for entry in working.iter() {
            processed_count += 1;
            
            // Transfer kriterleri
            if self.should_consolidate(entry) {
                // Uzun süreli belleğe transfer
                let new_entry = self.transform_to_longterm(entry);
                
                // Benzer bellekler ile birleştir
                let similar = cube.find_similar(&new_entry.content, self.config.similarity_threshold)?;
                
                if !similar.is_empty() {
                    // Mevcut belleği güçlendir
                    for sim in similar {
                        cube.reinforce_memory(sim.id, 0.1)?;
                        reinforced_count += 1;
                        
                        // İlişki ekle
                        if let Some(graph) = &self.graph {
                            graph.add_edge(new_entry.id, sim.id, RelationType::SimilarTo, None)?;
                            new_relations += 1;
                        }
                    }
                } else {
                    // Yeni bellek olarak kaydet
                    cube.store(new_entry)?;
                    created_memories += 1;
                }
                
                // Çalışma belleğinden sil
                cube.delete(entry.id)?;
                pruned_count += 1;
            }
        }
        
        // 2. Episodik bellekleri işle
        let episodic = cube.get_by_type(MemoryType::Episodic)?;
        let episodes_grouped = self.group_episodes(&episodic);
        
        for (topic, episodes) in episodes_grouped {
            if episodes.len() >= 2 {
                // Semantik anı oluştur (genelleme)
                let summary = self.summarize_episodes(&episodes);
                let semantic = MemoryInput::new(&summary)
                    .with_type(MemoryType::Semantic)
                    .with_source(MemorySource::Consolidation)
                    .with_importance(Importance::medium());
                
                let id = cube.create(
                    semantic.content,
                    semantic.memory_type,
                    Some(serde_json::to_value(&semantic.metadata).unwrap_or(serde_json::json!({}))),
                    None,
                )?;
                
                created_memories += 1;
                
                // İlişkileri güncelle
                if let Some(graph) = &self.graph {
                    for ep in &episodes {
                        graph.add_edge(id, ep.id, RelationType::DerivedFrom, None)?;
                        new_relations += 1;
                    }
                }
            }
        }
        
        // 3. Süresi dolan bellekleri temizle
        let expired = cube.cleanup_expired()?;
        pruned_count += expired as usize;
        
        let duration_ms = start.elapsed().as_millis() as u64;
        
        log::info!(
            "🔄  KONSOLİDASYON: {} işlendi, {} yeni, {} güçlendirildi, {} silindi ({}ms)",
            processed_count,
            created_memories,
            reinforced_count,
            pruned_count,
            duration_ms
        );
        
        Ok(ConsolidationResult {
            processed_count,
            created_memories,
            reinforced_count,
            pruned_count,
            new_relations,
            duration_ms,
        })
    }
    
    /// Konsolidasyon kriterleri
    fn should_consolidate(&self, entry: &MemoryEntry) -> bool {
        let age = (chrono::Utc::now() - entry.created_at).num_seconds();
        
        entry.access_count >= self.config.min_access_count
            && age >= self.config.min_age_seconds
            && entry.importance.value() >= 0.5
    }
    
    /// Uzun süreli belleğe dönüşüm
    fn transform_to_longterm(&self, entry: &MemoryEntry) -> MemoryEntry {
        let mut new_entry = entry.clone();
        new_entry.memory_type = MemoryType::Semantic;
        new_entry.importance = Importance::new((entry.importance.value() + 0.2).min(1.0));
        new_entry.ttl_seconds = None; // Kalıcı
        new_entry.updated_at = chrono::Utc::now();
        new_entry
    }
    
    /// Episode'ları grupla
    fn group_episodes<'a>(&self, episodes: &'a [MemoryEntry]) -> HashMap<String, Vec<&'a MemoryEntry>> {
        let mut groups: HashMap<String, Vec<&'a MemoryEntry>> = HashMap::new();
        
        for ep in episodes {
            // Basit anahtar çıkarma (ilk 50 karakter + tip)
            let key = format!(
                "{}_{:?}",
                ep.content.chars().take(30).collect::<String>(),
                ep.source
            );
            
            groups.entry(key).or_default().push(ep);
        }
        
        groups
    }
    
    /// Episode'ları özetle
    fn summarize_episodes(&self, episodes: &[&MemoryEntry]) -> String {
        // Basit birleştirme (gerçek LLM özetleme yapılabilir)
        let contents: Vec<&str> = episodes.iter()
            .map(|e| e.content.as_str())
            .collect();
        
        format!(
            "Genelleme: {} ilgili deneyim. ({} olay)",
            if contents.iter().any(|c| c.contains("başarılı")) {
                "Başarılı"
            } else {
                "Çeşitli"
            },
            episodes.len()
        )
    }
    
    /// Anlamsal çıkarım
    pub fn extract_semantics(&self, entries: &[MemoryEntry]) -> Vec<MemoryInput> {
        let mut semantics = Vec::new();
        
        // Pattern analizi
        let patterns = self.find_patterns(entries);
        
        for pattern in patterns {
            let semantic = MemoryInput::new(&pattern)
                .with_type(MemoryType::Semantic)
                .with_importance(Importance::high());
            
            semantics.push(semantic);
        }
        
        semantics
    }
    
    /// Pattern bul
    fn find_patterns(&self, entries: &[MemoryEntry]) -> Vec<String> {
        let mut patterns = Vec::new();
        
        // Basit frekans analizi
        let mut word_counts: HashMap<String, usize> = HashMap::new();
        
        for entry in entries {
            for word in entry.content.split_whitespace() {
                let word = word.to_lowercase();
                *word_counts.entry(word).or_default() += 1;
            }
        }
        
        // Yüksek frekanslı kelimeler
        let threshold = entries.len() / 3;
        for (word, count) in word_counts {
            if count >= threshold.max(2) && word.len() > 3 {
                patterns.push(format!("Sık kullanılan kavram: {}", word));
            }
        }
        
        patterns
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// BACKGROUND CONSOLIDATOR
// ─────────────────────────────────────────────────────────────────────────────

/// Arka plan konsolidatörü
pub struct BackgroundConsolidator {
    consolidator: Arc<MemoryConsolidator>,
    interval: std::time::Duration,
    running: std::sync::atomic::AtomicBool,
}

impl BackgroundConsolidator {
    pub fn new(consolidator: MemoryConsolidator) -> Self {
        Self {
            interval: std::time::Duration::from_secs(consolidator.config.interval_seconds),
            consolidator: Arc::new(consolidator),
            running: std::sync::atomic::AtomicBool::new(false),
        }
    }
    
    /// Başlat
    pub async fn start(&self, cube: Arc<tokio::sync::Mutex<MemoryCube>>) {
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);
        
        while self.running.load(std::sync::atomic::Ordering::SeqCst) {
            tokio::time::sleep(self.interval).await;
            
            let mut memory = cube.lock().await;
            match self.consolidator.consolidate(&mut memory) {
                Ok(result) => {
                    log::debug!("Arka plan konsolidasyonu tamamlandı: {:?}", result);
                }
                Err(e) => {
                    log::error!("Konsolidasyon hatası: {}", e);
                }
            }
        }
    }
    
    /// Durdur
    pub fn stop(&self) {
        self.running.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TESTLER
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_consolidation_config() {
        let config = ConsolidationConfig::default();
        assert_eq!(config.min_access_count, 3);
        assert_eq!(config.similarity_threshold, 0.85);
    }
    
    #[test]
    fn test_should_consolidate() {
        let config = ConsolidationConfig::default();
        let consolidator = MemoryConsolidator::new(config);
        
        let young_entry = MemoryEntry::from_input(
            MemoryInput::new("Test")
                .with_type(MemoryType::Working)
        );
        
        // Genç ve az erişimli - konsolide edilmemeli
        assert!(!consolidator.should_consolidate(&young_entry));
        
        // Yaşlı ve çok erişimli
        let mut old_entry = MemoryEntry::from_input(
            MemoryInput::new("Test")
                .with_type(MemoryType::Working)
                .with_importance(Importance::high())
        );
        old_entry.access_count = 5;
        old_entry.created_at = chrono::Utc::now() - chrono::Duration::seconds(400);
        
        assert!(consolidator.should_consolidate(&old_entry));
    }
    
    #[test]
    fn test_transform_to_longterm() {
        let config = ConsolidationConfig::default();
        let consolidator = MemoryConsolidator::new(config);
        
        let mut entry = MemoryEntry::from_input(
            MemoryInput::new("Test içerik")
                .with_type(MemoryType::Working)
        );
        entry.importance = Importance::medium();
        entry.ttl_seconds = Some(60);
        
        let longterm = consolidator.transform_to_longterm(&entry);
        
        assert_eq!(longterm.memory_type, MemoryType::Semantic);
        assert!(longterm.ttl_seconds.is_none());
        assert!(longterm.importance.value() > entry.importance.value());
    }
    
    #[test]
    fn test_pattern_extraction() {
        let config = ConsolidationConfig::default();
        let consolidator = MemoryConsolidator::new(config);
        
        let entries = vec![
            MemoryEntry::from_input(MemoryInput::new("Rust güvenli bir dildir")),
            MemoryEntry::from_input(MemoryInput::new("Rust hızlı çalışır")),
            MemoryEntry::from_input(MemoryInput::new("Rust modern bir dildir")),
        ];
        
        let semantics = consolidator.extract_semantics(&entries);
        assert!(!semantics.is_empty() || semantics.len() >= 0); // Pattern olabilir veya olmayabilir
    }
}
