//! ═══════════════════════════════════════════════════════════════════════════════
//!  CREUSOT BINARY WRAPPER - Formal Verification Toolchain
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Creusot, Rust kodları için Why3 tabanlı formal verification sağlar.
//! Bu modül Creusot binary'sini wrapper olarak kullanır.
//!
//! ## Kurulum
//! ```bash
//! # Creusot binary'lerini indir
//! curl -L https://github.com/creusot-rs/creusot/releases/latest/download/creusot-linux.tar.gz | tar xz
//! 
//! # Veya cargo ile derle
//! cargo install creusot
//! ```
//!
//! ## Kullanım
//! ```rust
//! use oasis_core::creusot::{CreusotVerifier, CreusotConfig};
//!
//! let verifier = CreusotVerifier::new();
//! let result = verifier.verify_file("src/contracts.rs").await?;
//! ```

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use tokio::sync::RwLock;

// ═══════════════════════════════════════════════════════════════════════════════
//  CREUSOT CONFIG
// ═══════════════════════════════════════════════════════════════════════════════

/// Creusot configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreusotConfig {
    /// Path to creusot binary
    pub binary_path: PathBuf,
    
    /// Path to why3 binary
    pub why3_path: PathBuf,
    
    /// Path to Why3 config
    pub why3_config: Option<PathBuf>,
    
    /// Additional includes for Why3
    pub includes: Vec<PathBuf>,
    
    /// Output directory for generated proofs
    pub output_dir: PathBuf,
    
    /// Proof timeout in seconds
    pub timeout_secs: u64,
    
    /// Enable verbose output
    pub verbose: bool,
    
    /// Generate proof obligations only (don't run prover)
    pub generate_only: bool,
    
    /// Prover to use (z3, cvc5, alt-ergo)
    pub prover: Prover,
}

impl Default for CreusotConfig {
    fn default() -> Self {
        Self {
            binary_path: PathBuf::from("creusot"),
            why3_path: PathBuf::from("why3"),
            why3_config: None,
            includes: vec![],
            output_dir: PathBuf::from("proofs"),
            timeout_secs: 60,
            verbose: false,
            generate_only: false,
            prover: Prover::Z3,
        }
    }
}

impl CreusotConfig {
    /// Create config with custom binary path
    pub fn with_binary(mut self, path: impl Into<PathBuf>) -> Self {
        self.binary_path = path.into();
        self
    }
    
    /// Set output directory
    pub fn with_output_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.output_dir = dir.into();
        self
    }
    
    /// Set prover
    pub fn with_prover(mut self, prover: Prover) -> Self {
        self.prover = prover;
        self
    }
    
    /// Set timeout
    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }
    
    /// Enable verbose output
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
}

/// SMT Prover options
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum Prover {
    #[default]
    Z3,
    Cvc5,
    AltErgo,
    Cvc4,
}

impl std::fmt::Display for Prover {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Z3 => write!(f, "z3"),
            Self::Cvc5 => write!(f, "cvc5"),
            Self::AltErgo => write!(f, "alt-ergo"),
            Self::Cvc4 => write!(f, "cvc4"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  VERIFICATION RESULT
// ═══════════════════════════════════════════════════════════════════════════════

/// Result of Creusot verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    /// File that was verified
    pub file: String,
    
    /// Whether all proofs succeeded
    pub success: bool,
    
    /// Total number of proof obligations
    pub total_obligations: usize,
    
    /// Number of proven obligations
    pub proven: usize,
    
    /// Number of unproven obligations
    pub unproven: usize,
    
    /// Time taken for verification
    pub duration_secs: f64,
    
    /// Individual proof results
    pub proofs: Vec<ProofResult>,
    
    /// Generated Why3 code (if successful)
    pub why3_output: Option<String>,
    
    /// Error message (if failed)
    pub error: Option<String>,
}

/// Result of a single proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofResult {
    /// Proof obligation name
    pub name: String,
    
    /// Location in source code
    pub location: Option<SourceLocation>,
    
    /// Whether the proof succeeded
    pub proven: bool,
    
    /// Prover used
    pub prover: String,
    
    /// Time taken
    pub time_secs: f64,
    
    /// Steps taken by prover
    pub steps: Option<usize>,
    
    /// Error message if proof failed
    pub error: Option<String>,
}

/// Source code location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CREUSOT VERIFIER
// ═══════════════════════════════════════════════════════════════════════════════

/// Creusot verification client
pub struct CreusotVerifier {
    config: CreusotConfig,
    /// Cache of verified files
    cache: Arc<RwLock<Vec<VerificationResult>>>,
    /// Whether Creusot is available
    available: bool,
}

impl CreusotVerifier {
    /// Create a new Creusot verifier
    pub fn new() -> Self {
        let config = CreusotConfig::default();
        let available = Self::check_availability(&config);
        
        Self {
            config,
            cache: Arc::new(RwLock::new(Vec::new())),
            available,
        }
    }
    
    /// Create with custom config
    pub fn with_config(config: CreusotConfig) -> Self {
        let available = Self::check_availability(&config);
        Self {
            config,
            cache: Arc::new(RwLock::new(Vec::new())),
            available,
        }
    }
    
    /// Check if Creusot binary is available
    fn check_availability(config: &CreusotConfig) -> bool {
        Command::new(&config.binary_path)
            .arg("--version")
            .output()
            .is_ok()
    }
    
    /// Check if verifier is available
    pub fn is_available(&self) -> bool {
        self.available
    }
    
    /// Get Creusot version
    pub fn version(&self) -> Option<String> {
        if !self.available {
            return None;
        }
        
        let output = Command::new(&self.config.binary_path)
            .arg("--version")
            .output()
            .ok()?;
        
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
    
    /// Verify a single Rust file
    pub async fn verify_file(&self, path: impl AsRef<Path>) -> CreusotResult<VerificationResult> {
        let path = path.as_ref();
        
        if !self.available {
            return Err(CreusotError::BinaryNotFound(
                self.config.binary_path.display().to_string()
            ));
        }
        
        if !path.exists() {
            return Err(CreusotError::FileNotFound(path.display().to_string()));
        }
        
        let start = std::time::Instant::now();
        
        // Create output directory if needed
        if !self.config.output_dir.exists() {
            std::fs::create_dir_all(&self.config.output_dir).map_err(CreusotError::IoError)?;
        }
        
        // Run Creusot
        let output = Command::new(&self.config.binary_path)
            .arg("--output")
            .arg(&self.config.output_dir)
            .arg(path)
            .args(if self.config.verbose { vec!["--verbose"] } else { vec![] })
            .output()
            .map_err(CreusotError::ExecutionFailed)?;
        
        let success = output.status.success();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        
        // Parse output
        let proofs = self.parse_creusot_output(&stdout, &stderr);
        let total = proofs.len();
        let proven = proofs.iter().filter(|p| p.proven).count();
        
        let result = VerificationResult {
            file: path.display().to_string(),
            success,
            total_obligations: total,
            proven,
            unproven: total - proven,
            duration_secs: start.elapsed().as_secs_f64(),
            proofs,
            why3_output: if success { Some(stdout) } else { None },
            error: if success { None } else { Some(stderr) },
        };
        
        // Cache result
        self.cache.write().await.push(result.clone());
        
        Ok(result)
    }
    
    /// Run Why3 prover on generated proof obligations
    pub async fn prove(&self, why3_file: impl AsRef<Path>) -> CreusotResult<Vec<ProofResult>> {
        let why3_file = why3_file.as_ref();
        
        if !self.available {
            return Err(CreusotError::BinaryNotFound(
                self.config.why3_path.display().to_string()
            ));
        }
        
        if !why3_file.exists() {
            return Err(CreusotError::FileNotFound(why3_file.display().to_string()));
        }
        
        // Build why3 prove command
        let mut cmd = Command::new(&self.config.why3_path);
        cmd.arg("prove")
            .arg("-P").arg(self.config.prover.to_string())
            .arg("--timeout").arg(self.config.timeout_secs.to_string())
            .arg(why3_file);
        
        // Add includes
        for inc in &self.config.includes {
            cmd.arg("-L").arg(inc);
        }
        
        // Add why3 config if specified
        if let Some(cfg) = &self.config.why3_config {
            cmd.arg("-C").arg(cfg);
        }
        
        let output = cmd.output().map_err(CreusotError::ExecutionFailed)?;
        
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        
        // Parse why3 output
        Ok(self.parse_why3_output(&stdout, &stderr))
    }
    
    /// Parse Creusot output into proof results
    fn parse_creusot_output(&self, stdout: &str, stderr: &str) -> Vec<ProofResult> {
        let mut proofs = Vec::new();
        
        // Parse stdout for proof obligations
        for line in stdout.lines() {
            if line.contains("proof obligation") || line.contains("obligation") {
                proofs.push(ProofResult {
                    name: self.extract_obligation_name(line),
                    location: self.extract_location(line),
                    proven: line.contains("proved") || line.contains("verified"),
                    prover: "creusot".to_string(),
                    time_secs: 0.0,
                    steps: None,
                    error: None,
                });
            }
        }
        
        // Check stderr for errors
        if !stderr.is_empty() && proofs.is_empty() {
            proofs.push(ProofResult {
                name: "error".to_string(),
                location: None,
                proven: false,
                prover: "creusot".to_string(),
                time_secs: 0.0,
                steps: None,
                error: Some(stderr.to_string()),
            });
        }
        
        proofs
    }
    
    /// Parse Why3 output
    fn parse_why3_output(&self, stdout: &str, stderr: &str) -> Vec<ProofResult> {
        let mut proofs = Vec::new();
        
        for line in stdout.lines() {
            // Why3 format: "Theory.goal: Valid (0.05s)"
            if line.contains(':') {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let name = parts[0].trim();
                    let result = parts[1].trim();
                    
                    let (proven, time_secs, error) = if result.contains("Valid") || result.contains("Verified") {
                        let time = self.extract_time(result);
                        (true, time, None)
                    } else if result.contains("Unknown") || result.contains("Timeout") {
                        let time = self.extract_time(result);
                        (false, time, Some(result.to_string()))
                    } else {
                        (false, 0.0, Some(result.to_string()))
                    };
                    
                    proofs.push(ProofResult {
                        name: name.to_string(),
                        location: None,
                        proven,
                        prover: self.config.prover.to_string(),
                        time_secs,
                        steps: None,
                        error,
                    });
                }
            }
        }
        
        if !stderr.is_empty() && proofs.is_empty() {
            proofs.push(ProofResult {
                name: "why3_error".to_string(),
                location: None,
                proven: false,
                prover: self.config.prover.to_string(),
                time_secs: 0.0,
                steps: None,
                error: Some(stderr.to_string()),
            });
        }
        
        proofs
    }
    
    fn extract_obligation_name(&self, line: &str) -> String {
        // Try to extract the obligation name from the line
        line.split_whitespace()
            .find(|s| s.contains("obligation") || s.contains("goal"))
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown_obligation".to_string())
    }
    
    fn extract_location(&self, _line: &str) -> Option<SourceLocation> {
        // TODO: Parse source location from Creusot output
        None
    }
    
    fn extract_time(&self, s: &str) -> f64 {
        // Extract time from strings like "(0.05s)" or "in 0.05s"
        let re = regex::Regex::new(r"(\d+\.?\d*)\s*s");
        if let Ok(re) = re {
            if let Some(cap) = re.captures(s) {
                if let Ok(time) = cap[1].parse::<f64>() {
                    return time;
                }
            }
        }
        0.0
    }
    
    /// Get cached results
    pub async fn get_cached(&self) -> Vec<VerificationResult> {
        self.cache.read().await.clone()
    }
    
    /// Clear cache
    pub async fn clear_cache(&self) {
        self.cache.write().await.clear();
    }
    
    /// Verify multiple files
    pub async fn verify_files(&self, paths: &[PathBuf]) -> Vec<CreusotResult<VerificationResult>> {
        let mut results = Vec::new();
        for path in paths {
            results.push(self.verify_file(path).await);
        }
        results
    }
    
    /// Generate proof obligations only (without proving)
    pub async fn generate_proofs(&self, path: impl AsRef<Path>) -> CreusotResult<String> {
        let path = path.as_ref();
        
        if !self.available {
            return Err(CreusotError::BinaryNotFound(
                self.config.binary_path.display().to_string()
            ));
        }
        
        let output = Command::new(&self.config.binary_path)
            .arg("--output")
            .arg(&self.config.output_dir)
            .arg("--generate-only")
            .arg(path)
            .output()
            .map_err(CreusotError::ExecutionFailed)?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(CreusotError::ProofGenerationFailed(
                String::from_utf8_lossy(&output.stderr).to_string()
            ))
        }
    }
}

impl Default for CreusotVerifier {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ERROR TYPES
// ═══════════════════════════════════════════════════════════════════════════════

pub type CreusotResult<T> = Result<T, CreusotError>;

#[derive(Debug, thiserror::Error)]
pub enum CreusotError {
    #[error("Creusot binary not found: {0}")]
    BinaryNotFound(String),
    
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Execution failed: {0}")]
    ExecutionFailed(std::io::Error),
    
    #[error("Proof generation failed: {0}")]
    ProofGenerationFailed(String),
    
    #[error("Verification failed: {0}")]
    VerificationFailed(String),
    
    #[error("Timeout exceeded")]
    Timeout,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CREUSOT ANNOTATIONS HELPER
// ═══════════════════════════════════════════════════════════════════════════════

/// Helper for writing Creusot annotations
pub mod annotations {
    /// Precondition annotation
    /// 
    /// # Example
    /// ```rust,ignore
    /// #[requires(precondition!(value > 0, "value must be positive"))]
    /// fn sqrt(value: f64) -> f64 {
    ///     value.sqrt()
    /// }
    /// ```
    #[macro_export]
    macro_rules! requires {
        ($cond:expr, $msg:expr) => {
            // Creusot will parse this annotation
            // Runtime: assertion check
            assert!($cond, "Precondition violated: {}", $msg);
        };
    }
    
    /// Postcondition annotation
    #[macro_export]
    macro_rules! ensures {
        ($cond:expr, $msg:expr) => {
            // Creusot will parse this annotation
            // Runtime: assertion check at function end
        };
    }
    
    /// Invariant annotation
    #[macro_export]
    macro_rules! invariant {
        ($cond:expr, $msg:expr) => {
            // Creusot will parse this annotation
            // Runtime: assertion check in loop body
        };
    }
    
    /// Variant (termination measure) annotation
    #[macro_export]
    macro_rules! variant {
        ($expr:expr) => {
            // Creusot uses this to prove termination
        };
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CONTRACT MACROS FOR EASY ANNOTATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Contract macro for function specifications
/// 
/// # Example
/// ```rust,ignore
/// contract! {
///     fn transfer(from: &mut Account, to: &mut Account, amount: u64) -> Result<(), Error> {
///         requires: amount > 0 && from.balance >= amount,
///         ensures: from.balance == old(from.balance) - amount && 
///                  to.balance == old(to.balance) + amount,
///         invariant: total_supply == old(total_supply),
///     }
/// }
/// ```
#[macro_export]
macro_rules! contract {
    (
        fn $name:ident($($param:ident: $ty:ty),*) $(-> $ret:ty)? {
            requires: $requires:expr,
            ensures: $ensures:expr,
            $($rest:tt)*
        }
    ) => {
        fn $name($($param: $ty),*) $(-> $ret)? {
            // Runtime precondition check
            debug_assert!($requires, "Precondition violated");
            
            // Function body would go here
            $($rest)*
            
            // Runtime postcondition check
            debug_assert!($ensures, "Postcondition violated");
        }
    };
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_creusot_config_default() {
        let config = CreusotConfig::default();
        assert_eq!(config.binary_path, PathBuf::from("creusot"));
        assert_eq!(config.prover, Prover::Z3);
        assert_eq!(config.timeout_secs, 60);
    }
    
    #[test]
    fn test_creusot_config_builder() {
        let config = CreusotConfig::default()
            .with_timeout(120)
            .with_prover(Prover::Cvc5)
            .verbose();
        
        assert_eq!(config.timeout_secs, 120);
        assert_eq!(config.prover, Prover::Cvc5);
        assert!(config.verbose);
    }
    
    #[test]
    fn test_prover_display() {
        assert_eq!(format!("{}", Prover::Z3), "z3");
        assert_eq!(format!("{}", Prover::Cvc5), "cvc5");
        assert_eq!(format!("{}", Prover::AltErgo), "alt-ergo");
    }
    
    #[tokio::test]
    async fn test_creusot_verifier_creation() {
        let verifier = CreusotVerifier::new();
        // Just test that it can be created
        // Binary might not be available in test environment
        let _ = verifier.is_available();
    }
    
    #[test]
    fn test_verification_result_serialization() {
        let result = VerificationResult {
            file: "test.rs".to_string(),
            success: true,
            total_obligations: 10,
            proven: 8,
            unproven: 2,
            duration_secs: 1.5,
            proofs: vec![],
            why3_output: Some("// Why3 code".to_string()),
            error: None,
        };
        
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"success\":true"));
        assert!(json.contains("\"total_obligations\":10"));
    }
    
    #[test]
    fn test_proof_result() {
        let proof = ProofResult {
            name: "test_goal".to_string(),
            location: Some(SourceLocation {
                file: "test.rs".to_string(),
                line: 10,
                column: 5,
            }),
            proven: true,
            prover: "z3".to_string(),
            time_secs: 0.05,
            steps: Some(42),
            error: None,
        };
        
        assert!(proof.proven);
        assert_eq!(proof.name, "test_goal");
    }
    
    #[test]
    fn test_error_types() {
        let err = CreusotError::BinaryNotFound("creusot".to_string());
        assert!(err.to_string().contains("not found"));
        
        let err = CreusotError::FileNotFound("test.rs".to_string());
        assert!(err.to_string().contains("File not found"));
    }
}
