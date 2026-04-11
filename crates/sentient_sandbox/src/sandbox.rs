// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - E2B Sandbox Client
// ═══════════════════════════════════════════════════════════════════════════════
//  Main client for E2B code sandbox operations
// ═══════════════════════════════════════════════════════════════════════════════

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, info};
use uuid::Uuid;

use crate::{
    SandboxConfig, SandboxError, Result, ExecutionResult, CodeSnippet,
    files::{FileInfo, FileContent, WriteFileRequest, ListDirRequest},
    terminal::{RunCommandRequest, TerminalOutput},
    templates::BuiltinTemplate,
};

/// Sandbox metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxMetadata {
    /// Sandbox ID
    pub sandbox_id: String,
    /// Template used
    pub template: String,
    /// Client ID for reconnection
    pub client_id: String,
    /// Created timestamp
    pub created_at: String,
    /// Alias (optional)
    pub alias: Option<String>,
}

/// Running sandbox instance
#[derive(Debug)]
pub struct Sandbox {
    metadata: SandboxMetadata,
    config: SandboxConfig,
    http: Client,
}

impl Sandbox {
    /// Create sandbox from existing metadata
    pub fn from_metadata(metadata: SandboxMetadata, config: SandboxConfig) -> Result<Self> {
        let http = Client::builder()
            .timeout(Duration::from_secs(config.request_timeout_secs))
            .build()
            .map_err(SandboxError::HttpError)?;

        Ok(Self { metadata, config, http })
    }

    /// Get sandbox ID
    pub fn id(&self) -> &str {
        &self.metadata.sandbox_id
    }

    /// Get template
    pub fn template(&self) -> &str {
        &self.metadata.template
    }

    /// Get metadata
    pub fn metadata(&self) -> &SandboxMetadata {
        &self.metadata
    }

    // ═══════════════════════════════════════════════════════════════════════════
    //  Code Execution
    // ═══════════════════════════════════════════════════════════════════════════

    /// Run code snippet
    pub async fn run_code(&self, snippet: &CodeSnippet) -> Result<ExecutionResult> {
        debug!("Running {} code", snippet.language);

        // Write code to file
        let entrypoint = snippet.entrypoint.clone()
            .unwrap_or_else(|| format!("main.{}", snippet.language));
        
        self.write_file(&entrypoint, &snippet.source).await?;

        // Execute
        let start = std::time::Instant::now();
        let output = self.run_command(&RunCommandRequest::shell(
            &format!("{} {}", get_run_command(&snippet.language), entrypoint)
        )).await?;
        
        let duration = start.elapsed().as_millis() as u64;

        Ok(ExecutionResult {
            stdout: output.stdout,
            stderr: output.stderr,
            exit_code: output.exit_code,
            duration_ms: duration,
            success: output.exit_code == 0,
        })
    }

    /// Run Python code
    pub async fn run_python(&self, code: &str) -> Result<ExecutionResult> {
        self.run_code(&CodeSnippet::python(code)).await
    }

    /// Run JavaScript code
    pub async fn run_javascript(&self, code: &str) -> Result<ExecutionResult> {
        self.run_code(&CodeSnippet::javascript(code)).await
    }

    /// Run Rust code (compiles and runs)
    pub async fn run_rust(&self, code: &str) -> Result<ExecutionResult> {
        // Create Cargo.toml for single file
        let cargo_toml = r#"[package]
name = "sandbox"
version = "0.1.0"
edition = "2021"

[dependencies]
"#;

        self.write_file("Cargo.toml", cargo_toml).await?;
        self.write_file("src/main.rs", code).await?;
        
        let start = std::time::Instant::now();
        let output = self.run_command(&RunCommandRequest::cargo_run()).await?;
        let duration = start.elapsed().as_millis() as u64;

        Ok(ExecutionResult {
            stdout: output.stdout,
            stderr: output.stderr,
            exit_code: output.exit_code,
            duration_ms: duration,
            success: output.exit_code == 0,
        })
    }

    // ═══════════════════════════════════════════════════════════════════════════
    //  Terminal Commands
    // ═══════════════════════════════════════════════════════════════════════════

    /// Run terminal command
    pub async fn run_command(&self, request: &RunCommandRequest) -> Result<TerminalOutput> {
        debug!("Running command: {}", request.command);

        let response = self.http
            .post(format!(
                "{}/sandboxes/{}/commands",
                self.config.base_url,
                self.metadata.sandbox_id
            ))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Run shell command
    pub async fn shell(&self, command: &str) -> Result<TerminalOutput> {
        self.run_command(&RunCommandRequest::shell(command)).await
    }

    /// Install Python package
    pub async fn pip_install(&self, package: &str) -> Result<TerminalOutput> {
        self.run_command(&RunCommandRequest::pip_install(package)).await
    }

    /// Install Node package
    pub async fn npm_install(&self, package: &str) -> Result<TerminalOutput> {
        self.run_command(&RunCommandRequest::npm_install(package)).await
    }

    // ═══════════════════════════════════════════════════════════════════════════
    //  File Operations
    // ═══════════════════════════════════════════════════════════════════════════

    /// Write file
    pub async fn write_file(&self, path: &str, content: &str) -> Result<()> {
        debug!("Writing file: {}", path);

        let request = WriteFileRequest::new(path, content);

        self.http
            .post(format!(
                "{}/sandboxes/{}/files",
                self.config.base_url,
                self.metadata.sandbox_id
            ))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        Ok(())
    }

    /// Read file
    pub async fn read_file(&self, path: &str) -> Result<String> {
        debug!("Reading file: {}", path);

        #[derive(Deserialize)]
        struct ReadResponse {
            content: String,
        }

        let response = self.http
            .get(format!(
                "{}/sandboxes/{}/files?path={}",
                self.config.base_url,
                self.metadata.sandbox_id,
                urlencoding::encode(path)
            ))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await?;

        let result: ReadResponse = self.handle_response(response).await?;
        Ok(result.content)
    }

    /// List directory
    pub async fn list_dir(&self, path: &str) -> Result<Vec<FileInfo>> {
        debug!("Listing directory: {}", path);

        #[derive(Deserialize)]
        struct ListResponse {
            files: Vec<FileInfo>,
        }

        let response = self.http
            .get(format!(
                "{}/sandboxes/{}/files/list?path={}",
                self.config.base_url,
                self.metadata.sandbox_id,
                urlencoding::encode(path)
            ))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await?;

        let result: ListResponse = self.handle_response(response).await?;
        Ok(result.files)
    }

    /// Delete file
    pub async fn delete_file(&self, path: &str) -> Result<()> {
        debug!("Deleting file: {}", path);

        self.http
            .delete(format!(
                "{}/sandboxes/{}/files?path={}",
                self.config.base_url,
                self.metadata.sandbox_id,
                urlencoding::encode(path)
            ))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await?;

        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════════════════
    //  Lifecycle
    // ═══════════════════════════════════════════════════════════════════════════

    /// Kill sandbox
    pub async fn kill(self) -> Result<()> {
        info!("Killing sandbox: {}", self.metadata.sandbox_id);

        self.http
            .delete(format!(
                "{}/sandboxes/{}",
                self.config.base_url,
                self.metadata.sandbox_id
            ))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await?;

        Ok(())
    }

    /// Set timeout (keep alive)
    pub async fn set_timeout(&self, seconds: u64) -> Result<()> {
        debug!("Setting timeout: {}s", seconds);

        #[derive(Serialize)]
        struct TimeoutRequest {
            timeout: u64,
        }

        self.http
            .patch(format!(
                "{}/sandboxes/{}",
                self.config.base_url,
                self.metadata.sandbox_id
            ))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&TimeoutRequest { timeout: seconds })
            .send()
            .await?;

        Ok(())
    }

    /// Handle HTTP response
    async fn handle_response<T: for<'de> Deserialize<'de>>(
        &self,
        response: reqwest::Response,
    ) -> Result<T> {
        let status = response.status();

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();

            #[derive(Deserialize)]
            struct ErrorResponse {
                error: Option<String>,
                message: Option<String>,
            }

            let error_msg = if let Ok(err) = serde_json::from_str::<ErrorResponse>(&body) {
                err.error.or(err.message).unwrap_or(body.clone())
            } else {
                body
            };

            return Err(match status.as_u16() {
                401 => SandboxError::InvalidApiKey,
                404 => SandboxError::SandboxNotFound(error_msg),
                402 => SandboxError::InsufficientCredits,
                429 => SandboxError::RateLimitExceeded,
                _ => SandboxError::ApiError(format!("HTTP {}: {}", status, error_msg)),
            });
        }

        response.json().await.map_err(SandboxError::HttpError)
    }
}

/// Sandbox builder
pub struct SandboxBuilder {
    config: SandboxConfig,
    template: String,
    timeout: u64,
}

impl SandboxBuilder {
    pub fn new(config: SandboxConfig) -> Self {
        Self {
            template: config.default_template.clone(),
            timeout: config.timeout_secs,
            config,
        }
    }

    /// Use template
    pub fn template(mut self, template: impl Into<String>) -> Self {
        self.template = template.into();
        self
    }

    /// Use builtin template
    pub fn builtin_template(mut self, template: BuiltinTemplate) -> Self {
        self.template = template.id().to_string();
        self
    }

    /// Set timeout
    pub fn timeout(mut self, secs: u64) -> Self {
        self.timeout = secs;
        self
    }

    /// Create sandbox
    pub async fn create(self) -> Result<Sandbox> {
        let http = Client::builder()
            .timeout(Duration::from_secs(self.config.request_timeout_secs))
            .build()
            .map_err(SandboxError::HttpError)?;

        info!("Creating sandbox with template: {}", self.template);

        #[derive(Serialize)]
        struct CreateRequest {
            template: String,
            timeout: u64,
        }

        #[derive(Deserialize)]
        struct CreateResponse {
            sandbox_id: String,
            client_id: String,
            template: String,
            #[serde(default)]
            alias: Option<String>,
        }

        let response = http
            .post(format!("{}/sandboxes", self.config.base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&CreateRequest {
                template: self.template.clone(),
                timeout: self.timeout,
            })
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(match status.as_u16() {
                401 => SandboxError::InvalidApiKey,
                404 => SandboxError::TemplateNotFound(self.template),
                402 => SandboxError::InsufficientCredits,
                _ => SandboxError::ApiError(format!("HTTP {}: {}", status, body)),
            });
        }

        let result: CreateResponse = response.json().await.map_err(SandboxError::HttpError)?;

        let metadata = SandboxMetadata {
            sandbox_id: result.sandbox_id,
            template: result.template,
            client_id: result.client_id,
            created_at: chrono_lite_now(),
            alias: result.alias,
        };

        Ok(Sandbox {
            metadata,
            config: self.config,
            http,
        })
    }
}

/// Get run command for language
fn get_run_command(language: &str) -> &'static str {
    match language {
        "python" => "python",
        "javascript" => "node",
        "typescript" => "npx tsx",
        "rust" => "cargo run",
        "go" => "go run",
        "java" => "java",
        _ => "echo 'Unknown language'",
    }
}

/// Simple current time string
fn chrono_lite_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    format!("{}", duration.as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_builder() {
        let config = SandboxConfig::new("test-key");
        let builder = SandboxBuilder::new(config)
            .builtin_template(BuiltinTemplate::Python311)
            .timeout(600);

        assert_eq!(builder.template, "python-3.11");
        assert_eq!(builder.timeout, 600);
    }

    #[test]
    fn test_sandbox_metadata() {
        let metadata = SandboxMetadata {
            sandbox_id: "sb_123".to_string(),
            template: "python".to_string(),
            client_id: "client_123".to_string(),
            created_at: "12345".to_string(),
            alias: None,
        };

        let config = SandboxConfig::new("test-key");
        let sandbox = Sandbox::from_metadata(metadata.clone(), config).unwrap();

        assert_eq!(sandbox.id(), "sb_123");
        assert_eq!(sandbox.template(), "python");
    }
}
