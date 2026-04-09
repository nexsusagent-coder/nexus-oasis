//! ─── LanceDB Memory Storage ───

use crate::{MemoryError, Result};
use lancedb::prelude::*;
use arrow::array::{StringArray, Float32Array, RecordBatch};
use arrow::datatypes::{Schema, Field, DataType};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;

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
    db: Connection,
    table_name: String,
    embedding_dim: usize,
}

impl LanceMemory {
    /// Create new memory store
    pub async fn new(path: impl AsRef<Path>) -> Result<Self> {
        let db = connect(path.as_ref())
            .execute()
            .await
            .map_err(|e| MemoryError::Database(e.to_string()))?;
        
        Ok(Self {
            db,
            table_name: "memories".into(),
            embedding_dim: 384,  // Default: all-MiniLM-L6-v2
        })
    }
    
    /// Create with custom embedding dimension
    pub fn with_embedding_dim(mut self, dim: usize) -> Self {
        self.embedding_dim = dim;
        self
    }
    
    /// Initialize table
    pub async fn init(&self) -> Result<()> {
        let schema = self.schema();
        
        // Create empty table if not exists
        let empty_batch = RecordBatch::new_empty(schema);
        
        self.db
            .create_table(&self.table_name, empty_batch)
            .execute()
            .await
            .map_err(|e| MemoryError::Database(e.to_string()))?;
        
        Ok(())
    }
    
    /// Get arrow schema
    fn schema(&self) -> Arc<Schema> {
        Arc::new(Schema::new(vec![
            Field::new("id", DataType::Utf8, false),
            Field::new("content", DataType::Utf8, false),
            Field::new("embedding", DataType::Float32, false),
            Field::new("metadata", DataType::Utf8, false),
            Field::new("timestamp", DataType::Int64, false),
            Field::new("source", DataType::Utf8, false),
        ]))
    }
    
    /// Store memory entry
    pub async fn store(&self, entry: MemoryEntry) -> Result<()> {
        let embedding = entry.embedding.unwrap_or_else(|| vec![0.0; self.embedding_dim]);
        
        let batch = RecordBatch::try_new(
            self.schema(),
            vec![
                Arc::new(StringArray::from(vec![entry.id.as_str()])),
                Arc::new(StringArray::from(vec![entry.content.as_str()])),
                Arc::new(Float32Array::from(embedding.clone())),
                Arc::new(StringArray::from(vec![entry.metadata.to_string().as_str()])),
                Arc::new(arrow::array::Int64Array::from(vec![entry.timestamp])),
                Arc::new(StringArray::from(vec![entry.source.as_str()])),
            ],
        ).map_err(|e| MemoryError::Database(e.to_string()))?;
        
        self.db
            .open_table(&self.table_name)
            .execute()
            .await
            .map_err(|e| MemoryError::Database(e.to_string()))?
            .add(box_stream::iter(vec![batch]))
            .execute()
            .await
            .map_err(|e| MemoryError::Database(e.to_string()))?;
        
        Ok(())
    }
    
    /// Store batch of entries
    pub async fn store_batch(&self, entries: Vec<MemoryEntry>) -> Result<()> {
        let mut batches = Vec::new();
        
        for chunk in entries.chunks(100) {
            let ids: Vec<&str> = chunk.iter().map(|e| e.id.as_str()).collect();
            let contents: Vec<&str> = chunk.iter().map(|e| e.content.as_str()).collect();
            let embeddings: Vec<f32> = chunk.iter()
                .flat_map(|e| e.embedding.clone().unwrap_or_else(|| vec![0.0; self.embedding_dim]))
                .collect();
            let metadatas: Vec<&str> = chunk.iter().map(|e| e.metadata.to_string().as_str()).collect();
            let timestamps: Vec<i64> = chunk.iter().map(|e| e.timestamp).collect();
            let sources: Vec<&str> = chunk.iter().map(|e| e.source.as_str()).collect();
            
            let batch = RecordBatch::try_new(
                self.schema(),
                vec![
                    Arc::new(StringArray::from(ids)),
                    Arc::new(StringArray::from(contents)),
                    Arc::new(Float32Array::from(embeddings)),
                    Arc::new(StringArray::from(metadatas)),
                    Arc::new(arrow::array::Int64Array::from(timestamps)),
                    Arc::new(StringArray::from(sources)),
                ],
            ).map_err(|e| MemoryError::Database(e.to_string()))?;
            
            batches.push(batch);
        }
        
        self.db
            .open_table(&self.table_name)
            .execute()
            .await
            .map_err(|e| MemoryError::Database(e.to_string()))?
            .add(box_stream::iter(batches))
            .execute()
            .await
            .map_err(|e| MemoryError::Database(e.to_string()))?;
        
        Ok(())
    }
    
    /// Semantic search
    pub async fn search(&self, query: &str, query_embedding: Vec<f32>, k: usize) -> Result<Vec<MemorySearchResult>> {
        let table = self.db
            .open_table(&self.table_name)
            .execute()
            .await
            .map_err(|e| MemoryError::Database(e.to_string()))?;
        
        // Vector search
        let results = table
            .query()
            .nearest_to(&query_embedding)
            .map_err(|e| MemoryError::Database(e.to_string()))?
            .limit(k as u64)
            .execute()
            .await
            .map_err(|e| MemoryError::Database(e.to_string()))?;
        
        let mut search_results = Vec::new();
        
        for batch in results {
            let batch = batch.map_err(|e| MemoryError::Database(e.to_string()))?;
            
            let ids = batch.column_by_name("id")
                .and_then(|c| c.as_any().downcast_ref::<StringArray>())
                .ok_or_else(|| MemoryError::Database("Invalid id column".into()))?;
            
            let contents = batch.column_by_name("content")
                .and_then(|c| c.as_any().downcast_ref::<StringArray>())
                .ok_or_else(|| MemoryError::Database("Invalid content column".into()))?;
            
            let metadatas = batch.column_by_name("metadata")
                .and_then(|c| c.as_any().downcast_ref::<StringArray>())
                .ok_or_else(|| MemoryError::Database("Invalid metadata column".into()))?;
            
            let timestamps = batch.column_by_name("timestamp")
                .and_then(|c| c.as_any().downcast_ref::<arrow::array::Int64Array>())
                .ok_or_else(|| MemoryError::Database("Invalid timestamp column".into()))?;
            
            let sources = batch.column_by_name("source")
                .and_then(|c| c.as_any().downcast_ref::<StringArray>())
                .ok_or_else(|| MemoryError::Database("Invalid source column".into()))?;
            
            let distances = batch.column_by_name("_distance")
                .and_then(|c| c.as_any().downcast_ref::<Float32Array>())
                .ok_or_else(|| MemoryError::Database("Invalid distance column".into()))?;
            
            for i in 0..batch.num_rows() {
                let entry = MemoryEntry {
                    id: ids.value(i).to_string(),
                    content: contents.value(i).to_string(),
                    embedding: None,
                    metadata: serde_json::from_str(metadatas.value(i))
                        .unwrap_or(serde_json::Value::Null),
                    timestamp: timestamps.value(i),
                    source: sources.value(i).to_string(),
                };
                
                let score = 1.0 - distances.value(i);
                
                search_results.push(MemorySearchResult { entry, score });
            }
        }
        
        Ok(search_results)
    }
    
    /// Get by ID
    pub async fn get(&self, id: &str) -> Result<Option<MemoryEntry>> {
        let table = self.db
            .open_table(&self.table_name)
            .execute()
            .await
            .map_err(|e| MemoryError::Database(e.to_string()))?;
        
        let results = table
            .query()
            .only_query(format!("id = '{}'", id))
            .execute()
            .await
            .map_err(|e| MemoryError::Database(e.to_string()))?;
        
        // Extract first result
        // ... implementation
        
        Ok(None)
    }
    
    /// Delete by ID
    pub async fn delete(&self, id: &str) -> Result<()> {
        let table = self.db
            .open_table(&self.table_name)
            .execute()
            .await
            .map_err(|e| MemoryError::Database(e.to_string()))?;
        
        table
            .delete(&format!("id = '{}'", id))
            .execute()
            .await
            .map_err(|e| MemoryError::Database(e.to_string()))?;
        
        Ok(())
    }
    
    /// Clear all memories
    pub async fn clear(&self) -> Result<()> {
        self.db
            .drop_table(&self.table_name)
            .execute()
            .await
            .map_err(|e| MemoryError::Database(e.to_string()))?;
        
        self.init().await
    }
    
    /// Get statistics
    pub async fn stats(&self) -> Result<MemoryStats> {
        let table = self.db
            .open_table(&self.table_name)
            .execute()
            .await
            .map_err(|e| MemoryError::Database(e.to_string()))?;
        
        Ok(MemoryStats {
            total_entries: table.count_rows(None).await
                .map_err(|e| MemoryError::Database(e.to_string()))? as u64,
        })
    }
}

/// Memory statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub total_entries: u64,
}

/// Box stream helper
mod box_stream {
    use futures::stream::{Stream, StreamExt};
    
    pub fn iter<T>(items: Vec<T>) -> std::pin::Pin<Box<dyn Stream<Item = T> + Send>> {
        Box::pin(futures::stream::iter(items))
    }
}
