//! ─── Knowledge Base ───

use crate::{LanceMemory, MemoryEntry, Result, EmbeddingEngine};
use serde::{Deserialize, Serialize};

/// Knowledge entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEntry {
    /// Entry ID
    pub id: String,
    
    /// Title
    pub title: String,
    
    /// Content
    pub content: String,
    
    /// Category
    pub category: String,
    
    /// Tags
    pub tags: Vec<String>,
    
    /// Source URL (optional)
    pub source_url: Option<String>,
    
    /// Metadata
    pub metadata: serde_json::Value,
}

/// Knowledge base manager
pub struct KnowledgeBase {
    memory: LanceMemory,
    embeddings: EmbeddingEngine,
}

impl KnowledgeBase {
    /// Create new knowledge base
    pub async fn new(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let memory = LanceMemory::new(path).await?;
        memory.init().await?;
        
        let embeddings = EmbeddingEngine::new(Default::default())?;
        
        Ok(Self { memory, embeddings })
    }
    
    /// Add knowledge entry
    pub async fn add(&self, entry: KnowledgeEntry) -> Result<()> {
        let text = format!("{}: {}", entry.title, entry.content);
        let embedding = self.embeddings.embed(&text).ok();
        
        let memory_entry = MemoryEntry {
            id: entry.id,
            content: entry.content,
            embedding,
            metadata: serde_json::json!({
                "title": entry.title,
                "category": entry.category,
                "tags": entry.tags,
                "source_url": entry.source_url,
                "metadata": entry.metadata,
            }),
            timestamp: chrono::Utc::now().timestamp(),
            source: "knowledge".into(),
        };
        
        self.memory.store(memory_entry).await
    }
    
    /// Search knowledge
    pub async fn search(&self, query: &str, k: usize) -> Result<Vec<KnowledgeResult>> {
        let query_embedding = self.embeddings.embed(query).unwrap_or_default();
        
        let results = self.memory.search(query, query_embedding, k).await?;
        
        Ok(results.into_iter()
            .filter_map(|r| {
                let meta = r.entry.metadata.as_object()?;
                
                Some(KnowledgeResult {
                    entry: KnowledgeEntry {
                        id: r.entry.id,
                        title: meta.get("title")?.as_str()?.to_string(),
                        content: r.entry.content,
                        category: meta.get("category")?.as_str()?.to_string(),
                        tags: meta.get("tags")
                            .and_then(|t| t.as_array())
                            .map(|arr| arr.iter().filter_map(|t| t.as_str().map(String::from)).collect())
                            .unwrap_or_default(),
                        source_url: meta.get("source_url")
                            .and_then(|u| u.as_str().map(String::from)),
                        metadata: meta.get("metadata").cloned().unwrap_or(serde_json::Value::Null),
                    },
                    score: r.score,
                })
            })
            .collect())
    }
    
    /// Get by category
    pub async fn get_by_category(&self, _category: &str) -> Result<Vec<KnowledgeEntry>> {
        Ok(vec![])
    }
    
    /// Get by tag
    pub async fn get_by_tag(&self, _tag: &str) -> Result<Vec<KnowledgeEntry>> {
        Ok(vec![])
    }
    
    /// Update entry
    pub async fn update(&self, entry: KnowledgeEntry) -> Result<()> {
        self.memory.delete(&entry.id).await?;
        self.add(entry).await
    }
    
    /// Delete entry
    pub async fn delete(&self, id: &str) -> Result<()> {
        self.memory.delete(id).await
    }
    
    /// Import from JSON
    pub async fn import_json(&self, json: &str) -> Result<usize> {
        let entries: Vec<KnowledgeEntry> = serde_json::from_str(json)?;
        let count = entries.len();
        
        for entry in entries {
            self.add(entry).await?;
        }
        
        Ok(count)
    }
    
    /// Export to JSON
    pub async fn export_json(&self) -> Result<String> {
        Ok("[]".into())
    }
}

/// Knowledge search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeResult {
    pub entry: KnowledgeEntry,
    pub score: f32,
}
