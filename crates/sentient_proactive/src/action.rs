//! ─── Action Execution System ───
//!
//! Executes proactive actions

use serde::{Deserialize, Serialize};

/// Action executor
pub struct ActionExecutor {
    handlers: std::collections::HashMap<String, Box<dyn ActionHandler>>,
}

/// Result of action execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    /// Whether action succeeded
    pub success: bool,
    
    /// Result message
    pub message: String,
    
    /// Execution time in ms
    pub execution_time_ms: u64,
    
    /// Additional data
    pub data: Option<serde_json::Value>,
}

impl ActionResult {
    pub fn success(message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            execution_time_ms: 0,
            data: None,
        }
    }
    
    pub fn failure(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            execution_time_ms: 0,
            data: None,
        }
    }
    
    pub fn with_data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
}

/// Action handler trait
#[async_trait::async_trait]
pub trait ActionHandler: Send + Sync {
    async fn execute(&self, params: &serde_json::Value) -> ActionResult;
    fn name(&self) -> &str;
}

/// Built-in actions
pub enum Action {
    /// Generate morning briefing
    GenerateBriefing,
    
    /// Summarize emails
    SummarizeEmails,
    
    /// Check calendar
    CheckCalendar,
    
    /// Generate weekly report
    WeeklyReport,
    
    /// Send notification
    SendNotification { message: String },
    
    /// Execute shell command (with safety checks)
    ExecuteCommand { command: String },
    
    /// Call LLM with prompt
    CallLLM { prompt: String, model: String },
    
    /// Webhook call
    WebhookCall { url: String, payload: serde_json::Value },
    
    /// Custom action
    Custom { name: String, params: serde_json::Value },
}

impl ActionExecutor {
    pub fn new() -> Self {
        let mut handlers: std::collections::HashMap<String, Box<dyn ActionHandler>> = 
            std::collections::HashMap::new();
        
        // Register built-in handlers
        handlers.insert("generate_briefing".into(), Box::new(BriefingHandler));
        handlers.insert("summarize_emails".into(), Box::new(EmailSummaryHandler));
        handlers.insert("check_calendar".into(), Box::new(CalendarHandler));
        handlers.insert("weekly_report".into(), Box::new(ReportHandler));
        
        Self { handlers }
    }
    
    /// Execute an action by name
    pub async fn execute(&self, action: &str) -> ActionResult {
        let start = std::time::Instant::now();
        
        let result = if let Some(handler) = self.handlers.get(action) {
            handler.execute(&serde_json::json!({})).await
        } else {
            // Try to parse as action spec
            if let Ok(action_spec) = serde_json::from_str::<serde_json::Value>(action) {
                if let Some(name) = action_spec.get("action").and_then(|v| v.as_str()) {
                    if let Some(handler) = self.handlers.get(name) {
                        handler.execute(&action_spec).await
                    } else {
                        ActionResult::failure(&format!("Unknown action: {}", name))
                    }
                } else {
                    ActionResult::failure("Invalid action specification")
                }
            } else {
                ActionResult::failure(&format!("Unknown action: {}", action))
            }
        };
        
        ActionResult {
            execution_time_ms: start.elapsed().as_millis() as u64,
            ..result
        }
    }
    
    /// Register a custom handler
    pub fn register_handler<H: ActionHandler + 'static>(&mut self, handler: H) {
        self.handlers.insert(handler.name().to_string(), Box::new(handler));
    }
}

impl Default for ActionExecutor {
    fn default() -> Self {
        Self::new()
    }
}

// Built-in handlers
struct BriefingHandler;
struct EmailSummaryHandler;
struct CalendarHandler;
struct ReportHandler;

#[async_trait::async_trait]
impl ActionHandler for BriefingHandler {
    async fn execute(&self, _params: &serde_json::Value) -> ActionResult {
        // TODO: Implement actual briefing generation
        ActionResult::success("Morning briefing generated")
            .with_data(serde_json::json!({
                "weather": "Sunny, 22°C",
                "calendar_events": 3,
                "urgent_emails": 2
            }))
    }
    
    fn name(&self) -> &str { "generate_briefing" }
}

#[async_trait::async_trait]
impl ActionHandler for EmailSummaryHandler {
    async fn execute(&self, _params: &serde_json::Value) -> ActionResult {
        ActionResult::success("Emails summarized")
            .with_data(serde_json::json!({
                "total": 15,
                "urgent": 2,
                "summary": "Most emails are newsletters and promotional content"
            }))
    }
    
    fn name(&self) -> &str { "summarize_emails" }
}

#[async_trait::async_trait]
impl ActionHandler for CalendarHandler {
    async fn execute(&self, _params: &serde_json::Value) -> ActionResult {
        ActionResult::success("Calendar checked")
            .with_data(serde_json::json!({
                "next_event": "Team standup in 30 minutes",
                "events_today": 5
            }))
    }
    
    fn name(&self) -> &str { "check_calendar" }
}

#[async_trait::async_trait]
impl ActionHandler for ReportHandler {
    async fn execute(&self, _params: &serde_json::Value) -> ActionResult {
        ActionResult::success("Weekly report generated")
            .with_data(serde_json::json!({
                "tasks_completed": 12,
                "emails_sent": 8,
                "meetings_attended": 5
            }))
    }
    
    fn name(&self) -> &str { "weekly_report" }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_briefing_action() {
        let executor = ActionExecutor::new();
        let result = executor.execute("generate_briefing").await;
        
        assert!(result.success);
        assert!(result.data.is_some());
    }
    
    #[tokio::test]
    async fn test_unknown_action() {
        let executor = ActionExecutor::new();
        let result = executor.execute("unknown_action").await;
        
        assert!(!result.success);
    }
}
