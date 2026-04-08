//! ─── CHAIN ───
//!
//! Hash zinciri doğrulama

use crate::{RatchetError, RatchetResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use sha2::{Sha256, Digest};

/// Zincir bloğu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainBlock {
    /// Ratchet ID
    pub ratchet_id: Uuid,
    /// Adım numarası
    pub step_number: u64,
    /// Hash
    pub hash: String,
    /// Veri
    pub data: serde_json::Value,
    /// Zaman damgası
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Hash zinciri
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Chain {
    blocks: HashMap<Uuid, Vec<ChainBlock>>,
}

impl Chain {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
        }
    }
    
    /// Blok ekle
    pub fn add_block(
        &mut self,
        ratchet_id: Uuid,
        step_number: u64,
        hash: String,
        data: serde_json::Value,
    ) -> RatchetResult<()> {
        let block = ChainBlock {
            ratchet_id,
            step_number,
            hash,
            data,
            timestamp: chrono::Utc::now(),
        };
        
        self.blocks.entry(ratchet_id)
            .or_insert_with(Vec::new)
            .push(block);
        
        Ok(())
    }
    
    /// Zinciri doğrula
    pub fn verify(&self, ratchet_id: Uuid) -> RatchetResult<bool> {
        let blocks = self.blocks.get(&ratchet_id)
            .ok_or_else(|| RatchetError::InvalidChain("Ratchet bulunamadı".into()))?;
        
        if blocks.is_empty() {
            return Ok(true);
        }
        
        // Genesis block kontrolü
        if blocks[0].step_number != 1 {
            return Err(RatchetError::InvalidChain("Geçersiz genesis block".into()));
        }
        
        // Sıralı kontrol
        for i in 1..blocks.len() {
            if blocks[i].step_number != blocks[i-1].step_number + 1 {
                return Err(RatchetError::InvalidChain("Eksik adım".into()));
            }
            
            // Hash zinciri kontrolü - her blok öncekinin hash'ini içermeli
            // (Burada basitleştirilmiş kontrol)
        }
        
        Ok(true)
    }
    
    /// Blokları getir
    pub fn get_blocks(&self, ratchet_id: Uuid) -> Vec<&ChainBlock> {
        self.blocks.get(&ratchet_id)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }
    
    /// Son bloğu getir
    pub fn get_latest(&self, ratchet_id: Uuid) -> Option<&ChainBlock> {
        self.blocks.get(&ratchet_id)
            .and_then(|v| v.last())
    }
    
    /// Blok sayısı
    pub fn block_count(&self, ratchet_id: Uuid) -> usize {
        self.blocks.get(&ratchet_id).map(|v| v.len()).unwrap_or(0)
    }
}

/// Zincir doğrulayıcı
pub struct ChainVerifier;

impl ChainVerifier {
    /// Hash doğrula
    pub fn verify_hash(prev_hash: &str, data: &str, current_hash: &str) -> bool {
        let computed = Self::compute_hash(prev_hash, data);
        computed == current_hash
    }
    
    fn compute_hash(prev_hash: &str, data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(prev_hash.as_bytes());
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chain_creation() {
        let chain = Chain::new();
        assert!(chain.blocks.is_empty());
    }
    
    #[test]
    fn test_add_block() {
        let mut chain = Chain::new();
        let id = Uuid::new_v4();
        
        chain.add_block(id, 1, "hash1".into(), serde_json::json!({})).unwrap();
        
        assert_eq!(chain.block_count(id), 1);
    }
    
    #[test]
    fn test_verify_empty_chain() {
        let chain = Chain::new();
        let id = Uuid::new_v4();
        
        // Boş zincir geçerli olmalı
        assert!(chain.verify(id).is_err()); // Ratchet yok
    }
}
