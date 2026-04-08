//! ═══════════════════════════════════════════════════════════════════════════════
//!  HUMAN MIMICRY ENGINE - İnsan Taklidi Sistemi
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Bumblebee ve typerr kütüphanelerinin Rust asimilasyonu.
//! Bezier eğrileri, RNN-LSTM modelleri ve fiziksel tuş mesafesi optimizasyonu.
//!
//! KAYNAKLAR:
//! - Bumblebee: RNN-LSTM tabanlı fare hareketi simülasyonu
//! - typerr: Fiziksel klavye mesafesine göre yazma hızı optimizasyonu
//! - Bezier Curves: Doğal fare yörüngeleri için matematiksel model

pub mod bezier;
pub mod typing_dynamics;
pub mod mouse_dynamics;
pub mod behavior_model;
pub mod bumblebee;

pub use bezier::{BezierCurve, BezierPoint, CubicBezier};
pub use typing_dynamics::{TypingDynamics, TypingProfile, KeyDistance};
pub use mouse_dynamics::{MouseDynamics, MouseProfile, TremorConfig};
pub use behavior_model::{BehaviorModel, BehaviorProfile, ActionType};
pub use bumblebee::{BumblebeeEngine, BumblebeeConfig, MovementPattern};

use serde::{Deserialize, Serialize};

/// Human Mimicry Engine - Ana motor
pub struct HumanMimicryEngine {
    /// Bezier eğri motoru
    bezier_engine: bezier::BezierEngine,
    /// Yazma dinamiği
    typing: typing_dynamics::TypingDynamics,
    /// Fare dinamiği
    mouse: mouse_dynamics::MouseDynamics,
    /// Davranış modeli
    behavior: behavior_model::BehaviorModel,
    /// Yapılandırma
    config: HumanMimicryConfig,
}

/// Human Mimicry yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanMimicryConfig {
    /// Fare titremesi aktif mi?
    pub enable_tremor: bool,
    /// Yazma hızı (WPM - Words Per Minute)
    pub typing_wpm: u32,
    /// Bezier eğri kalitesi (nokta sayısı)
    pub bezier_quality: u32,
    /// RNN-LSTM modeli kullanılsın mı?
    pub use_rnn_model: bool,
    /// İnsan benzerliği seviyesi (0.0 - 1.0)
    pub humanlikeness: f64,
    /// Proxy rotasyonu aktif mi?
    pub proxy_rotation: bool,
    /// Auto-CAPTCHA aktif mi?
    pub auto_captcha: bool,
}

impl Default for HumanMimicryConfig {
    fn default() -> Self {
        Self {
            enable_tremor: true,
            typing_wpm: 45, // Ortalama insan yazma hızı
            bezier_quality: 50,
            use_rnn_model: true,
            humanlikeness: 0.85,
            proxy_rotation: false,
            auto_captcha: false,
        }
    }
}

impl HumanMimicryEngine {
    /// Yeni motor oluştur
    pub fn new(config: HumanMimicryConfig) -> Self {
        log::info!("🎭 HUMAN-MIMICRY: İnsan taklidi motoru başlatılıyor...");
        
        let bezier_engine = bezier::BezierEngine::new(config.bezier_quality);
        let typing = typing_dynamics::TypingDynamics::new(config.typing_wpm);
        let mouse = mouse_dynamics::MouseDynamics::new(
            config.enable_tremor,
            config.humanlikeness,
        );
        let behavior = behavior_model::BehaviorModel::new(config.use_rnn_model);
        
        log::info!("🎭 HUMAN-MIMICRY: WPM: {}, Bezier Kalite: {}, İnsan Benzerliği: {:.0}%",
            config.typing_wpm, config.bezier_quality, config.humanlikeness * 100.0);
        
        Self {
            bezier_engine,
            typing,
            mouse,
            behavior,
            config,
        }
    }
    
    /// İnsan benzeri fare yolu oluştur
    pub fn generate_mouse_path(
        &self,
        from: (f64, f64),
        to: (f64, f64),
    ) -> Vec<(f64, f64, u64)> {
        // Bezier eğrisi ile temel yol
        let bezier_path = self.bezier_engine.generate_path(from, to);
        
        // Titreşim ekle
        let path_with_tremor = self.mouse.add_tremor(bezier_path);
        
        // Hız varyasyonu ekle (zaman damgaları)
        self.mouse.add_velocity_variation(path_with_tremor)
    }
    
    /// İnsan benzeri yazma gecikmeleri oluştur
    pub fn generate_typing_delays(&mut self, text: &str) -> Vec<(char, u64)> {
        self.typing.generate_delays(text)
    }
    
    /// Davranış analizi yap
    pub fn analyze_behavior(&self, actions: &[ActionType]) -> f64 {
        self.behavior.calculate_human_score(actions)
    }
    
    /// Rastgele insan benzeri bekleme süresi
    pub fn human_delay(&self) -> u64 {
        self.behavior.random_delay()
    }
    
    /// Yapılandırmayı güncelle
    pub fn update_config(&mut self, config: HumanMimicryConfig) {
        self.config = config.clone();
        self.typing.set_wpm(config.typing_wpm);
        self.mouse.set_tremor_enabled(config.enable_tremor);
        self.mouse.set_humanlikeness(config.humanlikeness);
        log::info!("🎭 HUMAN-MIMICRY: Yapılandırma güncellendi");
    }
    
    /// Mevcut yapılandırma
    pub fn config(&self) -> &HumanMimicryConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = HumanMimicryConfig::default();
        assert!(config.enable_tremor);
        assert_eq!(config.typing_wpm, 45);
        assert!(config.use_rnn_model);
    }
    
    #[test]
    fn test_engine_creation() {
        let engine = HumanMimicryEngine::new(HumanMimicryConfig::default());
        let path = engine.generate_mouse_path((0.0, 0.0), (100.0, 100.0));
        assert!(!path.is_empty());
    }
    
    #[test]
    fn test_typing_delays() {
        let mut engine = HumanMimicryEngine::new(HumanMimicryConfig::default());
        let delays = engine.generate_typing_delays("Hello");
        assert_eq!(delays.len(), 5);
    }
}
