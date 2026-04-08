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
pub struct DistributedRateLimiter {
    // TODO: Governor implementasyonu
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
