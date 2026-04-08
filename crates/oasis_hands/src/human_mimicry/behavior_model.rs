//! ═══════════════════════════════════════════════════════════════════════════════
//!  BEHAVIOR MODEL - Agent-S3 Behavior Best-of-N Algoritması
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Agent-S3 mimarisindeki "Behavior Best-of-N" algoritması implementasyonu.
//! GUI eylemleri için insan üstü başarı oranı (%72.6+) hedefi.
//!
//! KONSEPT:
//! - N adet aday aksiyon oluştur
//! - Her aksiyonu "insan skor" ile değerlendir
//! - En yüksek skorlu aksiyonu seç
//! - Başarı oranını artır

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Aksiyon türü
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    /// Fare hareketi
    MouseMove { from: (f64, f64), to: (f64, f64) },
    /// Tıklama
    Click { x: f64, y: f64, button: String },
    /// Yazma
    Type { text: String },
    /// Kaydırma
    Scroll { delta: f64 },
    /// Bekleme
    Wait { duration_ms: u64 },
    /// Klavye kısayolu
    Shortcut { keys: Vec<String> },
}

/// Davranış profili
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorProfile {
    /// Ortalama aksiyon süresi (ms)
    pub avg_action_duration: u64,
    /// Aksiyonlar arası bekleme (ms)
    pub inter_action_delay: u64,
    /// Hata oranı
    pub error_rate: f64,
    /// Düzeltme olasılığı
    pub correction_probability: f64,
    /// Keşif olasılığı (random exploration)
    pub exploration_rate: f64,
    /// Başarı eşiği
    pub success_threshold: f64,
    /// Best-of-N değeri
    pub best_of_n: usize,
}

impl Default for BehaviorProfile {
    fn default() -> Self {
        Self {
            avg_action_duration: 200,
            inter_action_delay: 150,
            error_rate: 0.05,
            correction_probability: 0.8,
            exploration_rate: 0.1,
            success_threshold: 0.726, // %72.6 hedef
            best_of_n: 5,
        }
    }
}

/// Aksiyon adayı (Best-of-N için)
#[derive(Debug, Clone)]
struct ActionCandidate {
    action: ActionType,
    human_score: f64,
    predicted_success: f64,
}

/// RNN-LSTM benzeri davranış modeli (basitleştirilmiş)
#[derive(Debug, Clone, Default)]
struct RecurrentState {
    /// Gizli durum (hidden state)
    hidden: Vec<f64>,
    /// Hafıza (son aksiyonlar)
    memory: VecDeque<ActionType>,
    /// Bağlam (context window)
    context: Vec<f64>,
}

/// Davranış modeli
pub struct BehaviorModel {
    /// Profil
    profile: BehaviorProfile,
    /// RNN durumu
    state: RecurrentState,
    /// Başarı geçmişi
    success_history: VecDeque<bool>,
    /// Toplam aksiyon sayısı
    total_actions: u64,
    /// Başarılı aksiyon sayısı
    successful_actions: u64,
    /// RNN-LSTM modu aktif mi?
    use_rnn: bool,
}

impl BehaviorModel {
    /// Yeni davranış modeli oluştur
    pub fn new(use_rnn: bool) -> Self {
        Self {
            profile: BehaviorProfile::default(),
            state: RecurrentState {
                hidden: vec![0.0; 64],
                memory: VecDeque::with_capacity(10),
                context: vec![0.0; 32],
            },
            success_history: VecDeque::with_capacity(100),
            total_actions: 0,
            successful_actions: 0,
            use_rnn,
        }
    }
    
    /// Best-of-N algoritması ile en iyi aksiyonu seç
    pub fn select_best_action(&mut self, candidates: Vec<ActionType>) -> ActionType {
        if candidates.is_empty() {
            return ActionType::Wait { duration_ms: 100 };
        }
        
        // N aday oluştur
        let n = self.profile.best_of_n.min(candidates.len());
        let mut scored_candidates: Vec<ActionCandidate> = candidates
            .into_iter()
            .take(n)
            .map(|action| {
                let human_score = self.calculate_human_score(&[action.clone()]);
                let predicted_success = self.predict_success(&action);
                
                ActionCandidate {
                    action,
                    human_score,
                    predicted_success,
                }
            })
            .collect();
        
        // Skorlara göre sırala
        scored_candidates.sort_by(|a, b| {
            let score_a = a.human_score * 0.4 + a.predicted_success * 0.6;
            let score_b = b.human_score * 0.4 + b.predicted_success * 0.6;
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        // Keşif modu (exploration)
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < self.profile.exploration_rate {
            let idx = rng.gen_range(0..scored_candidates.len());
            return scored_candidates.remove(idx).action;
        }
        
        // En iyi aksiyonu döndür
        scored_candidates.remove(0).action
    }
    
    /// İnsan benzerliği skoru hesapla
    pub fn calculate_human_score(&self, actions: &[ActionType]) -> f64 {
        let mut score = 0.0;
        let mut weight_sum = 0.0;
        
        for (i, action) in actions.iter().enumerate() {
            let recency_weight = 1.0 / (1.0 + i as f64);
            weight_sum += recency_weight;
            
            let action_score = match action {
                ActionType::MouseMove { from, to } => {
                    // Mesafe bazlı skor
                    let distance = ((to.0 - from.0).powi(2) + (to.1 - from.1).powi(2)).sqrt();
                    let distance_score = 1.0 - (distance / 2000.0).min(1.0);
                    
                    // RNN durum tutarlılığı
                    let consistency = if self.use_rnn {
                        self.rnn_consistency_score()
                    } else {
                        0.8
                    };
                    
                    distance_score * 0.6 + consistency * 0.4
                }
                
                ActionType::Click { x: _, y: _, .. } => {
                    // Tıklama pozisyonu doğruluğu
                    let position_score = 0.9; // Hedef yakın
                    
                    // Context uygunluğu
                    let context_score = self.context_relevance();
                    
                    position_score * 0.5 + context_score * 0.5
                }
                
                ActionType::Type { text } => {
                    // Yazma doğallığı
                    let length = text.len();
                    let length_score = if length < 5 { 0.95 } else if length < 20 { 0.85 } else { 0.75 };
                    
                    // Hafıza ile tutarlılık
                    let memory_score = self.memory_relevance(text);
                    
                    length_score * 0.4 + memory_score * 0.6
                }
                
                ActionType::Scroll { delta } => {
                    // Kaydırma doğallığı
                    let scroll_score = if delta.abs() < 100.0 { 0.9 } else { 0.7 };
                    scroll_score
                }
                
                ActionType::Wait { duration_ms } => {
                    // Bekleme süresi uygunluğu
                    let wait_score = if *duration_ms < 500 { 0.95 } 
                                     else if *duration_ms < 2000 { 0.85 } 
                                     else { 0.6 };
                    wait_score
                }
                
                ActionType::Shortcut { keys } => {
                    // Kısayol karmaşıklığı
                    let complexity = keys.len();
                    let shortcut_score = if complexity <= 2 { 0.9 } else { 0.7 };
                    shortcut_score
                }
            };
            
            score += action_score * recency_weight;
        }
        
        if weight_sum > 0.0 {
            score / weight_sum
        } else {
            0.5
        }
    }
    
    /// Başarı tahmini
    fn predict_success(&self, action: &ActionType) -> f64 {
        // Geçmiş başarı oranından tahmin
        let historical_rate = if self.total_actions > 0 {
            self.successful_actions as f64 / self.total_actions as f64
        } else {
            0.5
        };
        
        // Aksiyon türüne göre ayarla
        let action_modifier = match action {
            ActionType::MouseMove { .. } => 0.05,
            ActionType::Click { .. } => 0.1,
            ActionType::Type { .. } => -0.05,
            ActionType::Scroll { .. } => 0.15,
            ActionType::Wait { .. } => 0.2,
            ActionType::Shortcut { .. } => -0.1,
        };
        
        // RNN tahmini
        let rnn_boost = if self.use_rnn {
            self.rnn_prediction_boost()
        } else {
            0.0
        };
        
        (historical_rate + action_modifier + rnn_boost).clamp(0.0, 1.0)
    }
    
    /// RNN tutarlılık skoru
    fn rnn_consistency_score(&self) -> f64 {
        if !self.use_rnn {
            return 0.8;
        }
        
        // Hidden state norm
        let norm: f64 = self.state.hidden.iter().map(|x| x * x).sum::<f64>().sqrt();
        let normalized_norm = (norm / 8.0).min(1.0);
        
        0.7 + normalized_norm * 0.3
    }
    
    /// Bağlam uygunluğu
    fn context_relevance(&self) -> f64 {
        if self.state.context.is_empty() {
            return 0.8;
        }
        
        let mean: f64 = self.state.context.iter().sum::<f64>() / self.state.context.len() as f64;
        mean.clamp(0.5, 1.0)
    }
    
    /// Hafıza tutarlılığı
    fn memory_relevance(&self, text: &str) -> f64 {
        if self.state.memory.is_empty() {
            return 0.85;
        }
        
        // Son yazma işlemlerine bak
        let recent_typing: Vec<_> = self.state.memory.iter()
            .filter_map(|a| {
                if let ActionType::Type { text } = a {
                    Some(text.clone())
                } else {
                    None
                }
            })
            .collect();
        
        if recent_typing.is_empty() {
            return 0.85;
        }
        
        // Benzerlik hesapla
        let similarities: f64 = recent_typing.iter()
            .map(|t| {
                let common = t.chars()
                    .filter(|c| text.contains(*c))
                    .count();
                common as f64 / t.len().max(1) as f64
            })
            .sum();
        
        let avg_similarity = similarities / recent_typing.len() as f64;
        0.5 + avg_similarity * 0.5
    }
    
    /// RNN tahmin artışı
    fn rnn_prediction_boost(&self) -> f64 {
        if !self.use_rnn {
            return 0.0;
        }
        
        // Hidden state'in pozitif elemanları
        let positive_ratio = self.state.hidden.iter()
            .filter(|&&x| x > 0.0)
            .count() as f64 / self.state.hidden.len() as f64;
        
        (positive_ratio - 0.5) * 0.1
    }
    
    /// Aksiyon kaydet ve RNN durumunu güncelle
    pub fn record_action(&mut self, action: ActionType, success: bool) {
        self.total_actions += 1;
        if success {
            self.successful_actions += 1;
        }
        
        // Geçmişe ekle
        self.success_history.push_back(success);
        if self.success_history.len() > 100 {
            self.success_history.pop_front();
        }
        
        // Hafızaya ekle
        self.state.memory.push_back(action);
        if self.state.memory.len() > 10 {
            self.state.memory.pop_front();
        }
        
        // RNN durumunu güncelle (basitleştirilmiş LSTM benzeri)
        if self.use_rnn {
            self.update_rnn_state(success);
        }
    }
    
    /// RNN durum güncellemesi
    fn update_rnn_state(&mut self, success: bool) {
        let reward = if success { 1.0 } else { -0.5 };
        
        // Basit gradyan benzeri güncelleme
        for i in 0..self.state.hidden.len() {
            let gradient = reward * (rand::thread_rng().gen::<f64>() - 0.5) * 0.1;
            self.state.hidden[i] = (self.state.hidden[i] + gradient).clamp(-1.0, 1.0);
        }
        
        // Context güncelle
        let success_val = if success { 1.0 } else { 0.0 };
        self.state.context.push(success_val);
        if self.state.context.len() > 32 {
            self.state.context.remove(0);
        }
    }
    
    /// Rastgele gecikme (insan benzeri)
    pub fn random_delay(&self) -> u64 {
        let mut rng = rand::thread_rng();
        
        let base = self.profile.inter_action_delay as f64;
        let variation = (rng.gen::<f64>() - 0.5) * base * 0.4;
        
        (base + variation).max(50.0) as u64
    }
    
    /// Mevcut başarı oranı
    pub fn success_rate(&self) -> f64 {
        if self.total_actions == 0 {
            return 0.0;
        }
        self.successful_actions as f64 / self.total_actions as f64
    }
    
    /// Hedef başarıya ulaşıldı mı?
    pub fn is_above_target(&self) -> bool {
        self.success_rate() >= self.profile.success_threshold
    }
    
    /// Profil getir
    pub fn profile(&self) -> &BehaviorProfile {
        &self.profile
    }
    
    /// Profil ayarla
    pub fn set_profile(&mut self, profile: BehaviorProfile) {
        self.profile = profile;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_behavior_profile_default() {
        let profile = BehaviorProfile::default();
        assert_eq!(profile.best_of_n, 5);
        assert!(profile.success_threshold > 0.7);
    }
    
    #[test]
    fn test_behavior_model_creation() {
        let model = BehaviorModel::new(true);
        assert_eq!(model.total_actions, 0);
    }
    
    #[test]
    fn test_select_best_action() {
        let mut model = BehaviorModel::new(true);
        
        let candidates = vec![
            ActionType::Click { x: 100.0, y: 100.0, button: "left".into() },
            ActionType::Click { x: 200.0, y: 200.0, button: "left".into() },
            ActionType::Wait { duration_ms: 100 },
        ];
        
        let selected = model.select_best_action(candidates);
        
        // Bir aksiyon seçilmeli
        match selected {
            ActionType::Click { .. } | ActionType::Wait { .. } => {}
            _ => panic!("Beklenmeyen aksiyon türü"),
        }
    }
    
    #[test]
    fn test_calculate_human_score() {
        let model = BehaviorModel::new(true);
        
        let actions = vec![
            ActionType::Click { x: 100.0, y: 100.0, button: "left".into() },
        ];
        
        let score = model.calculate_human_score(&actions);
        assert!(score > 0.0 && score <= 1.0);
    }
    
    #[test]
    fn test_record_action() {
        let mut model = BehaviorModel::new(true);
        
        model.record_action(ActionType::Wait { duration_ms: 100 }, true);
        model.record_action(ActionType::Wait { duration_ms: 100 }, true);
        model.record_action(ActionType::Wait { duration_ms: 100 }, false);
        
        assert_eq!(model.total_actions, 3);
        assert_eq!(model.successful_actions, 2);
        assert!((model.success_rate() - 0.666).abs() < 0.01);
    }
    
    #[test]
    fn test_random_delay() {
        let model = BehaviorModel::new(true);
        
        for _ in 0..100 {
            let delay = model.random_delay();
            assert!(delay >= 50);
        }
    }
}
