//! ─── RESEARCH SOURCES ───
//!
//! Source integrations for Deep Research Agent.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::error::ResearchError;
use crate::types::{SourceResult, SourceType, ResearchQuery};

/// Source trait for research data collection
#[async_trait]
pub trait Source: Send + Sync {
    /// Source name
    fn name(&self) -> &str;
    
    /// Source type
    fn source_type(&self) -> SourceType;
    
    /// Search for sources
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SourceResult>, ResearchError>;
    
    /// Extract full content
    async fn extract(&self, url: &str) -> Result<String, ResearchError>;
    
    /// Check if source is available
    async fn is_available(&self) -> bool;
}

/// Web source (general web scraping)
pub struct WebSource {
    client: reqwest::Client,
}

impl WebSource {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("SENTIENT-Research/4.0")
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
        }
    }
}

impl Default for WebSource {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Source for WebSource {
    fn name(&self) -> &str {
        "Web"
    }

    fn source_type(&self) -> SourceType {
        SourceType::Web
    }

    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SourceResult>, ResearchError> {
        // Placeholder - in production, use search API or web scraping
        Ok(Vec::new())
    }

    async fn extract(&self, url: &str) -> Result<String, ResearchError> {
        let response = self.client.get(url).send().await?;
        let html = response.text().await?;
        
        // Basic HTML to text conversion using scraper
        let document = scraper::Html::parse_document(&html);
        let selector = scraper::Selector::parse("p, h1, h2, h3, h4, h5, h6, article, main, .content, #content")
            .map_err(|e| ResearchError::ParsingFailed(format!("Selector error: {:?}", e)))?;
        
        let mut text_parts = Vec::new();
        for element in document.select(&selector) {
            let text = element.text().collect::<String>();
            let text = text.trim();
            if !text.is_empty() {
                text_parts.push(text.to_string());
            }
        }
        
        Ok(text_parts.join("\n\n"))
    }

    async fn is_available(&self) -> bool {
        true
    }
}

/// Arxiv source for academic papers
pub struct ArxivSource {
    client: reqwest::Client,
    base_url: String,
}

impl ArxivSource {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "http://export.arxiv.org/api/query".to_string(),
        }
    }

    /// Build Arxiv query URL
    fn build_query_url(&self, query: &str, limit: usize) -> String {
        let query = urlencoding::encode(query);
        format!(
            "{}?search_query=all:{}&start=0&max_results={}",
            self.base_url, query, limit
        )
    }

    /// Parse Arxiv XML response
    fn parse_response(&self, xml: &str) -> Vec<SourceResult> {
        // Placeholder - in production, use proper XML parsing
        Vec::new()
    }
}

impl Default for ArxivSource {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Source for ArxivSource {
    fn name(&self) -> &str {
        "Arxiv"
    }

    fn source_type(&self) -> SourceType {
        SourceType::Academic
    }

    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SourceResult>, ResearchError> {
        let url = self.build_query_url(query, limit);
        let response = self.client.get(&url).send().await?;
        let xml = response.text().await?;
        Ok(self.parse_response(&xml))
    }

    async fn extract(&self, url: &str) -> Result<String, ResearchError> {
        // Arxiv abstract extraction
        let response = self.client.get(url).send().await?;
        let html = response.text().await?;
        
        // Extract abstract from Arxiv page
        let document = scraper::Html::parse_document(&html);
        let selector = scraper::Selector::parse("blockquote.abstract")
            .map_err(|e| ResearchError::ParsingFailed(format!("Selector error: {:?}", e)))?;
        
        for element in document.select(&selector) {
            let text = element.text().collect::<String>();
            return Ok(text.trim().to_string());
        }
        
        Ok(String::new())
    }

    async fn is_available(&self) -> bool {
        self.client.get(&self.base_url).send().await.is_ok()
    }
}

/// GitHub source for code repositories
pub struct GitHubSource {
    client: reqwest::Client,
    token: Option<String>,
    base_url: String,
}

impl GitHubSource {
    pub fn new(token: Option<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            token,
            base_url: "https://api.github.com".to_string(),
        }
    }

    /// Build GitHub search URL
    fn build_search_url(&self, query: &str, limit: usize) -> String {
        let query = urlencoding::encode(query);
        format!("{}/search/repositories?q={}&per_page={}", self.base_url, query, limit)
    }

    /// Add authorization header if token is available
    fn add_auth(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(token) = &self.token {
            request.header("Authorization", format!("Bearer {}", token))
        } else {
            request
        }
    }
}

impl Default for GitHubSource {
    fn default() -> Self {
        Self::new(None)
    }
}

#[async_trait]
impl Source for GitHubSource {
    fn name(&self) -> &str {
        "GitHub"
    }

    fn source_type(&self) -> SourceType {
        SourceType::Code
    }

    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SourceResult>, ResearchError> {
        let url = self.build_search_url(query, limit);
        let request = self.client.get(&url);
        let request = self.add_auth(request);
        
        let response = request.send().await?;
        let json: serde_json::Value = response.json().await?;
        
        let mut results = Vec::new();
        if let Some(items) = json["items"].as_array() {
            for item in items {
                let result = SourceResult {
                    id: uuid::Uuid::new_v4().to_string(),
                    url: item["html_url"].as_str().unwrap_or("").to_string(),
                    title: item["full_name"].as_str().unwrap_or("").to_string(),
                    snippet: item["description"].as_str().unwrap_or("").to_string(),
                    content: None,
                    source_type: SourceType::Code,
                    credibility_score: 70.0,
                    relevance_score: 75.0,
                    published_at: None,
                    authors: vec![item["owner"]["login"].as_str().unwrap_or("").to_string()],
                    extracted_at: chrono::Utc::now(),
                    metadata: item.clone(),
                };
                results.push(result);
            }
        }
        
        Ok(results)
    }

    async fn extract(&self, url: &str) -> Result<String, ResearchError> {
        // Extract README content
        let readme_url = format!(
            "{}/repos{}/readme",
            self.base_url,
            url.trim_start_matches("https://github.com")
        );
        
        let request = self.client.get(&readme_url);
        let request = self.add_auth(request);
        
        let response = request.send().await?;
        if response.status().is_success() {
            let json: serde_json::Value = response.json().await?;
            if let Some(content) = json["content"].as_str() {
                // Decode base64 content
                let decoded = base64_decode(content.replace('\n', ""));
                return Ok(decoded);
            }
        }
        
        Ok(String::new())
    }

    async fn is_available(&self) -> bool {
        self.client.get(&format!("{}/rate_limit", self.base_url))
            .send().await.is_ok()
    }
}

/// News source aggregator
pub struct NewsSource {
    client: reqwest::Client,
    api_key: Option<String>,
}

impl NewsSource {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
        }
    }
}

impl Default for NewsSource {
    fn default() -> Self {
        Self::new(None)
    }
}

#[async_trait]
impl Source for NewsSource {
    fn name(&self) -> &str {
        "News"
    }

    fn source_type(&self) -> SourceType {
        SourceType::News
    }

    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SourceResult>, ResearchError> {
        // Placeholder - integrate with news API if available
        Ok(Vec::new())
    }

    async fn extract(&self, url: &str) -> Result<String, ResearchError> {
        let source = WebSource::new();
        source.extract(url).await
    }

    async fn is_available(&self) -> bool {
        self.api_key.is_some()
    }
}

/// Helper function to decode base64
fn base64_decode(input: String) -> String {
    use base64::{Engine, engine::general_purpose::STANDARD};
    match STANDARD.decode(&input) {
        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
        Err(_) => String::new(),
    }
}

/// Source registry for managing multiple sources
pub struct SourceRegistry {
    sources: Vec<Box<dyn Source>>,
}

impl SourceRegistry {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
        }
    }

    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        registry.register(Box::new(WebSource::new()));
        registry.register(Box::new(ArxivSource::new()));
        registry.register(Box::new(GitHubSource::new(None)));
        registry
    }

    pub fn register(&mut self, source: Box<dyn Source>) {
        self.sources.push(source);
    }

    pub async fn search_all(&self, query: &str, limit: usize) -> Vec<SourceResult> {
        let mut all_results = Vec::new();
        
        for source in &self.sources {
            if source.is_available().await {
                if let Ok(results) = source.search(query, limit).await {
                    all_results.extend(results);
                }
            }
        }
        
        // Sort by relevance score
        all_results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap_or(std::cmp::Ordering::Equal));
        all_results.truncate(limit);
        all_results
    }

    /// Extract content from URL using appropriate source
    pub async fn extract(&self, url: &str) -> Result<String, ResearchError> {
        // Determine source type from URL
        let domain = url::Url::parse(url)
            .ok()
            .and_then(|u| u.host_str().map(|h| h.to_string()))
            .unwrap_or_default();

        // Find matching source
        for source in &self.sources {
            if domain.contains("github.com") && source.name() == "GitHub" {
                return source.extract(url).await;
            }
            if domain.contains("arxiv.org") && source.name() == "Arxiv" {
                return source.extract(url).await;
            }
        }

        // Default to web source
        let web = WebSource::new();
        web.extract(url).await
    }
}

impl Default for SourceRegistry {
    fn default() -> Self {
        Self::with_defaults()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_registry() {
        let registry = SourceRegistry::with_defaults();
        assert!(!registry.sources.is_empty());
    }

    #[test]
    fn test_arxiv_query_url() {
        let source = ArxivSource::new();
        let url = source.build_query_url("machine learning", 10);
        assert!(url.contains("machine%20learning"));
    }

    #[test]
    fn test_github_search_url() {
        let source = GitHubSource::new(None);
        let url = source.build_search_url("rust llm", 20);
        assert!(url.contains("rust%20llm"));
    }
}
