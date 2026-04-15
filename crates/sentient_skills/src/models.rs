//! ─── Skill Models ───

use serde::{Deserialize, Serialize};

/// A generated skill
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: SkillCategory,
    pub actions: Vec<SkillAction>,
    pub trigger: Option<SkillTrigger>,
    pub parameters: Vec<SkillParameter>,
    pub created: chrono::DateTime<chrono::Utc>,
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
    pub use_count: u32,
    pub success_rate: f64,
    pub auto_generated: bool,
}

impl Skill {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            description: description.to_string(),
            category: SkillCategory::General,
            actions: vec![],
            trigger: None,
            parameters: vec![],
            created: chrono::Utc::now(),
            last_used: None,
            use_count: 0,
            success_rate: 1.0,
            auto_generated: false,
        }
    }
    
    pub fn with_category(mut self, category: SkillCategory) -> Self {
        self.category = category;
        self
    }
    
    pub fn with_action(mut self, action: SkillAction) -> Self {
        self.actions.push(action);
        self
    }
    
    pub fn with_trigger(mut self, trigger: SkillTrigger) -> Self {
        self.trigger = Some(trigger);
        self
    }
    
    pub fn mark_used(&mut self) {
        self.last_used = Some(chrono::Utc::now());
        self.use_count += 1;
    }
    
    pub fn record_success(&mut self, success: bool) {
        let total = self.use_count as f64;
        let current_rate = self.success_rate * (total - 1.0) / total;
        self.success_rate = if success {
            current_rate + 1.0 / total
        } else {
            current_rate
        };
    }
}

/// Skill category
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkillCategory {
    General,
    Productivity,
    Communication,
    Automation,
    DataProcessing,
    WebScraping,
    FileManagement,
    SystemControl,
    Custom,
}

impl std::fmt::Display for SkillCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::General => write!(f, "📁 General"),
            Self::Productivity => write!(f, "⚡ Productivity"),
            Self::Communication => write!(f, "💬 Communication"),
            Self::Automation => write!(f, "🤖 Automation"),
            Self::DataProcessing => write!(f, "📊 Data Processing"),
            Self::WebScraping => write!(f, "🌐 Web Scraping"),
            Self::FileManagement => write!(f, "📄 File Management"),
            Self::SystemControl => write!(f, "🖥️ System Control"),
            Self::Custom => write!(f, "🔧 Custom"),
        }
    }
}

/// Skill action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillAction {
    pub id: String,
    pub action_type: ActionType,
    pub description: String,
    pub parameters: serde_json::Value,
    pub order: u32,
    pub condition: Option<String>,
    pub on_failure: Option<FailureAction>,
}

impl SkillAction {
    pub fn new(action_type: ActionType, description: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            action_type,
            description: description.to_string(),
            parameters: serde_json::json!({}),
            order: 0,
            condition: None,
            on_failure: None,
        }
    }
    
    pub fn with_params(mut self, params: serde_json::Value) -> Self {
        self.parameters = params;
        self
    }
    
    pub fn with_order(mut self, order: u32) -> Self {
        self.order = order;
        self
    }
}

/// Action type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionType {
    KeyboardShortcut,
    MouseClick,
    MouseMove,
    TextInput,
    OpenApp,
    OpenUrl,
    RunCommand,
    WaitForElement,
    WaitForTime,
    ReadFile,
    WriteFile,
    HttpGet,
    HttpPost,
    ParseData,
    Condition,
    Loop,
    VoiceCommand,
    ApiCall,
    Custom,
}

/// Skill trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTrigger {
    pub trigger_type: TriggerType,
    pub condition: String,
    pub enabled: bool,
}

impl SkillTrigger {
    pub fn new(trigger_type: TriggerType, condition: &str) -> Self {
        Self {
            trigger_type,
            condition: condition.to_string(),
            enabled: true,
        }
    }
}

/// Trigger type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriggerType {
    Voice,
    Keyboard,
    Time,
    Event,
    State,
    Manual,
}

/// Skill parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillParameter {
    pub name: String,
    pub param_type: ParameterType,
    pub required: bool,
    pub default_value: Option<String>,
    pub description: String,
}

/// Parameter type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParameterType {
    String,
    Number,
    Boolean,
    FilePath,
    Url,
    Enum(Vec<String>),
}

/// Failure action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FailureAction {
    Retry { max_attempts: u32, delay_ms: u64 },
    Skip,
    Abort,
    ExecuteSkill { skill_id: String },
    Notify { message: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_skill_creation() {
        let skill = Skill::new("test_skill", "A test skill")
            .with_category(SkillCategory::Productivity);
        
        assert_eq!(skill.name, "test_skill");
        assert_eq!(skill.category, SkillCategory::Productivity);
    }
    
    #[test]
    fn test_skill_usage_tracking() {
        let mut skill = Skill::new("test", "test");
        skill.mark_used();
        skill.record_success(true);
        
        assert_eq!(skill.use_count, 1);
        assert!(skill.last_used.is_some());
    }
}
