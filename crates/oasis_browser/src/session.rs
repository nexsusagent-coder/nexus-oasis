//! ═══════════════════════════════════════════════════════════════════════════════
//!  BROWSER SESSION - Oturum Yönetimi
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::agent::AgentConfig;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Browser oturumu
pub struct BrowserSession {
    /// Oturum ID
    pub id: String,
    /// Başlangıç zamanı
    pub started_at: DateTime<Utc>,
    /// Yapılandırma
    config: SessionConfig,
    /// İstatistikler
    stats: SessionStats,
    /// Aktif mi?
    active: bool,
}

/// Oturum yapılandırması
#[derive(Debug, Clone)]
pub struct SessionConfig {
    /// Headless mod
    pub headless: bool,
    /// User-Agent
    pub user_agent: String,
    /// Sayfa timeout (ms)
    pub page_timeout_ms: u64,
    /// Stealth mod
    pub stealth_mode: bool,
}

impl From<AgentConfig> for SessionConfig {
    fn from(config: AgentConfig) -> Self {
        Self {
            headless: config.headless,
            user_agent: config.user_agent,
            page_timeout_ms: config.page_timeout_ms,
            stealth_mode: config.stealth_mode,
        }
    }
}

/// Oturum istatistikleri
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionStats {
    /// Toplam istek sayısı
    pub total_requests: u64,
    /// Başarılı istekler
    pub successful_requests: u64,
    /// Başarısız istekler
    pub failed_requests: u64,
    /// Toplam byte indirildi
    pub bytes_downloaded: u64,
    /// Toplam sayfa yükleme süresi (ms)
    pub total_load_time_ms: u64,
    /// Ziyaret edilen URL'ler
    pub urls_visited: Vec<String>,
    /// Aksiyon sayısı
    pub actions_taken: u64,
    /// LLM token kullanımı
    pub tokens_used: u64,
}

impl BrowserSession {
    /// Yeni oturum oluştur
    pub fn new(config: AgentConfig) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            started_at: Utc::now(),
            config: SessionConfig::from(config),
            stats: SessionStats::default(),
            active: true,
        }
    }
    
    /// İstek kaydet
    pub fn record_request(&mut self, success: bool, bytes: u64, load_time_ms: u64) {
        self.stats.total_requests += 1;
        if success {
            self.stats.successful_requests += 1;
        } else {
            self.stats.failed_requests += 1;
        }
        self.stats.bytes_downloaded += bytes;
        self.stats.total_load_time_ms += load_time_ms;
    }
    
    /// URL ziyareti kaydet
    pub fn record_url_visit(&mut self, url: &str) {
        if !self.stats.urls_visited.contains(&url.to_string()) {
            self.stats.urls_visited.push(url.to_string());
        }
    }
    
    /// Aksiyon kaydet
    pub fn record_action(&mut self) {
        self.stats.actions_taken += 1;
    }
    
    /// Token kullanımı kaydet
    pub fn record_tokens(&mut self, tokens: u64) {
        self.stats.tokens_used += tokens;
    }
    
    /// İstatistikleri al
    pub fn stats(&self) -> SessionStats {
        self.stats.clone()
    }
    
    /// Oturumu kapat
    pub fn close(&mut self) {
        self.active = false;
    }
    
    /// Süre hesapla
    pub fn duration(&self) -> Duration {
        let now = Utc::now();
        (now - self.started_at).to_std().unwrap_or(Duration::ZERO)
    }
    
    /// Aktif mi?
    pub fn is_active(&self) -> bool {
        self.active
    }
    
    /// Başarı oranı
    pub fn success_rate(&self) -> f64 {
        if self.stats.total_requests == 0 {
            return 0.0;
        }
        self.stats.successful_requests as f64 / self.stats.total_requests as f64
    }
    
    /// Ortalama yükleme süresi
    pub fn avg_load_time_ms(&self) -> u64 {
        if self.stats.successful_requests == 0 {
            return 0;
        }
        self.stats.total_load_time_ms / self.stats.successful_requests
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_session_creation() {
        let config = AgentConfig::default();
        let session = BrowserSession::new(config);
        
        assert!(session.is_active());
        assert!(!session.id.is_empty());
    }
    
    #[test]
    fn test_record_request() {
        let config = AgentConfig::default();
        let mut session = BrowserSession::new(config);
        
        session.record_request(true, 1024, 500);
        session.record_request(false, 0, 0);
        
        assert_eq!(session.stats.total_requests, 2);
        assert_eq!(session.stats.successful_requests, 1);
        assert_eq!(session.stats.failed_requests, 1);
        assert_eq!(session.stats.bytes_downloaded, 1024);
    }
    
    #[test]
    fn test_success_rate() {
        let config = AgentConfig::default();
        let mut session = BrowserSession::new(config);
        
        session.record_request(true, 0, 0);
        session.record_request(true, 0, 0);
        session.record_request(false, 0, 0);
        
        let rate = session.success_rate();
        assert!((rate - 0.666).abs() < 0.1);
    }
}
