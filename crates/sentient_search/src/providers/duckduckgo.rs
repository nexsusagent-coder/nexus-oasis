// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - DuckDuckGo Search Provider
// ═══════════════════════════════════════════════════════════════════════════════
//  Free search, no API key required
//  Rate limited but reliable
//  Best for development and testing
// ═══════════════════════════════════════════════════════════════════════════════

use async_trait::async_trait;
use reqwest::Client;
use tracing::{debug, info};

use crate::providers::SearchProvider;
use crate::types::{SearchOptions, SearchResponse, SearchResult, SearchProvider as ProviderType};
use crate::error::Result;

/// DuckDuckGo Instant Answer API client
pub struct DuckDuckGoSearch {
    client: Client,
}

impl DuckDuckGoSearch {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// HTML search (for getting actual results)
    pub async fn search_html(&self, query: &str) -> Result<Vec<SearchResult>> {
        let response = self.client
            .get("https://html.duckduckgo.com/html/")
            .query(&[("q", query)])
            .header("User-Agent", "Mozilla/5.0 (compatible; SENTIENT-OS/1.0)")
            .send()
            .await?;

        let html = response.text().await?;
        self.parse_html_results(&html)
    }

    fn parse_html_results(&self, html: &str) -> Result<Vec<SearchResult>> {
        let mut results = Vec::new();

        // Simple regex-based parsing
        // Looking for: <a class="result__a" href="URL">TITLE</a>
        let title_pattern = regex::Regex::new(r#"<a[^>]*class="result__a"[^>]*href="([^"]+)"[^>]*>([^<]+)</a>"#).unwrap();
        let snippet_pattern = regex::Regex::new(r#"<a[^>]*class="result__snippet"[^>]*>([^<]+)</a>"#).unwrap();

        for cap in title_pattern.captures_iter(html) {
            let url = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
            let title = cap.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();

            // DuckDuckGo redirect URL'yi temizle
            let clean_url = self.clean_ddg_url(&url);

            if !title.is_empty() && !clean_url.is_empty() {
                results.push(SearchResult {
                    title,
                    url: clean_url,
                    snippet: String::new(),
                    content: None,
                    score: None,
                    published_date: None,
                    source: None,
                });
            }
        }

        // Snippet'leri ekle
        for (i, cap) in snippet_pattern.captures_iter(html).enumerate() {
            if let Some(result) = results.get_mut(i) {
                result.snippet = cap.get(1)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default();
            }
        }

        Ok(results)
    }

    fn clean_ddg_url(&self, url: &str) -> String {
        // DuckDuckGo uses redirect URLs like:
        // //duckduckgo.com/l/?uddg=ENCODED_URL
        if url.contains("uddg=") {
            if let Some(encoded) = url.split("uddg=").nth(1) {
                if let Some(decoded) = urlencoding_decode(encoded.split('&').next().unwrap_or("")) {
                    return decoded;
                }
            }
        }
        url.to_string()
    }
}

fn urlencoding_decode(s: &str) -> Option<String> {
    // Simple URL decoding
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '%' {
            let hex: String = chars.by_ref().take(2).collect();
            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                result.push(byte as char);
            }
        } else if c == '+' {
            result.push(' ');
        } else {
            result.push(c);
        }
    }
    
    Some(result)
}

impl Default for DuckDuckGoSearch {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SearchProvider for DuckDuckGoSearch {
    async fn search(&self, query: &str, options: SearchOptions) -> Result<SearchResponse> {
        info!("Searching DuckDuckGo for: {}", query);

        let mut results = self.search_html(query).await?;

        // Limit results
        results.truncate(options.max_results);

        debug!("DuckDuckGo returned {} results", results.len());

        Ok(SearchResponse {
            query: query.to_string(),
            results,
            provider: ProviderType::DuckDuckGo,
            total: None,
            took_ms: None,
            answer: None,
        })
    }

    fn name(&self) -> &'static str {
        "duckduckgo"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duckduckgo_search_creation() {
        let search = DuckDuckGoSearch::new();
        assert_eq!(search.name(), "duckduckgo");
    }
}
