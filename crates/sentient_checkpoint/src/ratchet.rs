//! ─── RATCHET ───
//!
//! Tek yönlü ilerleme mekanizması

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use crate::{RatchetError, RatchetResult};

/// Ratchet - Çark mekanizması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ratchet {
    /// Benzersiz ID
    pub id: Uuid,
    /// Yapılandırma
    pub config: RatchetConfig,
    /// Durum
    pub state: RatchetState,
    /// Adım geçmişi
    pub steps: Vec<RatchetStepRecord>,
    /// Oluşturulma zamanı
    pub created_at: DateTime<Utc>,
}

impl Ratchet {
    /// Yeni ratchet oluştur
    pub fn new(config: RatchetConfig) -> Self {
        let initial_hash = Self::compute_hash(&[]);
        
        Self {
            id: Uuid::new_v4(),
            config,
            state: RatchetState {
                step_count: 0,
                current_hash: initial_hash,
                data: serde_json::Value::Null,
                last_advanced: Utc::now(),
            },
            steps: Vec::new(),
            created_at: Utc::now(),
        }
    }
    
    /// İlerlet (sadece ileri)
    pub fn advance(&mut self, step: RatchetStep) -> RatchetResult<RatchetState> {
        // Yeni hash hesapla
        let new_hash = Self::compute_step_hash(&self.state.current_hash, &step);
        
        // Adım kaydı oluştur
        let record = RatchetStepRecord {
            step_number: self.state.step_count + 1,
            step: step.clone(),
            previous_hash: self.state.current_hash.clone(),
            new_hash: new_hash.clone(),
            timestamp: Utc::now(),
        };
        
        // Kaydet
        self.steps.push(record);
        
        // Durumu güncelle
        self.state.step_count += 1;
        self.state.current_hash = new_hash;
        self.state.data = step.data;
        self.state.last_advanced = Utc::now();
        
        Ok(self.state.clone())
    }
    
    /// Mevcut durumu getir
    pub fn current_state(&self) -> &RatchetState {
        &self.state
    }
    
    /// Belirli bir adıma ait veriyi getir
    pub fn get_step_data(&self, step_number: u64) -> Option<&serde_json::Value> {
        self.steps.iter()
            .find(|r| r.step_number == step_number)
            .map(|r| &r.step.data)
    }
    
    /// Toplam adım sayısı
    pub fn total_steps(&self) -> u64 {
        self.state.step_count
    }
    
    /// Hash hesapla
    fn compute_hash(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }
    
    /// Adım hash'i hesapla
    fn compute_step_hash(prev_hash: &str, step: &RatchetStep) -> String {
        let mut hasher = Sha256::new();
        hasher.update(prev_hash.as_bytes());
        hasher.update(step.name.as_bytes());
        hasher.update(step.data.to_string().as_bytes());
        hex::encode(hasher.finalize())
    }
}

/// Ratchet yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatchetConfig {
    /// Maksimum adım sayısı (0 = sınırsız)
    pub max_steps: u64,
    /// Otomatik checkpoint aralığı
    pub checkpoint_interval: u64,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl Default for RatchetConfig {
    fn default() -> Self {
        Self {
            max_steps: 0,
            checkpoint_interval: 100,
            metadata: HashMap::new(),
        }
    }
}

/// Ratchet durumu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatchetState {
    /// Adım sayısı
    pub step_count: u64,
    /// Mevcut hash
    pub current_hash: String,
    /// Mevcut veri
    pub data: serde_json::Value,
    /// Son ilerleme zamanı
    pub last_advanced: DateTime<Utc>,
}

/// Ratchet adımı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatchetStep {
    /// Adım adı
    pub name: String,
    /// Veri
    pub data: serde_json::Value,
    /// Metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Adım kaydı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatchetStepRecord {
    /// Adım numarası
    pub step_number: u64,
    /// Adım
    pub step: RatchetStep,
    /// Önceki hash
    pub previous_hash: String,
    /// Yeni hash
    pub new_hash: String,
    /// Zaman damgası
    pub timestamp: DateTime<Utc>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ratchet_creation() {
        let ratchet = Ratchet::new(RatchetConfig::default());
        assert_eq!(ratchet.state.step_count, 0);
        assert!(!ratchet.state.current_hash.is_empty());
    }
    
    #[test]
    fn test_ratchet_advance() {
        let mut ratchet = Ratchet::new(RatchetConfig::default());
        
        let step = RatchetStep {
            name: "init".into(),
            data: serde_json::json!({"status": "started"}),
            metadata: HashMap::new(),
        };
        
        let state = ratchet.advance(step).unwrap();
        assert_eq!(state.step_count, 1);
    }
    
    #[test]
    fn test_hash_chain() {
        let mut ratchet = Ratchet::new(RatchetConfig::default());
        
        let hash0 = ratchet.state.current_hash.clone();
        
        let step1 = RatchetStep {
            name: "step1".into(),
            data: serde_json::json!({}),
            metadata: HashMap::new(),
        };
        ratchet.advance(step1).unwrap();
        let hash1 = ratchet.state.current_hash.clone();
        
        let step2 = RatchetStep {
            name: "step2".into(),
            data: serde_json::json!({}),
            metadata: HashMap::new(),
        };
        ratchet.advance(step2).unwrap();
        let hash2 = ratchet.state.current_hash.clone();
        
        // Hash'ler farklı olmalı
        assert_ne!(hash0, hash1);
        assert_ne!(hash1, hash2);
        
        // Hash zinciri tutarlı olmalı
        assert_eq!(ratchet.steps[0].new_hash, hash1);
        assert_eq!(ratchet.steps[0].previous_hash, hash0);
    }
}
