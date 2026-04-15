//! ─── Search Models ───

use serde::{Deserialize, Serialize};

/// Search query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub engines: Vec<SearchEngine>,
    pub language: Option<String>,
    pub page: u32,
    pub safe_search: bool,
    pub time_range: Option<TimeRange>,
    pub result_type: ResultType,
}

impl SearchQuery {
    pub fn new(query: &str) -> Self {
        Self {
            query: query.to_string(),
            engines: vec![SearchEngine::Google, SearchEngine::Bing],
            language: Some("en".into()),
            page: 1,
            safe_search: true,
            time_range: None,
            result_type: ResultType::General,
        }
    }
    
    pub fn with_engine(mut self, engine: SearchEngine) -> Self {
        if !self.engines.contains(&engine) {
            self.engines.push(engine);
        }
        self
    }
    
    pub fn with_language(mut self, lang: &str) -> Self {
        self.language = Some(lang.to_string());
        self
    }
    
    pub fn with_time_range(mut self, range: TimeRange) -> Self {
        self.time_range = Some(range);
        self
    }
    
    pub fn images() -> Self {
        Self {
            result_type: ResultType::Images,
            ..Self::new("")
        }
    }
    
    pub fn news() -> Self {
        Self {
            result_type: ResultType::News,
            ..Self::new("")
        }
    }
}

/// Search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: Option<String>,
    pub engine: String,
    pub position: u32,
    pub published_date: Option<String>,
    pub thumbnail: Option<String>,
}

impl SearchResult {
    pub fn new(title: &str, url: &str, engine: &str) -> Self {
        Self {
            title: title.to_string(),
            url: url.to_string(),
            snippet: None,
            engine: engine.to_string(),
            position: 0,
            published_date: None,
            thumbnail: None,
        }
    }
    
    pub fn with_snippet(mut self, snippet: &str) -> Self {
        self.snippet = Some(snippet.to_string());
        self
    }
    
    pub fn with_thumbnail(mut self, url: &str) -> Self {
        self.thumbnail = Some(url.to_string());
        self
    }
}

/// Search engine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchEngine {
    Google,
    Bing,
    DuckDuckGo,
    Wikipedia,
    Reddit,
    YouTube,
    GitHub,
    StackOverflow,
    Arxiv,
    Scholar,
    News,
}

impl SearchEngine {
    pub fn to_searxng_id(&self) -> &'static str {
        match self {
            Self::Google => "google",
            Self::Bing => "bing",
            Self::DuckDuckGo => "duckduckgo",
            Self::Wikipedia => "wikipedia",
            Self::Reddit => "reddit",
            Self::YouTube => "youtube",
            Self::GitHub => "github",
            Self::StackOverflow => "stackoverflow",
            Self::Arxiv => "arxiv",
            Self::Scholar => "google scholar",
            Self::News => "bing news",
        }
    }
}

/// Result type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResultType {
    General,
    Images,
    News,
    Videos,
    Maps,
}

/// Time range filter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeRange {
    Day,
    Week,
    Month,
    Year,
}

impl TimeRange {
    pub fn to_searxng_value(&self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Week => "week",
            Self::Month => "month",
            Self::Year => "year",
        }
    }
}

/// Search response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub query: String,
    pub results: Vec<SearchResult>,
    pub total: usize,
    pub page: u32,
    pub suggestions: Vec<String>,
    pub corrections: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_query_builder() {
        let query = SearchQuery::new("test")
            .with_engine(SearchEngine::Wikipedia)
            .with_language("tr");
        
        assert!(query.engines.contains(&SearchEngine::Wikipedia));
        assert_eq!(query.language, Some("tr".into()));
    }
}
