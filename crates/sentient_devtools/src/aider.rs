//! Aider Integration
//!
//! AI pair programmer for terminal-based code editing.
//! Aider lets you pair program with AI to edit code in your local git repository.
//!
//! Features:
//! - Git-aware code editing
//! - Multi-file refactoring
//! - Context-aware suggestions
//! - Auto-commit with AI-generated messages
//!
//! Source: integrations/framework/aider

use std::process::Command;
use std::path::PathBuf;
use tracing::{info, debug, warn};

/// Aider Configuration
#[derive(Debug, Clone)]
pub struct AiderConfig {
    /// Model to use (e.g., "claude-3.5-sonnet", "gpt-4")
    pub model: String,
    /// Editor for viewing/editing files
    pub editor: String,
    /// Enable automatic git commits
    pub auto_commits: bool,
    /// Working directory
    pub workdir: PathBuf,
    /// Additional files to include in context
    pub context_files: Vec<PathBuf>,
    /// Enable git history for context
    pub git_history: bool,
    /// Show diff output
    pub show_diff: bool,
    /// Auto-test after changes
    pub auto_test: bool,
    /// Test command to run
    pub test_command: Option<String>,
}

impl Default for AiderConfig {
    fn default() -> Self {
        Self {
            model: "claude-3.5-sonnet".to_string(),
            editor: "vim".to_string(),
            auto_commits: true,
            workdir: PathBuf::from("."),
            context_files: Vec::new(),
            git_history: true,
            show_diff: true,
            auto_test: false,
            test_command: None,
        }
    }
}

impl AiderConfig {
    /// Create config for a specific model
    pub fn with_model(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            ..Default::default()
        }
    }

    /// Set working directory
    pub fn in_dir(dir: impl Into<PathBuf>) -> Self {
        Self {
            workdir: dir.into(),
            ..Default::default()
        }
    }

    /// Add context file
    pub fn with_context(mut self, file: impl Into<PathBuf>) -> Self {
        self.context_files.push(file.into());
        self
    }

    /// Enable auto-testing
    pub fn with_tests(mut self, command: impl Into<String>) -> Self {
        self.auto_test = true;
        self.test_command = Some(command.into());
        self
    }
}

/// Aider session result
#[derive(Debug, Clone)]
pub struct AiderResult {
    /// Files that were modified
    pub modified_files: Vec<String>,
    /// Git commit hash (if auto_commit enabled)
    pub commit_hash: Option<String>,
    /// Output from aider
    pub output: String,
    /// Whether the session was successful
    pub success: bool,
}

/// Check if aider is installed
pub fn is_aider_installed() -> bool {
    Command::new("aider")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Install aider via pip
pub fn install_aider() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("📦 Installing aider...");

    let output = Command::new("pip")
        .args(["install", "aider-chat"])
        .output()?;

    if output.status.success() {
        info!("✅ Aider installed successfully");
        Ok(())
    } else {
        Err(format!(
            "Failed to install aider: {}",
            String::from_utf8_lossy(&output.stderr)
        ).into())
    }
}

/// Run aider with specified configuration
pub async fn run_aider(config: AiderConfig, files: Vec<&str>) -> Result<AiderResult, Box<dyn std::error::Error + Send + Sync>> {
    info!("🤖 Running aider with model: {}", config.model);

    // Check if aider is installed
    if !is_aider_installed() {
        warn!("⚠️ Aider not installed, attempting installation...");
        install_aider()?;
    }

    // Build command arguments
    let mut args = vec![
        "--model".to_string(),
        config.model.clone(),
    ];

    // Add editor
    args.push("--editor".to_string());
    args.push(config.editor.clone());

    // Add auto-commit flag
    if config.auto_commits {
        args.push("--auto-commits".to_string());
    }

    // Add git history flag
    if config.git_history {
        args.push("--git".to_string());
    }

    // Add show diff flag
    if config.show_diff {
        args.push("--show-diff".to_string());
    }

    // Add files to edit
    for file in &files {
        args.push(file.to_string());
    }

    // Add context files
    for ctx_file in &config.context_files {
        args.push("--read".to_string());
        args.push(ctx_file.to_string_lossy().to_string());
    }

    debug!("Aider args: {:?}", args);

    // Run aider
    let output = Command::new("aider")
        .args(&args)
        .current_dir(&config.workdir)
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            // Get modified files from git status
            let modified_files = get_modified_files(&config.workdir)?;

            // Get latest commit hash if auto-commit was enabled
            let commit_hash = if config.auto_commits {
                get_latest_commit(&config.workdir).ok()
            } else {
                None
            };

            Ok(AiderResult {
                modified_files,
                commit_hash,
                output: format!("{}\n{}", stdout, stderr),
                success: output.status.success(),
            })
        }
        Err(e) => {
            Err(format!("Failed to run aider: {}", e).into())
        }
    }
}

/// Run aider with a prompt (interactive mode)
pub async fn run_aider_prompt(
    config: AiderConfig,
    files: Vec<&str>,
    prompt: &str,
) -> Result<AiderResult, Box<dyn std::error::Error + Send + Sync>> {
    info!("🤖 Running aider with prompt: {}", prompt);

    // Check if aider is installed
    if !is_aider_installed() {
        install_aider()?;
    }

    // Build command
    let mut args = vec![
        "--model".to_string(),
        config.model.clone(),
        "--message".to_string(),
        prompt.to_string(),
    ];

    if config.auto_commits {
        args.push("--auto-commits".to_string());
    }

    for file in &files {
        args.push(file.to_string());
    }

    // Run aider
    let output = Command::new("aider")
        .args(&args)
        .current_dir(&config.workdir)
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            let modified_files = get_modified_files(&config.workdir)?;
            let commit_hash = if config.auto_commits {
                get_latest_commit(&config.workdir).ok()
            } else {
                None
            };

            Ok(AiderResult {
                modified_files,
                commit_hash,
                output: format!("{}\n{}", stdout, stderr),
                success: output.status.success(),
            })
        }
        Err(e) => {
            Err(format!("Failed to run aider: {}", e).into())
        }
    }
}

/// Get modified files from git
fn get_modified_files(workdir: &PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let output = Command::new("git")
        .args(["diff", "--name-only", "HEAD"])
        .current_dir(workdir)
        .output()?;

    let files: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(files)
}

/// Get latest commit hash
fn get_latest_commit(workdir: &PathBuf) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .current_dir(workdir)
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = AiderConfig::default();
        assert_eq!(config.model, "claude-3.5-sonnet");
        assert!(config.auto_commits);
    }

    #[test]
    fn test_config_with_model() {
        let config = AiderConfig::with_model("gpt-4");
        assert_eq!(config.model, "gpt-4");
    }
}
