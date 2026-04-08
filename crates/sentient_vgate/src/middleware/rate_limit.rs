//! ─── RATE LIMITING MIDDLEWARE ───
//!
//! İstekleri saniye/dakika bazında sınırlar.
//! Her IP için ayrı sayaç tutar.

use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// ─── Rate Limit Yapılandırması ───

#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Dakika başına maksimum istek
    pub requests_per_minute: u32,
    /// Saniye başına maksimum istek
    pub requests_per_second: u32,
    /// IP başına ayrı limit uygula
    pub per_ip: bool,
    /// Global limit uygula
    pub global_limit: bool,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            requests_per_second: 5,
            per_ip: true,
            global_limit: true,
        }
    }
}

/// ─── Rate Limit Sayaç ───

#[derive(Debug)]
struct RateCounter {
    request_times: Vec<Instant>,
}

impl RateCounter {
    fn new() -> Self {
        Self {
            request_times: Vec::new(),
        }
    }

    fn add_request(&mut self) {
        self.request_times.push(Instant::now());
        if self.request_times.len() > 100 {
            self.cleanup();
        }
    }

    fn cleanup(&mut self) {
        let now = Instant::now();
        let cutoff = now - Duration::from_secs(60);
        self.request_times.retain(|&t| t > cutoff);
    }

    fn count_last_second(&self) -> usize {
        let now = Instant::now();
        let cutoff = now - Duration::from_secs(1);
        self.request_times.iter().filter(|&&t| t > cutoff).count()
    }

    fn count_last_minute(&self) -> usize {
        let now = Instant::now();
        let cutoff = now - Duration::from_secs(60);
        self.request_times.iter().filter(|&&t| t > cutoff).count()
    }
}

/// ─── Rate Limiter ───

pub struct RateLimiter {
    config: RateLimitConfig,
    global_counter: RwLock<RateCounter>,
    ip_counters: RwLock<HashMap<String, RateCounter>>,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        log::info!("⏱️  RATE_LIMIT: Başlatıldı ({}/dk, {}/sn)", 
            config.requests_per_minute, config.requests_per_second);
        Self {
            config,
            global_counter: RwLock::new(RateCounter::new()),
            ip_counters: RwLock::new(HashMap::new()),
        }
    }

    /// İstek kontrolü - true dönerse izin verilir
    pub async fn check(&self, ip: Option<&str>) -> RateLimitResult {
        // Global limit kontrolü
        if self.config.global_limit {
            let global = self.global_counter.read().await;
            let global_per_sec = global.count_last_second();
            let global_per_min = global.count_last_minute();
            drop(global);

            if global_per_sec >= self.config.requests_per_second as usize {
                log::warn!("⏱️  RATE_LIMIT: Global saniye limiti aşıldı ({})", global_per_sec);
                return RateLimitResult::Denied {
                    reason: RateLimitReason::GlobalSecondLimit,
                    retry_after: 1,
                };
            }

            if global_per_min >= self.config.requests_per_minute as usize {
                log::warn!("⏱️  RATE_LIMIT: Global dakika limiti aşıldı ({})", global_per_min);
                return RateLimitResult::Denied {
                    reason: RateLimitReason::GlobalMinuteLimit,
                    retry_after: 60,
                };
            }
        }

        // IP bazlı limit kontrolü
        if self.config.per_ip {
            if let Some(ip_str) = ip {
                let ip = ip_str.to_string();
                let counters = self.ip_counters.read().await;
                let ip_count = counters.get(&ip).map(|c| c.count_last_second()).unwrap_or(0);
                drop(counters);

                if ip_count >= self.config.requests_per_second as usize {
                    log::warn!("⏱️  RATE_LIMIT: IP saniye limiti aşıldı ({}/{})", ip, ip_count);
                    return RateLimitResult::Denied {
                        reason: RateLimitReason::IpSecondLimit,
                        retry_after: 1,
                    };
                }
            }
        }

        RateLimitResult::Allowed
    }

    /// İstek kaydet
    pub async fn record(&self, ip: Option<&str>) {
        self.global_counter.write().await.add_request();

        if let Some(ip_str) = ip {
            let ip = ip_str.to_string();
            let mut counters = self.ip_counters.write().await;
            counters.entry(ip).or_insert_with(RateCounter::new).add_request();
        }
    }

    /// İstatistikler
    pub async fn stats(&self) -> RateLimitStats {
        let global = self.global_counter.read().await;
        let global_per_sec = global.count_last_second();
        let global_per_min = global.count_last_minute();
        drop(global);

        let ip_count = self.ip_counters.read().await.len();

        RateLimitStats {
            global_requests_per_second: global_per_sec as u32,
            global_requests_per_minute: global_per_min as u32,
            active_ips: ip_count as u32,
        }
    }
}

#[derive(Debug)]
pub enum RateLimitResult {
    Allowed,
    Denied { reason: RateLimitReason, retry_after: u64 },
}

#[derive(Debug)]
pub enum RateLimitReason {
    GlobalSecondLimit,
    GlobalMinuteLimit,
    IpSecondLimit,
}

#[derive(Debug, Clone)]
pub struct RateLimitStats {
    pub global_requests_per_second: u32,
    pub global_requests_per_minute: u32,
    pub active_ips: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter() {
        let config = RateLimitConfig {
            requests_per_minute: 10,
            requests_per_second: 3,
            per_ip: false,
            global_limit: true,
        };
        let limiter = RateLimiter::new(config);

        for _ in 0..3 {
            assert!(matches!(limiter.check(None).await, RateLimitResult::Allowed));
            limiter.record(None).await;
        }
    }
}
