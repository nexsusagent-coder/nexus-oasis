//! ─── RATE LIMITING MIDDLEWARE ───
//!
//! SENTIENT Gateway için rate limiting implementasyonu.
//! OpenClaw, Cursor ve diğer rakiplerle parite için kritik.
//!
//! Özellikler:
//! - IP bazlı rate limiting
//! - Token bucket algoritması
//! - Configurable limits

use axum::{
    body::Body,
    http::{Request, Response, StatusCode, HeaderMap, header},
    middleware::Next,
    response::IntoResponse,
};
use std::{
    sync::Arc,
    time::{Duration, Instant},
    collections::HashMap,
};
use std::net::IpAddr;
use std::str::FromStr;
use parking_lot::RwLock;
use once_cell::sync::Lazy;

/// Token bucket for rate limiting
#[derive(Debug, Clone)]
struct TokenBucket {
    tokens: f64,
    max_tokens: f64,
    refill_rate: f64, // tokens per second
    last_refill: Instant,
}

impl TokenBucket {
    fn new(max_tokens: u32, refill_per_minute: u32) -> Self {
        Self {
            tokens: max_tokens as f64,
            max_tokens: max_tokens as f64,
            refill_rate: refill_per_minute as f64 / 60.0,
            last_refill: Instant::now(),
        }
    }
    
    fn try_consume(&mut self, tokens: f64) -> bool {
        self.refill();
        if self.tokens >= tokens {
            self.tokens -= tokens;
            true
        } else {
            false
        }
    }
    
    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = (now - self.last_refill).as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.max_tokens);
        self.last_refill = now;
    }
    
    fn remaining(&self) -> u32 {
        self.tokens as u32
    }
}

/// IP bazlı rate limiter state
type IpBuckets = Arc<RwLock<HashMap<IpAddr, TokenBucket>>>;

/// Global rate limiter bucket
static GLOBAL_BUCKET: Lazy<Arc<RwLock<TokenBucket>>> = Lazy::new(|| {
    Arc::new(RwLock::new(TokenBucket::new(60, 60))) // 60 req/min
});

/// IP bazlı rate limiter'lar
static IP_BUCKETS: Lazy<IpBuckets> = Lazy::new(|| {
    Arc::new(RwLock::new(HashMap::new()))
});

/// Rate limit konfigürasyonu
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Dakikada maksimum istek (global)
    pub requests_per_minute: u32,
    /// Saatte maksimum istek (per IP)
    pub requests_per_hour: u32,
    /// Burst limiti
    pub burst: u32,
    /// Aktif mi?
    pub enabled: bool,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            requests_per_hour: 1000,
            burst: 10,
            enabled: true,
        }
    }
}

impl RateLimitConfig {
    /// Environment variable'lardan konfigürasyon okuma
    pub fn from_env() -> Self {
        Self {
            requests_per_minute: std::env::var("RATE_LIMIT_REQUESTS_PER_MINUTE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(60),
            requests_per_hour: std::env::var("RATE_LIMIT_REQUESTS_PER_HOUR")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1000),
            burst: std::env::var("RATE_LIMIT_BURST")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
            enabled: true,
        }
    }
}

/// Rate limit hatası
#[derive(Debug)]
pub struct RateLimitError {
    pub retry_after: Duration,
    pub limit: u32,
    pub remaining: u32,
}

impl IntoResponse for RateLimitError {
    fn into_response(self) -> Response<Body> {
        let body = serde_json::json!({
            "error": "rate_limit_exceeded",
            "message": "Too many requests. Please try again later.",
            "retry_after_seconds": self.retry_after.as_secs()
        });
        
        let mut response = Response::new(Body::from(body.to_string()));
        let status = StatusCode::TOO_MANY_REQUESTS;
        *response.status_mut() = status;
        
        let headers = response.headers_mut();
        headers.insert(
            "X-RateLimit-Limit",
            self.limit.to_string().parse().unwrap_or_else(|_| "60".parse().expect("operation failed")),
        );
        headers.insert(
            "X-RateLimit-Remaining",
            self.remaining.to_string().parse().unwrap_or_else(|_| "0".parse().expect("operation failed")),
        );
        headers.insert(
            "Retry-After",
            self.retry_after.as_secs().to_string().parse().unwrap_or_else(|_| "60".parse().expect("operation failed")),
        );
        headers.insert(
            header::CONTENT_TYPE,
            "application/json".parse().expect("operation failed"),
        );
        
        response
    }
}

/// İstekten IP adresi çıkarma
fn extract_ip(headers: &HeaderMap) -> Option<IpAddr> {
    // Önce X-Forwarded-For kontrol et (proxy arkasında)
    if let Some(forwarded) = headers.get("X-Forwarded-For") {
        if let Ok(forwarded_str) = forwarded.to_str() {
            if let Some(first_ip) = forwarded_str.split(',').next() {
                if let Ok(ip) = IpAddr::from_str(first_ip.trim()) {
                    return Some(ip);
                }
            }
        }
    }
    
    // X-Real-IP kontrol et
    if let Some(real_ip) = headers.get("X-Real-IP") {
        if let Ok(ip_str) = real_ip.to_str() {
            if let Ok(ip) = IpAddr::from_str(ip_str) {
                return Some(ip);
            }
        }
    }
    
    // Direct connection (localhost fallback)
    Some(IpAddr::from_str("127.0.0.1").unwrap_or(IpAddr::from([127, 0, 0, 1])))
}

/// IP için bucket al veya oluştur
fn get_or_create_ip_bucket(ip: IpAddr, config: &RateLimitConfig) -> TokenBucket {
    let buckets = IP_BUCKETS.read();
    if let Some(bucket) = buckets.get(&ip) {
        return bucket.clone();
    }
    drop(buckets);
    
    let bucket = TokenBucket::new(config.burst, config.requests_per_minute);
    
    let mut buckets = IP_BUCKETS.write();
    buckets.insert(ip, bucket.clone());
    
    bucket
}

/// Rate limiting middleware
pub async fn rate_limit_middleware(
    request: Request<Body>,
    next: Next,
) -> Result<Response<Body>, RateLimitError> {
    let config = RateLimitConfig::from_env();
    
    if !config.enabled {
        return Ok(next.run(request).await);
    }
    
    // Global rate limit kontrolü
    {
        let mut global = GLOBAL_BUCKET.write();
        if !global.try_consume(1.0) {
            tracing::warn!("Global rate limit exceeded");
            return Err(RateLimitError {
                retry_after: Duration::from_secs(60),
                limit: config.requests_per_minute,
                remaining: 0,
            });
        }
    }
    
    // IP bazlı rate limit
    if let Some(ip) = extract_ip(request.headers()) {
        let mut bucket = get_or_create_ip_bucket(ip, &config);
        if !bucket.try_consume(1.0) {
            tracing::warn!("Rate limit exceeded for IP: {}", ip);
            return Err(RateLimitError {
                retry_after: Duration::from_secs(60),
                limit: config.requests_per_minute,
                remaining: bucket.remaining(),
            });
        }
        
        // Update bucket
        let mut buckets = IP_BUCKETS.write();
        buckets.insert(ip, bucket);
    }
    
    // İsteği devam ettir
    let response = next.run(request).await;
    
    Ok(response)
}

/// Endpoint bazlı rate limiter oluşturucu
pub fn create_bucket(max_tokens: u32, refill_per_minute: u32) -> Arc<RwLock<TokenBucket>> {
    Arc::new(RwLock::new(TokenBucket::new(max_tokens, refill_per_minute)))
}

/// Auth endpoint'leri için sıkı limiter (10 req/min)
pub static AUTH_BUCKET: Lazy<Arc<RwLock<TokenBucket>>> = Lazy::new(|| {
    create_bucket(5, 10)
});

/// API endpoint'leri için standart limiter (60 req/min)
pub static API_BUCKET: Lazy<Arc<RwLock<TokenBucket>>> = Lazy::new(|| {
    create_bucket(20, 60)
});

/// WebSocket endpoint'leri için gevşek limiter (120 req/min)
pub static WS_BUCKET: Lazy<Arc<RwLock<TokenBucket>>> = Lazy::new(|| {
    create_bucket(30, 120)
});

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rate_limit_config() {
        let config = RateLimitConfig::default();
        assert_eq!(config.requests_per_minute, 60);
        assert_eq!(config.burst, 10);
    }
    
    #[test]
    fn test_token_bucket() {
        let mut bucket = TokenBucket::new(10, 60);
        
        // İlk istekler geçmeli
        assert!(bucket.try_consume(1.0));
        assert!(bucket.try_consume(1.0));
        
        // Limit'e kadar
        for _ in 0..8 {
            assert!(bucket.try_consume(1.0));
        }
        
        // Limit aşıldı
        assert!(!bucket.try_consume(1.0));
    }
    
    #[test]
    fn test_ip_extraction() {
        let mut headers = HeaderMap::new();
        headers.insert("X-Forwarded-For", "192.168.1.1, 10.0.0.1".parse().expect("operation failed"));
        
        let ip = extract_ip(&headers);
        assert!(ip.is_some());
        assert_eq!(ip.expect("operation failed").to_string(), "192.168.1.1");
    }
}