//! Plugin type definitions

use serde::{Deserialize, Serialize};
use semver::Version;
use std::collections::HashMap;
use std::path::PathBuf;

/// Plugin API version
pub const PLUGIN_API_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Plugin manifest (plugin.json)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PluginManifest {
    /// Plugin ID (unique identifier)
    pub id: String,
    /// Plugin name (display name)
    pub name: String,
    /// Plugin version
    pub version: String,
    /// Plugin description
    pub description: String,
    /// Plugin author
    pub author: String,
    /// Plugin homepage URL
    #[serde(default)]
    pub homepage: Option<String>,
    /// Plugin repository URL
    #[serde(default)]
    pub repository: Option<String>,
    /// Plugin license
    pub license: String,
    /// Minimum SENTIENT OS version required
    #[serde(default)]
    pub min_os_version: Option<String>,
    /// Required API version
    #[serde(default)]
    pub api_version: String,
    /// Plugin entry point (dynamic library or WASM)
    pub entry_point: String,
    /// Plugin type
    #[serde(default)]
    pub plugin_type: PluginType,
    /// Plugin capabilities
    #[serde(default)]
    pub capabilities: Vec<PluginCapability>,
    /// Plugin dependencies
    #[serde(default)]
    pub dependencies: Vec<PluginDependency>,
    /// Plugin configuration schema
    #[serde(default)]
    pub config_schema: Option<serde_json::Value>,
    /// Plugin permissions
    #[serde(default)]
    pub permissions: Vec<PluginPermission>,
    /// Plugin icon (base64 or URL)
    #[serde(default)]
    pub icon: Option<String>,
    /// Plugin tags
    #[serde(default)]
    pub tags: Vec<String>,
    /// Whether plugin is enabled by default
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Plugin priority (lower = higher priority)
    #[serde(default)]
    pub priority: u32,
}

fn default_true() -> bool { true }

impl PluginManifest {
    /// Load manifest from file
    pub fn from_file(path: &std::path::Path) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let manifest: Self = serde_json::from_str(&content)?;
        manifest.validate()?;
        Ok(manifest)
    }

    /// Load manifest from JSON string
    pub fn from_json(json: &str) -> crate::Result<Self> {
        let manifest: Self = serde_json::from_str(json)?;
        manifest.validate()?;
        Ok(manifest)
    }

    /// Validate manifest
    pub fn validate(&self) -> crate::Result<()> {
        if self.id.is_empty() {
            return Err(crate::PluginError::invalid_manifest("Plugin ID is required"));
        }
        if self.name.is_empty() {
            return Err(crate::PluginError::invalid_manifest("Plugin name is required"));
        }
        if self.version.is_empty() {
            return Err(crate::PluginError::invalid_manifest("Plugin version is required"));
        }

        // Validate version format
        if Version::parse(&self.version).is_err() {
            return Err(crate::PluginError::invalid_manifest(
                format!("Invalid version format: {}", self.version)
            ));
        }

        Ok(())
    }

    /// Check if compatible with given API version
    pub fn is_compatible(&self, api_version: &str) -> bool {
        if self.api_version.is_empty() {
            return true;
        }

        let required = match Version::parse(&self.api_version) {
            Ok(v) => v,
            Err(_) => return false,
        };
        
        let found = match Version::parse(api_version) {
            Ok(v) => v,
            Err(_) => return false,
        };
        
        found >= required
    }

    /// Get semantic version
    pub fn semver(&self) -> Option<Version> {
        Version::parse(&self.version).ok()
    }
}

/// Plugin type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PluginType {
    /// Native dynamic library (.so, .dll, .dylib)
    #[default]
    Native,
    /// WebAssembly module (.wasm)
    Wasm,
    /// Script-based (Lua, JavaScript)
    Script,
    /// Hybrid (native + WASM)
    Hybrid,
}

/// Plugin capability
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginCapability {
    /// Can provide tools
    Tools,
    /// Can provide resources
    Resources,
    /// Can provide prompts
    Prompts,
    /// Can hook into events
    Events,
    /// Can extend UI
    Ui,
    /// Can add commands
    Commands,
    /// Can add settings
    Settings,
    /// Can add skills
    Skills,
    /// Can modify memory
    Memory,
    /// Can access network
    Network,
    /// Can access filesystem
    Filesystem,
}

/// Plugin dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    /// Dependency plugin ID
    pub id: String,
    /// Minimum version required
    #[serde(default)]
    pub version: Option<String>,
    /// Whether dependency is optional
    #[serde(default)]
    pub optional: bool,
}

impl PluginDependency {
    pub fn required(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            version: None,
            optional: false,
        }
    }

    pub fn optional(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            version: None,
            optional: true,
        }
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }
}

/// Plugin permission
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginPermission {
    /// Full filesystem access
    FilesystemFull,
    /// Read-only filesystem access
    FilesystemRead,
    /// Write-only filesystem access
    FilesystemWrite,
    /// Network access
    Network,
    /// Process execution
    ProcessExecution,
    /// Memory access
    MemoryAccess,
    /// System information
    SystemInfo,
    /// Plugin management
    PluginManagement,
    /// Configuration access
    ConfigAccess,
    /// User data access
    UserData,
}

/// Plugin status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum PluginStatus {
    /// Plugin is not loaded
    #[default]
    Unloaded,
    /// Plugin is loading
    Loading,
    /// Plugin is loaded and active
    Active,
    /// Plugin is loaded but disabled
    Disabled,
    /// Plugin failed to load
    Error,
    /// Plugin is being unloaded
    Unloading,
}

/// Plugin instance info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    /// Plugin manifest
    pub manifest: PluginManifest,
    /// Current status
    pub status: PluginStatus,
    /// Plugin path
    pub path: PathBuf,
    /// Load time
    pub loaded_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Error message if status is Error
    pub error: Option<String>,
    /// Plugin metrics
    #[serde(default)]
    pub metrics: PluginMetrics,
}

/// Plugin runtime metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PluginMetrics {
    /// Number of times plugin was invoked
    pub invocations: u64,
    /// Total execution time in milliseconds
    pub total_time_ms: u64,
    /// Average execution time in milliseconds
    pub avg_time_ms: f64,
    /// Number of errors
    pub errors: u64,
    /// Last invocation time
    pub last_invocation: Option<chrono::DateTime<chrono::Utc>>,
}

impl PluginInfo {
    pub fn new(manifest: PluginManifest, path: PathBuf) -> Self {
        Self {
            manifest,
            status: PluginStatus::Unloaded,
            path,
            loaded_at: None,
            error: None,
            metrics: PluginMetrics::default(),
        }
    }

    /// Mark as loading
    pub fn set_loading(&mut self) {
        self.status = PluginStatus::Loading;
        self.error = None;
    }

    /// Mark as active
    pub fn set_active(&mut self) {
        self.status = PluginStatus::Active;
        self.loaded_at = Some(chrono::Utc::now());
        self.error = None;
    }

    /// Mark as error
    pub fn set_error(&mut self, error: impl Into<String>) {
        self.status = PluginStatus::Error;
        self.error = Some(error.into());
    }

    /// Mark as disabled
    pub fn set_disabled(&mut self) {
        self.status = PluginStatus::Disabled;
    }

    /// Mark as unloaded
    pub fn set_unloaded(&mut self) {
        self.status = PluginStatus::Unloaded;
        self.loaded_at = None;
    }

    /// Check if plugin is active
    pub fn is_active(&self) -> bool {
        self.status == PluginStatus::Active
    }
}

/// Plugin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    /// Plugin ID
    pub plugin_id: String,
    /// Configuration values
    pub values: HashMap<String, serde_json::Value>,
    /// Whether plugin is enabled
    pub enabled: bool,
}

impl PluginConfig {
    pub fn new(plugin_id: impl Into<String>) -> Self {
        Self {
            plugin_id: plugin_id.into(),
            values: HashMap::new(),
            enabled: true,
        }
    }

    pub fn set(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.values.insert(key.into(), value);
        self
    }

    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.values.get(key)
    }
}

/// Plugin execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginContext {
    /// Request ID
    pub request_id: String,
    /// User ID (if authenticated)
    pub user_id: Option<String>,
    /// Session ID
    pub session_id: Option<String>,
    /// Configuration
    pub config: HashMap<String, serde_json::Value>,
    /// Additional metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl PluginContext {
    pub fn new() -> Self {
        Self {
            request_id: uuid::Uuid::new_v4().to_string(),
            user_id: None,
            session_id: None,
            config: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_user(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    pub fn with_session(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    pub fn with_config(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.config.insert(key.into(), value);
        self
    }
}

impl Default for PluginContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Plugin execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginResult {
    /// Whether execution was successful
    pub success: bool,
    /// Result data
    pub data: Option<serde_json::Value>,
    /// Error message if failed
    pub error: Option<String>,
    /// Execution time in milliseconds
    pub duration_ms: u64,
}

impl PluginResult {
    pub fn success(data: serde_json::Value, duration_ms: u64) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            duration_ms,
        }
    }

    pub fn error(msg: impl Into<String>, duration_ms: u64) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.into()),
            duration_ms,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_validation() {
        let manifest = PluginManifest {
            id: "test-plugin".to_string(),
            name: "Test Plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "A test plugin".to_string(),
            author: "Test Author".to_string(),
            license: "MIT".to_string(),
            api_version: "1.0.0".to_string(),
            entry_point: "libtest.so".to_string(),
            ..Default::default()
        };

        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn test_manifest_invalid() {
        let manifest = PluginManifest {
            id: "".to_string(),
            name: "Test".to_string(),
            version: "1.0.0".to_string(),
            description: "Test".to_string(),
            author: "Test".to_string(),
            license: "MIT".to_string(),
            api_version: "1.0.0".to_string(),
            entry_point: "lib.so".to_string(),
            ..Default::default()
        };

        assert!(manifest.validate().is_err());
    }

    #[test]
    fn test_plugin_info() {
        let manifest = PluginManifest {
            id: "test".to_string(),
            name: "Test".to_string(),
            version: "1.0.0".to_string(),
            description: "Test".to_string(),
            author: "Test".to_string(),
            license: "MIT".to_string(),
            api_version: "1.0.0".to_string(),
            entry_point: "lib.so".to_string(),
            ..Default::default()
        };

        let mut info = PluginInfo::new(manifest, PathBuf::from("/plugins/test"));
        assert!(!info.is_active());

        info.set_active();
        assert!(info.is_active());
    }

    #[test]
    fn test_plugin_context() {
        let ctx = PluginContext::new()
            .with_user("user1")
            .with_session("session1");

        assert_eq!(ctx.user_id, Some("user1".to_string()));
        assert_eq!(ctx.session_id, Some("session1".to_string()));
    }

    #[test]
    fn test_plugin_result() {
        let result = PluginResult::success(serde_json::json!({"key": "value"}), 100);
        assert!(result.success);
        assert_eq!(result.duration_ms, 100);
    }
}
