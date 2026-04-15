//! ─── RESEARCH TYPES ───
//!
//! Core types for the Deep Research Agent.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Research query configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchQuery {
    /// Research topic/question
    pub topic: String,
    /// Search depth (quick/standard/comprehensive)
    pub depth: ResearchDepth,
    /// Maximum sources to analyze
    pub max_sources: usize,
    /// Include academic papers (Arxiv, etc.)
    pub include_academic: bool,
    /// Include code repositories
    pub include_code: bool,
    /// Include news sources
    pub include_news: bool,
    /// Target language for report
    pub language: String,
    /// Time limit in seconds
    pub time_limit_secs: u64,
    /// Custom filters
    pub filters: Vec<ResearchFilter>,
}

impl Default for ResearchQuery {
    fn default() -> Self {
        Self {
            topic: String::new(),
            depth: ResearchDepth::Standard,
            max_sources: 20,
            include_academic: true,
            include_code: true,
            include_news: true,
            language: "tr".to_string(),
            time_limit_secs: 300,
            filters: Vec::new(),
        }
    }
}

/// Research depth level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResearchDepth {
    /// Quick overview (5 sources, 30 seconds)
    Quick,
    /// Standard research (20 sources, 5 minutes)
    Standard,
    /// Comprehensive analysis (50+ sources, 30 minutes)
    Comprehensive,
}

impl Default for ResearchDepth {
    fn default() -> Self {
        Self::Standard
    }
}

/// Research filter criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchFilter {
    /// Filter type
    pub filter_type: FilterType,
    /// Filter value
    pub value: String,
    /// Negate filter
    pub negate: bool,
}

/// Types of filters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilterType {
    /// Date range (e.g., "2024-01-01..2024-12-31")
    DateRange,
    /// Domain filter (e.g., "github.com")
    Domain,
    /// Language filter (ISO code)
    Language,
    /// Author/source filter
    Author,
    /// Keyword exclusion
    ExcludeKeyword,
    /// Minimum credibility score
    MinCredibility,
    /// Content type
    ContentType,
}

/// Research source result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceResult {
    /// Unique identifier
    pub id: String,
    /// Source URL
    pub url: String,
    /// Source title
    pub title: String,
    /// Source snippet/summary
    pub snippet: String,
    /// Full content (if extracted)
    pub content: Option<String>,
    /// Source type
    pub source_type: SourceType,
    /// Credibility score (0-100)
    pub credibility_score: f32,
    /// Relevance score (0-100)
    pub relevance_score: f32,
    /// Publication date
    pub published_at: Option<DateTime<Utc>>,
    /// Authors
    pub authors: Vec<String>,
    /// Extraction timestamp
    pub extracted_at: DateTime<Utc>,
    /// Metadata (source-specific)
    pub metadata: serde_json::Value,
}

impl Default for SourceResult {
    fn default() -> Self {
        Self {
            id: String::new(),
            url: String::new(),
            title: String::new(),
            snippet: String::new(),
            content: None,
            source_type: SourceType::Web,
            credibility_score: 50.0,
            relevance_score: 50.0,
            published_at: None,
            authors: Vec::new(),
            extracted_at: Utc::now(),
            metadata: serde_json::Value::Null,
        }
    }
}

/// Types of sources
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceType {
    /// General web page
    Web,
    /// Academic paper (Arxiv, etc.)
    Academic,
    /// GitHub repository
    Code,
    /// News article
    News,
    /// Blog post
    Blog,
    /// Documentation
    Documentation,
    /// Social media
    Social,
    /// Video content
    Video,
    /// Other/Unknown
    Other,
}

impl SourceType {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Web => "Web",
            Self::Academic => "Academic",
            Self::Code => "Code",
            Self::News => "News",
            Self::Blog => "Blog",
            Self::Documentation => "Docs",
            Self::Social => "Social",
            Self::Video => "Video",
            Self::Other => "Other",
        }
    }
}

/// Research note/annotation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchNote {
    /// Note content
    pub content: String,
    /// Related sources
    pub source_ids: Vec<String>,
    /// Note type
    pub note_type: NoteType,
    /// Timestamp
    pub created_at: DateTime<Utc>,
}

/// Types of research notes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NoteType {
    /// Key finding
    Finding,
    /// Question to investigate
    Question,
    /// Connection between sources
    Connection,
    /// Contradiction found
    Contradiction,
    /// Important quote
    Quote,
    /// Personal thought
    Thought,
}

/// Research statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResearchStats {
    /// Total sources found
    pub sources_found: usize,
    /// Sources analyzed
    pub sources_analyzed: usize,
    /// Sources included in report
    pub sources_included: usize,
    /// Academic papers found
    pub academic_papers: usize,
    /// Code repositories found
    pub code_repos: usize,
    /// News articles found
    pub news_articles: usize,
    /// Total words analyzed
    pub words_analyzed: usize,
    /// Research duration in seconds
    pub duration_secs: f64,
    /// API calls made
    pub api_calls: u32,
    /// Errors encountered
    pub errors: u32,
}

/// Credibility assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredibilityAssessment {
    /// Overall score (0-100)
    pub score: f32,
    /// Domain reputation
    pub domain_reputation: f32,
    /// Author authority
    pub author_authority: f32,
    /// Citation count
    pub citations: u32,
    /// Publication age factor
    pub freshness: f32,
    /// Content quality signals
    pub quality_signals: Vec<QualitySignal>,
}

/// Quality signals for credibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualitySignal {
    /// Signal type
    pub signal_type: QualitySignalType,
    /// Signal strength (0-1)
    pub strength: f32,
}

/// Types of quality signals
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualitySignalType {
    /// Has citations
    HasCitations,
    /// Peer reviewed
    PeerReviewed,
    /// From reputable domain
    ReputableDomain,
    /// Recent publication
    Recent,
    /// Has references
    HasReferences,
    /// Good grammar/structure
    GoodStructure,
    /// Has author info
    HasAuthor,
    /// Factual citations
    FactualCitations,
    /// Bias indicators (negative)
    BiasIndicators,
    /// Clickbait (negative)
    Clickbait,
}

/// Research finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    /// Finding summary
    pub summary: String,
    /// Supporting evidence
    pub evidence: Vec<Evidence>,
    /// Finding confidence (0-100)
    pub confidence: f32,
    /// Related findings
    pub related: Vec<String>,
}

/// Evidence for a finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    /// Source ID
    pub source_id: String,
    /// Quote or excerpt
    pub excerpt: String,
    /// Position in source
    pub position: Option<usize>,
}

/// Comparison between sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceComparison {
    /// Topics being compared
    pub topics: Vec<String>,
    /// Sources involved
    pub source_ids: Vec<String>,
    /// Agreement level (0-100)
    pub agreement: f32,
    /// Key differences
    pub differences: Vec<String>,
    /// Synthesis
    pub synthesis: String,
}
