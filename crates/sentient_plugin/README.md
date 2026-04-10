# sentient_plugin

**Plugin and Extension System** for SENTIENT OS.

[![Crates.io](https://img.shields.io/crates/v/sentient_plugin.svg)](https://crates.io/crates/sentient_plugin)
[![Documentation](https://docs.rs/sentient_plugin/badge.svg)](https://docs.rs/sentient_plugin)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

## Overview

This crate provides a comprehensive plugin system for SENTIENT OS:

- 🔌 **Plugin Lifecycle**: Load, initialize, execute, shutdown
- 🔍 **Discovery**: Automatic plugin discovery from filesystem
- 🔒 **Sandbox**: Security isolation for plugins
- 📦 **Registry**: Plugin marketplace and dependency management
- 📡 **Events**: Plugin event system
- 🔧 **Middleware**: Request/response interception

## Features

| Feature | Description | Default |
|---------|-------------|---------|
| `discovery` | Filesystem plugin discovery | ✅ |
| `dynamic` | Dynamic library loading | ❌ |
| `wasm` | WebAssembly plugin support | ❌ |
| `download` | Plugin download/install | ❌ |
| `full` | All features enabled | ❌ |

## Installation

```toml
[dependencies]
sentient_plugin = { path = "crates/sentient_plugin" }

# With all features
sentient_plugin = { path = "crates/sentient_plugin", features = ["full"] }
```

## Quick Start

### Create a Plugin

```rust
use sentient_plugin::{Plugin, PluginManifest, PluginConfig, PluginContext, PluginStatus};
use async_trait::async_trait;

struct MyPlugin {
    manifest: PluginManifest,
}

#[async_trait]
impl Plugin for MyPlugin {
    fn metadata(&self) -> &PluginManifest {
        &self.manifest
    }

    async fn initialize(&mut self, config: &PluginConfig) -> sentient_plugin::Result<()> {
        // Initialize plugin
        Ok(())
    }

    async fn shutdown(&mut self) -> sentient_plugin::Result<()> {
        // Cleanup
        Ok(())
    }

    fn status(&self) -> PluginStatus {
        PluginStatus::Active
    }

    async fn execute(
        &self,
        command: &str,
        args: serde_json::Value,
        context: &PluginContext,
    ) -> sentient_plugin::Result<serde_json::Value> {
        match command {
            "hello" => Ok(serde_json::json!({"message": "Hello from plugin!"})),
            _ => Err(sentient_plugin::PluginError::execution_failed("Unknown command")),
        }
    }
}
```

### Use Plugin Manager

```rust
use sentient_plugin::{PluginManager, PluginContext};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create manager
    let manager = PluginManager::default_manager();
    
    // Register plugin
    let plugin = Box::new(MyPlugin::new());
    manager.register(plugin).await?;
    
    // Execute plugin command
    let ctx = PluginContext::new();
    let result = manager.execute(
        "my-plugin",
        "hello",
        serde_json::json!({}),
        &ctx,
    ).await?;
    
    println!("Result: {:?}", result);
    
    // Get all tools from plugins
    let tools = manager.get_all_tools().await;
    for (plugin_id, tool) in tools {
        println!("Tool: {} from {}", tool.name, plugin_id);
    }
    
    Ok(())
}
```

### Plugin Manifest (plugin.json)

```json
{
    "id": "my-plugin",
    "name": "My Plugin",
    "version": "1.0.0",
    "description": "A sample plugin",
    "author": "Your Name",
    "license": "MIT",
    "api_version": "4.0.0",
    "entry_point": "libmy_plugin.so",
    "capabilities": ["tools", "events"],
    "permissions": ["filesystem_read", "network"],
    "dependencies": [
        {
            "id": "other-plugin",
            "version": ">=1.0.0",
            "optional": false
        }
    ]
}
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    sentient_plugin                          │
├─────────────────────────────────────────────────────────────┤
│  ┌───────────────┐  ┌───────────────┐  ┌───────────────┐   │
│  │ PluginManager │  │ PluginDiscovery│ │ PluginRegistry│   │
│  └───────────────┘  └───────────────┘  └───────────────┘   │
│          │                  │                  │            │
│          ▼                  ▼                  ▼            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                  Plugin Layer                        │   │
│  │  ┌─────┐ ┌───────┐ ┌────────┐ ┌───────┐ ┌───────┐  │   │
│  │  │Tools│ │Events │ │Commands│ │Hooks  │ │Config │  │   │
│  │  └─────┘ └───────┘ └────────┘ └───────┘ └───────┘  │   │
│  └─────────────────────────────────────────────────────┘   │
│          │                                                   │
│          ▼                                                   │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                  Sandbox Layer                       │   │
│  │  Filesystem │ Network │ Memory │ Process │ Time     │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Plugin Capabilities

| Capability | Description |
|------------|-------------|
| `Tools` | Provide AI tools |
| `Resources` | Provide resources |
| `Prompts` | Provide prompt templates |
| `Events` | Hook into events |
| `Ui` | Extend UI |
| `Commands` | Add CLI commands |
| `Settings` | Add settings |
| `Skills` | Provide skills |
| `Memory` | Access memory |
| `Network` | Network access |
| `Filesystem` | File access |

## Security

Plugins run in a sandbox with configurable permissions:

```rust
use sentient_plugin::{SandboxConfig, PluginSandbox};

// Strict sandbox (default)
let strict = SandboxConfig::strict();

// Permissive sandbox
let permissive = SandboxConfig::permissive();

// Custom sandbox
let custom = SandboxConfig::default()
    .allow_path("/data")
    .allow_host("api.example.com")
    .max_execution_time(60);
```

## Registry

The plugin registry provides marketplace functionality:

```rust
use sentient_plugin::{PluginRegistry, RegistrySearch};

let mut registry = PluginRegistry::new();

// Search plugins
let search = RegistrySearch {
    query: Some("vision".to_string()),
    tags: vec!["ai".to_string()],
    ..Default::default()
};

let results = registry.search(&search);
for plugin in results.plugins {
    println!("{} - {}", plugin.name, plugin.description);
}
```

## License

Apache License 2.0

---

*SENTIENT OS - The Operating System That Thinks*
