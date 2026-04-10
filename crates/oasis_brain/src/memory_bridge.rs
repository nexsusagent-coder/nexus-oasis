//! ─── MEMORY BRIDGE ───
//!
//! Zero-copy integration with Memory Cube (L3)

use crate::{BrainError, reasoning::ReasoningResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

/// Zero-copy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCopyConfig {
    pub enabled: bool,
    pub auto_persist: bool,
}

impl Default for ZeroCopyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_persist: true,
        }
    }
}

/// Memory Bridge for L3 Integration
pub struct MemoryBridge {
    config: ZeroCopyConfig,
    /// Zero-copy buffers
    buffers: Arc<RwLock<Vec<ZeroCopyBuffer>>>,
    /// Memory stats
    stats: Arc<RwLock<MemoryStats>>,
}

impl MemoryBridge {
    pub fn new(config: ZeroCopyConfig) -> Self {
        Self {
            config,
            buffers: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(RwLock::new(MemoryStats::default())),
        }
    }
    
    /// Store reasoning result (zero-copy)
    pub async fn store_reasoning(&self, result: &ReasoningResult) -> Result<String, BrainError> {
        if !self.config.enabled {
            return Ok("zero_copy_disabled".to_string());
        }
        
        // Create zero-copy buffer
        let content = Arc::new(result.full_reasoning());
        let buffer = ZeroCopyBuffer {
            id: uuid::Uuid::new_v4().to_string(),
            content: content.clone(),
            memory_type: MemoryType::Reasoning,
            created_at: Utc::now(),
            token_count: content.len() / 4, // Approximate
        };
        
        let buffer_id = buffer.id.clone();
        
        // Store buffer
        {
            let mut buffers = self.buffers.write().await;
            buffers.push(buffer);
        }
        
        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.total_stored += 1;
            stats.reasoning_count += 1;
        }
        
        Ok(buffer_id)
    }
    
    /// Store generic content (zero-copy)
    pub async fn store(&self, content: String, memory_type: MemoryType) -> Result<String, BrainError> {
        if !self.config.enabled {
            return Ok("zero_copy_disabled".to_string());
        }
        
        let buffer = ZeroCopyBuffer {
            id: uuid::Uuid::new_v4().to_string(),
            content: Arc::new(content),
            memory_type,
            created_at: Utc::now(),
            token_count: 0,
        };
        
        let buffer_id = buffer.id.clone();
        
        {
            let mut buffers = self.buffers.write().await;
            buffers.push(buffer);
        }
        
        {
            let mut stats = self.stats.write().await;
            stats.total_stored += 1;
        }
        
        Ok(buffer_id)
    }
    
    /// Retrieve buffer by ID (zero-copy - returns Arc)
    pub async fn retrieve(&self, id: &str) -> Option<Arc<String>> {
        let buffers = self.buffers.read().await;
        buffers.iter()
            .find(|b| b.id == id)
            .map(|b| b.content.clone())
    }
    
    /// Get memory stats
    pub async fn stats(&self) -> MemoryStats {
        let stats = self.stats.read().await;
        stats.clone()
    }
    
    /// Clear buffers
    pub async fn clear(&self) {
        let mut buffers = self.buffers.write().await;
        buffers.clear();
    }
}

// ─── Types ───

/// Zero-copy buffer
#[derive(Debug, Clone)]
pub struct ZeroCopyBuffer {
    pub id: String,
    pub content: Arc<String>,
    pub memory_type: MemoryType,
    pub created_at: DateTime<Utc>,
    pub token_count: usize,
}

/// Memory type classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryType {
    Reasoning,
    Perception,
    Action,
    Conversation,
    Knowledge,
}

/// Memory statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryStats {
    pub total_stored: u64,
    pub reasoning_count: u64,
    pub perception_count: u64,
    pub action_count: u64,
}

impl Default for MemoryBridge {
    fn default() -> Self {
        Self::new(ZeroCopyConfig::default())
    }
}
