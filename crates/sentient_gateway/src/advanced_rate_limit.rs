//! ═══════════════════════════════════════════════════════════════════════════════
//!  ADVANCED RATE LIMITING - Enterprise Grade Implementation
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Gelişmiş rate limiting özellikleri:
//! - Sliding Window Algorithm
//! - Distributed Rate Limiting (Redis-backed)
//! - User-based Rate Limiting
//! - Endpoint-specific Configurations
//! - Admin Bypass
//! - Rate Limit Headers (RFC 6585)
//! - Circuit Breaker Integration

use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
    net::IpAddr,
};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock as AsyncRwLock;

// ═══════════════════════════════════════════════════════════════════════════════
//  SLIDING WINDOW RATE LIMITER
// ═══════════════════════════════════════════════════════════════════════════════

/// Sliding window rate limiter
/// 
/// Fixed window yerine sliding window kullanarak
/// daha doğru rate limiting sağlar.
pub struct SlidingWindowLimiter {
    /// Window size in seconds
    window_size: Duration,
    /// Maximum requests per window
    max_requests: u64,
    /// Request timestamps within current window
    requests: Arc<RwLock<Vec<u64>>>,
}

impl SlidingWindowLimiter {
    pub fn new(window_size: Duration, max_requests: u64) -> Self {
        Self {
            window_size,
            max_requests,
            requests: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Check if request is allowed
    pub fn check(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let window_start = now.saturating_sub(self.window_size.as_secs());
        
        let mut requests = self.requests.write();
        
        // Remove old requests outside window
        requests.retain(|&ts| ts > window_start);
        
        // Check if under limit
        if (requests.len() as u64) < self.max_requests {
            requests.push(now);
            true
        } else {
            false
        }
    }
    
    /// Get remaining requests in current window
    pub fn remaining(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let window_start = now.saturating_sub(self.window_size.as_secs());
        
        let requests = self.requests.read();
        let count = requests.iter().filter(|&&ts| ts > window_start).count();
        
        self.max_requests.saturating_sub(count as u64)
    }
    
    /// Get time until window resets
    pub fn reset_after(&self) -> Duration {
        let requests = self.requests.read();
        if let Some(&oldest) = requests.first() {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            
            let reset = oldest + self.window_size.as_secs();
            Duration::from_secs(reset.saturating_sub(now))
        } else {
            self.window_size
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DISTRIBUTED RATE LIMITER (Redis-like interface)
// ═══════════════════════════════════════════════════════════════════════════════

/// Distributed rate limiter interface
/// 
/// Redis veya başka bir distributed store ile
/// rate limiting yapabilmek için trait.
#[async_trait::async_trait]
pub trait DistributedRateStore: Send + Sync {
    /// Increment counter for key
    async fn increment(&self, key: &str, window_secs: u64) -> Result<u64, RateLimitError>;
    
    /// Get current count for key
    async fn get(&self, key: &str) -> Result<u64, RateLimitError>;
    
    /// Set TTL for key
    async fn set_ttl(&self, key: &str, secs: u64) -> Result<(), RateLimitError>;
}

/// In-memory distributed store (for single-node deployments)
pub struct InMemoryDistributedStore {
    counters: Arc<AsyncRwLock<HashMap<String, (u64, Instant)>>>,
}

impl InMemoryDistributedStore {
    pub fn new() -> Self {
        Self {
            counters: Arc::new(AsyncRwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryDistributedStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl DistributedRateStore for InMemoryDistributedStore {
    async fn increment(&self, key: &str, window_secs: u64) -> Result<u64, RateLimitError> {
        let mut counters = self.counters.write().await;
        let now = Instant::now();
        
        let (count, expires) = counters.entry(key.to_string()).or_insert((0, now + Duration::from_secs(window_secs)));
        
        // Check if expired
        if now > *expires {
            *count = 0;
            *expires = now + Duration::from_secs(window_secs);
        }
        
        *count += 1;
        Ok(*count)
    }
    
    async fn get(&self, key: &str) -> Result<u64, RateLimitError> {
        let counters = self.counters.read().await;
        Ok(counters.get(key).map(|(c, _)| *c).unwrap_or(0))
    }
    
    async fn set_ttl(&self, key: &str, secs: u64) -> Result<(), RateLimitError> {
        let mut counters = self.counters.write().await;
        if let Some((_, expires)) = counters.get_mut(key) {
            *expires = Instant::now() + Duration::from_secs(secs);
        }
        Ok(())
    }
}

/// Distributed rate limiter
pub struct DistributedRateLimiter<S: DistributedRateStore> {
    store: Arc<S>,
    config: RateLimitConfig,
}

impl<S: DistributedRateStore> DistributedRateLimiter<S> {
    pub fn new(store: Arc<S>, config: RateLimitConfig) -> Self {
        Self { store, config }
    }
    
    /// Check rate limit for key
    pub async fn check(&self, key: &str) -> RateLimitStatus {
        let count = self.store.increment(key, self.config.window_secs).await
            .unwrap_or(0);
        
        RateLimitStatus {
            allowed: count <= self.config.max_requests,
            limit: self.config.max_requests,
            remaining: self.config.max_requests.saturating_sub(count),
            reset_after: Duration::from_secs(self.config.window_secs),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  RATE LIMIT CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Rate limit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Maximum requests per window
    pub max_requests: u64,
    /// Window size in seconds
    pub window_secs: u64,
    /// Burst allowance
    pub burst: u64,
    /// Enable rate limiting
    pub enabled: bool,
    /// Admin bypass enabled
    pub admin_bypass: bool,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 60,
            window_secs: 60,
            burst: 10,
            enabled: true,
            admin_bypass: true,
        }
    }
}

/// Endpoint-specific rate limit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointRateLimits {
    /// Authentication endpoints (strict)
    pub auth: RateLimitConfig,
    /// API endpoints (standard)
    pub api: RateLimitConfig,
    /// WebSocket endpoints (relaxed)
    pub websocket: RateLimitConfig,
    /// Search endpoints (high volume)
    pub search: RateLimitConfig,
    /// LLM inference endpoints (expensive)
    pub llm: RateLimitConfig,
    /// Admin endpoints (bypass)
    pub admin: RateLimitConfig,
}

impl Default for EndpointRateLimits {
    fn default() -> Self {
        Self {
            auth: RateLimitConfig {
                max_requests: 10,
                window_secs: 60,
                burst: 3,
                enabled: true,
                admin_bypass: true,
            },
            api: RateLimitConfig {
                max_requests: 60,
                window_secs: 60,
                burst: 20,
                enabled: true,
                admin_bypass: true,
            },
            websocket: RateLimitConfig {
                max_requests: 120,
                window_secs: 60,
                burst: 30,
                enabled: true,
                admin_bypass: true,
            },
            search: RateLimitConfig {
                max_requests: 100,
                window_secs: 60,
                burst: 20,
                enabled: true,
                admin_bypass: true,
            },
            llm: RateLimitConfig {
                max_requests: 30,
                window_secs: 60,
                burst: 5,
                enabled: true,
                admin_bypass: true,
            },
            admin: RateLimitConfig {
                max_requests: 1000,
                window_secs: 60,
                burst: 100,
                enabled: false, // No rate limiting for admin
                admin_bypass: true,
            },
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  RATE LIMIT STATUS
// ═══════════════════════════════════════════════════════════════════════════════

/// Rate limit check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitStatus {
    /// Request is allowed
    pub allowed: bool,
    /// Maximum requests per window
    pub limit: u64,
    /// Remaining requests in current window
    pub remaining: u64,
    /// Time until window resets
    pub reset_after: Duration,
}

impl RateLimitStatus {
    pub fn headers(&self) -> Vec<(&'static str, String)> {
        vec![
            ("X-RateLimit-Limit", self.limit.to_string()),
            ("X-RateLimit-Remaining", self.remaining.to_string()),
            ("X-RateLimit-Reset", self.reset_after.as_secs().to_string()),
        ]
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  MULTI-KEY RATE LIMITER
// ═══════════════════════════════════════════════════════════════════════════════

/// Multi-key rate limiter with support for:
/// - IP-based limiting
/// - User-based limiting
/// - Endpoint-based limiting
/// - Global limiting
pub struct MultiKeyRateLimiter {
    /// IP-based limiters
    ip_limiters: Arc<RwLock<HashMap<IpAddr, SlidingWindowLimiter>>>,
    /// User-based limiters
    user_limiters: Arc<RwLock<HashMap<String, SlidingWindowLimiter>>>,
    /// Global limiter
    global_limiter: SlidingWindowLimiter,
    /// Configuration
    config: EndpointRateLimits,
    /// Admin user IDs (bypass rate limits)
    admin_users: Vec<String>,
}

impl MultiKeyRateLimiter {
    pub fn new(config: EndpointRateLimits) -> Self {
        Self {
            ip_limiters: Arc::new(RwLock::new(HashMap::new())),
            user_limiters: Arc::new(RwLock::new(HashMap::new())),
            global_limiter: SlidingWindowLimiter::new(Duration::from_secs(60), 10000),
            config,
            admin_users: Vec::new(),
        }
    }
    
    /// Add admin user (bypasses rate limits)
    pub fn add_admin(&mut self, user_id: impl Into<String>) {
        self.admin_users.push(user_id.into());
    }
    
    /// Check rate limit
    pub fn check(
        &self,
        ip: Option<IpAddr>,
        user_id: Option<&str>,
        endpoint_type: EndpointType,
    ) -> RateLimitStatus {
        let config = match endpoint_type {
            EndpointType::Auth => &self.config.auth,
            EndpointType::Api => &self.config.api,
            EndpointType::WebSocket => &self.config.websocket,
            EndpointType::Search => &self.config.search,
            EndpointType::Llm => &self.config.llm,
            EndpointType::Admin => &self.config.admin,
        };
        
        // Check if admin bypass applies
        if config.admin_bypass {
            if let Some(uid) = user_id {
                if self.admin_users.contains(&uid.to_string()) {
                    return RateLimitStatus {
                        allowed: true,
                        limit: config.max_requests,
                        remaining: config.max_requests,
                        reset_after: Duration::from_secs(config.window_secs),
                    };
                }
            }
        }
        
        // Check if rate limiting is enabled
        if !config.enabled {
            return RateLimitStatus {
                allowed: true,
                limit: config.max_requests,
                remaining: config.max_requests,
                reset_after: Duration::from_secs(config.window_secs),
            };
        }
        
        // Check global limit first
        if !self.global_limiter.check() {
            return RateLimitStatus {
                allowed: false,
                limit: 10000,
                remaining: 0,
                reset_after: self.global_limiter.reset_after(),
            };
        }
        
        // Check user-based limit (if authenticated)
        if let Some(uid) = user_id {
            let mut limiters = self.user_limiters.write();
            let limiter = limiters.entry(uid.to_string()).or_insert_with(|| {
                SlidingWindowLimiter::new(
                    Duration::from_secs(config.window_secs),
                    config.max_requests,
                )
            });
            
            if !limiter.check() {
                return RateLimitStatus {
                    allowed: false,
                    limit: config.max_requests,
                    remaining: limiter.remaining(),
                    reset_after: limiter.reset_after(),
                };
            }
            
            return RateLimitStatus {
                allowed: true,
                limit: config.max_requests,
                remaining: limiter.remaining(),
                reset_after: limiter.reset_after(),
            };
        }
        
        // Check IP-based limit (if not authenticated)
        if let Some(ip) = ip {
            let mut limiters = self.ip_limiters.write();
            let limiter = limiters.entry(ip).or_insert_with(|| {
                SlidingWindowLimiter::new(
                    Duration::from_secs(config.window_secs),
                    config.max_requests,
                )
            });
            
            if !limiter.check() {
                return RateLimitStatus {
                    allowed: false,
                    limit: config.max_requests,
                    remaining: limiter.remaining(),
                    reset_after: limiter.reset_after(),
                };
            }
            
            return RateLimitStatus {
                allowed: true,
                limit: config.max_requests,
                remaining: limiter.remaining(),
                reset_after: limiter.reset_after(),
            };
        }
        
        // No IP, no user - allow (shouldn't happen in production)
        RateLimitStatus {
            allowed: true,
            limit: config.max_requests,
            remaining: config.max_requests,
            reset_after: Duration::from_secs(config.window_secs),
        }
    }
}

/// Endpoint types for rate limiting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointType {
    Auth,
    Api,
    WebSocket,
    Search,
    Llm,
    Admin,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ERROR TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, thiserror::Error)]
pub enum RateLimitError {
    #[error("Rate limit exceeded. Retry after {0} seconds")]
    Exceeded(u64),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CIRCUIT BREAKER INTEGRATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Circuit breaker state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

/// Circuit breaker for protecting services
pub struct CircuitBreaker {
    state: Arc<RwLock<CircuitState>>,
    failure_count: Arc<RwLock<u64>>,
    success_count: Arc<RwLock<u64>>,
    last_failure: Arc<RwLock<Option<Instant>>>,
    config: CircuitBreakerConfig,
}

#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Failures before opening
    pub failure_threshold: u64,
    /// Successes before closing (from half-open)
    pub success_threshold: u64,
    /// Time before attempting half-open
    pub open_timeout: Duration,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 3,
            open_timeout: Duration::from_secs(30),
        }
    }
}

impl CircuitBreaker {
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            state: Arc::new(RwLock::new(CircuitState::Closed)),
            failure_count: Arc::new(RwLock::new(0)),
            success_count: Arc::new(RwLock::new(0)),
            last_failure: Arc::new(RwLock::new(None)),
            config,
        }
    }
    
    /// Check if requests are allowed
    pub fn is_allowed(&self) -> bool {
        let state = *self.state.read();
        
        match state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                // Check if timeout has passed
                let last = self.last_failure.read();
                if let Some(last_time) = *last {
                    if last_time.elapsed() > self.config.open_timeout {
                        // Transition to half-open
                        *self.state.write() = CircuitState::HalfOpen;
                        *self.success_count.write() = 0;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
            CircuitState::HalfOpen => true,
        }
    }
    
    /// Record a success
    pub fn record_success(&self) {
        let state = *self.state.read();
        
        match state {
            CircuitState::HalfOpen => {
                let mut successes = self.success_count.write();
                *successes += 1;
                
                if *successes >= self.config.success_threshold {
                    // Close circuit
                    *self.state.write() = CircuitState::Closed;
                    *self.failure_count.write() = 0;
                }
            },
            CircuitState::Closed => {
                *self.failure_count.write() = 0;
            },
            _ => {},
        }
    }
    
    /// Record a failure
    pub fn record_failure(&self) {
        let state = *self.state.read();
        
        match state {
            CircuitState::Closed => {
                let mut failures = self.failure_count.write();
                *failures += 1;
                
                if *failures >= self.config.failure_threshold {
                    // Open circuit
                    *self.state.write() = CircuitState::Open;
                    *self.last_failure.write() = Some(Instant::now());
                }
            },
            CircuitState::HalfOpen => {
                // Reopen circuit
                *self.state.write() = CircuitState::Open;
                *self.last_failure.write() = Some(Instant::now());
            },
            _ => {},
        }
    }
    
    /// Get current state
    pub fn state(&self) -> CircuitState {
        *self.state.read()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;
    use std::str::FromStr;
    
    #[test]
    fn test_sliding_window_limiter() {
        let limiter = SlidingWindowLimiter::new(Duration::from_secs(60), 5);
        
        // Should allow first 5 requests
        for _ in 0..5 {
            assert!(limiter.check());
        }
        
        // Should deny 6th
        assert!(!limiter.check());
        
        // Remaining should be 0
        assert_eq!(limiter.remaining(), 0);
    }
    
    #[tokio::test]
    async fn test_in_memory_distributed_store() {
        let store = InMemoryDistributedStore::new();
        
        let count = store.increment("test_key", 60).await.expect("increment failed");
        assert_eq!(count, 1);
        
        let count = store.increment("test_key", 60).await.expect("increment failed");
        assert_eq!(count, 2);
        
        let val = store.get("test_key").await.expect("get failed");
        assert_eq!(val, 2);
    }
    
    #[test]
    fn test_multi_key_rate_limiter() {
        let limiter = MultiKeyRateLimiter::new(EndpointRateLimits::default());
        
        let ip = IpAddr::from_str("192.168.1.1").expect("invalid IP");
        
        // Should allow requests
        let status = limiter.check(Some(ip), None, EndpointType::Api);
        assert!(status.allowed);
        
        // Exhaust limit
        for _ in 0..60 {
            limiter.check(Some(ip), None, EndpointType::Api);
        }
        
        // Should be denied now
        let status = limiter.check(Some(ip), None, EndpointType::Api);
        assert!(!status.allowed);
    }
    
    #[test]
    fn test_admin_bypass() {
        let mut limiter = MultiKeyRateLimiter::new(EndpointRateLimits::default());
        limiter.add_admin("admin_user_123");
        
        // Admin should always be allowed
        for _ in 0..100 {
            let status = limiter.check(None, Some("admin_user_123"), EndpointType::Api);
            assert!(status.allowed);
        }
    }
    
    #[test]
    fn test_circuit_breaker() {
        let breaker = CircuitBreaker::new(CircuitBreakerConfig::default());
        
        // Should start closed
        assert_eq!(breaker.state(), CircuitState::Closed);
        assert!(breaker.is_allowed());
        
        // Record failures to open circuit
        for _ in 0..5 {
            breaker.record_failure();
        }
        
        assert_eq!(breaker.state(), CircuitState::Open);
        assert!(!breaker.is_allowed());
    }
    
    #[test]
    fn test_endpoint_rate_limits_default() {
        let limits = EndpointRateLimits::default();
        
        // Auth should be strict
        assert_eq!(limits.auth.max_requests, 10);
        
        // API should be standard
        assert_eq!(limits.api.max_requests, 60);
        
        // LLM should be limited
        assert_eq!(limits.llm.max_requests, 30);
        
        // Admin should have no limit
        assert!(!limits.admin.enabled);
    }
    
    #[test]
    fn test_rate_limit_status_headers() {
        let status = RateLimitStatus {
            allowed: true,
            limit: 60,
            remaining: 45,
            reset_after: Duration::from_secs(30),
        };
        
        let headers = status.headers();
        assert_eq!(headers.len(), 3);
        assert_eq!(headers[0].0, "X-RateLimit-Limit");
        assert_eq!(headers[0].1, "60");
    }
}
