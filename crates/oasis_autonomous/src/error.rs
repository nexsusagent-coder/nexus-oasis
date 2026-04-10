//! ═══════════════════════════════════════════════════════════════════════════════
//!  ERROR TYPES - Autonomous System Errors
//! ═══════════════════════════════════════════════════════════════════════════════

use thiserror::Error;

/// Autonomous sistem hatası
#[derive(Debug, Error)]
pub enum AutonomousError {
    // ───────────────────────────────────────────────────────────────────────────
    //  AGENT ERRORS
    // ─────────────────────────────────────────────────────────────────────────--
    #[error("Agent başlatılamadı: {0}")]
    AgentInitializationFailed(String),
    
    #[error("Agent durduruldu: {0}")]
    AgentStopped(String),
    
    #[error("Agent timeout: {0}s")]
    AgentTimeout(u64),
    
    #[error("Maksimum iterasyon aşıldı: {0}")]
    MaxIterationsExceeded(usize),
    
    // ───────────────────────────────────────────────────────────────────────────
    //  PERCEPTION ERRORS
    // ─────────────────────────────────────────────────────────────────────────--
    #[error("Ekran yakalanamadı: {0}")]
    ScreenCaptureFailed(String),
    
    #[error("Görüntü analiz hatası: {0}")]
    VisionAnalysisFailed(String),
    
    #[error("Element bulunamadı: {0}")]
    ElementNotFound(String),
    
    #[error("OCR hatası: {0}")]
    OCRError(String),
    
    // ───────────────────────────────────────────────────────────────────────────
    //  ACTION ERRORS
    // ─────────────────────────────────────────────────────────────────────────--
    #[error("Mouse aksiyonu başarısız: {0}")]
    MouseActionFailed(String),
    
    #[error("Klavye aksiyonu başarısız: {0}")]
    KeyboardActionFailed(String),
    
    #[error("Browser aksiyonu başarısız: {0}")]
    BrowserActionFailed(String),
    
    #[error("Aksiyon timeout: {0}")]
    ActionTimeout(String),
    
    #[error("Aksiyon iptal edildi: {0}")]
    ActionCancelled(String),
    
    // ───────────────────────────────────────────────────────────────────────────
    //  SAFETY ERRORS
    // ─────────────────────────────────────────────────────────────────────────--
    #[error("Güvenlik ihlali: {0}")]
    SafetyViolation(String),
    
    #[error("İnsan onayı gerekli: {0}")]
    HumanApprovalRequired(String),
    
    #[error("Yasaklı bölge: ({0}, {1})")]
    ForbiddenRegion(i32, i32),
    
    #[error("Maksimum hata sayısı aşıldı: {0}")]
    MaxErrorsExceeded(usize),
    
    // ───────────────────────────────────────────────────────────────────────────
    //  PLANNING ERRORS
    // ─────────────────────────────────────────────────────────────────────────--
    #[error("Görev planlanamadı: {0}")]
    PlanningFailed(String),
    
    #[error("Hedefe ulaşılamadı: {0}")]
    GoalUnreachable(String),
    
    #[error("Geçersiz hedef: {0}")]
    InvalidTarget(String),
    
    #[error("Döngü tespit edildi: {0}")]
    LoopDetected(String),
    
    // ───────────────────────────────────────────────────────────────────────────
    //  MEMORY ERRORS
    // ─────────────────────────────────────────────────────────────────────────--
    #[error("Bellek hatası: {0}")]
    MemoryError(String),
    
    #[error("Episode kaydedilemedi: {0}")]
    EpisodeSaveFailed(String),
    
    #[error("Öğrenme hatası: {0}")]
    LearningError(String),
    
    // ───────────────────────────────────────────────────────────────────────────
    //  ORCHESTRATION ERRORS
    // ─────────────────────────────────────────────────────────────────────────--
    #[error("Agent koordinasyon hatası: {0}")]
    OrchestrationError(String),
    
    #[error("Agent iletişim hatası: {0}")]
    AgentCommunicationError(String),
    
    #[error("Agent çakışması: {0}")]
    AgentConflict(String),
    
    // ───────────────────────────────────────────────────────────────────────────
    //  HEALING ERRORS
    // ─────────────────────────────────────────────────────────────────────────--
    #[error("Self-healing başarısız: {0}")]
    HealingFailed(String),
    
    #[error("Kurtarma mümkün değil: {0}")]
    RecoveryImpossible(String),
    
    // ───────────────────────────────────────────────────────────────────────────
    //  SYSTEM ERRORS
    // ─────────────────────────────────────────────────────────────────────────--
    #[error("IO hatası: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serileştirme hatası: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Genel hata: {0}")]
    Other(String),
}

/// Autonomous sonuç tipi
pub type AutonomousResult<T> = Result<T, AutonomousError>;

impl AutonomousError {
    /// Hata kritik mi?
    pub fn is_critical(&self) -> bool {
        matches!(self, 
            AutonomousError::SafetyViolation(_) |
            AutonomousError::HumanApprovalRequired(_) |
            AutonomousError::MaxErrorsExceeded(_) |
            AutonomousError::AgentStopped(_)
        )
    }
    
    /// Hata kurtarılabilir mi?
    pub fn is_recoverable(&self) -> bool {
        matches!(self,
            AutonomousError::ElementNotFound(_) |
            AutonomousError::ActionTimeout(_) |
            AutonomousError::OCRError(_) |
            AutonomousError::VisionAnalysisFailed(_)
        )
    }
    
    /// Retry yapılabilir mi?
    pub fn can_retry(&self) -> bool {
        self.is_recoverable() && !self.is_critical()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_critical() {
        let err = AutonomousError::SafetyViolation("test".into());
        assert!(err.is_critical());
        
        let err2 = AutonomousError::ElementNotFound("test".into());
        assert!(!err2.is_critical());
    }
    
    #[test]
    fn test_error_recoverable() {
        let err = AutonomousError::ElementNotFound("test".into());
        assert!(err.is_recoverable());
        
        let err2 = AutonomousError::SafetyViolation("test".into());
        assert!(!err2.is_recoverable());
    }
    
    #[test]
    fn test_error_can_retry() {
        let err = AutonomousError::ElementNotFound("test".into());
        assert!(err.can_retry());
        
        let err2 = AutonomousError::SafetyViolation("test".into());
        assert!(!err2.can_retry());
    }
}
