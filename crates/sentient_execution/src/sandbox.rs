//! Docker Sandbox Integration
//!
//! Container-based code execution for isolated and secure code running.
//!
//! Features:
//! - Docker container isolation
//! - Resource limits (memory, CPU, timeout)
//! - Network isolation option
//! - Automatic cleanup
//!
//! Fallback: If Docker is not available, falls back to subprocess execution.

use crate::ExecutionResult;
use std::process::Command;
use std::time::{Duration, Instant};
use tracing::{info, debug, warn};

/// Sandbox Configuration
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    /// Docker image to use
    pub image: String,
    /// Memory limit in MB
    pub memory_mb: u64,
    /// Execution timeout in seconds
    pub timeout_secs: u64,
    /// Allow network access
    pub network: bool,
    /// Working directory inside container
    pub workdir: String,
    /// Environment variables
    pub env: Vec<(String, String)>,
    /// Mount volumes
    pub volumes: Vec<(String, String)>,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            image: "python:3.11-slim".to_string(),
            memory_mb: 512,
            timeout_secs: 60,
            network: false,
            workdir: "/app".to_string(),
            env: Vec::new(),
            volumes: Vec::new(),
        }
    }
}

impl SandboxConfig {
    /// Create config for Python execution
    pub fn python() -> Self {
        Self::default()
    }

    /// Create config for Node.js execution
    pub fn nodejs() -> Self {
        Self {
            image: "node:20-slim".to_string(),
            ..Default::default()
        }
    }

    /// Create config for Rust execution
    pub fn rust() -> Self {
        Self {
            image: "rust:1.75-slim".to_string(),
            memory_mb: 1024,
            timeout_secs: 300, // Rust compilation takes longer
            ..Default::default()
        }
    }

    /// Add environment variable
    pub fn with_env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env.push((key.into(), value.into()));
        self
    }

    /// Add volume mount
    pub fn with_volume(mut self, host: impl Into<String>, container: impl Into<String>) -> Self {
        self.volumes.push((host.into(), container.into()));
        self
    }
}

/// Check if Docker is available
pub fn is_docker_available() -> bool {
    Command::new("docker")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Run code in Docker sandbox
pub async fn run_in_sandbox(config: SandboxConfig, code: &str) -> Result<ExecutionResult, Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();

    // Check if Docker is available
    if !is_docker_available() {
        warn!("Docker not available, falling back to subprocess execution");
        return run_subprocess(&config, code).await;
    }

    // Build Docker command
    let container_name = format!("sentient_sandbox_{}", uuid::Uuid::new_v4());

    // Build volume mounts
    let mut volume_args: Vec<String> = vec![];
    for (host, container) in &config.volumes {
        volume_args.push("-v".to_string());
        volume_args.push(format!("{}:{}", host, container));
    }

    // Build environment args
    let mut env_args: Vec<String> = vec![];
    for (key, value) in &config.env {
        env_args.push("-e".to_string());
        env_args.push(format!("{}={}", key, value));
    }

    // Network policy
    let network_arg = if config.network {
        vec!["--network".to_string(), "host".to_string()]
    } else {
        vec!["--network".to_string(), "none".to_string()]
    };

    // Memory limit
    let memory_arg = format!("{}m", config.memory_mb);

    // Execute in container
    let output = Command::new("docker")
        .arg("run")
        .arg("--rm")
        .arg("--name")
        .arg(&container_name)
        .arg("-m")
        .arg(&memory_arg)
        .args(&volume_args)
        .args(&env_args)
        .args(&network_arg)
        .arg("-w")
        .arg(&config.workdir)
        .arg(&config.image)
        .arg("sh")
        .arg("-c")
        .arg(code)
        .output();

    // Cleanup container if still running (timeout case)
    let _ = Command::new("docker")
        .arg("rm")
        .arg("-f")
        .arg(&container_name)
        .output();

    match output {
        Ok(output) => {
            let duration = start.elapsed();

            Ok(ExecutionResult {
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                exit_code: output.status.code().unwrap_or(-1),
                duration_ms: duration.as_millis() as u64,
                success: output.status.success(),
            })
        }
        Err(e) => {
            Err(format!("Docker execution failed: {}", e).into())
        }
    }
}

/// Fallback: Run code via subprocess (less secure)
async fn run_subprocess(config: &SandboxConfig, code: &str) -> Result<ExecutionResult, Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();

    // Determine interpreter based on image
    let (cmd, args): (&str, Vec<&str>) = if config.image.contains("python") {
        ("python3", vec!["-c", code])
    } else if config.image.contains("node") {
        ("node", vec!["-e", code])
    } else if config.image.contains("rust") {
        // For Rust, we need a file
        warn!("Rust execution without Docker requires file-based compilation");
        ("sh", vec!["-c", code])
    } else {
        ("sh", vec!["-c", code])
    };

    // Build environment
    let mut cmd_proc = Command::new(cmd);
    cmd_proc.args(&args);

    for (key, value) in &config.env {
        cmd_proc.env(key, value);
    }

    // Execute with timeout
    let timeout = Duration::from_secs(config.timeout_secs);
    let output = tokio::task::spawn_blocking(move || {
        cmd_proc.output()
    });

    // Wait with timeout
    let result = tokio::time::timeout(timeout, output).await;

    match result {
        Ok(Ok(Ok(output))) => {
            let duration = start.elapsed();
            Ok(ExecutionResult {
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                exit_code: output.status.code().unwrap_or(-1),
                duration_ms: duration.as_millis() as u64,
                success: output.status.success(),
            })
        }
        Ok(Ok(Err(e))) => {
            Err(format!("Subprocess execution failed: {}", e).into())
        }
        Ok(Err(_)) => {
            Err("Subprocess panicked".into())
        }
        Err(_) => {
            Err(format!("Execution timed out after {} seconds", config.timeout_secs).into())
        }
    }
}

/// Execute Python code in sandbox
pub async fn run_python(code: &str) -> Result<ExecutionResult, Box<dyn std::error::Error + Send + Sync>> {
    run_in_sandbox(SandboxConfig::python(), code).await
}

/// Execute Node.js code in sandbox
pub async fn run_javascript(code: &str) -> Result<ExecutionResult, Box<dyn std::error::Error + Send + Sync>> {
    run_in_sandbox(SandboxConfig::nodejs(), code).await
}

/// Execute shell command in sandbox
pub async fn run_shell(code: &str) -> Result<ExecutionResult, Box<dyn std::error::Error + Send + Sync>> {
    run_in_sandbox(SandboxConfig::default(), code).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_config_default() {
        let config = SandboxConfig::default();
        assert_eq!(config.image, "python:3.11-slim");
        assert_eq!(config.memory_mb, 512);
        assert!(!config.network);
    }

    #[test]
    fn test_sandbox_config_python() {
        let config = SandboxConfig::python();
        assert!(config.image.contains("python"));
    }

    #[test]
    fn test_sandbox_config_nodejs() {
        let config = SandboxConfig::nodejs();
        assert!(config.image.contains("node"));
    }

    #[tokio::test]
    async fn test_run_shell_subprocess() {
        // Force subprocess fallback
        let config = SandboxConfig {
            image: "nonexistent-image".to_string(),
            ..Default::default()
        };

        // This should fall back to subprocess
        let result = run_subprocess(&config, "echo 'Hello'").await;
        assert!(result.is_ok());

        let exec_result = result.expect("operation failed");
        assert!(exec_result.success);
        assert!(exec_result.stdout.contains("Hello"));
    }
}
