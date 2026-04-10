//! # Sentient Plugin
//!
//! Plugin and extension system for SENTIENT OS.
//!
//! ## Features
//!
//! - **Plugin Lifecycle**: Load, initialize, execute, shutdown
//! - **Discovery**: Automatic plugin discovery from filesystem
//! - **Sandbox**: Security isolation for plugins
//! - **Registry**: Plugin marketplace and dependency management
//! - **Events**: Plugin event system
//! - **Middleware**: Request/response interception
//!
//! ## Example
//!
//! ```rust
//! use sentient_plugin::{PluginManager, Plugin, PluginManifest, PluginConfig, PluginContext};
//! use async_trait::async_trait;
//!
//! // Define a plugin
//! struct MyPlugin {
//!     manifest: PluginManifest,
//! }
//!
//! #[async_trait]
//! impl Plugin for MyPlugin {
//!     fn metadata(&self) -> &PluginManifest {
//!         &self.manifest
//!     }
//!
//!     async fn initialize(&mut self, config: &PluginConfig) -> sentient_plugin::Result<()> {
//!         Ok(())
//!     }
//!
//!     async fn shutdown(&mut self) -> sentient_plugin::Result<()> {
//!         Ok(())
//!     }
//!
//!     fn status(&self) -> sentient_plugin::PluginStatus {
//!         sentient_plugin::PluginStatus::Active
//!     }
//!
//!     async fn execute(
//!         &self,
//!         command: &str,
//!         args: serde_json::Value,
//!         context: &PluginContext,
//!     ) -> sentient_plugin::Result<serde_json::Value> {
//!         Ok(serde_json::json!({"result": "ok"}))
//!     }
//! }
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create plugin manager
//! let manager = PluginManager::default_manager();
//!
//! // Register plugin
//! // manager.register(Box::new(MyPlugin { ... })).await?;
//!
//! // Execute plugin
//! // let result = manager.execute("my-plugin", "command", json!({}), &ctx).await?;
//! # Ok(())
//! # }
//! ```

pub mod error;
pub mod types;
pub mod plugin;
pub mod manager;
pub mod loader;
pub mod sandbox;
pub mod registry;

pub use error::{PluginError, Result};
pub use types::*;
pub use plugin::{Plugin, PluginHook, PluginMiddleware, PluginEvent, EventType, ToolDefinition, ResourceDefinition, PromptDefinition, ExamplePlugin};
pub use manager::PluginManager;
pub use loader::{PluginDiscovery, PluginLoader, DiscoveryOptions, DiscoveredPlugin, PluginPathResolver};
pub use sandbox::{PluginSandbox, SandboxConfig, SandboxManager, FileAccess};
pub use registry::{PluginRegistry, RegistryEntry, RegistrySearch, RegistrySearchResult};

/// Plugin API version
pub const PLUGIN_API_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_version() {
        assert!(!PLUGIN_API_VERSION.is_empty());
    }
}
