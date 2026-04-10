//! Error types for sentient_plugin

use thiserror::Error;

/// Plugin error type
#[derive(Error, Debug)]
pub enum PluginError {
    /// Plugin not found
    #[error("Plugin not found: {0}")]
    NotFound(String),

    /// Plugin already loaded
    #[error("Plugin already loaded: {0}")]
    AlreadyLoaded(String),

    /// Plugin initialization failed
    #[error("Plugin initialization failed: {0}")]
    InitFailed(String),

    /// Plugin load failed
    #[error("Failed to load plugin '{0}': {1}")]
    LoadFailed(String, String),

    /// Plugin unload failed
    #[error("Failed to unload plugin '{0}': {1}")]
    UnloadFailed(String, String),

    /// Plugin execution failed
    #[error("Plugin execution failed: {0}")]
    ExecutionFailed(String),

    /// Invalid plugin manifest
    #[error("Invalid plugin manifest: {0}")]
    InvalidManifest(String),

    /// Version mismatch
    #[error("Version mismatch: plugin requires {required}, but {found} was found")]
    VersionMismatch {
        required: String,
        found: String,
    },

    /// Missing dependency
    #[error("Plugin '{plugin}' requires dependency '{dependency}' which is not available")]
    MissingDependency {
        plugin: String,
        dependency: String,
    },

    /// Circular dependency
    #[error("Circular dependency detected: {0}")]
    CircularDependency(String),

    /// Plugin disabled
    #[error("Plugin '{0}' is disabled")]
    Disabled(String),

    /// Plugin not compatible
    #[error("Plugin '{plugin}' is not compatible with API version {api_version}")]
    NotCompatible {
        plugin: String,
        api_version: String,
    },

    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Sandbox violation
    #[error("Sandbox violation: {0}")]
    SandboxViolation(String),

    /// Invalid plugin path
    #[error("Invalid plugin path: {0}")]
    InvalidPath(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Dynamic library error
    #[cfg(feature = "dynamic")]
    #[error("Dynamic library error: {0}")]
    Library(String),

    /// WASM error
    #[cfg(feature = "wasm")]
    #[error("WASM error: {0}")]
    Wasm(String),

    /// Download error
    #[cfg(feature = "download")]
    #[error("Download error: {0}")]
    Download(String),

    /// Extract error
    #[cfg(feature = "download")]
    #[error("Extract error: {0}")]
    Extract(String),

    /// Generic error
    #[error("{0}")]
    Other(String),
}

/// Result type alias for Plugin operations
pub type Result<T> = std::result::Result<T, PluginError>;

impl PluginError {
    /// Create a not found error
    pub fn not_found(name: impl Into<String>) -> Self {
        Self::NotFound(name.into())
    }

    /// Create an init failed error
    pub fn init_failed(msg: impl Into<String>) -> Self {
        Self::InitFailed(msg.into())
    }

    /// Create a load failed error
    pub fn load_failed(name: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::LoadFailed(name.into(), reason.into())
    }

    /// Create an execution failed error
    pub fn execution_failed(msg: impl Into<String>) -> Self {
        Self::ExecutionFailed(msg.into())
    }

    /// Create an invalid manifest error
    pub fn invalid_manifest(msg: impl Into<String>) -> Self {
        Self::InvalidManifest(msg.into())
    }

    /// Create a version mismatch error
    pub fn version_mismatch(required: impl Into<String>, found: impl Into<String>) -> Self {
        Self::VersionMismatch {
            required: required.into(),
            found: found.into(),
        }
    }

    /// Create a missing dependency error
    pub fn missing_dependency(plugin: impl Into<String>, dependency: impl Into<String>) -> Self {
        Self::MissingDependency {
            plugin: plugin.into(),
            dependency: dependency.into(),
        }
    }

    /// Create a sandbox violation error
    pub fn sandbox_violation(msg: impl Into<String>) -> Self {
        Self::SandboxViolation(msg.into())
    }

    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::NotFound(_) |
            Self::MissingDependency { .. } |
            Self::Disabled(_)
        )
    }

    /// Check if error is due to version mismatch
    pub fn is_version_error(&self) -> bool {
        matches!(self, Self::VersionMismatch { .. } | Self::NotCompatible { .. })
    }
}

#[cfg(feature = "dynamic")]
impl From<libloading::Error> for PluginError {
    fn from(e: libloading::Error) -> Self {
        Self::Library(e.to_string())
    }
}

#[cfg(feature = "wasm")]
impl From<wasmtime::Error> for PluginError {
    fn from(e: wasmtime::Error) -> Self {
        Self::Wasm(e.to_string())
    }
}

#[cfg(feature = "download")]
impl From<reqwest::Error> for PluginError {
    fn from(e: reqwest::Error) -> Self {
        Self::Download(e.to_string())
    }
}

#[cfg(feature = "download")]
impl From<zip::result::ZipError> for PluginError {
    fn from(e: zip::result::ZipError) -> Self {
        Self::Extract(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = PluginError::not_found("test-plugin");
        assert!(err.to_string().contains("test-plugin"));
    }

    #[test]
    fn test_version_mismatch() {
        let err = PluginError::version_mismatch("1.0.0", "0.9.0");
        assert!(err.is_version_error());
    }

    #[test]
    fn test_recoverable() {
        let err = PluginError::not_found("test");
        assert!(err.is_recoverable());

        let err = PluginError::init_failed("test");
        assert!(!err.is_recoverable());
    }
}
