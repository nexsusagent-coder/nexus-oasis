//! ─── SENTIENT PLUGINS SYSTEM ───
//!
//! Dynamic plugin management:
//! - Plugin loading and unloading
//! - Hot reload support
//! - Plugin isolation
//! - Dependency management

pub mod hot_reload;

pub use hot_reload::{
    PluginHotReloader, PluginMeta, PluginStatus, HotReloadConfig,
    HotReloadError, ReloadEvent, PluginWatcher,
};
