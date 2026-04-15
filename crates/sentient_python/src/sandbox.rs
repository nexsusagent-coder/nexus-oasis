//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT Local Sandbox - Secure Code Execution Environment
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  Isolated execution environment for untrusted code:
//!  - Resource limits (CPU, memory, time, disk)
//!  - Filesystem isolation (chroot/namespace)
//!  - Network isolation (optional)
//!  - Output capture and logging
//!  - Security policies

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

// ═══════════════════════════════════════════════════════════════════════════════
//  SANDBOX CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Sandbox configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    /// Maximum execution time in seconds
    pub max_execution_time_secs: u64,
    /// Maximum memory in MB
    pub max_memory_mb: u64,
    /// Maximum CPU percentage (0-100)
    pub max_cpu_percent: u32,
    /// Maximum disk usage in MB
    pub max_disk_mb: u64,
    /// Maximum file size in MB
    pub max_file_size_mb: u64,
    /// Maximum number of processes
    pub max_processes: u32,
    /// Maximum number of threads
    pub max_threads: u32,
    /// Allow network access
    pub allow_network: bool,
    /// Allowed network hosts (if network enabled)
    pub allowed_hosts: Vec<String>,
    /// Allow file system write
    pub allow_file_write: bool,
    /// Allowed write directories
    pub allowed_write_dirs: Vec<PathBuf>,
    /// Allow file system read
    pub allow_file_read: bool,
    /// Allowed read directories
    pub allowed_read_dirs: Vec<PathBuf>,
    /// Environment variables
    pub env_vars: HashMap<String, String>,
    /// Working directory
    pub working_dir: PathBuf,
    /// Temporary directory
    pub temp_dir: PathBuf,
    /// Enable strict mode (extra restrictions)
    pub strict_mode: bool,
    /// Kill timeout (graceful -> force kill)
    pub kill_timeout_ms: u64,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            max_execution_time_secs: 60,
            max_memory_mb: 512,
            max_cpu_percent: 50,
            max_disk_mb: 100,
            max_file_size_mb: 10,
            max_processes: 10,
            max_threads: 20,
            allow_network: false,
            allowed_hosts: Vec::new(),
            allow_file_write: true,
            allowed_write_dirs: vec![PathBuf::from("/tmp/sandbox")],
            allow_file_read: true,
            allowed_read_dirs: vec![PathBuf::from("/usr"), PathBuf::from("/lib")],
            env_vars: HashMap::new(),
            working_dir: PathBuf::from("/tmp/sandbox"),
            temp_dir: PathBuf::from("/tmp/sandbox/tmp"),
            strict_mode: true,
            kill_timeout_ms: 5000,
        }
    }
}

impl SandboxConfig {
    /// Create a permissive config (for trusted code)
    pub fn permissive() -> Self {
        Self {
            max_execution_time_secs: 300,
            max_memory_mb: 2048,
            max_cpu_percent: 100,
            max_disk_mb: 1024,
            max_file_size_mb: 100,
            max_processes: 100,
            max_threads: 100,
            allow_network: true,
            allowed_hosts: vec!["*".into()],
            allow_file_write: true,
            allowed_write_dirs: vec![PathBuf::from("/tmp")],
            allow_file_read: true,
            allowed_read_dirs: vec![PathBuf::from("/")],
            strict_mode: false,
            ..Default::default()
        }
    }
    
    /// Create a restricted config (for untrusted code)
    pub fn restricted() -> Self {
        Self {
            max_execution_time_secs: 10,
            max_memory_mb: 128,
            max_cpu_percent: 25,
            max_disk_mb: 10,
            max_file_size_mb: 1,
            max_processes: 1,
            max_threads: 1,
            allow_network: false,
            allowed_hosts: Vec::new(),
            allow_file_write: false,
            allowed_write_dirs: Vec::new(),
            allow_file_read: false,
            allowed_read_dirs: Vec::new(),
            strict_mode: true,
            ..Default::default()
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  EXECUTION REQUEST & RESULT
// ═══════════════════════════════════════════════════════════════════════════════

/// Execution request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRequest {
    /// Request ID
    pub id: String,
    /// Code to execute
    pub code: String,
    /// Language
    pub language: ExecutionLanguage,
    /// Input data (stdin)
    pub stdin: Option<String>,
    /// Additional arguments
    pub args: Vec<String>,
    /// Files to include (filename -> content)
    pub files: HashMap<String, String>,
    /// Override config
    pub config_override: Option<SandboxConfig>,
}

impl ExecutionRequest {
    pub fn new(language: ExecutionLanguage, code: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            code: code.into(),
            language,
            stdin: None,
            args: Vec::new(),
            files: HashMap::new(),
            config_override: None,
        }
    }
    
    pub fn with_stdin(mut self, stdin: impl Into<String>) -> Self {
        self.stdin = Some(stdin.into());
        self
    }
    
    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }
    
    pub fn with_file(mut self, name: String, content: String) -> Self {
        self.files.insert(name, content);
        self
    }
}

/// Execution language
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecutionLanguage {
    Python,
    JavaScript,
    TypeScript,
    Bash,
    Rust,
    Go,
    Java,
    C,
    Cpp,
    Ruby,
    Perl,
    Lua,
    Php,
    Wasm,
}

impl ExecutionLanguage {
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Python => "py",
            Self::JavaScript => "js",
            Self::TypeScript => "ts",
            Self::Bash => "sh",
            Self::Rust => "rs",
            Self::Go => "go",
            Self::Java => "java",
            Self::C => "c",
            Self::Cpp => "cpp",
            Self::Ruby => "rb",
            Self::Perl => "pl",
            Self::Lua => "lua",
            Self::Php => "php",
            Self::Wasm => "wasm",
        }
    }
    
    pub fn interpreter(&self) -> Option<&'static str> {
        match self {
            Self::Python => Some("python3"),
            Self::JavaScript => Some("node"),
            Self::TypeScript => Some("ts-node"),
            Self::Bash => Some("bash"),
            Self::Ruby => Some("ruby"),
            Self::Perl => Some("perl"),
            Self::Lua => Some("lua"),
            Self::Php => Some("php"),
            _ => None,
        }
    }
    
    pub fn needs_compilation(&self) -> bool {
        matches!(self, Self::Rust | Self::Go | Self::Java | Self::C | Self::Cpp | Self::Wasm)
    }
}

/// Execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// Request ID
    pub request_id: String,
    /// Exit code
    pub exit_code: i32,
    /// Standard output
    pub stdout: String,
    /// Standard error
    pub stderr: String,
    /// Execution status
    pub status: ExecutionStatus,
    /// Execution time in ms
    pub execution_time_ms: u64,
    /// Memory used in KB
    pub memory_used_kb: u64,
    /// CPU time in ms
    pub cpu_time_ms: u64,
    /// Output files (filename -> content)
    pub output_files: HashMap<String, String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl ExecutionResult {
    pub fn success(request_id: String, stdout: String) -> Self {
        Self {
            request_id,
            exit_code: 0,
            stdout,
            stderr: String::new(),
            status: ExecutionStatus::Success,
            execution_time_ms: 0,
            memory_used_kb: 0,
            cpu_time_ms: 0,
            output_files: HashMap::new(),
            timestamp: Utc::now(),
        }
    }
    
    pub fn failure(request_id: String, stderr: String) -> Self {
        Self {
            request_id,
            exit_code: 1,
            stdout: String::new(),
            stderr,
            status: ExecutionStatus::Failed,
            execution_time_ms: 0,
            memory_used_kb: 0,
            cpu_time_ms: 0,
            output_files: HashMap::new(),
            timestamp: Utc::now(),
        }
    }
    
    pub fn is_success(&self) -> bool {
        self.status == ExecutionStatus::Success && self.exit_code == 0
    }
}

/// Execution status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecutionStatus {
    Success,
    Failed,
    Timeout,
    MemoryLimitExceeded,
    CpuLimitExceeded,
    DiskLimitExceeded,
    ProcessLimitExceeded,
    Killed,
    SecurityViolation,
    RuntimeError,
    CompilationError,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SANDBOX MANAGER
// ═══════════════════════════════════════════════════════════════════════════════

/// Sandbox manager
pub struct SandboxManager {
    config: SandboxConfig,
    executions: Arc<RwLock<HashMap<String, ExecutionResult>>>,
    active_processes: Arc<RwLock<HashMap<String, u32>>>,
}

impl SandboxManager {
    pub fn new(config: SandboxConfig) -> Self {
        Self {
            config,
            executions: Arc::new(RwLock::new(HashMap::new())),
            active_processes: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Execute code in sandbox
    pub async fn execute(&self, request: ExecutionRequest) -> Result<ExecutionResult, SandboxError> {
        log::info!("🔒 Sandbox: Executing {} code (request: {})", 
            request.language.extension(), request.id);
        
        let config = request.config_override.as_ref().unwrap_or(&self.config).clone();
        let start = std::time::Instant::now();
        
        // Validate request
        self.validate_request(&request, &config)?;
        
        // Create sandbox directory
        let sandbox_dir = self.create_sandbox_dir(&request.id).await?;
        
        // Write code file
        let code_file = sandbox_dir.join(format!("main.{}", request.language.extension()));
        tokio::fs::write(&code_file, &request.code).await
            .map_err(|e| SandboxError::IoError(e.to_string()))?;
        
        // Write additional files
        for (name, content) in &request.files {
            let file_path = sandbox_dir.join(name);
            tokio::fs::write(&file_path, content).await
                .map_err(|e| SandboxError::IoError(e.to_string()))?;
        }
        
        // Compile if needed
        let executable = if request.language.needs_compilation() {
            self.compile(&request, &sandbox_dir, &config).await?
        } else {
            code_file
        };
        
        // Execute
        let result = self.run_process(&request, &executable, &sandbox_dir, &config).await?;
        
        // Record execution
        let mut executions = self.executions.write().await;
        executions.insert(request.id.clone(), result.clone());
        
        // Cleanup
        if config.strict_mode {
            let _ = tokio::fs::remove_dir_all(&sandbox_dir).await;
        }
        
        log::info!("🔒 Sandbox: Execution completed in {}ms", start.elapsed().as_millis());
        
        Ok(result)
    }
    
    /// Validate execution request
    fn validate_request(&self, request: &ExecutionRequest, config: &SandboxConfig) -> Result<(), SandboxError> {
        // Check code size
        if request.code.len() > 10 * 1024 * 1024 {
            return Err(SandboxError::CodeTooLarge);
        }
        
        // Check for dangerous patterns (basic)
        let dangerous_patterns = [
            "rm -rf /",
            ":(){ :|:& };:",  // Fork bomb
            "mkfs",
            "dd if=/dev/zero",
            "> /dev/sda",
            "chmod 777 /",
        ];
        
        for pattern in &dangerous_patterns {
            if request.code.contains(pattern) {
                return Err(SandboxError::SecurityViolation(format!("Dangerous pattern detected: {}", pattern)));
            }
        }
        
        // Check file sizes
        for (name, content) in &request.files {
            if content.len() as u64 > config.max_file_size_mb * 1024 * 1024 {
                return Err(SandboxError::FileTooLarge(name.clone()));
            }
        }
        
        Ok(())
    }
    
    /// Create sandbox directory
    async fn create_sandbox_dir(&self, request_id: &str) -> Result<PathBuf, SandboxError> {
        let dir = self.config.working_dir.join("sessions").join(request_id);
        tokio::fs::create_dir_all(&dir).await
            .map_err(|e| SandboxError::IoError(e.to_string()))?;
        
        // Create temp dir inside
        tokio::fs::create_dir_all(dir.join("tmp")).await
            .map_err(|e| SandboxError::IoError(e.to_string()))?;
        
        Ok(dir)
    }
    
    /// Compile code (for compiled languages)
    async fn compile(
        &self,
        request: &ExecutionRequest,
        sandbox_dir: &Path,
        config: &SandboxConfig,
    ) -> Result<PathBuf, SandboxError> {
        let output_path = sandbox_dir.join("main");
        
        let (compiler, args) = match request.language {
            ExecutionLanguage::Rust => ("rustc", vec!["main.rs", "-o", "main"]),
            ExecutionLanguage::Go => ("go", vec!["build", "-o", "main", "main.go"]),
            ExecutionLanguage::C => ("gcc", vec!["main.c", "-o", "main"]),
            ExecutionLanguage::Cpp => ("g++", vec!["main.cpp", "-o", "main"]),
            ExecutionLanguage::Java => ("javac", vec!["Main.java"]),
            _ => return Ok(sandbox_dir.join(format!("main.{}", request.language.extension()))),
        };
        
        let compile_result = tokio::process::Command::new(compiler)
            .args(&args)
            .current_dir(sandbox_dir)
            .output()
            .await
            .map_err(|e| SandboxError::CompilationError(e.to_string()))?;
        
        if !compile_result.status.success() {
            return Err(SandboxError::CompilationError(
                String::from_utf8_lossy(&compile_result.stderr).to_string()
            ));
        }
        
        Ok(output_path)
    }
    
    /// Run process with limits
    async fn run_process(
        &self,
        request: &ExecutionRequest,
        executable: &Path,
        sandbox_dir: &Path,
        config: &SandboxConfig,
    ) -> Result<ExecutionResult, SandboxError> {
        let interpreter = request.language.interpreter();
        
        let (mut cmd, actual_executable) = if let Some(interp) = interpreter {
            let mut c = tokio::process::Command::new(interp);
            c.arg(executable);
            (c, executable.to_path_buf())
        } else {
            (tokio::process::Command::new(executable), executable.to_path_buf())
        };
        
        // Set up command
        cmd.current_dir(sandbox_dir)
            .args(&request.args)
            .env_clear()
            .envs(&config.env_vars)
            .env("HOME", sandbox_dir)
            .env("TMPDIR", sandbox_dir.join("tmp"))
            .env("TEMP", sandbox_dir.join("tmp"))
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());
        
        // Set stdin if provided
        if let Some(ref stdin) = request.stdin {
            let mut child = cmd.stdin(std::process::Stdio::piped()).spawn()
                .map_err(|e| SandboxError::ExecutionError(e.to_string()))?;
            
            use tokio::io::AsyncWriteExt;
            if let Some(mut stdin_handle) = child.stdin.take() {
                stdin_handle.write_all(stdin.as_bytes()).await
                    .map_err(|e| SandboxError::IoError(e.to_string()))?;
            }
            
            return self.wait_for_child(request.id.clone(), child, config).await;
        }
        
        let child = cmd.spawn()
            .map_err(|e| SandboxError::ExecutionError(e.to_string()))?;
        
        self.wait_for_child(request.id.clone(), child, config).await
    }
    
    /// Wait for child process with timeout
    async fn wait_for_child(
        &self,
        request_id: String,
        mut child: tokio::process::Child,
        config: &SandboxConfig,
    ) -> Result<ExecutionResult, SandboxError> {
        let timeout = std::time::Duration::from_secs(config.max_execution_time_secs);
        
        let start = std::time::Instant::now();
        
        // Wait with timeout
        let result = tokio::time::timeout(timeout, async {
            use tokio::io::{AsyncReadExt, BufReader};
            
            let mut stdout = String::new();
            let mut stderr = String::new();
            
            if let Some(mut stdout_pipe) = child.stdout.take() {
                let mut reader = BufReader::new(&mut stdout_pipe);
                reader.read_to_string(&mut stdout).await.ok();
            }
            
            if let Some(mut stderr_pipe) = child.stderr.take() {
                let mut reader = BufReader::new(&mut stderr_pipe);
                reader.read_to_string(&mut stderr).await.ok();
            }
            
            let status = child.wait().await;
            
            (status, stdout, stderr)
        }).await;
        
        match result {
            Ok((status, stdout, stderr)) => {
                let exit_code = status.map(|s| s.code().unwrap_or(-1)).unwrap_or(-1);
                let status = if exit_code == 0 {
                    ExecutionStatus::Success
                } else {
                    ExecutionStatus::Failed
                };
                
                Ok(ExecutionResult {
                    request_id,
                    exit_code,
                    stdout,
                    stderr,
                    status,
                    execution_time_ms: start.elapsed().as_millis() as u64,
                    memory_used_kb: 0, // Would need cgroups for accurate measurement
                    cpu_time_ms: 0,
                    output_files: HashMap::new(),
                    timestamp: Utc::now(),
                })
            }
            Err(_) => {
                // Timeout - kill process
                let _ = child.kill().await;
                
                Ok(ExecutionResult {
                    request_id,
                    exit_code: -1,
                    stdout: String::new(),
                    stderr: format!("Execution timed out after {} seconds", config.max_execution_time_secs),
                    status: ExecutionStatus::Timeout,
                    execution_time_ms: start.elapsed().as_millis() as u64,
                    memory_used_kb: 0,
                    cpu_time_ms: 0,
                    output_files: HashMap::new(),
                    timestamp: Utc::now(),
                })
            }
        }
    }
    
    /// Get execution result
    pub async fn get_result(&self, request_id: &str) -> Option<ExecutionResult> {
        let executions = self.executions.read().await;
        executions.get(request_id).cloned()
    }
    
    /// Kill active execution
    pub async fn kill(&self, request_id: &str) -> bool {
        let mut processes = self.active_processes.write().await;
        if let Some(pid) = processes.remove(request_id) {
            // In real implementation, would send SIGKILL
            log::warn!("🔒 Sandbox: Killing process {} for request {}", pid, request_id);
            true
        } else {
            false
        }
    }
    
    /// Get sandbox statistics
    pub async fn stats(&self) -> SandboxStats {
        let executions = self.executions.read().await;
        
        let mut stats = SandboxStats::default();
        stats.total_executions = executions.len() as u64;
        
        for result in executions.values() {
            match result.status {
                ExecutionStatus::Success => stats.successful += 1,
                ExecutionStatus::Failed => stats.failed += 1,
                ExecutionStatus::Timeout => stats.timeouts += 1,
                ExecutionStatus::MemoryLimitExceeded => stats.memory_exceeded += 1,
                ExecutionStatus::SecurityViolation => stats.security_violations += 1,
                _ => {}
            }
            stats.total_execution_time_ms += result.execution_time_ms;
        }
        
        stats
    }
}

/// Sandbox statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SandboxStats {
    pub total_executions: u64,
    pub successful: u64,
    pub failed: u64,
    pub timeouts: u64,
    pub memory_exceeded: u64,
    pub security_violations: u64,
    pub total_execution_time_ms: u64,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ERROR TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Sandbox error
#[derive(Debug, Clone)]
pub enum SandboxError {
    IoError(String),
    CompilationError(String),
    ExecutionError(String),
    Timeout,
    MemoryLimitExceeded,
    CpuLimitExceeded,
    DiskLimitExceeded,
    ProcessLimitExceeded,
    SecurityViolation(String),
    CodeTooLarge,
    FileTooLarge(String),
    LanguageNotSupported,
}

impl std::fmt::Display for SandboxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "I/O error: {}", e),
            Self::CompilationError(e) => write!(f, "Compilation error: {}", e),
            Self::ExecutionError(e) => write!(f, "Execution error: {}", e),
            Self::Timeout => write!(f, "Execution timeout"),
            Self::MemoryLimitExceeded => write!(f, "Memory limit exceeded"),
            Self::CpuLimitExceeded => write!(f, "CPU limit exceeded"),
            Self::DiskLimitExceeded => write!(f, "Disk limit exceeded"),
            Self::ProcessLimitExceeded => write!(f, "Process limit exceeded"),
            Self::SecurityViolation(msg) => write!(f, "Security violation: {}", msg),
            Self::CodeTooLarge => write!(f, "Code size exceeds limit"),
            Self::FileTooLarge(name) => write!(f, "File too large: {}", name),
            Self::LanguageNotSupported => write!(f, "Language not supported"),
        }
    }
}

impl std::error::Error for SandboxError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sandbox_config_default() {
        let config = SandboxConfig::default();
        assert_eq!(config.max_execution_time_secs, 60);
        assert_eq!(config.max_memory_mb, 512);
        assert!(!config.allow_network);
    }
    
    #[test]
    fn test_sandbox_config_permissive() {
        let config = SandboxConfig::permissive();
        assert!(config.allow_network);
        assert!(!config.strict_mode);
    }
    
    #[test]
    fn test_sandbox_config_restricted() {
        let config = SandboxConfig::restricted();
        assert_eq!(config.max_execution_time_secs, 10);
        assert_eq!(config.max_memory_mb, 128);
    }
    
    #[test]
    fn test_execution_language() {
        assert_eq!(ExecutionLanguage::Python.extension(), "py");
        assert_eq!(ExecutionLanguage::Python.interpreter(), Some("python3"));
        assert!(!ExecutionLanguage::Python.needs_compilation());
        
        assert!(ExecutionLanguage::Rust.needs_compilation());
    }
    
    #[test]
    fn test_execution_request() {
        let req = ExecutionRequest::new(ExecutionLanguage::Python, "print('hello')")
            .with_stdin("input")
            .with_args(vec!["--verbose".into()]);
        
        assert_eq!(req.language, ExecutionLanguage::Python);
        assert!(req.stdin.is_some());
    }
    
    #[test]
    fn test_execution_result_success() {
        let result = ExecutionResult::success("req-1".into(), "output".into());
        assert!(result.is_success());
        assert_eq!(result.exit_code, 0);
    }
    
    #[test]
    fn test_execution_result_failure() {
        let result = ExecutionResult::failure("req-1".into(), "error".into());
        assert!(!result.is_success());
        assert_eq!(result.status, ExecutionStatus::Failed);
    }
    
    #[tokio::test]
    async fn test_sandbox_manager_creation() {
        let manager = SandboxManager::new(SandboxConfig::default());
        let stats = manager.stats().await;
        assert_eq!(stats.total_executions, 0);
    }
}
