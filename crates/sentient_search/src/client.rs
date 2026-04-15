//! ─── Search Client ───

use reqwest::Client;

use crate::models::*;
use crate::{SearchResultType, SearchError};
use crate::searxng::SearXNGClient;
use crate::rate_limiter::RateLimiter;

/// Search client configuration
#[derive(Debug, Clone)]
pub struct SearchConfig {
    pub searxng_url: String,
    pub timeout_seconds: u64,
    pub max_results: usize,
    pub rate_limit_per_second: u32,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            searxng_url: "http://localhost:8888".into(),
            timeout_seconds: 30,
            max_results: 10,
            rate_limit_per_second: 2,
        }
    }
}

impl SearchConfig {
    pub fn new(url: &str) -> Self {
        Self {
            searxng_url: url.to_string(),
            ..Default::default()
        }
    }
    
    pub fn from_env() -> Self {
        let url = std::env::var("SEARXNG_URL")
            .unwrap_or_else(|_| "http://localhost:8888".into());
        Self::new(&url)
    }
}

/// Main search client
pub struct SearchClient {
    config: SearchConfig,
    searxng: SearXNGClient,
    rate_limiter: RateLimiter,
}

impl SearchClient {
    /// Create new search client
    pub fn new(url: &str) -> Self {
        Self::with_config(SearchConfig::new(url))
    }
    
    /// Create with configuration
    pub fn with_config(config: SearchConfig) -> Self {
        let http = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .unwrap();
        
        let searxng = SearXNGClient::new(config.searxng_url.clone(), http);
        let rate_limiter = RateLimiter::new(config.rate_limit_per_second);
        
        Self { config, searxng, rate_limiter }
    }
    
    /// Perform search
    pub async fn search(&self, query: &str) -> SearchResultType<SearchResponse> {
        self.search_with_query(SearchQuery::new(query)).await
    }
    
    /// Perform search with full query options
    pub async fn search_with_query(&self, query: SearchQuery) -> SearchResultType<SearchResponse> {
        // Check rate limit
        if !self.rate_limiter.allow() {
            return Err(SearchError::RateLimited("Please wait before searching again".into()));
        }
        
        let results = self.searxng.search(&query).await?;
        
        // Sort by position and limit
        let mut results = results;
        results.sort_by_key(|r| r.position);
        results.truncate(self.config.max_results);
        
        Ok(SearchResponse {
            query: query.query.clone(),
            total: results.len(),
            results,
            page: query.page,
            suggestions: vec![],
            corrections: vec![],
        })
    }
    
    /// Quick search (returns top 3 results)
    pub async fn quick_search(&self, query: &str) -> SearchResultType<Vec<SearchResult>> {
        let response = self.search(query).await?;
        Ok(response.results.into_iter().take(3).collect())
    }
    
    /// Search for images
    pub async fn search_images(&self, query: &str) -> SearchResultType<Vec<SearchResult>> {
        let query = SearchQuery {
            query: query.to_string(),
            result_type: ResultType::Images,
            ..SearchQuery::new(query)
        };
        
        let response = self.search_with_query(query).await?;
        Ok(response.results)
    }
    
    /// Search for news
    pub async fn search_news(&self, query: &str) -> SearchResultType<Vec<SearchResult>> {
        let query = SearchQuery {
            query: query.to_string(),
            result_type: ResultType::News,
            engines: vec![SearchEngine::News],
            ..SearchQuery::new(query)
        };
        
        let response = self.search_with_query(query).await?;
        Ok(response.results)
    }
    
    /// Search Wikipedia
    pub async fn search_wikipedia(&self, query: &str) -> SearchResultType<Vec<SearchResult>> {
        let query = SearchQuery::new(query)
            .with_engine(SearchEngine::Wikipedia);
        
        let response = self.search_with_query(query).await?;
        Ok(response.results)
    }
    
    /// Search code (GitHub, StackOverflow)
    pub async fn search_code(&self, query: &str) -> SearchResultType<Vec<SearchResult>> {
        let query = SearchQuery::new(query)
            .with_engine(SearchEngine::GitHub)
            .with_engine(SearchEngine::StackOverflow);
        
        let response = self.search_with_query(query).await?;
        Ok(response.results)
    }
    
    /// Search academic papers
    pub async fn search_academic(&self, query: &str) -> SearchResultType<Vec<SearchResult>> {
        let query = SearchQuery::new(query)
            .with_engine(SearchEngine::Arxiv)
            .with_engine(SearchEngine::Scholar);
        
        let response = self.search_with_query(query).await?;
        Ok(response.results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config() {
        let config = SearchConfig::new("http://test:8080");
        assert_eq!(config.searxng_url, "http://test:8080");
    }
}
