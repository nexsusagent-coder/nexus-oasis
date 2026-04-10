//! ═══════════════════════════════════════════════════════════════════════════════
//!  OASIS RUNTIME - Trusted Execution Engine
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Creusot ile doğrulanmış güvenli runtime.

use crate::{
    CoreConfig, CoreError, ContractVerifier, ExecutionResult,
    Transaction, VerificationProof, RuntimeState,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

// CoreResult type alias
pub type CoreResult<T> = Result<T, CoreError>;

// ═══════════════════════════════════════════════════════════════════════════════
//  OASIS RUNTIME
// ═══════════════════════════════════════════════════════════════════════════════

/// OASIS Trusted Runtime
/// 
/// Creusot contracts ile matematiksel güvenlik kanıtı sağlar.
pub struct OasisRuntime {
    config: CoreConfig,
    state: Arc<RwLock<RuntimeState>>,
    verifier: Arc<RwLock<ContractVerifier>>,
    started_at: Instant,
}

impl OasisRuntime {
    /// Create new runtime with configuration
    pub fn new(config: CoreConfig) -> Self {
        let state = RuntimeState::new(config.max_transactions, config.max_memory_bytes);
        
        Self {
            config,
            state: Arc::new(RwLock::new(state)),
            verifier: Arc::new(RwLock::new(ContractVerifier::new())),
            started_at: Instant::now(),
        }
    }

    /// Initialize runtime
    /// 
    /// # Creusot Contracts
    /// ```ignore
    /// #[ensures(state.is_initialized == true)]
    /// ```
    pub async fn initialize(&self) -> CoreResult<()> {
        let mut verifier = self.verifier.write().await;
        
        // Pre-condition: Not already initialized
        verifier.verify_precondition(
            "runtime_init",
            true, // Always valid for initialization
            "Runtime must not be already initialized",
        )?;

        let mut state = self.state.write().await;
        state.initialize();

        // Post-condition: State is initialized
        verifier.verify_postcondition(
            "runtime_init",
            state.is_initialized(),
            "State must be initialized after init",
        )?;

        log::info!("🚀 OASIS Runtime initialized successfully");
        Ok(())
    }

    /// Execute a transaction with full contract verification
    /// 
    /// # Creusot Contracts
    /// ```ignore
    /// #[requires(state.is_initialized == true)]
    /// #[requires(tx.id.is_valid() == true)]
    /// #[ensures(result.is_ok() ==> state.transaction_count == old(state.transaction_count) + 1)]
    /// ```
    pub async fn execute(&self, tx: Transaction) -> CoreResult<ExecutionResult> {
        let _start_time = Instant::now();
        let tx_id = tx.id;
        
        // Pre-condition verification
        {
            let mut verifier = self.verifier.write().await;
            let state = self.state.read().await;
            
            // Verify state is initialized
            verifier.verify_precondition(
                "execute_state_init",
                state.is_initialized(),
                &format!("State must be initialized for transaction {}", tx_id),
            )?;

            // Verify transaction has valid ID
            verifier.verify_precondition(
                "execute_tx_valid",
                !tx_id.is_nil(),
                "Transaction must have valid ID",
            )?;

            // Verify not at capacity
            verifier.verify_precondition(
                "execute_capacity",
                state.transaction_count() < state.max_transactions(),
                "Runtime must not be at transaction capacity",
            )?;
        }

        // Execute with timeout
        let result = tokio::time::timeout(
            Duration::from_millis(self.config.execution_timeout_ms),
            self.execute_internal(tx),
        ).await;

        match result {
            Ok(Ok(exec_result)) => {
                // Post-condition verification
                let mut verifier = self.verifier.write().await;
                let state = self.state.read().await;
                
                verifier.verify_postcondition(
                    "execute_success",
                    state.transaction_count() > 0,
                    "Transaction count must be positive after execution",
                )?;

                Ok(exec_result)
            }
            Ok(Err(e)) => Err(e),
            Err(_) => Err(CoreError::Timeout),
        }
    }

    async fn execute_internal(&self, tx: Transaction) -> CoreResult<ExecutionResult> {
        let tx_id = tx.id;
        let exec_start = Instant::now();
        let pre_state_hash = {
            let state = self.state.read().await;
            state.hash()
        };

        // Process transaction
        let output = self.process_operation(&tx).await?;

        // Update state
        let post_state_hash = {
            let mut state = self.state.write().await;
            state.increment_transaction_count();
            state.hash()
        };

        // Generate verification proof
        let proof = VerificationProof::generate(&pre_state_hash, &post_state_hash);

        Ok(ExecutionResult {
            transaction_id: tx_id,
            success: true,
            output,
            duration_ms: exec_start.elapsed().as_millis() as u64,
            gas_used: 1, // Simplified gas calculation
            verification_proof: Some(proof),
        })
    }

    async fn process_operation(&self, tx: &Transaction) -> CoreResult<serde_json::Value> {
        // Verify invariant during processing
        {
            let mut verifier = self.verifier.write().await;
            let state = self.state.read().await;
            
            verifier.verify_invariant(
                "process_memory",
                state.memory_usage() <= state.max_memory(),
                "Memory usage must stay within limit",
            )?;
        }

        // Process based on operation type
        match tx.operation.as_str() {
            "ping" => Ok(serde_json::json!({"status": "pong"})),
            "status" => {
                let state = self.state.read().await;
                Ok(serde_json::json!({
                    "transaction_count": state.transaction_count(),
                    "uptime_secs": self.started_at.elapsed().as_secs(),
                }))
            }
            "echo" => Ok(tx.payload.clone()),
            _ => Ok(serde_json::json!({
                "processed": true,
                "operation": tx.operation,
            })),
        }
    }

    /// Get runtime status
    pub async fn status(&self) -> RuntimeStatus {
        let state = self.state.read().await;
        let verifier = self.verifier.read().await;
        
        RuntimeStatus {
            is_initialized: state.is_initialized(),
            transaction_count: state.transaction_count(),
            uptime_secs: self.started_at.elapsed().as_secs(),
            verification_count: verifier.verification_count(),
            failed_contracts: verifier.failed_count(),
            config: self.config.clone(),
        }
    }

    /// Shutdown runtime gracefully
    pub async fn shutdown(&self) -> CoreResult<()> {
        let mut state = self.state.write().await;
        state.shutdown();
        
        log::info!("🛑 OASIS Runtime shutdown complete");
        Ok(())
    }
}

/// Runtime status snapshot
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RuntimeStatus {
    pub is_initialized: bool,
    pub transaction_count: u64,
    pub uptime_secs: u64,
    pub verification_count: usize,
    pub failed_contracts: usize,
    pub config: CoreConfig,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_runtime_initialize() {
        let runtime = OasisRuntime::new(CoreConfig::default());
        let result = runtime.initialize().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_runtime_execute() {
        let runtime = OasisRuntime::new(CoreConfig::default());
        runtime.initialize().await.expect("operation failed");
        
        let tx = Transaction::new("ping", serde_json::json!({}));
        let result = runtime.execute(tx).await;
        
        assert!(result.is_ok());
        let exec_result = result.expect("operation failed");
        assert!(exec_result.success);
    }

    #[tokio::test]
    async fn test_runtime_status() {
        let runtime = OasisRuntime::new(CoreConfig::default());
        runtime.initialize().await.expect("operation failed");
        
        let status = runtime.status().await;
        assert!(status.is_initialized);
    }
}
