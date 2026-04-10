//! ═══════════════════════════════════════════════════════════════════════════════
//!  HUMAN EMULATION SETTINGS - İnsan Taklidi Ayarları
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Dashboard ve ayarlar sekmesi için Human Emulation paneli.
//! Fare titremesi, yazma hızı, proxy rotasyonu ve Auto-CAPTCHA yönetimi.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Human Emulation ana ayarları
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanEmulationSettings {
    /// İnsan taklidi aktif mi?
    pub enabled: bool,
    /// Fare ayarları
    pub mouse: MouseEmulationSettings,
    /// Yazma ayarları
    pub typing: TypingEmulationSettings,
    /// Proxy ayarları
    pub proxy: ProxyEmulationSettings,
    /// CAPTCHA ayarları
    pub captcha: CaptchaEmulationSettings,
    /// Davranış ayarları
    pub behavior: BehaviorEmulationSettings,
    /// Gelişmiş ayarlar
    pub advanced: AdvancedEmulationSettings,
}

impl Default for HumanEmulationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            mouse: MouseEmulationSettings::default(),
            typing: TypingEmulationSettings::default(),
            proxy: ProxyEmulationSettings::default(),
            captcha: CaptchaEmulationSettings::default(),
            behavior: BehaviorEmulationSettings::default(),
            advanced: AdvancedEmulationSettings::default(),
        }
    }
}

/// Fare emülasyon ayarları
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseEmulationSettings {
    /// Titreme aktif mi?
    pub tremor_enabled: bool,
    /// Titreme genliği (piksel)
    pub tremor_amplitude: f64,
    /// Titreme frekansı
    pub tremor_frequency: f64,
    /// İnsan benzerliği seviyesi (0.0 - 1.0)
    pub humanlikeness: f64,
    /// Hareket hızı faktörü
    pub speed_factor: f64,
    /// Bezier eğri kalitesi
    pub bezier_quality: u32,
    /// Hareket paterni
    pub movement_pattern: MovementPatternSetting,
}

impl Default for MouseEmulationSettings {
    fn default() -> Self {
        Self {
            tremor_enabled: true,
            tremor_amplitude: 1.5,
            tremor_frequency: 8.0,
            humanlikeness: 0.85,
            speed_factor: 1.0,
            bezier_quality: 50,
            movement_pattern: MovementPatternSetting::Natural,
        }
    }
}

/// Hareket paterni seçimi
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MovementPatternSetting {
    Linear,
    Curved,
    Wavy,
    Spiral,
    Zigzag,
    Natural,
}

/// Yazma emülasyon ayarları
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingEmulationSettings {
    /// Yazma hızı (WPM - Words Per Minute)
    pub wpm: u32,
    /// Mesafe faktörü
    pub distance_factor: f64,
    /// Rastgele varyasyon
    pub variation: f64,
    /// Hata olasılığı
    pub error_rate: f64,
    /// Otomatik düzeltme
    pub auto_correction: bool,
    /// Shift gecikmesi (ms)
    pub shift_delay: u64,
    /// Space gecikmesi (ms)
    pub space_delay: u64,
}

impl Default for TypingEmulationSettings {
    fn default() -> Self {
        Self {
            wpm: 45,
            distance_factor: 15.0,
            variation: 0.3,
            error_rate: 0.02,
            auto_correction: true,
            shift_delay: 80,
            space_delay: 100,
        }
    }
}

/// Proxy emülasyon ayarları
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyEmulationSettings {
    /// Proxy rotasyonu aktif mi?
    pub rotation_enabled: bool,
    /// Rotasyon modu
    pub rotation_mode: ProxyRotationMode,
    /// Tercih edilen ülkeler
    pub preferred_countries: Vec<String>,
    /// Minimum başarı oranı
    pub min_success_rate: f64,
    /// Sağlık kontrolü aralığı (saniye)
    pub health_check_interval: u64,
    /// Otomatik blacklist
    pub auto_blacklist: bool,
    /// Proxy listesi
    pub proxy_list: Vec<ProxyEntry>,
}

impl Default for ProxyEmulationSettings {
    fn default() -> Self {
        Self {
            rotation_enabled: false,
            rotation_mode: ProxyRotationMode::RoundRobin,
            preferred_countries: vec!["US".into(), "DE".into(), "GB".into()],
            min_success_rate: 0.8,
            health_check_interval: 300,
            auto_blacklist: true,
            proxy_list: Vec::new(),
        }
    }
}

/// Proxy rotasyon modu
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProxyRotationMode {
    RoundRobin,
    Random,
    LeastUsed,
    Fastest,
    LocationBased,
    WeightedRandom,
}

/// Proxy girişi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyEntry {
    /// Proxy ID
    pub id: String,
    /// Proxy URL
    pub url: String,
    /// Ülke
    pub country: String,
    /// Aktif mi?
    pub active: bool,
}

/// CAPTCHA emülasyon ayarları
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptchaEmulationSettings {
    /// Otomatik çözüm aktif mi?
    pub auto_solve: bool,
    /// Maksimum deneme
    pub max_retries: u32,
    /// Zaman aşımı (ms)
    pub timeout_ms: u64,
    /// Minimum güven eşiği
    pub min_confidence: f64,
    /// Human-like gecikme
    pub human_delay: bool,
    /// API anahtarı (2captcha vb.)
    pub api_key: Option<String>,
    /// Hangi CAPTCHA türleri çözülecek
    pub enabled_types: Vec<CaptchaTypeSetting>,
}

impl Default for CaptchaEmulationSettings {
    fn default() -> Self {
        Self {
            auto_solve: false,
            max_retries: 3,
            timeout_ms: 60000,
            min_confidence: 0.8,
            human_delay: true,
            api_key: None,
            enabled_types: vec![
                CaptchaTypeSetting::RecaptchaV2,
                CaptchaTypeSetting::HCaptcha,
                CaptchaTypeSetting::ImageText,
            ],
        }
    }
}

/// CAPTCHA türü ayarı
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CaptchaTypeSetting {
    RecaptchaV2,
    RecaptchaV3,
    HCaptcha,
    Turnstile,
    ImageText,
    ImageSelect,
    Slider,
    MathCaptcha,
}

/// Davranış emülasyon ayarları
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorEmulationSettings {
    /// Best-of-N algoritması aktif mi?
    pub best_of_n_enabled: bool,
    /// N değeri
    pub best_of_n: usize,
    /// Başarı eşiği
    pub success_threshold: f64,
    /// RNN-LSTM modeli kullanılsın mı?
    pub use_rnn_model: bool,
    /// Keşif oranı
    pub exploration_rate: f64,
    /// Ortalama aksiyon süresi (ms)
    pub avg_action_duration: u64,
    /// Aksiyonlar arası bekleme (ms)
    pub inter_action_delay: u64,
}

impl Default for BehaviorEmulationSettings {
    fn default() -> Self {
        Self {
            best_of_n_enabled: true,
            best_of_n: 5,
            success_threshold: 0.726, // %72.6 hedef
            use_rnn_model: true,
            exploration_rate: 0.1,
            avg_action_duration: 200,
            inter_action_delay: 150,
        }
    }
}

/// Gelişmiş emülasyon ayarları
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedEmulationSettings {
    /// RNN-LSTM gizli katman boyutu
    pub rnn_hidden_size: usize,
    /// Bağlam penceresi
    pub context_window: usize,
    /// Öğrenme oranı
    pub learning_rate: f64,
    /// Hafıza boyutu
    pub memory_size: usize,
    /// Debug modu
    pub debug_mode: bool,
    /// Performans loglaması
    pub performance_logging: bool,
    /// Özel parametreler
    pub custom_params: HashMap<String, f64>,
}

impl Default for AdvancedEmulationSettings {
    fn default() -> Self {
        Self {
            rnn_hidden_size: 64,
            context_window: 10,
            learning_rate: 0.01,
            memory_size: 100,
            debug_mode: false,
            performance_logging: false,
            custom_params: HashMap::new(),
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  HUMAN EMULATION MANAGER
// ─────────────────────────────────────────────────────────────────────────────--

/// Human Emulation yöneticisi
pub struct HumanEmulationManager {
    /// Ayarlar
    settings: HumanEmulationSettings,
    /// Çalışma zamanı istatistikleri
    runtime_stats: RuntimeStats,
}

/// Çalışma zamanı istatistikleri
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuntimeStats {
    /// Toplam fare hareketi
    pub total_mouse_movements: u64,
    /// Toplam yazılan karakter
    pub total_chars_typed: u64,
    /// Toplam CAPTCHA çözümü
    pub total_captchas_solved: u64,
    /// CAPTCHA başarı oranı
    pub captcha_success_rate: f64,
    /// Ortalama WPM
    pub avg_wpm: f64,
    /// Davranış başarı oranı
    pub behavior_success_rate: f64,
    /// Proxy başarı oranı
    pub proxy_success_rate: f64,
    /// Toplam çalışma süresi (saniye)
    pub total_runtime_secs: u64,
}

impl HumanEmulationManager {
    /// Yeni yönetici oluştur
    pub fn new(settings: HumanEmulationSettings) -> Self {
        log::info!("🎭 HUMAN-EMULATION: İnsan taklidi yöneticisi başlatıldı");
        
        Self {
            settings,
            runtime_stats: RuntimeStats::default(),
        }
    }
    
    /// Ayarları getir
    pub fn settings(&self) -> &HumanEmulationSettings {
        &self.settings
    }
    
    /// Ayarları güncelle
    pub fn update_settings(&mut self, settings: HumanEmulationSettings) {
        log::info!("🎭 HUMAN-EMULATION: Ayarlar güncellendi");
        self.settings = settings;
    }
    
    /// Belirli bir ayarı güncelle
    pub fn update_mouse_settings(&mut self, mouse: MouseEmulationSettings) {
        self.settings.mouse = mouse;
    }
    
    pub fn update_typing_settings(&mut self, typing: TypingEmulationSettings) {
        self.settings.typing = typing;
    }
    
    pub fn update_proxy_settings(&mut self, proxy: ProxyEmulationSettings) {
        self.settings.proxy = proxy;
    }
    
    pub fn update_captcha_settings(&mut self, captcha: CaptchaEmulationSettings) {
        self.settings.captcha = captcha;
    }
    
    pub fn update_behavior_settings(&mut self, behavior: BehaviorEmulationSettings) {
        self.settings.behavior = behavior;
    }
    
    /// İstatistikleri güncelle
    pub fn record_mouse_movement(&mut self) {
        self.runtime_stats.total_mouse_movements += 1;
    }
    
    pub fn record_typing(&mut self, char_count: u64) {
        self.runtime_stats.total_chars_typed += char_count;
    }
    
    pub fn record_captcha(&mut self, success: bool) {
        self.runtime_stats.total_captchas_solved += 1;
        
        let total = self.runtime_stats.total_captchas_solved;
        let successful = if success {
            (self.runtime_stats.captcha_success_rate * (total - 1) as f64 + 1.0) / total as f64
        } else {
            (self.runtime_stats.captcha_success_rate * (total - 1) as f64) / total as f64
        };
        self.runtime_stats.captcha_success_rate = successful;
    }
    
    /// İstatistikleri getir
    pub fn stats(&self) -> &RuntimeStats {
        &self.runtime_stats
    }
    
    /// İstatistikleri sıfırla
    pub fn reset_stats(&mut self) {
        self.runtime_stats = RuntimeStats::default();
    }
    
    /// JSON olarak dışa aktar
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self.settings)
    }
    
    /// JSON'dan içe aktar
    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        let settings: HumanEmulationSettings = serde_json::from_str(json)?;
        Ok(Self::new(settings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_settings() {
        let settings = HumanEmulationSettings::default();
        assert!(settings.enabled);
        assert!(settings.mouse.tremor_enabled);
        assert_eq!(settings.typing.wpm, 45);
    }
    
    #[test]
    fn test_manager_creation() {
        let manager = HumanEmulationManager::new(HumanEmulationSettings::default());
        assert!(manager.settings().enabled);
    }
    
    #[test]
    fn test_update_mouse_settings() {
        let mut manager = HumanEmulationManager::new(HumanEmulationSettings::default());
        
        let mut mouse = MouseEmulationSettings::default();
        mouse.tremor_amplitude = 3.0;
        
        manager.update_mouse_settings(mouse);
        
        assert_eq!(manager.settings().mouse.tremor_amplitude, 3.0);
    }
    
    #[test]
    fn test_record_stats() {
        let mut manager = HumanEmulationManager::new(HumanEmulationSettings::default());
        
        manager.record_mouse_movement();
        manager.record_typing(10);
        manager.record_captcha(true);
        manager.record_captcha(false);
        
        assert_eq!(manager.stats().total_mouse_movements, 1);
        assert_eq!(manager.stats().total_chars_typed, 10);
        assert_eq!(manager.stats().total_captchas_solved, 2);
        assert!((manager.stats().captcha_success_rate - 0.5).abs() < 0.01);
    }
    
    #[test]
    fn test_json_export_import() {
        let settings = HumanEmulationSettings::default();
        let manager = HumanEmulationManager::new(settings);
        
        let json = manager.to_json().expect("operation failed");
        let imported = HumanEmulationManager::from_json(&json).expect("operation failed");
        
        assert_eq!(imported.settings().typing.wpm, 45);
    }
}
