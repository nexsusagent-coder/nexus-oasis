//! Plugin sandbox for security isolation

use crate::types::{PluginPermission, PluginManifest};
use crate::{PluginError, Result};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Sandbox configuration
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    /// Allowed paths for filesystem access
    pub allowed_paths: Vec<PathBuf>,
    /// Denied paths (blacklist)
    pub denied_paths: Vec<PathBuf>,
    /// Allowed network hosts
    pub allowed_hosts: Vec<String>,
    /// Maximum memory in bytes (0 = unlimited)
    pub max_memory: usize,
    /// Maximum execution time in seconds
    pub max_execution_time: u64,
    /// Maximum file size in bytes
    pub max_file_size: usize,
    /// Enable network access
    pub network_enabled: bool,
    /// Enable process execution
    pub process_execution: bool,
    /// Enable system info access
    pub system_info: bool,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            allowed_paths: Vec::new(),
            denied_paths: vec![
                PathBuf::from("/etc/passwd"),
                PathBuf::from("/etc/shadow"),
                PathBuf::from("/root"),
            ],
            allowed_hosts: Vec::new(),
            max_memory: 256 * 1024 * 1024, // 256MB
            max_execution_time: 30,         // 30 seconds
            max_file_size: 10 * 1024 * 1024, // 10MB
            network_enabled: false,
            process_execution: false,
            system_info: false,
        }
    }
}

impl SandboxConfig {
    /// Create strict sandbox
    pub fn strict() -> Self {
        Self {
            allowed_paths: Vec::new(),
            denied_paths: vec![
                PathBuf::from("/etc"),
                PathBuf::from("/root"),
                PathBuf::from("/home"),
                PathBuf::from("/var"),
            ],
            allowed_hosts: Vec::new(),
            max_memory: 64 * 1024 * 1024,  // 64MB
            max_execution_time: 10,         // 10 seconds
            max_file_size: 1 * 1024 * 1024, // 1MB
            network_enabled: false,
            process_execution: false,
            system_info: false,
        }
    }

    /// Create permissive sandbox
    pub fn permissive() -> Self {
        Self {
            allowed_paths: vec![PathBuf::from("/")],
            denied_paths: Vec::new(),
            allowed_hosts: vec!["*".to_string()],
            max_memory: 0, // unlimited
            max_execution_time: 300,
            max_file_size: 100 * 1024 * 1024, // 100MB
            network_enabled: true,
            process_execution: true,
            system_info: true,
        }
    }

    /// Add allowed path
    pub fn allow_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.allowed_paths.push(path.into());
        self
    }

    /// Add denied path
    pub fn deny_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.denied_paths.push(path.into());
        self
    }

    /// Add allowed host
    pub fn allow_host(mut self, host: impl Into<String>) -> Self {
        self.allowed_hosts.push(host.into());
        self
    }

    /// Create from permissions
    pub fn from_permissions(permissions: &[PluginPermission]) -> Self {
        let mut config = Self::default();

        for perm in permissions {
            match perm {
                PluginPermission::FilesystemFull => {
                    config.allowed_paths.push(PathBuf::from("/"));
                }
                PluginPermission::FilesystemRead => {
                    // Read-only handled separately
                }
                PluginPermission::FilesystemWrite => {
                    // Write handled separately
                }
                PluginPermission::Network => {
                    config.network_enabled = true;
                }
                PluginPermission::ProcessExecution => {
                    config.process_execution = true;
                }
                PluginPermission::SystemInfo => {
                    config.system_info = true;
                }
                _ => {}
            }
        }

        config
    }
}

/// Plugin sandbox
pub struct PluginSandbox {
    config: SandboxConfig,
    permissions: HashSet<PluginPermission>,
    plugin_id: String,
    plugin_data_dir: PathBuf,
}

impl PluginSandbox {
    /// Create new sandbox
    pub fn new(plugin_id: impl Into<String>, config: SandboxConfig) -> Self {
        let plugin_id = plugin_id.into();
        let plugin_data_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("sentient")
            .join("plugins")
            .join(&plugin_id)
            .join("data");

        Self {
            config,
            permissions: HashSet::new(),
            plugin_id,
            plugin_data_dir,
        }
    }

    /// Create sandbox from manifest
    pub fn from_manifest(manifest: &PluginManifest) -> Self {
        let config = SandboxConfig::from_permissions(&manifest.permissions);
        let permissions: HashSet<PluginPermission> = manifest.permissions.iter().cloned().collect();

        Self {
            config,
            permissions,
            plugin_id: manifest.id.clone(),
            plugin_data_dir: dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("sentient")
                .join("plugins")
                .join(&manifest.id)
                .join("data"),
        }
    }

    /// Get plugin data directory
    pub fn data_dir(&self) -> &Path {
        &self.plugin_data_dir
    }

    /// Ensure data directory exists
    pub fn ensure_data_dir(&self) -> Result<()> {
        std::fs::create_dir_all(&self.plugin_data_dir)?;
        Ok(())
    }

    /// Check if path access is allowed
    pub fn check_path_access(&self, path: &Path, access: FileAccess) -> Result<()> {
        // Check denied paths first
        for denied in &self.config.denied_paths {
            if path.starts_with(denied) {
                return Err(PluginError::sandbox_violation(format!(
                    "Access to {:?} is denied",
                    path
                )));
            }
        }

        // Check allowed paths
        let has_full_access = self.permissions.contains(&PluginPermission::FilesystemFull);
        let has_read = self.permissions.contains(&PluginPermission::FilesystemRead);
        let has_write = self.permissions.contains(&PluginPermission::FilesystemWrite);

        match access {
            FileAccess::Read => {
                if !has_full_access && !has_read && !self.is_in_allowed_paths(path) {
                    return Err(PluginError::sandbox_violation(format!(
                        "Read access to {:?} not allowed",
                        path
                    )));
                }
            }
            FileAccess::Write => {
                if !has_full_access && !has_write && !self.is_in_allowed_paths(path) {
                    return Err(PluginError::sandbox_violation(format!(
                        "Write access to {:?} not allowed",
                        path
                    )));
                }
            }
        }

        Ok(())
    }

    fn is_in_allowed_paths(&self, path: &Path) -> bool {
        // Plugin data dir is always allowed
        if path.starts_with(&self.plugin_data_dir) {
            return true;
        }

        for allowed in &self.config.allowed_paths {
            if path.starts_with(allowed) {
                return true;
            }
        }

        false
    }

    /// Check network access
    pub fn check_network_access(&self, host: &str) -> Result<()> {
        if !self.config.network_enabled && !self.permissions.contains(&PluginPermission::Network) {
            return Err(PluginError::sandbox_violation("Network access not allowed"));
        }

        if !self.config.allowed_hosts.is_empty() {
            let allowed = self.config.allowed_hosts.iter().any(|h| {
                h == "*" || h == host || host.ends_with(&format!(".{}", h))
            });

            if !allowed {
                return Err(PluginError::sandbox_violation(format!(
                    "Access to host '{}' not allowed",
                    host
                )));
            }
        }

        Ok(())
    }

    /// Check process execution
    pub fn check_process_execution(&self) -> Result<()> {
        if !self.config.process_execution && !self.permissions.contains(&PluginPermission::ProcessExecution) {
            return Err(PluginError::sandbox_violation("Process execution not allowed"));
        }
        Ok(())
    }

    /// Check memory limit
    pub fn check_memory(&self, requested: usize) -> Result<()> {
        if self.config.max_memory > 0 && requested > self.config.max_memory {
            return Err(PluginError::sandbox_violation(format!(
                "Memory limit exceeded: {} > {}",
                requested, self.config.max_memory
            )));
        }
        Ok(())
    }

    /// Get execution timeout
    pub fn execution_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.config.max_execution_time)
    }

    /// Check file size
    pub fn check_file_size(&self, size: usize) -> Result<()> {
        if size > self.config.max_file_size {
            return Err(PluginError::sandbox_violation(format!(
                "File size limit exceeded: {} > {}",
                size, self.config.max_file_size
            )));
        }
        Ok(())
    }

    /// Get sandbox config
    pub fn config(&self) -> &SandboxConfig {
        &self.config
    }

    /// Check if permission is granted
    pub fn has_permission(&self, permission: &PluginPermission) -> bool {
        self.permissions.contains(permission)
    }
}

/// File access type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileAccess {
    Read,
    Write,
}

/// Sandbox manager for all plugins
pub struct SandboxManager {
    sandboxes: std::collections::HashMap<String, PluginSandbox>,
    default_config: SandboxConfig,
}

impl SandboxManager {
    pub fn new() -> Self {
        Self {
            sandboxes: std::collections::HashMap::new(),
            default_config: SandboxConfig::default(),
        }
    }

    /// Create sandbox for plugin
    pub fn create_sandbox(&mut self, manifest: &PluginManifest) -> Result<&PluginSandbox> {
        let sandbox = PluginSandbox::from_manifest(manifest);
        self.sandboxes.insert(manifest.id.clone(), sandbox);
        Ok(self.sandboxes.get(&manifest.id).unwrap())
    }

    /// Get sandbox for plugin
    pub fn get(&self, plugin_id: &str) -> Option<&PluginSandbox> {
        self.sandboxes.get(plugin_id)
    }

    /// Remove sandbox
    pub fn remove(&mut self, plugin_id: &str) -> Option<PluginSandbox> {
        self.sandboxes.remove(plugin_id)
    }

    /// Set default config
    pub fn set_default_config(&mut self, config: SandboxConfig) {
        self.default_config = config;
    }

    /// Get default config
    pub fn default_config(&self) -> &SandboxConfig {
        &self.default_config
    }
}

impl Default for SandboxManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_config_default() {
        let config = SandboxConfig::default();
        assert_eq!(config.max_memory, 256 * 1024 * 1024);
        assert!(!config.network_enabled);
    }

    #[test]
    fn test_sandbox_config_strict() {
        let config = SandboxConfig::strict();
        assert!(!config.network_enabled);
        assert!(!config.process_execution);
    }

    #[test]
    fn test_sandbox_path_check() {
        let sandbox = PluginSandbox::new("test-plugin", SandboxConfig::default());

        // Denied path
        let result = sandbox.check_path_access(Path::new("/etc/passwd"), FileAccess::Read);
        assert!(result.is_err());
    }

    #[test]
    fn test_sandbox_network_check() {
        let sandbox = PluginSandbox::new("test-plugin", SandboxConfig::strict());

        let result = sandbox.check_network_access("example.com");
        assert!(result.is_err());
    }

    #[test]
    fn test_sandbox_from_manifest() {
        let manifest = PluginManifest {
            id: "test".to_string(),
            name: "Test".to_string(),
            version: "1.0.0".to_string(),
            description: "Test".to_string(),
            author: "Test".to_string(),
            license: "MIT".to_string(),
            api_version: "1.0.0".to_string(),
            entry_point: "lib.so".to_string(),
            permissions: vec![PluginPermission::Network],
            ..Default::default()
        };

        let sandbox = PluginSandbox::from_manifest(&manifest);
        assert!(sandbox.has_permission(&PluginPermission::Network));
    }
}
