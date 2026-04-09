//! ─── Conversation Memory ───

use crate::{LanceMemory, MemoryEntry, MemoryError, Result, EmbeddingEngine};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Conversation entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationEntry {
    /// Conversation ID
    pub conversation_id: String,
    
    /// Message ID
    pub message_id: String,
    
    /// Role (user, assistant, system)
    pub role: String,
    
    /// Message content
    pub content: String,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Metadata
    pub metadata: serde_json::Value,
}

/// Conversation memory manager
pub struct ConversationMemory {
    memory: LanceMemory,
    embeddings: EmbeddingEngine,
}

impl ConversationMemory {
    /// Create new conversation memory
    pub async fn new(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let memory = LanceMemory::new(path).await?;
        memory.init().await?;
        
        let embeddings = EmbeddingEngine::new(Default::default())?;
        
        Ok(Self { memory, embeddings })
    }
    
    /// Store conversation turn
    pub async fn store_turn(&self, entry: ConversationEntry) -> Result<()> {
        // Generate embedding
        let embedding = self.embeddings.embed(&entry.content).await?;
        
        // Create memory entry
        let memory_entry = MemoryEntry {
            id: entry.message_id.clone(),
            content: format!("{}: {}", entry.role, entry.content),
            embedding: Some(embedding),
            metadata: serde_json::json!({
                "conversation_id": entry.conversation_id,
                "role": entry.role,
                "timestamp": entry.timestamp.to_rfc3339(),
                "metadata": entry.metadata,
            }),
            timestamp: entry.timestamp.timestamp(),
            source: "conversation".into(),
        };
        
        self.memory.store(memory_entry).await
    }
    
    /// Get conversation history
    pub async fn get_history(&self, conversation_id: &str, limit: usize) -> Result<Vec<ConversationEntry>> {
        // Search for conversation entries
        let query_embedding = vec![0.0; self.embeddings.dimension()];  // Placeholder
        
        let results = self.memory.search(
            &format!("conversation_id:{}", conversation_id),
            query_embedding,
            limit,
        ).await?;
        
        Ok(results.into_iter()
            .filter_map(|r| {
                let meta = r.entry.metadata.as_object()?;
                let role = meta.get("role")?.as_str()?.to_string();
                let timestamp_str = meta.get("timestamp")?.as_str()?;
                let timestamp = DateTime::parse_from_rfc3339(timestamp_str).ok()?.with_timezone(&Utc);
                
                // Extract content after role prefix
                let content = r.entry.content.strip_prefix(&format!("{}: ", role))
                    .unwrap_or(&r.entry.content)
                    .to_string();
                
                Some(ConversationEntry {
                    conversation_id: meta.get("conversation_id")?.as_str()?.to_string(),
                    message_id: r.entry.id,
                    role,
                    content,
                    timestamp,
                    metadata: meta.get("metadata").cloned().unwrap_or(serde_json::Value::Null),
                })
            })
            .collect())
    }
    
    /// Search related conversations
    pub async fn search_related(&self, query: &str, k: usize) -> Result<Vec<ConversationEntry>> {
        let query_embedding = self.embeddings.embed(query).await?;
        
        let results = self.memory.search(query, query_embedding, k).await?;
        
        Ok(results.into_iter()
            .filter_map(|r| {
                let meta = r.entry.metadata.as_object()?;
                let role = meta.get("role")?.as_str()?.to_string();
                let timestamp_str = meta.get("timestamp")?.as_str()?;
                let timestamp = DateTime::parse_from_rfc3339(timestamp_str).ok()?.with_timezone(&Utc);
                
                let content = r.entry.content.strip_prefix(&format!("{}: ", role))
                    .unwrap_or(&r.entry.content)
                    .to_string();
                
                Some(ConversationEntry {
                    conversation_id: meta.get("conversation_id")?.as_str()?.to_string(),
                    message_id: r.entry.id,
                    role,
                    content,
                    timestamp,
                    metadata: meta.get("metadata").cloned().unwrap_or(serde_json::Value::Null),
                })
            })
            .collect())
    }
    
    /// Summarize conversation
    pub async fn summarize(&self, conversation_id: &str) -> Result<String> {
        let history = self.get_history(conversation_id, 100).await?;
        
        // Create summary
        let mut summary = format!("Conversation {} ({} messages):\n", 
            conversation_id, history.len());
        
        for entry in history.iter().take(10) {
            summary.push_str(&format!("{}: {}\n", entry.role, 
                entry.content.chars().take(100).collect::<String>()));
        }
        
        if history.len() > 10 {
            summary.push_str(&format!("... and {} more messages", history.len() - 10));
        }
        
        Ok(summary)
    }
}
