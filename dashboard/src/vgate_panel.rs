//! ═══════════════════════════════════════════════════════════════════════════════
//!  V-GATE PANEL - Bağlantı Durumu
//! ═══════════════════════════════════════════════════════════════════════════════

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// V-GATE bağlantı durumu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VGateStatus {
    pub connected: bool,
    pub url: String,
    pub last_ping: Option<DateTime<Utc>>,
    pub latency_ms: Option<u64>,
    pub requests_today: u64,
    pub rate_limit_remaining: u64,
}

/// V-GATE Panel
pub struct VGatePanel {
    status: VGateStatus,
}

impl VGatePanel {
    pub fn new(url: &str) -> Self {
        Self {
            status: VGateStatus {
                connected: false,
                url: url.to_string(),
                last_ping: None,
                latency_ms: None,
                requests_today: 0,
                rate_limit_remaining: 1000,
            },
        }
    }
    
    /// Bağlantı durumunu güncelle
    pub fn update_connection(&mut self, connected: bool, latency_ms: u64) {
        self.status.connected = connected;
        self.status.last_ping = Some(Utc::now());
        self.status.latency_ms = Some(latency_ms);
    }
    
    /// İstek sayısını artır
    pub fn increment_requests(&mut self) {
        self.status.requests_today += 1;
        if self.status.rate_limit_remaining > 0 {
            self.status.rate_limit_remaining -= 1;
        }
    }
    
    /// Durumu getir
    pub fn get_status(&self) -> &VGateStatus {
        &self.status
    }
    
    /// Ping at
    pub async fn ping(&mut self) -> Result<u64, String> {
        let start = std::time::Instant::now();
        
        // Basit HTTP ping
        let client = reqwest::Client::new();
        match client.get(&format!("{}/health", self.status.url))
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
        {
            Ok(resp) => {
                if resp.status().is_success() {
                    let latency = start.elapsed().as_millis() as u64;
                    self.update_connection(true, latency);
                    Ok(latency)
                } else {
                    self.status.connected = false;
                    Err(format!("V-GATE health check failed: {}", resp.status()))
                }
            }
            Err(e) => {
                self.status.connected = false;
                Err(format!("V-GATE bağlantı hatası: {}", e))
            }
        }
    }
}

impl Default for VGatePanel {
    fn default() -> Self {
        Self::new("http://localhost:8080")
    }
}
