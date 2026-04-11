// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Sandbox Terminal
// ═══════════════════════════════════════════════════════════════════════════════
//  Terminal and command execution
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};

/// Terminal output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalOutput {
    /// stdout
    pub stdout: String,
    /// stderr
    pub stderr: String,
    /// Exit code
    pub exit_code: i32,
}

impl TerminalOutput {
    /// Check if command succeeded
    pub fn is_success(&self) -> bool {
        self.exit_code == 0
    }

    /// Get combined output
    pub fn output(&self) -> String {
        if self.stderr.is_empty() {
            self.stdout.clone()
        } else {
            format!("{}\n{}", self.stdout, self.stderr)
        }
    }
}

/// Terminal error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalError {
    /// Error message
    pub message: String,
    /// Error code
    pub code: Option<i32>,
}

/// Command execution request
#[derive(Debug, Clone, Serialize)]
pub struct RunCommandRequest {
    /// Command to run
    pub command: String,
    /// Arguments
    #[serde(default)]
    pub args: Vec<String>,
    /// Working directory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    /// Environment variables
    #[serde(default)]
    pub env: std::collections::HashMap<String, String>,
    /// Timeout in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
}

impl RunCommandRequest {
    /// Create new command request
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            args: Vec::new(),
            cwd: None,
            env: std::collections::HashMap::new(),
            timeout: None,
        }
    }

    /// Add arguments
    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    /// Set working directory
    pub fn with_cwd(mut self, cwd: impl Into<String>) -> Self {
        self.cwd = Some(cwd.into());
        self
    }

    /// Add environment variable
    pub fn with_env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env.insert(key.into(), value.into());
        self
    }

    /// Set timeout
    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout = Some(secs);
        self
    }

    /// Run Python script
    pub fn python(script: impl Into<String>) -> Self {
        Self::new("python").with_args(vec![script.into()])
    }

    /// Run Node.js script
    pub fn node(script: impl Into<String>) -> Self {
        Self::new("node").with_args(vec![script.into()])
    }

    /// Run Rust program
    pub fn cargo_run() -> Self {
        Self::new("cargo").with_args(vec!["run".to_string()])
    }

    /// Run shell command
    pub fn shell(command: impl Into<String>) -> Self {
        Self::new("sh").with_args(vec!["-c".to_string(), command.into()])
    }

    /// Install package with pip
    pub fn pip_install(package: impl Into<String>) -> Self {
        Self::new("pip").with_args(vec!["install".to_string(), package.into()])
    }

    /// Install package with npm
    pub fn npm_install(package: impl Into<String>) -> Self {
        Self::new("npm").with_args(vec!["install".to_string(), package.into()])
    }
}

/// Terminal session (for persistent terminals)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSession {
    /// Session ID
    pub id: String,
    /// Terminal size (cols, rows)
    pub size: (u16, u16),
    /// Whether session is active
    pub active: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_command() {
        let req = RunCommandRequest::new("ls")
            .with_args(vec!["-la".to_string()])
            .with_cwd("/home");

        assert_eq!(req.command, "ls");
        assert_eq!(req.args, vec!["-la"]);
        assert_eq!(req.cwd, Some("/home".to_string()));
    }

    #[test]
    fn test_python_command() {
        let req = RunCommandRequest::python("main.py");
        assert_eq!(req.command, "python");
        assert_eq!(req.args, vec!["main.py"]);
    }

    #[test]
    fn test_terminal_output() {
        let output = TerminalOutput {
            stdout: "success".to_string(),
            stderr: String::new(),
            exit_code: 0,
        };

        assert!(output.is_success());
        assert_eq!(output.output(), "success");
    }
}
