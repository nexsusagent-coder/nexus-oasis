//! ─── BELLEK UNUTMA (DECAY) ───
//!
//! Zaman tabanlı bellek azalması:
//! - Ebbinghaus forgetting curve
//! - Öneme göre unutma
//! - Ani hatırlama (retrieval-induced reconsolidation)

use crate::{
    MemoryCube, MemoryEntry, MemoryType,
    Importance, MemoryResult, MemoryError,
};
use chrono::{Utc, Duration};

// ─────────────────────────────────────────────────────────────────────────────
// DECAY CONFIG
// ─────────────────────────────────────────────────────────────────────────────

/// Unutma yapılandırması
#[derive(Debug, Clone)]
pub struct DecayConfig {
    /// Temel unutma oranı (gün başına)
    pub base_decay_rate: f32,
    /// Episodik bellek azalma çarpanı (daha hızlı unutulur)
    pub episodic_multiplier: f32,
    /// Semantik bellek azalma çarpanı (daha yavaş unutulur)
    pub semantic_multiplier: f32,
    /// Prosedürel bellek azalma çarpanı
    pub procedural_multiplier: f32,
    /// Minimum önem eşiği (altında silinir)
    pub min_importance_threshold: f32,
    /// Erişim bonusu (her erişimde)
    pub access_bonus: f32,
    /// Maksimum unutma oranı
    pub max_decay_per_cycle: f32,
}

impl Default for DecayConfig {
    fn default() -> Self {
        Self {
            base_decay_rate: 0.1, // %10 günlük
            episodic_multiplier: 1.5, // Daha hızlı
            semantic_multiplier: 0.5, // Daha yavaş
            procedural_multiplier: 0.7,
            min_importance_threshold: 0.1,
            access_bonus: 0.05,
            max_decay_per_cycle: 0.3,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MEMORY DECAY
// ─────────────────────────────────────────────────────────────────────────────

/// Bellek unutma sistemi
pub struct MemoryDecay {
    config: DecayConfig,
}

impl MemoryDecay {
    /// Yeni unutma sistemi oluştur
    pub fn new(config: DecayConfig) -> Self {
        Self { config }
    }
    
    /// Varsayılan yapılandırma ile
    pub fn with_defaults() -> Self {
        Self::new(DecayConfig::default())
    }
    
    /// Bellek önemini azalt
    pub fn apply_decay(&self, entry: &mut MemoryEntry) -> f32 {
        let age_days = (Utc::now() - entry.created_at).num_days().max(0) as f32;
        
        // Tip çarpanı
        let type_multiplier = match entry.memory_type {
            MemoryType::Episodic => self.config.episodic_multiplier,
            MemoryType::Semantic => self.config.semantic_multiplier,
            MemoryType::Procedural => self.config.procedural_multiplier,
            MemoryType::Working => 2.0, // En hızlı
            MemoryType::Emotional => 0.3, // En yavaş
            MemoryType::Meta => 0.4,
        };
        
        // Ebbinghaus curve: R = e^(-t/S)
        // Basitleştirilmiş: decay = base * multiplier * days
        let decay_amount = (self.config.base_decay_rate 
            * type_multiplier 
            * age_days 
            / 7.0) // Haftalık baz
            .min(self.config.max_decay_per_cycle);
        
        // Yeni önem
        let new_importance = (entry.importance.value() - decay_amount)
            .max(0.0);
        
        entry.importance = Importance::new(new_importance);
        
        new_importance
    }
    
    /// Bellek güçlendirme (başarılı hatırlama)
    pub fn reinforce(&self, entry: &mut MemoryEntry) {
        let bonus = self.config.access_bonus 
            * (1.0 - entry.importance.value() * 0.5); // Düşük önemliler daha çok kazanır
        
        let new_importance = (entry.importance.value() + bonus).min(1.0);
        entry.importance = Importance::new(new_importance);
        entry.access();
        
        log::debug!(
            "💪  Güçlendirme: {} → {:.2}",
            entry.id,
            new_importance
        );
    }
    
    /// Toplu unutma uygula
    pub fn apply_batch_decay(&self, cube: &mut MemoryCube) -> MemoryResult<DecayReport> {
        let entries = cube.list_all()?;
        let mut report = DecayReport::default();
        
        for mut entry in entries {
            let old_importance = entry.importance.value();
            let new_importance = self.apply_decay(&mut entry);
            
            report.total_processed += 1;
            
            // Silme eşiği kontrolü
            if new_importance < self.config.min_importance_threshold {
                cube.delete(entry.id)?;
                report.removed_count += 1;
                log::debug!("🗑️  Silindi: {} (önem: {:.3})", entry.id, new_importance);
            } else if (old_importance - new_importance).abs() > 0.01 {
                cube.update_importance(entry.id, entry.importance)?;
                report.decayed_count += 1;
            }
        }
        
        log::info!(
            "⏳  UNUTMA: {} işlendi, {} azaldı, {} silindi",
            report.total_processed,
            report.decayed_count,
            report.removed_count
        );
        
        Ok(report)
    }
    
    /// Etkileşim sonucu güncelle
    pub fn update_from_interaction(
        &self,
        cube: &mut MemoryCube,
        memory_id: uuid::Uuid,
        was_useful: bool,
    ) -> MemoryResult<()> {
        if let Some(mut entry) = cube.recall(memory_id)? {
            if was_useful {
                self.reinforce(&mut entry);
            } else {
                // Kullanışsız bulundu - azalt
                let penalty = self.config.access_bonus * 2.0;
                let new_importance = (entry.importance.value() - penalty).max(0.0);
                entry.importance = Importance::new(new_importance);
            }
            cube.store(entry)?;
        }
        Ok(())
    }
    
    /// Bellek sağlığını kontrol et
    pub fn health_check(&self, cube: &MemoryCube) -> MemoryResult<MemoryHealth> {
        let entries = cube.list_all()?;
        
        let mut by_importance: Vec<f32> = entries.iter()
            .map(|e| e.importance.value())
            .collect();
        by_importance.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        
        let avg_importance = if by_importance.is_empty() {
            0.0
        } else {
            by_importance.iter().sum::<f32>() / by_importance.len() as f32
        };
        
        let critical_count = by_importance.iter()
            .filter(|&&i| i < 0.2)
            .count();
        
        let healthy_count = by_importance.iter()
            .filter(|&&i| i >= 0.6)
            .count();
        
        Ok(MemoryHealth {
            total_memories: entries.len(),
            average_importance: avg_importance,
            critical_memories: critical_count,
            healthy_memories: healthy_count,
            memory_pressure: critical_count as f32 / entries.len().max(1) as f32,
        })
    }
    
    /// Yapılandırma
    pub fn config(&self) -> &DecayConfig {
        &self.config
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// DECAY REPORT
// ─────────────────────────────────────────────────────────────────────────────

/// Unutma raporu
#[derive(Debug, Clone, Default)]
pub struct DecayReport {
    pub total_processed: usize,
    pub decayed_count: usize,
    pub removed_count: usize,
}

impl DecayReport {
    pub fn is_empty(&self) -> bool {
        self.total_processed == 0
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MEMORY HEALTH
// ─────────────────────────────────────────────────────────────────────────────

/// Bellek sağlık durumu
#[derive(Debug, Clone)]
pub struct MemoryHealth {
    pub total_memories: usize,
    pub average_importance: f32,
    pub critical_memories: usize,
    pub healthy_memories: usize,
    pub memory_pressure: f32,
}

impl MemoryHealth {
    pub fn is_healthy(&self) -> bool {
        self.memory_pressure < 0.3 && self.average_importance > 0.4
    }
    
    pub fn status(&self) -> &'static str {
        if self.memory_pressure > 0.5 {
            "KRİTİK"
        } else if self.memory_pressure > 0.3 {
            "DİKKAT"
        } else if self.average_importance > 0.6 {
            "SAĞLIKLI"
        } else {
            "NORMAL"
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// FORGETTING CURVE (Ebbinghaus)
// ─────────────────────────────────────────────────────────────────────────────

/// Ebbinghaus forgetting curve hesaplayıcı
pub struct ForgettingCurve {
    /// Başlangıç güç
    initial_strength: f32,
    /// Bellek stabilitesi
    stability: f32,
}

impl ForgettingCurve {
    pub fn new(initial_strength: f32, stability: f32) -> Self {
        Self {
            initial_strength: initial_strength.clamp(0.0, 1.0),
            stability: stability.max(0.01),
        }
    }
    
    /// t gün sonraki hatırlama olasılığı
    pub fn recall_probability(&self, days: f32) -> f32 {
        // R = S * e^(-t/stability)
        self.initial_strength * (-days / self.stability).exp()
    }
    
    /// Belirli bir olasılık için geçen süre
    pub fn time_for_probability(&self, probability: f32) -> f32 {
        // t = -stability * ln(probability/initial_strength)
        if probability >= self.initial_strength {
            return 0.0;
        }
        -self.stability * (probability / self.initial_strength).ln()
    }
    
    /// Optimal tekrar zamanı (spaced repetition)
    pub fn optimal_review_time(&self, current_days: f32) -> f32 {
        // İdeal tekrar %90 hatırlama noktasında
        let target_probability = 0.9 * self.initial_strength;
        self.time_for_probability(target_probability) - current_days
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TESTLER
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_decay_config_default() {
        let config = DecayConfig::default();
        assert!((config.base_decay_rate - 0.1).abs() < 0.01);
        assert!((config.min_importance_threshold - 0.1).abs() < 0.01);
    }
    
    #[test]
    fn test_apply_decay() {
        let decay = MemoryDecay::with_defaults();
        
        let mut entry = MemoryEntry::from_input(
            crate::MemoryInput::new("Test")
                .with_type(MemoryType::Semantic)
                .with_importance(Importance::high())
        );
        entry.created_at = Utc::now() - Duration::days(7);
        
        let old_importance = entry.importance.value();
        let new_importance = decay.apply_decay(&mut entry);
        
        assert!(new_importance < old_importance);
    }
    
    #[test]
    fn test_reinforce() {
        let decay = MemoryDecay::with_defaults();
        
        let mut entry = MemoryEntry::from_input(
            crate::MemoryInput::new("Test")
                .with_importance(Importance::low())
        );
        
        let old_importance = entry.importance.value();
        decay.reinforce(&mut entry);
        
        assert!(entry.importance.value() > old_importance);
        assert_eq!(entry.access_count, 1);
    }
    
    #[test]
    fn test_forgetting_curve() {
        let curve = ForgettingCurve::new(1.0, 7.0);
        
        // 0 günde tam hatırlama
        let prob_0 = curve.recall_probability(0.0);
        assert!((prob_0 - 1.0).abs() < 0.01);
        
        // 7 günde %37
        let prob_7 = curve.recall_probability(7.0);
        assert!(prob_7 > 0.3 && prob_7 < 0.5);
        
        // Optimal tekrar zamanı
        let review_time = curve.optimal_review_time(0.0);
        assert!(review_time > 0.0);
    }
    
    #[test]
    fn test_memory_health() {
        let health = MemoryHealth {
            total_memories: 100,
            average_importance: 0.7,
            critical_memories: 10,
            healthy_memories: 60,
            memory_pressure: 0.1,
        };
        
        assert!(health.is_healthy());
        assert_eq!(health.status(), "SAĞLIKLI");
        
        let critical_health = MemoryHealth {
            total_memories: 100,
            average_importance: 0.2,
            critical_memories: 60,
            healthy_memories: 10,
            memory_pressure: 0.6,
        };
        
        assert!(!critical_health.is_healthy());
        assert_eq!(critical_health.status(), "KRİTİK");
    }
}
