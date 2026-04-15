//! ─── Instagram Automation ───

use crate::models::*;
use crate::{SocialResult, SocialError};

/// Instagram client configuration
#[derive(Debug, Clone)]
pub struct InstagramConfig {
    pub access_token: String,
    pub business_account_id: String,
}

impl InstagramConfig {
    pub fn from_env() -> SocialResult<Self> {
        Ok(Self {
            access_token: std::env::var("INSTAGRAM_ACCESS_TOKEN")
                .map_err(|_| SocialError::AuthFailed("INSTAGRAM_ACCESS_TOKEN not set".into()))?,
            business_account_id: std::env::var("INSTAGRAM_BUSINESS_ACCOUNT_ID")
                .unwrap_or_default(),
        })
    }
}

/// Instagram client
pub struct InstagramClient {
    config: InstagramConfig,
    http: reqwest::Client,
}

impl InstagramClient {
    pub fn new(config: InstagramConfig) -> Self {
        Self {
            config,
            http: reqwest::Client::new(),
        }
    }
    
    /// Get user's media
    pub async fn get_media(&self, limit: u32) -> SocialResult<Vec<InstagramPost>> {
        let url = format!(
            "https://graph.instagram.com/me/media?fields=id,caption,media_type,media_url,permalink,timestamp&access_token={}&limit={}",
            self.config.access_token, limit
        );
        
        let response = self.http.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(SocialError::ApiError("Instagram API error".into()));
        }
        
        let json: serde_json::Value = response.json().await?;
        self.parse_media(&json)
    }
    
    /// Create media container for publishing
    pub async fn create_media_container(&self, image_url: &str, caption: &str) -> SocialResult<String> {
        let url = format!(
            "https://graph.facebook.com/{}/media?image_url={}&caption={}&access_token={}",
            self.config.business_account_id,
            urlencoding_encode(image_url),
            urlencoding_encode(caption),
            self.config.access_token
        );
        
        let response = self.http.post(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(SocialError::ApiError("Failed to create media container".into()));
        }
        
        let json: serde_json::Value = response.json().await?;
        Ok(json["id"].as_str().unwrap_or("unknown").to_string())
    }
    
    /// Publish media container
    pub async fn publish_media(&self, creation_id: &str) -> SocialResult<String> {
        let url = format!(
            "https://graph.facebook.com/{}/media_publish?creation_id={}&access_token={}",
            self.config.business_account_id,
            creation_id,
            self.config.access_token
        );
        
        let response = self.http.post(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(SocialError::ApiError("Failed to publish media".into()));
        }
        
        let json: serde_json::Value = response.json().await?;
        Ok(json["id"].as_str().unwrap_or("unknown").to_string())
    }
    
    /// Create and publish post in one step
    pub async fn create_post(&self, image_url: &str, caption: &str) -> SocialResult<String> {
        let creation_id = self.create_media_container(image_url, caption).await?;
        self.publish_media(&creation_id).await
    }
    
    /// Get insights for media
    pub async fn get_insights(&self, media_id: &str) -> SocialResult<Engagement> {
        let url = format!(
            "https://graph.instagram.com/{}/insights?metric=impressions,reach,engagement&access_token={}",
            media_id, self.config.access_token
        );
        
        let response = self.http.get(&url).send().await?;
        let json: serde_json::Value = response.json().await?;
        
        Ok(Engagement {
            likes: json["data"][0]["values"][0]["value"].as_u64().unwrap_or(0),
            comments: 0,
            shares: 0,
            views: json["data"][0]["values"][0]["value"].as_u64().unwrap_or(0),
        })
    }
    
    /// Get hashtag suggestions
    pub async fn get_hashtag_suggestions(&self, query: &str) -> SocialResult<Vec<String>> {
        // TODO: Implement hashtag search API
        let default_tags = vec![
            format!("#{}", query.to_lowercase().replace(' ', "")),
            "#ai".into(),
            "#technology".into(),
            "#innovation".into(),
        ];
        Ok(default_tags)
    }
    
    fn parse_media(&self, json: &serde_json::Value) -> SocialResult<Vec<InstagramPost>> {
        let data = json["data"].as_array()
            .ok_or_else(|| SocialError::ApiError("Invalid response".into()))?;
        
        let posts: Vec<InstagramPost> = data.iter()
            .filter_map(|item| {
                Some(InstagramPost {
                    id: item["id"].as_str()?.to_string(),
                    caption: item["caption"].as_str().unwrap_or("").to_string(),
                    media_type: item["media_type"].as_str().unwrap_or("IMAGE").to_string(),
                    media_url: item["media_url"].as_str().unwrap_or("").to_string(),
                    permalink: item["permalink"].as_str().unwrap_or("").to_string(),
                    timestamp: chrono::DateTime::parse_from_rfc3339(
                        item["timestamp"].as_str().unwrap_or("")
                    ).ok()?.with_timezone(&chrono::Utc),
                })
            })
            .collect();
        
        Ok(posts)
    }
}

fn urlencoding_encode(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_url_encoding() {
        let encoded = urlencoding_encode("hello world");
        assert_eq!(encoded, "hello%20world");
    }
}
