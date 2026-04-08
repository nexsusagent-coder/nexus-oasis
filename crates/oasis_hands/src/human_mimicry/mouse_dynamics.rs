//! ═══════════════════════════════════════════════════════════════════════════════
//!  MOUSE DYNAMICS - Fare Hareket Dinamiği
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! İnsan fare hareket simülasyonu:
//! - Titreşim (tremor) simülasyonu
//! - Hız varyasyonu
//! - Fitts yasası modellenmesi
//! - Doğal hızlanma/yavaşlama

use rand::Rng;
use serde::{Deserialize, Serialize};

/// Titreşim yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TremorConfig {
    /// Titreşim aktif mi?
    pub enabled: bool,
    /// Titreşim genliği (piksel)
    pub amplitude: f64,
    /// Titreşim frekansı
    pub frequency: f64,
    /// Rastgele varyasyon
    pub randomness: f64,
}

impl Default for TremorConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            amplitude: 1.5,
            frequency: 8.0,
            randomness: 0.3,
        }
    }
}

/// Fare profili
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseProfile {
    /// Titreşim yapılandırması
    pub tremor: TremorConfig,
    /// Hız katsayısı
    pub speed_factor: f64,
    /// İnsan benzerliği (0.0 - 1.0)
    pub humanlikeness: f64,
    /// Fitts yasası katsayısı
    pub fitts_coefficient: f64,
    /// Minimum hareket süresi (ms)
    pub min_duration: u64,
    /// Maksimum hareket süresi (ms)
    pub max_duration: u64,
    /// Click öncesi bekleme (ms)
    pub pre_click_delay: u64,
    /// Click sonrası bekleme (ms)
    pub post_click_delay: u64,
}

impl Default for MouseProfile {
    fn default() -> Self {
        Self {
            tremor: TremorConfig::default(),
            speed_factor: 1.0,
            humanlikeness: 0.85,
            fitts_coefficient: 0.1,
            min_duration: 100,
            max_duration: 2000,
            pre_click_delay: 50,
            post_click_delay: 80,
        }
    }
}

/// Fare dinamiği motoru
pub struct MouseDynamics {
    /// Profil
    profile: MouseProfile,
    /// Mevcut pozisyon
    current_pos: (f64, f64),
    /// Zaman sayacı (titreşim için)
    time_counter: f64,
}

impl MouseDynamics {
    /// Yeni fare dinamiği oluştur
    pub fn new(enable_tremor: bool, humanlikeness: f64) -> Self {
        let mut profile = MouseProfile::default();
        profile.tremor.enabled = enable_tremor;
        profile.humanlikeness = humanlikeness;
        
        Self {
            profile,
            current_pos: (0.0, 0.0),
            time_counter: 0.0,
        }
    }
    
    /// Titreşim aktif/pasif
    pub fn set_tremor_enabled(&mut self, enabled: bool) {
        self.profile.tremor.enabled = enabled;
    }
    
    /// İnsan benzerliği ayarla
    pub fn set_humanlikeness(&mut self, value: f64) {
        self.profile.humanlikeness = value.clamp(0.0, 1.0);
    }
    
    /// Yola titreşim ekle
    pub fn add_tremor(&self, path: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
        if !self.profile.tremor.enabled {
            return path;
        }
        
        let mut rng = rand::thread_rng();
        let mut result = Vec::with_capacity(path.len());
        
        for (i, (x, y)) in path.into_iter().enumerate() {
            let t = i as f64 * self.profile.tremor.frequency * 0.01;
            
            // Sinüzoidal titreşim + rastgele gürültü
            let tremor_x = (t * std::f64::consts::TAU).sin() * self.profile.tremor.amplitude * 0.5
                + (rng.gen::<f64>() - 0.5) * self.profile.tremor.amplitude * self.profile.tremor.randomness;
            
            let tremor_y = (t * std::f64::consts::TAU * 1.3).sin() * self.profile.tremor.amplitude * 0.5
                + (rng.gen::<f64>() - 0.5) * self.profile.tremor.amplitude * self.profile.tremor.randomness;
            
            result.push((x + tremor_x, y + tremor_y));
        }
        
        result
    }
    
    /// Hız varyasyonu ekle (zaman damgaları)
    pub fn add_velocity_variation(&self, path: Vec<(f64, f64)>) -> Vec<(f64, f64, u64)> {
        if path.len() < 2 {
            return path.into_iter().map(|(x, y)| (x, y, 0)).collect();
        }
        
        let path_len = path.len();
        let mut rng = rand::thread_rng();
        let mut result = Vec::with_capacity(path_len);
        let mut cumulative_time = 0u64;
        
        for (i, (x, y)) in path.into_iter().enumerate() {
            if i == 0 {
                result.push((x, y, 0));
                continue;
            }
            
            // Önceki nokta ile mesafe
            let (prev_x, prev_y, _) = result[i - 1];
            let dx = x - prev_x;
            let dy = y - prev_y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            // Fitts yasası: MT = a + b * log2(D/W + 1)
            // Basitleştirilmiş: süre = k * distance^0.5
            let base_time = self.profile.fitts_coefficient * distance.sqrt() * 10.0;
            
            // İnsan benzerliği varyasyonu
            let variation = 1.0 + (rng.gen::<f64>() - 0.5) * (1.0 - self.profile.humanlikeness) * 0.5;
            
            // Hızlanma profili (bell curve)
            let progress = i as f64 / path_len as f64;
            let acceleration_factor = 1.0 - 4.0 * (progress - 0.5).powi(2);
            let speed_factor = 1.0 + acceleration_factor * 0.3;
            
            // Her noktada yavaşlama (insanlar hedefe yaklaşırken yavaşlar)
            let deceleration_factor = if progress > 0.7 {
                1.0 + (progress - 0.7) * 2.0
            } else {
                1.0
            };
            
            let segment_time = (base_time * variation / speed_factor * deceleration_factor)
                .max(self.profile.min_duration as f64 / 10.0)
                .min(self.profile.max_duration as f64 / 10.0) as u64;
            
            cumulative_time += segment_time;
            result.push((x, y, cumulative_time));
        }
        
        result
    }
    
    /// Fitts yasası ile hareket süresi hesapla
    pub fn fitts_duration(&self, distance: f64, target_width: f64) -> u64 {
        // MT = a + b * log2(D/W + 1)
        let a = 50.0; // Intercept (ms)
        let b = self.profile.fitts_coefficient * 100.0;
        
        let mt = a + b * (distance / target_width + 1.0).log2();
        
        let mut rng = rand::thread_rng();
        let variation = 1.0 + (rng.gen::<f64>() - 0.5) * 0.2;
        
        (mt * variation).max(self.profile.min_duration as f64).min(self.profile.max_duration as f64) as u64
    }
    
    /// Click gecikmesi oluştur
    pub fn click_delay(&self) -> (u64, u64) {
        let mut rng = rand::thread_rng();
        
        let pre = self.profile.pre_click_delay as f64 
            * (1.0 + (rng.gen::<f64>() - 0.5) * 0.4);
        
        let post = self.profile.post_click_delay as f64
            * (1.0 + (rng.gen::<f64>() - 0.5) * 0.4);
        
        (pre as u64, post as u64)
    }
    
    /// Rastgele fare hareketi (canlılık için)
    pub fn idle_movement(&self) -> (f64, f64) {
        let mut rng = rand::thread_rng();
        
        let dx = (rng.gen::<f64>() - 0.5) * self.profile.tremor.amplitude * 2.0;
        let dy = (rng.gen::<f64>() - 0.5) * self.profile.tremor.amplitude * 2.0;
        
        (dx, dy)
    }
    
    /// Profil getir
    pub fn profile(&self) -> &MouseProfile {
        &self.profile
    }
    
    /// Profil ayarla
    pub fn set_profile(&mut self, profile: MouseProfile) {
        self.profile = profile;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tremor_config_default() {
        let config = TremorConfig::default();
        assert!(config.enabled);
        assert!(config.amplitude > 0.0);
    }
    
    #[test]
    fn test_mouse_profile_default() {
        let profile = MouseProfile::default();
        assert!(profile.humanlikeness > 0.0 && profile.humanlikeness <= 1.0);
    }
    
    #[test]
    fn test_mouse_dynamics_creation() {
        let md = MouseDynamics::new(true, 0.85);
        assert!(md.profile.tremor.enabled);
    }
    
    #[test]
    fn test_add_tremor() {
        let md = MouseDynamics::new(true, 0.85);
        let path = vec![(0.0, 0.0), (100.0, 100.0)];
        let tremored = md.add_tremor(path);
        
        // Titreşim eklenmiş olmalı
        assert_ne!(tremored[0].0, 0.0);
    }
    
    #[test]
    fn test_add_velocity_variation() {
        let md = MouseDynamics::new(true, 0.85);
        let path = vec![(0.0, 0.0), (50.0, 50.0), (100.0, 100.0)];
        let timed = md.add_velocity_variation(path);
        
        assert_eq!(timed.len(), 3);
        assert_eq!(timed[0].2, 0);
        assert!(timed[1].2 > 0);
        assert!(timed[2].2 > timed[1].2);
    }
    
    #[test]
    fn test_fitts_duration() {
        let md = MouseDynamics::new(true, 0.85);
        
        // Kısa mesafe
        let short = md.fitts_duration(10.0, 10.0);
        
        // Uzun mesafe
        let long = md.fitts_duration(1000.0, 10.0);
        
        assert!(short < long);
    }
    
    #[test]
    fn test_click_delay() {
        let md = MouseDynamics::new(true, 0.85);
        let (pre, post) = md.click_delay();
        
        assert!(pre > 0);
        assert!(post > 0);
    }
}
