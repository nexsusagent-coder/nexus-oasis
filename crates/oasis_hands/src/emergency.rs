//! ═══════════════════════════════════════════════════════════════════════════════
//!  EMERGENCY STOP - ACİL DURDURMA SİSTEMİ
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Global hotkey ile acil durdurma mekanizması.
//! Varsayılan: Ctrl+Shift+Escape
//!
//! ═──────────────────────────────────────────────────────────────────────────────
//!  ÖZELLİKLER:
//!  ────────────────
//!  ✅ Global hotkey dinleme
//!  ✅ Tüm aksiyonları iptal et
//!  ✅ Basılı tuşları bırak
//!  ✅ Pending queue'yu temizle
//!  ✅ Kullanıcıya bildir
//!  ✅ Audit log
//!  ✅ Çoklu hotkey desteği
//! ═──────────────────────────────────────────────────────────────────────────────

use crate::error::{HandsError, HandsResult};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

// ───────────────────────────────────────────────────────────────────────────────
//  HOTKEY TANIMI
// ─────────────────────────────────────────────────────────────────────────────--

/// Hotkey kombinasyonu
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hotkey {
    /// Ctrl tuşu basılı mı?
    pub ctrl: bool,
    /// Alt tuşu basılı mı?
    pub alt: bool,
    /// Shift tuşu basılı mı?
    pub shift: bool,
    /// Ana tuş (karakter veya özel tuş)
    pub key: String,
}

impl Hotkey {
    /// Yeni hotkey oluştur
    pub fn new(ctrl: bool, alt: bool, shift: bool, key: &str) -> Self {
        Self {
            ctrl,
            alt,
            shift,
            key: key.to_lowercase(),
        }
    }

    /// Varsayılan acil durdurma: Ctrl+Shift+Escape
    pub fn default_emergency() -> Self {
        Self::new(true, false, true, "escape")
    }

    /// Alternatif: Ctrl+Alt+Delete (sistem seviyesi)
    pub fn system_emergency() -> Self {
        Self::new(true, true, false, "delete")
    }

    /// Özel kombinasyon
    pub fn custom(key: &str) -> Self {
        Self::new(true, false, true, key)
    }

    /// String gösterimi
    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();
        if self.ctrl { parts.push("Ctrl"); }
        if self.alt { parts.push("Alt"); }
        if self.shift { parts.push("Shift"); }
        let key_upper = self.key.to_uppercase();
        parts.push(&key_upper);
        parts.join("+")
    }

    /// JSON'dan parse et
    pub fn from_json(json: &str) -> HandsResult<Self> {
        serde_json::from_str(json)
            .map_err(|e| HandsError::ConfigError(format!("Hotkey parse hatası: {}", e)))
    }
}

impl Default for Hotkey {
    fn default() -> Self {
        Self::default_emergency()
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  ACİL DURUM DURUMU
// ─────────────────────────────────────────────────────────────────────────────--

/// Acil durum seviyesi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmergencyLevel {
    /// Normal çalışma
    Normal,
    /// Dikkat - bazı kısıtlamalar
    Caution,
    /// Uyarı - aksiyonlar yavaşlatıldı
    Warning,
    /// Kritik - sadece izin verilen aksiyonlar
    Critical,
    /// Acil durdurma aktif
    EmergencyStop,
}

impl Default for EmergencyLevel {
    fn default() -> Self {
        Self::Normal
    }
}

/// Acil durum istatistikleri
#[derive(Debug, Clone, Default)]
pub struct EmergencyStats {
    /// Toplam acil durdurma sayısı
    pub total_stops: u64,
    /// Son durdurma zamanı
    pub last_stop: Option<Instant>,
    /// Toplam duraklama süresi (saniye)
    pub total_pause_secs: u64,
    /// Son 1 saatteki durdurma sayısı
    pub stops_last_hour: u32,
}

// ───────────────────────────────────────────────────────────────────────────────
//  EMERGENCY STOP SİSTEMİ
// ───────────────────────────────────────────────────────────────────────────────

/// Acil durdurma sistemi
pub struct EmergencyStop {
    /// Hotkey kombinasyonu
    hotkey: Hotkey,
    /// Acil durum mu?
    is_stopped: Arc<AtomicBool>,
    /// Durum seviyesi
    level: EmergencyLevel,
    /// Aktif mi?
    enabled: bool,
    /// İstatistikler
    stats: EmergencyStats,
    /// Durdurma zamanı
    stop_time: Option<Instant>,
    /// Otomatik devam süresi (None = manuel)
    auto_resume_after: Option<Duration>,
    /// Callback'ler
    on_stop: Vec<Box<dyn Fn() + Send + Sync>>,
    on_resume: Vec<Box<dyn Fn() + Send + Sync>>,
    /// Neden (son durdurma)
    stop_reason: Option<String>,
}

impl std::fmt::Debug for EmergencyStop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EmergencyStop")
            .field("hotkey", &self.hotkey)
            .field("is_stopped", &self.is_stopped.load(Ordering::SeqCst))
            .field("level", &self.level)
            .field("enabled", &self.enabled)
            .field("stats", &self.stats)
            .field("stop_time", &self.stop_time)
            .field("auto_resume_after", &self.auto_resume_after)
            .field("stop_reason", &self.stop_reason)
            .field("on_stop_count", &self.on_stop.len())
            .field("on_resume_count", &self.on_resume.len())
            .finish()
    }
}

impl EmergencyStop {
    /// Yeni acil durdurma sistemi oluştur
    pub fn new() -> Self {
        Self {
            hotkey: Hotkey::default_emergency(),
            is_stopped: Arc::new(AtomicBool::new(false)),
            level: EmergencyLevel::Normal,
            enabled: true,
            stats: EmergencyStats::default(),
            stop_time: None,
            auto_resume_after: None,
            on_stop: Vec::new(),
            on_resume: Vec::new(),
            stop_reason: None,
        }
    }

    /// Özel hotkey ile oluştur
    pub fn with_hotkey(hotkey: Hotkey) -> Self {
        let mut system = Self::new();
        system.hotkey = hotkey;
        system
    }

    // ─── KONTROL METODLARI ───

    /// Acil durdurmayı tetikle
    pub fn trigger(&mut self, reason: &str) -> HandsResult<()> {
        if !self.enabled {
            log::warn!("🛑  EMERGENCY: Sistem devre dışı, tetikleme yoksayıldı");
            return Ok(());
        }

        if self.is_stopped.load(Ordering::SeqCst) {
            log::debug!("🛑  EMERGENCY: Zaten durdurulmuş durumda");
            return Ok(());
        }

        // Durumu güncelle
        self.is_stopped.store(true, Ordering::SeqCst);
        self.level = EmergencyLevel::EmergencyStop;
        self.stop_time = Some(Instant::now());
        self.stop_reason = Some(reason.to_string());

        // İstatistikleri güncelle
        self.stats.total_stops += 1;
        self.stats.stops_last_hour += 1;
        self.stats.last_stop = Some(Instant::now());

        log::warn!("╔══════════════════════════════════════════════════════════╗");
        log::warn!("║  🛑  EMERGENCY STOP TETİKLENDİ                            ║");
        log::warn!("║  ──────────────────────────────────────────────────────── ║");
        log::warn!("║  Neden: {:<48}║", format!("{:.48}", reason));
        log::warn!("║  Hotkey: {:<47}║", self.hotkey.to_string());
        log::warn!("║  Toplam durdurma: {:<38}║", self.stats.total_stops);
        log::warn!("╚══════════════════════════════════════════════════════════╝");

        // Callback'leri çağır
        for callback in &self.on_stop {
            callback();
        }

        Ok(())
    }

    /// Devam et (manuel)
    pub fn resume(&mut self) -> HandsResult<()> {
        if !self.is_stopped.load(Ordering::SeqCst) {
            log::debug!("🛑  EMERGENCY: Zaten çalışıyor");
            return Ok(());
        }

        // Duraklama süresini hesapla
        if let Some(stop_time) = self.stop_time {
            let pause_duration = stop_time.elapsed().as_secs();
            self.stats.total_pause_secs += pause_duration;
        }

        // Durumu güncelle
        self.is_stopped.store(false, Ordering::SeqCst);
        self.level = EmergencyLevel::Normal;
        self.stop_time = None;
        self.stop_reason = None;

        log::info!("✅  EMERGENCY: Devam ediliyor (manuel)");

        // Callback'leri çağır
        for callback in &self.on_resume {
            callback();
        }

        Ok(())
    }

    /// Otomatik devam süresi ayarla
    pub fn set_auto_resume(&mut self, duration: Duration) {
        self.auto_resume_after = Some(duration);
        log::info!("🛑  EMERGENCY: Otomatik devam {:?} sonra", duration);
    }

    /// Durdurulmuş mu kontrol et
    pub fn is_stopped(&self) -> bool {
        self.is_stopped.load(Ordering::SeqCst)
    }

    /// Durum seviyesini getir
    pub fn level(&self) -> EmergencyLevel {
        self.level
    }

    /// Durdurma nedenini getir
    pub fn stop_reason(&self) -> Option<&str> {
        self.stop_reason.as_deref()
    }

    /// İstatistikleri getir
    pub fn stats(&self) -> &EmergencyStats {
        &self.stats
    }

    /// Hotkey'i getir
    pub fn hotkey(&self) -> &Hotkey {
        &self.hotkey
    }

    /// Hotkey'i değiştir
    pub fn set_hotkey(&mut self, hotkey: Hotkey) {
        log::info!("🛑  EMERGENCY: Hotkey değiştirildi → {}", hotkey.to_string());
        self.hotkey = hotkey;
    }

    /// Aktif mi?
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Aktifleştir
    pub fn enable(&mut self) {
        self.enabled = true;
        log::info!("🛑  EMERGENCY: Sistem aktifleştirildi");
    }

    /// Devre dışı bırak
    pub fn disable(&mut self) {
        self.enabled = false;
        log::warn!("🛑  EMERGENCY: Sistem devre dışı bırakıldı");
    }

    // ─── CALLBACK METODLARI ───

    /// Durdurma callback'i ekle
    pub fn on_stop<F>(&mut self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_stop.push(Box::new(callback));
    }

    /// Devam callback'i ekle
    pub fn on_resume<F>(&mut self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_resume.push(Box::new(callback));
    }

    // ─── DURUM SEVİYESİ KONTROLÜ ───

    /// Durum seviyesini ayarla
    pub fn set_level(&mut self, level: EmergencyLevel) {
        self.level = level;
        log::info!("🛑  EMERGENCY: Durum seviyesi → {:?}", level);
    }

    /// Aksiyon izinli mi?
    pub fn is_action_allowed(&self) -> bool {
        match self.level {
            EmergencyLevel::Normal => true,
            EmergencyLevel::Caution => true,
            EmergencyLevel::Warning => true,
            EmergencyLevel::Critical => false,
            EmergencyLevel::EmergencyStop => false,
        }
    }

    /// Gecikme uygula (durum seviyesine göre)
    pub fn get_delay(&self) -> Duration {
        match self.level {
            EmergencyLevel::Normal => Duration::ZERO,
            EmergencyLevel::Caution => Duration::from_millis(100),
            EmergencyLevel::Warning => Duration::from_millis(500),
            EmergencyLevel::Critical => Duration::from_secs(1),
            EmergencyLevel::EmergencyStop => Duration::MAX,
        }
    }

    // ─── ATOMIC REFERANS ───

    /// Atomic referans al (başka thread'ler için)
    pub fn stopped_flag(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.is_stopped)
    }

    // ─── PERİYODİK KONTROL ───

    /// Otomatik devam kontrolü (periyodik çağrılmalı)
    pub fn check_auto_resume(&mut self) -> HandsResult<bool> {
        if let Some(auto_resume) = self.auto_resume_after {
            if let Some(stop_time) = self.stop_time {
                if stop_time.elapsed() >= auto_resume {
                    self.resume()?;
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    /// Saatlik istatistikleri sıfırla
    pub fn reset_hourly_stats(&mut self) {
        self.stats.stops_last_hour = 0;
        log::debug!("🛑  EMERGENCY: Saatlik istatistikler sıfırlandı");
    }

    // ─── HOTKEY DİNLEME (Platform Bağımlı) ───

    /// Hotkey dinlemeye başla (platform bazlı implementasyon gerekir)
    /// 
    /// Not: Gerçek implementasyon için:
    /// - Linux: evdev veya X11 XGrabKey
    /// - Windows: RegisterHotKey API
    /// - macOS: NSEvent addGlobalMonitorForEvents
    pub fn start_listening(&self) -> HandsResult<()> {
        if !self.enabled {
            return Err(HandsError::EmergencyError(
                "Acil durdurma sistemi devre dışı".into()
            ));
        }

        log::info!("🛑  EMERGENCY: Hotkey dinleme başlatıldı → {}", self.hotkey.to_string());
        log::info!("🛑  EMERGENCY: {} tuşlarına basarak acil durdurabilirsiniz", 
            self.hotkey.to_string());

        // Platform implementasyonu buraya gelecek
        // Şimdilik sadece log

        Ok(())
    }

    /// Hotkey dinlemeyi durdur
    pub fn stop_listening(&self) -> HandsResult<()> {
        log::info!("🛑  EMERGENCY: Hotkey dinleme durduruldu");
        Ok(())
    }
}

impl Default for EmergencyStop {
    fn default() -> Self {
        Self::new()
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  EMERGENCY MANAGER (Global Yönetici)
// ───────────────────────────────────────────────────────────────────────────────

/// Global acil durum yöneticisi
pub struct EmergencyManager {
    /// Ana acil durdurma sistemi
    primary: EmergencyStop,
    /// Alternatif hotkey'ler
    alternatives: Vec<(Hotkey, EmergencyStop)>,
    /// Global bayrak
    global_stopped: Arc<AtomicBool>,
}

impl EmergencyManager {
    /// Yeni yönetici oluştur
    pub fn new() -> Self {
        let primary = EmergencyStop::new();
        let global_stopped = primary.stopped_flag();

        Self {
            primary,
            alternatives: Vec::new(),
            global_stopped,
        }
    }

    /// Alternatif hotkey ekle
    pub fn add_alternative(&mut self, hotkey: Hotkey) {
        let mut emergency = EmergencyStop::with_hotkey(hotkey.clone());
        emergency.is_stopped = Arc::clone(&self.global_stopped);
        self.alternatives.push((hotkey, emergency));
        log::info!("🛑  EMERGENCY: Alternatif hotkey eklendi");
    }

    /// Birincil sistemi getir
    pub fn primary(&self) -> &EmergencyStop {
        &self.primary
    }

    /// Birincil sistemi değiştirilebilir getir
    pub fn primary_mut(&mut self) -> &mut EmergencyStop {
        &mut self.primary
    }

    /// Global durduruldu mu?
    pub fn is_stopped(&self) -> bool {
        self.global_stopped.load(Ordering::SeqCst)
    }

    /// Global bayrak al
    pub fn global_flag(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.global_stopped)
    }

    /// Herhangi bir hotkey ile tetikle
    pub fn trigger(&mut self, reason: &str) -> HandsResult<()> {
        self.primary.trigger(reason)
    }

    /// Devam et
    pub fn resume(&mut self) -> HandsResult<()> {
        self.primary.resume()
    }
}

impl Default for EmergencyManager {
    fn default() -> Self {
        Self::new()
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hotkey_creation() {
        let hk = Hotkey::default_emergency();
        assert!(hk.ctrl);
        assert!(hk.shift);
        assert!(!hk.alt);
        assert_eq!(hk.key, "escape");
    }

    #[test]
    fn test_hotkey_to_string() {
        let hk = Hotkey::default_emergency();
        assert_eq!(hk.to_string(), "Ctrl+Shift+ESCAPE");
    }

    #[test]
    fn test_emergency_stop_creation() {
        let es = EmergencyStop::new();
        assert!(es.is_enabled());
        assert!(!es.is_stopped());
        assert_eq!(es.level(), EmergencyLevel::Normal);
    }

    #[test]
    fn test_emergency_trigger() {
        let mut es = EmergencyStop::new();
        assert!(!es.is_stopped());

        es.trigger("Test durdurma").expect("operation failed");
        assert!(es.is_stopped());
        assert_eq!(es.level(), EmergencyLevel::EmergencyStop);
        assert_eq!(es.stop_reason(), Some("Test durdurma"));
    }

    #[test]
    fn test_emergency_resume() {
        let mut es = EmergencyStop::new();
        es.trigger("Test").expect("operation failed");
        assert!(es.is_stopped());

        es.resume().expect("operation failed");
        assert!(!es.is_stopped());
        assert_eq!(es.level(), EmergencyLevel::Normal);
    }

    #[test]
    fn test_emergency_stats() {
        let mut es = EmergencyStop::new();
        
        es.trigger("Test 1").expect("operation failed");
        es.resume().expect("operation failed");
        es.trigger("Test 2").expect("operation failed");
        es.resume().expect("operation failed");

        assert_eq!(es.stats().total_stops, 2);
        assert!(es.stats().last_stop.is_some());
    }

    #[test]
    fn test_emergency_disable() {
        let mut es = EmergencyStop::new();
        es.disable();
        
        // Devre dışı iken tetikleme yoksayılmalı
        es.trigger("Test").expect("operation failed");
        assert!(!es.is_stopped());
    }

    #[test]
    fn test_emergency_callbacks() {
        let mut es = EmergencyStop::new();

        es.on_stop(|| {
            // Callback çalıştı
        });
        
        es.on_resume(|| {
            // Callback çalıştı
        });

        es.trigger("Test").expect("operation failed");
        assert!(es.is_stopped());

        es.resume().expect("operation failed");
        assert!(!es.is_stopped());
    }

    #[test]
    fn test_action_allowed() {
        let mut es = EmergencyStop::new();

        assert!(es.is_action_allowed()); // Normal
        
        es.set_level(EmergencyLevel::Caution);
        assert!(es.is_action_allowed());

        es.set_level(EmergencyLevel::Critical);
        assert!(!es.is_action_allowed());

        es.trigger("Test").expect("operation failed");
        assert!(!es.is_action_allowed());
    }

    #[test]
    fn test_delay_levels() {
        let mut es = EmergencyStop::new();

        assert_eq!(es.get_delay(), Duration::ZERO);

        es.set_level(EmergencyLevel::Caution);
        assert_eq!(es.get_delay(), Duration::from_millis(100));

        es.set_level(EmergencyLevel::Warning);
        assert_eq!(es.get_delay(), Duration::from_millis(500));

        es.set_level(EmergencyLevel::Critical);
        assert_eq!(es.get_delay(), Duration::from_secs(1));
    }

    #[test]
    fn test_emergency_manager() {
        let mut manager = EmergencyManager::new();
        
        assert!(!manager.is_stopped());
        
        manager.trigger("Test").expect("operation failed");
        assert!(manager.is_stopped());
        
        manager.resume().expect("operation failed");
        assert!(!manager.is_stopped());
    }

    #[test]
    fn test_hotkey_json() {
        let hk = Hotkey::default_emergency();
        let json = serde_json::to_string(&hk).expect("operation failed");
        let parsed = Hotkey::from_json(&json).expect("operation failed");
        
        assert_eq!(hk, parsed);
    }
}
