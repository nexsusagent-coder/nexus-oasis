//! ═══════════════════════════════════════════════════════════════════════════════
//!  ASENSA RESEARCH - L5: RESEARCH KATMANI
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! MindSearch ve AutoResearch araçlarının SENTIENT'ya tam asimilasyonu.
//! PyO3 tabanlı native Python köprüleri.
//!
//! ═──────────────────────────────────────────────────────────────────────────────
//!  L1 SOVEREIGN ANAYASASI:
//!  ───────────────────────

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
//!  ✓ Python modülleri native Rust modülü olarak sarılır (PyO3)
//!  ✓ Sıfır kopyalı veri akışı (zero-copy data flow)
//!  ✓ V-GATE üzerinden şifreli LLM iletişimi
//!  ✓ Tüm hatalar SENTIENT diline çevrilir
//!  ✓ Bellek güvenliği: unsafe blokları minimize edilir
//! ═──────────────────────────────────────────────────────────────────────────────
//!
//! MİMARİ:
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │                         ASENSA RESEARCH                                       │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │                                                                             │
//! │  ┌─────────────────────────────────────────────────────────────────────┐   │
//! │  │                    NATIVE LAYER                                       │   │
//! │  │  ┌───────────────┐  ┌───────────────┐  ┌───────────────┐           │   │
//! │  │  │ MindSearch    │  │ AutoResearch  │  │ WebSearch     │           │   │
//! │  │  │   Wrapper     │  │   Wrapper     │  │   Engine      │           │   │
//! │  │  └───────────────┘  └───────────────┘  └───────────────┘           │   │
//! │  └─────────────────────────────────────────────────────────────────────┘   │
//! │                                                                             │
//! │  ┌─────────────────────────────────────────────────────────────────────┐   │
//! │  │                    RUST CORE                                          │   │
//! │  │  ┌───────────────┐  ┌───────────────┐  ┌───────────────┐           │   │
//! │  │  │ ResearchTask  │  │ SearchGraph    │  │ Citation      │           │   │
//! │  │  │   Executor    │  │   Builder      │  │   Manager     │           │   │
//! │  │  └───────────────┘  └───────────────┘  └───────────────┘           │   │
//! │  └─────────────────────────────────────────────────────────────────────┘   │
//! │                                                                             │
//! │  ┌─────────────────────────────────────────────────────────────────────┐   │
//! │  │                    V-GATE (L2)                                       │   │
//! │  │  LLM Request → Guardrails → Encrypted → LLM → Response              │   │
//! │  └─────────────────────────────────────────────────────────────────────┘   │
//! │                                                                             │
//! └─────────────────────────────────────────────────────────────────────────────┘
//! ```

pub mod error;
pub mod mindsearch;
pub mod autoresearch;
pub mod web_search;
pub mod graph;
pub mod citation;
pub mod vgate;
pub mod memory_bridge;

// Re-exports
pub use error::{ResearchError, ResearchResult};
pub use mindsearch::{MindSearchWrapper, MindSearchConfig, SearchNode, NodeStatus};
pub use autoresearch::{AutoResearchWrapper, AutoResearchConfig, ResearchPlan, Subtopic, Source, PlanStatus, SourceType, OutputFormat};
pub use web_search::{WebSearchEngine, SearchQuery, SearchResult, TimeRange};
pub use graph::{SearchGraph, GraphNode, GraphEdge};
pub use citation::{CitationManager, Citation, ReferenceStyle};
pub use vgate::ResearchVGate;
pub use memory_bridge::ResearchMemoryBridge;

use sentient_common::error::SENTIENTResult;

/// SENTIENT Research sürümü
pub const ASENSA_RESEARCH_VERSION: &str = "0.1.0-sentient";

// ───────────────────────────────────────────────────────────────────────────────
//  ASENSA RESEARCH MANAGER
// ───────────────────────────────────────────────────────────────────────────────

/// SENTIENT Research yöneticisi - Ana giriş noktası
pub struct SENTIENTResearch {
    /// MindSearch wrapper
    mindsearch: Option<mindsearch::MindSearchWrapper>,
    /// AutoResearch wrapper
    autoresearch: Option<autoresearch::AutoResearchWrapper>,
    /// Web arama motoru
    web_search: web_search::WebSearchEngine,
    /// V-GATE köprüsü
    vgate: vgate::ResearchVGate,
    /// Bellek köprüsü
    memory: memory_bridge::ResearchMemoryBridge,
    /// Yapılandırma
    config: ResearchConfig,
    /// Başlatıldı mı?
    initialized: bool,
}

/// Research yapılandırması
#[derive(Debug, Clone)]
pub struct ResearchConfig {
    /// V-GATE URL
    pub vgate_url: String,
    /// Maksimum arama derinliği
    pub max_depth: u32,
    /// Maksimum node sayısı
    pub max_nodes: u32,
    /// Zaman aşımı (saniye)
    pub timeout_secs: u64,
    /// Citation stili
    pub citation_style: citation::ReferenceStyle,
    /// Paralel arama sayısı
    pub parallel_searches: u32,
}

impl Default for ResearchConfig {
    fn default() -> Self {
        Self {
            vgate_url: "http://127.0.0.1:1071".into(),
            max_depth: 5,
            max_nodes: 50,
            timeout_secs: 120,
            citation_style: citation::ReferenceStyle::APA,
            parallel_searches: 3,
        }
    }
}

impl SENTIENTResearch {
    /// Yeni SENTIENT Research oluştur
    pub fn new(config: ResearchConfig) -> Self {
        log::info!("🔬 ASENSA-RESEARCH: L5 RESEARCH katmanı başlatılıyor...");
        
        let vgate = vgate::ResearchVGate::new(&config.vgate_url);
        let memory = memory_bridge::ResearchMemoryBridge::new();
        let web_search = web_search::WebSearchEngine::new(config.parallel_searches);
        
        log::info!("🔬 ASENSA-RESEARCH: Native Layer hazırlanıyor...");
        
        Self {
            mindsearch: None,
            autoresearch: None,
            web_search,
            vgate,
            memory,
            config,
            initialized: false,
        }
    }
    
    /// Research sistemini başlat
    pub async fn initialize(&mut self) -> ResearchResult<()> {
        if self.initialized {
            return Ok(());
        }
        
        log::info!("🔬 ASENSA-RESEARCH: Araştırma modülleri başlatılıyor...");
        
        // Native fallback modda başlat
        self.mindsearch = Some(mindsearch::MindSearchWrapper::new_fallback());
        self.autoresearch = Some(autoresearch::AutoResearchWrapper::new_fallback());
        
        self.initialized = true;
        log::info!("✅ ASENSA-RESEARCH: Tüm araştırma modülleri hazır");
        Ok(())
    }
    
    /// MindSearch ile araştırma yap
    pub async fn search(&mut self, query: &str) -> ResearchResult<graph::SearchGraph> {
        self.ensure_initialized()?;
        
        log::info!("🔬 ASENSA-RESEARCH: MindSearch sorgusu → {}", query.chars().take(50).collect::<String>());
        
        let ms = self.mindsearch.as_mut().ok_or_else(|| {
            ResearchError::NotInitialized {
                reason: "MindSearch başlatılmadı".into()
            }
        })?;
        
        ms.search(query).await
    }
    
    /// AutoResearch ile detaylı araştırma yap
    pub async fn research(&mut self, topic: &str) -> ResearchResult<autoresearch::ResearchPlan> {
        self.ensure_initialized()?;
        
        log::info!("🔬 ASENSA-RESEARCH: AutoResearch konu → {}", topic.chars().take(50).collect::<String>());
        
        let ar = self.autoresearch.as_mut().ok_or_else(|| {
            ResearchError::NotInitialized {
                reason: "AutoResearch başlatılmadı".into()
            }
        })?;
        
        ar.research(topic).await
    }
    
    /// Web araması yap
    pub async fn web_search(&mut self, query: &str) -> ResearchResult<Vec<web_search::SearchResult>> {
        self.ensure_initialized()?;
        
        self.web_search.search(query).await
    }
    
    /// Citation oluştur
    pub fn cite(&self, results: &[web_search::SearchResult]) -> Vec<citation::Citation> {
        let manager = citation::CitationManager::new(self.config.citation_style);
        manager.cite(results)
    }
    
    /// Araştırma sonucunu belleğe kaydet
    pub async fn save_to_memory(&mut self, session_id: &str, graph: &graph::SearchGraph) -> ResearchResult<()> {
        self.memory.save_research(session_id, graph).await
    }
    
    /// Bellekten araştırma yükle
    pub async fn load_from_memory(&mut self, session_id: &str) -> ResearchResult<Option<graph::SearchGraph>> {
        self.memory.load_research(session_id).await
    }
    
    /// Sistemi kapat
    pub fn close(&mut self) -> ResearchResult<()> {
        if let Some(ref mut ms) = self.mindsearch {
            ms.close()?;
        }
        if let Some(ref mut ar) = self.autoresearch {
            ar.close()?;
        }
        
        self.initialized = false;
        
        log::info!("🔬 ASENSA-RESEARCH: Araştırma sistemleri kapatıldı");
        Ok(())
    }
    
    // ─── Yardımcı Metodlar ───
    
    fn ensure_initialized(&self) -> ResearchResult<()> {
        if !self.initialized {
            Err(ResearchError::NotInitialized {
                reason: "SENTIENT Research başlatılmadı. Önce initialize() çağırın.".into()
            })
        } else {
            Ok(())
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_default() {
        let config = ResearchConfig::default();
        assert_eq!(config.max_depth, 5);
        assert_eq!(config.max_nodes, 50);
        assert_eq!(config.parallel_searches, 3);
    }
    
    #[test]
    fn test_sentient_research_creation() {
        let research = SENTIENTResearch::new(ResearchConfig::default());
        assert!(!research.initialized);
    }
    
    #[test]
    fn test_citation_manager() {
        let manager = CitationManager::new(ReferenceStyle::APA);
        let results = vec![
            SearchResult {
                title: "Test Title".into(),
                url: "https://example.com".into(),
                snippet: "Test snippet".into(),
                rank: 1,
                source_type: web_search::SourceType::Wikipedia,
                credibility: 0.8,
            }
        ];
        
        let citations = manager.cite(&results);
        assert_eq!(citations.len(), 1);
    }
    
    #[test]
    fn test_graph_node_creation() {
        let node = graph::GraphNode {
            id: "test-1".into(),
            query: "Test query".into(),
            response: Some("Test response".into()),
            children: vec![],
            references: std::collections::HashMap::new(),
        };
        
        assert_eq!(node.id, "test-1");
        assert!(node.response.is_some());
    }
}
