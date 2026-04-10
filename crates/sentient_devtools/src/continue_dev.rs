//! Continue.dev Integration
//!
//! Open-source autopilot for VS Code and JetBrains IDEs.
//! Continue provides AI-powered code completion, chat, and editing.
//!
//! Features:
//! - VS Code / JetBrains extension
//! - Custom model configuration
//! - Context providers (codebase, docs, web)
//! - Custom slash commands
//!
//! Source: integrations/framework/continue-dev

use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};
use tracing::{info, debug};

/// Continue.dev Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinueConfig {
    /// Models to use
    pub models: Vec<ModelConfig>,
    /// Context providers
    pub context_providers: Vec<ContextProvider>,
    /// Custom slash commands
    pub slash_commands: Vec<SlashCommand>,
    /// Tab autocomplete settings
    pub tab_autocomplete: TabAutocompleteConfig,
    /// Embeddings settings
    pub embeddings: EmbeddingsConfig,
}

impl Default for ContinueConfig {
    fn default() -> Self {
        Self {
            models: vec![
                ModelConfig {
                    title: "Claude 3.5 Sonnet".to_string(),
                    provider: "anthropic".to_string(),
                    model: "claude-3-5-sonnet-20241022".to_string(),
                    api_key: "${ANTHROPIC_API_KEY}".to_string(),
                    context_length: 200000,
                    temperature: 0.7,
                },
            ],
            context_providers: vec![
                ContextProvider {
                    name: "codebase".to_string(),
                    enabled: true,
                },
                ContextProvider {
                    name: "docs".to_string(),
                    enabled: true,
                },
            ],
            slash_commands: vec![],
            tab_autocomplete: TabAutocompleteConfig {
                enabled: true,
                model: "starcoder2:3b".to_string(),
            },
            embeddings: EmbeddingsConfig {
                provider: "ollama".to_string(),
                model: "nomic-embed-text".to_string(),
            },
        }
    }
}

/// Model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub title: String,
    pub provider: String,
    pub model: String,
    pub api_key: String,
    #[serde(default = "default_context_length")]
    pub context_length: usize,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
}

fn default_context_length() -> usize { 8192 }
fn default_temperature() -> f32 { 0.7 }

/// Context provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextProvider {
    pub name: String,
    pub enabled: bool,
}

/// Custom slash command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlashCommand {
    pub name: String,
    pub description: String,
    pub prompt: String,
}

/// Tab autocomplete configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabAutocompleteConfig {
    pub enabled: bool,
    pub model: String,
}

/// Embeddings configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingsConfig {
    pub provider: String,
    pub model: String,
}

/// Continue.dev setup result
#[derive(Debug, Clone)]
pub struct SetupResult {
    /// Path to config file
    pub config_path: PathBuf,
    /// Whether setup was successful
    pub success: bool,
    /// Message
    pub message: String,
}

/// Setup Continue.dev with configuration
pub async fn setup_continue(config: ContinueConfig) -> Result<SetupResult, Box<dyn std::error::Error + Send + Sync>> {
    info!("🔧 Setting up Continue.dev...");

    // Determine config path based on OS
    let config_dir = get_continue_config_dir();
    let config_path = config_dir.join("config.json");

    // Create config directory if it doesn't exist
    fs::create_dir_all(&config_dir)?;

    // Serialize config to JSON
    let config_json = serde_json::to_string_pretty(&config)?;

    // Write config file
    fs::write(&config_path, &config_json)?;

    info!("✅ Continue.dev config written to {:?}", config_path);

    Ok(SetupResult {
        config_path,
        success: true,
        message: "Continue.dev configuration created successfully".to_string(),
    })
}

/// Get Continue.dev config directory based on OS
fn get_continue_config_dir() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        PathBuf::from(env!("HOME")).join(".continue")
    }

    #[cfg(target_os = "linux")]
    {
        PathBuf::from(env!("HOME")).join(".continue")
    }

    #[cfg(target_os = "windows")]
    {
        PathBuf::from(env!("APPDATA")).join("Continue")
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        PathBuf::from(".").join(".continue")
    }
}

/// Create default Continue.dev configuration
pub fn create_default_config() -> ContinueConfig {
    ContinueConfig::default()
}

/// Create config for a specific model
pub fn create_model_config(model: &str, provider: &str, api_key: &str) -> ContinueConfig {
    let model_config = ModelConfig {
        title: format!("Custom {}", model),
        provider: provider.to_string(),
        model: model.to_string(),
        api_key: api_key.to_string(),
        context_length: 8192,
        temperature: 0.7,
    };

    ContinueConfig {
        models: vec![model_config],
        ..Default::default()
    }
}

/// Add a slash command to config
pub fn add_slash_command(mut config: ContinueConfig, name: &str, description: &str, prompt: &str) -> ContinueConfig {
    config.slash_commands.push(SlashCommand {
        name: name.to_string(),
        description: description.to_string(),
        prompt: prompt.to_string(),
    });
    config
}

/// Read existing Continue.dev config
pub fn read_config() -> Result<ContinueConfig, Box<dyn std::error::Error + Send + Sync>> {
    let config_path = get_continue_config_dir().join("config.json");

    if !config_path.exists() {
        return Ok(ContinueConfig::default());
    }

    let content = fs::read_to_string(&config_path)?;
    let config: ContinueConfig = serde_json::from_str(&content)?;

    Ok(config)
}

/// Update Continue.dev config
pub async fn update_config<F>(f: F) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    F: FnOnce(ContinueConfig) -> ContinueConfig,
{
    let config = read_config().unwrap_or_default();
    let updated = f(config);
    setup_continue(updated).await?;
    Ok(())
}

/// Setup Continue.dev for a specific project
pub async fn setup_for_project(project_path: &PathBuf) -> Result<SetupResult, Box<dyn std::error::Error + Send + Sync>> {
    info!("🔧 Setting up Continue.dev for project: {:?}", project_path);

    // Create .continuerc.json in project root
    let config = ContinueConfig {
        context_providers: vec![
            ContextProvider {
                name: "codebase".to_string(),
                enabled: true,
            },
            ContextProvider {
                name: "docs".to_string(),
                enabled: true,
            },
            ContextProvider {
                name: "file".to_string(),
                enabled: true,
            },
        ],
        ..Default::default()
    };

    let config_path = project_path.join(".continuerc.json");
    let config_json = serde_json::to_string_pretty(&config)?;
    fs::write(&config_path, &config_json)?;

    Ok(SetupResult {
        config_path,
        success: true,
        message: "Project-level Continue.dev config created".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = ContinueConfig::default();
        assert!(!config.models.is_empty());
        assert!(config.tab_autocomplete.enabled);
    }

    #[test]
    fn test_create_model_config() {
        let config = create_model_config("gpt-4", "openai", "sk-test");
        assert_eq!(config.models.len(), 1);
        assert_eq!(config.models[0].model, "gpt-4");
    }

    #[test]
    fn test_add_slash_command() {
        let config = ContinueConfig::default();
        let updated = add_slash_command(config, "test", "Test command", "Run tests");
        assert_eq!(updated.slash_commands.len(), 1);
    }

    #[test]
    fn test_config_serialization() {
        let config = ContinueConfig::default();
        let json = serde_json::to_string(&config).expect("operation failed");
        let parsed: ContinueConfig = serde_json::from_str(&json).expect("operation failed");
        assert_eq!(parsed.models.len(), config.models.len());
    }
}
