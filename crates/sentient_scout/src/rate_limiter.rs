//! ─── RATE LIMITER ───

use crate::Platform;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Rate limiter
pub struct RateLimiter {
    /// Platform bazli sinirlar
    limits: HashMap<Platform, RateLimit>,
    /// Istek gecmisi
    history: HashMap<Platform, Vec<Instant>>,
}

#[derive(Debug, Clone)]
pub struct RateLimit {
    /// Istek sayisi siniri
    pub max_requests: u32,
    /// Zaman penceresi
    pub window: Duration,
}

impl RateLimiter {
    pub fn new() -> Self {
        let mut limiter = Self {
            limits: HashMap::new(),
            history: HashMap::new(),
        };
        
        // Varsayilan sinirlari ayarla
        limiter.set_limit(Platform::Twitter, RateLimit { max_requests: 1, window: Duration::from_secs(1) });
        limiter.set_limit(Platform::Instagram, RateLimit { max_requests: 1, window: Duration::from_secs(1) });
        limiter.set_limit(Platform::LinkedIn, RateLimit { max_requests: 1, window: Duration::from_secs(2) });
        limiter.set_limit(Platform::GitHub, RateLimit { max_requests: 10, window: Duration::from_secs(1) });
        limiter.set_limit(Platform::Reddit, RateLimit { max_requests: 2, window: Duration::from_secs(1) });
        
        limiter
    }
    
    /// Limit ayarla
    pub fn set_limit(&mut self, platform: Platform, limit: RateLimit) {
        self.limits.insert(platform, limit);
    }
    
    /// Istek yapabilir mi?
    pub fn can_request(&self, platform: Platform) -> bool {
        let limit = match self.limits.get(&platform) {
            Some(l) => l,
            None => return true,
        };
        
        let history = match self.history.get(&platform) {
            Some(h) => h,
            None => return true,
        };
        
        let now = Instant::now();
        let cutoff = now - limit.window;
        
        // Son penceredeki istek sayisi
        let recent_count = history.iter().filter(|&&t| t > cutoff).count();
        
        recent_count < limit.max_requests as usize
    }
    
    /// Istek kaydet
    pub fn record_request(&mut self, platform: Platform) {
        let now = Instant::now();
        let history = self.history.entry(platform).or_default();
        
        history.push(now);
        
        // Eski kayitlari temizle
        let limit = self.limits.get(&platform);
        if let Some(limit) = limit {
            let cutoff = now - limit.window * 2;
            history.retain(|&t| t > cutoff);
        }
    }
    
    /// Bekleme suresi
    pub fn wait_duration(&self, platform: Platform) -> Option<Duration> {
        let limit = self.limits.get(&platform)?;
        let history = self.history.get(&platform)?;
        
        let now = Instant::now();
        let cutoff = now - limit.window;
        
        let recent: Vec<_> = history.iter().filter(|&&t| t > cutoff).collect();
        
        if recent.len() < limit.max_requests as usize {
            return None;
        }
        
        // En eski istek ne zaman bitiyor?
        let oldest = recent.iter().min()?;
        let oldest_age = now.duration_since(**oldest);
        Some(limit.window.saturating_sub(oldest_age))
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

/// Distributed rate limiter (Governor tabanli)
/// 
/// Uses the token bucket algorithm for smooth rate limiting.
/// Supports both direct (in-memory) and keyed (per-platform) rate limiting.
pub struct DistributedRateLimiter {
    /// Per-platform rate limiters
    limiters: HashMap<Platform, PlatformLimiter>,
    /// Global rate limit (requests per second)
    global_rps: u32,
    /// Burst capacity
    burst_size: u32,
}

/// Platform-specific limiter using token bucket
struct PlatformLimiter {
    /// Tokens available
    tokens: std::sync::atomic::AtomicU32,
    /// Max tokens (burst size)
    max_tokens: u32,
    /// Refill rate (tokens per second)
    refill_rate: u32,
    /// Last refill time
    last_refill: std::sync::Mutex<Instant>,
}

impl PlatformLimiter {
    fn new(max_tokens: u32, refill_rate: u32) -> Self {
        Self {
            tokens: std::sync::atomic::AtomicU32::new(max_tokens),
            max_tokens,
            refill_rate,
            last_refill: std::sync::Mutex::new(Instant::now()),
        }
    }

    fn try_acquire(&self) -> bool {
        self.refill();
        loop {
            let current = self.tokens.load(std::sync::atomic::Ordering::Relaxed);
            if current == 0 {
                return false;
            }
            if self.tokens.compare_exchange(
                current,
                current - 1,
                std::sync::atomic::Ordering::Relaxed,
                std::sync::atomic::Ordering::Relaxed,
            ).is_ok() {
                return true;
            }
        }
    }

    fn refill(&self) {
        let mut last = self.last_refill.lock().expect("operation failed");
        let now = Instant::now();
        let elapsed = now.duration_since(*last);
        
        // Refill tokens based on elapsed time
        let tokens_to_add = (elapsed.as_secs_f64() * self.refill_rate as f64) as u32;
        if tokens_to_add > 0 {
            let current = self.tokens.load(std::sync::atomic::Ordering::Relaxed);
            let new_tokens = (current + tokens_to_add).min(self.max_tokens);
            self.tokens.store(new_tokens, std::sync::atomic::Ordering::Relaxed);
            *last = now;
        }
    }

    fn available(&self) -> u32 {
        self.refill();
        self.tokens.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl DistributedRateLimiter {
    /// Create new distributed rate limiter
    pub fn new() -> Self {
        let mut limiters = HashMap::new();
        
        // Platform-specific limits
        limiters.insert(Platform::Twitter, PlatformLimiter::new(1, 1));      // 1 req/sec
        limiters.insert(Platform::Instagram, PlatformLimiter::new(1, 1));   // 1 req/sec
        limiters.insert(Platform::LinkedIn, PlatformLimiter::new(2, 1));   // 1 req/2sec
        limiters.insert(Platform::GitHub, PlatformLimiter::new(10, 10));   // 10 req/sec
        limiters.insert(Platform::Reddit, PlatformLimiter::new(2, 2));     // 2 req/sec
        limiters.insert(Platform::Facebook, PlatformLimiter::new(1, 1));   // 1 req/sec
        limiters.insert(Platform::TikTok, PlatformLimiter::new(1, 1));     // 1 req/sec
        
        Self {
            limiters,
            global_rps: 100,
            burst_size: 50,
        }
    }

    /// Try to acquire a permit for the platform
    pub fn try_acquire(&self, platform: Platform) -> bool {
        if let Some(limiter) = self.limiters.get(&platform) {
            limiter.try_acquire()
        } else {
            true // No limit for unknown platforms
        }
    }

    /// Get available permits for platform
    pub fn available(&self, platform: Platform) -> u32 {
        if let Some(limiter) = self.limiters.get(&platform) {
            limiter.available()
        } else {
            u32::MAX
        }
    }

    /// Wait until a permit is available
    pub async fn wait_for_permit(&self, platform: Platform) {
        while !self.try_acquire(platform) {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Set custom rate limit for a platform
    pub fn set_limit(&mut self, platform: Platform, burst: u32, rate: u32) {
        self.limiters.insert(platform, PlatformLimiter::new(burst, rate));
    }
}

impl Default for DistributedRateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rate_limiter_can_request() {
        let limiter = RateLimiter::new();
        assert!(limiter.can_request(Platform::GitHub));
    }
    
    #[test]
    fn test_rate_limiter_record() {
        let mut limiter = RateLimiter::new();
        limiter.record_request(Platform::GitHub);
        assert!(limiter.can_request(Platform::GitHub));
    }
}
