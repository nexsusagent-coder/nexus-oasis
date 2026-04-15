//! ─── Social Media Models ───

use serde::{Deserialize, Serialize};

/// Social media post
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialPost {
    pub id: String,
    pub platform: Platform,
    pub content: String,
    pub media_urls: Vec<String>,
    pub author: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub engagement: Engagement,
    pub status: PostStatus,
}

impl SocialPost {
    pub fn new(platform: Platform, content: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            platform,
            content: content.to_string(),
            media_urls: vec![],
            author: "me".into(),
            created: chrono::Utc::now(),
            engagement: Engagement::default(),
            status: PostStatus::Draft,
        }
    }
    
    pub fn with_media(mut self, url: &str) -> Self {
        self.media_urls.push(url.to_string());
        self
    }
}

/// Engagement metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Engagement {
    pub likes: u64,
    pub comments: u64,
    pub shares: u64,
    pub views: u64,
}

/// Platform
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    Reddit,
    Instagram,
    Twitter,
    LinkedIn,
    Facebook,
    TikTok,
    YouTube,
}

impl Platform {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Reddit => "Reddit",
            Self::Instagram => "Instagram",
            Self::Twitter => "X (Twitter)",
            Self::LinkedIn => "LinkedIn",
            Self::Facebook => "Facebook",
            Self::TikTok => "TikTok",
            Self::YouTube => "YouTube",
        }
    }
}

/// Post status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PostStatus {
    Draft,
    Scheduled,
    Published,
    Failed,
    Deleted,
}

/// Social account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialAccount {
    pub id: String,
    pub platform: Platform,
    pub username: String,
    pub display_name: Option<String>,
    pub followers: u64,
    pub following: u64,
    pub verified: bool,
    pub connected: bool,
}

impl SocialAccount {
    pub fn new(platform: Platform, username: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            platform,
            username: username.to_string(),
            display_name: None,
            followers: 0,
            following: 0,
            verified: false,
            connected: true,
        }
    }
}

/// Content type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContentType {
    Text,
    Image,
    Video,
    Carousel,
    Story,
    Reel,
    Thread,
}

/// Reddit specific models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedditPost {
    pub id: String,
    pub subreddit: String,
    pub title: String,
    pub selftext: String,
    pub author: String,
    pub score: i64,
    pub upvote_ratio: f64,
    pub num_comments: u64,
    pub created: chrono::DateTime<chrono::Utc>,
    pub permalink: String,
}

/// Instagram specific models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstagramPost {
    pub id: String,
    pub caption: String,
    pub media_type: String,
    pub media_url: String,
    pub permalink: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_post_creation() {
        let post = SocialPost::new(Platform::Reddit, "Hello World")
            .with_media("https://example.com/image.jpg");
        
        assert_eq!(post.platform, Platform::Reddit);
        assert_eq!(post.media_urls.len(), 1);
    }
}
