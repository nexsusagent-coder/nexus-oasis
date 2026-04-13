//! ═══════════════════════════════════════════════════════════════════════════════
//!  OASIS MANUS - L5: EXECUTION KATMANI
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! OpenManus aracının SENTIENT'ya tam asimilasyonu.
//! Docker içinde yalıtılmış kod çalıştırma ve otonom görev planlama.
//! 
//! ═──────────────────────────────────────────────────────────────────────────────
//!  L1 SOVEREIGN ANAYASASI:
//!  ───────────────────────

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
//!  ✓ Kod SADECE Docker container içinde çalışır
//!  ✓ Ana sisteme DOSYA ERİŞİMİ YASAKTIR
//!  ✓ Ağ erişimi KISITLANABİLİR (default: kapalı)
//!  ✓ Memory/CPU limitleri ZORUNLUDUR
//!  ✓ Timeout ZORUNLUDUR (max 5 dakika)
//!  ✓ Tüm hatalar SENTIENT diline çevrilir
//! ═──────────────────────────────────────────────────────────────────────────────
//!
//! MİMARİ:
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │                         OASIS MANUS                                          │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │                                                                             │
//! │  ┌─────────────────────────────────────────────────────────────────────┐   │
//! │  │                    SOVEREIGN SANDBOX (L1)                           │   │
//! │  │  ┌───────────────┐  ┌───────────────┐  ┌───────────────┐           │   │
//! │  │  │ FileSystem    │  │ Network       │  │ Resource      │           │   │
//! │  │  │   CONTAINER   │  │   OPTIONAL    │  │   LIMITED     │           │   │
//! │  │  └───────────────┘  └───────────────┘  └───────────────┘           │   │
//! │  └─────────────────────────────────────────────────────────────────────┘   │
//! │                                    │                                        │
//! │                                    ▼                                        │
//! │  ┌─────────────────────────────────────────────────────────────────────┐   │
//! │  │                    MANUS AGENT                                       │   │
//! │  │  ┌───────────┐  ┌───────────┐  ┌───────────┐  ┌───────────┐        │   │
//! │  │  │  Plan     │  │  Code     │  │  Execute  │  │  Verify   │        │   │
//! │  │  └───────────┘  └───────────┘  └───────────┘  └───────────┘        │   │
//! │  └─────────────────────────────────────────────────────────────────────┘   │
//! │                                    │                                        │
//! │                                    ▼                                        │
//! │  ┌─────────────────────────────────────────────────────────────────────┐   │
//! │  │                    CONTAINER POOL                                    │   │
//! │  │  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐                │   │
//! │  │  │ Python  │  │ Node.js │  │  Bash   │  │  Custom │                │   │
//! │  │  └─────────┘  └─────────┘  └─────────┘  └─────────┘                │   │
//! │  └─────────────────────────────────────────────────────────────────────┘   │
//! │                                    │                                        │
//! │                                    ▼                                        │
//! │  ┌─────────────────────────────────────────────────────────────────────┐   │
//! │  │                    V-GATE (L2)                                       │   │
//! │  │  LLM Request → Guardrails → Encrypted Channel → LLM Response       │   │
//! │  └─────────────────────────────────────────────────────────────────────┘   │
//! │                                                                             │
//! └─────────────────────────────────────────────────────────────────────────────┘
//! ```

pub mod error;
pub mod sovereign;
pub mod container;
pub mod executor;
pub mod planner;
pub mod agent;
pub mod vgate;
pub mod tools;
pub mod session;

// Re-exports
pub use error::{ManusError, ManusResult};
pub use sovereign::{SovereignSandbox, SandboxPolicy, ResourceLimits};
pub use container::{ContainerPool, ContainerConfig, ContainerStatus};
pub use executor::{CodeExecutor, ExecutionResult, ExecutionLanguage};
pub use planner::{TaskPlanner, TaskPlan, TaskStep, StepType};
pub use agent::{ManusAgent, AgentConfig, AgentState, ManusTask};
pub use vgate::ManusVGate;
pub use tools::{ToolRegistry, Tool, ToolResult};
pub use session::{ManusSession, SessionStats};

/// OpenManus asimilasyon sürümü
pub const OASIS_MANUS_VERSION: &str = "0.1.0-sentient";

/// Desteklenen diller
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Language {
    Python,
    JavaScript,
    TypeScript,
    Bash,
    Rust,
    Go,
}

impl Language {
    pub fn extension(&self) -> &'static str {
        match self {
            Language::Python => ".py",
            Language::JavaScript => ".js",
            Language::TypeScript => ".ts",
            Language::Bash => ".sh",
            Language::Rust => ".rs",
            Language::Go => ".go",
        }
    }
    
    pub fn docker_image(&self) -> &'static str {
        match self {
            Language::Python => "python:3.11-slim",
            Language::JavaScript => "node:20-slim",
            Language::TypeScript => "node:20-slim",
            Language::Bash => "bash:5.2",
            Language::Rust => "rust:1.75-slim",
            Language::Go => "golang:1.21-alpine",
        }
    }
    
    pub fn run_command(&self, filename: &str) -> Vec<String> {
        match self {
            Language::Python => vec!["python3".into(), filename.into()],
            Language::JavaScript => vec!["node".into(), filename.into()],
            Language::TypeScript => vec!["npx".into(), "ts-node".into(), filename.into()],
            Language::Bash => vec!["bash".into(), filename.into()],
            Language::Rust => vec!["rustc".into(), filename.into(), "-o".into(), "/tmp/out".into(), "&&".into(), "/tmp/out".into()],
            Language::Go => vec!["go".into(), "run".into(), filename.into()],
        }
    }
}

/// ─── OASIS MANUS YÖNETİCİSİ ───

pub struct OasisManus {
    /// Sovereign Sandbox
    sandbox: SovereignSandbox,
    /// Container havuzu
    pool: ContainerPool,
    /// Kod çalıştırıcı
    executor: CodeExecutor,
    /// Manus Agent
    agent: ManusAgent,
    /// V-GATE köprüsü
    vgate: ManusVGate,
    /// Aktif oturumlar
    sessions: Vec<ManusSession>,
}

impl OasisManus {
    /// Yeni Oasis Manus oluştur
    pub async fn new() -> ManusResult<Self> {
        log::info!("╔════════════════════════════════════════════════════════════════╗");
        log::info!("║  OASIS MANUS v{} - L5: EXECUTION                        ║", OASIS_MANUS_VERSION);
        log::info!("╚════════════════════════════════════════════════════════════════╝");
        
        let sandbox = SovereignSandbox::new(SandboxPolicy::sovereign());
        let pool = ContainerPool::new().await?;
        let executor = CodeExecutor::new(&pool);
        let vgate = ManusVGate::new("http://localhost:1071");
        let agent = ManusAgent::new(&vgate, &executor);
        
        Ok(Self {
            sandbox,
            pool,
            executor,
            agent,
            vgate,
            sessions: Vec::new(),
        })
    }
    
    /// Kod çalıştır
    pub async fn execute(&mut self, code: &str, language: Language) -> ManusResult<ExecutionResult> {
        log::info!("🐍  MANUS: Kod çalıştırma isteği ({:?})", language);
        
        // Sovereign kontrol
        self.sandbox.validate_code(code)?;
        
        // Container al
        let container_id = self.pool.acquire(language).await?;
        
        // Çalıştır
        let result = self.executor.execute(&container_id, code, language).await;
        
        // Container bırak
        self.pool.release(&container_id).await?;
        
        result
    }
    
    /// Görev planla ve çalıştır
    pub async fn run_task(&mut self, task: &str) -> ManusResult<String> {
        log::info!("🎯  MANUS: Görev başlatılıyor → {}", task);
        
        // Agent ile çalıştır
        let result = self.agent.run(task).await?;
        
        Ok(result)
    }
    
    /// Durum raporu
    pub fn status(&self) -> ManusStatus {
        ManusStatus {
            version: OASIS_MANUS_VERSION.to_string(),
            containers: self.pool.status(),
            sessions: self.sessions.len(),
            sandbox_active: self.sandbox.is_active(),
        }
    }
    
    /// Temizlik
    pub async fn cleanup(&mut self) -> ManusResult<()> {
        log::info!("🧹  MANUS: Temizlik yapılıyor...");
        self.pool.cleanup_all().await?;
        self.sessions.clear();
        Ok(())
    }
}

/// Manus durumu
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ManusStatus {
    pub version: String,
    pub containers: PoolStatus,
    pub sessions: usize,
    pub sandbox_active: bool,
}

/// Container havuz durumu
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PoolStatus {
    pub total: usize,
    pub active: usize,
    pub available: usize,
}

impl Default for PoolStatus {
    fn default() -> Self {
        Self {
            total: 0,
            active: 0,
            available: 0,
        }
    }
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_extension() {
        assert_eq!(Language::Python.extension(), ".py");
        assert_eq!(Language::JavaScript.extension(), ".js");
        assert_eq!(Language::Rust.extension(), ".rs");
    }

    #[test]
    fn test_language_docker_image() {
        assert!(Language::Python.docker_image().contains("python"));
        assert!(Language::JavaScript.docker_image().contains("node"));
    }

    #[test]
    fn test_language_run_command() {
        let cmd = Language::Python.run_command("test.py");
        assert_eq!(cmd[0], "python3");
    }

    #[test]
    fn test_pool_status_default() {
        let status = PoolStatus::default();
        assert_eq!(status.total, 0);
    }
}
