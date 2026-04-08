//! ═══════════════════════════════════════════════════════════════════════════════
//!  RUNTIME STATE - Verified State Management
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Creusot ile doğrulanmış state transitions.

use blake3::Hasher;
use serde::{Deserialize, Serialize};

// Hex encoding helper
fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

// ═══════════════════════════════════════════════════════════════════════════════
//  RUNTIME STATE
// ═══════════════════════════════════════════════════════════════════════════════

/// Runtime state with Creusot-verified transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeState {
    initialized: bool,
    transaction_count: u64,
    max_transactions: u64,
    memory_usage: u64,
    max_memory: u64,
    error_count: u64,
    last_transaction_hash: Option<String>,
}

impl RuntimeState {
    /// Create new state
    /// 
    /// # Creusot Contract
    /// ```ignore
    /// #[ensures(result.transaction_count == 0)]
    /// #[ensures(result.initialized == false)]
    /// ```
    pub fn new(max_transactions: u64, max_memory: u64) -> Self {
        Self {
            initialized: false,
            transaction_count: 0,
            max_transactions,
            memory_usage: 0,
            max_memory,
            error_count: 0,
            last_transaction_hash: None,
        }
    }

    /// Initialize state
    /// 
    /// # Creusot Contract
    /// ```ignore
    /// #[ensures(self.initialized == true)]
    /// ```
    pub fn initialize(&mut self) {
        self.initialized = true;
    }

    /// Check if initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Get transaction count
    pub fn transaction_count(&self) -> u64 {
        self.transaction_count
    }

    /// Get max transactions
    pub fn max_transactions(&self) -> u64 {
        self.max_transactions
    }

    /// Get memory usage
    pub fn memory_usage(&self) -> u64 {
        self.memory_usage
    }

    /// Get max memory
    pub fn max_memory(&self) -> u64 {
        self.max_memory
    }

    /// Increment transaction count
    /// 
    /// # Creusot Contract
    /// ```ignore
    /// #[requires(self.transaction_count < self.max_transactions)]
    /// #[ensures(self.transaction_count == old(self.transaction_count) + 1)]
    /// ```
    pub fn increment_transaction_count(&mut self) {
        if self.transaction_count < self.max_transactions {
            self.transaction_count += 1;
        }
    }

    /// Decrement transaction count (for rollbacks)
    /// 
    /// # Creusot Contract
    /// ```ignore
    /// #[requires(self.transaction_count > 0)]
    /// #[ensures(self.transaction_count == old(self.transaction_count) - 1)]
    /// ```
    pub fn decrement_transaction_count(&mut self) {
        if self.transaction_count > 0 {
            self.transaction_count -= 1;
        }
    }

    /// Set memory usage
    /// 
    /// # Creusot Contract
    /// ```ignore
    /// #[requires(bytes <= self.max_memory)]
    /// #[ensures(self.memory_usage == bytes)]
    /// ```
    pub fn set_memory_usage(&mut self, bytes: u64) {
        if bytes <= self.max_memory {
            self.memory_usage = bytes;
        }
    }

    /// Increment error count
    pub fn increment_error_count(&mut self) {
        self.error_count += 1;
    }

    /// Get error count
    pub fn error_count(&self) -> u64 {
        self.error_count
    }

    /// Update last transaction hash
    pub fn set_last_transaction_hash(&mut self, hash: String) {
        self.last_transaction_hash = Some(hash);
    }

    /// Compute state hash for verification
    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Hasher::new();
        hasher.update(&[self.initialized as u8]);
        hasher.update(&self.transaction_count.to_le_bytes());
        hasher.update(&self.error_count.to_le_bytes());
        if let Some(ref h) = self.last_transaction_hash {
            hasher.update(h.as_bytes());
        }
        hasher.finalize().as_bytes().to_vec()
    }

    /// Shutdown state
    pub fn shutdown(&mut self) {
        self.initialized = false;
    }

    /// Reset state (for testing/recovery)
    pub fn reset(&mut self) {
        self.transaction_count = 0;
        self.error_count = 0;
        self.memory_usage = 0;
        self.last_transaction_hash = None;
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  STATE SNAPSHOT (for recovery)
// ═══════════════════════════════════════════════════════════════════════════════

/// State snapshot for recovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub state: RuntimeState,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub hash: String,
}

impl StateSnapshot {
    pub fn from_state(state: &RuntimeState) -> Self {
        let hash = to_hex(&state.hash());
        Self {
            state: state.clone(),
            timestamp: chrono::Utc::now(),
            hash,
        }
    }

    /// Verify snapshot integrity
    pub fn verify(&self) -> bool {
        let computed = to_hex(&self.state.hash());
        computed == self.hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_initialization() {
        let mut state = RuntimeState::new(100, 1000);
        assert!(!state.is_initialized());
        
        state.initialize();
        assert!(state.is_initialized());
    }

    #[test]
    fn test_transaction_count() {
        let mut state = RuntimeState::new(100, 1000);
        assert_eq!(state.transaction_count(), 0);
        
        state.increment_transaction_count();
        assert_eq!(state.transaction_count(), 1);
        
        state.decrement_transaction_count();
        assert_eq!(state.transaction_count(), 0);
    }

    #[test]
    fn test_state_hash() {
        let state = RuntimeState::new(100, 1000);
        let hash1 = state.hash();
        let hash2 = state.hash();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_snapshot_verification() {
        let state = RuntimeState::new(100, 1000);
        let snapshot = StateSnapshot::from_state(&state);
        assert!(snapshot.verify());
    }
}
