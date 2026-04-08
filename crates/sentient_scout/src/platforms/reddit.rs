//! ─── REDDIT HANDLER ───

use super::PlatformHandler;
use crate::{ScrapedData, SearchParams, Platform, DataType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Reddit handler (OAuth destekli)
pub struct RedditHandler {
    api_base: String,
}

impl RedditHandler {
    pub fn new() -> Self {
        Self {
            api_base: "https://oauth.reddit.com".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedditPost {
    pub id: String,
    pub subreddit: String,
    pub title: String,
    pub selftext: String,
    pub author: String,
    pub score: i64,
    pub num_comments: u32,
    pub url: String,
    pub created_utc: f64,
}

#[async_trait]
impl PlatformHandler for RedditHandler {
    fn platform(&self) -> Platform {
        Platform::Reddit
    }
    
    async fn search(&self, params: SearchParams, client: &reqwest::Client) -> crate::Result<Vec<ScrapedData>> {
        log::debug!("[Reddit] Arama: {}", params.query);
        
        // Reddit search API
        let url = format!(
            "https://www.reddit.com/search.json?q={}&limit={}",
            urlencoding::encode(&params.query),
            params.limit
        );
        
        let response = client
            .get(&url)
            .header("User-Agent", "SENTIENT-Scout/1.0")
            .send()
            .await
            .map_err(|e| crate::ScoutError::Connection(e.to_string()))?;
        
        let search_result: RedditSearchResponse = response.json().await
            .map_err(|e| crate::ScoutError::ParseError(e.to_string()))?;
        
        let results: Vec<ScrapedData> = search_result.data.children
            .into_iter()
            .map(|child| {
                let post = child.data;
                ScrapedData {
                    id: uuid::Uuid::new_v4(),
                    platform: Platform::Reddit,
                    data_type: DataType::Post,
                    raw: serde_json::to_string(&post).unwrap_or_default(),
                    metadata: [
                        ("subreddit".into(), post.subreddit),
                        ("author".into(), post.author),
                        ("score".into(), post.score.to_string()),
                        ("comments".into(), post.num_comments.to_string()),
                    ].into_iter().collect(),
                    scraped_at: chrono::Utc::now(),
                    source_url: post.url.clone(),
                }
            })
            .collect();
        
        log::info!("[Reddit] {} post bulundu", results.len());
        Ok(results)
    }
    
    async fn get_profile(&self, username: &str, client: &reqwest::Client) -> crate::Result<ScrapedData> {
        let url = format!("https://www.reddit.com/user/{}/about.json", username);
        
        let response = client
            .get(&url)
            .header("User-Agent", "SENTIENT-Scout/1.0")
            .send()
            .await
            .map_err(|e| crate::ScoutError::Connection(e.to_string()))?;
        
        let user_data: RedditUserResponse = response.json().await
            .map_err(|e| crate::ScoutError::ParseError(e.to_string()))?;
        
        Ok(ScrapedData {
            id: uuid::Uuid::new_v4(),
            platform: Platform::Reddit,
            data_type: DataType::UserProfile,
            raw: serde_json::to_string(&user_data.data).unwrap_or_default(),
            metadata: [
                ("username".into(), user_data.data.name.clone()),
                ("link_karma".into(), user_data.data.link_karma.to_string()),
                ("comment_karma".into(), user_data.data.comment_karma.to_string()),
            ].into_iter().collect(),
            scraped_at: chrono::Utc::now(),
            source_url: format!("https://www.reddit.com/user/{}", username),
        })
    }
    
    async fn get_trending(&self, client: &reqwest::Client) -> crate::Result<Vec<ScrapedData>> {
        // r/popular'dan trending
        let url = "https://www.reddit.com/r/popular.json?limit=25";
        
        let response = client
            .get(url)
            .header("User-Agent", "SENTIENT-Scout/1.0")
            .send()
            .await
            .map_err(|e| crate::ScoutError::Connection(e.to_string()))?;
        
        let popular: RedditSearchResponse = response.json().await
            .map_err(|e| crate::ScoutError::ParseError(e.to_string()))?;
        
        let results: Vec<ScrapedData> = popular.data.children
            .into_iter()
            .map(|child| {
                let post = child.data;
                ScrapedData {
                    id: uuid::Uuid::new_v4(),
                    platform: Platform::Reddit,
                    data_type: DataType::Trending,
                    raw: serde_json::to_string(&post).unwrap_or_default(),
                    metadata: [
                        ("subreddit".into(), post.subreddit),
                        ("score".into(), post.score.to_string()),
                    ].into_iter().collect(),
                    scraped_at: chrono::Utc::now(),
                    source_url: post.url,
                }
            })
            .collect();
        
        Ok(results)
    }
    
    fn supported_data_types(&self) -> Vec<DataType> {
        vec![
            DataType::UserProfile,
            DataType::Post,
            DataType::Comment,
            DataType::Trending,
            DataType::Custom("subreddit".into()),
        ]
    }
}

impl Default for RedditHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize)]
struct RedditSearchResponse {
    data: RedditData,
}

#[derive(Debug, Deserialize)]
struct RedditData {
    children: Vec<RedditChild>,
}

#[derive(Debug, Deserialize)]
struct RedditChild {
    data: RedditPost,
}

#[derive(Debug, Deserialize)]
struct RedditUserResponse {
    data: RedditUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RedditUser {
    name: String,
    link_karma: i64,
    comment_karma: i64,
}

mod urlencoding {
    pub fn encode(s: &str) -> String {
        url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
    }
}
