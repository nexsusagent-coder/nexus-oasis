//! ─── YÜRÜTME SONUÇLARI ───
//!
//! Görev yürütme sonuçları ve raporlama veri yapıları.

use crate::goal::{Goal, Task, TaskResult};
use crate::state::AgentState;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// ─── EXECUTION RESULT ───
/// 
/// Bir hedefin tam yürütme sonucu.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// Benzersiz ID
    pub id: Uuid,
    /// İlgili hedef
    pub goal_id: Uuid,
    /// Hedef açıklaması
    pub goal_description: String,
    /// Başarı durumu
    pub success: bool,
    /// Sonuç özeti
    pub summary: String,
    /// Tam sonuç (detaylı)
    pub details: serde_json::Value,
    /// Tamamlanan görevler
    pub completed_tasks: Vec<StepResult>,
    /// Başarısız görevler
    pub failed_tasks: Vec<StepResult>,
    /// Toplam iterasyon
    pub total_iterations: u32,
    /// Toplam token
    pub total_tokens: u64,
    /// Süre (ms)
    pub duration_ms: u64,
    /// Bitiş durumu
    pub final_state: AgentState,
    /// Zaman damgası
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ExecutionResult {
    /// Başarılı sonuç oluştur
    pub fn success(
        goal: &Goal,
        completed: Vec<StepResult>,
        iterations: u32,
        tokens: u64,
        duration_ms: u64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            goal_id: goal.id,
            goal_description: goal.description.clone(),
            success: true,
            summary: "Görev başarıyla tamamlandı".into(),
            details: serde_json::Value::Null,
            completed_tasks: completed,
            failed_tasks: Vec::new(),
            total_iterations: iterations,
            total_tokens: tokens,
            duration_ms,
            final_state: AgentState::Completed,
            timestamp: chrono::Utc::now(),
        }
    }
    
    /// Başarısız sonuç oluştur
    pub fn failure(
        goal: &Goal,
        error: String,
        completed: Vec<StepResult>,
        failed: Vec<StepResult>,
        iterations: u32,
        tokens: u64,
        duration_ms: u64,
        final_state: AgentState,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            goal_id: goal.id,
            goal_description: goal.description.clone(),
            success: false,
            summary: format!("Görev başarısız: {}", error),
            details: serde_json::json!({ "error": error }),
            completed_tasks: completed,
            failed_tasks: failed,
            total_iterations: iterations,
            total_tokens: tokens,
            duration_ms,
            final_state,
            timestamp: chrono::Utc::now(),
        }
    }
    
    /// Özet rapor
    pub fn report(&self) -> String {
        let status = if self.success { "✅ BAŞARILI" } else { "❌ BAŞARISIZ" };
        let duration = if self.duration_ms < 1000 {
            format!("{}ms", self.duration_ms)
        } else {
            format!("{:.2}s", self.duration_ms as f64 / 1000.0)
        };
        
        format!(
            r#"
════════════════════════════════════════════════════════════
  🎯 YÜRÜTME SONUCU
════════════════════════════════════════════════════════════
  Durum:        {}
  Hedef:        {}
  Süre:         {}
  İterasyon:    {}
  Token:        {}
  ────────────────────────────────────────────────────────────
  ✓ Tamamlanan: {}
  ✗ Başarısız:  {}
────────────────────────────────────────────────────────────
  Özet: {}
════════════════════════════════════════════════════════════"#,
            status,
            self.goal_description.chars().take(50).collect::<String>(),
            duration,
            self.total_iterations,
            self.total_tokens,
            self.completed_tasks.len(),
            self.failed_tasks.len(),
            self.summary.chars().take(100).collect::<String>()
        )
    }
}

/// ─── STEP RESULT ───
/// 
/// Tek bir adımın sonucu.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    /// Görev ID
    pub task_id: Uuid,
    /// Görev açıklaması
    pub description: String,
    /// Kullanılan araç
    pub tool: String,
    /// Girdi parametreleri
    pub input: serde_json::Value,
    /// Çıktı
    pub output: serde_json::Value,
    /// Başarı
    pub success: bool,
    /// Hata mesajı
    pub error: Option<String>,
    /// Süre (ms)
    pub duration_ms: u64,
    /// Token kullanımı (varsa)
    pub tokens_used: Option<u64>,
}

impl StepResult {
    /// Görev sonucundan oluştur
    pub fn from_task(task: &Task, duration_ms: u64) -> Self {
        let (success, output, error) = match &task.result {
            Some(TaskResult::Success(v)) => (true, v.clone(), None),
            Some(TaskResult::Error(e)) => (false, serde_json::Value::Null, Some(e.clone())),
            Some(TaskResult::Partial { result, warnings }) => {
                (true, result.clone(), Some(warnings.join(", ")))
            }
            None => (false, serde_json::Value::Null, Some("Sonuç yok".into())),
        };
        
        Self {
            task_id: task.id,
            description: task.description.clone(),
            tool: format!("{:?}", task.tool),
            input: task.input.clone(),
            output,
            success,
            error,
            duration_ms,
            tokens_used: None,
        }
    }
    
    /// Kısa özet
    pub fn summary(&self) -> String {
        if self.success {
            format!("✓ {} ({}ms)", self.description.chars().take(40).collect::<String>(), self.duration_ms)
        } else {
            format!("✗ {} → {}", self.description.chars().take(30).collect::<String>(), self.error.as_deref().unwrap_or("?"))
        }
    }
}

/// ─── EXECUTION METRICS ───

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecutionMetrics {
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub total_duration_ms: u64,
    pub total_tokens: u64,
    pub total_tasks: u64,
    pub completed_tasks: u64,
    pub failed_tasks: u64,
}

impl ExecutionMetrics {
    pub fn record(&mut self, result: &ExecutionResult) {
        self.total_executions += 1;
        
        if result.success {
            self.successful_executions += 1;
        } else {
            self.failed_executions += 1;
        }
        
        self.total_duration_ms += result.duration_ms;
        self.total_tokens += result.total_tokens;
        self.total_tasks += result.completed_tasks.len() as u64 + result.failed_tasks.len() as u64;
        self.completed_tasks += result.completed_tasks.len() as u64;
        self.failed_tasks += result.failed_tasks.len() as u64;
    }
    
    pub fn success_rate(&self) -> f64 {
        if self.total_executions == 0 { return 0.0; }
        (self.successful_executions as f64 / self.total_executions as f64) * 100.0
    }
    
    pub fn avg_duration_ms(&self) -> f64 {
        if self.total_executions == 0 { return 0.0; }
        self.total_duration_ms as f64 / self.total_executions as f64
    }
    
    pub fn avg_tokens(&self) -> f64 {
        if self.total_executions == 0 { return 0.0; }
        self.total_tokens as f64 / self.total_executions as f64
    }
    
    pub fn report(&self) -> String {
        format!(
            r#"
════════════════════════════════════════════════════════════
  📊 YÜRÜTME METRİKLERİ
════════════════════════════════════════════════════════════
  Toplam Yürütme:    {}
  Başarı Oranı:      {:.1}%
  Ort. Süre:        {:.0}ms
  Ort. Token:       {:.0}
  ────────────────────────────────────────────────────────────
  Görevler:
    ✓ Tamamlanan:   {}
    ✗ Başarısız:    {}
════════════════════════════════════════════════════════════"#,
            self.total_executions,
            self.success_rate(),
            self.avg_duration_ms(),
            self.avg_tokens(),
            self.completed_tasks,
            self.failed_tasks
        )
    }
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    use crate::goal::{Goal, Task, ToolType};
    
    #[test]
    fn test_execution_result_success() {
        let goal = Goal::new("Test hedefi");
        let result = ExecutionResult::success(&goal, vec![], 5, 100, 1500);
        
        assert!(result.success);
        assert_eq!(result.total_iterations, 5);
        assert_eq!(result.total_tokens, 100);
    }
    
    #[test]
    fn test_execution_result_failure() {
        let goal = Goal::new("Test hedefi");
        let result = ExecutionResult::failure(
            &goal,
            "Test hatası".into(),
            vec![],
            vec![],
            3,
            50,
            800,
            AgentState::Error
        );
        
        assert!(!result.success);
        assert!(result.summary.contains("Test hatası"));
    }
    
    #[test]
    fn test_step_result_from_task() {
        let mut task = Task::new("Test görevi", ToolType::Calculator);
        task.complete(TaskResult::Success(serde_json::json!(42)));
        
        let result = StepResult::from_task(&task, 100);
        assert!(result.success);
        assert_eq!(result.duration_ms, 100);
    }
    
    #[test]
    fn test_execution_metrics() {
        let mut metrics = ExecutionMetrics::default();
        
        let goal = Goal::new("Test");
        let result1 = ExecutionResult::success(&goal, vec![], 5, 100, 1000);
        let result2 = ExecutionResult::success(&goal, vec![], 3, 50, 500);
        
        metrics.record(&result1);
        metrics.record(&result2);
        
        assert_eq!(metrics.total_executions, 2);
        assert_eq!(metrics.successful_executions, 2);
        assert_eq!(metrics.success_rate(), 100.0);
        assert_eq!(metrics.avg_duration_ms(), 750.0);
    }
    
    #[test]
    fn test_execution_metrics_failure() {
        let mut metrics = ExecutionMetrics::default();
        
        let goal = Goal::new("Test");
        let success = ExecutionResult::success(&goal, vec![], 1, 10, 100);
        let failure = ExecutionResult::failure(
            &goal, "Hata".into(), vec![], vec![], 1, 10, 100, AgentState::Error
        );
        
        metrics.record(&success);
        metrics.record(&failure);
        
        assert_eq!(metrics.success_rate(), 50.0);
        assert_eq!(metrics.failed_executions, 1);
    }
    
    #[test]
    fn test_result_report() {
        let goal = Goal::new("Test hedefi");
        let result = ExecutionResult::success(&goal, vec![], 5, 100, 1500);
        
        let report = result.report();
        assert!(report.contains("BAŞARILI"));
        assert!(report.contains("Test hedefi"));
    }
}
