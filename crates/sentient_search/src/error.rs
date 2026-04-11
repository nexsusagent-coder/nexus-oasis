// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Search Errors
// ═══════════════════════════════════════════════════════════════════════════════

use thiserror::Error;

pub type Result<T> = std::result::Result<T, SearchError>;

/// Search-related errors
#[derive(Debug, Error)]
pub enum SearchError {
    #[error("API key is missing or invalid")]
    InvalidApiKey,

    #[error("Rate limit exceeded for provider")]
    RateLimitExceeded,

    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("No results found for query")]
    NoResults,

    #[error("Provider error: {0}")]
    ProviderError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Timeout while searching")]
    Timeout,

    #[error("Invalid query: {0}")]
    InvalidQuery(String),

    #[error("Scraping failed for URL: {0}")]
    ScrapingError(String),
}

impl SearchError {
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            SearchError::RateLimitExceeded |
            SearchError::Timeout |
            SearchError::NetworkError(_)
        )
    }
}
