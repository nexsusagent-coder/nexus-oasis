//! ─── Skill Search ───

use serde::{Deserialize, Serialize};
use crate::Category;

/// Search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub score: f32,
    pub downloads: u64,
    pub rating: f32,
    pub icon_url: Option<String>,
}

/// Search filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilter {
    pub query: String,
    pub categories: Vec<String>,
    pub min_rating: Option<f32>,
    pub min_downloads: Option<u64>,
    pub license: Option<String>,
    pub verified_only: bool,
    pub sort_by: SortBy,
    pub limit: usize,
    pub offset: usize,
}

/// Sort options
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SortBy {
    Relevance,
    Downloads,
    Rating,
    Updated,
    Name,
}

/// Search builder
pub struct SkillSearch {
    filter: SearchFilter,
}

impl SkillSearch {
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            filter: SearchFilter {
                query: query.into(),
                categories: Vec::new(),
                min_rating: None,
                min_downloads: None,
                license: None,
                verified_only: false,
                sort_by: SortBy::Relevance,
                limit: 20,
                offset: 0,
            },
        }
    }
    
    /// Add category filter
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.filter.categories.push(category.into());
        self
    }
    
    /// Set minimum rating
    pub fn min_rating(mut self, rating: f32) -> Self {
        self.filter.min_rating = Some(rating);
        self
    }
    
    /// Set minimum downloads
    pub fn min_downloads(mut self, downloads: u64) -> Self {
        self.filter.min_downloads = Some(downloads);
        self
    }
    
    /// Set license filter
    pub fn license(mut self, license: impl Into<String>) -> Self {
        self.filter.license = Some(license.into());
        self
    }
    
    /// Only verified skills
    pub fn verified_only(mut self) -> Self {
        self.filter.verified_only = true;
        self
    }
    
    /// Set sort order
    pub fn sort_by(mut self, sort: SortBy) -> Self {
        self.filter.sort_by = sort;
        self
    }
    
    /// Set limit
    pub fn limit(mut self, limit: usize) -> Self {
        self.filter.limit = limit;
        self
    }
    
    /// Set offset
    pub fn offset(mut self, offset: usize) -> Self {
        self.filter.offset = offset;
        self
    }
    
    /// Build filter
    pub fn build(self) -> SearchFilter {
        self.filter
    }
}

impl Default for SortBy {
    fn default() -> Self {
        Self::Relevance
    }
}
