// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Web Search Integration
// ═══════════════════════════════════════════════════════════════════════════════
//  Multi-provider web search for AI agents
//  Providers: Tavily, SerpAPI, Brave Search, DuckDuckGo, Bing, Google Custom
// ═══════════════════════════════════════════════════════════════════════════════

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

pub mod providers;
pub mod types;
pub mod error;

pub use providers::{SearchProvider, tavily::TavilySearch, brave::BraveSearch, duckduckgo::DuckDuckGoSearch};
pub use types::{SearchResult, SearchResponse, SearchOptions, SearchProvider as ProviderType};
pub use error::{SearchError, Result};


/// Main search client with multiple provider support
pub struct WebSearch {
    provider: Box<dyn SearchProvider + Send + Sync>,
}

impl WebSearch {
    /// Create a new WebSearch with Tavily provider (recommended for AI)
    pub fn tavily(api_key: impl Into<String>) -> Self {
        Self {
            provider: Box::new(TavilySearch::new(api_key)),
        }
    }

    /// Create a new WebSearch with Brave Search provider
    pub fn brave(api_key: impl Into<String>) -> Self {
        Self {
            provider: Box::new(BraveSearch::new(api_key)),
        }
    }

    /// Create a new WebSearch with DuckDuckGo (free, no API key required)
    pub fn duckduckgo() -> Self {
        Self {
            provider: Box::new(DuckDuckGoSearch::new()),
        }
    }

    /// Perform a web search
    pub async fn search(&self, query: &str) -> Result<SearchResponse> {
        self.provider.search(query, Default::default()).await
    }

    /// Perform a web search with custom options
    pub async fn search_with_options(&self, query: &str, options: SearchOptions) -> Result<SearchResponse> {
        self.provider.search(query, options).await
    }

    /// Search and return context string for LLM (summarized results)
    pub async fn search_for_context(&self, query: &str) -> Result<String> {
        let response = self.search(query).await?;
        Ok(response.to_context())
    }

    /// Search and return raw content from top results
    pub async fn search_and_scrape(&self, query: &str, max_pages: usize) -> Result<Vec<(String, String)>> {
        let response = self.search(query).await?;
        let mut results = Vec::new();
        
        for result in response.results.iter().take(max_pages) {
            if let Ok(content) = self.scrape_url(&result.url).await {
                results.push((result.url.clone(), content));
            }
        }
        
        Ok(results)
    }

    async fn scrape_url(&self, url: &str) -> Result<String> {
        // Basic URL scraping - can be enhanced with sentient_ingestor
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header("User-Agent", "SENTIENT-OS/1.0")
            .send()
            .await?;
        
        let html = response.text().await?;
        
        // Simple text extraction (remove HTML tags)
        let text = html
            .replace("<script[^>]*>.*?</script>", "")
            .replace("<style[^>]*>.*?</style>", "")
            .replace("<[^>]+>", " ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        
        // Limit to first 5000 chars
        Ok(text.chars().take(5000).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_options_default() {
        let options = SearchOptions::default();
        assert_eq!(options.max_results, 10);
    }
}
