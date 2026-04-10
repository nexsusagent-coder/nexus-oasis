//! Plugin manager - loads, manages, and orchestrates plugins

use crate::plugin::{Plugin, PluginHook, PluginMiddleware, PluginEvent};
use crate::types::*;
use crate::{PluginError, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Plugin manager
pub struct PluginManager {
    /// Loaded plugins
    plugins: Arc<RwLock<HashMap<String, Box<dyn Plugin>>>>,
    /// Plugin info
    info: Arc<RwLock<HashMap<String, PluginInfo>>>,
    /// Plugin configs
    configs: Arc<RwLock<HashMap<String, PluginConfig>>>,
    /// Plugin hooks
    hooks: Arc<RwLock<HashMap<String, Vec<Box<dyn PluginHook>>>>>,
    /// Plugin middleware
    middleware: Arc<RwLock<Vec<Box<dyn PluginMiddleware>>>>,
    /// Plugin directory
    plugin_dir: PathBuf,
    /// API version
    api_version: String,
}

impl PluginManager {
    /// Create new plugin manager
    pub fn new(plugin_dir: impl Into<PathBuf>) -> Self {
        Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
            info: Arc::new(RwLock::new(HashMap::new())),
            configs: Arc::new(RwLock::new(HashMap::new())),
            hooks: Arc::new(RwLock::new(HashMap::new())),
            middleware: Arc::new(RwLock::new(Vec::new())),
            plugin_dir: plugin_dir.into(),
            api_version: PLUGIN_API_VERSION.to_string(),
        }
    }

    /// Create with default plugin directory
    pub fn default_manager() -> Self {
        let plugin_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("sentient")
            .join("plugins");

        Self::new(plugin_dir)
    }

    /// Set API version
    pub fn with_api_version(mut self, version: impl Into<String>) -> Self {
        self.api_version = version.into();
        self
    }

    /// Get plugin directory
    pub fn plugin_dir(&self) -> &Path {
        &self.plugin_dir
    }

    /// Register a plugin directly
    pub async fn register(&self, plugin: Box<dyn Plugin>) -> Result<()> {
        let manifest = plugin.metadata().clone();
        let id = manifest.id.clone();
        let path = PathBuf::from(&manifest.entry_point);

        // Check if already loaded
        {
            let plugins = self.plugins.read().await;
            if plugins.contains_key(&id) {
                return Err(PluginError::AlreadyLoaded(id));
            }
        }

        // Get config
        let config = {
            let configs = self.configs.read().await;
            configs.get(&id).cloned().unwrap_or_else(|| PluginConfig::new(&id))
        };

        // Create info
        let mut info = PluginInfo::new(manifest.clone(), path);
        info.set_loading();

        {
            let mut info_map = self.info.write().await;
            info_map.insert(id.clone(), info);
        }

        // Initialize plugin
        let mut plugin = plugin;
        if let Err(e) = plugin.initialize(&config).await {
            let mut info_map = self.info.write().await;
            if let Some(info) = info_map.get_mut(&id) {
                info.set_error(e.to_string());
            }
            return Err(e);
        }

        // Store plugin
        {
            let mut plugins = self.plugins.write().await;
            plugins.insert(id.clone(), plugin);
        }

        // Update info
        {
            let mut info_map = self.info.write().await;
            if let Some(info) = info_map.get_mut(&id) {
                info.set_active();
            }
        }

        // Store config
        {
            let mut configs = self.configs.write().await;
            configs.insert(id.clone(), config);
        }

        tracing::info!("Plugin '{}' registered successfully", id);
        Ok(())
    }

    /// Unregister a plugin
    pub async fn unregister(&self, id: &str) -> Result<()> {
        let mut plugin = {
            let mut plugins = self.plugins.write().await;
            plugins.remove(id).ok_or_else(|| PluginError::not_found(id))?
        };

        // Shutdown plugin
        if let Err(e) = plugin.shutdown().await {
            tracing::error!("Failed to shutdown plugin '{}': {}", id, e);
        }

        // Update info
        {
            let mut info_map = self.info.write().await;
            if let Some(info) = info_map.get_mut(id) {
                info.set_unloaded();
            }
        }

        tracing::info!("Plugin '{}' unregistered", id);
        Ok(())
    }

    /// Get plugin by ID
    pub async fn get(&self, id: &str) -> Option<Arc<dyn Plugin>> {
        let plugins = self.plugins.read().await;
        // Note: We return a reference since we can't clone Box<dyn Plugin>
        // In production, we'd use Arc<dyn Plugin> instead
        None
    }

    /// Check if plugin is loaded
    pub async fn is_loaded(&self, id: &str) -> bool {
        let plugins = self.plugins.read().await;
        plugins.contains_key(id)
    }

    /// Get plugin info
    pub async fn get_info(&self, id: &str) -> Option<PluginInfo> {
        let info = self.info.read().await;
        info.get(id).cloned()
    }

    /// List all plugins
    pub async fn list(&self) -> Vec<PluginInfo> {
        let info = self.info.read().await;
        info.values().cloned().collect()
    }

    /// List active plugins
    pub async fn list_active(&self) -> Vec<PluginInfo> {
        let info = self.info.read().await;
        info.values()
            .filter(|i| i.is_active())
            .cloned()
            .collect()
    }

    /// Execute plugin command
    pub async fn execute(
        &self,
        plugin_id: &str,
        command: &str,
        args: serde_json::Value,
        context: &PluginContext,
    ) -> Result<serde_json::Value> {
        let mut args = args;

        // Run middleware before
        {
            let middleware = self.middleware.read().await;
            for mw in middleware.iter() {
                mw.before(context, command, &mut args).await?;
            }
        }

        // Execute plugin
        let result = {
            let plugins = self.plugins.read().await;
            let plugin = plugins.get(plugin_id)
                .ok_or_else(|| PluginError::not_found(plugin_id))?;

            plugin.execute(command, args, context).await
        };

        // Run middleware after
        let mut result = result;
        {
            let middleware = self.middleware.read().await;
            for mw in middleware.iter() {
                mw.after(context, &mut result).await;
            }
        }

        // Update metrics
        {
            let mut info = self.info.write().await;
            if let Some(info) = info.get_mut(plugin_id) {
                info.metrics.invocations += 1;
                info.metrics.last_invocation = Some(chrono::Utc::now());
            }
        }

        result
    }

    /// Get all tools from active plugins
    pub async fn get_all_tools(&self) -> Vec<(String, crate::plugin::ToolDefinition)> {
        let plugins = self.plugins.read().await;
        let mut tools = Vec::new();

        for (id, plugin) in plugins.iter() {
            for tool in plugin.tools() {
                tools.push((id.clone(), tool));
            }
        }

        tools
    }

    /// Get all resources from active plugins
    pub async fn get_all_resources(&self) -> Vec<(String, crate::plugin::ResourceDefinition)> {
        let plugins = self.plugins.read().await;
        let mut resources = Vec::new();

        for (id, plugin) in plugins.iter() {
            for resource in plugin.resources() {
                resources.push((id.clone(), resource));
            }
        }

        resources
    }

    /// Get all prompts from active plugins
    pub async fn get_all_prompts(&self) -> Vec<(String, crate::plugin::PromptDefinition)> {
        let plugins = self.plugins.read().await;
        let mut prompts = Vec::new();

        for (id, plugin) in plugins.iter() {
            for prompt in plugin.prompts() {
                prompts.push((id.clone(), prompt));
            }
        }

        prompts
    }

    /// Emit event to all plugins
    pub async fn emit_event(&self, event: &PluginEvent) -> Result<()> {
        let plugins = self.plugins.read().await;

        for (id, plugin) in plugins.iter() {
            if let Err(e) = plugin.on_event(event).await {
                tracing::error!("Plugin '{}' failed to handle event: {}", id, e);
            }
        }

        Ok(())
    }

    /// Add middleware
    pub async fn add_middleware(&self, middleware: Box<dyn PluginMiddleware>) {
        let mut mw = self.middleware.write().await;
        mw.push(middleware);
    }

    /// Set plugin config
    pub async fn set_config(&self, id: &str, config: PluginConfig) -> Result<()> {
        let plugins = self.plugins.read().await;
        let plugin = plugins.get(id)
            .ok_or_else(|| PluginError::not_found(id))?;

        // Get old config
        let old_config = {
            let configs = self.configs.read().await;
            configs.get(id).cloned().unwrap_or_else(|| PluginConfig::new(id))
        };

        // Notify plugin of config change
        // Note: In production, we'd need mutable access to the plugin
        // For now, we just update the stored config
        {
            let mut configs = self.configs.write().await;
            configs.insert(id.to_string(), config);
        }

        Ok(())
    }

    /// Get plugin config
    pub async fn get_config(&self, id: &str) -> Option<PluginConfig> {
        let configs = self.configs.read().await;
        configs.get(id).cloned()
    }

    /// Enable plugin
    pub async fn enable(&self, id: &str) -> Result<()> {
        let mut info = self.info.write().await;
        let info = info.get_mut(id)
            .ok_or_else(|| PluginError::not_found(id))?;

        info.manifest.enabled = true;
        Ok(())
    }

    /// Disable plugin
    pub async fn disable(&self, id: &str) -> Result<()> {
        let mut info = self.info.write().await;
        let info = info.get_mut(id)
            .ok_or_else(|| PluginError::not_found(id))?;

        info.manifest.enabled = false;
        info.set_disabled();
        Ok(())
    }

    /// Health check all plugins
    pub async fn health_check(&self) -> HashMap<String, bool> {
        let plugins = self.plugins.read().await;
        let mut results = HashMap::new();

        for (id, plugin) in plugins.iter() {
            match plugin.health_check().await {
                Ok(healthy) => {
                    results.insert(id.clone(), healthy);
                }
                Err(e) => {
                    tracing::error!("Health check failed for '{}': {}", id, e);
                    results.insert(id.clone(), false);
                }
            }
        }

        results
    }

    /// Shutdown all plugins
    pub async fn shutdown(&self) -> Result<()> {
        let plugin_ids: Vec<String> = {
            let plugins = self.plugins.read().await;
            plugins.keys().cloned().collect()
        };

        for id in plugin_ids {
            if let Err(e) = self.unregister(&id).await {
                tracing::error!("Failed to shutdown plugin '{}': {}", id, e);
            }
        }

        Ok(())
    }

    /// Get plugin count
    pub async fn count(&self) -> usize {
        let plugins = self.plugins.read().await;
        plugins.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::ExamplePlugin;

    #[tokio::test]
    async fn test_plugin_manager_creation() {
        let manager = PluginManager::default_manager();
        assert!(manager.plugin_dir().to_string_lossy().contains("plugins"));
    }

    #[tokio::test]
    async fn test_register_plugin() {
        let manager = PluginManager::new("/tmp/test-plugins");
        let plugin = Box::new(ExamplePlugin::new());

        let result = manager.register(plugin).await;
        assert!(result.is_ok());

        let count = manager.count().await;
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_list_plugins() {
        let manager = PluginManager::new("/tmp/test-plugins");
        manager.register(Box::new(ExamplePlugin::new())).await.unwrap();

        let plugins = manager.list().await;
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0].manifest.id, "sentient-example");
    }

    #[tokio::test]
    async fn test_execute_plugin() {
        let manager = PluginManager::new("/tmp/test-plugins");
        manager.register(Box::new(ExamplePlugin::new())).await.unwrap();

        let ctx = PluginContext::new();
        let result = manager.execute(
            "sentient-example",
            "version",
            serde_json::json!({}),
            &ctx,
        ).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap()["version"], "1.0.0");
    }

    #[tokio::test]
    async fn test_get_tools() {
        let manager = PluginManager::new("/tmp/test-plugins");
        manager.register(Box::new(ExamplePlugin::new())).await.unwrap();

        let tools = manager.get_all_tools().await;
        assert!(!tools.is_empty());
        assert_eq!(tools[0].1.name, "example_echo");
    }
}
