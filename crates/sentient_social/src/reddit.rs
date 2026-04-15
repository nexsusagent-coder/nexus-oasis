//! ─── Reddit Automation ───

use reqwest::Client;

use crate::models::*;
use crate::{SocialResult, SocialError};

/// Reddit client configuration
#[derive(Debug, Clone)]
pub struct RedditConfig {
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String,
    pub user_agent: String,
}

impl RedditConfig {
    pub fn from_env() -> SocialResult<Self> {
        Ok(Self {
            client_id: std::env::var("REDDIT_CLIENT_ID")
                .map_err(|_| SocialError::AuthFailed("REDDIT_CLIENT_ID not set".into()))?,
            client_secret: std::env::var("REDDIT_CLIENT_SECRET")
                .map_err(|_| SocialError::AuthFailed("REDDIT_CLIENT_SECRET not set".into()))?,
            username: std::env::var("REDDIT_USERNAME")
                .map_err(|_| SocialError::AuthFailed("REDDIT_USERNAME not set".into()))?,
            password: std::env::var("REDDIT_PASSWORD")
                .map_err(|_| SocialError::AuthFailed("REDDIT_PASSWORD not set".into()))?,
            user_agent: "SENTIENT-OS/1.0".into(),
        })
    }
}

/// Reddit client
pub struct RedditClient {
    config: RedditConfig,
    http: Client,
    access_token: Option<String>,
}

impl RedditClient {
    pub fn new(config: RedditConfig) -> Self {
        Self {
            config,
            http: Client::new(),
            access_token: None,
        }
    }
    
    /// Authenticate with Reddit
    pub async fn authenticate(&mut self) -> SocialResult<()> {
        let response = self.http
            .post("https://www.reddit.com/api/v1/access_token")
            .header("User-Agent", &self.config.user_agent)
            .basic_auth(&self.config.client_id, Some(&self.config.client_secret))
            .form(&[
                ("grant_type", "password"),
                ("username", &self.config.username),
                ("password", &self.config.password),
            ])
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(SocialError::AuthFailed("Reddit authentication failed".into()));
        }
        
        let json: serde_json::Value = response.json().await?;
        self.access_token = json["access_token"].as_str().map(|s| s.to_string());
        
        Ok(())
    }
    
    /// Get posts from subreddit
    pub async fn get_subreddit_posts(&self, subreddit: &str, limit: u32) -> SocialResult<Vec<RedditPost>> {
        let token = self.access_token.as_ref()
            .ok_or_else(|| SocialError::AuthFailed("Not authenticated".into()))?;
        
        let url = format!("https://oauth.reddit.com/r/{}/hot?limit={}", subreddit, limit);
        
        let response = self.http
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", &self.config.user_agent)
            .send()
            .await?;
        
        let json: serde_json::Value = response.json().await?;
        self.parse_posts(&json)
    }
    
    /// Post a comment
    pub async fn comment(&self, post_id: &str, text: &str) -> SocialResult<String> {
        let token = self.access_token.as_ref()
            .ok_or_else(|| SocialError::AuthFailed("Not authenticated".into()))?;
        
        let response = self.http
            .post("https://oauth.reddit.com/api/comment")
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", &self.config.user_agent)
            .form(&[
                ("thing_id", post_id),
                ("text", text),
            ])
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(SocialError::ApiError("Failed to post comment".into()));
        }
        
        Ok("comment_posted".into())
    }
    
    /// Upvote a post
    pub async fn upvote(&self, post_id: &str) -> SocialResult<()> {
        let token = self.access_token.as_ref()
            .ok_or_else(|| SocialError::AuthFailed("Not authenticated".into()))?;
        
        self.http
            .post("https://oauth.reddit.com/api/vote")
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", &self.config.user_agent)
            .form(&[
                ("id", post_id),
                ("dir", "1"),
            ])
            .send()
            .await?;
        
        Ok(())
    }
    
    /// Submit a post
    pub async fn submit_post(&self, subreddit: &str, title: &str, text: &str) -> SocialResult<String> {
        let token = self.access_token.as_ref()
            .ok_or_else(|| SocialError::AuthFailed("Not authenticated".into()))?;
        
        let response = self.http
            .post("https://oauth.reddit.com/api/submit")
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", &self.config.user_agent)
            .form(&[
                ("sr", subreddit),
                ("kind", "self"),
                ("title", title),
                ("text", text),
            ])
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(SocialError::ApiError("Failed to submit post".into()));
        }
        
        let json: serde_json::Value = response.json().await?;
        Ok(json["json"]["data"]["name"].as_str().unwrap_or("unknown").to_string())
    }
    
    fn parse_posts(&self, json: &serde_json::Value) -> SocialResult<Vec<RedditPost>> {
        let children = json["data"]["children"].as_array()
            .ok_or_else(|| SocialError::ApiError("Invalid response".into()))?;
        
        let posts: Vec<RedditPost> = children.iter()
            .filter_map(|child| {
                let data = &child["data"];
                Some(RedditPost {
                    id: data["id"].as_str()?.to_string(),
                    subreddit: data["subreddit"].as_str()?.to_string(),
                    title: data["title"].as_str()?.to_string(),
                    selftext: data["selftext"].as_str().unwrap_or("").to_string(),
                    author: data["author"].as_str().unwrap_or("unknown").to_string(),
                    score: data["score"].as_i64()? as i64,
                    upvote_ratio: data["upvote_ratio"].as_f64()? as f64,
                    num_comments: data["num_comments"].as_u64()? as u64,
                    created: chrono::DateTime::from_timestamp(data["created_utc"].as_f64()? as i64, 0)?
                        .with_timezone(&chrono::Utc),
                    permalink: data["permalink"].as_str().unwrap_or("").to_string(),
                })
            })
            .collect();
        
        Ok(posts)
    }
}

/// Subreddit monitor for auto-engagement
pub struct SubredditMonitor {
    client: RedditClient,
    watched: Vec<String>,
    auto_comment_rules: Vec<CommentRule>,
}

#[derive(Debug, Clone)]
pub struct CommentRule {
    pub subreddit: String,
    pub keywords: Vec<String>,
    pub template: String,
    pub min_score: i64,
}

impl SubredditMonitor {
    pub fn new(client: RedditClient) -> Self {
        Self {
            client,
            watched: vec![],
            auto_comment_rules: vec![],
        }
    }
    
    pub fn watch(mut self, subreddit: &str) -> Self {
        self.watched.push(subreddit.to_string());
        self
    }
    
    pub fn add_comment_rule(mut self, rule: CommentRule) -> Self {
        self.auto_comment_rules.push(rule);
        self
    }
    
    /// Check for new posts and potentially comment
    pub async fn check_and_engage(&self) -> SocialResult<Vec<String>> {
        let mut actions = Vec::new();
        
        for subreddit in &self.watched {
            let posts = self.client.get_subreddit_posts(subreddit, 10).await?;
            
            for post in posts {
                if self.should_comment(&post) {
                    let comment = self.generate_comment(&post);
                    self.client.comment(&format!("t3_{}", post.id), &comment).await?;
                    actions.push(format!("Commented on: {}", post.title));
                }
            }
        }
        
        Ok(actions)
    }
    
    fn should_comment(&self, post: &RedditPost) -> bool {
        for rule in &self.auto_comment_rules {
            if rule.subreddit == post.subreddit && post.score >= rule.min_score {
                if rule.keywords.iter().any(|k| post.title.to_lowercase().contains(&k.to_lowercase())) {
                    return true;
                }
            }
        }
        false
    }
    
    fn generate_comment(&self, post: &RedditPost) -> String {
        // Find matching rule and use template
        for rule in &self.auto_comment_rules {
            if rule.subreddit == post.subreddit {
                return rule.template.replace("{title}", &post.title);
            }
        }
        format!("Great post about {}!", post.title)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_comment_rule() {
        let rule = CommentRule {
            subreddit: "rust".into(),
            keywords: vec!["help".into()],
            template: "Check out the Rust book!".into(),
            min_score: 5,
        };
        
        assert_eq!(rule.subreddit, "rust");
    }
}
