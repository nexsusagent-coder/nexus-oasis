//! ═══════════════════════════════════════════════════════════════════════════════
//!  RATE LIMITER - KATEGORİ BAZLI HIZ SINIRLAMA
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Mouse, klavye ve genel aksiyonlar için detaylı rate limiting.
//! Burst koruması ve kategori bazlı sınırlar.
//!
//! ═──────────────────────────────────────────────────────────────────────────────
//!  ÖZELLİKLER:
//!  ────────────────
//!  ✅ Mouse rate limiting (move, click, distance)
//!  ✅ Keyboard rate limiting (key press, text)
//!  ✅ General rate limiting (total actions, burst)
//!  ✅ Sliding window tracking
//!  ✅ Otomatik engelleme
//!  ✅ İstatistikler ve raporlama
//! ═──────────────────────────────────────────────────────────────────────────────

use crate::error::{HandsError, HandsResult};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

// ───────────────────────────────────────────────────────────────────────────────
//  RATE LİMİT TANIMLARI
// ─────────────────────────────────────────────────────────────────────────────--

/// Mouse rate limit ayarları
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseRateLimits {
    /// Saniyede maksimum fare hareketi
    pub moves_per_sec: u32,
    /// Saniyede maksimum tıklama
    pub clicks_per_sec: u32,
    /// Saniyede maksimum mesafe (piksel)
    pub distance_per_sec: u32,
}

impl Default for MouseRateLimits {
    fn default() -> Self {
        Self {
            moves_per_sec: 50,
            clicks_per_sec: 10,
            distance_per_sec: 2000,
        }
    }
}

/// Klavye rate limit ayarları
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardRateLimits {
    /// Saniyede maksimum tuş basışı
    pub key_presses_per_sec: u32,
    /// Saniyede maksimum karakter yazma
    pub chars_per_sec: u32,
}

impl Default for KeyboardRateLimits {
    fn default() -> Self {
        Self {
            key_presses_per_sec: 30,
            chars_per_sec: 50,
        }
    }
}

/// Genel rate limit ayarları
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralRateLimits {
    /// Dakikada maksimum toplam aksiyon
    pub actions_per_min: u32,
    /// Burst limit (kısa vadeli spike koruması)
    pub burst_limit: u32,
    /// Burst penceresi (saniye)
    pub burst_window_secs: u32,
}

impl Default for GeneralRateLimits {
    fn default() -> Self {
        Self {
            actions_per_min: 120,
            burst_limit: 20,
            burst_window_secs: 5,
        }
    }
}

/// Tüm rate limit ayarları
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub mouse: MouseRateLimits,
    pub keyboard: KeyboardRateLimits,
    pub general: GeneralRateLimits,
    /// Rate limiting aktif mi?
    pub enabled: bool,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            mouse: MouseRateLimits::default(),
            keyboard: KeyboardRateLimits::default(),
            general: GeneralRateLimits::default(),
            enabled: true,
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  AKSİYON KAYDI
// ─────────────────────────────────────────────────────────────────────────────--

/// Aksiyon türü
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionType {
    /// Fare hareketi
    MouseMove,
    /// Fare tıklama
    MouseClick,
    /// Tuş basışı
    KeyPress,
    /// Karakter yazma
    TextChar,
    /// Genel aksiyon
    General,
}

/// Zaman damgalı aksiyon
#[derive(Debug, Clone)]
struct TimestampedAction {
    /// Aksiyon türü
    action_type: ActionType,
    /// Zaman damgası
    timestamp: Instant,
    /// Değer (mesafe, karakter sayısı, vb.)
    value: u32,
}

// ───────────────────────────────────────────────────────────────────────────────
//  RATE LİMİTER
// ───────────────────────────────────────────────────────────────────────────────

/// Rate limiter istatistikleri
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RateLimiterStats {
    /// Toplam aksiyon sayısı
    pub total_actions: u64,
    /// Engellenen aksiyon sayısı
    pub blocked_actions: u64,
    /// Mouse hareket sayısı
    pub mouse_moves: u64,
    /// Mouse tıklama sayısı
    pub mouse_clicks: u64,
    /// Tuş basma sayısı
    pub key_presses: u64,
    /// Karakter sayısı
    pub text_chars: u64,
    /// Toplam fare mesafesi
    pub total_mouse_distance: u64,
    /// Son engelleme nedeni
    pub last_block_reason: Option<String>,
    /// Son engelleme zamanı
    pub last_block_time: Option<String>,
}

/// Ana Rate Limiter
#[derive(Debug)]
pub struct RateLimiter {
    /// Yapılandırma
    config: RateLimitConfig,
    /// Aksiyon geçmişi (sliding window)
    action_history: VecDeque<TimestampedAction>,
    /// İstatistikler
    stats: RateLimiterStats,
    /// Başlangıç zamanı
    start_time: Instant,
}

impl RateLimiter {
    /// Yeni rate limiter oluştur
    pub fn new() -> Self {
        Self {
            config: RateLimitConfig::default(),
            action_history: VecDeque::with_capacity(1000),
            stats: RateLimiterStats::default(),
            start_time: Instant::now(),
        }
    }

    /// Özel yapılandırma ile oluştur
    pub fn with_config(config: RateLimitConfig) -> Self {
        Self {
            config,
            action_history: VecDeque::with_capacity(1000),
            stats: RateLimiterStats::default(),
            start_time: Instant::now(),
        }
    }

    // ─── KONTROL METODLARI ───

    /// Aksiyon izinli mi kontrol et
    pub fn check(&mut self, action_type: ActionType, value: u32) -> HandsResult<()> {
        if !self.config.enabled {
            return Ok(());
        }

        let now = Instant::now();

        // Eski kayıtları temizle (1 dakikadan eski)
        self.cleanup_old_actions(now);

        // Kategori bazlı kontrol
        match action_type {
            ActionType::MouseMove => self.check_mouse_move(now, value)?,
            ActionType::MouseClick => self.check_mouse_click(now)?,
            ActionType::KeyPress => self.check_key_press(now)?,
            ActionType::TextChar => self.check_text_char(now, value)?,
            ActionType::General => {}
        }

        // Genel limit kontrolü
        self.check_general_limits(now)?;

        // Burst kontrolü
        self.check_burst(now)?;

        // Aksiyonu kaydet
        self.record_action(action_type, now, value);

        Ok(())
    }

    /// Fare hareketi kontrolü
    fn check_mouse_move(&self, now: Instant, distance: u32) -> HandsResult<()> {
        let window = Duration::from_secs(1);
        let moves_last_sec: u32 = self.action_history
            .iter()
            .filter(|a| a.action_type == ActionType::MouseMove && now.duration_since(a.timestamp) < window)
            .count() as u32;

        let distance_last_sec: u32 = self.action_history
            .iter()
            .filter(|a| a.action_type == ActionType::MouseMove && now.duration_since(a.timestamp) < window)
            .map(|a| a.value)
            .sum();

        if moves_last_sec >= self.config.mouse.moves_per_sec {
            return Err(HandsError::RateLimitExceeded(format!(
                "Fare hareket limiti aşıldı: {}/{} (saniye)",
                moves_last_sec, self.config.mouse.moves_per_sec
            )));
        }

        if distance_last_sec + distance > self.config.mouse.distance_per_sec {
            return Err(HandsError::RateLimitExceeded(format!(
                "Fare mesafe limiti aşıldı: {}/{} px (saniye)",
                distance_last_sec + distance, self.config.mouse.distance_per_sec
            )));
        }

        Ok(())
    }

    /// Fare tıklama kontrolü
    fn check_mouse_click(&self, now: Instant) -> HandsResult<()> {
        let window = Duration::from_secs(1);
        let clicks_last_sec: u32 = self.action_history
            .iter()
            .filter(|a| a.action_type == ActionType::MouseClick && now.duration_since(a.timestamp) < window)
            .count() as u32;

        if clicks_last_sec >= self.config.mouse.clicks_per_sec {
            return Err(HandsError::RateLimitExceeded(format!(
                "Tıklama limiti aşıldı: {}/{} (saniye)",
                clicks_last_sec, self.config.mouse.clicks_per_sec
            )));
        }

        Ok(())
    }

    /// Tuş basma kontrolü
    fn check_key_press(&self, now: Instant) -> HandsResult<()> {
        let window = Duration::from_secs(1);
        let keys_last_sec: u32 = self.action_history
            .iter()
            .filter(|a| a.action_type == ActionType::KeyPress && now.duration_since(a.timestamp) < window)
            .count() as u32;

        if keys_last_sec >= self.config.keyboard.key_presses_per_sec {
            return Err(HandsError::RateLimitExceeded(format!(
                "Tuş basma limiti aşıldı: {}/{} (saniye)",
                keys_last_sec, self.config.keyboard.key_presses_per_sec
            )));
        }

        Ok(())
    }

    /// Karakter yazma kontrolü
    fn check_text_char(&self, now: Instant, count: u32) -> HandsResult<()> {
        let window = Duration::from_secs(1);
        let chars_last_sec: u32 = self.action_history
            .iter()
            .filter(|a| a.action_type == ActionType::TextChar && now.duration_since(a.timestamp) < window)
            .map(|a| a.value)
            .sum();

        if chars_last_sec + count > self.config.keyboard.chars_per_sec {
            return Err(HandsError::RateLimitExceeded(format!(
                "Karakter limiti aşıldı: {}/{} (saniye)",
                chars_last_sec + count, self.config.keyboard.chars_per_sec
            )));
        }

        Ok(())
    }

    /// Genel limit kontrolü
    fn check_general_limits(&self, now: Instant) -> HandsResult<()> {
        let window = Duration::from_secs(60);
        let actions_last_min: u32 = self.action_history
            .iter()
            .filter(|a| now.duration_since(a.timestamp) < window)
            .count() as u32;

        if actions_last_min >= self.config.general.actions_per_min {
            return Err(HandsError::RateLimitExceeded(format!(
                "Dakikalık aksiyon limiti aşıldı: {}/{}",
                actions_last_min, self.config.general.actions_per_min
            )));
        }

        Ok(())
    }

    /// Burst kontrolü
    fn check_burst(&self, now: Instant) -> HandsResult<()> {
        let window = Duration::from_secs(self.config.general.burst_window_secs as u64);
        let actions_in_window: u32 = self.action_history
            .iter()
            .filter(|a| now.duration_since(a.timestamp) < window)
            .count() as u32;

        if actions_in_window >= self.config.general.burst_limit {
            return Err(HandsError::RateLimitExceeded(format!(
                "Burst limiti aşıldı: {} aksiyon/{} saniye (max: {})",
                actions_in_window,
                self.config.general.burst_window_secs,
                self.config.general.burst_limit
            )));
        }

        Ok(())
    }

    // ─── YARDIMCI METODLAR ───

    /// Aksiyon kaydet
    fn record_action(&mut self, action_type: ActionType, timestamp: Instant, value: u32) {
        self.action_history.push_back(TimestampedAction {
            action_type,
            timestamp,
            value,
        });

        // İstatistikleri güncelle
        self.stats.total_actions += 1;
        match action_type {
            ActionType::MouseMove => {
                self.stats.mouse_moves += 1;
                self.stats.total_mouse_distance += value as u64;
            }
            ActionType::MouseClick => self.stats.mouse_clicks += 1,
            ActionType::KeyPress => self.stats.key_presses += 1,
            ActionType::TextChar => self.stats.text_chars += value as u64,
            ActionType::General => {}
        }
    }

    /// Engellenen aksiyonu kaydet
    pub fn record_block(&mut self, reason: &str) {
        self.stats.blocked_actions += 1;
        self.stats.last_block_reason = Some(reason.to_string());
        self.stats.last_block_time = Some(chrono::Utc::now().to_rfc3339());
        
        log::warn!("🚦  RATE LIMIT: Aksiyon engellendi → {}", reason);
    }

    /// Eski aksiyonları temizle
    fn cleanup_old_actions(&mut self, now: Instant) {
        let max_age = Duration::from_secs(120); // 2 dakika
        while let Some(front) = self.action_history.front() {
            if now.duration_since(front.timestamp) > max_age {
                self.action_history.pop_front();
            } else {
                break;
            }
        }
    }

    // ─── GETTER/SETTER ───

    /// Yapılandırmayı getir
    pub fn config(&self) -> &RateLimitConfig {
        &self.config
    }

    /// Yapılandırmayı güncelle
    pub fn set_config(&mut self, config: RateLimitConfig) {
        log::info!("🚦  RATE LIMIT: Yapılandırma güncellendi");
        self.config = config;
    }

    /// İstatistikleri getir
    pub fn stats(&self) -> &RateLimiterStats {
        &self.stats
    }

    /// İstatistikleri sıfırla
    pub fn reset_stats(&mut self) {
        self.stats = RateLimiterStats::default();
        self.action_history.clear();
        log::info!("🚦  RATE LIMIT: İstatistikler sıfırlandı");
    }

    /// Aktif mi?
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Aktifleştir
    pub fn enable(&mut self) {
        self.config.enabled = true;
        log::info!("🚦  RATE LIMIT: Aktifleştirildi");
    }

    /// Devre dışı bırak
    pub fn disable(&mut self) {
        self.config.enabled = false;
        log::warn!("🚦  RATE LIMIT: Devre dışı bırakıldı");
    }

    // ─── RAPORLAMA ───

    /// Mevcut durumu raporla
    pub fn report(&self) -> RateLimitReport {
        let now = Instant::now();
        
        // Son 1 saniye
        let last_sec = Duration::from_secs(1);
        let last_min = Duration::from_secs(60);

        let moves_last_sec = self.action_history
            .iter()
            .filter(|a| a.action_type == ActionType::MouseMove && now.duration_since(a.timestamp) < last_sec)
            .count() as u32;

        let clicks_last_sec = self.action_history
            .iter()
            .filter(|a| a.action_type == ActionType::MouseClick && now.duration_since(a.timestamp) < last_sec)
            .count() as u32;

        let keys_last_sec = self.action_history
            .iter()
            .filter(|a| a.action_type == ActionType::KeyPress && now.duration_since(a.timestamp) < last_sec)
            .count() as u32;

        let actions_last_min = self.action_history
            .iter()
            .filter(|a| now.duration_since(a.timestamp) < last_min)
            .count() as u32;

        RateLimitReport {
            enabled: self.config.enabled,
            moves_last_sec,
            clicks_last_sec,
            keys_last_sec,
            actions_last_min,
            total_actions: self.stats.total_actions,
            blocked_actions: self.stats.blocked_actions,
            limits: self.config.clone(),
        }
    }

    /// Bekleme süresi hesapla (bir sonraki aksiyon için)
    pub fn wait_time(&self, action_type: ActionType) -> Duration {
        let now = Instant::now();
        let window = Duration::from_secs(1);

        match action_type {
            ActionType::MouseMove => {
                let oldest_in_window = self.action_history
                    .iter()
                    .filter(|a| a.action_type == ActionType::MouseMove && now.duration_since(a.timestamp) < window)
                    .min_by_key(|a| a.timestamp);

                if let Some(oldest) = oldest_in_window {
                    let elapsed = now.duration_since(oldest.timestamp);
                    if elapsed < window {
                        return window - elapsed;
                    }
                }
                Duration::ZERO
            }
            ActionType::MouseClick => {
                let count = self.action_history
                    .iter()
                    .filter(|a| a.action_type == ActionType::MouseClick && now.duration_since(a.timestamp) < window)
                    .count() as u32;

                if count >= self.config.mouse.clicks_per_sec {
                    let oldest = self.action_history
                        .iter()
                        .filter(|a| a.action_type == ActionType::MouseClick && now.duration_since(a.timestamp) < window)
                        .min_by_key(|a| a.timestamp);

                    if let Some(o) = oldest {
                        let elapsed = now.duration_since(o.timestamp);
                        if elapsed < window {
                            return window - elapsed;
                        }
                    }
                }
                Duration::ZERO
            }
            _ => Duration::ZERO,
        }
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  RAPOR YAPISI
// ─────────────────────────────────────────────────────────────────────────────--

/// Rate limit raporu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitReport {
    /// Aktif mi?
    pub enabled: bool,
    /// Son 1 saniyedeki fare hareketleri
    pub moves_last_sec: u32,
    /// Son 1 saniyedeki tıklamalar
    pub clicks_last_sec: u32,
    /// Son 1 saniyedeki tuş basmaları
    pub keys_last_sec: u32,
    /// Son 1 dakikadaki aksiyonlar
    pub actions_last_min: u32,
    /// Toplam aksiyon
    pub total_actions: u64,
    /// Engellenen aksiyon
    pub blocked_actions: u64,
    /// Limit ayarları
    pub limits: RateLimitConfig,
}

impl RateLimitReport {
    /// JSON çıktı
    pub fn to_json(&self) -> HandsResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| HandsError::ConfigError(format!("JSON hatası: {}", e)))
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter_creation() {
        let rl = RateLimiter::new();
        assert!(rl.is_enabled());
        assert_eq!(rl.stats().total_actions, 0);
    }

    #[test]
    fn test_mouse_move_allowed() {
        let mut rl = RateLimiter::new();
        
        // İlk hareket izin verilmeli
        let result = rl.check(ActionType::MouseMove, 100);
        assert!(result.is_ok());
        assert_eq!(rl.stats().mouse_moves, 1);
    }

    #[test]
    fn test_mouse_click_allowed() {
        let mut rl = RateLimiter::new();
        
        let result = rl.check(ActionType::MouseClick, 1);
        assert!(result.is_ok());
        assert_eq!(rl.stats().mouse_clicks, 1);
    }

    #[test]
    fn test_key_press_allowed() {
        let mut rl = RateLimiter::new();
        
        let result = rl.check(ActionType::KeyPress, 1);
        assert!(result.is_ok());
        assert_eq!(rl.stats().key_presses, 1);
    }

    #[test]
    fn test_text_char_allowed() {
        let mut rl = RateLimiter::new();
        
        let result = rl.check(ActionType::TextChar, 10);
        assert!(result.is_ok());
        assert_eq!(rl.stats().text_chars, 10);
    }

    #[test]
    fn test_rate_limit_disabled() {
        let mut rl = RateLimiter::new();
        rl.disable();
        
        // Devre dışı iken tüm aksiyonlar geçmeli
        for _ in 0..100 {
            assert!(rl.check(ActionType::MouseClick, 1).is_ok());
        }
    }

    #[test]
    fn test_rate_limit_config() {
        let config = RateLimitConfig {
            mouse: MouseRateLimits {
                moves_per_sec: 10,
                clicks_per_sec: 5,
                distance_per_sec: 500,
            },
            keyboard: KeyboardRateLimits {
                key_presses_per_sec: 15,
                chars_per_sec: 20,
            },
            general: GeneralRateLimits {
                actions_per_min: 50,
                burst_limit: 10,
                burst_window_secs: 3,
            },
            enabled: true,
        };

        let rl = RateLimiter::with_config(config);
        assert_eq!(rl.config().mouse.moves_per_sec, 10);
        assert_eq!(rl.config().keyboard.key_presses_per_sec, 15);
    }

    #[test]
    fn test_stats_update() {
        let mut rl = RateLimiter::new();
        
        rl.check(ActionType::MouseMove, 100).expect("ok");
        rl.check(ActionType::MouseMove, 200).expect("ok");
        rl.check(ActionType::MouseClick, 1).expect("ok");
        rl.check(ActionType::KeyPress, 1).expect("ok");
        rl.check(ActionType::TextChar, 5).expect("ok");

        let stats = rl.stats();
        assert_eq!(stats.mouse_moves, 2);
        assert_eq!(stats.mouse_clicks, 1);
        assert_eq!(stats.key_presses, 1);
        assert_eq!(stats.text_chars, 5);
        assert_eq!(stats.total_mouse_distance, 300);
    }

    #[test]
    fn test_record_block() {
        let mut rl = RateLimiter::new();
        
        rl.record_block("Test engelleme");
        
        assert_eq!(rl.stats().blocked_actions, 1);
        assert!(rl.stats().last_block_reason.is_some());
    }

    #[test]
    fn test_reset_stats() {
        let mut rl = RateLimiter::new();
        
        rl.check(ActionType::MouseMove, 100).expect("ok");
        rl.record_block("Test");
        
        rl.reset_stats();
        
        assert_eq!(rl.stats().total_actions, 0);
        assert_eq!(rl.stats().blocked_actions, 0);
    }

    #[test]
    fn test_report() {
        let mut rl = RateLimiter::new();
        
        rl.check(ActionType::MouseMove, 100).expect("ok");
        rl.check(ActionType::MouseClick, 1).expect("ok");
        
        let report = rl.report();
        assert!(report.enabled);
        assert_eq!(report.total_actions, 2);
    }

    #[test]
    fn test_report_json() {
        let rl = RateLimiter::new();
        let report = rl.report();
        
        let json = report.to_json().expect("ok");
        assert!(json.contains("enabled"));
        assert!(json.contains("total_actions"));
    }

    #[test]
    fn test_default_limits() {
        let mouse = MouseRateLimits::default();
        assert_eq!(mouse.moves_per_sec, 50);
        assert_eq!(mouse.clicks_per_sec, 10);
        assert_eq!(mouse.distance_per_sec, 2000);

        let keyboard = KeyboardRateLimits::default();
        assert_eq!(keyboard.key_presses_per_sec, 30);
        assert_eq!(keyboard.chars_per_sec, 50);

        let general = GeneralRateLimits::default();
        assert_eq!(general.actions_per_min, 120);
        assert_eq!(general.burst_limit, 20);
    }
}
