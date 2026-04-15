//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT Orchestrator Persistent State
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  State persistence for orchestrator:
//!  - Checkpoint-based state snapshots
//!  - Recovery from crashes
//!  - State migration between versions
//!  - Multiple storage backends (File, SQLite, Redis)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

// ═══════════════════════════════════════════════════════════════════════════════
//  STATE SNAPSHOT
// ═══════════════════════════════════════════════════════════════════════════════

/// State snapshot for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    /// Snapshot ID
    pub id: String,
    /// Agent ID
    pub agent_id: String,
    /// Snapshot type
    pub snapshot_type: SnapshotType,
    /// Agent state
    pub state: String,
    /// Agent context
    pub context: PersistentContext,
    /// Active goals
    pub goals: Vec<PersistentGoal>,
    /// Task queue state
    pub task_queue: Vec<PersistentTask>,
    /// Execution history (last N steps)
    pub execution_history: Vec<ExecutionStep>,
    /// Variables
    pub variables: HashMap<String, serde_json::Value>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Version
    pub version: String,
    /// Checksum (for integrity)
    pub checksum: String,
}

/// Snapshot type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SnapshotType {
    /// Automatic periodic checkpoint
    Checkpoint,
    /// Before risky operation
    PreAction,
    /// After successful operation
    PostAction,
    /// Manual save
    Manual,
    /// Before shutdown
    Shutdown,
    /// Recovery point
    Recovery,
}

/// Persistent agent context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistentContext {
    /// Current goal ID
    pub current_goal_id: Option<String>,
    /// Current task ID
    pub current_task_id: Option<String>,
    /// Step count
    pub step_count: u64,
    /// Total tokens used
    pub total_tokens: u64,
    /// Total cost
    pub total_cost: f64,
    /// Last action
    pub last_action: Option<String>,
    /// Last error
    pub last_error: Option<String>,
    /// Conversation history (last N messages)
    pub conversation: Vec<ConversationMessage>,
    /// Tool results cache
    pub tool_results: HashMap<String, serde_json::Value>,
    /// Custom data
    pub custom: HashMap<String, serde_json::Value>,
}

impl Default for PersistentContext {
    fn default() -> Self {
        Self {
            current_goal_id: None,
            current_task_id: None,
            step_count: 0,
            total_tokens: 0,
            total_cost: 0.0,
            last_action: None,
            last_error: None,
            conversation: Vec::new(),
            tool_results: HashMap::new(),
            custom: HashMap::new(),
        }
    }
}

/// Persistent goal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistentGoal {
    pub id: String,
    pub description: String,
    pub status: String,
    pub priority: u8,
    pub progress: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub parent_id: Option<String>,
    pub sub_goals: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Persistent task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistentTask {
    pub id: String,
    pub goal_id: String,
    pub name: String,
    pub status: String,
    pub priority: u8,
    pub retries: u32,
    pub max_retries: u32,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
}

/// Execution step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStep {
    pub step_number: u64,
    pub action: String,
    pub input: Option<serde_json::Value>,
    pub output: Option<serde_json::Value>,
    pub success: bool,
    pub duration_ms: u64,
    pub timestamp: DateTime<Utc>,
}

/// Conversation message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    pub role: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub tokens: Option<u32>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PERSISTENCE CONFIG
// ═══════════════════════════════════════════════════════════════════════════════

/// Persistence configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceConfig {
    /// Storage backend
    pub backend: StorageBackend,
    /// Checkpoint interval in seconds (0 = disabled)
    pub checkpoint_interval_secs: u64,
    /// Maximum snapshots to keep
    pub max_snapshots: usize,
    /// Maximum conversation history
    pub max_conversation_history: usize,
    /// Maximum execution history
    pub max_execution_history: usize,
    /// Enable compression
    pub compress: bool,
    /// State directory (for file backend)
    pub state_dir: PathBuf,
    /// Auto-recovery on startup
    pub auto_recovery: bool,
}

impl Default for PersistenceConfig {
    fn default() -> Self {
        Self {
            backend: StorageBackend::File,
            checkpoint_interval_secs: 60,
            max_snapshots: 100,
            max_conversation_history: 100,
            max_execution_history: 1000,
            compress: true,
            state_dir: PathBuf::from("./state/orchestrator"),
            auto_recovery: true,
        }
    }
}

/// Storage backend
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageBackend {
    File,
    SQLite,
    Redis,
    Memory,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  STATE PERSISTENCE MANAGER
// ═══════════════════════════════════════════════════════════════════════════════

/// State persistence manager
pub struct StatePersistence {
    config: PersistenceConfig,
    current_state: Arc<RwLock<Option<StateSnapshot>>>,
    snapshots: Arc<RwLock<Vec<StateSnapshot>>>,
    dirty: Arc<RwLock<bool>>,
}

impl StatePersistence {
    pub fn new(config: PersistenceConfig) -> Self {
        Self {
            config,
            current_state: Arc::new(RwLock::new(None)),
            snapshots: Arc::new(RwLock::new(Vec::new())),
            dirty: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Initialize persistence (create dirs, load state)
    pub async fn initialize(&self) -> Result<(), PersistenceError> {
        // Create state directory
        tokio::fs::create_dir_all(&self.config.state_dir).await
            .map_err(|e| PersistenceError::IoError(e.to_string()))?;
        
        // Load latest snapshot if auto-recovery enabled
        if self.config.auto_recovery {
            if let Some(snapshot) = self.load_latest_snapshot().await? {
                log::info!("🔄 Recovered state from snapshot: {}", snapshot.id);
                let mut state = self.current_state.write().await;
                *state = Some(snapshot);
            }
        }
        
        Ok(())
    }
    
    /// Save state snapshot
    pub async fn save(&self, snapshot: StateSnapshot) -> Result<(), PersistenceError> {
        // Calculate checksum
        let snapshot_json = serde_json::to_string(&snapshot)
            .map_err(|e| PersistenceError::SerializationError(e.to_string()))?;
        let checksum = format!("{:x}", md5::compute(&snapshot_json));
        
        let mut snapshot = snapshot;
        snapshot.checksum = checksum.clone();
        
        // Save to storage
        self.save_to_storage(&snapshot).await?;
        
        // Update current state
        let mut current = self.current_state.write().await;
        *current = Some(snapshot.clone());
        
        // Add to snapshots list
        let mut snapshots = self.snapshots.write().await;
        snapshots.push(snapshot);
        
        // Trim old snapshots
        while snapshots.len() > self.config.max_snapshots {
            let old = snapshots.remove(0);
            self.delete_snapshot(&old.id).await?;
        }
        
        // Mark as clean
        let mut dirty = self.dirty.write().await;
        *dirty = false;
        
        log::debug!("💾 State saved: {} snapshots in storage", snapshots.len());
        Ok(())
    }
    
    /// Load state snapshot
    pub async fn load(&self, snapshot_id: &str) -> Result<Option<StateSnapshot>, PersistenceError> {
        match self.config.backend {
            StorageBackend::File => self.load_from_file(snapshot_id).await,
            StorageBackend::Memory => {
                let snapshots = self.snapshots.read().await;
                Ok(snapshots.iter().find(|s| s.id == snapshot_id).cloned())
            }
            _ => Err(PersistenceError::BackendNotImplemented),
        }
    }
    
    /// Load latest snapshot
    pub async fn load_latest_snapshot(&self) -> Result<Option<StateSnapshot>, PersistenceError> {
        match self.config.backend {
            StorageBackend::File => {
                let files = self.list_snapshot_files().await?;
                if files.is_empty() {
                    return Ok(None);
                }
                
                // Get most recent
                let latest = files.into_iter().max_by_key(|(_, ts)| *ts);
                if let Some((file, _)) = latest {
                    self.load_from_file(&file).await
                } else {
                    Ok(None)
                }
            }
            StorageBackend::Memory => {
                let snapshots = self.snapshots.read().await;
                Ok(snapshots.last().cloned())
            }
            _ => Err(PersistenceError::BackendNotImplemented),
        }
    }
    
    /// Get current state
    pub async fn get_current(&self) -> Option<StateSnapshot> {
        self.current_state.read().await.clone()
    }
    
    /// Mark state as dirty
    pub async fn mark_dirty(&self) {
        let mut dirty = self.dirty.write().await;
        *dirty = true;
    }
    
    /// Check if dirty
    pub async fn is_dirty(&self) -> bool {
        *self.dirty.read().await
    }
    
    /// Create checkpoint from current state
    pub async fn checkpoint(&self, agent_id: &str, state: &str, context: PersistentContext) -> Result<String, PersistenceError> {
        let snapshot = StateSnapshot {
            id: uuid::Uuid::new_v4().to_string(),
            agent_id: agent_id.to_string(),
            snapshot_type: SnapshotType::Checkpoint,
            state: state.to_string(),
            context,
            goals: Vec::new(),
            task_queue: Vec::new(),
            execution_history: Vec::new(),
            variables: HashMap::new(),
            timestamp: Utc::now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            checksum: String::new(),
        };
        
        let id = snapshot.id.clone();
        self.save(snapshot).await?;
        Ok(id)
    }
    
    /// Clear all state
    pub async fn clear(&self) -> Result<(), PersistenceError> {
        let mut current = self.current_state.write().await;
        *current = None;
        
        let mut snapshots = self.snapshots.write().await;
        
        // Delete all snapshot files
        for snapshot in snapshots.drain(..) {
            self.delete_snapshot(&snapshot.id).await?;
        }
        
        Ok(())
    }
    
    // ═════════════════════════════════════════════════════════════════════════
    //  FILE BACKEND IMPLEMENTATION
    // ═════════════════════════════════════════════════════════════════════════
    
    async fn save_to_storage(&self, snapshot: &StateSnapshot) -> Result<(), PersistenceError> {
        match self.config.backend {
            StorageBackend::File => self.save_to_file(snapshot).await,
            StorageBackend::Memory => Ok(()),
            _ => Err(PersistenceError::BackendNotImplemented),
        }
    }
    
    async fn save_to_file(&self, snapshot: &StateSnapshot) -> Result<(), PersistenceError> {
        let file_path = self.config.state_dir.join(format!("{}.json", snapshot.id));
        
        let json = serde_json::to_string_pretty(snapshot)
            .map_err(|e| PersistenceError::SerializationError(e.to_string()))?;
        
        tokio::fs::write(&file_path, json).await
            .map_err(|e| PersistenceError::IoError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn load_from_file(&self, id: &str) -> Result<Option<StateSnapshot>, PersistenceError> {
        let file_path = self.config.state_dir.join(format!("{}.json", id));
        
        if !file_path.exists() {
            return Ok(None);
        }
        
        let json = tokio::fs::read_to_string(&file_path).await
            .map_err(|e| PersistenceError::IoError(e.to_string()))?;
        
        let snapshot: StateSnapshot = serde_json::from_str(&json)
            .map_err(|e| PersistenceError::DeserializationError(e.to_string()))?;
        
        // Verify checksum
        let expected = snapshot.checksum.clone();
        let mut snapshot = snapshot;
        snapshot.checksum = String::new();
        let json = serde_json::to_string(&snapshot)
            .map_err(|e| PersistenceError::SerializationError(e.to_string()))?;
        let actual = format!("{:x}", md5::compute(&json));
        
        if expected != actual {
            return Err(PersistenceError::ChecksumMismatch);
        }
        
        snapshot.checksum = expected;
        Ok(Some(snapshot))
    }
    
    async fn delete_snapshot(&self, id: &str) -> Result<(), PersistenceError> {
        match self.config.backend {
            StorageBackend::File => {
                let file_path = self.config.state_dir.join(format!("{}.json", id));
                if file_path.exists() {
                    tokio::fs::remove_file(&file_path).await
                        .map_err(|e| PersistenceError::IoError(e.to_string()))?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
    
    async fn list_snapshot_files(&self) -> Result<Vec<(String, DateTime<Utc>)>, PersistenceError> {
        let mut dir = tokio::fs::read_dir(&self.config.state_dir).await
            .map_err(|e| PersistenceError::IoError(e.to_string()))?;
        
        let mut files = Vec::new();
        
        while let Some(entry) = dir.next_entry().await.map_err(|e| PersistenceError::IoError(e.to_string()))? {
            let path = entry.path();
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                let metadata = entry.metadata().await.map_err(|e| PersistenceError::IoError(e.to_string()))?;
                let modified: DateTime<Utc> = metadata.modified()
                    .map_err(|e| PersistenceError::IoError(e.to_string()))?
                    .into();
                
                let id = path.file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                
                files.push((id, modified));
            }
        }
        
        Ok(files)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  STATE BUILDER
// ═══════════════════════════════════════════════════════════════════════════════

/// Builder for creating state snapshots
pub struct StateBuilder {
    agent_id: String,
    state: String,
    context: PersistentContext,
    goals: Vec<PersistentGoal>,
    tasks: Vec<PersistentTask>,
    history: Vec<ExecutionStep>,
    variables: HashMap<String, serde_json::Value>,
}

impl StateBuilder {
    pub fn new(agent_id: String, state: String) -> Self {
        Self {
            agent_id,
            state,
            context: PersistentContext::default(),
            goals: Vec::new(),
            tasks: Vec::new(),
            history: Vec::new(),
            variables: HashMap::new(),
        }
    }
    
    pub fn with_context(mut self, context: PersistentContext) -> Self {
        self.context = context;
        self
    }
    
    pub fn with_goal(mut self, goal: PersistentGoal) -> Self {
        self.goals.push(goal);
        self
    }
    
    pub fn with_task(mut self, task: PersistentTask) -> Self {
        self.tasks.push(task);
        self
    }
    
    pub fn with_history(mut self, step: ExecutionStep) -> Self {
        self.history.push(step);
        self
    }
    
    pub fn with_variable(mut self, key: String, value: serde_json::Value) -> Self {
        self.variables.insert(key, value);
        self
    }
    
    pub fn build(self) -> StateSnapshot {
        StateSnapshot {
            id: uuid::Uuid::new_v4().to_string(),
            agent_id: self.agent_id,
            snapshot_type: SnapshotType::Manual,
            state: self.state,
            context: self.context,
            goals: self.goals,
            task_queue: self.tasks,
            execution_history: self.history,
            variables: self.variables,
            timestamp: Utc::now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            checksum: String::new(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ERROR TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub enum PersistenceError {
    IoError(String),
    SerializationError(String),
    DeserializationError(String),
    ChecksumMismatch,
    SnapshotNotFound(String),
    BackendNotImplemented,
}

impl std::fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "I/O error: {}", e),
            Self::SerializationError(e) => write!(f, "Serialization error: {}", e),
            Self::DeserializationError(e) => write!(f, "Deserialization error: {}", e),
            Self::ChecksumMismatch => write!(f, "Checksum mismatch - data corrupted"),
            Self::SnapshotNotFound(id) => write!(f, "Snapshot not found: {}", id),
            Self::BackendNotImplemented => write!(f, "Backend not implemented"),
        }
    }
}

impl std::error::Error for PersistenceError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_state_builder() {
        let snapshot = StateBuilder::new("agent-1".into(), "Running".into())
            .with_variable("count".into(), serde_json::json!(42))
            .build();
        
        assert_eq!(snapshot.agent_id, "agent-1");
        assert_eq!(snapshot.state, "Running");
    }
    
    #[test]
    fn test_persistent_context_default() {
        let ctx = PersistentContext::default();
        assert_eq!(ctx.step_count, 0);
        assert_eq!(ctx.total_tokens, 0);
    }
    
    #[tokio::test]
    async fn test_state_persistence_memory() {
        let config = PersistenceConfig {
            backend: StorageBackend::Memory,
            ..Default::default()
        };
        
        let persistence = StatePersistence::new(config);
        
        let snapshot = StateBuilder::new("agent-1".into(), "Running".into()).build();
        let id = snapshot.id.clone();
        
        persistence.save(snapshot).await.unwrap();
        
        let loaded = persistence.load(&id).await.unwrap();
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().agent_id, "agent-1");
    }
}
