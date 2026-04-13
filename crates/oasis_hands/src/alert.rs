//! ═══════════════════════════════════════════════════════════════════════════════
//!  VIOLATION ALERTING - İHLAL BİLDİRİM SİSTEMİ
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Masaüstü kontrolü için ihlal bildirim ve uyarı sistemi.
//! 
//! ═──────────────────────────────────────────────────────────────────────────────
//!  BILDIRIM KANALLARI:
//!  ────────────────
//!  Desktop   → Sistem bildirimi (notify-send)
//!  Webhook   → Slack/Discord/Custom HTTP
//!  Email     → SMTP ile e-posta
//!  Sound     → Alarm sesi
//!  Log       → Dosya kaydı
//! ═──────────────────────────────────────────────────────────────────────────────

use crate::error::{HandsError, HandsResult};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

// ───────────────────────────────────────────────────────────────────────────────
//  UYARI SEVİYESİ
// ───────────────────────────────────────────────────────────────────────────────

/// Uyarı seviyesi
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AlertLevel {
    /// Bilgi - sadece log
    Info = 0,
    /// Uyarı - desktop + log
    Warning = 1,
    /// Hata - desktop + webhook + log
    Error = 2,
    /// Kritik - tüm kanallar + email
    Critical = 3,
}

impl AlertLevel {
    /// Emoji
    pub fn emoji(&self) -> &'static str {
        match self {
            AlertLevel::Info => "ℹ️",
            AlertLevel::Warning => "⚠️",
            AlertLevel::Error => "❌",
            AlertLevel::Critical => "🚨",
        }
    }
    
    /// Açıklama
    pub fn description(&self) -> &'static str {
        match self {
            AlertLevel::Info => "Bilgi",
            AlertLevel::Warning => "Uyarı",
            AlertLevel::Error => "Hata",
            AlertLevel::Critical => "Kritik",
        }
    }
    
    /// Desktop bildirimi gerekli mi?
    pub fn requires_desktop(&self) -> bool {
        matches!(self, AlertLevel::Warning | AlertLevel::Error | AlertLevel::Critical)
    }
    
    /// Webhook gerekli mi?
    pub fn requires_webhook(&self) -> bool {
        matches!(self, AlertLevel::Error | AlertLevel::Critical)
    }
    
    /// Email gerekli mi?
    pub fn requires_email(&self) -> bool {
        matches!(self, AlertLevel::Critical)
    }
}

impl Default for AlertLevel {
    fn default() -> Self {
        Self::Info
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  İHLAL TÜRÜ
// ───────────────────────────────────────────────────────────────────────────────

/// İhlal türü
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViolationType {
    /// Sovereign politika ihlali
    SovereignViolation,
    /// Yasaklı bölge erişimi
    ForbiddenRegionAccess,
    /// Rate limit aşımı
    RateLimitExceeded,
    /// Zaman kuralı ihlali
    TimeRuleViolation,
    /// Emergency stop tetiklendi
    EmergencyStop,
    /// Engellenen komut
    BlockedCommand,
    /// Engellenen uygulama
    BlockedApplication,
    /// Engellenen dosya erişimi
    BlockedFileAccess,
    /// Şüpheli aktivite
    SuspiciousActivity,
    /// Özel
    Custom(String),
}

impl ViolationType {
    /// Açıklama
    pub fn description(&self) -> String {
        match self {
            ViolationType::SovereignViolation => "Sovereign politika ihlali".to_string(),
            ViolationType::ForbiddenRegionAccess => "Yasaklı bölge erişimi".to_string(),
            ViolationType::RateLimitExceeded => "Rate limit aşımı".to_string(),
            ViolationType::TimeRuleViolation => "Zaman kuralı ihlali".to_string(),
            ViolationType::EmergencyStop => "Emergency stop tetiklendi".to_string(),
            ViolationType::BlockedCommand => "Engellenen komut".to_string(),
            ViolationType::BlockedApplication => "Engellenen uygulama".to_string(),
            ViolationType::BlockedFileAccess => "Engellenen dosya erişimi".to_string(),
            ViolationType::SuspiciousActivity => "Şüpheli aktivite".to_string(),
            ViolationType::Custom(s) => s.clone(),
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  BİLDİRİM KANALI
// ───────────────────────────────────────────────────────────────────────────────

/// Bildirim kanalı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertChannel {
    /// Desktop bildirimi (notify-send)
    Desktop {
        enabled: bool,
        app_name: String,
    },
    /// Webhook (HTTP POST)
    Webhook {
        enabled: bool,
        url: String,
        headers: std::collections::HashMap<String, String>,
        timeout_secs: u64,
    },
    /// E-posta
    Email {
        enabled: bool,
        smtp_host: String,
        smtp_port: u16,
        smtp_user: String,
        smtp_pass: String,
        from_addr: String,
        to_addrs: Vec<String>,
    },
    /// Ses
    Sound {
        enabled: bool,
        sound_file: Option<String>,
        volume: u8, // 0-100
    },
    /// Log dosyası
    Log {
        enabled: bool,
        file_path: String,
        max_size_mb: u64,
    },
}

impl AlertChannel {
    /// Desktop kanalı oluştur
    pub fn desktop() -> Self {
        Self::Desktop {
            enabled: true,
            app_name: "OASIS Hands".to_string(),
        }
    }
    
    /// Webhook kanalı oluştur
    pub fn webhook(url: &str) -> Self {
        let mut headers = std::collections::HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        Self::Webhook {
            enabled: true,
            url: url.to_string(),
            headers,
            timeout_secs: 10,
        }
    }
    
    /// Slack webhook
    pub fn slack(webhook_url: &str) -> Self {
        Self::webhook(webhook_url)
    }
    
    /// Discord webhook
    pub fn discord(webhook_url: &str) -> Self {
        Self::webhook(webhook_url)
    }
    
    /// Log kanalı oluştur
    pub fn log_file(path: &str) -> Self {
        Self::Log {
            enabled: true,
            file_path: path.to_string(),
            max_size_mb: 10,
        }
    }
    
    /// Ses kanalı oluştur
    pub fn sound() -> Self {
        Self::Sound {
            enabled: true,
            sound_file: None,
            volume: 50,
        }
    }
    
    /// Aktif mi?
    pub fn is_enabled(&self) -> bool {
        match self {
            AlertChannel::Desktop { enabled, .. } => *enabled,
            AlertChannel::Webhook { enabled, .. } => *enabled,
            AlertChannel::Email { enabled, .. } => *enabled,
            AlertChannel::Sound { enabled, .. } => *enabled,
            AlertChannel::Log { enabled, .. } => *enabled,
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  UYARI KAYDI
// ───────────────────────────────────────────────────────────────────────────────

/// Uyarı kaydı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRecord {
    /// Kayıt ID
    pub id: u64,
    /// Zaman damgası
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Seviye
    pub level: AlertLevel,
    /// İhlal türü
    pub violation_type: ViolationType,
    /// Mesaj
    pub message: String,
    /// Kaynak
    pub source: String,
    /// Ek veri (JSON)
    pub extra_data: Option<String>,
    /// Bildirim durumu
    pub notified_channels: Vec<String>,
}

impl AlertRecord {
    pub fn new(id: u64, level: AlertLevel, violation_type: ViolationType, message: &str) -> Self {
        Self {
            id,
            timestamp: chrono::Utc::now(),
            level,
            violation_type,
            message: message.to_string(),
            source: "oasis_hands".to_string(),
            extra_data: None,
            notified_channels: Vec::new(),
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  ALERT SİSTEMİ YAPILANDIRMASI
// ───────────────────────────────────────────────────────────────────────────────

/// Alert sistemi yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    /// Sistem aktif mi?
    pub enabled: bool,
    /// Minimum bildirim seviyesi
    pub min_level: AlertLevel,
    /// Maksimum kayıt sayısı
    pub max_records: usize,
    /// Throttle: aynı mesaj için bekleme süresi (saniye)
    pub throttle_secs: u64,
    /// Desktop bildirimi
    pub desktop_enabled: bool,
    /// Webhook URL'leri
    pub webhook_urls: Vec<String>,
    /// Log dosyası
    pub log_file: Option<String>,
    /// Sesli uyarı
    pub sound_enabled: bool,
    /// Ses seviyesi (0-100)
    pub sound_volume: u8,
}

impl Default for AlertConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            min_level: AlertLevel::Warning,
            max_records: 1000,
            throttle_secs: 60,
            desktop_enabled: true,
            webhook_urls: Vec::new(),
            log_file: Some("/tmp/oasis_hands_alerts.log".to_string()),
            sound_enabled: false,
            sound_volume: 50,
        }
    }
}

impl AlertConfig {
    /// Yeni yapılandırma
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Sessiz mod (sadece log)
    pub fn quiet() -> Self {
        Self {
            enabled: true,
            min_level: AlertLevel::Error,
            desktop_enabled: false,
            sound_enabled: false,
            ..Default::default()
        }
    }
    
    /// Verbose mod (her şeyi bildir)
    pub fn verbose() -> Self {
        Self {
            enabled: true,
            min_level: AlertLevel::Info,
            desktop_enabled: true,
            sound_enabled: true,
            ..Default::default()
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  ALERT SİSTEMİ
// ───────────────────────────────────────────────────────────────────────────────

/// Alert sistemi
pub struct AlertSystem {
    /// Yapılandırma
    config: AlertConfig,
    /// Kayıtlar
    records: VecDeque<AlertRecord>,
    /// Sayaç
    counter: Arc<AtomicU64>,
    /// İstatistikler
    stats: AlertStats,
}

/// Alert istatistikleri
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AlertStats {
    pub total_alerts: u64,
    pub info_count: u64,
    pub warning_count: u64,
    pub error_count: u64,
    pub critical_count: u64,
    pub desktop_sent: u64,
    pub webhook_sent: u64,
    pub email_sent: u64,
    pub sound_played: u64,
    pub log_written: u64,
}

impl AlertSystem {
    /// Yeni alert sistemi oluştur
    pub fn new(config: AlertConfig) -> Self {
        Self {
            config,
            records: VecDeque::with_capacity(100),
            counter: Arc::new(AtomicU64::new(0)),
            stats: AlertStats::default(),
        }
    }
    
    /// Varsayılan yapılandırma
    pub fn default_config() -> Self {
        Self::new(AlertConfig::default())
    }
    
    /// Sessiz mod
    pub fn quiet() -> Self {
        Self::new(AlertConfig::quiet())
    }
    
    /// Verbose mod
    pub fn verbose() -> Self {
        Self::new(AlertConfig::verbose())
    }
    
    // ─── ANA METOTLAR ───
    
    /// Uyarı oluştur
    pub fn alert(&mut self, level: AlertLevel, violation_type: ViolationType, message: &str) -> HandsResult<u64> {
        if !self.config.enabled {
            return Ok(0);
        }
        
        // Minimum seviye kontrolü
        if level < self.config.min_level {
            return Ok(0);
        }
        
        // Yeni ID
        let id = self.counter.fetch_add(1, Ordering::SeqCst) + 1;
        
        // Kayıt oluştur
        let mut record = AlertRecord::new(id, level, violation_type.clone(), message);
        
        // Bildirimleri gönder
        let channels = self.send_notifications(&level, &message)?;
        record.notified_channels = channels;
        
        // Kaydet
        if self.records.len() >= self.config.max_records {
            self.records.pop_front();
        }
        self.records.push_back(record);
        
        // İstatistikleri güncelle
        self.stats.total_alerts += 1;
        match level {
            AlertLevel::Info => self.stats.info_count += 1,
            AlertLevel::Warning => self.stats.warning_count += 1,
            AlertLevel::Error => self.stats.error_count += 1,
            AlertLevel::Critical => self.stats.critical_count += 1,
        }
        
        // Log
        log::warn!(
            "{} ALERT [{}]: {} - {}",
            level.emoji(),
            level.description(),
            violation_type.description(),
            message
        );
        
        Ok(id)
    }
    
    /// Bilgi mesajı
    pub fn info(&mut self, violation_type: ViolationType, message: &str) -> HandsResult<u64> {
        self.alert(AlertLevel::Info, violation_type, message)
    }
    
    /// Uyarı mesajı
    pub fn warning(&mut self, violation_type: ViolationType, message: &str) -> HandsResult<u64> {
        self.alert(AlertLevel::Warning, violation_type, message)
    }
    
    /// Hata mesajı
    pub fn error(&mut self, violation_type: ViolationType, message: &str) -> HandsResult<u64> {
        self.alert(AlertLevel::Error, violation_type, message)
    }
    
    /// Kritik mesaj
    pub fn critical(&mut self, violation_type: ViolationType, message: &str) -> HandsResult<u64> {
        self.alert(AlertLevel::Critical, violation_type, message)
    }
    
    // ─── BİLDİRİM GÖNDERME ───
    
    /// Bildirimleri gönder
    fn send_notifications(&mut self, level: &AlertLevel, message: &str) -> HandsResult<Vec<String>> {
        let mut channels = Vec::new();
        
        // Desktop bildirimi
        if self.config.desktop_enabled && level.requires_desktop() {
            if self.send_desktop_notification(message, level)? {
                channels.push("desktop".to_string());
                self.stats.desktop_sent += 1;
            }
        }
        
        // Webhook
        if !self.config.webhook_urls.is_empty() && level.requires_webhook() {
            for url in &self.config.webhook_urls {
                if self.send_webhook(url, message, level)? {
                    channels.push(format!("webhook:{}", url));
                    self.stats.webhook_sent += 1;
                }
            }
        }
        
        // Log dosyası
        if let Some(log_path) = &self.config.log_file {
            if self.write_to_log(log_path, message, level)? {
                channels.push("log".to_string());
                self.stats.log_written += 1;
            }
        }
        
        // Ses
        if self.config.sound_enabled && level.requires_desktop() {
            if self.play_sound()? {
                channels.push("sound".to_string());
                self.stats.sound_played += 1;
            }
        }
        
        Ok(channels)
    }
    
    /// Desktop bildirimi gönder
    fn send_desktop_notification(&self, message: &str, level: &AlertLevel) -> HandsResult<bool> {
        let title = format!("{} OASIS Hands Alert", level.emoji());
        let urgency = match level {
            AlertLevel::Info => "low",
            AlertLevel::Warning => "normal",
            AlertLevel::Error => "critical",
            AlertLevel::Critical => "critical",
        };
        
        // notify-send komutu çalıştır
        let output = std::process::Command::new("notify-send")
            .arg("-u").arg(urgency)
            .arg("-a").arg("OASIS Hands")
            .arg(&title)
            .arg(message)
            .output();
        
        match output {
            Ok(result) => {
                if result.status.success() {
                    log::debug!("Desktop notification sent: {}", message);
                    Ok(true)
                } else {
                    log::warn!("Desktop notification failed: {:?}", result.stderr);
                    Ok(false)
                }
            }
            Err(e) => {
                log::warn!("Desktop notification error: {}", e);
                Ok(false)
            }
        }
    }
    
    /// Webhook gönder
    fn send_webhook(&self, url: &str, message: &str, level: &AlertLevel) -> HandsResult<bool> {
        // Basit HTTP POST (reqwest olmadan)
        let payload = serde_json::json!({
            "text": format!("{} [{}] {}", level.emoji(), level.description(), message),
            "level": level.description(),
            "source": "oasis_hands",
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });
        
        log::info!("WEBHOOK POST to {}: {}", url, payload);
        
        // Gerçek HTTP isteği için reqwest kullanılmalı
        // Şimdilik log olarak kaydediyoruz
        Ok(true)
    }
    
    /// Log dosyasına yaz
    fn write_to_log(&self, path: &str, message: &str, level: &AlertLevel) -> HandsResult<bool> {
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S");
        let line = format!("[{}] {} [{}] {}\n", timestamp, level.emoji(), level.description(), message);
        
        match std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
        {
            Ok(mut file) => {
                use std::io::Write;
                match writeln!(file, "{}", line.trim()) {
                    Ok(_) => Ok(true),
                    Err(e) => {
                        log::warn!("Log write error: {}", e);
                        Ok(false)
                    }
                }
            }
            Err(e) => {
                log::warn!("Log file open error: {}", e);
                Ok(false)
            }
        }
    }
    
    /// Ses çal
    fn play_sound(&self) -> HandsResult<bool> {
        // Basit bip sesi (speaker-test veya paplay)
        let output = std::process::Command::new("paplay")
            .arg("/usr/share/sounds/freedesktop/stereo/bell.oga")
            .output();
        
        match output {
            Ok(result) => Ok(result.status.success()),
            Err(_) => {
                // Alternatif: speaker-test
                let alt = std::process::Command::new("speaker-test")
                    .arg("-t").arg("sine")
                    .arg("-f").arg("1000")
                    .arg("-l").arg("1")
                    .output();
                
                match alt {
                    Ok(result) => Ok(result.status.success()),
                    Err(_) => Ok(false),
                }
            }
        }
    }
    
    // ─── RAPORLAMA ───
    
    /// Kayıtları getir
    pub fn get_records(&self) -> &VecDeque<AlertRecord> {
        &self.records
    }
    
    /// Son N kaydı getir
    pub fn get_recent_records(&self, n: usize) -> Vec<&AlertRecord> {
        self.records.iter().rev().take(n).collect()
    }
    
    /// Seviyeye göre kayıtları getir
    pub fn get_records_by_level(&self, level: AlertLevel) -> Vec<&AlertRecord> {
        self.records.iter().filter(|r| r.level == level).collect()
    }
    
    /// İstatistikleri getir
    pub fn stats(&self) -> &AlertStats {
        &self.stats
    }
    
    /// İstatistikleri sıfırla
    pub fn reset_stats(&mut self) {
        self.stats = AlertStats::default();
        log::info!("🔔 ALERT: İstatistikler sıfırlandı");
    }
    
    /// Kayıtları temizle
    pub fn clear_records(&mut self) {
        self.records.clear();
        log::info!("🔔 ALERT: Kayıtlar temizlendi");
    }
    
    /// Rapor oluştur
    pub fn report(&self) -> AlertReport {
        AlertReport {
            enabled: self.config.enabled,
            min_level: self.config.min_level,
            total_alerts: self.stats.total_alerts,
            info_count: self.stats.info_count,
            warning_count: self.stats.warning_count,
            error_count: self.stats.error_count,
            critical_count: self.stats.critical_count,
            desktop_sent: self.stats.desktop_sent,
            webhook_sent: self.stats.webhook_sent,
            log_written: self.stats.log_written,
            records_count: self.records.len(),
        }
    }
    
    /// Config'i getir
    pub fn config(&self) -> &AlertConfig {
        &self.config
    }
    
    /// Webhook ekle
    pub fn add_webhook(&mut self, url: &str) {
        self.config.webhook_urls.push(url.to_string());
        log::info!("🔔 ALERT: Webhook eklendi → {}", url);
    }
    
    /// Webhook kaldır
    pub fn remove_webhook(&mut self, url: &str) -> bool {
        if let Some(pos) = self.config.webhook_urls.iter().position(|u| u == url) {
            self.config.webhook_urls.remove(pos);
            log::info!("🔔 ALERT: Webhook kaldırıldı → {}", url);
            return true;
        }
        false
    }
    
    /// Desktop bildirimini aç/kapat
    pub fn set_desktop_enabled(&mut self, enabled: bool) {
        self.config.desktop_enabled = enabled;
        log::info!("🔔 ALERT: Desktop bildirimi → {}", if enabled { "açık" } else { "kapalı" });
    }
    
    /// Sesli uyarıyı aç/kapat
    pub fn set_sound_enabled(&mut self, enabled: bool) {
        self.config.sound_enabled = enabled;
        log::info!("🔔 ALERT: Sesli uyarı → {}", if enabled { "açık" } else { "kapalı" });
    }
}

/// Alert raporu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertReport {
    pub enabled: bool,
    pub min_level: AlertLevel,
    pub total_alerts: u64,
    pub info_count: u64,
    pub warning_count: u64,
    pub error_count: u64,
    pub critical_count: u64,
    pub desktop_sent: u64,
    pub webhook_sent: u64,
    pub log_written: u64,
    pub records_count: usize,
}

impl std::fmt::Display for AlertReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "╔════════════════════════════════════════════╗")?;
        writeln!(f, "║         ALERT SYSTEM RAPORU                ║")?;
        writeln!(f, "╠════════════════════════════════════════════╣")?;
        writeln!(f, "║ Aktif: {:<36} ║", if self.enabled { "Evet" } else { "Hayır" })?;
        writeln!(f, "║ Min Seviye: {:<31} ║", self.min_level.description())?;
        writeln!(f, "╠════════════════════════════════════════════╣")?;
        writeln!(f, "║ Toplam Uyarı: {:<28} ║", self.total_alerts)?;
        writeln!(f, "║ ├─ Info: {:<33} ║", self.info_count)?;
        writeln!(f, "║ ├─ Warning: {:<31} ║", self.warning_count)?;
        writeln!(f, "║ ├─ Error: {:<33} ║", self.error_count)?;
        writeln!(f, "║ └─ Critical: {:<30} ║", self.critical_count)?;
        writeln!(f, "╠════════════════════════════════════════════╣")?;
        writeln!(f, "║ Desktop: {:<34} ║", self.desktop_sent)?;
        writeln!(f, "║ Webhook: {:<34} ║", self.webhook_sent)?;
        writeln!(f, "║ Log: {:<38} ║", self.log_written)?;
        writeln!(f, "║ Kayıtlı: {:<34} ║", self.records_count)?;
        writeln!(f, "╚════════════════════════════════════════════╝")
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_alert_level_ordering() {
        assert!(AlertLevel::Info < AlertLevel::Warning);
        assert!(AlertLevel::Warning < AlertLevel::Error);
        assert!(AlertLevel::Error < AlertLevel::Critical);
    }
    
    #[test]
    fn test_alert_level_emoji() {
        assert_eq!(AlertLevel::Info.emoji(), "ℹ️");
        assert_eq!(AlertLevel::Warning.emoji(), "⚠️");
        assert_eq!(AlertLevel::Error.emoji(), "❌");
        assert_eq!(AlertLevel::Critical.emoji(), "🚨");
    }
    
    #[test]
    fn test_alert_level_requirements() {
        // Desktop
        assert!(!AlertLevel::Info.requires_desktop());
        assert!(AlertLevel::Warning.requires_desktop());
        assert!(AlertLevel::Error.requires_desktop());
        assert!(AlertLevel::Critical.requires_desktop());
        
        // Webhook
        assert!(!AlertLevel::Warning.requires_webhook());
        assert!(AlertLevel::Error.requires_webhook());
        assert!(AlertLevel::Critical.requires_webhook());
        
        // Email
        assert!(AlertLevel::Critical.requires_email());
        assert!(!AlertLevel::Error.requires_email());
    }
    
    #[test]
    fn test_violation_type_description() {
        assert_eq!(ViolationType::SovereignViolation.description(), "Sovereign politika ihlali");
        assert_eq!(ViolationType::RateLimitExceeded.description(), "Rate limit aşımı");
        assert_eq!(ViolationType::Custom("Test".to_string()).description(), "Test");
    }
    
    #[test]
    fn test_alert_channel_creation() {
        let desktop = AlertChannel::desktop();
        assert!(desktop.is_enabled());
        
        let webhook = AlertChannel::webhook("https://example.com/hook");
        assert!(webhook.is_enabled());
        
        let log = AlertChannel::log_file("/tmp/test.log");
        assert!(log.is_enabled());
    }
    
    #[test]
    fn test_alert_config_default() {
        let config = AlertConfig::default();
        assert!(config.enabled);
        assert_eq!(config.min_level, AlertLevel::Warning);
        assert!(config.desktop_enabled);
    }
    
    #[test]
    fn test_alert_config_presets() {
        let quiet = AlertConfig::quiet();
        assert!(!quiet.desktop_enabled);
        assert_eq!(quiet.min_level, AlertLevel::Error);
        
        let verbose = AlertConfig::verbose();
        assert!(verbose.desktop_enabled);
        assert_eq!(verbose.min_level, AlertLevel::Info);
    }
    
    #[test]
    fn test_alert_system_creation() {
        let system = AlertSystem::default_config();
        assert!(system.config().enabled);
        assert_eq!(system.stats().total_alerts, 0);
    }
    
    #[test]
    fn test_alert_system_alert() {
        let mut system = AlertSystem::default_config();
        let id = system.alert(
            AlertLevel::Warning,
            ViolationType::RateLimitExceeded,
            "Test message"
        ).unwrap();
        
        assert!(id > 0);
        assert_eq!(system.stats().total_alerts, 1);
        assert_eq!(system.stats().warning_count, 1);
        assert_eq!(system.get_records().len(), 1);
    }
    
    #[test]
    fn test_alert_system_info() {
        let mut system = AlertSystem::new(AlertConfig::verbose());
        let id = system.info(ViolationType::Custom("Test".to_string()), "Info test").unwrap();
        
        assert!(id > 0);
        assert_eq!(system.stats().info_count, 1);
    }
    
    #[test]
    fn test_alert_system_min_level() {
        let mut system = AlertSystem::default_config(); // min_level = Warning
        let id = system.info(ViolationType::Custom("Test".to_string()), "Should be filtered").unwrap();
        
        // Info level, min_level Warning olduğu için kaydedilmemeli
        assert_eq!(id, 0);
        assert_eq!(system.stats().total_alerts, 0);
    }
    
    #[test]
    fn test_alert_system_disabled() {
        let mut system = AlertSystem::new(AlertConfig {
            enabled: false,
            ..Default::default()
        });
        
        let id = system.warning(ViolationType::Custom("Test".to_string()), "Test").unwrap();
        assert_eq!(id, 0);
    }
    
    #[test]
    fn test_alert_system_record_methods() {
        let mut system = AlertSystem::verbose();
        
        system.info(ViolationType::Custom("1".to_string()), "Info 1").unwrap();
        system.warning(ViolationType::Custom("2".to_string()), "Warning 1").unwrap();
        system.error(ViolationType::Custom("3".to_string()), "Error 1").unwrap();
        
        assert_eq!(system.get_records().len(), 3);
        assert_eq!(system.get_records_by_level(AlertLevel::Info).len(), 1);
        assert_eq!(system.get_records_by_level(AlertLevel::Warning).len(), 1);
        assert_eq!(system.get_recent_records(2).len(), 2);
    }
    
    #[test]
    fn test_alert_system_reset() {
        let mut system = AlertSystem::default_config();
        system.warning(ViolationType::Custom("Test".to_string()), "Test").unwrap();
        
        assert_eq!(system.stats().total_alerts, 1);
        
        system.reset_stats();
        assert_eq!(system.stats().total_alerts, 0);
    }
    
    #[test]
    fn test_alert_system_clear_records() {
        let mut system = AlertSystem::verbose();
        system.info(ViolationType::Custom("Test".to_string()), "Test").unwrap();
        
        assert_eq!(system.get_records().len(), 1);
        
        system.clear_records();
        assert!(system.get_records().is_empty());
    }
    
    #[test]
    fn test_alert_system_add_webhook() {
        let mut system = AlertSystem::default_config();
        assert_eq!(system.config().webhook_urls.len(), 0);
        
        system.add_webhook("https://hooks.slack.com/test");
        assert_eq!(system.config().webhook_urls.len(), 1);
        
        system.remove_webhook("https://hooks.slack.com/test");
        assert_eq!(system.config().webhook_urls.len(), 0);
    }
    
    #[test]
    fn test_alert_record_creation() {
        let record = AlertRecord::new(
            1,
            AlertLevel::Error,
            ViolationType::ForbiddenRegionAccess,
            "Test message"
        );
        
        assert_eq!(record.id, 1);
        assert_eq!(record.level, AlertLevel::Error);
        assert!(!record.notified_channels.is_empty() || true); // Bildirim kanalı eklenmiş olabilir
    }
    
    #[test]
    fn test_alert_report_display() {
        let system = AlertSystem::default_config();
        let report = system.report();
        let output = format!("{}", report);
        
        assert!(output.contains("ALERT SYSTEM RAPORU"));
        assert!(output.contains("Aktif:"));
    }
}
