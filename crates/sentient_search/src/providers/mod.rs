// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Search Providers
// ═══════════════════════════════════════════════════════════════════════════════

pub mod tavily;
pub mod brave;
pub mod duckduckgo;

use async_trait::async_trait;
use crate::types::{SearchResponse, SearchOptions};
use crate::Result;

/// Trait for search providers
#[async_trait]
pub trait SearchProvider {
    /// Perform a search query
    async fn search(&self, query: &str, options: SearchOptions) -> Result<SearchResponse>;
    
    /// Get provider name
    fn name(&self) -> &'static str;
}
