//! ─── DEEP RESEARCH AGENT ───
//!
//! Autonomous research agent that conducts comprehensive research.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

use crate::error::ResearchError;
use crate::types::*;
use crate::sources::SourceRegistry;
use crate::analysis::{AnalysisEngine, AnalysisResults};
use crate::report::{ResearchReport, ReportGenerator, ReportFormat};

/// Research agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchConfig {
    /// V-GATE URL (backward compatibility)
    pub vgate_url: String,
    /// Maximum research depth (backward compatibility)
    pub max_depth: u32,
    /// Parallel searches (backward compatibility)
    pub parallel_searches: u32,
    /// Maximum sources to retrieve
    pub max_sources: usize,
    /// Maximum concurrent source fetches
    pub max_concurrent: usize,
    /// Request timeout in seconds
    pub timeout_secs: u64,
    /// Minimum credibility threshold (0-100)
    pub min_credibility: f32,
    /// Enable cache
    pub use_cache: bool,
    /// User agent for requests
    pub user_agent: String,
    /// Model for synthesis
    pub model: String,
    /// Report format
    pub report_format: ReportFormat,
}

impl Default for ResearchConfig {
    fn default() -> Self {
        Self {
            vgate_url: "http://127.0.0.1:1071".to_string(),
            max_depth: 5,
            parallel_searches: 3,
            max_sources: 20,
            max_concurrent: 10,
            timeout_secs: 300,
            min_credibility: 30.0,
            use_cache: true,
            user_agent: "SENTIENT-Research/4.0".to_string(),
            model: "qwen/qwen3.6-plus:free".to_string(),
            report_format: ReportFormat::Markdown,
        }
    }
}

/// Research status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResearchStatus {
    Idle,
    Searching,
    Extracting,
    Analyzing,
    Synthesizing,
    GeneratingReport,
    Completed,
    Failed,
}

impl Default for ResearchStatus {
    fn default() -> Self {
        Self::Idle
    }
}

/// Research result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchResult {
    /// Research query
    pub query: ResearchQuery,
    /// Research report
    pub report: ResearchReport,
    /// Statistics
    pub stats: ResearchStats,
    /// Duration
    pub duration_secs: f64,
    /// Status
    pub status: ResearchStatus,
}

impl Default for ResearchResult {
    fn default() -> Self {
        Self {
            query: ResearchQuery::default(),
            report: ResearchReport::default(),
            stats: ResearchStats::default(),
            duration_secs: 0.0,
            status: ResearchStatus::Idle,
        }
    }
}

/// Progress callback type
pub type ProgressCallback = Box<dyn Fn(ResearchStatus, String, usize) + Send + Sync>;

/// Deep Research Agent
pub struct ResearchAgent {
    config: ResearchConfig,
    sources: SourceRegistry,
    analysis: AnalysisEngine,
    status: Arc<RwLock<ResearchStatus>>,
    progress: Option<ProgressCallback>,
}

impl ResearchAgent {
    /// Create new research agent
    pub fn new(config: ResearchConfig) -> Self {
        Self {
            sources: SourceRegistry::with_defaults(),
            analysis: AnalysisEngine::new(),
            status: Arc::new(RwLock::new(ResearchStatus::Idle)),
            progress: None,
            config,
        }
    }

    /// Create agent with default config
    pub fn with_defaults() -> Self {
        Self::new(ResearchConfig::default())
    }

    /// Set progress callback
    pub fn with_progress(mut self, callback: ProgressCallback) -> Self {
        self.progress = Some(callback);
        self
    }

    /// Get current status
    pub async fn status(&self) -> ResearchStatus {
        *self.status.read().await
    }

    /// Update status and notify
    async fn update_status(&self, new_status: ResearchStatus, message: &str, progress: usize) {
        let mut status = self.status.write().await;
        *status = new_status;
        
        if let Some(ref callback) = self.progress {
            callback(new_status, message.to_string(), progress);
        }
        
        log::debug!("Research status: {:?} - {}", new_status, message);
    }

    /// Conduct research
    pub async fn research(&self, query: ResearchQuery) -> Result<ResearchResult, ResearchError> {
        let start = Instant::now();
        let mut stats = ResearchStats::default();
        
        // Phase 1: Search
        self.update_status(ResearchStatus::Searching, "Kaynaklar aranıyor...", 0).await;
        
        let search_terms = self.generate_search_terms(&query.topic);
        let mut all_sources = Vec::new();
        
        for term in &search_terms {
            let sources = self.sources.search_all(term, query.max_sources / search_terms.len()).await;
            all_sources.extend(sources);
        }
        
        stats.sources_found = all_sources.len();
        self.update_status(ResearchStatus::Searching, &format!("{} kaynak bulundu", stats.sources_found), 10).await;

        if all_sources.is_empty() {
            return Err(ResearchError::NoSourcesFound);
        }

        // Phase 2: Extract
        self.update_status(ResearchStatus::Extracting, "İçerikler çıkarılıyor...", 20).await;
        
        // Filter by credibility
        all_sources.retain(|s| s.credibility_score >= self.config.min_credibility);
        
        // Extract full content (in parallel, with concurrency limit)
        let sources_to_analyze = self.extract_content(all_sources).await?;
        stats.sources_analyzed = sources_to_analyze.len();

        // Phase 3: Analyze
        self.update_status(ResearchStatus::Analyzing, "Analiz ediliyor...", 40).await;
        
        let analysis_results = self.analysis.analyze_sources(&sources_to_analyze).await?;
        
        // Calculate word count
        for source in &sources_to_analyze {
            if let Some(content) = &source.content {
                stats.words_analyzed += content.split_whitespace().count();
            }
        }

        // Count by type
        stats.academic_papers = sources_to_analyze.iter().filter(|s| s.source_type == SourceType::Academic).count();
        stats.code_repos = sources_to_analyze.iter().filter(|s| s.source_type == SourceType::Code).count();
        stats.news_articles = sources_to_analyze.iter().filter(|s| s.source_type == SourceType::News).count();

        // Phase 4: Synthesize
        self.update_status(ResearchStatus::Synthesizing, "Bulgular sentezleniyor...", 60).await;
        
        let synthesis = self.synthesize_findings(&query, &sources_to_analyze, &analysis_results).await?;

        // Phase 5: Generate Report
        self.update_status(ResearchStatus::GeneratingReport, "Rapor oluşturuluyor...", 80).await;
        
        let generator = ReportGenerator::new(self.config.report_format.clone());
        let report = generator.generate(&query, &sources_to_analyze, synthesis, &stats).await?;

        stats.sources_included = sources_to_analyze.len();
        stats.duration_secs = start.elapsed().as_secs_f64();

        self.update_status(ResearchStatus::Completed, "Araştırma tamamlandı!", 100).await;

        Ok(ResearchResult {
            query,
            report,
            stats,
            duration_secs: start.elapsed().as_secs_f64(),
            status: ResearchStatus::Completed,
        })
    }

    /// Generate search terms from topic
    fn generate_search_terms(&self, topic: &str) -> Vec<String> {
        let mut terms = vec![topic.to_string()];
        
        // Add variations
        terms.push(format!("{} latest research", topic));
        terms.push(format!("{} review", topic));
        terms.push(format!("{} tutorial", topic));
        
        terms
    }

    /// Extract content from sources (with concurrency limit)
    async fn extract_content(&self, sources: Vec<SourceResult>) -> Result<Vec<SourceResult>, ResearchError> {
        let mut extracted = Vec::new();
        let semaphore = Arc::new(tokio::sync::Semaphore::new(self.config.max_concurrent));
        
        for mut source in sources {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            
            // Try to extract content
            if let Ok(content) = self.sources.extract(&source.url).await {
                source.content = Some(content);
                extracted.push(source);
            }
            
            drop(permit);
        }
        
        Ok(extracted)
    }

    /// Synthesize findings using LLM
    async fn synthesize_findings(
        &self,
        query: &ResearchQuery,
        sources: &[SourceResult],
        analysis: &AnalysisResults,
    ) -> Result<Synthesis, ResearchError> {
        // Placeholder - would call LLM for synthesis
        Ok(Synthesis {
            summary: format!("{} hakkında kapsamlı bir araştırma yapıldı.", query.topic),
            key_findings: Vec::new(),
            recommendations: Vec::new(),
            knowledge_gaps: Vec::new(),
            future_research: Vec::new(),
        })
    }

    /// Quick research (depth=Quick)
    pub async fn quick_research(&self, topic: &str) -> Result<ResearchResult, ResearchError> {
        let query = ResearchQuery {
            topic: topic.to_string(),
            depth: ResearchDepth::Quick,
            max_sources: 5,
            time_limit_secs: 30,
            ..Default::default()
        };
        
        self.research(query).await
    }

    /// Comprehensive research
    pub async fn comprehensive_research(&self, topic: &str) -> Result<ResearchResult, ResearchError> {
        let query = ResearchQuery {
            topic: topic.to_string(),
            depth: ResearchDepth::Comprehensive,
            max_sources: 50,
            time_limit_secs: 1800,
            ..Default::default()
        };
        
        self.research(query).await
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // BACKWARD COMPATIBILITY METHODS
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Initialize (backward compatibility)
    pub async fn initialize(&mut self) -> Result<(), ResearchError> {
        log::info!("🔬 ResearchAgent initialized");
        Ok(())
    }
    
    /// Search returning SearchGraph (backward compatibility)
    pub async fn search(&mut self, query: &str) -> Result<crate::graph::SearchGraph, ResearchError> {
        let results = self.sources.search_all(query, self.config.max_sources).await;
        
        let nodes: Vec<crate::graph::SearchNode> = results.into_iter().map(|r| crate::graph::SearchNode {
            id: r.id,
            query: query.to_string(),
            response: r.content,
            references: std::collections::HashMap::new(),
        }).collect();
        
        Ok(crate::graph::SearchGraph {
            nodes,
            edges: Vec::new(),
        })
    }
    
    /// Research from string (backward compatibility - returns SearchGraph)
    pub async fn research_str(&mut self, topic: &str) -> Result<crate::graph::SearchGraph, ResearchError> {
        self.search(topic).await
    }
}

/// Research synthesis
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Synthesis {
    /// Overall summary
    pub summary: String,
    /// Key findings
    pub key_findings: Vec<Finding>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Knowledge gaps identified
    pub knowledge_gaps: Vec<String>,
    /// Future research directions
    pub future_research: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = ResearchConfig::default();
        assert_eq!(config.max_concurrent, 10);
        assert_eq!(config.min_credibility, 30.0);
    }

    #[test]
    fn test_search_terms_generation() {
        let agent = ResearchAgent::with_defaults();
        let terms = agent.generate_search_terms("Rust programming");
        assert!(terms.len() >= 3);
        assert!(terms[0] == "Rust programming");
    }
}
