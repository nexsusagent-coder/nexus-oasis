//! Connector trait and registry

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{ConnectorError, ConnectorResult, Credentials, Document, SyncConfig};

// ═══════════════════════════════════════════════════════════════════════════════
// CONNECTOR TRAIT
// ═══════════════════════════════════════════════════════════════════════════════

/// Connector status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectorStatus {
    Disconnected,
    Connecting,
    Connected,
    Syncing,
    Error(String),
}

impl std::fmt::Display for ConnectorStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectorStatus::Disconnected => write!(f, "disconnected"),
            ConnectorStatus::Connecting => write!(f, "connecting"),
            ConnectorStatus::Connected => write!(f, "connected"),
            ConnectorStatus::Syncing => write!(f, "syncing"),
            ConnectorStatus::Error(e) => write!(f, "error: {}", e),
        }
    }
}

/// Sync result
#[derive(Debug, Clone)]
pub struct SyncResult {
    pub connector_id: String,
    pub items_synced: usize,
    pub items_new: usize,
    pub items_updated: usize,
    pub items_deleted: usize,
    pub sync_duration_ms: u64,
    pub last_sync: DateTime<Utc>,
    pub errors: Vec<String>,
}

impl SyncResult {
    pub fn new(connector_id: &str) -> Self {
        Self {
            connector_id: connector_id.to_string(),
            items_synced: 0,
            items_new: 0,
            items_updated: 0,
            items_deleted: 0,
            sync_duration_ms: 0,
            last_sync: Utc::now(),
            errors: Vec::new(),
        }
    }

    pub fn with_items(mut self, new: usize, updated: usize) -> Self {
        self.items_new = new;
        self.items_updated = updated;
        self.items_synced = new + updated;
        self
    }
}

/// Core connector trait - all connectors must implement this
#[async_trait]
pub trait Connector: Send + Sync {
    /// Unique connector identifier (e.g., "gmail", "weather", "rss")
    fn connector_id(&self) -> &str;

    /// Display name
    fn connector_name(&self) -> &str;

    /// Connector category (email, calendar, news, etc.)
    fn category(&self) -> &str;

    /// Current status
    fn status(&self) -> ConnectorStatus;

    /// Required credential type
    fn required_credentials(&self) -> Vec<String>;

    /// Connect with credentials
    async fn connect(&mut self, credentials: Credentials) -> ConnectorResult<()>;

    /// Disconnect
    async fn disconnect(&mut self) -> ConnectorResult<()>;

    /// Test connection
    async fn test_connection(&self) -> ConnectorResult<bool>;

    /// Sync data since given timestamp (None = full sync)
    async fn sync(&self, since: Option<DateTime<Utc>>, config: &SyncConfig) -> ConnectorResult<SyncResult>;

    /// Fetch documents
    async fn fetch(&self, query: &str, limit: usize) -> ConnectorResult<Vec<Document>>;

    /// Get a single document by ID
    async fn get_document(&self, id: &str) -> ConnectorResult<Option<Document>>;

    /// Search documents
    async fn search(&self, query: &str, limit: usize) -> ConnectorResult<Vec<Document>>;

    /// Get last sync timestamp
    fn last_sync(&self) -> Option<DateTime<Utc>>;

    /// Set sync config
    fn set_config(&mut self, config: SyncConfig);

    /// Get sync config
    fn config(&self) -> &SyncConfig;
}

// ═══════════════════════════════════════════════════════════════════════════════
// CONNECTOR REGISTRY
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};

/// Connector registry - manages all connectors
pub struct ConnectorRegistry {
    connectors: HashMap<String, Arc<RwLock<Box<dyn Connector>>>>,
    configs: HashMap<String, ConnectorConfig>,
}

/// Connector configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorConfig {
    pub enabled: bool,
    pub credentials: Option<Credentials>,
    pub sync_config: SyncConfig,
    pub last_sync: Option<DateTime<Utc>>,
}

impl Default for ConnectorConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            credentials: None,
            sync_config: SyncConfig::default(),
            last_sync: None,
        }
    }
}

impl ConnectorRegistry {
    pub fn new() -> Self {
        Self {
            connectors: HashMap::new(),
            configs: HashMap::new(),
        }
    }

    /// Register a connector
    pub fn register<C: Connector + 'static>(&mut self, connector: C) {
        let id = connector.connector_id().to_string();
        self.configs.insert(id.clone(), ConnectorConfig::default());
        self.connectors.insert(id, Arc::new(RwLock::new(Box::new(connector))));
    }

    /// Get a connector by ID
    pub fn get(&self, id: &str) -> Option<Arc<RwLock<Box<dyn Connector>>>> {
        self.connectors.get(id).cloned()
    }

    /// List all registered connectors
    pub fn list(&self) -> Vec<&str> {
        self.connectors.keys().map(|s| s.as_str()).collect()
    }

    /// List connectors by category
    pub fn list_by_category(&self, category: &str) -> Vec<String> {
        self.connectors
            .values()
            .filter_map(|c| {
                let guard = c.try_read().ok()?;
                if guard.category() == category {
                    Some(guard.connector_id().to_string())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Connect a connector
    pub async fn connect(&mut self, id: &str, credentials: Credentials) -> ConnectorResult<()> {
        let connector = self.connectors.get(id)
            .ok_or_else(|| ConnectorError::NotFound(id.to_string()))?;
        
        let mut guard = connector.write().await;
        guard.connect(credentials.clone()).await?;

        // Store credentials
        if let Some(config) = self.configs.get_mut(id) {
            config.credentials = Some(credentials);
        }

        Ok(())
    }

    /// Sync a connector
    pub async fn sync(&self, id: &str) -> ConnectorResult<SyncResult> {
        let connector = self.connectors.get(id)
            .ok_or_else(|| ConnectorError::NotFound(id.to_string()))?;
        
        let guard = connector.read().await;
        let config = guard.config().clone();
        let last_sync = guard.last_sync();
        
        guard.sync(last_sync, &config).await
    }

    /// Sync all connectors
    pub async fn sync_all(&self) -> HashMap<String, ConnectorResult<SyncResult>> {
        let mut results = HashMap::new();
        
        for (id, connector) in &self.connectors {
            let guard = connector.read().await;
            let config = guard.config().clone();
            let last_sync = guard.last_sync();
            let result = guard.sync(last_sync, &config).await;
            results.insert(id.clone(), result);
        }
        
        results
    }

    /// Get connector status
    pub async fn status(&self, id: &str) -> ConnectorResult<ConnectorStatus> {
        let connector = self.connectors.get(id)
            .ok_or_else(|| ConnectorError::NotFound(id.to_string()))?;
        
        let guard = connector.read().await;
        Ok(guard.status())
    }

    /// Check if connector is connected
    pub async fn is_connected(&self, id: &str) -> bool {
        if let Some(connector) = self.connectors.get(id) {
            let guard = connector.read().await;
            matches!(guard.status(), ConnectorStatus::Connected | ConnectorStatus::Syncing)
        } else {
            false
        }
    }
}

impl Default for ConnectorRegistry {
    fn default() -> Self {
        Self::new()
    }
}
