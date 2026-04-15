//! ─── Rate Limiter ───

use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, Instant};
use std::sync::Mutex;

/// Simple rate limiter using token bucket algorithm
pub struct RateLimiter {
    tokens: Mutex<RateLimiterState>,
    max_tokens: u32,
    refill_rate: Duration,
}

struct RateLimiterState {
    tokens: f64,
    last_refill: Instant,
}

impl RateLimiter {
    pub fn new(requests_per_second: u32) -> Self {
        Self {
            tokens: Mutex::new(RateLimiterState {
                tokens: requests_per_second as f64,
                last_refill: Instant::now(),
            }),
            max_tokens: requests_per_second,
            refill_rate: Duration::from_secs(1),
        }
    }
    
    /// Check if a request is allowed
    pub fn allow(&self) -> bool {
        let mut state = self.tokens.lock().unwrap();
        
        // Refill tokens
        let now = Instant::now();
        let elapsed = now.duration_since(state.last_refill);
        let tokens_to_add = elapsed.as_secs_f64() / self.refill_rate.as_secs_f64() * self.max_tokens as f64;
        
        state.tokens = (state.tokens + tokens_to_add).min(self.max_tokens as f64);
        state.last_refill = now;
        
        // Check if we have tokens
        if state.tokens >= 1.0 {
            state.tokens -= 1.0;
            true
        } else {
            false
        }
    }
    
    /// Get time until next available token
    pub fn time_until_available(&self) -> Duration {
        let state = self.tokens.lock().unwrap();
        
        if state.tokens >= 1.0 {
            Duration::ZERO
        } else {
            let needed = 1.0 - state.tokens;
            let secs = needed / self.max_tokens as f64;
            Duration::from_secs_f64(secs)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rate_limiter() {
        let limiter = RateLimiter::new(5);
        
        // Should allow first request
        assert!(limiter.allow());
    }
}
