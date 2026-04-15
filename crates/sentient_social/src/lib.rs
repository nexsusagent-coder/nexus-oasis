//! ─── SENTIENT SOCIAL MEDIA SYSTEM ───
//!
//! AI-powered social media automation with anti-bot bypass
//!
//! # Features
//! - **Reddit Automation**: Karma building, valuable commenting
//! - **Instagram Content**: AI image + caption generation
//! - **Anti-bot Bypass**: Real browser automation via oasis_hands
//!
//! # Example
//! ```rust,ignore
//! use sentient_social::{RedditClient, InstagramClient};
//!
//! #[tokio::main]
//! async fn main() {
//!     let reddit = RedditClient::new("credentials");
//!     
//!     // Auto-comment on relevant posts
//!     reddit.auto_comment("rust", "This is a great post about Rust!").await;
//!     
//!     // Instagram content creation
//!     let insta = InstagramClient::new("credentials");
//!     insta.create_post("AI generated caption", image_bytes).await;
//! }
//! ```

pub mod models;
pub mod reddit;
pub mod instagram;
pub mod antobot;
pub mod content;

pub use models::{SocialPost, SocialAccount, Platform, PostStatus};
pub use reddit::{RedditClient, RedditConfig, SubredditMonitor};
pub use instagram::{InstagramClient, InstagramConfig};
pub use antobot::{AntiBotBypass, BrowserAutomation};
pub use content::{ContentGenerator, ContentType};

pub mod prelude {
    pub use crate::{RedditClient, InstagramClient, SocialPost};
}

/// Result type for social operations
pub type SocialResult<T> = Result<T, SocialError>;

/// Error type
#[derive(Debug, thiserror::Error)]
pub enum SocialError {
    #[error("Authentication failed: {0}")]
    AuthFailed(String),
    
    #[error("Rate limited: {0}")]
    RateLimited(String),
    
    #[error("Content rejected: {0}")]
    ContentRejected(String),
    
    #[error("API error: {0}")]
    ApiError(String),
    
    #[error("Browser automation failed: {0}")]
    BrowserError(String),
    
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_creation() {
        let err = SocialError::RateLimited("test".into());
        assert!(err.to_string().contains("Rate limited"));
    }
}
