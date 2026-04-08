//! ═══════════════════════════════════════════════════════════════════════════════
//!  TYPING DYNAMICS - typerr Kütüphanesi Asimilasyonu
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Fiziksel klavye tuş mesafesine göre yazma hızı optimizasyonu.
//! RNN-LSTM modeli ile doğal yazma paternleri.
//!
//! TYPERR KONSEPTI:
//! - Tuşlar arası fiziksel mesafe hesaplanır
//! - Mesafeye göre geçiş süresi ayarlanır
//! - QWERTY layout baz alınır

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Standart QWERTY klavye layout'u
const QWERTY_LAYOUT: &[&[&str]] = &[
    &["`", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=", "Backspace"],
    &["Tab", "q", "w", "e", "r", "t", "y", "u", "i", "o", "p", "[", "]", "\\"],
    &["Caps", "a", "s", "d", "f", "g", "h", "j", "k", "l", ";", "'", "Enter"],
    &["Shift", "z", "x", "c", "v", "b", "n", "m", ",", ".", "/", "Shift"],
    &["Ctrl", "Win", "Alt", "Space", "Alt", "Win", "Menu", "Ctrl"],
];

/// Tuş pozisyonu (satır, sütun)
#[derive(Debug, Clone, Copy)]
pub struct KeyPosition {
    pub row: usize,
    pub col: usize,
    pub x: f64,
    pub y: f64,
}

/// Tuş mesafesi hesaplayıcısı
pub struct KeyDistance {
    /// Tuş pozisyonları haritası
    key_positions: HashMap<char, KeyPosition>,
    /// Standart tuş boyutu (birim)
    key_size: f64,
}

impl KeyDistance {
    /// Yeni hesaplayıcı oluştur
    pub fn new() -> Self {
        let mut key_positions = HashMap::new();
        let key_size = 1.0;
        
        for (row_idx, row) in QWERTY_LAYOUT.iter().enumerate() {
            let mut col_offset = 0.0;
            
            for (col_idx, key) in row.iter().enumerate() {
                // Tuş genişlikleri
                let width = match *key {
                    "Backspace" => 2.0,
                    "Tab" => 1.5,
                    "\\" => 1.5,
                    "Caps" => 1.8,
                    "Enter" => 2.2,
                    "Shift" => 2.5,
                    "Space" => 6.0,
                    "Ctrl" | "Alt" | "Win" | "Menu" => 1.3,
                    _ => 1.0,
                };
                
                // Tek karakterli tuşları kaydet
                if key.len() == 1 {
                    let key_char = key.chars().next().unwrap();
                    key_positions.insert(key_char, KeyPosition {
                        row: row_idx,
                        col: col_idx,
                        x: col_offset + width / 2.0,
                        y: row_idx as f64,
                    });
                }
                
                col_offset += width;
            }
        }
        
        // Büyük harfler için de ekle
        let lowercase = "qwertyuiopasdfghjklzxcvbnm";
        let uppercase = "QWERTYUIOPASDFGHJKLZXCVBNM";
        
        for (l, u) in lowercase.chars().zip(uppercase.chars()) {
            if let Some(pos) = key_positions.get(&l) {
                key_positions.insert(u, *pos);
            }
        }
        
        Self {
            key_positions,
            key_size,
        }
    }
    
    /// İki tuş arasındaki fiziksel mesafeyi hesapla
    pub fn distance(&self, from: char, to: char) -> f64 {
        let from_pos = match self.key_positions.get(&from.to_ascii_lowercase()) {
            Some(p) => p,
            None => return 2.0, // Bilinmeyen tuş için varsayılan
        };
        
        let to_pos = match self.key_positions.get(&to.to_ascii_lowercase()) {
            Some(p) => p,
            None => return 2.0,
        };
        
        let dx = (to_pos.x - from_pos.x) * self.key_size;
        let dy = (to_pos.y - from_pos.y) * self.key_size;
        
        (dx * dx + dy * dy).sqrt()
    }
    
    /// Tuşun hangi el tarafından kullanıldığını belirle
    pub fn hand_for_key(&self, key: char) -> Hand {
        let key_lower = key.to_ascii_lowercase();
        
        // Sol el: q, w, e, r, t, a, s, d, f, g, z, x, c, v, b
        // Sağ el: y, u, i, o, p, h, j, k, l, n, m
        match key_lower {
            'q' | 'w' | 'e' | 'r' | 't' |
            'a' | 's' | 'd' | 'f' | 'g' |
            'z' | 'x' | 'c' | 'v' | 'b' |
            '1' | '2' | '3' | '4' | '5' => Hand::Left,
            'y' | 'u' | 'i' | 'o' | 'p' |
            'h' | 'j' | 'k' | 'l' |
            'n' | 'm' |
            '6' | '7' | '8' | '9' | '0' => Hand::Right,
            _ => Hand::Either,
        }
    }
    
    /// Tuş parmak mapping'i
    pub fn finger_for_key(&self, key: char) -> Finger {
        let key_lower = key.to_ascii_lowercase();
        
        match key_lower {
            // Sol el
            'q' | 'a' | 'z' | '1' => Finger::LeftPinky,
            'w' | 's' | 'x' | '2' => Finger::LeftRing,
            'e' | 'd' | 'c' | '3' => Finger::LeftMiddle,
            'r' | 'f' | 'v' | 't' | 'g' | 'b' | '4' | '5' => Finger::LeftIndex,
            
            // Sağ el
            'y' | 'h' | 'n' | 'u' | 'j' | 'm' | '6' | '7' => Finger::RightIndex,
            'i' | 'k' | ',' | '8' => Finger::RightMiddle,
            'o' | 'l' | '.' | '9' => Finger::RightRing,
            'p' | ';' | '/' | '0' => Finger::RightPinky,
            
            _ => Finger::Thumb, // Space ve diğerleri
        }
    }
}

impl Default for KeyDistance {
    fn default() -> Self {
        Self::new()
    }
}

/// El türü
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hand {
    Left,
    Right,
    Either,
}

/// Parmak türü
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Finger {
    LeftPinky,
    LeftRing,
    LeftMiddle,
    LeftIndex,
    Thumb,
    RightIndex,
    RightMiddle,
    RightRing,
    RightPinky,
}

/// Yazma profili
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingProfile {
    /// Ortalama WPM (Words Per Minute)
    pub wpm: u32,
    /// Mesafe katsayısı (mesafeye göre gecikme)
    pub distance_factor: f64,
    /// Rastgele varyasyon (0.0 - 1.0)
    pub variation: f64,
    /// Hata olasılığı (0.0 - 1.0)
    pub error_rate: f64,
    /// Düzeltme süresi (ms)
    pub correction_delay: u64,
    /// Shift tuşu gecikmesi (ms)
    pub shift_delay: u64,
    /// Space tuşu gecikmesi (ms)
    pub space_delay: u64,
}

impl Default for TypingProfile {
    fn default() -> Self {
        Self {
            wpm: 45,
            distance_factor: 15.0,
            variation: 0.3,
            error_rate: 0.02,
            correction_delay: 150,
            shift_delay: 80,
            space_delay: 100,
        }
    }
}

/// Yazma dinamiği motoru
pub struct TypingDynamics {
    /// Tuş mesafesi hesaplayıcı
    key_distance: KeyDistance,
    /// Yazma profili
    profile: TypingProfile,
    /// Önceki tuş
    last_key: Option<char>,
    /// RNN-LSTM benzeri durum
    state: TypingState,
}

/// RNN-LSTM benzeri durum (basitleştirilmiş)
#[derive(Debug, Clone, Default)]
struct TypingState {
    /// Son N tuş (context window)
    context: Vec<char>,
    /// Ortalama hız
    avg_speed: f64,
    /// Yorgunluk faktörü
    fatigue: f64,
}

impl TypingDynamics {
    /// Yeni yazma dinamiği oluştur
    pub fn new(wpm: u32) -> Self {
        let mut profile = TypingProfile::default();
        profile.wpm = wpm;
        
        Self {
            key_distance: KeyDistance::new(),
            profile,
            last_key: None,
            state: TypingState::default(),
        }
    }
    
    /// WPM ayarla
    pub fn set_wpm(&mut self, wpm: u32) {
        self.profile.wpm = wpm;
    }
    
    /// Metin için gecikme listesi oluştur
    pub fn generate_delays(&mut self, text: &str) -> Vec<(char, u64)> {
        let mut result = Vec::new();
        let mut rng = rand::thread_rng();
        
        for c in text.chars() {
            let delay = self.calculate_delay(c, &mut rng);
            result.push((c, delay));
            
            // Durumu güncelle
            self.update_state(c);
        }
        
        result
    }
    
    /// Tek tuş için gecikme hesapla
    fn calculate_delay(&self, key: char, rng: &mut impl Rng) -> u64 {
        // Baz gecikme: WPM'den
        // 1 WPM = 5 karakter/dakika = 12000ms / karakter
        let base_delay = 60000.0 / (self.profile.wpm as f64 * 5.0);
        
        // Mesafe faktörü
        let distance_delay = if let Some(last) = self.last_key {
            let dist = self.key_distance.distance(last, key);
            dist * self.profile.distance_factor
        } else {
            0.0
        };
        
        // Shift tuşu kontrolü
        let shift_delay = if key.is_uppercase() && key.is_alphabetic() {
            self.profile.shift_delay as f64
        } else {
            0.0
        };
        
        // Space tuşu gecikmesi
        let space_delay = if key == ' ' {
            self.profile.space_delay as f64
        } else {
            0.0
        };
        
        // Yorgunluk faktörü
        let fatigue_factor = 1.0 + self.state.fatigue * 0.3;
        
        // Rastgele varyasyon
        let variation = 1.0 + (rng.gen::<f64>() - 0.5) * self.profile.variation;
        
        // Toplam gecikme
        let total = (base_delay + distance_delay + shift_delay + space_delay) 
            * fatigue_factor 
            * variation;
        
        total.max(20.0).min(500.0) as u64
    }
    
    /// Durumu güncelle (RNN-LSTM benzeri)
    fn update_state(&mut self, key: char) {
        // Context window'u güncelle
        self.state.context.push(key);
        if self.state.context.len() > 10 {
            self.state.context.remove(0);
        }
        
        // Yorgunluğu artır (uzun yazmalarda)
        self.state.fatigue = (self.state.fatigue + 0.001).min(1.0);
        
        // Önceki tuşu güncelle
        self.last_key = Some(key);
    }
    
    /// Yorgunluğu sıfırla
    pub fn reset_fatigue(&mut self) {
        self.state.fatigue = 0.0;
    }
    
    /// Profil ayarla
    pub fn set_profile(&mut self, profile: TypingProfile) {
        self.profile = profile;
    }
    
    /// Profil getir
    pub fn profile(&self) -> &TypingProfile {
        &self.profile
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_key_distance_creation() {
        let kd = KeyDistance::new();
        assert!(kd.key_positions.contains_key(&'a'));
        assert!(kd.key_positions.contains_key(&'A'));
    }
    
    #[test]
    fn test_key_distance_calculation() {
        let kd = KeyDistance::new();
        
        // Aynı tuş
        let dist = kd.distance('a', 'a');
        assert_eq!(dist, 0.0);
        
        // Yan yana tuşlar
        let dist = kd.distance('a', 's');
        assert!(dist > 0.0 && dist < 2.0);
        
        // Uzak tuşlar
        let dist = kd.distance('a', 'p');
        assert!(dist > 5.0);
    }
    
    #[test]
    fn test_hand_for_key() {
        let kd = KeyDistance::new();
        
        assert_eq!(kd.hand_for_key('a'), Hand::Left);
        assert_eq!(kd.hand_for_key('j'), Hand::Right);
    }
    
    #[test]
    fn test_finger_for_key() {
        let kd = KeyDistance::new();
        
        assert_eq!(kd.finger_for_key('a'), Finger::LeftPinky);
        assert_eq!(kd.finger_for_key('j'), Finger::RightIndex);
    }
    
    #[test]
    fn test_typing_dynamics() {
        let mut td = TypingDynamics::new(45);
        let delays = td.generate_delays("Hello World");
        
        assert_eq!(delays.len(), 11);
        
        // Her gecikme makul aralıkta olmalı
        for (_, delay) in &delays {
            assert!(*delay >= 20 && *delay <= 500);
        }
    }
    
    #[test]
    fn test_wpm_affects_speed() {
        let mut td_fast = TypingDynamics::new(80);
        let mut td_slow = TypingDynamics::new(20);
        
        let fast_delays = td_fast.generate_delays("test");
        let slow_delays = td_slow.generate_delays("test");
        
        let fast_total: u64 = fast_delays.iter().map(|(_, d)| d).sum();
        let slow_total: u64 = slow_delays.iter().map(|(_, d)| d).sum();
        
        assert!(fast_total < slow_total);
    }
}
