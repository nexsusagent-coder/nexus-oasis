//! ─── SearXNG Client ───

use reqwest::Client;

use crate::models::*;
use crate::{SearchResultType, SearchError};

/// SearXNG client
pub struct SearXNGClient {
    base_url: String,
    http: Client,
}

impl SearXNGClient {
    pub fn new(base_url: String, http: Client) -> Self {
        Self { base_url, http }
    }
    
    /// Perform search via SearXNG API
    pub async fn search(&self, query: &SearchQuery) -> SearchResultType<Vec<SearchResult>> {
        let url = format!("{}/search", self.base_url);
        
        // Pre-create owned strings for params
        let page_str = query.page.to_string();
        
        // Build query parameters
        let mut params = vec![
            ("q", query.query.as_str()),
            ("format", "json"),
            ("pageno", &page_str),
        ];
        
        // Add engines
        let engines: String = query.engines.iter()
            .map(|e| e.to_searxng_id())
            .collect::<Vec<_>>()
            .join(",");
        if !engines.is_empty() {
            params.push(("engines", &engines));
        }
        
        // Add language
        if let Some(lang) = &query.language {
            params.push(("language", lang));
        }
        
        // Add time range
        if let Some(range) = &query.time_range {
            params.push(("time_range", range.to_searxng_value()));
        }
        
        // Add result type
        let category = match query.result_type {
            ResultType::Images => "images",
            ResultType::News => "news",
            ResultType::Videos => "videos",
            ResultType::Maps => "map",
            _ => "general",
        };
        params.push(("category", category));
        
        let response = self.http
            .get(&url)
            .query(&params)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(SearchError::SearchFailed(format!("SearXNG error: {}", response.status())));
        }
        
        let json: serde_json::Value = response.json().await?;
        self.parse_results(&json)
    }
    
    fn parse_results(&self, json: &serde_json::Value) -> SearchResultType<Vec<SearchResult>> {
        let results = json["results"].as_array()
            .ok_or(SearchError::NoResults)?;
        
        let parsed: Vec<SearchResult> = results.iter()
            .enumerate()
            .filter_map(|(i, r)| {
                Some(SearchResult {
                    title: r["title"].as_str()?.to_string(),
                    url: r["url"].as_str()?.to_string(),
                    snippet: r["content"].as_str().map(|s| s.to_string()),
                    engine: r["engine"].as_str().unwrap_or("unknown").to_string(),
                    position: i as u32 + 1,
                    published_date: r["publishedDate"].as_str().map(|s| s.to_string()),
                    thumbnail: r["thumbnail"].as_str().map(|s| s.to_string()),
                })
            })
            .collect();
        
        if parsed.is_empty() {
            Err(SearchError::NoResults)
        } else {
            Ok(parsed)
        }
    }
    
    /// Get search suggestions
    pub async fn suggestions(&self, query: &str) -> SearchResultType<Vec<String>> {
        let url = format!("{}/autocompleter", self.base_url);
        
        let response = self.http
            .get(&url)
            .query(&[("q", query)])
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Ok(vec![]);
        }
        
        let json: serde_json::Value = response.json().await?;
        
        Ok(json["suggestions"].as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default())
    }
}
