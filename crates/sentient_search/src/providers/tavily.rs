// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Tavily Search Provider
// ═══════════════════════════════════════════════════════════════════════════════
//  Tavily is an AI-optimized search API
//  Free tier: 1,000 searches/month
//  Paid: $0.005/search
//  Docs: https://docs.tavily.com/
// ═══════════════════════════════════════════════════════════════════════════════

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::providers::SearchProvider;
use crate::types::{SearchOptions, SearchResponse, SearchResult, SearchProvider as ProviderType};
use crate::error::{SearchError, Result};

/// Tavily API client
pub struct TavilySearch {
    api_key: String,
    client: Client,
    base_url: String,
}

impl TavilySearch {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            client: Client::new(),
            base_url: "https://api.tavily.com".to_string(),
        }
    }

    /// Create with custom base URL (for testing)
    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            client: Client::new(),
            base_url: base_url.into(),
        }
    }
}

#[derive(Serialize)]
struct TavilyRequest {
    api_key: String,
    query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    search_depth: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    include_domains: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    exclude_domains: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_results: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_answer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_raw_content: Option<bool>,
}

#[derive(Deserialize)]
struct TavilyResponse {
    results: Vec<TavilyResult>,
    answer: Option<String>,
    #[serde(default)]
    response_time: f64,
}

#[derive(Deserialize)]
struct TavilyResult {
    title: String,
    url: String,
    content: String,
    score: f64,
    #[serde(default)]
    raw_content: Option<String>,
}

#[async_trait]
impl SearchProvider for TavilySearch {
    async fn search(&self, query: &str, options: SearchOptions) -> Result<SearchResponse> {
        info!("Searching Tavily for: {}", query);
        
        let search_depth = match options.search_depth {
            crate::types::SearchDepth::Basic => "basic",
            crate::types::SearchDepth::Advanced => "advanced",
        };

        let request = TavilyRequest {
            api_key: self.api_key.clone(),
            query: query.to_string(),
            search_depth: Some(search_depth.to_string()),
            include_domains: options.include_domains.clone(),
            exclude_domains: options.exclude_domains.clone(),
            max_results: Some(options.max_results),
            include_answer: Some(true),
            include_raw_content: Some(options.include_content),
        };

        let response = self.client
            .post(format!("{}/search", self.base_url))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(SearchError::ProviderError(format!(
                "Tavily API error: {} - {}",
                status, body
            )));
        }

        let tavily_response: TavilyResponse = response.json().await?;

        debug!("Tavily returned {} results", tavily_response.results.len());

        let results: Vec<SearchResult> = tavily_response
            .results
            .into_iter()
            .map(|r| SearchResult {
                title: r.title,
                url: r.url,
                snippet: r.content,
                content: r.raw_content,
                score: Some(r.score as f32),
                published_date: None,
                source: None,
            })
            .collect();

        Ok(SearchResponse {
            query: query.to_string(),
            results,
            provider: ProviderType::Tavily,
            total: None,
            took_ms: Some(tavily_response.response_time as u64),
            answer: tavily_response.answer,
        })
    }

    fn name(&self) -> &'static str {
        "tavily"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tavily_search_creation() {
        let search = TavilySearch::new("test-key");
        assert_eq!(search.name(), "tavily");
    }
}
