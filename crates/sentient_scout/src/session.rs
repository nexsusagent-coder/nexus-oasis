//! ─── SCOUT OTURUM ───

use crate::{Platform, config::StealthConfig};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

/// Scout oturumu
#[derive(Debug)]
pub struct ScoutSession {
    /// Oturum ID
    pub id: uuid::Uuid,
    /// Baslangic zamani
    pub started_at: Instant,
    /// Istek gecmisi
    request_history: VecDeque<RequestRecord>,
    /// Platform istatistikleri
    platform_stats: HashMap<Platform, PlatformStats>,
    /// Cookie deposu
    pub cookies: HashMap<String, String>,
    /// Header deposu
    pub headers: HashMap<String, String>,
    /// Gizlilik durumu
    stealth_state: StealthState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RequestRecord {
    platform: Platform,
    timestamp: String,  // ISO 8601 format
    success: bool,
    duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub last_request: Option<String>,  // ISO 8601
    pub avg_response_time_ms: f64,
}

#[derive(Debug, Clone)]
struct StealthState {
    fingerprint_noise: u64,
    timezone_offset: i32,
    screen_resolution: (u32, u32),
    viewport_size: (u32, u32),
}

impl Default for StealthState {
    fn default() -> Self {
        Self {
            fingerprint_noise: fastrand::u64(0..u64::MAX),
            timezone_offset: -180, // UTC+3
            screen_resolution: (1920, 1080),
            viewport_size: (1920, 969),
        }
    }
}

impl ScoutSession {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            started_at: Instant::now(),
            request_history: VecDeque::with_capacity(1000),
            platform_stats: HashMap::new(),
            cookies: HashMap::new(),
            headers: HashMap::new(),
            stealth_state: StealthState::default(),
        }
    }
    
    /// Anti-detection uygula
    pub fn apply_stealth(&mut self, config: &StealthConfig) {
        if config.canvas_noise {
            self.stealth_state.fingerprint_noise = fastrand::u64(0..u64::MAX);
        }
        
        // Rastgele viewport boyutu
        let width = 1200 + fastrand::u32(0..720);
        let height = 700 + fastrand::u32(0..380);
        self.stealth_state.viewport_size = (width, height);
    }
    
    /// Istek kaydet
    pub fn record_request(&mut self, platform: Platform, success_count: usize) {
        let now = chrono::Utc::now().to_rfc3339();
        
        // Istek gecmisine ekle
        self.request_history.push_back(RequestRecord {
            platform,
            timestamp: now.clone(),
            success: success_count > 0,
            duration_ms: 0,
        });
        
        // Platform istatistiklerini guncelle
        let stats = self.platform_stats.entry(platform).or_insert(PlatformStats {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            last_request: None,
            avg_response_time_ms: 0.0,
        });
        
        stats.total_requests += 1;
        if success_count > 0 {
            stats.successful_requests += 1;
        } else {
            stats.failed_requests += 1;
        }
        stats.last_request = Some(now);
        
        // Eski kayitlari temizle
        while self.request_history.len() > 1000 {
            self.request_history.pop_front();
        }
    }
    
    /// Toplam istek sayisi
    pub fn total_requests(&self) -> u64 {
        self.platform_stats.values().map(|s| s.total_requests).sum()
    }
    
    /// Basari orani
    pub fn success_rate(&self) -> f64 {
        let total: u64 = self.platform_stats.values().map(|s| s.total_requests).sum();
        let success: u64 = self.platform_stats.values().map(|s| s.successful_requests).sum();
        
        if total == 0 { return 100.0; }
        (success as f64 / total as f64) * 100.0
    }
    
    /// Aktif platformlar
    pub fn active_platforms(&self) -> Vec<Platform> {
        self.platform_stats
            .iter()
            .filter(|(_, s)| s.total_requests > 0)
            .map(|(&p, _)| p)
            .collect()
    }
    
    /// Rate limiter al
    pub fn rate_limiter(&self, platform: Platform) -> Option<&PlatformStats> {
        self.platform_stats.get(&platform)
    }
    
    /// Cookie ekle
    pub fn set_cookie(&mut self, name: &str, value: &str) {
        self.cookies.insert(name.into(), value.into());
    }
    
    /// Header ekle
    pub fn set_header(&mut self, name: &str, value: &str) {
        self.headers.insert(name.into(), value.into());
    }
}

impl Default for ScoutSession {
    fn default() -> Self {
        Self::new()
    }
}
