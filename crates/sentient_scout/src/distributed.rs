//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT Distributed Scraping Coordinator
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  Distributed web scraping with:
//!  - Multi-node coordination
//!  - Task queue management
//!  - Result aggregation
//!  - Fault tolerance & recovery
//!  - Smart URL distribution

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc, Semaphore};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════════════════════
//  DISTRIBUTED NODE TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Scraping node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapingNode {
    /// Node ID
    pub id: String,
    /// Node endpoint URL
    pub endpoint: String,
    /// Node capabilities
    pub capabilities: NodeCapabilities,
    /// Current status
    pub status: NodeStatus,
    /// Current load (0.0 - 1.0)
    pub load: f32,
    /// Success rate (0.0 - 1.0)
    pub success_rate: f32,
    /// Last heartbeat
    pub last_heartbeat: DateTime<Utc>,
    /// Total tasks completed
    pub tasks_completed: u64,
    /// Total bytes scraped
    pub bytes_scraped: u64,
}

impl ScrapingNode {
    pub fn new(endpoint: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            endpoint,
            capabilities: NodeCapabilities::default(),
            status: NodeStatus::Idle,
            load: 0.0,
            success_rate: 1.0,
            last_heartbeat: Utc::now(),
            tasks_completed: 0,
            bytes_scraped: 0,
        }
    }
    
    pub fn is_available(&self) -> bool {
        matches!(self.status, NodeStatus::Idle | NodeStatus::Busy)
            && self.success_rate > 0.5
    }
    
    pub fn can_handle(&self, task: &ScrapingTask) -> bool {
        // Check if node can handle the task based on capabilities
        if task.requires_javascript && !self.capabilities.javascript {
            return false;
        }
        if task.requires_proxies && !self.capabilities.proxies {
            return false;
        }
        true
    }
}

/// Node capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCapabilities {
    /// Can execute JavaScript
    pub javascript: bool,
    /// Has proxy support
    pub proxies: bool,
    /// Can solve captchas
    pub captcha_solver: bool,
    /// Max concurrent tasks
    pub max_concurrent: u32,
    /// Supported domains
    pub supported_domains: Vec<String>,
    /// Region/geo location
    pub region: String,
}

impl Default for NodeCapabilities {
    fn default() -> Self {
        Self {
            javascript: true,
            proxies: true,
            captcha_solver: false,
            max_concurrent: 10,
            supported_domains: vec!["*".into()],
            region: "default".into(),
        }
    }
}

/// Node status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeStatus {
    /// Node is online and idle
    Idle,
    /// Node is processing tasks
    Busy,
    /// Node is offline
    Offline,
    /// Node is paused for maintenance
    Maintenance,
    /// Node has errors
    Error,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SCRAPING TASK TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Scraping task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapingTask {
    /// Task ID
    pub id: String,
    /// URLs to scrape
    pub urls: Vec<String>,
    /// Priority (higher = more important)
    pub priority: u8,
    /// Task type
    pub task_type: TaskType,
    /// Configuration
    pub config: ScrapingConfig,
    /// Requires JavaScript rendering
    pub requires_javascript: bool,
    /// Requires proxy rotation
    pub requires_proxies: bool,
    /// Max retries
    pub max_retries: u32,
    /// Current retry count
    pub retry_count: u32,
    /// Parent job ID
    pub job_id: String,
    /// Creation time
    pub created_at: DateTime<Utc>,
    /// Deadline
    pub deadline: Option<DateTime<Utc>>,
    /// Assigned node
    pub assigned_node: Option<String>,
}

impl ScrapingTask {
    pub fn new(job_id: String, urls: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            urls,
            priority: 5,
            task_type: TaskType::SinglePage,
            config: ScrapingConfig::default(),
            requires_javascript: false,
            requires_proxies: false,
            max_retries: 3,
            retry_count: 0,
            job_id,
            created_at: Utc::now(),
            deadline: None,
            assigned_node: None,
        }
    }
    
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority.min(10);
        self
    }
    
    pub fn with_javascript(mut self) -> Self {
        self.requires_javascript = true;
        self
    }
    
    pub fn with_proxies(mut self) -> Self {
        self.requires_proxies = true;
        self
    }
    
    pub fn with_deadline(mut self, deadline: DateTime<Utc>) -> Self {
        self.deadline = Some(deadline);
        self
    }
    
    pub fn is_expired(&self) -> bool {
        self.deadline.map(|d| Utc::now() > d).unwrap_or(false)
    }
}

/// Task type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskType {
    /// Single page scrape
    SinglePage,
    /// Paginated scrape
    Paginated,
    /// Follow links
    Crawl,
    /// Sitemap-based
    Sitemap,
    /// API endpoint
    ApiEndpoint,
    /// Search results
    SearchResults,
    /// Social media profile
    SocialProfile,
}

/// Scraping configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapingConfig {
    /// Request timeout in seconds
    pub timeout_secs: u64,
    /// Delay between requests (ms)
    pub delay_ms: u64,
    /// User agent
    pub user_agent: Option<String>,
    /// Headers
    pub headers: HashMap<String, String>,
    /// Cookies
    pub cookies: HashMap<String, String>,
    /// Wait for selector
    pub wait_for_selector: Option<String>,
    /// Extract rules
    pub extract_rules: Vec<ExtractRule>,
    /// Output format
    pub output_format: OutputFormat,
}

impl Default for ScrapingConfig {
    fn default() -> Self {
        Self {
            timeout_secs: 30,
            delay_ms: 1000,
            user_agent: None,
            headers: HashMap::new(),
            cookies: HashMap::new(),
            wait_for_selector: None,
            extract_rules: Vec::new(),
            output_format: OutputFormat::Json,
        }
    }
}

/// Extraction rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractRule {
    /// Field name
    pub field: String,
    /// CSS selector
    pub selector: String,
    /// Attribute to extract (None = text)
    pub attribute: Option<String>,
    /// Is multiple values
    pub multiple: bool,
}

/// Output format
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OutputFormat {
    Json,
    Csv,
    Xml,
    Html,
    Markdown,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SCRAPING JOB
// ═══════════════════════════════════════════════════════════════════════════════

/// Scraping job (collection of tasks)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapingJob {
    /// Job ID
    pub id: String,
    /// Job name
    pub name: String,
    /// Seed URLs
    pub seed_urls: Vec<String>,
    /// Total tasks
    pub total_tasks: usize,
    /// Completed tasks
    pub completed_tasks: usize,
    /// Failed tasks
    pub failed_tasks: usize,
    /// Job status
    pub status: JobStatus,
    /// Priority
    pub priority: u8,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Started at
    pub started_at: Option<DateTime<Utc>>,
    /// Completed at
    pub completed_at: Option<DateTime<Utc>>,
    /// Total bytes scraped
    pub total_bytes: u64,
    /// Results
    pub results: Vec<ScrapingResult>,
}

impl ScrapingJob {
    pub fn new(name: String, seed_urls: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            seed_urls,
            total_tasks: 0,
            completed_tasks: 0,
            failed_tasks: 0,
            status: JobStatus::Pending,
            priority: 5,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            total_bytes: 0,
            results: Vec::new(),
        }
    }
    
    pub fn progress(&self) -> f32 {
        if self.total_tasks == 0 {
            return 0.0;
        }
        self.completed_tasks as f32 / self.total_tasks as f32
    }
    
    pub fn is_complete(&self) -> bool {
        matches!(self.status, JobStatus::Completed | JobStatus::Failed | JobStatus::Cancelled)
    }
}

/// Job status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum JobStatus {
    Pending,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

/// Scraping result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapingResult {
    /// Task ID
    pub task_id: String,
    /// Node ID
    pub node_id: String,
    /// URL scraped
    pub url: String,
    /// Status code
    pub status_code: u16,
    /// Response time in ms
    pub response_time_ms: u64,
    /// Size in bytes
    pub size_bytes: u64,
    /// Extracted data
    pub data: Option<serde_json::Value>,
    /// Raw HTML (if stored)
    pub raw_html: Option<String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Error message (if failed)
    pub error: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DISTRIBUTED COORDINATOR
// ═══════════════════════════════════════════════════════════════════════════════

/// Distributed scraping coordinator
pub struct DistributedCoordinator {
    /// Registered nodes
    nodes: Arc<RwLock<HashMap<String, ScrapingNode>>>,
    /// Task queue (priority queue)
    task_queue: Arc<RwLock<VecDeque<ScrapingTask>>>,
    /// Running jobs
    jobs: Arc<RwLock<HashMap<String, ScrapingJob>>>,
    /// Result buffer
    results: mpsc::Sender<ScrapingResult>,
    /// Max concurrent tasks per node
    max_concurrent: usize,
    /// Heartbeat timeout in seconds
    heartbeat_timeout_secs: u64,
    /// Task assignment semaphore
    semaphore: Arc<Semaphore>,
}

impl DistributedCoordinator {
    pub fn new(results_tx: mpsc::Sender<ScrapingResult>) -> Self {
        Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            task_queue: Arc::new(RwLock::new(VecDeque::new())),
            jobs: Arc::new(RwLock::new(HashMap::new())),
            results: results_tx,
            max_concurrent: 10,
            heartbeat_timeout_secs: 30,
            semaphore: Arc::new(Semaphore::new(100)),
        }
    }
    
    /// Register a new node
    pub async fn register_node(&self, node: ScrapingNode) -> String {
        let node_id = node.id.clone();
        let mut nodes = self.nodes.write().await;
        nodes.insert(node_id.clone(), node);
        log::info!("🌐 Node registered: {}", node_id);
        node_id
    }
    
    /// Unregister a node
    pub async fn unregister_node(&self, node_id: &str) {
        let mut nodes = self.nodes.write().await;
        if nodes.remove(node_id).is_some() {
            log::info!("🌐 Node unregistered: {}", node_id);
        }
    }
    
    /// Update node heartbeat
    pub async fn heartbeat(&self, node_id: &str) -> bool {
        let mut nodes = self.nodes.write().await;
        if let Some(node) = nodes.get_mut(node_id) {
            node.last_heartbeat = Utc::now();
            true
        } else {
            false
        }
    }
    
    /// Submit a new job
    pub async fn submit_job(&self, mut job: ScrapingJob) -> String {
        let job_id = job.id.clone();
        
        // Create tasks from seed URLs
        let mut tasks = VecDeque::new();
        for url in &job.seed_urls {
            tasks.push_back(ScrapingTask::new(job_id.clone(), vec![url.clone()]));
        }
        job.total_tasks = tasks.len();
        job.status = JobStatus::Pending;
        
        // Add tasks to queue
        let mut queue = self.task_queue.write().await;
        queue.extend(tasks);
        
        // Store job
        let mut jobs = self.jobs.write().await;
        jobs.insert(job_id.clone(), job);
        
        log::info!("📋 Job submitted: {} ({} tasks)", job_id, queue.len());
        job_id
    }
    
    /// Get next task for a node
    pub async fn get_task(&self, node_id: &str) -> Option<ScrapingTask> {
        let nodes = self.nodes.read().await;
        let node = nodes.get(node_id)?;
        
        if !node.is_available() {
            return None;
        }
        
        let mut queue = self.task_queue.write().await;
        
        // Find best task for this node
        let task_idx = queue.iter().position(|t| {
            t.assigned_node.is_none() && node.can_handle(t) && !t.is_expired()
        })?;
        
        let mut task = queue.remove(task_idx)?;
        task.assigned_node = Some(node_id.to_string());
        
        log::debug!("📤 Task {} assigned to node {}", task.id, node_id);
        Some(task)
    }
    
    /// Submit task result
    pub async fn submit_result(&self, result: ScrapingResult) {
        let node_id = result.node_id.clone();
        let task_id = result.task_id.clone();
        
        // Update job progress
        let mut jobs = self.jobs.write().await;
        if let Some(task) = result.task_id.split('-').next() {
            if let Some(job) = jobs.get_mut(task) {
                if result.error.is_some() {
                    job.failed_tasks += 1;
                } else {
                    job.completed_tasks += 1;
                    job.total_bytes += result.size_bytes;
                    job.results.push(result.clone());
                }
                
                // Check if job is complete
                if job.completed_tasks + job.failed_tasks >= job.total_tasks {
                    job.status = if job.failed_tasks > job.completed_tasks {
                        JobStatus::Failed
                    } else {
                        JobStatus::Completed
                    };
                    job.completed_at = Some(Utc::now());
                    log::info!("✅ Job {} {} - {}/{} tasks", 
                        job.id, 
                        if job.status == JobStatus::Completed { "completed" } else { "failed" },
                        job.completed_tasks, 
                        job.total_tasks
                    );
                }
            }
        }
        
        // Update node stats
        let mut nodes = self.nodes.write().await;
        if let Some(node) = nodes.get_mut(&node_id) {
            node.tasks_completed += 1;
            node.bytes_scraped += result.size_bytes;
            if result.error.is_none() {
                node.success_rate = (node.success_rate * 0.9) + 0.1;
            } else {
                node.success_rate = (node.success_rate * 0.9);
            }
        }
        
        // Send result to channel
        let _ = self.results.send(result).await;
    }
    
    /// Check node health and remove stale nodes
    pub async fn health_check(&self) {
        let mut nodes = self.nodes.write().await;
        let now = Utc::now();
        let timeout = chrono::Duration::seconds(self.heartbeat_timeout_secs as i64);
        
        let dead_nodes: Vec<String> = nodes
            .iter()
            .filter(|(_, node)| now - node.last_heartbeat > timeout)
            .map(|(id, _)| id.clone())
            .collect();
        
        for node_id in dead_nodes {
            log::warn!("💀 Node {} removed (heartbeat timeout)", node_id);
            nodes.remove(&node_id);
        }
    }
    
    /// Get coordinator stats
    pub async fn stats(&self) -> CoordinatorStats {
        let nodes = self.nodes.read().await;
        let queue = self.task_queue.read().await;
        let jobs = self.jobs.read().await;
        
        CoordinatorStats {
            total_nodes: nodes.len(),
            active_nodes: nodes.values().filter(|n| n.is_available()).count(),
            pending_tasks: queue.len(),
            total_jobs: jobs.len(),
            running_jobs: jobs.values().filter(|j| j.status == JobStatus::Running).count(),
            completed_jobs: jobs.values().filter(|j| j.status == JobStatus::Completed).count(),
        }
    }
}

/// Coordinator statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinatorStats {
    pub total_nodes: usize,
    pub active_nodes: usize,
    pub pending_tasks: usize,
    pub total_jobs: usize,
    pub running_jobs: usize,
    pub completed_jobs: usize,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  URL DISTRIBUTOR
// ═══════════════════════════════════════════════════════════════════════════════

/// URL distribution strategy
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DistributionStrategy {
    /// Round-robin
    RoundRobin,
    /// Least loaded node
    LeastLoaded,
    /// By domain (same domain to same node)
    ByDomain,
    /// By region
    ByRegion,
    /// Random
    Random,
}

/// URL distributor
pub struct UrlDistributor {
    strategy: DistributionStrategy,
    domain_mapping: HashMap<String, String>, // domain -> node_id
}

impl UrlDistributor {
    pub fn new(strategy: DistributionStrategy) -> Self {
        Self {
            strategy,
            domain_mapping: HashMap::new(),
        }
    }
    
    /// Get domain from URL
    fn get_domain(url: &str) -> Option<String> {
        url::Url::parse(url)
            .ok()
            .and_then(|u| u.host_str().map(|s| s.to_string()))
    }
    
    /// Distribute URLs among nodes
    pub fn distribute(&mut self, urls: Vec<String>, nodes: &[ScrapingNode]) -> HashMap<String, Vec<String>> {
        if nodes.is_empty() {
            return HashMap::new();
        }
        
        let mut distribution: HashMap<String, Vec<String>> = HashMap::new();
        for node in nodes {
            distribution.insert(node.id.clone(), Vec::new());
        }
        
        match self.strategy {
            DistributionStrategy::RoundRobin => {
                for (i, url) in urls.into_iter().enumerate() {
                    let node_id = &nodes[i % nodes.len()].id;
                    distribution.get_mut(node_id).unwrap().push(url);
                }
            }
            DistributionStrategy::ByDomain => {
                for url in urls {
                    let domain = Self::get_domain(&url).unwrap_or_default();
                    let node_id = if let Some(existing) = self.domain_mapping.get(&domain) {
                        existing.clone()
                    } else {
                        // Assign to node with least domains
                        let mut counts: HashMap<String, usize> = HashMap::new();
                        for nid in self.domain_mapping.values() {
                            *counts.entry(nid.clone()).or_insert(0) += 1;
                        }
                        let best_node = nodes.iter()
                            .min_by_key(|n| counts.get(&n.id).unwrap_or(&0))
                            .map(|n| n.id.clone())
                            .unwrap_or_default();
                        self.domain_mapping.insert(domain.clone(), best_node.clone());
                        best_node
                    };
                    distribution.get_mut(&node_id).unwrap().push(url);
                }
            }
            DistributionStrategy::LeastLoaded => {
                for url in urls {
                    let node_id = nodes.iter()
                        .min_by(|a, b| a.load.partial_cmp(&b.load).unwrap())
                        .map(|n| n.id.clone())
                        .unwrap_or_default();
                    distribution.get_mut(&node_id).unwrap().push(url);
                }
            }
            DistributionStrategy::Random => {
                use rand::seq::SliceRandom;
                let mut rng = rand::thread_rng();
                for url in urls {
                    let node = nodes.choose(&mut rng).unwrap();
                    distribution.get_mut(&node.id).unwrap().push(url);
                }
            }
            DistributionStrategy::ByRegion => {
                // Group by node region
                for url in urls {
                    let node_id = nodes.first().map(|n| n.id.clone()).unwrap_or_default();
                    distribution.get_mut(&node_id).unwrap().push(url);
                }
            }
        }
        
        distribution
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_node_creation() {
        let node = ScrapingNode::new("http://localhost:8080".into());
        assert!(node.is_available());
        assert!(node.capabilities.javascript);
    }
    
    #[test]
    fn test_task_creation() {
        let task = ScrapingTask::new("job-1".into(), vec!["https://example.com".into()])
            .with_priority(8)
            .with_javascript();
        
        assert_eq!(task.priority, 8);
        assert!(task.requires_javascript);
    }
    
    #[test]
    fn test_job_progress() {
        let mut job = ScrapingJob::new("test".into(), vec!["https://example.com".into()]);
        job.total_tasks = 10;
        job.completed_tasks = 5;
        assert!((job.progress() - 0.5).abs() < 0.001);
    }
    
    #[tokio::test]
    async fn test_coordinator_registration() {
        let (tx, _rx) = mpsc::channel(100);
        let coord = DistributedCoordinator::new(tx);
        
        let node = ScrapingNode::new("http://localhost:8080".into());
        let node_id = coord.register_node(node).await;
        
        assert!(!node_id.is_empty());
        
        let stats = coord.stats().await;
        assert_eq!(stats.total_nodes, 1);
    }
}
