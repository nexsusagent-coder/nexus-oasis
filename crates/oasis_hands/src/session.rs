//! ═══════════════════════════════════════════════════════════════════════════════
//!  SESSION - OTURUM YÖNETİMİ
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Masaüstü kontrol oturumu kayıt ve istatistik.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// ───────────────────────────────────────────────────────────────────────────────
//  OTURUM YAPILANDIRMASI
// ───────────────────────────────────────────────────────────────────────────────

/// Oturum yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Oturum zaman aşımı (saniye)
    pub timeout_secs: u64,
    /// Maksimum aksiyon sayısı
    pub max_actions: usize,
    /// Maksimum görev sayısı
    pub max_tasks: usize,
    /// Detaylı loglama
    pub verbose: bool,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            timeout_secs: 3600, // 1 saat
            max_actions: 1000,
            max_tasks: 100,
            verbose: false,
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  OTURUM İSTATİSTİKLERİ
// ───────────────────────────────────────────────────────────────────────────────

/// Oturum istatistikleri
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionStats {
    /// Toplam aksiyon sayısı
    pub total_actions: u64,
    /// Başarılı aksiyonlar
    pub successful_actions: u64,
    /// Başarısız aksiyonlar
    pub failed_actions: u64,
    /// Toplam görev sayısı
    pub total_tasks: u64,
    /// Başarılı görevler
    pub successful_tasks: u64,
    /// Toplam fare hareketi (px)
    pub total_mouse_distance: u64,
    /// Toplam tuş vuruşu
    pub total_keystrokes: u64,
    /// Toplam ekran görüntüsü
    pub total_screenshots: u64,
}

impl SessionStats {
    /// Başarı oranı
    pub fn action_success_rate(&self) -> f32 {
        if self.total_actions == 0 {
            return 0.0;
        }
        self.successful_actions as f32 / self.total_actions as f32
    }
    
    /// Görev başarı oranı
    pub fn task_success_rate(&self) -> f32 {
        if self.total_tasks == 0 {
            return 0.0;
        }
        self.successful_tasks as f32 / self.total_tasks as f32
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  AKSİYON KAYDI
// ───────────────────────────────────────────────────────────────────────────────

/// Aksiyon kaydı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRecord {
    /// Aksiyon ID
    pub id: String,
    /// Aksiyon tipi
    pub action_type: String,
    /// Aksiyon detayı
    pub detail: String,
    /// Başarılı mı?
    pub success: bool,
    /// Zaman damgası
    pub timestamp: DateTime<Utc>,
    /// Süre (ms)
    pub duration_ms: u64,
    /// Hata mesajı (varsa)
    pub error: Option<String>,
}

// ───────────────────────────────────────────────────────────────────────────────
//  OTURUM
// ───────────────────────────────────────────────────────────────────────────────

/// Masaüstü kontrol oturumu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandsSession {
    /// Oturum ID
    pub id: String,
    /// Başlangıç zamanı
    pub started_at: DateTime<Utc>,
    /// Son aksiyon zamanı
    pub last_action_at: Option<DateTime<Utc>>,
    /// Yapılandırma
    pub config: SessionConfig,
    /// İstatistikler
    pub stats: SessionStats,
    /// Aksiyon geçmişi
    pub actions: Vec<ActionRecord>,
    /// Aktif mi?
    pub active: bool,
}

impl HandsSession {
    /// Yeni oturum oluştur
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            started_at: Utc::now(),
            last_action_at: None,
            config: SessionConfig::default(),
            stats: SessionStats::default(),
            actions: Vec::new(),
            active: true,
        }
    }
    
    /// Yapılandırma ile yeni oturum
    pub fn with_config(config: SessionConfig) -> Self {
        Self {
            config,
            ..Self::new()
        }
    }
    
    /// Aksiyon kaydet
    pub fn record_action(&mut self, action: &str, result: &str) {
        let record = ActionRecord {
            id: uuid::Uuid::new_v4().to_string(),
            action_type: action.split(':').next().unwrap_or(action).into(),
            detail: action.into(),
            success: result.starts_with("success") || result.contains("başarı"),
            timestamp: Utc::now(),
            duration_ms: 0,
            error: if result.contains("başarı") { None } else { Some(result.into()) },
        };
        
        self.stats.total_actions += 1;
        if record.success {
            self.stats.successful_actions += 1;
        } else {
            self.stats.failed_actions += 1;
        }
        
        self.last_action_at = Some(Utc::now());
        self.actions.push(record);
        
        // Maksimum kontrolü
        if self.actions.len() > self.config.max_actions {
            self.actions.remove(0);
        }
    }
    
    /// Toplam aksiyon sayısı
    pub fn action_count(&self) -> usize {
        self.actions.len()
    }
    
    /// Oturum süresi (saniye)
    pub fn duration_secs(&self) -> i64 {
        (Utc::now() - self.started_at).num_seconds()
    }
    
    /// Süre dolmuş mu?
    pub fn is_expired(&self) -> bool {
        self.duration_secs() > self.config.timeout_secs as i64
    }
    
    /// Oturumu kapat
    pub fn close(&mut self) {
        self.active = false;
        log::info!("📋  SESSION: Oturum kapatıldı (ID: {}, Süre: {}s, Aksiyon: {})", 
            self.id, self.duration_secs(), self.action_count());
    }
    
    /// Özet rapor
    pub fn summary(&self) -> SessionSummary {
        SessionSummary {
            session_id: self.id.clone(),
            duration_secs: self.duration_secs(),
            total_actions: self.stats.total_actions,
            success_rate: self.stats.action_success_rate(),
            total_tasks: self.stats.total_tasks,
            task_success_rate: self.stats.task_success_rate(),
        }
    }
}

impl Default for HandsSession {
    fn default() -> Self {
        Self::new()
    }
}

/// Oturum özeti
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    pub session_id: String,
    pub duration_secs: i64,
    pub total_actions: u64,
    pub success_rate: f32,
    pub total_tasks: u64,
    pub task_success_rate: f32,
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_session_creation() {
        let session = HandsSession::new();
        assert!(session.active);
        assert_eq!(session.action_count(), 0);
    }
    
    #[test]
    fn test_session_record_action() {
        let mut session = HandsSession::new();
        session.record_action("test:action", "success");
        
        assert_eq!(session.action_count(), 1);
        assert_eq!(session.stats.total_actions, 1);
        assert_eq!(session.stats.successful_actions, 1);
    }
    
    #[test]
    fn test_session_failed_action() {
        let mut session = HandsSession::new();
        session.record_action("test:fail", "error message");
        
        assert_eq!(session.stats.failed_actions, 1);
    }
    
    #[test]
    fn test_session_duration() {
        let session = HandsSession::new();
        assert!(session.duration_secs() >= 0);
    }
    
    #[test]
    fn test_session_close() {
        let mut session = HandsSession::new();
        session.close();
        assert!(!session.active);
    }
    
    #[test]
    fn test_session_stats_success_rate() {
        let stats = SessionStats {
            total_actions: 10,
            successful_actions: 8,
            failed_actions: 2,
            ..Default::default()
        };
        
        let rate = stats.action_success_rate();
        assert!((rate - 0.8).abs() < 0.01);
    }
    
    #[test]
    fn test_session_summary() {
        let session = HandsSession::new();
        let summary = session.summary();
        assert_eq!(summary.session_id, session.id);
    }
    
    #[test]
    fn test_session_config_default() {
        let config = SessionConfig::default();
        assert_eq!(config.timeout_secs, 3600);
        assert_eq!(config.max_actions, 1000);
    }
    
    #[test]
    fn test_action_record_creation() {
        let record = ActionRecord {
            id: "test-id".into(),
            action_type: "click".into(),
            detail: "button click".into(),
            success: true,
            timestamp: Utc::now(),
            duration_ms: 50,
            error: None,
        };
        
        assert!(record.success);
        assert!(record.error.is_none());
    }
}
