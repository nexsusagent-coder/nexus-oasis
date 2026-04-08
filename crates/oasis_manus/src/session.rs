//! ═══════════════════════════════════════════════════════════════════════════════
//!  MANUS SESSION - Oturum Yönetimi
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Manus oturumları ve istatistikler.

use crate::error::ManusResult;
use crate::executor::ExecutionResult;
use serde::{Deserialize, Serialize};

/// ─── MANUS SESSION ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManusSession {
    /// Oturum ID
    pub id: String,
    /// Başlangıç zamanı
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// Bitiş zamanı
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    /// İşlem sayısı
    pub execution_count: u64,
    /// Başarılı işlem sayısı
    pub success_count: u64,
    /// Başarısız işlem sayısı
    pub failure_count: u64,
    /// Toplam süre (ms)
    pub total_duration_ms: u64,
    /// Kullanılan container'lar
    pub containers: Vec<String>,
    /// Sonuçlar
    pub results: Vec<ExecutionResult>,
}

impl ManusSession {
    /// Yeni oturum oluştur
    pub fn new() -> Self {
        Self {
            id: format!("session_{}", uuid::Uuid::new_v4()),
            started_at: chrono::Utc::now(),
            ended_at: None,
            execution_count: 0,
            success_count: 0,
            failure_count: 0,
            total_duration_ms: 0,
            containers: Vec::new(),
            results: Vec::new(),
        }
    }
    
    /// Sonuç kaydet
    pub fn record_result(&mut self, result: &ExecutionResult) {
        self.execution_count += 1;
        self.total_duration_ms += result.duration_ms;
        
        if result.success {
            self.success_count += 1;
        } else {
            self.failure_count += 1;
        }
        
        if !self.containers.contains(&result.container_id) {
            self.containers.push(result.container_id.clone());
        }
        
        self.results.push(result.clone());
    }
    
    /// Oturumu kapat
    pub fn end(&mut self) {
        self.ended_at = Some(chrono::Utc::now());
    }
    
    /// Oturum süresi (ms)
    pub fn duration_ms(&self) -> i64 {
        match self.ended_at {
            Some(end) => (end - self.started_at).num_milliseconds(),
            None => (chrono::Utc::now() - self.started_at).num_milliseconds(),
        }
    }
    
    /// Başarı oranı
    pub fn success_rate(&self) -> f32 {
        if self.execution_count == 0 {
            return 0.0;
        }
        self.success_count as f32 / self.execution_count as f32
    }
    
    /// Özet
    pub fn summary(&self) -> SessionSummary {
        SessionSummary {
            session_id: self.id.clone(),
            total_executions: self.execution_count,
            success_rate: self.success_rate(),
            total_duration_ms: self.total_duration_ms,
            containers_used: self.containers.len(),
        }
    }
}

impl Default for ManusSession {
    fn default() -> Self {
        Self::new()
    }
}

/// Oturum özeti
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    pub session_id: String,
    pub total_executions: u64,
    pub success_rate: f32,
    pub total_duration_ms: u64,
    pub containers_used: usize,
}

/// Oturum istatistikleri
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionStats {
    /// Toplam oturum sayısı
    pub total_sessions: u64,
    /// Toplam işlem sayısı
    pub total_executions: u64,
    /// Toplam başarılı
    pub total_success: u64,
    /// Toplam başarısız
    pub total_failures: u64,
    /// Toplam süre (ms)
    pub total_duration_ms: u64,
    /// Ortalama başarı oranı
    pub avg_success_rate: f32,
}

impl SessionStats {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn update(&mut self, session: &ManusSession) {
        self.total_sessions += 1;
        self.total_executions += session.execution_count;
        self.total_success += session.success_count;
        self.total_failures += session.failure_count;
        self.total_duration_ms += session.total_duration_ms;
        
        // Ortalama başarı oranını güncelle
        if self.total_executions > 0 {
            self.avg_success_rate = self.total_success as f32 / self.total_executions as f32;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::executor::ExecutionLanguage;

    #[test]
    fn test_session_creation() {
        let session = ManusSession::new();
        assert_eq!(session.execution_count, 0);
    }

    #[test]
    fn test_session_record_result() {
        let mut session = ManusSession::new();
        let result = ExecutionResult {
            container_id: "test".into(),
            success: true,
            exit_code: Some(0),
            stdout: "ok".into(),
            stderr: String::new(),
            duration_ms: 100,
            error: None,
            language: ExecutionLanguage::Python,
        };
        
        session.record_result(&result);
        assert_eq!(session.execution_count, 1);
        assert_eq!(session.success_count, 1);
    }

    #[test]
    fn test_session_success_rate() {
        let mut session = ManusSession::new();
        
        let success = ExecutionResult {
            container_id: "test".into(),
            success: true,
            exit_code: Some(0),
            stdout: "ok".into(),
            stderr: String::new(),
            duration_ms: 100,
            error: None,
            language: ExecutionLanguage::Python,
        };
        
        let failure = ExecutionResult {
            container_id: "test".into(),
            success: false,
            exit_code: Some(1),
            stdout: String::new(),
            stderr: "error".into(),
            duration_ms: 50,
            error: Some("error".into()),
            language: ExecutionLanguage::Python,
        };
        
        session.record_result(&success);
        session.record_result(&success);
        session.record_result(&failure);
        
        assert_eq!(session.success_rate(), 2.0 / 3.0);
    }

    #[test]
    fn test_session_summary() {
        let session = ManusSession::new();
        let summary = session.summary();
        assert_eq!(summary.total_executions, 0);
    }

    #[test]
    fn test_session_stats() {
        let mut stats = SessionStats::new();
        let session = ManusSession::new();
        
        stats.update(&session);
        assert_eq!(stats.total_sessions, 1);
    }
}
