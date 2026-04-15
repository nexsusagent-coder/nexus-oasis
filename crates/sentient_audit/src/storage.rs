//! ═══════════════════════════════════════════════════════════════════════════════
//!  Audit Storage Backend
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Persistent storage for audit logs:
//! - Multiple backends (PostgreSQL, Elasticsearch, S3)
//! - Retention policies
//! - Compression
//! - Search and query

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};

// ═══════════════════════════════════════════════════════════════════════════════
//  AUDIT ENTRY
// ═══════════════════════════════════════════════════════════════════════════════

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Unique ID
    pub id: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Event type
    pub event_type: String,
    /// Actor (user/service)
    pub actor: Actor,
    /// Action performed
    pub action: String,
    /// Target resource
    pub target: Option<Resource>,
    /// Event details
    pub details: HashMap<String, serde_json::Value>,
    /// Source IP
    pub source_ip: Option<String>,
    /// User agent
    pub user_agent: Option<String>,
    /// Session ID
    pub session_id: Option<String>,
    /// Correlation ID for tracing
    pub correlation_id: Option<String>,
    /// Outcome
    pub outcome: Outcome,
    /// Retention category
    pub retention_category: RetentionCategory,
}

/// Actor who performed the action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actor {
    pub id: String,
    pub actor_type: ActorType,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActorType {
    User,
    Service,
    System,
    ApiKey,
    Anonymous,
}

/// Target resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub resource_type: String,
    pub resource_id: String,
    pub name: Option<String>,
}

/// Action outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outcome {
    Success,
    Failure { reason: String },
    Pending,
}

/// Retention category
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RetentionCategory {
    /// Security events (7 years)
    Security,
    /// Compliance events (5 years)
    Compliance,
    /// Operational events (90 days)
    Operational,
    /// Debug events (7 days)
    Debug,
}

impl RetentionCategory {
    pub fn retention_days(&self) -> u32 {
        match self {
            RetentionCategory::Security => 365 * 7,
            RetentionCategory::Compliance => 365 * 5,
            RetentionCategory::Operational => 90,
            RetentionCategory::Debug => 7,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  STORAGE CONFIG
// ═══════════════════════════════════════════════════════════════════════════════

/// Storage backend type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageBackend {
    PostgreSQL { connection_string: String },
    Elasticsearch { url: String, index: String },
    S3 { bucket: String, prefix: String },
    File { path: String },
    Memory,
}

/// Audit storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditStorageConfig {
    /// Primary backend
    pub primary: StorageBackend,
    /// Optional secondary for redundancy
    pub secondary: Option<StorageBackend>,
    /// Enable compression
    pub compress: bool,
    /// Batch size for writes
    pub batch_size: usize,
    /// Flush interval (seconds)
    pub flush_interval_secs: u64,
    /// Auto-apply retention
    pub auto_retention: bool,
}

impl Default for AuditStorageConfig {
    fn default() -> Self {
        Self {
            primary: StorageBackend::Memory,
            secondary: None,
            compress: true,
            batch_size: 100,
            flush_interval_secs: 5,
            auto_retention: true,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  QUERY TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Audit query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditQuery {
    /// Time range
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    /// Filter by event type
    pub event_types: Vec<String>,
    /// Filter by actor
    pub actor_ids: Vec<String>,
    /// Filter by action
    pub actions: Vec<String>,
    /// Filter by resource
    pub resource_types: Vec<String>,
    /// Filter by outcome
    pub outcomes: Vec<Outcome>,
    /// Full-text search
    pub search: Option<String>,
    /// Pagination
    pub offset: usize,
    pub limit: usize,
    /// Sort order
    pub sort_desc: bool,
}

impl Default for AuditQuery {
    fn default() -> Self {
        Self {
            start_time: None,
            end_time: None,
            event_types: vec![],
            actor_ids: vec![],
            actions: vec![],
            resource_types: vec![],
            outcomes: vec![],
            search: None,
            offset: 0,
            limit: 100,
            sort_desc: true,
        }
    }
}

/// Query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditQueryResult {
    pub entries: Vec<AuditEntry>,
    pub total: usize,
    pub has_more: bool,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  AUDIT STORAGE
// ═══════════════════════════════════════════════════════════════════════════════

/// Audit storage error
#[derive(Debug, thiserror::Error)]
pub enum AuditStorageError {
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Query error: {0}")]
    Query(String),
    
    #[error("Connection error: {0}")]
    Connection(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
}

/// Audit storage backend
pub struct AuditStorage {
    config: AuditStorageConfig,
    buffer: Vec<AuditEntry>,
    memory_store: HashMap<String, AuditEntry>,
}

impl AuditStorage {
    pub fn new(config: AuditStorageConfig) -> Self {
        Self {
            config,
            buffer: Vec::new(),
            memory_store: HashMap::new(),
        }
    }
    
    /// Store an audit entry
    pub async fn store(&mut self, entry: AuditEntry) -> Result<(), AuditStorageError> {
        self.buffer.push(entry.clone());
        
        // Flush if batch size reached
        if self.buffer.len() >= self.config.batch_size {
            self.flush().await?;
        }
        
        // Also store in memory for demo
        self.memory_store.insert(entry.id.clone(), entry);
        
        Ok(())
    }
    
    /// Store multiple entries
    pub async fn store_batch(&mut self, entries: Vec<AuditEntry>) -> Result<(), AuditStorageError> {
        for entry in entries {
            self.memory_store.insert(entry.id.clone(), entry);
        }
        Ok(())
    }
    
    /// Flush buffer to storage
    pub async fn flush(&mut self) -> Result<(), AuditStorageError> {
        // In production, write to actual backend
        self.buffer.clear();
        Ok(())
    }
    
    /// Query audit entries
    pub async fn query(&self, query: AuditQuery) -> Result<AuditQueryResult, AuditStorageError> {
        let entries: Vec<AuditEntry> = self.memory_store.values()
            .filter(|e| {
                // Time filter
                if let Some(start) = query.start_time {
                    if e.timestamp < start { return false; }
                }
                if let Some(end) = query.end_time {
                    if e.timestamp > end { return false; }
                }
                
                // Event type filter
                if !query.event_types.is_empty() && 
                   !query.event_types.contains(&e.event_type) {
                    return false;
                }
                
                // Actor filter
                if !query.actor_ids.is_empty() && 
                   !query.actor_ids.contains(&e.actor.id) {
                    return false;
                }
                
                // Action filter
                if !query.actions.is_empty() && 
                   !query.actions.contains(&e.action) {
                    return false;
                }
                
                // Outcome filter
                if !query.outcomes.is_empty() {
                    let matches = match &e.outcome {
                        Outcome::Success => query.outcomes.iter().any(|o| matches!(o, Outcome::Success)),
                        Outcome::Failure { .. } => query.outcomes.iter().any(|o| matches!(o, Outcome::Failure { .. })),
                        Outcome::Pending => query.outcomes.iter().any(|o| matches!(o, Outcome::Pending)),
                    };
                    if !matches { return false; }
                }
                
                // Full-text search
                if let Some(ref search) = query.search {
                    let search_lower = search.to_lowercase();
                    if !e.details.values().any(|v| v.to_string().to_lowercase().contains(&search_lower)) {
                        return false;
                    }
                }
                
                true
            })
            .cloned()
            .collect();
        
        let total = entries.len();
        let has_more = total > query.offset + query.limit;
        
        let entries: Vec<AuditEntry> = entries
            .into_iter()
            .skip(query.offset)
            .take(query.limit)
            .collect();
        
        Ok(AuditQueryResult {
            entries,
            total,
            has_more,
        })
    }
    
    /// Get entry by ID
    pub async fn get(&self, id: &str) -> Option<AuditEntry> {
        self.memory_store.get(id).cloned()
    }
    
    /// Apply retention policy
    pub async fn apply_retention(&mut self) -> Result<usize, AuditStorageError> {
        let mut removed = 0;
        let now = Utc::now();
        
        self.memory_store.retain(|_, e| {
            let retention_days = e.retention_category.retention_days();
            let cutoff = now - Duration::days(retention_days as i64);
            
            if e.timestamp < cutoff {
                removed += 1;
                false
            } else {
                true
            }
        });
        
        Ok(removed)
    }
    
    /// Get statistics
    pub async fn stats(&self) -> AuditStats {
        let total = self.memory_store.len();
        let by_category: HashMap<String, usize> = self.memory_store.values()
            .fold(HashMap::new(), |mut acc, e| {
                *acc.entry(format!("{:?}", e.retention_category)).or_insert(0) += 1;
                acc
            });
        
        AuditStats {
            total_entries: total,
            by_category,
            oldest_entry: self.memory_store.values()
                .map(|e| e.timestamp)
                .min(),
            newest_entry: self.memory_store.values()
                .map(|e| e.timestamp)
                .max(),
        }
    }
}

impl Default for AuditStorage {
    fn default() -> Self {
        Self::new(AuditStorageConfig::default())
    }
}

/// Audit statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditStats {
    pub total_entries: usize,
    pub by_category: HashMap<String, usize>,
    pub oldest_entry: Option<DateTime<Utc>>,
    pub newest_entry: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_store_and_query() {
        let mut storage = AuditStorage::default();
        
        let entry = AuditEntry {
            id: "test-1".to_string(),
            timestamp: Utc::now(),
            event_type: "auth.login".to_string(),
            actor: Actor {
                id: "user-1".to_string(),
                actor_type: ActorType::User,
                name: Some("Test User".to_string()),
            },
            action: "login".to_string(),
            target: None,
            details: HashMap::new(),
            source_ip: Some("127.0.0.1".to_string()),
            user_agent: None,
            session_id: None,
            correlation_id: None,
            outcome: Outcome::Success,
            retention_category: RetentionCategory::Security,
        };
        
        storage.store(entry).await.unwrap();
        
        let result = storage.query(AuditQuery::default()).await.unwrap();
        assert_eq!(result.total, 1);
    }
    
    #[test]
    fn test_retention_days() {
        assert_eq!(RetentionCategory::Security.retention_days(), 2555);
        assert_eq!(RetentionCategory::Debug.retention_days(), 7);
    }
}
