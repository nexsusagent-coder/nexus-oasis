//! Memory Tool - Mem0 pattern'inden adapte
//! SENTIENT oasis-memory crate için
//! Mevcut types::MemoryEntry yapısını kullanır

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use log::{info, debug};

use crate::types::{MemoryEntry, MemoryInput, MemoryType, MemorySource, Importance};
use crate::tools::{Tool, ToolError, ToolResult, ToolContext};

/// Memory işlemleri için input şeması
#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryToolInput {
    /// İşlem: store, recall, forget, list, search
    pub action: String,
    /// Anahtar (store, recall, forget için)
    pub key: Option<String>,
    /// Değer (store için)
    pub value: Option<serde_json::Value>,
    /// Namespace (farklı ajanlar/bağlamlar için)
    pub namespace: Option<String>,
    /// Arama sorgusu (search için)
    pub query: Option<String>,
    /// Limit (list, search için)
    pub limit: Option<usize>,
}

/// Memory işlem sonucu
#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryToolOutput {
    pub success: bool,
    pub action: String,
    pub data: Option<serde_json::Value>,
    pub entries: Option<Vec<MemoryEntrySummary>>,
    pub message: String,
}

/// Memory entry özeti (serialization için)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntrySummary {
    pub id: String,
    pub content: String,
    pub memory_type: String,
    pub importance: f32,
    pub created_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

impl From<&MemoryEntry> for MemoryEntrySummary {
    fn from(entry: &MemoryEntry) -> Self {
        Self {
            id: entry.id.to_string(),
            content: entry.content.clone(),
            memory_type: entry.memory_type.to_string(),
            importance: entry.importance.value(),
            created_at: entry.created_at,
            tags: entry.tags.clone(),
        }
    }
}

/// Memory Tool - Kalıcı cross-session bellek
pub struct MemoryTool {
    storage: HashMap<String, MemoryEntry>,
    default_namespace: String,
}

impl MemoryTool {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
            default_namespace: "default".to_string(),
        }
    }
    
    fn get_namespace(&self, input: &MemoryToolInput) -> String {
        input.namespace.clone().unwrap_or_else(|| self.default_namespace.clone())
    }
    
    fn make_key(&self, namespace: &str, key: &str) -> String {
        format!("{}:{}", namespace, key)
    }
}

#[async_trait]
impl Tool for MemoryTool {
    type Input = MemoryToolInput;
    type Output = MemoryToolOutput;
    
    fn name(&self) -> &'static str {
        "memory_tool"
    }
    
    fn description(&self) -> &'static str {
        "Kalıcı cross-session bellek yönetimi: store, recall, forget, list, search. Mem0 pattern."
    }
    
    fn is_read_only(&self, input: &Self::Input) -> bool {
        matches!(input.action.as_str(), "recall" | "list" | "search")
    }
    
    async fn execute(
        &mut self,
        input: Self::Input,
        _context: &ToolContext,
    ) -> ToolResult<Self::Output> {
        match input.action.as_str() {
            "store" => self.store(input).await,
            "recall" => self.recall(input).await,
            "forget" => self.forget(input).await,
            "list" => self.list(input).await,
            "search" => self.search(input).await,
            _ => Err(ToolError::InvalidInput(format!(
                "Bilinmeyen memory aksiyonu: {}", input.action
            ))),
        }
    }
}

impl MemoryTool {
    async fn store(&mut self, input: MemoryToolInput) -> ToolResult<MemoryToolOutput> {
        let namespace = self.get_namespace(&input);
        let key = input.key.ok_or_else(|| {
            ToolError::InvalidInput("store için 'key' gerekli".to_string())
        })?;
        let value = input.value.ok_or_else(|| {
            ToolError::InvalidInput("store için 'value' gerekli".to_string())
        })?;
        let full_key = self.make_key(&namespace, &key);
        
        // MemoryInput oluştur ve MemoryEntry'ye çevir
        let memory_input = MemoryInput::new(value.to_string())
            .with_type(MemoryType::Semantic)
            .with_source(MemorySource::UserInput);
        
        let entry = MemoryEntry::from_input(memory_input);
        
        self.storage.insert(full_key, entry);
        
        info!("💾 Memory stored: {}:{}", namespace, key);
        
        Ok(MemoryToolOutput {
            success: true,
            action: "store".to_string(),
            data: Some(value),
            entries: None,
            message: format!("Başarıyla kaydedildi: {}:{}", namespace, key),
        })
    }
    
    async fn recall(&self, input: MemoryToolInput) -> ToolResult<MemoryToolOutput> {
        let namespace = self.get_namespace(&input);
        let key = input.key.ok_or_else(|| {
            ToolError::InvalidInput("recall için 'key' gerekli".to_string())
        })?;
        let full_key = self.make_key(&namespace, &key);
        
        match self.storage.get(&full_key) {
            Some(entry) => {
                debug!("📖 Memory recalled: {}:{}", namespace, key);
                Ok(MemoryToolOutput {
                    success: true,
                    action: "recall".to_string(),
                    data: Some(serde_json::json!(entry.content)),
                    entries: Some(vec![MemoryEntrySummary::from(entry)]),
                    message: format!("Bulundu: {}:{}", namespace, key),
                })
            }
            None => {
                Ok(MemoryToolOutput {
                    success: false,
                    action: "recall".to_string(),
                    data: None,
                    entries: None,
                    message: format!("Bulunamadı: {}:{}", namespace, key),
                })
            }
        }
    }
    
    async fn forget(&mut self, input: MemoryToolInput) -> ToolResult<MemoryToolOutput> {
        let namespace = self.get_namespace(&input);
        let key = input.key.ok_or_else(|| {
            ToolError::InvalidInput("forget için 'key' gerekli".to_string())
        })?;
        let full_key = self.make_key(&namespace, &key);
        
        match self.storage.remove(&full_key) {
            Some(entry) => {
                info!("🗑️ Memory forgotten: {}:{}", namespace, key);
                Ok(MemoryToolOutput {
                    success: true,
                    action: "forget".to_string(),
                    data: Some(serde_json::json!(entry.content)),
                    entries: None,
                    message: format!("Silindi: {}:{}", namespace, key),
                })
            }
            None => {
                Ok(MemoryToolOutput {
                    success: false,
                    action: "forget".to_string(),
                    data: None,
                    entries: None,
                    message: format!("Zaten yok: {}:{}", namespace, key),
                })
            }
        }
    }
    
    async fn list(&self, input: MemoryToolInput) -> ToolResult<MemoryToolOutput> {
        let namespace = self.get_namespace(&input);
        let limit = input.limit.unwrap_or(100);
        
        let entries: Vec<MemoryEntrySummary> = self.storage
            .values()
            .filter(|e| e.content.contains(&namespace))
            .take(limit)
            .map(MemoryEntrySummary::from)
            .collect();
        
        let count = entries.len();
        info!("📋 Memory list: {} entries", count);
        
        Ok(MemoryToolOutput {
            success: true,
            action: "list".to_string(),
            data: None,
            entries: Some(entries),
            message: format!("{} kayıt bulundu", count),
        })
    }
    
    async fn search(&self, input: MemoryToolInput) -> ToolResult<MemoryToolOutput> {
        let query = input.query.ok_or_else(|| {
            ToolError::InvalidInput("search için 'query' gerekli".to_string())
        })?;
        let limit = input.limit.unwrap_or(50);
        
        let query_lower = query.to_lowercase();
        
        let entries: Vec<MemoryEntrySummary> = self.storage
            .values()
            .filter(|e| {
                e.content.to_lowercase().contains(&query_lower) ||
                e.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
            })
            .take(limit)
            .map(MemoryEntrySummary::from)
            .collect();
        
        let count = entries.len();
        info!("🔍 Memory search: {} results for '{}'", count, query);
        
        Ok(MemoryToolOutput {
            success: true,
            action: "search".to_string(),
            data: None,
            entries: Some(entries),
            message: format!("{} sonuç bulundu: '{}'", count, query),
        })
    }
}

impl Default for MemoryTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_store_and_recall() {
        let mut tool = MemoryTool::new();
        let context = ToolContext::default();
        
        // Store
        let store_input = MemoryToolInput {
            action: "store".to_string(),
            key: Some("test_key".to_string()),
            value: Some(serde_json::json!({"name": "SENTIENT"})),
            namespace: None,
            query: None,
            limit: None,
        };
        let result = tool.execute(store_input, &context).await.unwrap();
        assert!(result.success);
        
        // Recall
        let recall_input = MemoryToolInput {
            action: "recall".to_string(),
            key: Some("test_key".to_string()),
            value: None,
            namespace: None,
            query: None,
            limit: None,
        };
        let result = tool.execute(recall_input, &context).await.unwrap();
        assert!(result.success);
    }
}
