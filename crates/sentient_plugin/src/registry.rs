//! Plugin registry for marketplace and dependency management

use crate::{PluginError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Plugin registry entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEntry {
    /// Plugin ID
    pub id: String,
    /// Plugin name
    pub name: String,
    /// Plugin description
    pub description: String,
    /// Plugin author
    pub author: String,
    /// All available versions
    pub versions: Vec<RegistryVersion>,
    /// Plugin tags
    pub tags: Vec<String>,
    /// Download count
    pub downloads: u64,
    /// Rating (0-5)
    pub rating: f32,
    /// Rating count
    pub rating_count: u32,
    /// Homepage URL
    pub homepage: Option<String>,
    /// Repository URL
    pub repository: Option<String>,
    /// License
    pub license: String,
    /// Plugin icon URL
    pub icon: Option<String>,
    /// Verified by SENTIENT team
    pub verified: bool,
    /// Featured plugin
    pub featured: bool,
    /// Created at
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Updated at
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Registry version entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryVersion {
    /// Version string
    pub version: String,
    /// Download URL
    pub download_url: String,
    /// Checksum (SHA256)
    pub checksum: String,
    /// Required API version
    pub api_version: String,
    /// Release notes
    pub release_notes: Option<String>,
    /// Prerelease flag
    pub prerelease: bool,
    /// Deprecated flag
    pub deprecated: bool,
    /// File size in bytes
    pub size: u64,
    /// Published at
    pub published_at: chrono::DateTime<chrono::Utc>,
}

/// Registry search options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrySearch {
    /// Search query
    pub query: Option<String>,
    /// Filter by tags
    pub tags: Vec<String>,
    /// Filter by author
    pub author: Option<String>,
    /// Include prerelease versions
    #[serde(default)]
    pub include_prerelease: bool,
    /// Only verified plugins
    #[serde(default)]
    pub verified_only: bool,
    /// Only featured plugins
    #[serde(default)]
    pub featured_only: bool,
    /// Sort by
    #[serde(default)]
    pub sort_by: SortBy,
    /// Sort order
    #[serde(default)]
    pub order: SortOrder,
    /// Page number (1-indexed)
    #[serde(default = "default_page")]
    pub page: u32,
    /// Page size
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

impl Default for RegistrySearch {
    fn default() -> Self {
        Self {
            query: None,
            tags: Vec::new(),
            author: None,
            include_prerelease: false,
            verified_only: false,
            featured_only: false,
            sort_by: SortBy::default(),
            order: SortOrder::default(),
            page: default_page(),
            page_size: default_page_size(),
        }
    }
}

fn default_page() -> u32 { 1 }
fn default_page_size() -> u32 { 20 }

/// Sort by options
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortBy {
    #[default]
    Relevance,
    Downloads,
    Rating,
    Updated,
    Name,
}

/// Sort order
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    #[default]
    Desc,
    Asc,
}

/// Registry search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrySearchResult {
    /// Matching plugins
    pub plugins: Vec<RegistryEntry>,
    /// Total count
    pub total: u64,
    /// Current page
    pub page: u32,
    /// Page size
    pub page_size: u32,
    /// Total pages
    pub total_pages: u32,
}

impl RegistrySearchResult {
    pub fn new(plugins: Vec<RegistryEntry>, total: u64, search: &RegistrySearch) -> Self {
        let total_pages = if search.page_size == 0 {
            1
        } else {
            ((total as f64) / (search.page_size as f64)).ceil().max(1.0) as u32
        };

        Self {
            plugins,
            total,
            page: search.page,
            page_size: search.page_size,
            total_pages,
        }
    }
}

/// Local registry (in-memory)
pub struct PluginRegistry {
    entries: HashMap<String, RegistryEntry>,
    categories: HashMap<String, Vec<String>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            categories: HashMap::new(),
        }
    }

    /// Add plugin to registry
    pub fn add(&mut self, entry: RegistryEntry) {
        for tag in &entry.tags.clone() {
            self.categories
                .entry(tag.clone())
                .or_default()
                .push(entry.id.clone());
        }
        self.entries.insert(entry.id.clone(), entry);
    }

    /// Remove plugin from registry
    pub fn remove(&mut self, id: &str) -> Option<RegistryEntry> {
        let entry = self.entries.remove(id)?;
        for tag in &entry.tags {
            if let Some(plugins) = self.categories.get_mut(tag) {
                plugins.retain(|p| p != id);
            }
        }
        Some(entry)
    }

    /// Get plugin by ID
    pub fn get(&self, id: &str) -> Option<&RegistryEntry> {
        self.entries.get(id)
    }

    /// Get latest version of plugin
    pub fn get_latest_version(&self, id: &str) -> Option<&RegistryVersion> {
        self.entries.get(id).and_then(|e| {
            e.versions.iter().find(|v| !v.prerelease && !v.deprecated)
        })
    }

    /// Get specific version
    pub fn get_version(&self, id: &str, version: &str) -> Option<&RegistryVersion> {
        self.entries.get(id).and_then(|e| {
            e.versions.iter().find(|v| v.version == version)
        })
    }

    /// Search plugins
    pub fn search(&self, search: &RegistrySearch) -> RegistrySearchResult {
        let mut results: Vec<&RegistryEntry> = self.entries.values().collect();

        // Apply filters
        if let Some(query) = &search.query {
            let query_lower = query.to_lowercase();
            results.retain(|e| {
                e.name.to_lowercase().contains(&query_lower)
                    || e.description.to_lowercase().contains(&query_lower)
                    || e.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
            });
        }

        if !search.tags.is_empty() {
            results.retain(|e| {
                search.tags.iter().any(|t| e.tags.contains(t))
            });
        }

        if let Some(author) = &search.author {
            results.retain(|e| &e.author == author);
        }

        if search.verified_only {
            results.retain(|e| e.verified);
        }

        if search.featured_only {
            results.retain(|e| e.featured);
        }

        // Filter versions
        if !search.include_prerelease {
            for _entry in &mut results {
                // Note: Can't mutate through iterator, would need to clone
            }
        }

        // Sort
        match search.sort_by {
            SortBy::Downloads => results.sort_by(|a, b| b.downloads.cmp(&a.downloads)),
            SortBy::Rating => results.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap()),
            SortBy::Updated => results.sort_by(|a, b| b.updated_at.cmp(&a.updated_at)),
            SortBy::Name => results.sort_by(|a, b| a.name.cmp(&b.name)),
            SortBy::Relevance => {
                // Default order
            }
        }

        if matches!(search.order, SortOrder::Asc) {
            results.reverse();
        }

        let total = results.len() as u64;
        let skip = ((search.page - 1) * search.page_size) as usize;
        let take = search.page_size as usize;

        let plugins: Vec<RegistryEntry> = results
            .into_iter()
            .skip(skip)
            .take(take)
            .cloned()
            .collect();

        RegistrySearchResult::new(plugins, total, search)
    }

    /// Get plugins by category
    pub fn by_category(&self, category: &str) -> Vec<&RegistryEntry> {
        self.categories
            .get(category)
            .map(|ids| ids.iter().filter_map(|id| self.entries.get(id)).collect())
            .unwrap_or_default()
    }

    /// Get featured plugins
    pub fn featured(&self) -> Vec<&RegistryEntry> {
        self.entries.values().filter(|e| e.featured).collect()
    }

    /// Get all categories
    pub fn categories(&self) -> Vec<&str> {
        self.categories.keys().map(String::as_str).collect()
    }

    /// Get total count
    pub fn count(&self) -> usize {
        self.entries.len()
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Dependency resolver
pub struct DependencyResolver<'a> {
    registry: &'a PluginRegistry,
}

impl<'a> DependencyResolver<'a> {
    pub fn new(registry: &'a PluginRegistry) -> Self {
        Self { registry }
    }

    /// Resolve dependencies for plugin
    pub fn resolve(&self, plugin_id: &str) -> Result<Vec<String>> {
        let mut resolved = Vec::new();
        let mut visiting = Vec::new();
        self.resolve_recursive(plugin_id, &mut resolved, &mut visiting)?;
        Ok(resolved)
    }

    fn resolve_recursive(
        &self,
        plugin_id: &str,
        resolved: &mut Vec<String>,
        visiting: &mut Vec<String>,
    ) -> Result<()> {
        // Already resolved
        if resolved.contains(&plugin_id.to_string()) {
            return Ok(());
        }

        // Circular dependency
        if visiting.contains(&plugin_id.to_string()) {
            return Err(PluginError::CircularDependency(
                format!("{} -> {}", plugin_id, visiting.join(" -> "))
            ));
        }

        visiting.push(plugin_id.to_string());

        // Get plugin info
        let _entry = self.registry.get(plugin_id)
            .ok_or_else(|| PluginError::not_found(plugin_id))?;

        // In real implementation, would check manifest dependencies
        // For now, just add the plugin
        visiting.pop();
        resolved.push(plugin_id.to_string());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_entry(id: &str, name: &str) -> RegistryEntry {
        RegistryEntry {
            id: id.to_string(),
            name: name.to_string(),
            description: "Test plugin".to_string(),
            author: "Test".to_string(),
            versions: vec![RegistryVersion {
                version: "1.0.0".to_string(),
                download_url: "https://example.com/plugin.zip".to_string(),
                checksum: "abc123".to_string(),
                api_version: "1.0.0".to_string(),
                release_notes: None,
                prerelease: false,
                deprecated: false,
                size: 1024,
                published_at: chrono::Utc::now(),
            }],
            tags: vec!["test".to_string()],
            downloads: 100,
            rating: 4.5,
            rating_count: 10,
            homepage: None,
            repository: None,
            license: "MIT".to_string(),
            icon: None,
            verified: true,
            featured: false,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    #[test]
    fn test_registry_add() {
        let mut registry = PluginRegistry::new();
        let entry = create_test_entry("test-plugin", "Test Plugin");
        registry.add(entry);

        assert_eq!(registry.count(), 1);
        assert!(registry.get("test-plugin").is_some());
    }

    #[test]
    fn test_registry_search() {
        let mut registry = PluginRegistry::new();
        registry.add(create_test_entry("plugin1", "First Plugin"));
        registry.add(create_test_entry("plugin2", "Second Plugin"));

        let search = RegistrySearch::default();
        let result = registry.search(&search);

        assert_eq!(result.plugins.len(), 2);
        assert_eq!(result.total, 2);
    }

    #[test]
    fn test_registry_by_category() {
        let mut registry = PluginRegistry::new();
        registry.add(create_test_entry("plugin1", "Test Plugin"));

        let plugins = registry.by_category("test");
        assert_eq!(plugins.len(), 1);
    }

    #[test]
    fn test_dependency_resolver() {
        let registry = PluginRegistry::new();
        let resolver = DependencyResolver::new(&registry);

        // Non-existent plugin
        let result = resolver.resolve("nonexistent");
        assert!(result.is_err());
    }
}
