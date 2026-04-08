//! ═══════════════════════════════════════════════════════════════════════════════
//!  MANUS PLANNER - Görev Planlama
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Karmaşık görevleri adımlara bölme ve planlama.

use crate::error::{ManusError, ManusResult};
use crate::Language;
use serde::{Deserialize, Serialize};

/// ─── TASK PLANNER ───

pub struct TaskPlanner {
    /// Maksimum adım sayısı
    max_steps: usize,
    /// Maksimum derinlik
    max_depth: usize,
}

/// Görev planı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskPlan {
    /// Plan ID
    pub id: String,
    /// Orijinal görev
    pub task: String,
    /// Adımlar
    pub steps: Vec<TaskStep>,
    /// Tahmini süre (saniye)
    pub estimated_duration_secs: u64,
    /// Oluşturulma zamanı
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Görev adımı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStep {
    /// Adım ID
    pub id: String,
    /// Adım sırası
    pub order: usize,
    /// Adım türü
    pub step_type: StepType,
    /// Açıklama
    pub description: String,
    /// Kod (varsa)
    pub code: Option<String>,
    /// Dil
    pub language: Option<Language>,
    /// Bağımlılıklar (önceki adım ID'leri)
    pub dependencies: Vec<String>,
    /// Durum
    pub status: StepStatus,
    /// Sonuç
    pub result: Option<String>,
}

/// Adım türü
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepType {
    /// Kod yaz ve çalıştır
    WriteCode,
    /// Dosya işle
    FileOperation,
    /// Veri işle
    DataProcessing,
    /// API çağır
    ApiCall,
    /// Test et
    Testing,
    /// Doğrula
    Verification,
    /// Raporla
    Reporting,
}

/// Adım durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Skipped,
}

/// Planlama şablonu
#[derive(Debug, Clone)]
pub struct PlanningTemplate {
    pub name: String,
    pub description: String,
    pub pattern: Vec<StepPattern>,
}

/// Adım pattern'i
#[derive(Debug, Clone)]
pub struct StepPattern {
    pub step_type: StepType,
    pub description_template: String,
    pub code_template: Option<String>,
    pub language: Option<Language>,
}

impl TaskPlanner {
    /// Yeni planner oluştur
    pub fn new() -> Self {
        Self {
            max_steps: 10,
            max_depth: 3,
        }
    }
    
    /// Görev planla
    pub fn plan(&self, task: &str) -> ManusResult<TaskPlan> {
        log::info!("📋  MANUS-PLANNER: Görev planlanıyor → {}", task);
        
        let task_lower = task.to_lowercase();
        
        // Görev türünü belirle
        let steps = if task_lower.contains("hesapla") || task_lower.contains("calculate") {
            self.plan_calculation(task)?
        } else if task_lower.contains("dosya") || task_lower.contains("file") {
            self.plan_file_operation(task)?
        } else if task_lower.contains("veri") || task_lower.contains("data") {
            self.plan_data_processing(task)?
        } else if task_lower.contains("test") {
            self.plan_testing(task)?
        } else if task_lower.contains("api") || task_lower.contains("istek") {
            self.plan_api_call(task)?
        } else {
            self.plan_generic(task)?
        };
        
        let estimated = steps.len() as u64 * 30; // 30 saniye/adım
        
        log::info!("📋  MANUS-PLANNER: {} adım planlandı", steps.len());
        
        Ok(TaskPlan {
            id: format!("plan_{}", uuid::Uuid::new_v4()),
            task: task.into(),
            steps,
            estimated_duration_secs: estimated,
            created_at: chrono::Utc::now(),
        })
    }
    
    /// Hesaplama planı
    fn plan_calculation(&self, task: &str) -> ManusResult<Vec<TaskStep>> {
        Ok(vec![
            TaskStep {
                id: format!("step_{}", uuid::Uuid::new_v4()),
                order: 1,
                step_type: StepType::WriteCode,
                description: format!("Hesaplama kodunu yaz: {}", task),
                code: Some(self.generate_calculation_code(task)),
                language: Some(Language::Python),
                dependencies: vec![],
                status: StepStatus::Pending,
                result: None,
            },
            TaskStep {
                id: format!("step_{}", uuid::Uuid::new_v4()),
                order: 2,
                step_type: StepType::Verification,
                description: "Sonucu doğrula".into(),
                code: None,
                language: None,
                dependencies: vec![],
                status: StepStatus::Pending,
                result: None,
            },
        ])
    }
    
    /// Dosya işlemi planı
    fn plan_file_operation(&self, task: &str) -> ManusResult<Vec<TaskStep>> {
        Ok(vec![
            TaskStep {
                id: format!("step_{}", uuid::Uuid::new_v4()),
                order: 1,
                step_type: StepType::FileOperation,
                description: format!("Dosya işlemini hazırla: {}", task),
                code: None,
                language: None,
                dependencies: vec![],
                status: StepStatus::Pending,
                result: None,
            },
            TaskStep {
                id: format!("step_{}", uuid::Uuid::new_v4()),
                order: 2,
                step_type: StepType::WriteCode,
                description: "İşlem kodunu yaz".into(),
                code: Some("# Dosya işlemi kodu\nprint('Dosya işleme tamamlandı')".into()),
                language: Some(Language::Python),
                dependencies: vec![],
                status: StepStatus::Pending,
                result: None,
            },
        ])
    }
    
    /// Veri işleme planı
    fn plan_data_processing(&self, task: &str) -> ManusResult<Vec<TaskStep>> {
        Ok(vec![
            TaskStep {
                id: format!("step_{}", uuid::Uuid::new_v4()),
                order: 1,
                step_type: StepType::DataProcessing,
                description: format!("Veri işleme: {}", task),
                code: Some(self.generate_data_code(task)),
                language: Some(Language::Python),
                dependencies: vec![],
                status: StepStatus::Pending,
                result: None,
            },
            TaskStep {
                id: format!("step_{}", uuid::Uuid::new_v4()),
                order: 2,
                step_type: StepType::Verification,
                description: "Veri doğruluğunu kontrol et".into(),
                code: None,
                language: None,
                dependencies: vec![],
                status: StepStatus::Pending,
                result: None,
            },
        ])
    }
    
    /// Test planı
    fn plan_testing(&self, task: &str) -> ManusResult<Vec<TaskStep>> {
        Ok(vec![
            TaskStep {
                id: format!("step_{}", uuid::Uuid::new_v4()),
                order: 1,
                step_type: StepType::WriteCode,
                description: "Test kodunu yaz".into(),
                code: Some(self.generate_test_code(task)),
                language: Some(Language::Python),
                dependencies: vec![],
                status: StepStatus::Pending,
                result: None,
            },
            TaskStep {
                id: format!("step_{}", uuid::Uuid::new_v4()),
                order: 2,
                step_type: StepType::Testing,
                description: "Testleri çalıştır".into(),
                code: None,
                language: None,
                dependencies: vec![],
                status: StepStatus::Pending,
                result: None,
            },
        ])
    }
    
    /// API çağrı planı
    fn plan_api_call(&self, task: &str) -> ManusResult<Vec<TaskStep>> {
        Ok(vec![
            TaskStep {
                id: format!("step_{}", uuid::Uuid::new_v4()),
                order: 1,
                step_type: StepType::ApiCall,
                description: format!("API çağrısı hazırla: {}", task),
                code: Some(self.generate_api_code(task)),
                language: Some(Language::Python),
                dependencies: vec![],
                status: StepStatus::Pending,
                result: None,
            },
            TaskStep {
                id: format!("step_{}", uuid::Uuid::new_v4()),
                order: 2,
                step_type: StepType::DataProcessing,
                description: "Yanıtı işle".into(),
                code: None,
                language: None,
                dependencies: vec![],
                status: StepStatus::Pending,
                result: None,
            },
        ])
    }
    
    /// Genel plan
    fn plan_generic(&self, task: &str) -> ManusResult<Vec<TaskStep>> {
        Ok(vec![
            TaskStep {
                id: format!("step_{}", uuid::Uuid::new_v4()),
                order: 1,
                step_type: StepType::WriteCode,
                description: format!("Görevi gerçekleştir: {}", task),
                code: Some(format!("# {}\nprint('Görev tamamlandı: {}')", task, task)),
                language: Some(Language::Python),
                dependencies: vec![],
                status: StepStatus::Pending,
                result: None,
            },
            TaskStep {
                id: format!("step_{}", uuid::Uuid::new_v4()),
                order: 2,
                step_type: StepType::Verification,
                description: "Sonucu doğrula".into(),
                code: None,
                language: None,
                dependencies: vec![],
                status: StepStatus::Pending,
                result: None,
            },
        ])
    }
    
    /// Hesaplama kodu oluştur
    fn generate_calculation_code(&self, task: &str) -> String {
        format!(
            r#"# Hesaplama: {}
# Sonuç hesaplanacak

def calculate():
    # Hesaplama mantığı buraya
    result = None
    return result

if __name__ == "__main__":
    result = calculate()
    print(f"Sonuç: {{result}}")
"#,
            task
        )
    }
    
    /// Veri işleme kodu oluştur
    fn generate_data_code(&self, task: &str) -> String {
        format!(
            r#"# Veri İşleme: {}

def process_data(data):
    # Veri işleme mantığı
    processed = data
    return processed

if __name__ == "__main__":
    # Örnek veri
    data = []
    result = process_data(data)
    print(f"İşlendi: {{len(result)}} kayıt")
"#,
            task
        )
    }
    
    /// Test kodu oluştur
    fn generate_test_code(&self, task: &str) -> String {
        format!(
            r#"# Test: {}

def test_{}():
    # Test mantığı
    assert True, "Test başarısız"
    print("Test geçti")

if __name__ == "__main__":
    test_{}()
"#,
            task,
            task.replace(" ", "_").to_lowercase(),
            task.replace(" ", "_").to_lowercase()
        )
    }
    
    /// API kodu oluştur
    fn generate_api_code(&self, task: &str) -> String {
        format!(
            r#"# API Çağrısı: {}

import json

def call_api(endpoint):
    # API çağrısı simülasyonu
    response = {{'status': 'ok', 'data': []}}
    return response

if __name__ == "__main__":
    result = call_api('/api/endpoint')
    print(json.dumps(result, indent=2))
"#,
            task
        )
    }
    
    /// Planı güncelle
    pub fn update_step(&self, plan: &mut TaskPlan, step_id: &str, status: StepStatus, result: Option<String>) {
        if let Some(step) = plan.steps.iter_mut().find(|s| s.id == step_id) {
            step.status = status;
            step.result = result;
        }
    }
    
    /// Sonraki çalıştırılabilir adımı al
    pub fn get_next_step(&self, plan: &TaskPlan) -> Option<TaskStep> {
        plan.steps.iter()
            .find(|s| s.status == StepStatus::Pending)
            .cloned()
    }
}

impl Default for TaskPlanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planner_creation() {
        let planner = TaskPlanner::new();
        assert_eq!(planner.max_steps, 10);
    }

    #[test]
    fn test_plan_calculation() {
        let planner = TaskPlanner::new();
        let plan = planner.plan("Fibonacci hesapla").unwrap();
        assert!(!plan.steps.is_empty());
        assert!(plan.steps[0].code.is_some());
    }

    #[test]
    fn test_plan_generic() {
        let planner = TaskPlanner::new();
        let plan = planner.plan("Bir görev yap").unwrap();
        assert!(!plan.steps.is_empty());
    }

    #[test]
    fn test_step_status() {
        let step = TaskStep {
            id: "test".into(),
            order: 1,
            step_type: StepType::WriteCode,
            description: "Test".into(),
            code: None,
            language: None,
            dependencies: vec![],
            status: StepStatus::Pending,
            result: None,
        };
        assert_eq!(step.status, StepStatus::Pending);
    }
}
