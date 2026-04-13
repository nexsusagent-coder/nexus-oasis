//! ─── Distributed Inference ───
//!
//! Multi-node, multi-GPU inference coordination
//! - Model sharding across nodes
//! - Load balancing
//! - Fault tolerance
//! - Auto-scaling

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use crate::{ChatRequest, ChatResponse, LlmError, LlmResult};

// ═══════════════════════════════════════════════════════════════════════════════
//  NODE CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Node configuration for distributed inference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Unique node identifier
    pub id: String,
    /// Node endpoint URL
    pub endpoint: String,
    /// GPU count on this node
    pub gpu_count: usize,
    /// Total VRAM in GB
    pub vram_gb: usize,
    /// Maximum concurrent requests
    pub max_concurrent: usize,
    /// Node priority (lower = higher priority)
    pub priority: u32,
    /// Node labels for routing
    pub labels: HashMap<String, String>,
    /// Health check endpoint
    pub health_endpoint: String,
    /// Authentication token
    pub auth_token: Option<String>,
}

impl NodeConfig {
    /// Create new node configuration
    pub fn new(id: impl Into<String>, endpoint: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            endpoint: endpoint.into(),
            gpu_count: 1,
            vram_gb: 24,
            max_concurrent: 10,
            priority: 0,
            labels: HashMap::new(),
            health_endpoint: "/health".into(),
            auth_token: None,
        }
    }

    /// Set GPU count
    pub fn with_gpus(mut self, count: usize) -> Self {
        self.gpu_count = count;
        self
    }

    /// Set VRAM
    pub fn with_vram(mut self, gb: usize) -> Self {
        self.vram_gb = gb;
        self
    }

    /// Set priority
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    /// Add label
    pub fn with_label(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.labels.insert(key.into(), value.into());
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  NODE STATUS
// ═══════════════════════════════════════════════════════════════════════════════

/// Node health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeStatus {
    /// Node is healthy and accepting requests
    Healthy,
    /// Node is degraded but still functional
    Degraded,
    /// Node is unhealthy
    Unhealthy,
    /// Node is draining for maintenance
    Draining,
    /// Node is offline
    Offline,
}

/// Node runtime statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NodeStats {
    /// Current request count
    pub current_requests: usize,
    /// Total requests processed
    pub total_requests: u64,
    /// Total tokens generated
    pub total_tokens: u64,
    /// Average latency in ms
    pub avg_latency_ms: f64,
    /// GPU utilization (0-100)
    pub gpu_utilization: f32,
    /// VRAM used (GB)
    pub vram_used_gb: f32,
    /// Error count
    pub error_count: u64,
    /// Last error message
    pub last_error: Option<String>,
    /// Uptime in seconds
    pub uptime_secs: u64,
}

/// Runtime node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Node configuration
    pub config: NodeConfig,
    /// Current status
    pub status: NodeStatus,
    /// Runtime statistics
    pub stats: NodeStats,
    /// Last health check time
    pub last_health_check: u64,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  LOAD BALANCING
// ═══════════════════════════════════════════════════════════════════════════════

/// Load balancing strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBalanceStrategy {
    /// Round-robin distribution
    RoundRobin,
    /// Least connections
    LeastConnections,
    /// Least latency
    LeastLatency,
    /// Weighted by capacity
    WeightedCapacity,
    /// Resource-aware (GPU + VRAM)
    ResourceAware,
    /// Random distribution
    Random,
}

impl Default for LoadBalanceStrategy {
    fn default() -> Self {
        Self::LeastConnections
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SHARDING CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Model sharding strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShardingStrategy {
    /// No sharding (single node)
    None,
    /// Tensor parallelism (split layers across GPUs)
    TensorParallel {
        /// Number of GPUs to split across
        num_gpus: usize,
    },
    /// Pipeline parallelism (split layers across nodes)
    PipelineParallel {
        /// Number of pipeline stages
        num_stages: usize,
    },
    /// Hybrid (tensor + pipeline)
    Hybrid {
        /// Tensor parallel size
        tensor_size: usize,
        /// Pipeline parallel size
        pipeline_size: usize,
    },
}

impl Default for ShardingStrategy {
    fn default() -> Self {
        Self::None
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DISTRIBUTED CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Distributed inference configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedConfig {
    /// Enable distributed mode
    pub enabled: bool,
    /// Load balancing strategy
    pub load_balance: LoadBalanceStrategy,
    /// Sharding strategy
    pub sharding: ShardingStrategy,
    /// Health check interval in seconds
    pub health_check_interval_secs: u64,
    /// Request timeout in seconds
    pub request_timeout_secs: u64,
    /// Maximum retries
    pub max_retries: u32,
    /// Retry delay in ms
    pub retry_delay_ms: u64,
    /// Enable auto-scaling
    pub auto_scale: bool,
    /// Minimum nodes
    pub min_nodes: usize,
    /// Maximum nodes
    pub max_nodes: usize,
    /// Scale-up threshold (GPU utilization)
    pub scale_up_threshold: f32,
    /// Scale-down threshold
    pub scale_down_threshold: f32,
}

impl Default for DistributedConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            load_balance: LoadBalanceStrategy::LeastConnections,
            sharding: ShardingStrategy::None,
            health_check_interval_secs: 30,
            request_timeout_secs: 60,
            max_retries: 3,
            retry_delay_ms: 100,
            auto_scale: false,
            min_nodes: 1,
            max_nodes: 10,
            scale_up_threshold: 80.0,
            scale_down_threshold: 30.0,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DISTRIBUTED CLUSTER
// ═══════════════════════════════════════════════════════════════════════════════

/// Distributed inference cluster manager
pub struct DistributedCluster {
    config: DistributedConfig,
    nodes: Arc<RwLock<HashMap<String, NodeInfo>>>,
    round_robin_index: Arc<RwLock<usize>>,
    stats: Arc<RwLock<ClusterStats>>,
}

/// Cluster-wide statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClusterStats {
    pub total_nodes: usize,
    pub healthy_nodes: usize,
    pub total_requests: u64,
    pub total_tokens: u64,
    pub avg_latency_ms: f64,
    pub total_errors: u64,
}

impl DistributedCluster {
    /// Create new cluster
    pub fn new(config: DistributedConfig) -> Self {
        Self {
            config,
            nodes: Arc::new(RwLock::new(HashMap::new())),
            round_robin_index: Arc::new(RwLock::new(0)),
            stats: Arc::new(RwLock::new(ClusterStats::default())),
        }
    }

    /// Add node to cluster
    pub fn add_node(&self, config: NodeConfig) {
        let info = NodeInfo {
            config,
            status: NodeStatus::Offline,
            stats: NodeStats::default(),
            last_health_check: 0,
        };
        
        self.nodes.write().insert(info.config.id.clone(), info);
        self.update_cluster_stats();
    }

    /// Remove node from cluster
    pub fn remove_node(&self, node_id: &str) {
        self.nodes.write().remove(node_id);
        self.update_cluster_stats();
    }

    /// Get all nodes
    pub fn nodes(&self) -> Vec<NodeInfo> {
        self.nodes.read().values().cloned().collect()
    }

    /// Get healthy nodes
    pub fn healthy_nodes(&self) -> Vec<NodeInfo> {
        self.nodes.read()
            .values()
            .filter(|n| n.status == NodeStatus::Healthy)
            .cloned()
            .collect()
    }

    /// Select node for request
    pub fn select_node(&self) -> Option<NodeInfo> {
        let nodes = self.healthy_nodes();
        if nodes.is_empty() {
            return None;
        }

        match self.config.load_balance {
            LoadBalanceStrategy::RoundRobin => {
                let mut idx = self.round_robin_index.write();
                let node = nodes[*idx % nodes.len()].clone();
                *idx = (*idx + 1) % nodes.len();
                Some(node)
            }
            LoadBalanceStrategy::LeastConnections => {
                nodes.into_iter()
                    .min_by_key(|n| n.stats.current_requests)
            }
            LoadBalanceStrategy::LeastLatency => {
                nodes.into_iter()
                    .min_by(|a, b| a.stats.avg_latency_ms.partial_cmp(&b.stats.avg_latency_ms).unwrap())
            }
            LoadBalanceStrategy::WeightedCapacity => {
                nodes.into_iter()
                    .max_by(|a, b| {
                        let score_a = a.config.gpu_count * a.config.vram_gb;
                        let score_b = b.config.gpu_count * b.config.vram_gb;
                        score_a.cmp(&score_b)
                    })
            }
            LoadBalanceStrategy::ResourceAware => {
                nodes.into_iter()
                    .min_by(|a, b| {
                        let score_a = a.stats.gpu_utilization + (a.stats.vram_used_gb / a.config.vram_gb as f32) * 100.0;
                        let score_b = b.stats.gpu_utilization + (b.stats.vram_used_gb / b.config.vram_gb as f32) * 100.0;
                        score_a.partial_cmp(&score_b).unwrap()
                    })
            }
            LoadBalanceStrategy::Random => {
                use rand::Rng;
                let idx = rand::thread_rng().gen_range(0..nodes.len());
                nodes.into_iter().nth(idx)
            }
        }
    }

    /// Update node status
    pub fn update_node_status(&self, node_id: &str, status: NodeStatus) {
        if let Some(node) = self.nodes.write().get_mut(node_id) {
            node.status = status;
            node.last_health_check = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }
        self.update_cluster_stats();
    }

    /// Update node stats
    pub fn update_node_stats(&self, node_id: &str, stats: NodeStats) {
        if let Some(node) = self.nodes.write().get_mut(node_id) {
            node.stats = stats;
        }
    }

    /// Get cluster stats
    pub fn stats(&self) -> ClusterStats {
        self.stats.read().clone()
    }

    /// Update cluster stats
    fn update_cluster_stats(&self) {
        let nodes = self.nodes.read();
        let mut stats = self.stats.write();
        
        stats.total_nodes = nodes.len();
        stats.healthy_nodes = nodes.values().filter(|n| n.status == NodeStatus::Healthy).count();
        stats.total_requests = nodes.values().map(|n| n.stats.total_requests).sum();
        stats.total_tokens = nodes.values().map(|n| n.stats.total_tokens).sum();
        stats.total_errors = nodes.values().map(|n| n.stats.error_count).sum();
        
        let healthy_count = stats.healthy_nodes as f64;
        if healthy_count > 0.0 {
            stats.avg_latency_ms = nodes.values()
                .filter(|n| n.status == NodeStatus::Healthy)
                .map(|n| n.stats.avg_latency_ms)
                .sum::<f64>() / healthy_count;
        }
    }

    /// Check cluster health
    pub async fn health_check(&self) -> HashMap<String, NodeStatus> {
        let mut results = HashMap::new();
        
        for (id, node) in self.nodes.read().iter() {
            let status = self.check_node_health(node).await;
            results.insert(id.clone(), status);
        }
        
        // Update statuses
        for (id, status) in &results {
            self.update_node_status(id, *status);
        }
        
        results
    }

    /// Check single node health
    async fn check_node_health(&self, node: &NodeInfo) -> NodeStatus {
        let client = reqwest::Client::new();
        let url = format!("{}{}", node.config.endpoint, node.config.health_endpoint);
        
        match client
            .get(&url)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
        {
            Ok(response) if response.status().is_success() => NodeStatus::Healthy,
            Ok(_) => NodeStatus::Degraded,
            Err(_) => NodeStatus::Unhealthy,
        }
    }

    /// Get configuration
    pub fn config(&self) -> &DistributedConfig {
        &self.config
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DISTRIBUTED CLIENT
// ═══════════════════════════════════════════════════════════════════════════════

/// Client for distributed inference
pub struct DistributedClient {
    cluster: Arc<DistributedCluster>,
    http_client: reqwest::Client,
}

impl DistributedClient {
    /// Create new client
    pub fn new(cluster: Arc<DistributedCluster>) -> Self {
        Self {
            cluster,
            http_client: reqwest::Client::new(),
        }
    }

    /// Execute chat request with failover
    pub async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let mut last_error = None;
        
        for attempt in 0..=self.cluster.config.max_retries {
            // Select node
            let node = self.cluster.select_node()
                .ok_or_else(|| LlmError::NoHealthyNodes)?;
            
            // Make request
            match self.make_request(&node, &request).await {
                Ok(response) => {
                    // Update stats
                    self.cluster.update_node_stats(&node.config.id, NodeStats {
                        current_requests: node.stats.current_requests.saturating_sub(1),
                        total_requests: node.stats.total_requests + 1,
                        ..node.stats.clone()
                    });
                    return Ok(response);
                }
                Err(e) => {
                    last_error = Some(e);
                    
                    // Mark node as degraded on failure
                    self.cluster.update_node_status(&node.config.id, NodeStatus::Degraded);
                    
                    // Wait before retry
                    if attempt < self.cluster.config.max_retries {
                        tokio::time::sleep(tokio::time::Duration::from_millis(
                            self.cluster.config.retry_delay_ms * (attempt + 1) as u64
                        )).await;
                    }
                }
            }
        }
        
        Err(last_error.unwrap_or(LlmError::NoHealthyNodes))
    }

    /// Make request to node
    async fn make_request(&self, node: &NodeInfo, request: &ChatRequest) -> LlmResult<ChatResponse> {
        let url = format!("{}/v1/chat/completions", node.config.endpoint);
        
        let mut req = self.http_client
            .post(&url)
            .json(request)
            .timeout(std::time::Duration::from_secs(self.cluster.config.request_timeout_secs));
        
        if let Some(token) = &node.config.auth_token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }
        
        let response = req.send().await
            .map_err(|e| LlmError::RequestFailed(e.to_string()))?;
        
        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(LlmError::ApiError(text));
        }
        
        response.json().await
            .map_err(|e| LlmError::ParseError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_config() {
        let config = NodeConfig::new("node-1", "http://localhost:8080")
            .with_gpus(4)
            .with_vram(80);
        
        assert_eq!(config.id, "node-1");
        assert_eq!(config.gpu_count, 4);
        assert_eq!(config.vram_gb, 80);
    }

    #[test]
    fn test_cluster_creation() {
        let config = DistributedConfig::default();
        let cluster = DistributedCluster::new(config);
        
        let stats = cluster.stats();
        assert_eq!(stats.total_nodes, 0);
    }

    #[test]
    fn test_node_selection() {
        let config = DistributedConfig::default();
        let cluster = DistributedCluster::new(config);
        
        // Add nodes
        cluster.add_node(NodeConfig::new("node-1", "http://localhost:8080"));
        cluster.add_node(NodeConfig::new("node-2", "http://localhost:8081"));
        
        // Mark as healthy
        cluster.update_node_status("node-1", NodeStatus::Healthy);
        cluster.update_node_status("node-2", NodeStatus::Healthy);
        
        // Should be able to select node
        let node = cluster.select_node();
        assert!(node.is_some());
    }

    #[test]
    fn test_load_balance_strategies() {
        let strategies = vec![
            LoadBalanceStrategy::RoundRobin,
            LoadBalanceStrategy::LeastConnections,
            LoadBalanceStrategy::LeastLatency,
            LoadBalanceStrategy::WeightedCapacity,
            LoadBalanceStrategy::ResourceAware,
            LoadBalanceStrategy::Random,
        ];
        
        assert_eq!(strategies.len(), 6);
    }
}
