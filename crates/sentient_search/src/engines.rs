//! ─── Search Engine Configurations ───

use crate::models::SearchEngine;

/// Engine configuration
pub struct EngineConfig {
    pub engine: SearchEngine,
    pub enabled: bool,
    pub weight: f32,
    pub timeout_ms: u32,
}

impl EngineConfig {
    pub fn new(engine: SearchEngine) -> Self {
        Self {
            engine,
            enabled: true,
            weight: 1.0,
            timeout_ms: 5000,
        }
    }
    
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    
    pub fn with_weight(mut self, weight: f32) -> Self {
        self.weight = weight;
        self
    }
}

/// Default engine configurations
pub fn default_engines() -> Vec<EngineConfig> {
    vec![
        EngineConfig::new(SearchEngine::Google).with_weight(1.0),
        EngineConfig::new(SearchEngine::Bing).with_weight(0.9),
        EngineConfig::new(SearchEngine::DuckDuckGo).with_weight(0.8),
        EngineConfig::new(SearchEngine::Wikipedia).with_weight(0.7),
        EngineConfig::new(SearchEngine::Reddit).with_weight(0.5),
        EngineConfig::new(SearchEngine::GitHub).with_weight(0.6),
    ]
}

/// Engine categories
pub fn engines_for_category(category: &str) -> Vec<SearchEngine> {
    match category {
        "general" => vec![SearchEngine::Google, SearchEngine::Bing, SearchEngine::DuckDuckGo],
        "images" => vec![SearchEngine::Google, SearchEngine::Bing],
        "news" => vec![SearchEngine::Google, SearchEngine::Bing],
        "code" => vec![SearchEngine::GitHub, SearchEngine::StackOverflow],
        "academic" => vec![SearchEngine::Arxiv, SearchEngine::Scholar],
        "videos" => vec![SearchEngine::YouTube],
        _ => vec![SearchEngine::Google],
    }
}
