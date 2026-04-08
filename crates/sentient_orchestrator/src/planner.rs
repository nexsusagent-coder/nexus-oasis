//! ─── GÖREV PLANLAYICI ───
//!
//! Verilen bir hedefi mantıksal alt görevlere bölen planlama sistemi.

use crate::goal::{Goal, Task, TaskPriority, TaskStatus, ToolType};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

/// ─── PLANNER ───
/// 
/// Hedefleri alt görevlere bölen planlayıcı.

pub struct Planner {
    /// Plan şablonları (önceki deneyimlerden)
    templates: HashMap<String, PlanTemplate>,
    /// Plan geçmişi
    history: Vec<ExecutionPlan>,
}

impl Planner {
    /// Yeni planlayıcı oluştur
    pub fn new() -> Self {
        let mut planner = Self {
            templates: HashMap::new(),
            history: Vec::new(),
        };
        planner.load_default_templates();
        planner
    }
    
    /// Varsayılan şablonları yükle
    fn load_default_templates(&mut self) {
        // Araştırma şablonu
        self.templates.insert("research".into(), PlanTemplate {
            name: "Araştırma".into(),
            description: "Bir konuyu araştır ve raporla".into(),
            task_templates: vec![
                TaskTemplate {
                    description: "Web'de ara".into(),
                    tool: ToolType::WebSearch,
                    order: 0,
                },
                TaskTemplate {
                    description: "Bulguları kaydet".into(),
                    tool: ToolType::MemoryStore,
                    order: 1,
                },
                TaskTemplate {
                    description: "Rapor oluştur".into(),
                    tool: ToolType::LlmQuery,
                    order: 2,
                },
            ],
        });
        
        // Kod yazma şablonu
        self.templates.insert("code".into(), PlanTemplate {
            name: "Kod Yazma".into(),
            description: "Spesifikasyona göre kod yaz".into(),
            task_templates: vec![
                TaskTemplate {
                    description: "Spesifikasyonu analiz et".into(),
                    tool: ToolType::LlmReason,
                    order: 0,
                },
                TaskTemplate {
                    description: "Kodu yaz".into(),
                    tool: ToolType::SandboxExecute,
                    order: 1,
                },
                TaskTemplate {
                    description: "Kodu test et".into(),
                    tool: ToolType::SandboxTest,
                    order: 2,
                },
                TaskTemplate {
                    description: "Sonucu kaydet".into(),
                    tool: ToolType::MemoryStore,
                    order: 3,
                },
            ],
        });
        
        // Web görevi şablonu
        self.templates.insert("web".into(), PlanTemplate {
            name: "Web Görevi".into(),
            description: "Web'de bir görev gerçekleştir".into(),
            task_templates: vec![
                TaskTemplate {
                    description: "Siteye git".into(),
                    tool: ToolType::BrowserNavigate,
                    order: 0,
                },
                TaskTemplate {
                    description: "Sayfayı analiz et".into(),
                    tool: ToolType::BrowserExtract,
                    order: 1,
                },
                TaskTemplate {
                    description: "Eylem gerçekleştir".into(),
                    tool: ToolType::BrowserClick,
                    order: 2,
                },
            ],
        });
    }
    
    /// Hedef için plan oluştur
    pub fn plan(&self, goal: &Goal) -> ExecutionPlan {
        log::info!("📋  PLANLAMA: {}", goal.description.chars().take(50).collect::<String>());
        
        // Hedef türünü belirle
        let goal_type = self.classify_goal(&goal.description);
        
        // Şablon varsa kullan
        if let Some(template) = self.templates.get(&goal_type) {
            log::debug!("📋  Şablon kullanılıyor: {}", template.name);
            return self.instantiate_template(template, goal);
        }
        
        // Şablon yoksa dinamik plan oluştur
        self.create_dynamic_plan(goal)
    }
    
    /// Hedef sınıflandırması
    fn classify_goal(&self, description: &str) -> String {
        let lower = description.to_lowercase();
        
        // Anahtar kelimelere göre sınıflandır
        if lower.contains("araştır") || lower.contains("bul") || lower.contains("nedir") {
            "research".into()
        } else if lower.contains("kod") || lower.contains("program") || lower.contains("script") {
            "code".into()
        } else if lower.contains("web") || lower.contains("site") || lower.contains("sayfa") {
            "web".into()
        } else {
            "general".into()
        }
    }
    
    /// Şablondan plan oluştur
    fn instantiate_template(&self, template: &PlanTemplate, goal: &Goal) -> ExecutionPlan {
        let mut tasks: Vec<Task> = Vec::new();
        
        for (i, tt) in template.task_templates.iter().enumerate() {
            let mut task = Task::new(
                format!("{}. {}", i + 1, tt.description),
                tt.tool.clone()
            );
            task.dependencies = if i > 0 {
                vec![tasks[i - 1].id]
            } else {
                Vec::new()
            };
            tasks.push(task);
        }
        
        ExecutionPlan {
            id: Uuid::new_v4(),
            goal_id: goal.id,
            tasks,
            strategy: PlanStrategy::Sequential,
            created_at: chrono::Utc::now(),
            completed: false,
        }
    }
    
    /// Dinamik plan oluştur (LLM ile)
    fn create_dynamic_plan(&self, goal: &Goal) -> ExecutionPlan {
        log::debug!("📋  Dinamik plan oluşturuluyor...");
        
        // Basit bir varsayılan plan
        let tasks = vec![
            Task::new("1. Hedefi analiz et", ToolType::LlmReason),
            Task::new("2. Gerekli bilgileri topla", ToolType::WebSearch),
            Task::new("3. Görevi gerçekleştir", ToolType::SandboxExecute),
            Task::new("4. Sonucu doğrula", ToolType::LlmQuery),
            Task::new("5. Sonucu kaydet", ToolType::MemoryStore),
        ];
        
        ExecutionPlan {
            id: Uuid::new_v4(),
            goal_id: goal.id,
            tasks,
            strategy: PlanStrategy::Sequential,
            created_at: chrono::Utc::now(),
            completed: false,
        }
    }
    
    /// LLM'den gelen karar ile planı güncelle
    pub fn adapt_plan(&self, plan: &mut ExecutionPlan, llm_response: &str) {
        // LLM'den gelen yanıtı parse et ve planı uyarlar
        // Bu metod ileride daha gelişmiş hale getirilecek
        
        if llm_response.contains("başarısız") || llm_response.contains("hata") {
            log::warn!("📋  Plan uyarlanıyor: Hata tespit edildi");
            // Yeniden deneme görevi ekle
        }
        
        if llm_response.contains("tamamlandı") || llm_response.contains("bitti") {
            log::info!("📋  Plan tamamlandı olarak işaretleniyor");
            plan.completed = true;
        }
    }
}

impl Default for Planner {
    fn default() -> Self {
        Self::new()
    }
}

/// ─── EXECUTION PLAN ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    /// Plan ID
    pub id: Uuid,
    /// İlgili hedef
    pub goal_id: Uuid,
    /// Görevler
    pub tasks: Vec<Task>,
    /// Yürütme stratejisi
    pub strategy: PlanStrategy,
    /// Oluşturulma zamanı
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Tamamlandı mı?
    #[serde(default)]
    pub completed: bool,
}

impl ExecutionPlan {
    /// Sonraki çalıştırılabilir görevi al
    pub fn next_task(&self) -> Option<&Task> {
        self.tasks.iter()
            .filter(|t| t.status == TaskStatus::Pending)
            .find(|t| {
                // Tüm bağımlılıklar tamamlandı mı?
                t.dependencies.iter().all(|dep_id| {
                    self.tasks.iter()
                        .find(|t| t.id == *dep_id)
                        .map(|t| t.status == TaskStatus::Completed)
                        .unwrap_or(false)
                })
            })
    }
    
    /// İlerleme yüzdesi
    pub fn progress(&self) -> f32 {
        let total = self.tasks.len();
        if total == 0 { return 100.0; }
        
        let completed = self.tasks.iter()
            .filter(|t| t.status == TaskStatus::Completed)
            .count();
        
        (completed as f32 / total as f32) * 100.0
    }
    
    /// Durum özeti
    pub fn status_summary(&self) -> PlanStatusSummary {
        let mut summary = PlanStatusSummary::default();
        
        for task in &self.tasks {
            match task.status {
                TaskStatus::Pending => summary.pending += 1,
                TaskStatus::Running => summary.running += 1,
                TaskStatus::Completed => summary.completed += 1,
                TaskStatus::Failed => summary.failed += 1,
                TaskStatus::Skipped => summary.skipped += 1,
            }
        }
        
        summary.progress_pct = self.progress();
        summary
    }
}

#[derive(Debug, Clone, Default)]
pub struct PlanStatusSummary {
    pub pending: usize,
    pub running: usize,
    pub completed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub progress_pct: f32,
}

/// ─── PLAN STRATEGY ───

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlanStrategy {
    /// Görevler sırayla
    Sequential,
    /// Bağımsız görevler paralel
    Parallel,
    /// Duruma göre adapte olur
    Adaptive,
}

/// ─── PLAN TEMPLATE ───

#[derive(Debug, Clone)]
pub struct PlanTemplate {
    pub name: String,
    pub description: String,
    pub task_templates: Vec<TaskTemplate>,
}

#[derive(Debug, Clone)]
pub struct TaskTemplate {
    pub description: String,
    pub tool: ToolType,
    pub order: usize,
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_planner_creation() {
        let planner = Planner::new();
        assert!(!planner.templates.is_empty());
    }
    
    #[test]
    fn test_goal_classification() {
        let planner = Planner::new();
        
        assert_eq!(planner.classify_goal("Yapay zeka nedir araştır"), "research");
        assert_eq!(planner.classify_goal("Python kod yaz"), "code");
        assert_eq!(planner.classify_goal("Web sitesine git"), "web");
    }
    
    #[test]
    fn test_plan_creation() {
        let planner = Planner::new();
        let goal = Goal::research("Test konusu");
        let plan = planner.plan(&goal);
        
        assert!(!plan.tasks.is_empty());
        assert!(!plan.completed);
    }
    
    #[test]
    fn test_plan_progress() {
        let mut plan = ExecutionPlan {
            id: Uuid::new_v4(),
            goal_id: Uuid::new_v4(),
            tasks: vec![
                Task::new("Task 1", ToolType::LlmQuery),
                Task::new("Task 2", ToolType::LlmQuery),
            ],
            strategy: PlanStrategy::Sequential,
            created_at: chrono::Utc::now(),
            completed: false,
        };
        
        assert_eq!(plan.progress(), 0.0);
        
        plan.tasks[0].status = TaskStatus::Completed;
        assert_eq!(plan.progress(), 50.0);
        
        plan.tasks[1].status = TaskStatus::Completed;
        assert_eq!(plan.progress(), 100.0);
    }
    
    #[test]
    fn test_next_task() {
        let mut plan = ExecutionPlan {
            id: Uuid::new_v4(),
            goal_id: Uuid::new_v4(),
            tasks: vec![
                Task::new("Task 1", ToolType::LlmQuery),
                Task::new("Task 2", ToolType::LlmQuery),
            ],
            strategy: PlanStrategy::Sequential,
            created_at: chrono::Utc::now(),
            completed: false,
        };
        
        // Task 2, Task 1'e bağımlı
        let task1_id = plan.tasks[0].id;
        plan.tasks[1].dependencies.push(task1_id);
        
        // İlk olarak Task 1 dönmeli
        let next = plan.next_task();
        assert!(next.is_some());
        assert_eq!(next.unwrap().description, "Task 1");
        
        // Task 1'i tamamla
        plan.tasks[0].status = TaskStatus::Completed;
        
        // Şimdi Task 2 dönmeli
        let next = plan.next_task();
        assert!(next.is_some());
        assert_eq!(next.unwrap().description, "Task 2");
    }
    
    #[test]
    fn test_status_summary() {
        let mut plan = ExecutionPlan {
            id: Uuid::new_v4(),
            goal_id: Uuid::new_v4(),
            tasks: vec![
                Task::new("Task 1", ToolType::LlmQuery),
                Task::new("Task 2", ToolType::LlmQuery),
                Task::new("Task 3", ToolType::LlmQuery),
            ],
            strategy: PlanStrategy::Sequential,
            created_at: chrono::Utc::now(),
            completed: false,
        };
        
        plan.tasks[0].status = TaskStatus::Completed;
        plan.tasks[1].status = TaskStatus::Running;
        plan.tasks[2].status = TaskStatus::Pending;
        
        let summary = plan.status_summary();
        assert_eq!(summary.completed, 1);
        assert_eq!(summary.running, 1);
        assert_eq!(summary.pending, 1);
    }
}
