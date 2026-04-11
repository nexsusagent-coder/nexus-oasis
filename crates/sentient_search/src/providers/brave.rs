// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Brave Search Provider
// ═══════════════════════════════════════════════════════════════════════════════
//  Privacy-focused search API
//  Free tier: 2,000 searches/month
//  Paid: $5 per 1,000 searches
//  Docs: https://brave.com/search/api/
// ═══════════════════════════════════════════════════════════════════════════════

use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use tracing::{debug, info};

use crate::providers::SearchProvider;
use crate::types::{SearchOptions, SearchResponse, SearchResult, SearchProvider as ProviderType};
use crate::error::{SearchError, Result};

/// Brave Search API client
pub struct BraveSearch {
    api_key: String,
    client: Client,
    base_url: String,
}

impl BraveSearch {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            client: Client::new(),
            base_url: "https://api.search.brave.com/res/v1".to_string(),
        }
    }
}

#[derive(Deserialize)]
struct BraveResponse {
    web: BraveWebResults,
}

#[derive(Deserialize)]
struct BraveWebResults {
    results: Vec<BraveResult>,
}

#[derive(Deserialize)]
struct BraveResult {
    title: String,
    url: String,
    description: String,
    #[serde(default)]
    page_age: Option<String>,
}

#[async_trait]
impl SearchProvider for BraveSearch {
    async fn search(&self, query: &str, options: SearchOptions) -> Result<SearchResponse> {
        info!("Searching Brave for: {}", query);

        let response = self.client
            .get(format!("{}/web/search", self.base_url))
            .header("X-Subscription-Token", &self.api_key)
            .header("Accept", "application/json")
            .query(&[
                ("q", query),
                ("count", &options.max_results.to_string()),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(SearchError::ProviderError(format!(
                "Brave API error: {} - {}",
                status, body
            )));
        }

        let brave_response: BraveResponse = response.json().await?;

        debug!("Brave returned {} results", brave_response.web.results.len());

        let results: Vec<SearchResult> = brave_response
            .web
            .results
            .into_iter()
            .map(|r| SearchResult {
                title: r.title,
                url: r.url,
                snippet: r.description,
                content: None,
                score: None,
                published_date: r.page_age,
                source: None,
            })
            .collect();

        Ok(SearchResponse {
            query: query.to_string(),
            results,
            provider: ProviderType::Brave,
            total: None,
            took_ms: None,
            answer: None,
        })
    }

    fn name(&self) -> &'static str {
        "brave"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brave_search_creation() {
        let search = BraveSearch::new("test-key");
        assert_eq!(search.name(), "brave");
    }
}
