//! ─── Pre-built Workflow Templates ───

use crate::{Workflow, Node, NodeType, Trigger, TriggerType, Connection};
use crate::models::*;

/// Template library
pub struct TemplateLibrary {
    templates: Vec<WorkflowTemplate>,
}

impl TemplateLibrary {
    pub fn new() -> Self {
        Self {
            templates: Self::load_defaults(),
        }
    }
    
    fn load_defaults() -> Vec<WorkflowTemplate> {
        vec![
            // Morning routine
            WorkflowTemplate {
                id: "morning_routine".into(),
                name: "Güne Hazırlan".into(),
                description: "Sabah rutini: Hava durumu, takvim, email kontrolü".into(),
                category: "daily".into(),
            },
            // Focus mode
            WorkflowTemplate {
                id: "focus_mode".into(),
                name: "Odaklanma Modu".into(),
                description: "Dikkat dağıtıcıları kapat, timer başlat".into(),
                category: "productivity".into(),
            },
            // Meeting prep
            WorkflowTemplate {
                id: "meeting_prep".into(),
                name: "Toplantı Hazırlığı".into(),
                description: "Toplantı öncesi hazırlık ve hatırlatma".into(),
                category: "calendar".into(),
            },
            // End of day
            WorkflowTemplate {
                id: "end_of_day".into(),
                name: "Günü Kapat".into(),
                description: "Gün sonu raporu ve hazırlık".into(),
                category: "daily".into(),
            },
            // Project complete
            WorkflowTemplate {
                id: "project_complete".into(),
                name: "Proje Tamamla".into(),
                description: "Proje bitiş kontrolü ve arşivleme".into(),
                category: "project".into(),
            },
        ]
    }
    
    pub fn get_all(&self) -> &[WorkflowTemplate] {
        &self.templates
    }
    
    pub fn get(&self, id: &str) -> Option<&WorkflowTemplate> {
        self.templates.iter().find(|t| t.id == id)
    }
    
    pub fn create(&self, template_id: &str) -> Option<Workflow> {
        match template_id {
            "morning_routine" => Some(self.create_morning_routine()),
            "focus_mode" => Some(self.create_focus_mode()),
            "meeting_prep" => Some(self.create_meeting_prep()),
            "end_of_day" => Some(self.create_end_of_day()),
            "project_complete" => Some(self.create_project_complete()),
            _ => None,
        }
    }
    
    fn create_morning_routine(&self) -> Workflow {
        let mut wf = Workflow::new("Güne Hazırlan")
            .with_description("Sabah rutini: Hava durumu, takvim, email kontrolü");
        
        // Nodes
        let n1 = Node::new("Start", NodeType::Trigger(TriggerType::Schedule { cron: "0 7 * * *".into() }));
        let n2 = Node::new("Hava Durumu", NodeType::Http(HttpConfig {
            url: "https://api.openweathermap.org/data/2.5/weather".into(),
            method: "GET".into(),
            headers: Default::default(),
            body: None,
        }));
        let n3 = Node::new("Takvim Kontrol", NodeType::Custom(CustomConfig {
            action_id: "calendar_check".into(),
            params: serde_json::json!({}),
        }));
        let n4 = Node::new("Email Özet", NodeType::Custom(CustomConfig {
            action_id: "email_summary".into(),
            params: serde_json::json!({}),
        }));
        
        wf.add_node(n1);
        let n2_id = wf.add_node(n2);
        let n3_id = wf.add_node(n3);
        let n4_id = wf.add_node(n4);
        
        // Trigger
        wf.add_trigger(Trigger::new(TriggerType::Schedule { cron: "0 7 * * 1-5".into() }));
        
        wf
    }
    
    fn create_focus_mode(&self) -> Workflow {
        let mut wf = Workflow::new("Odaklanma Modu")
            .with_description("Dikkat dağıtıcıları kapat, Pomodoro timer başlat");
        
        let n1 = Node::new("Trigger", NodeType::Trigger(TriggerType::Voice { phrase: "odaklan".into() }));
        let n2 = Node::new("DND Aç", NodeType::Custom(CustomConfig {
            action_id: "enable_dnd".into(),
            params: serde_json::json!({}),
        }));
        let n3 = Node::new("Timer Başlat", NodeType::Custom(CustomConfig {
            action_id: "start_pomodoro".into(),
            params: serde_json::json!({ "duration": 25 }),
        }));
        
        wf.add_node(n1);
        wf.add_node(n2);
        wf.add_node(n3);
        
        wf.add_trigger(Trigger::new(TriggerType::Voice { phrase: "odaklan".into() }));
        wf.add_trigger(Trigger::new(TriggerType::Manual));
        
        wf
    }
    
    fn create_meeting_prep(&self) -> Workflow {
        let mut wf = Workflow::new("Toplantı Hazırlığı")
            .with_description("Toplantı öncesi hazırlık ve hatırlatma");
        
        let n1 = Node::new("Trigger", NodeType::Trigger(TriggerType::Event { event_type: "meeting_soon".into() }));
        let n2 = Node::new("Toplantı Bilgileri", NodeType::Custom(CustomConfig {
            action_id: "get_meeting_info".into(),
            params: serde_json::json!({}),
        }));
        let n3 = Node::new("AI Hazırlık", NodeType::Llm(LlmConfig {
            prompt: "Bu toplantı için hazırlık önerileri üret".into(),
            model: "gpt-4".into(),
            max_tokens: Some(500),
        }));
        
        wf.add_node(n1);
        wf.add_node(n2);
        wf.add_node(n3);
        
        wf
    }
    
    fn create_end_of_day(&self) -> Workflow {
        let mut wf = Workflow::new("Günü Kapat")
            .with_description("Gün sonu raporu ve yarına hazırlık");
        
        let n1 = Node::new("Trigger", NodeType::Trigger(TriggerType::Schedule { cron: "0 18 * * 1-5".into() }));
        let n2 = Node::new("Günlük Özet", NodeType::Custom(CustomConfig {
            action_id: "daily_summary".into(),
            params: serde_json::json!({}),
        }));
        let n3 = Node::new("Yarın Hazırlık", NodeType::Custom(CustomConfig {
            action_id: "tomorrow_prep".into(),
            params: serde_json::json!({}),
        }));
        
        wf.add_node(n1);
        wf.add_node(n2);
        wf.add_node(n3);
        
        wf
    }
    
    fn create_project_complete(&self) -> Workflow {
        let mut wf = Workflow::new("Proje Tamamla")
            .with_description("Proje bitiş kontrolü ve arşivleme");
        
        let n1 = Node::new("Trigger", NodeType::Trigger(TriggerType::Manual));
        let n2 = Node::new("Proje Kontrol", NodeType::Condition(ConditionConfig {
            expression: "project_status == complete".into(),
            true_output: "archive".into(),
            false_output: "notify".into(),
        }));
        let n3 = Node::new("Arşivle", NodeType::Custom(CustomConfig {
            action_id: "archive_project".into(),
            params: serde_json::json!({}),
        }));
        
        wf.add_node(n1);
        wf.add_node(n2);
        wf.add_node(n3);
        
        wf
    }
}

impl Default for TemplateLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct WorkflowTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_template_library() {
        let lib = TemplateLibrary::new();
        assert!(!lib.get_all().is_empty());
    }
    
    #[test]
    fn test_create_morning_routine() {
        let lib = TemplateLibrary::new();
        let wf = lib.create("morning_routine");
        assert!(wf.is_some());
    }
}
