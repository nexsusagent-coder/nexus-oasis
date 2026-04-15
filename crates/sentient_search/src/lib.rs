//! ─── SENTIENT SEARCH SYSTEM ───
//!
//! Privacy-first local search with SearXNG integration
//!
//! # Features
//! - **SearXNG Integration**: Self-hosted, privacy-focused search
//! - **Multi-engine**: Google, Bing, DuckDuckGo, Wikipedia, etc.
//! - **Rate Limiting**: Respectful API usage
//! - **Result Parsing**: Structured search results
//!
//! # Example
//! ```rust,ignore
//! use sentient_search::{SearchClient, SearchQuery};
//!
//! #[tokio::main]
//! async fn main() {
//!     let search = SearchClient::new("http://localhost:8888");
//!     
//!     let results = search.search("Rust programming language").await;
//!     
//!     for result in results.take(5) {
//!         println!("{} - {}", result.title, result.url);
//!     }
//! }
//! ```

pub mod models;
pub mod client;
pub mod searxng;
pub mod engines;
pub mod rate_limiter;

pub use models::{SearchResult, SearchQuery, SearchEngine, ResultType};
pub use client::{SearchClient, SearchConfig};
pub use searxng::SearXNGClient;
pub use rate_limiter::RateLimiter;

pub mod prelude {
    pub use crate::{SearchClient, SearchResult, SearchQuery};
}

/// Result type for search operations
pub type SearchResultType<T> = Result<T, SearchError>;

/// Error type
#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error("Search failed: {0}")]
    SearchFailed(String),
    
    #[error("Rate limited: {0}")]
    RateLimited(String),
    
    #[error("No results found")]
    NoResults,
    
    #[error("Invalid query: {0}")]
    InvalidQuery(String),
    
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
        let err = SearchError::SearchFailed("test".into());
        assert!(err.to_string().contains("test"));
    }
}
