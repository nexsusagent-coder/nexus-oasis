//! ─── LanceDB Memory Storage ───

use crate::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Memory entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    /// Unique ID
    pub id: String,
    
    /// Memory content
    pub content: String,
    
    /// Vector embedding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding: Option<Vec<f32>>,
    
    /// Metadata
    pub metadata: serde_json::Value,
    
    /// Timestamp
    pub timestamp: i64,
    
    /// Source (conversation, document, etc.)
    pub source: String,
}

/// Search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySearchResult {
    pub entry: MemoryEntry,
    pub score: f32,
}

/// LanceDB memory store
pub struct LanceMemory {
    db_path: String,
    table_name: String,
    embedding_dim: usize,
    // In-memory storage for simplicity
    entries: std::sync::Arc<tokio::sync::RwLock<Vec<MemoryEntry>>>,
}

impl LanceMemory {
    /// Create new memory store
    pub async fn new(path: impl AsRef<Path>) -> Result<Self> {
        let uri = path.as_ref().to_string_lossy().to_string();
        
        // Create directory if needed
        std::fs::create_dir_all(&path)?;
        
        Ok(Self {
            db_path: uri,
            table_name: "memories".into(),
            embedding_dim: 384,
            entries: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
        })
    }
    
    /// Create with custom embedding dimension
    pub fn with_embedding_dim(mut self, dim: usize) -> Self {
        self.embedding_dim = dim;
        self
    }
    
    /// Initialize table
    pub async fn init(&self) -> Result<()> {
        // Load existing entries from disk
        let data_path = format!("{}/{}.json", self.db_path, self.table_name);
        if std::path::Path::new(&data_path).exists() {
            let content = std::fs::read_to_string(&data_path)?;
            if let Ok(entries) = serde_json::from_str::<Vec<MemoryEntry>>(&content) {
                let mut data = self.entries.write().await;
                *data = entries;
            }
        }
        Ok(())
    }
    
    /// Save entries to disk
    async fn save(&self) -> Result<()> {
        let data_path = format!("{}/{}.json", self.db_path, self.table_name);
        let data = self.entries.read().await;
        let content = serde_json::to_string_pretty(&*data)?;
        std::fs::write(&data_path, content)?;
        Ok(())
    }
    
    /// Store memory entry
    pub async fn store(&self, entry: MemoryEntry) -> Result<()> {
        let mut data = self.entries.write().await;
        data.push(entry);
        drop(data);
        self.save().await
    }
    
    /// Add memory (alias for store)
    pub async fn add(&self, entry: MemoryEntry) -> Result<()> {
        self.store(entry).await
    }
    
    /// Search memories by vector similarity
    pub async fn search(&self, query: &str, query_embedding: Vec<f32>, limit: usize) -> Result<Vec<MemorySearchResult>> {
        let data = self.entries.read().await;
        let query_lower = query.to_lowercase();
        
        let mut results: Vec<MemorySearchResult> = data
            .iter()
            .filter(|e| {
                // Simple text matching
                e.content.to_lowercase().contains(&query_lower) ||
                e.source.to_lowercase().contains(&query_lower)
            })
            .map(|entry| {
                // Calculate simple similarity score
                let score = if let Some(emb) = &entry.embedding {
                    cosine_similarity(&query_embedding, emb)
                } else {
                    0.5
                };
                
                MemorySearchResult {
                    entry: entry.clone(),
                    score,
                }
            })
            .collect();
        
        // Sort by score
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit);
        
        Ok(results)
    }
    
    /// Get all memories (paginated)
    pub async fn list(&self, limit: usize, offset: usize) -> Result<Vec<MemoryEntry>> {
        let data = self.entries.read().await;
        Ok(data.iter().skip(offset).take(limit).cloned().collect())
    }
    
    /// Delete memory by ID
    pub async fn delete(&self, id: &str) -> Result<()> {
        let mut data = self.entries.write().await;
        data.retain(|e| e.id != id);
        drop(data);
        self.save().await
    }
    
    /// Get memory count
    pub async fn count(&self) -> Result<usize> {
        let data = self.entries.read().await;
        Ok(data.len())
    }
}

/// Calculate cosine similarity between two vectors
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let mag_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let mag_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    
    if mag_a == 0.0 || mag_b == 0.0 {
        return 0.0;
    }
    
    dot / (mag_a * mag_b)
}

/// Memory statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub total_entries: usize,
}
