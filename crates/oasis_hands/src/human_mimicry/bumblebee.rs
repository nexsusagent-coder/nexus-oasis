//! ═══════════════════════════════════════════════════════════════════════════════
//!  BUMBLEBEE ENGINE - RNN-LSTM Fare Hareket Modeli
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Bumblebee kütüphanesinin Rust asimilasyonu.
//! RNN-LSTM tabanlı doğal fare hareketi simülasyonu.
//!
//! ÖZELLİKLER:
//! - Hareket paterni öğrenimi
//! - Zaman serisi tahmini
//! - Doğal hızlanma/yavaşlama
//! - Hedef odaklı hareket

use rand::Rng;
use serde::{Deserialize, Serialize};

/// Bumblebee yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BumblebeeConfig {
    /// Gizli katman boyutu
    pub hidden_size: usize,
    /// Bağlam penceresi
    pub context_window: usize,
    /// Öğrenme oranı
    pub learning_rate: f64,
    /// Momentum
    pub momentum: f64,
    /// Hareket yumuşaklığı
    pub smoothness: f64,
    /// Hedef hassasiyeti
    pub target_precision: f64,
}

impl Default for BumblebeeConfig {
    fn default() -> Self {
        Self {
            hidden_size: 64,
            context_window: 10,
            learning_rate: 0.01,
            momentum: 0.9,
            smoothness: 0.8,
            target_precision: 5.0,
        }
    }
}

/// Hareket paterni
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MovementPattern {
    /// Doğrusal
    Linear,
    /// Eğrisel (Bezier benzeri)
    Curved,
    /// Dalgalı
    Wavy,
    /// Spiralsel
    Spiral,
    /// Zikzak
    Zigzag,
    /// Doğal (karışık)
    Natural,
}

/// LSTM hücre durumu
#[derive(Debug, Clone)]
struct LSTMState {
    /// Hidden state
    h: Vec<f64>,
    /// Cell state
    c: Vec<f64>,
    /// Forget gate
    f: Vec<f64>,
    /// Input gate
    i: Vec<f64>,
    /// Output gate
    o: Vec<f64>,
    /// Candidate cell state
    c_tilde: Vec<f64>,
}

impl LSTMState {
    fn new(hidden_size: usize) -> Self {
        Self {
            h: vec![0.0; hidden_size],
            c: vec![0.0; hidden_size],
            f: vec![0.0; hidden_size],
            i: vec![0.0; hidden_size],
            o: vec![0.0; hidden_size],
            c_tilde: vec![0.0; hidden_size],
        }
    }
}

/// LSTM ağırlıkları (basitleştirilmiş)
#[derive(Debug, Clone)]
struct LSTMWeights {
    w_f: Vec<Vec<f64>>,
    w_i: Vec<Vec<f64>>,
    w_o: Vec<Vec<f64>>,
    w_c: Vec<Vec<f64>>,
    b_f: Vec<f64>,
    b_i: Vec<f64>,
    b_o: Vec<f64>,
    b_c: Vec<f64>,
}

impl LSTMWeights {
    fn new(input_size: usize, hidden_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        
        let mut init_weight = || {
            (0..hidden_size)
                .map(|_| (rng.gen::<f64>() - 0.5) * 0.1)
                .collect::<Vec<f64>>()
        };
        
        Self {
            w_f: (0..input_size).map(|_| init_weight()).collect(),
            w_i: (0..input_size).map(|_| init_weight()).collect(),
            w_o: (0..input_size).map(|_| init_weight()).collect(),
            w_c: (0..input_size).map(|_| init_weight()).collect(),
            b_f: vec![0.0; hidden_size],
            b_i: vec![0.0; hidden_size],
            b_o: vec![0.0; hidden_size],
            b_c: vec![0.0; hidden_size],
        }
    }
}

/// Bumblebee Engine
pub struct BumblebeeEngine {
    /// Yapılandırma
    config: BumblebeeConfig,
    /// LSTM durumu
    state: LSTMState,
    /// LSTM ağırlıkları
    weights: LSTMWeights,
    /// Hareket geçmişi
    movement_history: Vec<(f64, f64, u64)>,
    /// Mevcut patern
    current_pattern: MovementPattern,
}

impl BumblebeeEngine {
    /// Yeni motor oluştur
    pub fn new(config: BumblebeeConfig) -> Self {
        let hidden_size = config.hidden_size;
        
        Self {
            config,
            state: LSTMState::new(hidden_size),
            weights: LSTMWeights::new(3, hidden_size), // 3 input: x, y, t
            movement_history: Vec::with_capacity(100),
            current_pattern: MovementPattern::Natural,
        }
    }
    
    /// İnsan benzeri hareket yolu oluştur
    pub fn generate_path(
        &mut self,
        from: (f64, f64),
        to: (f64, f64),
        pattern: MovementPattern,
    ) -> Vec<(f64, f64, u64)> {
        self.current_pattern = pattern.clone();
        
        let distance = ((to.0 - from.0).powi(2) + (to.1 - from.1).powi(2)).sqrt();
        let steps = self.calculate_steps(distance);
        
        let mut path = Vec::with_capacity(steps);
        let mut current = from;
        let mut time = 0u64;
        
        for i in 0..steps {
            let progress = i as f64 / steps as f64;
            
            // Patern bazlı ara hedef hesapla
            let (intermediate_x, intermediate_y) = self.calculate_intermediate_point(
                from, to, progress, &pattern,
            );
            
            // LSTM tahmini ile doğallaştır
            let (lstm_x, lstm_y) = self.lstm_predict(
                current.0, current.1,
                intermediate_x, intermediate_y,
            );
            
            // Zaman hesapla
            let step_time = self.calculate_step_time(progress);
            time += step_time;
            
            path.push((lstm_x, lstm_y, time));
            current = (lstm_x, lstm_y);
            
            // LSTM durumunu güncelle
            self.lstm_update(lstm_x, lstm_y, time as f64);
        }
        
        // Son noktayı kesin hedefe ayarla
        if let Some(last) = path.last_mut() {
            last.0 = to.0;
            last.1 = to.1;
        }
        
        // Hareket geçmişine ekle
        self.movement_history.extend(path.clone());
        if self.movement_history.len() > 100 {
            self.movement_history.drain(0..self.movement_history.len() - 100);
        }
        
        path
    }
    
    /// Adım sayısı hesapla
    fn calculate_steps(&self, distance: f64) -> usize {
        // Fitts yasası bazlı
        let base_steps = 10;
        let distance_factor = (distance / 100.0).sqrt();
        
        let steps = ((base_steps as f64) * distance_factor * self.config.smoothness) as usize;
        steps.max(5).min(100)
    }
    
    /// Ara nokta hesapla (patern bazlı)
    fn calculate_intermediate_point(
        &self,
        from: (f64, f64),
        to: (f64, f64),
        progress: f64,
        pattern: &MovementPattern,
    ) -> (f64, f64) {
        let dx = to.0 - from.0;
        let dy = to.1 - from.1;
        
        match pattern {
            MovementPattern::Linear => {
                (from.0 + dx * progress, from.1 + dy * progress)
            }
            
            MovementPattern::Curved => {
                // Bezier benzeri eğri
                let curve_progress = Self::ease_in_out(progress);
                let perpendicular = (progress - 0.5).abs() * 2.0;
                let offset = perpendicular.sin() * 30.0;
                
                (
                    from.0 + dx * curve_progress + offset,
                    from.1 + dy * curve_progress + offset * 0.5,
                )
            }
            
            MovementPattern::Wavy => {
                // Dalgalı hareket
                let wave = (progress * std::f64::consts::TAU * 2.0).sin() * 15.0;
                
                (
                    from.0 + dx * progress + wave,
                    from.1 + dy * progress,
                )
            }
            
            MovementPattern::Spiral => {
                // Spiral hareket
                let angle = progress * std::f64::consts::TAU;
                let radius = (1.0 - progress) * 30.0;
                
                (
                    from.0 + dx * progress + angle.cos() * radius,
                    from.1 + dy * progress + angle.sin() * radius,
                )
            }
            
            MovementPattern::Zigzag => {
                // Zikzak hareket
                let zigzag = if (progress * 10.0) as i32 % 2 == 0 { 20.0 } else { -20.0 };
                
                (
                    from.0 + dx * progress,
                    from.1 + dy * progress + zigzag * (1.0 - progress),
                )
            }
            
            MovementPattern::Natural => {
                // Doğal (rastgele varyasyonlu)
                let mut rng = rand::thread_rng();
                let variation = (rng.gen::<f64>() - 0.5) * 10.0 * (1.0 - progress);
                
                let eased_progress = Self::ease_in_out(progress);
                
                (
                    from.0 + dx * eased_progress + variation,
                    from.1 + dy * eased_progress + variation * 0.7,
                )
            }
        }
    }
    
    /// LSTM tahmini
    fn lstm_predict(&self, _current_x: f64, _current_y: f64, target_x: f64, target_y: f64) -> (f64, f64) {
        // Basitleştirilmiş LSTM çıktısı
        // Gerçek implementasyonda hidden state kullanılır
        
        let mut rng = rand::thread_rng();
        
        // Hidden state'in ortalamasını al
        let h_mean: f64 = self.state.h.iter().sum::<f64>() / self.state.h.len() as f64;
        
        // Doğallaştırma faktörü
        let natural_factor = h_mean.abs() * 5.0 + 1.0;
        
        let variation_x = (rng.gen::<f64>() - 0.5) * natural_factor;
        let variation_y = (rng.gen::<f64>() - 0.5) * natural_factor;
        
        (
            target_x + variation_x,
            target_y + variation_y,
        )
    }
    
    /// LSTM durum güncellemesi
    fn lstm_update(&mut self, x: f64, y: f64, t: f64) {
        let input = vec![x / 1000.0, y / 1000.0, t / 10000.0]; // Normalize
        
        // Basitleştirilmiş LSTM forward pass
        for j in 0..self.config.hidden_size {
            // Forget gate: f = sigmoid(Wf * [h, x] + bf)
            let mut f_sum = self.weights.b_f[j];
            for (i, &inp) in input.iter().enumerate() {
                f_sum += self.weights.w_f.get(i).map(|w| w[j]).unwrap_or(0.0) * inp;
            }
            self.state.f[j] = Self::sigmoid(f_sum);
            
            // Input gate: i = sigmoid(Wi * [h, x] + bi)
            let mut i_sum = self.weights.b_i[j];
            for (i, &inp) in input.iter().enumerate() {
                i_sum += self.weights.w_i.get(i).map(|w| w[j]).unwrap_or(0.0) * inp;
            }
            self.state.i[j] = Self::sigmoid(i_sum);
            
            // Output gate: o = sigmoid(Wo * [h, x] + bo)
            let mut o_sum = self.weights.b_o[j];
            for (i, &inp) in input.iter().enumerate() {
                o_sum += self.weights.w_o.get(i).map(|w| w[j]).unwrap_or(0.0) * inp;
            }
            self.state.o[j] = Self::sigmoid(o_sum);
            
            // Candidate cell state
            let mut c_sum = self.weights.b_c[j];
            for (i, &inp) in input.iter().enumerate() {
                c_sum += self.weights.w_c.get(i).map(|w| w[j]).unwrap_or(0.0) * inp;
            }
            self.state.c_tilde[j] = (c_sum).tanh();
            
            // Update cell state
            self.state.c[j] = self.state.f[j] * self.state.c[j] 
                            + self.state.i[j] * self.state.c_tilde[j];
            
            // Update hidden state
            self.state.h[j] = self.state.o[j] * self.state.c[j].tanh();
        }
    }
    
    /// Sigmoid fonksiyonu
    fn sigmoid(x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }
    
    /// Ease-in-out fonksiyonu
    fn ease_in_out(t: f64) -> f64 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
        }
    }
    
    /// Adım süresi hesapla
    fn calculate_step_time(&self, progress: f64) -> u64 {
        // Başlangıçta yavaş, ortada hızlı, sonda yavaş
        let base_time = 20.0;
        let acceleration = 1.0 - 4.0 * (progress - 0.5).powi(2);
        let speed_factor = 1.0 + acceleration * 0.5;
        
        let mut rng = rand::thread_rng();
        let variation = 1.0 + (rng.gen::<f64>() - 0.5) * 0.3;
        
        (base_time * speed_factor * variation) as u64
    }
    
    /// Hedef hassasiyetini kontrol et
    pub fn check_precision(&self, actual: (f64, f64), target: (f64, f64)) -> bool {
        let error = ((actual.0 - target.0).powi(2) + (actual.1 - target.1).powi(2)).sqrt();
        error <= self.config.target_precision
    }
    
    /// Yapılandırmayı getir
    pub fn config(&self) -> &BumblebeeConfig {
        &self.config
    }
    
    /// Hareket geçmişini temizle
    pub fn reset(&mut self) {
        self.movement_history.clear();
        self.state = LSTMState::new(self.config.hidden_size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bumblebee_config_default() {
        let config = BumblebeeConfig::default();
        assert_eq!(config.hidden_size, 64);
    }
    
    #[test]
    fn test_bumblebee_creation() {
        let engine = BumblebeeEngine::new(BumblebeeConfig::default());
        assert!(engine.movement_history.is_empty());
    }
    
    #[test]
    fn test_generate_linear_path() {
        let mut engine = BumblebeeEngine::new(BumblebeeConfig::default());
        let path = engine.generate_path((0.0, 0.0), (100.0, 100.0), MovementPattern::Linear);
        
        assert!(!path.is_empty());
        
        // Son nokta hedefe yakın olmalı
        let last = path.last().expect("operation failed");
        assert!((last.0 - 100.0).abs() < 0.1);
        assert!((last.1 - 100.0).abs() < 0.1);
    }
    
    #[test]
    fn test_generate_natural_path() {
        let mut engine = BumblebeeEngine::new(BumblebeeConfig::default());
        let path = engine.generate_path((0.0, 0.0), (500.0, 300.0), MovementPattern::Natural);
        
        assert!(!path.is_empty());
        
        // Zamansal sıralama doğru olmalı
        for i in 1..path.len() {
            assert!(path[i].2 >= path[i-1].2);
        }
    }
    
    #[test]
    fn test_check_precision() {
        let engine = BumblebeeEngine::new(BumblebeeConfig::default());
        
        // Hassas
        assert!(engine.check_precision((100.0, 100.0), (102.0, 102.0)));
        
        // Hassas değil
        assert!(!engine.check_precision((100.0, 100.0), (200.0, 200.0)));
    }
    
    #[test]
    fn test_sigmoid() {
        assert!((BumblebeeEngine::sigmoid(0.0) - 0.5).abs() < 0.01);
        assert!(BumblebeeEngine::sigmoid(10.0) > 0.99);
        assert!(BumblebeeEngine::sigmoid(-10.0) < 0.01);
    }
    
    #[test]
    fn test_ease_in_out() {
        // Başlangıç
        assert!(BumblebeeEngine::ease_in_out(0.0) < 0.01);
        
        // Ortası
        assert!((BumblebeeEngine::ease_in_out(0.5) - 0.5).abs() < 0.01);
        
        // Son
        assert!(BumblebeeEngine::ease_in_out(1.0) > 0.99);
    }
}
