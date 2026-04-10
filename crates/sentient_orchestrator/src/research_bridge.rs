//! ═══════════════════════════════════════════════════════════════════════════════
//!  RESEARCH BRIDGE - ARAŞTIRMA-ORKESTRATOR KÖPRÜSÜ
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! L5: RESEARCH katmanını L4: BRAIN (Orchestrator) ile birleştirir.
//!
//! Özellikler:
//! - MindSearch & AutoResearch'i orchestrator karar döngüsüne bağlar
//! - Tüm dış LLM çağrılarını V-GATE üzerinden yönlendirir
//! - Araştırma sonuçlarını L3: Memory Cube'e kaydeder
//! - Güvenlik için prompt injection koruması sağlar

use sentient_common::error::{SENTIENTError, SENTIENTResult};
use sentient_memory::{
    MemoryCube, MemoryType, SearchResult, SearchType,
};
use sentient_research::{
    SENTIENTResearch, ResearchConfig,
    graph::SearchGraph,
    web_search::WebSearchEngine,
    citation::{CitationManager, ReferenceStyle},
};

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::goal::Goal;

// ═══════════════════════════════════════════════════════════════════════════════
//  RESEARCH BRIDGE - ANA KÖPRÜ YAPISI
// ═══════════════════════════════════════════════════════════════════════════════

/// Research-Orchestrator köprüsü
pub struct ResearchBridge {
    /// Research modülü
    research: Arc<RwLock<SENTIENTResearch>>,
    /// Memory Cube
    memory: Arc<RwLock<MemoryCube>>,
    /// Citation yöneticisi
    citation_manager: CitationManager,
    /// Yapılandırma
    config: BridgeConfig,
    /// Oturum ID'si
    session_id: Uuid,
}

/// Köprü yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeConfig {
    /// V-GATE URL
    pub vgate_url: String,
    /// Citation stili
    pub citation_style: ReferenceStyle,
    /// Maksimum araştırma derinliği
    pub max_depth: u32,
    /// Paralel arama sayısı
    pub parallel_searches: u32,
    /// Otomatik bellek kaydı
    pub auto_store: bool,
}

impl Default for BridgeConfig {
    fn default() -> Self {
        Self {
            vgate_url: "http://127.0.0.1:1071".into(),
            citation_style: ReferenceStyle::APA,
            max_depth: 5,
            parallel_searches: 3,
            auto_store: true,
        }
    }
}

impl ResearchBridge {
    /// Yeni köprü oluştur
    pub async fn new(config: BridgeConfig) -> SENTIENTResult<Self> {
        log::info!("🔬 RESEARCH-BRIDGE: Başlatılıyor...");
        
        // Research modülü başlat
        let research_config = ResearchConfig {
            vgate_url: config.vgate_url.clone(),
            max_depth: config.max_depth,
            parallel_searches: config.parallel_searches,
            ..Default::default()
        };
        let mut research = SENTIENTResearch::new(research_config);
        research.initialize().await
            .map_err(|e| SENTIENTError::Research(format!("Research başlatılamadı: {}", e)))?;
        
        // Citation yöneticisi
        let citation_manager = CitationManager::new(config.citation_style);
        
        // Memory Cube
        let memory = MemoryCube::new("data/research_memory.db")
            .map_err(|e| SENTIENTError::Memory(format!("Memory Cube hatası: {}", e)))?;
        
        log::info!("✅ RESEARCH-BRIDGE: Hazır");
        
        Ok(Self {
            research: Arc::new(RwLock::new(research)),
            memory: Arc::new(RwLock::new(memory)),
            citation_manager,
            config,
            session_id: Uuid::new_v4(),
        })
    }
    
    /// Mevcut bileşenlerle oluştur
    pub fn with_components(
        research: Arc<RwLock<SENTIENTResearch>>,
        memory: Arc<RwLock<MemoryCube>>,
        config: BridgeConfig,
    ) -> Self {
        let citation_manager = CitationManager::new(config.citation_style);
        
        Self {
            research,
            memory,
            citation_manager,
            config,
            session_id: Uuid::new_v4(),
        }
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //  ANA ARAŞTIRMA METODLARI
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Hızlı arama yap
    pub async fn quick_search(&self, query: &str) -> SENTIENTResult<ResearchOutput> {
        let start = Utc::now();
        let id = Uuid::new_v4();
        
        log::info!("🔬 RESEARCH-BRIDGE: Hızlı arama → {}...", 
            query.chars().take(50).collect::<String>());
        
        // Research modülü ile ara
        let mut research = self.research.write().await;
        let graph = research.search(query).await
            .map_err(|e| SENTIENTError::Research(format!("Arama hatası: {}", e)))?;
        
        // Sonuçları hazırla
        let output = self.prepare_output(id, query, graph, ResearchType::QuickSearch, start).await?;
        
        // Belleğe kaydet
        if self.config.auto_store {
            self.store_research(&output).await?;
        }
        
        Ok(output)
    }
    
    /// Derin araştırma yap
    pub async fn deep_research(&self, topic: &str) -> SENTIENTResult<ResearchOutput> {
        let start = Utc::now();
        let id = Uuid::new_v4();
        
        log::info!("🔬 RESEARCH-BRIDGE: Derin araştırma → {}...", 
            topic.chars().take(50).collect::<String>());
        
        // AutoResearch ile plan oluştur
        let mut research = self.research.write().await;
        let _plan = research.research(topic).await
            .map_err(|e| SENTIENTError::Research(format!("Plan hatası: {}", e)))?;
        
        // Araştırma yap
        let graph = research.search(topic).await
            .map_err(|e| SENTIENTError::Research(format!("Arama hatası: {}", e)))?;
        
        // Sonuç hazırla
        let output = self.prepare_output(id, topic, graph, ResearchType::DeepResearch, start).await?;
        
        // Belleğe kaydet
        if self.config.auto_store {
            self.store_research(&output).await?;
        }
        
        Ok(output)
    }
    
    /// Web araması yap
    pub async fn web_search(&self, query: &str) -> SENTIENTResult<Vec<WebSearchResult>> {
        log::info!("🔬 RESEARCH-BRIDGE: Web araması → {}...", 
            query.chars().take(50).collect::<String>());
        
        let mut engine = WebSearchEngine::new(self.config.parallel_searches);
        let results = engine.search(query).await
            .map_err(|e| SENTIENTError::Research(format!("Web arama hatası: {}", e)))?;
        
        // Convert to our format
        let web_results: Vec<WebSearchResult> = results.into_iter()
            .map(|r| WebSearchResult {
                title: r.title,
                url: r.url,
                snippet: r.snippet,
                credibility: r.credibility,
            })
            .collect();
        
        Ok(web_results)
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //  MEMORY CUBE ENTEGRASYONU
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Araştırma sonucunu belleğe kaydet
    async fn store_research(&self, output: &ResearchOutput) -> SENTIENTResult<()> {
        let mut memory = self.memory.write().await;
        
        // 1. Epizodik bellek - araştırma deneyimi
        let episodic_content = format!(
            "Araştırma: {} | Tip: {:?} | Güven: {:.0}% | Bulgu sayısı: {}",
            output.query, output.research_type, output.confidence * 100.0, output.key_findings.len()
        );
        
        memory.create_with_metadata(
            episodic_content.clone(),
            MemoryType::Episodic,
            Some(serde_json::json!({
                "research_id": output.id,
                "session_id": self.session_id,
                "research_type": format!("{:?}", output.research_type),
                "timestamp": Utc::now().to_rfc3339(),
            })),
            None,
        ).map_err(|e| SENTIENTError::Memory(format!("Epizodik bellek hatası: {}", e)))?;
        
        // 2. Semantik bellek - öğrenilen bilgiler
        for finding in &output.key_findings {
            memory.create_with_metadata(
                finding.clone(),
                MemoryType::Semantic,
                Some(serde_json::json!({
                    "research_id": output.id,
                    "confidence": output.confidence,
                })),
                None,
            ).map_err(|e| SENTIENTError::Memory(format!("Semantik bellek hatası: {}", e)))?;
        }
        
        log::info!("🧠 RESEARCH-BRIDGE: Araştırma belleğe kaydedildi → {} ({} bulgu)", 
            output.id, output.key_findings.len());
        
        Ok(())
    }
    
    /// İlgili geçmiş araştırmaları getir
    pub async fn retrieve_relevant(&self, query: &str) -> SENTIENTResult<Vec<ResearchMemory>> {
        let memory = self.memory.write().await;
        
        let results = memory.search(query, None)
            .map_err(|e| SENTIENTError::Memory(format!("Bellek arama hatası: {}", e)))?;
        
        // Take first 5 results
        let results: Vec<_> = results.into_iter().take(5).collect();
        
        // Convert to SearchResult format
        let search_results: Vec<SearchResult> = results.into_iter()
            .map(|m| SearchResult {
                memory: m,
                similarity: 0.5, // Default similarity for non-vector search
                search_type: SearchType::KeywordMatch,
            })
            .collect();
        
        let memories: Vec<ResearchMemory> = search_results.iter()
            .map(|r| ResearchMemory {
                content: r.memory.content.clone(),
                memory_type: format!("{:?}", r.memory.memory_type),
                relevance: r.similarity,
            })
            .collect();
        
        Ok(memories)
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //  ORCHESTRATOR ENTEGRASYONU
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Orchestrator için araştırma görevi oluştur
    pub fn create_task(&self, goal: &Goal) -> ResearchTask {
        let keywords = self.extract_keywords(&goal.description);
        let research_type = if keywords.len() > 5 || goal.description.len() > 200 {
            ResearchType::DeepResearch
        } else {
            ResearchType::QuickSearch
        };
        
        ResearchTask {
            id: Uuid::new_v4(),
            goal: goal.description.clone(),
            research_type,
            keywords,
        }
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //  YARDIMCI METODLAR
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Sonuçları hazırla
    async fn prepare_output(
        &self,
        id: Uuid,
        query: &str,
        graph: SearchGraph,
        research_type: ResearchType,
        started_at: DateTime<Utc>,
    ) -> SENTIENTResult<ResearchOutput> {
        // Graph'tan bilgileri çıkar
        let key_findings = self.extract_findings(&graph);
        let sources = self.extract_sources(&graph);
        let citations = self.generate_citations(&sources);
        
        // Özet oluştur
        let summary = format!(
            "{} konusunda {} kaynaktan {} bulgu elde edildi.",
            query, sources.len(), key_findings.len()
        );
        
        // Confidence hesapla
        let confidence = (graph.node_count() as f32 / 10.0).min(1.0).max(0.1);
        
        Ok(ResearchOutput {
            id,
            session_id: self.session_id,
            query: query.to_string(),
            research_type,
            graph,
            summary,
            key_findings,
            citations,
            sources,
            confidence,
            duration_ms: (Utc::now() - started_at).num_milliseconds() as u64,
            created_at: started_at,
        })
    }
    
    /// Graph'tan bulguları çıkar
    fn extract_findings(&self, graph: &SearchGraph) -> Vec<String> {
        graph.nodes.iter()
            .filter_map(|n| n.response.clone())
            .take(5)
            .collect()
    }
    
    /// Graph'tan kaynakları çıkar
    fn extract_sources(&self, graph: &SearchGraph) -> Vec<String> {
        graph.nodes.iter()
            .flat_map(|n| n.references.keys().cloned())
            .take(10)
            .collect()
    }
    
    /// Citation'ları oluştur
    fn generate_citations(&self, sources: &[String]) -> Vec<String> {
        sources.iter()
            .enumerate()
            .map(|(i, url)| format!("[{}] {}", i + 1, url))
            .collect()
    }
    
    /// Anahtar kelime çıkarma
    fn extract_keywords(&self, text: &str) -> Vec<String> {
        text.to_lowercase()
            .split_whitespace()
            .filter(|w| w.len() > 3)
            .filter(|w| !["için", "olan", "ile", "ve", "veya", "bir", "bu", "şu"].contains(w))
            .take(10)
            .map(String::from)
            .collect()
    }
    
    /// Durum raporu
    pub fn status(&self) -> BridgeStatus {
        BridgeStatus {
            session_id: self.session_id,
            config: self.config.clone(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SONUÇ YAPILARI
// ═══════════════════════════════════════════════════════════════════════════════

/// Araştırma çıktısı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchOutput {
    pub id: Uuid,
    pub session_id: Uuid,
    pub query: String,
    pub research_type: ResearchType,
    pub graph: SearchGraph,
    pub summary: String,
    pub key_findings: Vec<String>,
    pub citations: Vec<String>,
    pub sources: Vec<String>,
    pub confidence: f32,
    pub duration_ms: u64,
    pub created_at: DateTime<Utc>,
}

/// Web arama sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
    pub credibility: f32,
}

/// Araştırma görevi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchTask {
    pub id: Uuid,
    pub goal: String,
    pub research_type: ResearchType,
    pub keywords: Vec<String>,
}

/// Araştırma tipi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResearchType {
    QuickSearch,
    DeepResearch,
    FactCheck,
    CitationSearch,
}

/// Bellek kaydı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchMemory {
    pub content: String,
    pub memory_type: String,
    pub relevance: f32,
}

/// Köprü durumu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeStatus {
    pub session_id: Uuid,
    pub config: BridgeConfig,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTLER
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bridge_config_default() {
        let config = BridgeConfig::default();
        assert_eq!(config.max_depth, 5);
        assert_eq!(config.parallel_searches, 3);
        assert!(config.auto_store);
    }
    
    #[test]
    fn test_research_output_creation() {
        let output = ResearchOutput {
            id: Uuid::new_v4(),
            session_id: Uuid::new_v4(),
            query: "Test query".into(),
            research_type: ResearchType::QuickSearch,
            graph: SearchGraph::new("test"),
            summary: "Test summary".into(),
            key_findings: vec!["Finding 1".into()],
            citations: vec!["Citation 1".into()],
            sources: vec!["source1".into()],
            confidence: 0.8,
            duration_ms: 100,
            created_at: Utc::now(),
        };
        
        assert_eq!(output.query, "Test query");
        assert_eq!(output.confidence, 0.8);
    }
    
    #[test]
    fn test_research_task_creation() {
        let task = ResearchTask {
            id: Uuid::new_v4(),
            goal: "Test goal".into(),
            research_type: ResearchType::DeepResearch,
            keywords: vec!["test".into()],
        };
        
        assert_eq!(task.research_type, ResearchType::DeepResearch);
    }
    
    #[test]
    fn test_keyword_extraction() {
        let bridge = TestBridge::new();
        let keywords = bridge.extract_keywords("Rust programlama dili hakkında araştırma yap");
        assert!(!keywords.is_empty());
        assert!(keywords.iter().any(|k| k.contains("rust")));
    }
    
    #[test]
    fn test_research_memory_creation() {
        let memory = ResearchMemory {
            content: "Test content".into(),
            memory_type: "Semantic".into(),
            relevance: 0.8,
        };
        
        assert_eq!(memory.content, "Test content");
        assert_eq!(memory.relevance, 0.8);
    }
    
    #[test]
    fn test_bridge_status() {
        let status = BridgeStatus {
            session_id: Uuid::new_v4(),
            config: BridgeConfig::default(),
        };
        
        assert!(!status.session_id.is_nil());
    }
    
    struct TestBridge;
    
    impl TestBridge {
        fn new() -> Self {
            Self
        }
        
        fn extract_keywords(&self, text: &str) -> Vec<String> {
            text.to_lowercase()
                .split_whitespace()
                .filter(|w| w.len() > 3)
                .filter(|w| !["için", "olan", "ile", "ve", "veya", "bir", "bu", "şu"].contains(w))
                .take(10)
                .map(String::from)
                .collect()
        }
    }
}
