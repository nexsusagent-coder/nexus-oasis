//! MCP Resource System
//!
//! Resources represent any kind of data that an MCP server wants to make
//! available to clients. This can include file contents, database records,
//! API responses, and more.

use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use std::collections::HashMap;

pub use crate::types::ResourceContents;

/// Resource metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    /// Resource URI (unique identifier)
    pub uri: String,
    /// Human-readable name
    pub name: String,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

impl Resource {
    /// Create a new resource
    pub fn new(uri: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            name: name.into(),
            description: None,
            mime_type: None,
        }
    }

    /// Add description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Add MIME type
    pub fn with_mime_type(mut self, mime_type: impl Into<String>) -> Self {
        self.mime_type = Some(mime_type.into());
        self
    }
}

/// Resource template for parameterized resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceTemplate {
    /// URI template (RFC 6570)
    pub uri_template: String,
    /// Human-readable name
    pub name: String,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

impl ResourceTemplate {
    /// Create a new resource template
    pub fn new(uri_template: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            uri_template: uri_template.into(),
            name: name.into(),
            description: None,
            mime_type: None,
        }
    }
}

/// Resource provider trait
#[async_trait]
pub trait ResourceProvider: Send + Sync {
    /// List available resources
    async fn list(&self) -> crate::Result<Vec<Resource>>;
    
    /// Read a resource by URI
    async fn read(&self, uri: &str) -> crate::Result<ResourceContents>;
    
    /// Check if resource exists
    async fn exists(&self, uri: &str) -> bool;
    
    /// Get resource metadata
    async fn metadata(&self, uri: &str) -> Option<Resource>;
    
    /// List resource templates (optional)
    async fn templates(&self) -> crate::Result<Vec<ResourceTemplate>> {
        Ok(Vec::new())
    }
}

/// File system resource provider
pub struct FileResourceProvider {
    base_path: std::path::PathBuf,
}

impl FileResourceProvider {
    /// Create a new file resource provider
    pub fn new(base_path: impl Into<std::path::PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }

    /// Resolve URI to file path
    fn resolve_path(&self, uri: &str) -> crate::Result<std::path::PathBuf> {
        // Remove "file://" prefix if present
        let path = uri.strip_prefix("file://").unwrap_or(uri);
        
        // Security: prevent path traversal
        let path = path.replace("..", "");
        
        Ok(self.base_path.join(path))
    }
}

#[async_trait]
impl ResourceProvider for FileResourceProvider {
    async fn list(&self) -> crate::Result<Vec<Resource>> {
        let mut resources = Vec::new();
        
        let mut entries = tokio::fs::read_dir(&self.base_path).await
            .map_err(|e| crate::McpError::transport(format!("Failed to read directory: {}", e)))?;
        
        while let Some(entry) = entries.next_entry().await.map_err(crate::McpError::Io)? {
            if let Some(name) = entry.file_name().to_str() {
                let uri = format!("file://{}", name);
                resources.push(Resource::new(uri, name));
            }
        }
        
        Ok(resources)
    }

    async fn read(&self, uri: &str) -> crate::Result<ResourceContents> {
        let path = self.resolve_path(uri)?;
        
        let content = tokio::fs::read(&path).await
            .map_err(|e| crate::McpError::resource_not_found(format!("{}: {}", uri, e)))?;
        
        // Try to decode as UTF-8
        let text = String::from_utf8(content.clone());
        
        let (text, blob) = match text {
            Ok(t) => (Some(t), None),
            Err(_) => (None, Some(base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &content))),
        };
        
        Ok(ResourceContents {
            uri: uri.to_string(),
            mime_type: mime_guess::from_path(&path).first().map(|m| m.to_string()),
            text,
            blob,
        })
    }

    async fn exists(&self, uri: &str) -> bool {
        if let Ok(path) = self.resolve_path(uri) {
            tokio::fs::metadata(&path).await.is_ok()
        } else {
            false
        }
    }

    async fn metadata(&self, uri: &str) -> Option<Resource> {
        if self.exists(uri).await {
            let name = uri.rsplit('/').next().unwrap_or(uri);
            Some(Resource::new(uri, name))
        } else {
            None
        }
    }
}

/// Resource manager for managing multiple resource providers
pub struct ResourceManager {
    providers: HashMap<String, Box<dyn ResourceProvider>>,
}

impl ResourceManager {
    /// Create a new resource manager
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    /// Register a resource provider
    pub fn register(&mut self, scheme: impl Into<String>, provider: Box<dyn ResourceProvider>) {
        self.providers.insert(scheme.into(), provider);
    }

    /// Get a provider by scheme
    pub fn get_provider(&self, scheme: &str) -> Option<&dyn ResourceProvider> {
        self.providers.get(scheme).map(|p| p.as_ref())
    }

    /// List all resources from all providers
    pub async fn list_all(&self) -> crate::Result<Vec<Resource>> {
        let mut all_resources = Vec::new();
        
        for provider in self.providers.values() {
            let resources = provider.list().await?;
            all_resources.extend(resources);
        }
        
        Ok(all_resources)
    }

    /// Read a resource by URI
    pub async fn read(&self, uri: &str) -> crate::Result<ResourceContents> {
        // Extract scheme from URI
        let scheme = uri.split(':').next()
            .ok_or_else(|| crate::McpError::invalid_params(format!("Invalid URI: {}", uri)))?;
        
        let provider = self.providers.get(scheme)
            .ok_or_else(|| crate::McpError::not_implemented(format!("No provider for scheme: {}", scheme)))?;
        
        provider.read(uri).await
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// List resources request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListResourcesRequest {
    /// Optional cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// List resources result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListResourcesResult {
    /// List of resources
    pub resources: Vec<Resource>,
    /// Next cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// Read resource request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadResourceRequest {
    /// Resource URI
    pub uri: String,
}

/// Read resource result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadResourceResult {
    /// Resource contents
    pub contents: Vec<ResourceContents>,
}

/// Resource updated notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUpdatedNotification {
    /// Resource URI
    pub uri: String,
}

/// Resource list changed notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceListChangedNotification {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_creation() {
        let resource = Resource::new("file://test.txt", "Test File")
            .with_description("A test file")
            .with_mime_type("text/plain");
        
        assert_eq!(resource.uri, "file://test.txt");
        assert_eq!(resource.name, "Test File");
        assert_eq!(resource.description, Some("A test file".to_string()));
    }

    #[test]
    fn test_resource_template() {
        let template = ResourceTemplate::new("file://{path}", "File by Path");
        assert_eq!(template.uri_template, "file://{path}");
    }
}
