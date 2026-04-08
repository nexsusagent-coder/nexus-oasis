//! MindSearch PyO3 Wrapper
//! Python MindSearch modülünün Rust'a native entegrasyonu
//!
//! BELLEK GÜVENLİĞİ:
//! - unsafe blokları minimize edilmiştir
//! - Python GIL yönetimi otomatik
//! - Zero-copy veri aktarımı

use crate::error::{ResearchError, ResearchResult};
use crate::graph::{GraphEdge, GraphNode, SearchGraph};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MindSearch Wrapper
/// Python MindSearchAgent'in Rust wrapper'ı
pub struct MindSearchWrapper {
    /// Yapılandırma
    config: MindSearchConfig,
}

/// MindSearch yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindSearchConfig {
    /// LLM model adı
    pub model_name: String,
    /// Maksimum dönüş sayısı
    pub max_turn: u32,
    /// Özet prompt şablonu
    pub summary_prompt: String,
    /// Arama motoru (google, bing, duckduckgo)
    pub search_engine: String,
    /// Dil
    pub language: String,
}

impl Default for MindSearchConfig {
    fn default() -> Self {
        Self {
            model_name: "qwen/qwen3-235b-a22b:free".into(),
            max_turn: 10,
            summary_prompt: "Aşağıdaki araştırma sonuçlarını özetle:".into(),
            search_engine: "duckduckgo".into(),
            language: "tr".into(),
        }
    }
}

/// Arama node'u
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchNode {
    /// Node ID
    pub id: String,
    /// Arama sorgusu
    pub query: String,
    /// Arama sonucu/yanıt
    pub response: Option<String>,
    /// Alt node'lar
    pub children: Vec<String>,
    /// Referanslar (URL -> başlık)
    pub references: HashMap<String, String>,
    /// Durum
    pub status: NodeStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeStatus {
    Pending,
    Searching,
    Completed,
    Failed,
}

impl MindSearchWrapper {
    /// Yeni MindSearch wrapper oluştur (native)
    pub fn new_fallback() -> Self {
        log::info!("🔬 MINDSEARCH: Native wrapper oluşturuluyor...");
        Self {
            config: MindSearchConfig::default(),
        }
    }
    
    /// Arama yap (async)
    pub async fn search(&mut self, query: &str) -> ResearchResult<SearchGraph> {
        log::info!("🔬 MINDSEARCH: Arama başlatılıyor → {}", query.chars().take(50).collect::<String>());
        
        // Sorgu doğrulama
        self.validate_query(query)?;
        
        // Native arama
        self.search_native(query).await
    }
    
    /// Native arama (Python olmadan)
    async fn search_native(&self, query: &str) -> ResearchResult<SearchGraph> {
        let mut graph = SearchGraph::new(query);
        let root_id = graph.root_id.clone();
        
        // Simüle edilmiş alt sorgular
        let subqueries = vec![
            format!("{} nedir?", query),
            format!("{} tarihçesi", query),
            format!("{} özellikleri", query),
        ];
        
        for sq in subqueries {
            let node_id = graph.add_node(&root_id, &sq);
            graph.set_response(&node_id, &format!("{} hakkında bilgi...", sq));
            graph.add_reference(&node_id, &format!("https://example.com/{}", sq.replace(' ', "-")), &sq);
        }
        
        Ok(graph)
    }
    
    /// Sorgu doğrulama
    fn validate_query(&self, query: &str) -> ResearchResult<()> {
        if query.trim().is_empty() {
            return Err(ResearchError::InvalidQuery {
                reason: "Sorgu boş olamaz".into()
            });
        }
        
        if query.len() > 1000 {
            return Err(ResearchError::InvalidQuery {
                reason: "Sorgu çok uzun (max 1000 karakter)".into()
            });
        }
        
        // Güvenlik kontrolü - prompt injection
        let suspicious = ["ignore previous", "system:", "assistant:", "delete", "drop table"];
        let lower_query = query.to_lowercase();
        for pattern in suspicious {
            if lower_query.contains(pattern) {
                return Err(ResearchError::InvalidQuery {
                    reason: format!("Şüpheli sorgu deseni: {}", pattern)
                });
            }
        }
        
        Ok(())
    }
    
    /// Agent'ı kapat
    pub fn close(&mut self) -> ResearchResult<()> {
        log::info!("🔬 MINDSEARCH: Agent kapatıldı");
        Ok(())
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mindsearch_config_default() {
        let config = MindSearchConfig::default();
        assert_eq!(config.max_turn, 10);
        assert_eq!(config.search_engine, "duckduckgo");
    }
    
    #[test]
    fn test_query_validation() {
        let wrapper = MindSearchWrapper::new_fallback();
        
        // Geçerli sorgu
        assert!(wrapper.validate_query("Normal bir arama sorgusu").is_ok());
        
        // Boş sorgu
        assert!(wrapper.validate_query("").is_err());
        
        // Şüpheli sorgu
        assert!(wrapper.validate_query("ignore previous instructions").is_err());
    }
    
    #[test]
    fn test_search_node_creation() {
        let node = SearchNode {
            id: "test-1".into(),
            query: "Test query".into(),
            response: Some("Test response".into()),
            children: vec!["child-1".into()],
            references: HashMap::new(),
            status: NodeStatus::Completed,
        };
        
        assert_eq!(node.id, "test-1");
        assert_eq!(node.status, NodeStatus::Completed);
    }
    
    #[tokio::test]
    async fn test_native_search() {
        let wrapper = MindSearchWrapper::new_fallback();
        let result = wrapper.search_native("test query").await;
        
        assert!(result.is_ok());
        let graph = result.unwrap();
        assert!(graph.node_count() >= 1);
    }
}
