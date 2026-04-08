//! ─── PLATFORM HANDLERS ───
//!
//! Her platform icin ozel veri toplama mantigi

mod twitter;
mod github;
mod linkedin;
mod reddit;
mod generic;

pub use twitter::TwitterHandler;
pub use github::GitHubHandler;
pub use linkedin::LinkedInHandler;
pub use reddit::RedditHandler;
pub use generic::GenericHandler;

use crate::{ScrapedData, SearchParams, Platform};
use async_trait::async_trait;

/// Platform handler trait
#[async_trait]
pub trait PlatformHandler {
    /// Platform turunu dondur
    fn platform(&self) -> Platform;
    
    /// Arama yap
    async fn search(&self, params: SearchParams, client: &reqwest::Client) -> crate::Result<Vec<ScrapedData>>;
    
    /// Profil getir
    async fn get_profile(&self, username: &str, client: &reqwest::Client) -> crate::Result<ScrapedData>;
    
    /// Trend topicleri getir
    async fn get_trending(&self, client: &reqwest::Client) -> crate::Result<Vec<ScrapedData>>;
    
    /// Desteklenen veri tipleri
    fn supported_data_types(&self) -> Vec<crate::DataType>;
}

/// Handler factory
pub fn create_handler(platform: Platform) -> Box<dyn PlatformHandler + Send + Sync> {
    match platform {
        Platform::Twitter => Box::new(TwitterHandler::new()),
        Platform::GitHub => Box::new(GitHubHandler::new()),
        Platform::LinkedIn => Box::new(LinkedInHandler::new()),
        Platform::Reddit => Box::new(RedditHandler::new()),
        _ => Box::new(GenericHandler::new(platform)),
    }
}
