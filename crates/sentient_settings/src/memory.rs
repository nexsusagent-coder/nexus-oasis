//! Memory Settings - Bellek ayarları

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySettings {
    /// Storage path
    pub storage_path: String,
    
    /// Max memory size (MB)
    pub max_size_mb: u32,
    
    /// Auto cleanup
    pub auto_cleanup: bool,
    
    /// Cleanup interval (saat)
    pub cleanup_interval: u32,
    
    /// Retention days
    pub retention_days: u32,
    
    /// Embedding model
    pub embedding_model: String,
    
    /// Vector dimensions
    pub vector_dimensions: usize,
    
    /// Similarity threshold
    pub similarity_threshold: f32,
    
    /// Max memories per query
    pub max_memories_per_query: usize,
    
    /// Enable semantic search
    pub semantic_search: bool,
    
    /// Enable episodic memory
    pub episodic_memory: bool,
    
    /// Enable semantic memory
    pub semantic_memory: bool,
    
    /// Enable procedural memory
    pub procedural_memory: bool,
}

impl Default for MemorySettings {
    fn default() -> Self {
        Self {
            storage_path: "./data/memory".to_string(),
            max_size_mb: 1024,
            auto_cleanup: true,
            cleanup_interval: 24,
            retention_days: 30,
            embedding_model: "all-MiniLM-L6-v2".to_string(),
            vector_dimensions: 384,
            similarity_threshold: 0.75,
            max_memories_per_query: 10,
            semantic_search: true,
            episodic_memory: true,
            semantic_memory: true,
            procedural_memory: true,
        }
    }
}
