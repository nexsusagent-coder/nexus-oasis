//! ─── SENTIENT RESEARCH AGENT ───
//!
//! Deep Research Agent for comprehensive research and analysis.
//! Inspired by OpenJarvis Monitor Operative Agent.

pub mod error;
pub mod types;
pub mod sources;
pub mod agent;
pub mod analysis;
pub mod report;

pub use error::ResearchError;
pub use types::*;
pub use agent::ResearchAgent;
pub use sources::{Source, WebSource, ArxivSource, GitHubSource, SourceRegistry};
pub use report::{ResearchReport, ReportSection, ReportFormat};
pub use analysis::{AnalysisEngine, SentimentAnalysis, FactCheckResult, AnalysisResults};

pub use agent::{ResearchConfig, ResearchResult, ResearchStatus};

// ═══════════════════════════════════════════════════════════════════════════════
// BACKWARD COMPATIBILITY - Eski API için uyumluluk tipleri
// ═══════════════════════════════════════════════════════════════════════════════

/// Backward compatibility: SENTIENTResearch alias
pub type SENTIENTResearch = ResearchAgent;

/// Graph module (placeholder for compatibility)
pub mod graph {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    
    /// Search graph node
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SearchNode {
        pub id: String,
        pub query: String,
        pub response: Option<String>,
        pub references: HashMap<String, String>,
    }
    
    /// Search graph
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SearchGraph {
        pub nodes: Vec<SearchNode>,
        pub edges: Vec<(String, String)>,
    }
    
    impl SearchGraph {
        pub fn new(_query: &str) -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
            }
        }
        
        pub fn node_count(&self) -> usize {
            self.nodes.len()
        }
    }
}

/// Web search module (placeholder for compatibility)
pub mod web_search {
    use serde::{Deserialize, Serialize};
    
    /// Web search result
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WebResult {
        pub title: String,
        pub url: String,
        pub snippet: String,
        pub credibility: f32,
    }
    
    /// Web search engine
    pub struct WebSearchEngine {
        parallel_count: u32,
    }
    
    impl WebSearchEngine {
        pub fn new(parallel_count: u32) -> Self {
            Self { parallel_count }
        }
        
        pub async fn search(&mut self, _query: &str) -> Result<Vec<WebResult>, crate::error::ResearchError> {
            // Placeholder - returns empty results
            Ok(Vec::new())
        }
    }
}

/// Citation module (placeholder for compatibility)
pub mod citation {
    use serde::{Deserialize, Serialize};
    
    /// Reference style
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ReferenceStyle {
        APA,
        MLA,
        Chicago,
        IEEE,
    }
    
    /// Citation manager
    pub struct CitationManager {
        style: ReferenceStyle,
    }
    
    impl CitationManager {
        pub fn new(style: ReferenceStyle) -> Self {
            Self { style }
        }
        
        pub fn format(&self, _url: &str) -> String {
            // Placeholder
            String::new()
        }
    }
}
