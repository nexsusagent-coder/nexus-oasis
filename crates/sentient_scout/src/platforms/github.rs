//! ─── GITHUB HANDLER ───

use super::PlatformHandler;
use crate::{ScrapedData, SearchParams, Platform, DataType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// GitHub handler
pub struct GitHubHandler {
    api_base: String,
}

impl GitHubHandler {
    pub fn new() -> Self {
        Self {
            api_base: "https://api.github.com".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubUser {
    pub login: String,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub public_repos: u32,
    pub followers: u32,
    pub following: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubRepo {
    pub full_name: String,
    pub description: Option<String>,
    pub stargazers_count: u32,
    pub forks_count: u32,
    pub language: Option<String>,
    pub topics: Vec<String>,
}

#[async_trait]
impl PlatformHandler for GitHubHandler {
    fn platform(&self) -> Platform {
        Platform::GitHub
    }
    
    async fn search(&self, params: SearchParams, client: &reqwest::Client) -> crate::Result<Vec<ScrapedData>> {
        log::debug!("[GitHub] Arama: {}", params.query);
        
        let url = format!(
            "{}/search/repositories?q={}&per_page={}",
            self.api_base,
            urlencoding::encode(&params.query),
            params.limit.min(100)
        );
        
        let response = client
            .get(&url)
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "SENTIENT-Scout/1.0")
            .send()
            .await
            .map_err(|e| crate::ScoutError::Connection(e.to_string()))?;
        
        let search_result: SearchResult = response.json().await
            .map_err(|e| crate::ScoutError::ParseError(e.to_string()))?;
        
        let results: Vec<ScrapedData> = search_result.items
            .into_iter()
            .map(|repo| ScrapedData {
                id: uuid::Uuid::new_v4(),
                platform: Platform::GitHub,
                data_type: DataType::Custom("repository".into()),
                raw: serde_json::to_string(&repo).unwrap_or_default(),
                metadata: [
                    ("full_name".into(), repo.full_name.clone()),
                    ("stars".into(), repo.stargazers_count.to_string()),
                    ("language".into(), repo.language.unwrap_or_default()),
                ].into_iter().collect(),
                scraped_at: chrono::Utc::now(),
                source_url: format!("https://github.com/{}", repo.full_name),
            })
            .collect();
        
        log::info!("[GitHub] {} repository bulundu", results.len());
        Ok(results)
    }
    
    async fn get_profile(&self, username: &str, client: &reqwest::Client) -> crate::Result<ScrapedData> {
        let url = format!("{}/users/{}", self.api_base, username);
        
        let response = client
            .get(&url)
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "SENTIENT-Scout/1.0")
            .send()
            .await
            .map_err(|e| crate::ScoutError::Connection(e.to_string()))?;
        
        let user: GitHubUser = response.json().await
            .map_err(|e| crate::ScoutError::ParseError(e.to_string()))?;
        
        Ok(ScrapedData {
            id: uuid::Uuid::new_v4(),
            platform: Platform::GitHub,
            data_type: DataType::UserProfile,
            raw: serde_json::to_string(&user).unwrap_or_default(),
            metadata: [
                ("login".into(), user.login.clone()),
                ("public_repos".into(), user.public_repos.to_string()),
                ("followers".into(), user.followers.to_string()),
            ].into_iter().collect(),
            scraped_at: chrono::Utc::now(),
            source_url: format!("https://github.com/{}", username),
        })
    }
    
    async fn get_trending(&self, client: &reqwest::Client) -> crate::Result<Vec<ScrapedData>> {
        // GitHub trending sayfasi
        let url = "https://github.com/trending";
        
        let response = client
            .get(url)
            .header("Accept", "text/html")
            .send()
            .await
            .map_err(|e| crate::ScoutError::Connection(e.to_string()))?;
        
        let html = response.text().await
            .map_err(|e| crate::ScoutError::ParseError(e.to_string()))?;
        
        // Trending repo'lari parse et
        let trending = parse_trending_repos(&html);
        
        Ok(trending)
    }
    
    fn supported_data_types(&self) -> Vec<DataType> {
        vec![
            DataType::UserProfile,
            DataType::CompanyProfile,
            DataType::Custom("repository".into()),
            DataType::Custom("gist".into()),
            DataType::Custom("issue".into()),
            DataType::Custom("pull_request".into()),
        ]
    }
}

impl Default for GitHubHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize)]
struct SearchResult {
    items: Vec<GitHubRepo>,
}

fn parse_trending_repos(html: &str) -> Vec<ScrapedData> {
    // Placeholder
    vec![]
}

mod urlencoding {
    pub fn encode(s: &str) -> String {
        url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
    }
}
