//! ═══════════════════════════════════════════════════════════════════════════════
//!  SELF-HEALING - Self-Healing System
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Hata tespiti, teşhis ve otomatik kurtarma sistemi.
//!
//! HEALING SÜRECİ:
//! ───────────────
//! 1. DETECT   → Anomali tespiti
//! 2. DIAGNOSE → Kök neden analizi
//! 3. RECOVER  → Kurtarma aksiyonu
//! 4. VERIFY   → Başarı kontrolü
//! 5. LEARN    → Gelecek için öğrenme
//!
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                        HEALING FLOW                                     │
//! │                                                                          │
//! │   Error ──► Detect ──► Diagnose ──► Strategy ──► Recover ──► Verify    │
//! │                │            │             │             │              │
//! │                ▼            ▼             ▼             ▼              │
//! │            [Anomaly]    [Root Cause]  [Strategy]   [Action]            │
//! │                                                                          │
//! │   Recovery Strategies:                                                   │
//! │   - Retry with delay                                                     │
//! │   - Alternative approach                                                 │
//! │   - Rollback to checkpoint                                               │
//! │   - Ask for human help                                                   │
//! │   - Graceful degradation                                                 │
//! └─────────────────────────────────────────────────────────────────────────┘

use crate::error::{AutonomousError, AutonomousResult};
use crate::agent_loop::AgentContext;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// ═══════════════════════════════════════════════════════════════════════════════
//  HEALTH STATUS
// ═══════════════════════════════════════════════════════════════════════════════

/// Sağlık durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    /// Sağlıklı
    Healthy,
    /// Uyarı
    Warning,
    /// Hata
    Error,
    /// Kritik
    Critical,
    /// Kurtarılıyor
    Recovering,
    /// Kurtarılamaz
    Unrecoverable,
}

impl Default for HealthStatus {
    fn default() -> Self {
        Self::Healthy
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ANOMALY
// ═══════════════════════════════════════════════════════════════════════════════

/// Anomali
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    /// Anomali ID
    pub id: String,
    /// Anomali türü
    pub anomaly_type: AnomalyType,
    /// Zaman damgası
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Açıklama
    pub description: String,
    /// Şiddet (1-10)
    pub severity: u8,
    /// Bağlam
    pub context: HashMap<String, String>,
}

/// Anomali türü
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnomalyType {
    /// Uzun süre yanıt yok
    Timeout,
    /// Kaynak tükendi
    ResourceExhaustion,
    /// Döngü tespit edildi
    LoopDetected,
    /// Element bulunamadı
    ElementNotFound,
    /// Aksiyon başarısız
    ActionFailed,
    /// Beklenmeyen durum
    UnexpectedState,
    /// Bellek sorunu
    MemoryIssue,
    /// Ağ sorunu
    NetworkIssue,
    /// Uygulama çöktü
    ApplicationCrash,
    /// İzin hatası
    PermissionDenied,
    /// Özel
    Custom,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DIAGNOSIS
// ═══════════════════════════════════════════════════════════════════════════════

/// Teşhis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnosis {
    /// Teşhis ID
    pub id: String,
    /// Anomali ID
    pub anomaly_id: String,
    /// Kök neden
    pub root_cause: RootCause,
    /// Önerilen stratejiler
    pub recovery_strategies: Vec<RecoveryStrategy>,
    /// Güven skoru
    pub confidence: f32,
}

/// Kök neden
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RootCause {
    /// Ağ bağlantısı kopmuş
    NetworkDisconnected,
    /// Sayfa yüklenmedi
    PageNotLoaded,
    /// Element değişmiş
    ElementChanged,
    /// Zaman aşımı
    Timeout,
    /// Kaynak yetersiz
    InsufficientResources,
    /// İzin reddedildi
    PermissionDenied,
    /// Uygulama hatası
    ApplicationError,
    /// Kullanıcı hatası
    UserError,
    /// Bilinmiyor
    Unknown,
}

/// Kurtarma stratejisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryStrategy {
    /// Retry
    Retry { 
        max_attempts: u32, 
        delay_ms: u64,
    },
    /// Alternatif yaklaşım
    AlternativeApproach { 
        description: String,
    },
    /// Rollback
    Rollback { 
        checkpoint_id: String,
    },
    /// Yeniden başlat
    Restart { 
        component: String,
    },
    /// İnsan yardımı iste
    RequestHumanHelp { 
        message: String,
    },
    /// Graceful degradation
    GracefulDegradation { 
        fallback_action: String,
    },
    /// Atla
    Skip { 
        reason: String,
    },
    /// Durdur
    Abort { 
        reason: String,
    },
}

// ═══════════════════════════════════════════════════════════════════════════════
//  RECOVERY ACTION
// ═══════════════════════════════════════════════════════════════════════════════

/// Kurtarma aksiyonu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryAction {
    /// Aksiyon ID
    pub id: String,
    /// Strateji
    pub strategy: RecoveryStrategy,
    /// Başlama zamanı
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// Bitiş zamanı
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Başarılı mı?
    pub success: bool,
    /// Mesaj
    pub message: String,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CHECKPOINT
// ═══════════════════════════════════════════════════════════════════════════════

/// Checkpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    /// Checkpoint ID
    pub id: String,
    /// Zaman damgası
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Açıklama
    pub description: String,
    /// Durum verisi
    pub state: HashMap<String, serde_json::Value>,
    /// Ekran görüntüsü (base64)
    pub screenshot: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SELF-HEALING ENGINE
// ═══════════════════════════════════════════════════════════════════════════════

/// Self-healing motoru
pub struct SelfHealing {
    /// Sağlık durumu
    health: Arc<RwLock<HealthStatus>>,
    /// Anomaliler
    anomalies: Vec<Anomaly>,
    /// Checkpointler
    checkpoints: Vec<Checkpoint>,
    /// Maksimum checkpoint
    max_checkpoints: usize,
    /// Son teşhis
    last_diagnosis: Option<Diagnosis>,
    /// Kurtarma geçmişi
    recovery_history: Vec<RecoveryAction>,
    /// Retry sayacı
    retry_counts: HashMap<String, u32>,
}

impl SelfHealing {
    pub fn new() -> Self {
        log::info!("💊 HEALING: Self-healing motoru başlatılıyor...");
        
        Self {
            health: Arc::new(RwLock::new(HealthStatus::Healthy)),
            anomalies: Vec::new(),
            checkpoints: Vec::new(),
            max_checkpoints: 20,
            last_diagnosis: None,
            recovery_history: Vec::new(),
            retry_counts: HashMap::new(),
        }
    }
    
    /// Hata durumunda kurtar
    pub async fn recover(&mut self, error: &AutonomousError, context: &AgentContext) -> AutonomousResult<()> {
        log::warn!("💊 HEALING: Error detected, initiating recovery...");
        
        // 1. Sağlık durumunu güncelle
        *self.health.write().await = HealthStatus::Recovering;
        
        // 2. Anomali oluştur
        let anomaly = self.detect_anomaly(error, context).await;
        self.anomalies.push(anomaly.clone());
        
        // 3. Teşhis koy
        let diagnosis = self.diagnose(&anomaly).await;
        self.last_diagnosis = Some(diagnosis.clone());
        
        // 4. Kurtarma stratejisi seç ve uygula
        if let Some(strategy) = diagnosis.recovery_strategies.first() {
            let action = self.apply_strategy(strategy, &anomaly).await?;
            
            if action.success {
                log::info!("💊 HEALING: Recovery successful");
                *self.health.write().await = HealthStatus::Healthy;
            } else {
                log::warn!("💊 HEALING: Recovery failed: {}", action.message);
                *self.health.write().await = HealthStatus::Error;
            }
            
            self.recovery_history.push(action);
        }
        
        Ok(())
    }
    
    /// Anomali tespit et
    async fn detect_anomaly(&self, error: &AutonomousError, _context: &AgentContext) -> Anomaly {
        let anomaly_type = match error {
            AutonomousError::AgentTimeout(_) => AnomalyType::Timeout,
            AutonomousError::ElementNotFound(_) => AnomalyType::ElementNotFound,
            AutonomousError::ActionTimeout(_) => AnomalyType::Timeout,
            AutonomousError::SafetyViolation(_) => AnomalyType::PermissionDenied,
            AutonomousError::LoopDetected(_) => AnomalyType::LoopDetected,
            _ => AnomalyType::Custom,
        };
        
        let severity = match error {
            AutonomousError::SafetyViolation(_) => 10,
            AutonomousError::AgentStopped(_) => 9,
            AutonomousError::MaxIterationsExceeded(_) => 7,
            AutonomousError::ElementNotFound(_) => 3,
            _ => 5,
        };
        
        Anomaly {
            id: uuid::Uuid::new_v4().to_string(),
            anomaly_type,
            timestamp: chrono::Utc::now(),
            description: error.to_string(),
            severity,
            context: HashMap::new(),
        }
    }
    
    /// Teşhis koy
    async fn diagnose(&self, anomaly: &Anomaly) -> Diagnosis {
        let root_cause = match anomaly.anomaly_type {
            AnomalyType::Timeout => RootCause::Timeout,
            AnomalyType::NetworkIssue => RootCause::NetworkDisconnected,
            AnomalyType::ElementNotFound => RootCause::ElementChanged,
            AnomalyType::PermissionDenied => RootCause::PermissionDenied,
            AnomalyType::ApplicationCrash => RootCause::ApplicationError,
            _ => RootCause::Unknown,
        };
        
        let strategies = self.suggest_strategies(&root_cause, anomaly);
        
        Diagnosis {
            id: uuid::Uuid::new_v4().to_string(),
            anomaly_id: anomaly.id.clone(),
            root_cause,
            recovery_strategies: strategies,
            confidence: 0.8,
        }
    }
    
    /// Strateji öner
    fn suggest_strategies(&self, root_cause: &RootCause, _anomaly: &Anomaly) -> Vec<RecoveryStrategy> {
        match root_cause {
            RootCause::Timeout => vec![
                RecoveryStrategy::Retry { max_attempts: 3, delay_ms: 1000 },
                RecoveryStrategy::AlternativeApproach { description: "Try alternative method".into() },
            ],
            
            RootCause::NetworkDisconnected => vec![
                RecoveryStrategy::Retry { max_attempts: 5, delay_ms: 5000 },
                RecoveryStrategy::RequestHumanHelp { message: "Network connection lost".into() },
            ],
            
            RootCause::ElementChanged => vec![
                RecoveryStrategy::AlternativeApproach { description: "Search for element again".into() },
                RecoveryStrategy::Rollback { checkpoint_id: "last".into() },
            ],
            
            RootCause::PermissionDenied => vec![
                RecoveryStrategy::RequestHumanHelp { message: "Permission denied".into() },
                RecoveryStrategy::Abort { reason: "Cannot proceed without permission".into() },
            ],
            
            RootCause::ApplicationError => vec![
                RecoveryStrategy::Restart { component: "application".into() },
                RecoveryStrategy::Rollback { checkpoint_id: "last".into() },
            ],
            
            RootCause::Unknown => vec![
                RecoveryStrategy::Retry { max_attempts: 1, delay_ms: 1000 },
                RecoveryStrategy::RequestHumanHelp { message: "Unknown error".into() },
            ],
            
            _ => vec![
                RecoveryStrategy::Retry { max_attempts: 2, delay_ms: 1000 },
            ],
        }
    }
    
    /// Strateji uygula
    async fn apply_strategy(&mut self, strategy: &RecoveryStrategy, anomaly: &Anomaly) -> AutonomousResult<RecoveryAction> {
        let action_id = uuid::Uuid::new_v4().to_string();
        let started_at = chrono::Utc::now();
        
        log::info!("💊 HEALING: Applying strategy: {:?}", strategy);
        
        let (success, message) = match strategy {
            RecoveryStrategy::Retry { max_attempts, delay_ms } => {
                // Retry count tracking
                let retry_key = format!("{}-{:?}", anomaly.id, anomaly.anomaly_type);
                let count = self.retry_counts.entry(retry_key).or_insert(0);
                
                if *count < *max_attempts {
                    *count += 1;
                    tokio::time::sleep(std::time::Duration::from_millis(*delay_ms)).await;
                    (true, format!("Retry {} of {}", *count, max_attempts))
                } else {
                    (false, "Max retries exceeded".into())
                }
            }
            
            RecoveryStrategy::Skip { reason } => {
                (true, format!("Skipped: {}", reason))
            }
            
            RecoveryStrategy::GracefulDegradation { fallback_action } => {
                (true, format!("Fallback: {}", fallback_action))
            }
            
            RecoveryStrategy::RequestHumanHelp { message } => {
                log::warn!("💊 HEALING: Human help requested: {}", message);
                (false, format!("Human help needed: {}", message))
            }
            
            RecoveryStrategy::Abort { reason } => {
                (false, format!("Aborted: {}", reason))
            }
            
            RecoveryStrategy::Rollback { checkpoint_id } => {
                if let Some(checkpoint) = self.find_checkpoint(checkpoint_id) {
                    log::info!("💊 HEALING: Rolling back to checkpoint {}", checkpoint.id);
                    (true, format!("Rolled back to {}", checkpoint.id))
                } else {
                    (false, "Checkpoint not found".into())
                }
            }
            
            RecoveryStrategy::Restart { component } => {
                log::info!("💊 HEALING: Restarting component: {}", component);
                (true, format!("Restarted: {}", component))
            }
            
            RecoveryStrategy::AlternativeApproach { description } => {
                log::info!("💊 HEALING: Trying alternative: {}", description);
                (true, description.clone())
            }
        };
        
        Ok(RecoveryAction {
            id: action_id,
            strategy: strategy.clone(),
            started_at,
            finished_at: Some(chrono::Utc::now()),
            success,
            message,
        })
    }
    
    /// Checkpoint oluştur
    pub fn create_checkpoint(&mut self, description: &str, state: HashMap<String, serde_json::Value>) -> String {
        let checkpoint = Checkpoint {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            description: description.into(),
            state,
            screenshot: None,
        };
        
        let id = checkpoint.id.clone();
        
        self.checkpoints.push(checkpoint);
        
        // Limite göre temizle
        while self.checkpoints.len() > self.max_checkpoints {
            self.checkpoints.remove(0);
        }
        
        log::debug!("💊 HEALING: Checkpoint created: {}", id);
        
        id
    }
    
    /// Checkpoint bul
    fn find_checkpoint(&self, id: &str) -> Option<&Checkpoint> {
        if id == "last" {
            self.checkpoints.last()
        } else {
            self.checkpoints.iter().find(|c| c.id == id)
        }
    }
    
    /// Sağlık durumu
    pub async fn health(&self) -> HealthStatus {
        *self.health.read().await
    }
    
    /// Anomali sayısı
    pub fn anomaly_count(&self) -> usize {
        self.anomalies.len()
    }
    
    /// İstatistikler
    pub fn stats(&self) -> HealingStats {
        HealingStats {
            total_anomalies: self.anomalies.len(),
            total_recoveries: self.recovery_history.len(),
            successful_recoveries: self.recovery_history.iter().filter(|r| r.success).count(),
            checkpoint_count: self.checkpoints.len(),
        }
    }
}

impl Default for SelfHealing {
    fn default() -> Self {
        Self::new()
    }
}

/// Healing istatistikleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealingStats {
    pub total_anomalies: usize,
    pub total_recoveries: usize,
    pub successful_recoveries: usize,
    pub checkpoint_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_healing_creation() {
        let healing = SelfHealing::new();
        assert_eq!(healing.health().await, HealthStatus::Healthy);
    }
    
    #[tokio::test]
    async fn test_create_checkpoint() {
        let mut healing = SelfHealing::new();
        let id = healing.create_checkpoint("test", HashMap::new());
        
        assert!(!id.is_empty());
        assert_eq!(healing.checkpoints.len(), 1);
    }
    
    #[tokio::test]
    async fn test_recover() {
        let mut healing = SelfHealing::new();
        let context = AgentContext::default();
        let error = AutonomousError::ElementNotFound("test".into());
        
        let result = healing.recover(&error, &context).await;
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_anomaly_severity() {
        let healing = SelfHealing::new();
        
        let error = AutonomousError::SafetyViolation("test".into());
        let anomaly = tokio_test::block_on(healing.detect_anomaly(&error, &AgentContext::default()));
        
        assert_eq!(anomaly.severity, 10);
    }
}
