// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Search Types
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};

/// Search provider types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchProvider {
    Tavily,
    Brave,
    DuckDuckGo,
    SerpApi,
    Bing,
    Google,
}

/// Individual search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Title of the result
    pub title: String,
    /// URL of the result
    pub url: String,
    /// Snippet/summary of content
    pub snippet: String,
    /// Full content (if scraped)
    pub content: Option<String>,
    /// Relevance score (0.0 - 1.0)
    pub score: Option<f32>,
    /// Published date (if available)
    pub published_date: Option<String>,
    /// Source domain
    pub source: Option<String>,
}

impl SearchResult {
    /// Format result for LLM context
    pub fn to_context(&self) -> String {
        let mut context = format!("**{}**\n", self.title);
        context.push_str(&format!("URL: {}\n", self.url));
        context.push_str(&format!("{}\n", self.snippet));
        
        if let Some(date) = &self.published_date {
            context.push_str(&format!("Published: {}\n", date));
        }
        
        if let Some(content) = &self.content {
            context.push_str(&format!("\nContent:\n{}\n", content));
        }
        
        context
    }
}

/// Search response containing multiple results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    /// Original query
    pub query: String,
    /// Search results
    pub results: Vec<SearchResult>,
    /// Provider used
    pub provider: SearchProvider,
    /// Total estimated results
    pub total: Option<usize>,
    /// Search took milliseconds
    pub took_ms: Option<u64>,
    /// AI-generated answer (if available, e.g., Tavily)
    pub answer: Option<String>,
}

impl SearchResponse {
    /// Format all results for LLM context
    pub fn to_context(&self) -> String {
        let mut context = format!("Search results for: \"{}\"\n\n", self.query);
        
        if let Some(answer) = &self.answer {
            context.push_str(&format!("Answer: {}\n\n", answer));
        }
        
        for (i, result) in self.results.iter().enumerate() {
            context.push_str(&format!("--- Result {} ---\n", i + 1));
            context.push_str(&result.to_context());
            context.push_str("\n");
        }
        
        context
    }

    /// Get just the URLs from results
    pub fn urls(&self) -> Vec<&str> {
        self.results.iter().map(|r| r.url.as_str()).collect()
    }

    /// Get just the snippets from results
    pub fn snippets(&self) -> Vec<&str> {
        self.results.iter().map(|r| r.snippet.as_str()).collect()
    }
}

/// Search options for customizing queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOptions {
    /// Maximum number of results
    pub max_results: usize,
    /// Include full content scraping
    pub include_content: bool,
    /// Search depth (basic or advanced)
    pub search_depth: SearchDepth,
    /// Include domains (filter)
    pub include_domains: Vec<String>,
    /// Exclude domains (filter)
    pub exclude_domains: Vec<String>,
    /// Time range for results
    pub time_range: Option<TimeRange>,
    /// Geographic location for results
    pub country: Option<String>,
    /// Language code
    pub language: Option<String>,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            max_results: 10,
            include_content: false,
            search_depth: SearchDepth::Basic,
            include_domains: Vec::new(),
            exclude_domains: Vec::new(),
            time_range: None,
            country: None,
            language: None,
        }
    }
}

/// Search depth level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchDepth {
    Basic,
    Advanced,
}

/// Time range for search results
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeRange {
    Day,
    Week,
    Month,
    Year,
}

impl TimeRange {
    pub fn to_string(&self) -> &'static str {
        match self {
            TimeRange::Day => "day",
            TimeRange::Week => "week",
            TimeRange::Month => "month",
            TimeRange::Year => "year",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_result_to_context() {
        let result = SearchResult {
            title: "Rust Programming".to_string(),
            url: "https://rust-lang.org".to_string(),
            snippet: "Rust is a systems programming language.".to_string(),
            content: None,
            score: Some(0.95),
            published_date: None,
            source: Some("rust-lang.org".to_string()),
        };
        
        let context = result.to_context();
        assert!(context.contains("Rust Programming"));
        assert!(context.contains("rust-lang.org"));
    }

    #[test]
    fn test_search_response_to_context() {
        let response = SearchResponse {
            query: "rust programming".to_string(),
            results: vec![SearchResult {
                title: "Rust".to_string(),
                url: "https://example.com".to_string(),
                snippet: "Test".to_string(),
                content: None,
                score: None,
                published_date: None,
                source: None,
            }],
            provider: SearchProvider::Tavily,
            total: Some(1),
            took_ms: Some(100),
            answer: Some("Rust is great!".to_string()),
        };
        
        let context = response.to_context();
        assert!(context.contains("Rust is great!"));
    }
}
