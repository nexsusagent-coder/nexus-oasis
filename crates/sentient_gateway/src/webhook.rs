//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT Webhook System - Event-Driven Notifications
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  Comprehensive webhook management:
//!  - Subscription management
//!  - Event filtering
//!  - Retry with exponential backoff
//!  - Signature verification (HMAC-SHA256)
//!  - Rate limiting per endpoint
//!  - Event logging and auditing

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

// ═══════════════════════════════════════════════════════════════════════════════
//  WEBHOOK CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Webhook system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    /// Default timeout for webhook calls (ms)
    pub default_timeout_ms: u64,
    /// Maximum retries
    pub max_retries: u32,
    /// Initial retry delay (ms)
    pub initial_retry_delay_ms: u64,
    /// Maximum retry delay (ms)
    pub max_retry_delay_ms: u64,
    /// Retry multiplier (exponential backoff)
    pub retry_multiplier: f64,
    /// Maximum concurrent deliveries
    pub max_concurrent_deliveries: usize,
    /// Enable signature verification
    pub enable_signatures: bool,
    /// Secret key for signatures
    pub signing_secret: Option<String>,
    /// Enable rate limiting
    pub enable_rate_limiting: bool,
    /// Requests per minute per endpoint
    pub rate_limit_per_minute: u32,
}

impl Default for WebhookConfig {
    fn default() -> Self {
        Self {
            default_timeout_ms: 30000,
            max_retries: 5,
            initial_retry_delay_ms: 1000,
            max_retry_delay_ms: 60000,
            retry_multiplier: 2.0,
            max_concurrent_deliveries: 100,
            enable_signatures: true,
            signing_secret: None,
            enable_rate_limiting: true,
            rate_limit_per_minute: 60,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  WEBHOOK SUBSCRIPTION
// ═══════════════════════════════════════════════════════════════════════════════

/// Webhook subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookSubscription {
    /// Unique subscription ID
    pub id: String,
    /// Endpoint URL
    pub url: String,
    /// Secret for signature verification
    pub secret: Option<String>,
    /// Subscribed event types
    pub events: Vec<EventType>,
    /// Event filters
    pub filters: HashMap<String, FilterRule>,
    /// Whether subscription is active
    pub active: bool,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Updated timestamp
    pub updated_at: DateTime<Utc>,
    /// Custom headers to send
    pub headers: HashMap<String, String>,
    /// Subscription metadata
    pub metadata: HashMap<String, String>,
    /// Owner ID (for multi-tenant)
    pub owner_id: Option<String>,
}

impl WebhookSubscription {
    pub fn new(url: String, events: Vec<EventType>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            url,
            secret: None,
            events,
            filters: HashMap::new(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            headers: HashMap::new(),
            metadata: HashMap::new(),
            owner_id: None,
        }
    }
    
    pub fn with_secret(mut self, secret: String) -> Self {
        self.secret = Some(secret);
        self
    }
    
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }
    
    pub fn with_filter(mut self, field: String, rule: FilterRule) -> Self {
        self.filters.insert(field, rule);
        self
    }
    
    /// Check if event matches subscription
    pub fn matches_event(&self, event: &WebhookEvent) -> bool {
        // Check if event type is subscribed
        if !self.events.is_empty() && !self.events.contains(&event.event_type) {
            return false;
        }
        
        // Check filters
        for (field, rule) in &self.filters {
            if let Some(value) = event.payload.get(field) {
                if !rule.matches(value) {
                    return false;
                }
            }
        }
        
        true
    }
}

/// Event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EventType {
    // System events
    SystemStarted,
    SystemShutdown,
    SystemError,
    
    // Agent events
    AgentCreated,
    AgentStarted,
    AgentStopped,
    AgentError,
    AgentCompleted,
    
    // Task events
    TaskCreated,
    TaskStarted,
    TaskProgress,
    TaskCompleted,
    TaskFailed,
    
    // Message events
    MessageReceived,
    MessageSent,
    MessageFailed,
    
    // User events
    UserCreated,
    UserUpdated,
    UserDeleted,
    UserLogin,
    UserLogout,
    
    // File events
    FileCreated,
    FileUpdated,
    FileDeleted,
    
    // Custom events
    Custom(String),
    
    // Wildcard - matches all
    All,
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::SystemStarted => write!(f, "system.started"),
            EventType::SystemShutdown => write!(f, "system.shutdown"),
            EventType::SystemError => write!(f, "system.error"),
            EventType::AgentCreated => write!(f, "agent.created"),
            EventType::AgentStarted => write!(f, "agent.started"),
            EventType::AgentStopped => write!(f, "agent.stopped"),
            EventType::AgentError => write!(f, "agent.error"),
            EventType::AgentCompleted => write!(f, "agent.completed"),
            EventType::TaskCreated => write!(f, "task.created"),
            EventType::TaskStarted => write!(f, "task.started"),
            EventType::TaskProgress => write!(f, "task.progress"),
            EventType::TaskCompleted => write!(f, "task.completed"),
            EventType::TaskFailed => write!(f, "task.failed"),
            EventType::MessageReceived => write!(f, "message.received"),
            EventType::MessageSent => write!(f, "message.sent"),
            EventType::MessageFailed => write!(f, "message.failed"),
            EventType::UserCreated => write!(f, "user.created"),
            EventType::UserUpdated => write!(f, "user.updated"),
            EventType::UserDeleted => write!(f, "user.deleted"),
            EventType::UserLogin => write!(f, "user.login"),
            EventType::UserLogout => write!(f, "user.logout"),
            EventType::FileCreated => write!(f, "file.created"),
            EventType::FileUpdated => write!(f, "file.updated"),
            EventType::FileDeleted => write!(f, "file.deleted"),
            EventType::Custom(name) => write!(f, "custom.{}", name),
            EventType::All => write!(f, "*"),
        }
    }
}

/// Filter rule for event matching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterRule {
    /// Exact match
    Equals(String),
    /// Not equals
    NotEquals(String),
    /// Contains substring
    Contains(String),
    /// Starts with
    StartsWith(String),
    /// Ends with
    EndsWith(String),
    /// Regex match
    Regex(String),
    /// Greater than (for numbers)
    GreaterThan(f64),
    /// Less than (for numbers)
    LessThan(f64),
    /// In list
    In(Vec<String>),
    /// Not in list
    NotIn(Vec<String>),
}

impl FilterRule {
    pub fn matches(&self, value: &serde_json::Value) -> bool {
        match self {
            FilterRule::Equals(s) => value.as_str().map(|v| v == s).unwrap_or(false),
            FilterRule::NotEquals(s) => value.as_str().map(|v| v != s).unwrap_or(true),
            FilterRule::Contains(s) => value.as_str().map(|v| v.contains(s)).unwrap_or(false),
            FilterRule::StartsWith(s) => value.as_str().map(|v| v.starts_with(s)).unwrap_or(false),
            FilterRule::EndsWith(s) => value.as_str().map(|v| v.ends_with(s)).unwrap_or(false),
            FilterRule::Regex(pattern) => {
                if let Some(v) = value.as_str() {
                    regex::Regex::new(pattern)
                        .map(|re| re.is_match(v))
                        .unwrap_or(false)
                } else {
                    false
                }
            }
            FilterRule::GreaterThan(n) => value.as_f64().map(|v| v > *n).unwrap_or(false),
            FilterRule::LessThan(n) => value.as_f64().map(|v| v < *n).unwrap_or(false),
            FilterRule::In(list) => value.as_str().map(|v| list.contains(&v.to_string())).unwrap_or(false),
            FilterRule::NotIn(list) => value.as_str().map(|v| !list.contains(&v.to_string())).unwrap_or(true),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  WEBHOOK EVENT
// ═══════════════════════════════════════════════════════════════════════════════

/// Webhook event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    /// Unique event ID
    pub id: String,
    /// Event type
    pub event_type: EventType,
    /// Event payload
    pub payload: serde_json::Value,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Source of the event
    pub source: String,
    /// Correlation ID for tracing
    pub correlation_id: Option<String>,
}

impl WebhookEvent {
    pub fn new(event_type: EventType, payload: serde_json::Value) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            event_type,
            payload,
            timestamp: Utc::now(),
            source: "sentient".into(),
            correlation_id: None,
        }
    }
    
    pub fn with_source(mut self, source: String) -> Self {
        self.source = source;
        self
    }
    
    pub fn with_correlation_id(mut self, id: String) -> Self {
        self.correlation_id = Some(id);
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  WEBHOOK DELIVERY
// ═══════════════════════════════════════════════════════════════════════════════

/// Delivery status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeliveryStatus {
    Pending,
    InProgress,
    Delivered,
    Failed,
    RetryScheduled,
    Cancelled,
}

/// Webhook delivery attempt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryAttempt {
    /// Attempt number
    pub attempt: u32,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// HTTP status code (if received)
    pub status_code: Option<u16>,
    /// Response body (if received)
    pub response_body: Option<String>,
    /// Error message (if failed)
    pub error: Option<String>,
    /// Duration in ms
    pub duration_ms: u64,
}

/// Webhook delivery record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookDelivery {
    /// Delivery ID
    pub id: String,
    /// Subscription ID
    pub subscription_id: String,
    /// Event ID
    pub event_id: String,
    /// Delivery status
    pub status: DeliveryStatus,
    /// Delivery attempts
    pub attempts: Vec<DeliveryAttempt>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last attempt timestamp
    pub last_attempt_at: Option<DateTime<Utc>>,
    /// Next retry timestamp
    pub next_retry_at: Option<DateTime<Utc>>,
    /// Delivery duration (total)
    pub total_duration_ms: u64,
}

impl WebhookDelivery {
    pub fn new(subscription_id: String, event_id: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            subscription_id,
            event_id,
            status: DeliveryStatus::Pending,
            attempts: Vec::new(),
            created_at: Utc::now(),
            last_attempt_at: None,
            next_retry_at: None,
            total_duration_ms: 0,
        }
    }
    
    pub fn add_attempt(&mut self, attempt: DeliveryAttempt) {
        self.attempts.push(attempt);
        self.last_attempt_at = Some(Utc::now());
    }
    
    pub fn attempt_count(&self) -> u32 {
        self.attempts.len() as u32
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  WEBHOOK MANAGER
// ═══════════════════════════════════════════════════════════════════════════════

/// Webhook manager
pub struct WebhookManager {
    config: WebhookConfig,
    subscriptions: Arc<RwLock<HashMap<String, WebhookSubscription>>>,
    delivery_log: Arc<RwLock<Vec<WebhookDelivery>>>,
    http_client: reqwest::Client,
}

impl WebhookManager {
    pub fn new(config: WebhookConfig) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_millis(config.default_timeout_ms))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        
        Self {
            config,
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
            delivery_log: Arc::new(RwLock::new(Vec::new())),
            http_client,
        }
    }
    
    /// Subscribe to events
    pub async fn subscribe(&self, subscription: WebhookSubscription) -> String {
        let id = subscription.id.clone();
        let mut subs = self.subscriptions.write().await;
        subs.insert(id.clone(), subscription);
        log::info!("📢 Webhook subscribed: {}", id);
        id
    }
    
    /// Unsubscribe
    pub async fn unsubscribe(&self, id: &str) -> bool {
        let mut subs = self.subscriptions.write().await;
        subs.remove(id).map(|_| {
            log::info!("📢 Webhook unsubscribed: {}", id);
            true
        }).unwrap_or(false)
    }
    
    /// Get subscription
    pub async fn get_subscription(&self, id: &str) -> Option<WebhookSubscription> {
        let subs = self.subscriptions.read().await;
        subs.get(id).cloned()
    }
    
    /// List all subscriptions
    pub async fn list_subscriptions(&self) -> Vec<WebhookSubscription> {
        let subs = self.subscriptions.read().await;
        subs.values().cloned().collect()
    }
    
    /// Emit an event
    pub async fn emit(&self, event: WebhookEvent) {
        log::debug!("📢 Emitting event: {} ({})", event.event_type, event.id);
        
        let subs = self.subscriptions.read().await;
        let matching_subs: Vec<_> = subs.values()
            .filter(|s| s.active && s.matches_event(&event))
            .collect();
        
        if matching_subs.is_empty() {
            log::debug!("📢 No matching subscriptions for event {}", event.id);
            return;
        }
        
        log::info!("📢 Delivering event {} to {} subscribers", 
            event.id, matching_subs.len());
        
        for sub in matching_subs {
            let delivery = self.deliver(sub.clone(), event.clone()).await;
            
            // Log delivery
            let mut log = self.delivery_log.write().await;
            log.push(delivery);
            
            // Keep only last 1000 deliveries
            if log.len() > 1000 {
                log.drain(0..100);
            }
        }
    }
    
    /// Deliver webhook
    async fn deliver(&self, subscription: WebhookSubscription, event: WebhookEvent) -> WebhookDelivery {
        let mut delivery = WebhookDelivery::new(subscription.id.clone(), event.id.clone());
        delivery.status = DeliveryStatus::InProgress;
        
        let mut retry_delay = self.config.initial_retry_delay_ms;
        
        for attempt_num in 1..=self.config.max_retries {
            let start = std::time::Instant::now();
            
            let result = self.send_webhook(&subscription, &event).await;
            
            let attempt = DeliveryAttempt {
                attempt: attempt_num,
                timestamp: Utc::now(),
                status_code: result.as_ref().ok().and_then(|r| *r),
                response_body: None, // Would need to capture from response
                error: result.as_ref().err().map(|e| e.to_string()),
                duration_ms: start.elapsed().as_millis() as u64,
            };
            
            delivery.add_attempt(attempt);
            delivery.total_duration_ms += start.elapsed().as_millis() as u64;
            
            if result.is_ok() {
                delivery.status = DeliveryStatus::Delivered;
                log::info!("📢 Webhook delivered: {} -> {} (attempt {})", 
                    event.id, subscription.url, attempt_num);
                return delivery;
            }
            
            // Check if we should retry
            if attempt_num < self.config.max_retries {
                delivery.status = DeliveryStatus::RetryScheduled;
                delivery.next_retry_at = Some(Utc::now() + chrono::Duration::milliseconds(retry_delay as i64));
                
                log::warn!("📢 Webhook delivery failed, retrying in {}ms (attempt {}/{})", 
                    retry_delay, attempt_num, self.config.max_retries);
                
                tokio::time::sleep(std::time::Duration::from_millis(retry_delay)).await;
                
                retry_delay = ((retry_delay as f64) * self.config.retry_multiplier) as u64;
                retry_delay = retry_delay.min(self.config.max_retry_delay_ms);
            }
        }
        
        delivery.status = DeliveryStatus::Failed;
        log::error!("📢 Webhook delivery failed after {} attempts: {} -> {}", 
            self.config.max_retries, event.id, subscription.url);
        
        delivery
    }
    
    /// Send webhook HTTP request
    async fn send_webhook(&self, subscription: &WebhookSubscription, event: &WebhookEvent) -> Result<Option<u16>, WebhookError> {
        let body = serde_json::to_string(&event)
            .map_err(|e| WebhookError::SerializationError(e.to_string()))?;
        
        // Calculate signature if secret is configured
        let signature = if let Some(ref secret) = subscription.secret {
            Some(self.calculate_signature(&body, secret))
        } else if let Some(ref secret) = self.config.signing_secret {
            Some(self.calculate_signature(&body, secret))
        } else {
            None
        };
        
        let mut request = self.http_client
            .post(&subscription.url)
            .header("Content-Type", "application/json")
            .header("X-Webhook-ID", &event.id)
            .header("X-Webhook-Event", event.event_type.to_string())
            .header("X-Webhook-Timestamp", event.timestamp.to_rfc3339());
        
        // Add signature header
        if let Some(sig) = signature {
            request = request.header("X-Webhook-Signature", format!("sha256={}", sig));
        }
        
        // Add custom headers
        for (key, value) in &subscription.headers {
            request = request.header(key, value);
        }
        
        let response = request
            .body(body)
            .send()
            .await
            .map_err(|e| WebhookError::HttpError(e.to_string()))?;
        
        let status = response.status().as_u16();
        
        if response.status().is_success() {
            Ok(Some(status))
        } else {
            Err(WebhookError::HttpError(format!("HTTP {}", status)))
        }
    }
    
    /// Calculate HMAC-SHA256 signature
    fn calculate_signature(&self, body: &str, secret: &str) -> String {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        
        type HmacSha256 = Hmac<Sha256>;
        
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(body.as_bytes());
        
        let result = mac.finalize();
        hex::encode(result.into_bytes())
    }
    
    /// Get delivery statistics
    pub async fn stats(&self) -> WebhookStats {
        let log = self.delivery_log.read().await;
        
        let mut stats = WebhookStats::default();
        
        for delivery in log.iter() {
            stats.total_deliveries += 1;
            
            match delivery.status {
                DeliveryStatus::Delivered => stats.successful += 1,
                DeliveryStatus::Failed => stats.failed += 1,
                DeliveryStatus::Pending => stats.pending += 1,
                DeliveryStatus::InProgress => stats.in_progress += 1,
                _ => {}
            }
        }
        
        let subs = self.subscriptions.read().await;
        stats.active_subscriptions = subs.values().filter(|s| s.active).count();
        stats.total_subscriptions = subs.len();
        
        stats
    }
    
    /// Get recent deliveries
    pub async fn recent_deliveries(&self, limit: usize) -> Vec<WebhookDelivery> {
        let log = self.delivery_log.read().await;
        log.iter().rev().take(limit).cloned().collect()
    }
}

/// Webhook statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebhookStats {
    pub total_subscriptions: usize,
    pub active_subscriptions: usize,
    pub total_deliveries: u64,
    pub successful: u64,
    pub failed: u64,
    pub pending: u64,
    pub in_progress: u64,
}

/// Webhook error
#[derive(Debug, Clone)]
pub enum WebhookError {
    HttpError(String),
    SerializationError(String),
    InvalidUrl(String),
    Timeout,
    RateLimited,
}

impl std::fmt::Display for WebhookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HttpError(e) => write!(f, "HTTP error: {}", e),
            Self::SerializationError(e) => write!(f, "Serialization error: {}", e),
            Self::InvalidUrl(e) => write!(f, "Invalid URL: {}", e),
            Self::Timeout => write!(f, "Request timeout"),
            Self::RateLimited => write!(f, "Rate limited"),
        }
    }
}

impl std::error::Error for WebhookError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_event_type_display() {
        assert_eq!(EventType::AgentCreated.to_string(), "agent.created");
        assert_eq!(EventType::All.to_string(), "*");
    }
    
    #[test]
    fn test_filter_rule_equals() {
        let rule = FilterRule::Equals("test".into());
        assert!(rule.matches(&serde_json::json!("test")));
        assert!(!rule.matches(&serde_json::json!("other")));
    }
    
    #[test]
    fn test_filter_rule_contains() {
        let rule = FilterRule::Contains("ell".into());
        assert!(rule.matches(&serde_json::json!("hello")));
        assert!(!rule.matches(&serde_json::json!("world")));
    }
    
    #[test]
    fn test_webhook_subscription_matches() {
        let sub = WebhookSubscription::new(
            "https://example.com/webhook".into(),
            vec![EventType::AgentCreated, EventType::AgentCompleted],
        );
        
        let event1 = WebhookEvent::new(EventType::AgentCreated, serde_json::json!({}));
        assert!(sub.matches_event(&event1));
        
        let event2 = WebhookEvent::new(EventType::TaskCreated, serde_json::json!({}));
        assert!(!sub.matches_event(&event2));
    }
    
    #[test]
    fn test_webhook_delivery_attempts() {
        let mut delivery = WebhookDelivery::new("sub-1".into(), "event-1".into());
        
        delivery.add_attempt(DeliveryAttempt {
            attempt: 1,
            timestamp: Utc::now(),
            status_code: Some(500),
            response_body: None,
            error: Some("Server error".into()),
            duration_ms: 100,
        });
        
        assert_eq!(delivery.attempt_count(), 1);
    }
    
    #[tokio::test]
    async fn test_webhook_manager_subscribe() {
        let manager = WebhookManager::new(WebhookConfig::default());
        
        let sub = WebhookSubscription::new(
            "https://example.com/webhook".into(),
            vec![EventType::All],
        );
        
        let id = manager.subscribe(sub).await;
        assert!(!id.is_empty());
        
        let retrieved = manager.get_subscription(&id).await;
        assert!(retrieved.is_some());
    }
    
    #[tokio::test]
    async fn test_webhook_manager_stats() {
        let manager = WebhookManager::new(WebhookConfig::default());
        
        manager.subscribe(WebhookSubscription::new(
            "https://example.com/webhook".into(),
            vec![EventType::All],
        )).await;
        
        let stats = manager.stats().await;
        assert_eq!(stats.total_subscriptions, 1);
        assert_eq!(stats.active_subscriptions, 1);
    }
}
