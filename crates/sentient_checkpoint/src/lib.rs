//! ─── A4 RATCHET PATTERN ───
//!
//! SENTIENT'nın ilerleme kaydetme sistemi.
//! Çark mekanizması - sadece ileri gider, geri döndürülemez.
//!
//! Özellikler:
//! - Aşamalı ilerleme kaydı
//! - Hash zinciri ile bütünlük
//! - Checkpoint oluşturma
//! - Durum kurtarma

pub mod ratchet;
pub mod chain;
pub mod recovery;

pub use ratchet::{Ratchet, RatchetConfig, RatchetState, RatchetStep};
pub use chain::{Chain, ChainBlock, ChainVerifier};
pub use recovery::{RecoveryManager, RecoveryPoint};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};

// ═══════════════════════════════════════════════════════════════════════════════
// RATCHET ERROR
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, thiserror::Error)]
pub enum RatchetError {
    #[error("Ratchet hatası: {0}")]
    General(String),
    
    #[error("Geçersiz hash zinciri: {0}")]
    InvalidChain(String),
    
    #[error("Geri alma girişimi: {0}")]
    ReverseAttempt(String),
    
    #[error("Kurtarma hatası: {0}")]
    RecoveryError(String),
}

pub type RatchetResult<T> = Result<T, RatchetError>;

// ═══════════════════════════════════════════════════════════════════════════════
// RATCHET MANAGER
// ═══════════════════════════════════════════════════════════════════════════════

/// Ratchet yöneticisi
pub struct RatchetManager {
    ratchets: Arc<RwLock<HashMap<Uuid, Ratchet>>>,
    chain: Arc<RwLock<Chain>>,
    recovery: RecoveryManager,
}

impl RatchetManager {
    pub fn new() -> Self {
        Self {
            ratchets: Arc::new(RwLock::new(HashMap::new())),
            chain: Arc::new(RwLock::new(Chain::new())),
            recovery: RecoveryManager::new("data/ratchet"),
        }
    }
    
    /// Yeni ratchet oluştur
    pub async fn create(&self, config: RatchetConfig) -> RatchetResult<Uuid> {
        let ratchet = Ratchet::new(config);
        let id = ratchet.id;
        
        self.ratchets.write().await.insert(id, ratchet);
        
        Ok(id)
    }
    
    /// Ratchet ilerlet
    pub async fn advance(&self, id: Uuid, step: RatchetStep) -> RatchetResult<RatchetState> {
        let mut ratchets = self.ratchets.write().await;
        let ratchet = ratchets.get_mut(&id)
            .ok_or_else(|| RatchetError::General(format!("Ratchet bulunamadı: {}", id)))?;
        
        // İlerlet
        let state = ratchet.advance(step)?;
        
        // Zincire ekle
        self.chain.write().await.add_block(
            id,
            state.step_count,
            state.current_hash.clone(),
            state.data.clone(),
        )?;
        
        Ok(state)
    }
    
    /// Ratchet durumu getir
    pub async fn get_state(&self, id: Uuid) -> RatchetResult<RatchetState> {
        let ratchets = self.ratchets.read().await;
        let ratchet = ratchets.get(&id)
            .ok_or_else(|| RatchetError::General(format!("Ratchet bulunamadı: {}", id)))?;
        
        Ok(ratchet.state.clone())
    }
    
    /// Checkpoint al
    pub async fn checkpoint(&self, id: Uuid) -> RatchetResult<String> {
        let ratchets = self.ratchets.read().await;
        let ratchet = ratchets.get(&id)
            .ok_or_else(|| RatchetError::General(format!("Ratchet bulunamadı: {}", id)))?;
        
        self.recovery.save(ratchet).await
    }
    
    /// Kurtar
    pub async fn recover(&self, checkpoint_id: &str) -> RatchetResult<Ratchet> {
        self.recovery.load(checkpoint_id).await
    }
    
    /// Zinciri doğrula
    pub async fn verify_chain(&self, id: Uuid) -> RatchetResult<bool> {
        self.chain.read().await.verify(id)
    }
}

impl Default for RatchetManager {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_ratchet_creation() {
        let manager = RatchetManager::new();
        let id = manager.create(RatchetConfig::default()).await.unwrap();
        
        let state = manager.get_state(id).await.unwrap();
        assert_eq!(state.step_count, 0);
    }
    
    #[tokio::test]
    async fn test_ratchet_advance() {
        let manager = RatchetManager::new();
        let id = manager.create(RatchetConfig::default()).await.unwrap();
        
        let step = RatchetStep {
            name: "test".into(),
            data: serde_json::json!({"value": 1}),
            metadata: HashMap::new(),
        };
        
        let state = manager.advance(id, step).await.unwrap();
        assert_eq!(state.step_count, 1);
    }
    
    #[tokio::test]
    async fn test_chain_verification() {
        let manager = RatchetManager::new();
        let id = manager.create(RatchetConfig::default()).await.unwrap();
        
        // İlerle
        for i in 0..5 {
            let step = RatchetStep {
                name: format!("step_{}", i),
                data: serde_json::json!({"index": i}),
                metadata: HashMap::new(),
            };
            manager.advance(id, step).await.unwrap();
        }
        
        // Doğrula
        let valid = manager.verify_chain(id).await.unwrap();
        assert!(valid);
    }
}
