//! Plugin discovery and loading

use crate::types::*;
use crate::{PluginError, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Plugin discovery options
#[derive(Debug, Clone)]
pub struct DiscoveryOptions {
    /// Search directories
    pub directories: Vec<PathBuf>,
    /// Include system plugins
    pub include_system: bool,
    /// Include user plugins
    pub include_user: bool,
    /// Max depth for directory traversal
    pub max_depth: usize,
    /// File patterns to match
    pub patterns: Vec<String>,
}

impl Default for DiscoveryOptions {
    fn default() -> Self {
        let mut directories = Vec::new();

        // User plugins
        if let Some(user_dir) = dirs::data_local_dir() {
            directories.push(user_dir.join("sentient").join("plugins"));
        }

        // System plugins
        #[cfg(unix)]
        directories.push(PathBuf::from("/usr/lib/sentient/plugins"));

        #[cfg(windows)]
        directories.push(PathBuf::from("C:\\Program Files\\SENTIENT\\plugins"));

        // Current directory
        directories.push(PathBuf::from("plugins"));

        Self {
            directories,
            include_system: true,
            include_user: true,
            max_depth: 3,
            patterns: vec!["plugin.json".to_string()],
        }
    }
}

impl DiscoveryOptions {
    /// Create with specific directories
    pub fn with_dirs(dirs: Vec<PathBuf>) -> Self {
        Self {
            directories: dirs,
            ..Default::default()
        }
    }

    /// Add directory
    pub fn add_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.directories.push(dir.into());
        self
    }
}

/// Discovered plugin
#[derive(Debug, Clone)]
pub struct DiscoveredPlugin {
    /// Plugin manifest
    pub manifest: PluginManifest,
    /// Path to plugin directory
    pub path: PathBuf,
    /// Path to manifest file
    pub manifest_path: PathBuf,
}

/// Plugin discovery
pub struct PluginDiscovery {
    options: DiscoveryOptions,
}

impl PluginDiscovery {
    /// Create new discovery
    pub fn new(options: DiscoveryOptions) -> Self {
        Self { options }
    }

    /// Create with default options
    pub fn default_discovery() -> Self {
        Self::new(DiscoveryOptions::default())
    }

    /// Discover plugins in all configured directories
    pub fn discover(&self) -> Result<Vec<DiscoveredPlugin>> {
        let mut plugins = Vec::new();

        for dir in &self.options.directories {
            if !dir.exists() {
                tracing::debug!("Plugin directory does not exist: {:?}", dir);
                continue;
            }

            let discovered = self.discover_in_dir(dir)?;
            plugins.extend(discovered);
        }

        Ok(plugins)
    }

    /// Discover plugins in a specific directory
    pub fn discover_in_dir(&self, dir: &Path) -> Result<Vec<DiscoveredPlugin>> {
        let mut plugins = Vec::new();

        for entry in WalkDir::new(dir)
            .max_depth(self.options.max_depth)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            // Check if it matches our patterns
            let file_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");

            if self.options.patterns.iter().any(|p| p == file_name) {
                if let Ok(plugin) = self.load_manifest(path) {
                    plugins.push(plugin);
                }
            }
        }

        Ok(plugins)
    }

    /// Load manifest from file
    fn load_manifest(&self, path: &Path) -> Result<DiscoveredPlugin> {
        let manifest = PluginManifest::from_file(path)?;
        let plugin_dir = path.parent()
            .ok_or_else(|| PluginError::invalid_manifest("No parent directory"))?
            .to_path_buf();

        Ok(DiscoveredPlugin {
            manifest,
            path: plugin_dir,
            manifest_path: path.to_path_buf(),
        })
    }

    /// Get discovery directories
    pub fn directories(&self) -> &[PathBuf] {
        &self.options.directories
    }
}

/// Plugin loader interface
pub trait PluginLoader: Send + Sync {
    /// Load plugin from discovered location
    fn load(&self, discovered: &DiscoveredPlugin) -> Result<Box<dyn crate::Plugin>>;
    
    /// Check if this loader can handle the plugin type
    fn can_load(&self, plugin: &DiscoveredPlugin) -> bool;
    
    /// Loader name
    fn name(&self) -> &str;
}

/// Built-in plugin loader (for internal plugins)
pub struct BuiltinLoader;

impl PluginLoader for BuiltinLoader {
    fn load(&self, _discovered: &DiscoveredPlugin) -> Result<Box<dyn crate::Plugin>> {
        // Built-in plugins are registered directly, not loaded
        Err(PluginError::load_failed("builtin", "Cannot load built-in plugins dynamically"))
    }

    fn can_load(&self, plugin: &DiscoveredPlugin) -> bool {
        plugin.manifest.entry_point == "builtin"
    }

    fn name(&self) -> &str {
        "builtin"
    }
}

/// Plugin path resolver
pub struct PluginPathResolver {
    base_dirs: Vec<PathBuf>,
}

impl PluginPathResolver {
    pub fn new() -> Self {
        let mut base_dirs = Vec::new();

        if let Some(user_dir) = dirs::data_local_dir() {
            base_dirs.push(user_dir.join("sentient").join("plugins"));
        }

        base_dirs.push(PathBuf::from("plugins"));

        Self { base_dirs }
    }

    /// Resolve plugin path by ID
    pub fn resolve(&self, plugin_id: &str) -> Option<PathBuf> {
        for base in &self.base_dirs {
            let plugin_dir = base.join(plugin_id);
            if plugin_dir.exists() {
                return Some(plugin_dir);
            }
        }
        None
    }

    /// Get manifest path for plugin
    pub fn manifest_path(&self, plugin_id: &str) -> Option<PathBuf> {
        self.resolve(plugin_id).map(|p| p.join("plugin.json"))
    }

    /// Get all plugin directories
    pub fn plugin_dirs(&self) -> &[PathBuf] {
        &self.base_dirs
    }

    /// Ensure plugin directory exists
    pub fn ensure_plugin_dir(&self) -> Result<PathBuf> {
        let dir = self.base_dirs.first()
            .ok_or_else(|| PluginError::invalid_manifest("No plugin directory configured"))?;

        std::fs::create_dir_all(dir)?;
        Ok(dir.clone())
    }
}

impl Default for PluginPathResolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Plugin installer
#[cfg(feature = "download")]
pub struct PluginInstaller {
    client: reqwest::Client,
    download_dir: PathBuf,
}

#[cfg(feature = "download")]
impl PluginInstaller {
    pub fn new(download_dir: impl Into<PathBuf>) -> Self {
        Self {
            client: reqwest::Client::new(),
            download_dir: download_dir.into(),
        }
    }

    /// Download plugin from URL
    pub async fn download(&self, url: &str) -> Result<PathBuf> {
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(PluginError::Download(format!("HTTP {}", response.status())));
        }

        let filename = url.split('/').last().unwrap_or("plugin.zip");
        let dest_path = self.download_dir.join(filename);

        std::fs::create_dir_all(&self.download_dir)?;

        let bytes = response.bytes().await?;
        std::fs::write(&dest_path, &bytes)?;

        Ok(dest_path)
    }

    /// Install plugin from archive
    pub fn install(&self, archive_path: &Path, plugin_id: &str) -> Result<PathBuf> {
        let reader = std::fs::File::open(archive_path)?;
        let mut archive = zip::ZipArchive::new(reader)?;

        let install_dir = self.download_dir.join(plugin_id);
        std::fs::create_dir_all(&install_dir)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = install_dir.join(file.name());

            if file.is_dir() {
                std::fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    std::fs::create_dir_all(p)?;
                }
                let mut outfile = std::fs::File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }

        Ok(install_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discovery_options_default() {
        let opts = DiscoveryOptions::default();
        assert!(!opts.directories.is_empty());
    }

    #[test]
    fn test_plugin_discovery_creation() {
        let discovery = PluginDiscovery::default_discovery();
        assert!(!discovery.options.directories.is_empty());
    }

    #[test]
    fn test_path_resolver() {
        let resolver = PluginPathResolver::new();
        assert!(!resolver.plugin_dirs().is_empty());
    }

    #[test]
    fn test_builtin_loader() {
        let loader = BuiltinLoader;
        assert_eq!(loader.name(), "builtin");
    }
}
