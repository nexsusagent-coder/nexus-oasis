//! ─── Skill Executor ───

use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::models::*;
use crate::{SkillResult, SkillError};

/// Execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub skill_id: String,
    pub success: bool,
    pub duration_ms: u64,
    pub actions_executed: u32,
    pub actions_failed: u32,
    pub error: Option<String>,
    pub output: Option<String>,
}

impl ExecutionResult {
    pub fn success(skill_id: &str, duration_ms: u64, actions_executed: u32) -> Self {
        Self {
            skill_id: skill_id.to_string(),
            success: true,
            duration_ms,
            actions_executed,
            actions_failed: 0,
            error: None,
            output: None,
        }
    }
    
    pub fn failure(skill_id: &str, error: &str) -> Self {
        Self {
            skill_id: skill_id.to_string(),
            success: false,
            duration_ms: 0,
            actions_executed: 0,
            actions_failed: 1,
            error: Some(error.to_string()),
            output: None,
        }
    }
    
    pub fn with_output(mut self, output: &str) -> Self {
        self.output = Some(output.to_string());
        self
    }
}

/// Skill executor
pub struct SkillExecutor {
    dry_run: bool,
    timeout_ms: u64,
    retry_count: u32,
}

impl SkillExecutor {
    pub fn new() -> Self {
        Self {
            dry_run: false,
            timeout_ms: 30000,
            retry_count: 3,
        }
    }
    
    /// Enable dry run mode (no actual execution)
    pub fn dry_run(mut self, enabled: bool) -> Self {
        self.dry_run = enabled;
        self
    }
    
    /// Set timeout
    pub fn timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = ms;
        self
    }
    
    /// Execute a skill
    pub async fn execute(&self, skill: &Skill) -> SkillResult<ExecutionResult> {
        let start = Instant::now();
        let mut actions_executed = 0u32;
        let mut actions_failed = 0u32;
        let mut last_error = None;
        
        // Sort actions by order
        let mut actions = skill.actions.clone();
        actions.sort_by_key(|a| a.order);
        
        for action in actions {
            match self.execute_action(&action).await {
                Ok(_) => actions_executed += 1,
                Err(e) => {
                    actions_failed += 1;
                    last_error = Some(e.to_string());
                    
                    // Handle failure action
                    if let Some(failure_action) = &action.on_failure {
                        self.handle_failure(failure_action).await;
                    }
                }
            }
        }
        
        let duration = start.elapsed().as_millis() as u64;
        
        if actions_failed > 0 {
            Ok(ExecutionResult {
                skill_id: skill.id.clone(),
                success: false,
                duration_ms: duration,
                actions_executed,
                actions_failed,
                error: last_error,
                output: None,
            })
        } else {
            Ok(ExecutionResult::success(&skill.id, duration, actions_executed))
        }
    }
    
    /// Execute a single action
    async fn execute_action(&self, action: &SkillAction) -> SkillResult<()> {
        if self.dry_run {
            tracing::info!("DRY RUN: Would execute {:?}", action.action_type);
            return Ok(());
        }
        
        match action.action_type {
            ActionType::KeyboardShortcut => {
                let shortcut = action.parameters["shortcut"].as_str()
                    .ok_or_else(|| SkillError::ExecutionFailed("Missing shortcut".into()))?;
                self.send_keyboard_shortcut(shortcut).await
            }
            ActionType::MouseClick => {
                let x = action.parameters["x"].as_i64().unwrap_or(0) as i32;
                let y = action.parameters["y"].as_i64().unwrap_or(0) as i32;
                self.send_mouse_click(x, y).await
            }
            ActionType::TextInput => {
                let text = action.parameters["text"].as_str()
                    .ok_or_else(|| SkillError::ExecutionFailed("Missing text".into()))?;
                self.send_text_input(text).await
            }
            ActionType::OpenApp => {
                let app = action.parameters["app"].as_str()
                    .ok_or_else(|| SkillError::ExecutionFailed("Missing app".into()))?;
                self.open_application(app).await
            }
            ActionType::OpenUrl => {
                let url = action.parameters["url"].as_str()
                    .ok_or_else(|| SkillError::ExecutionFailed("Missing url".into()))?;
                self.open_url(url).await
            }
            ActionType::RunCommand => {
                let cmd = action.parameters["command"].as_str()
                    .ok_or_else(|| SkillError::ExecutionFailed("Missing command".into()))?;
                self.run_command(cmd).await
            }
            ActionType::WaitForTime => {
                let duration_ms = action.parameters["duration_ms"].as_u64().unwrap_or(1000);
                tokio::time::sleep(std::time::Duration::from_millis(duration_ms)).await;
                Ok(())
            }
            _ => {
                tracing::warn!("Unsupported action type: {:?}", action.action_type);
                Ok(())
            }
        }
    }
    
    /// Handle failure action
    async fn handle_failure(&self, failure: &FailureAction) {
        match failure {
            FailureAction::Retry { max_attempts, delay_ms } => {
                tracing::info!("Would retry (max {} attempts, {}ms delay)", max_attempts, delay_ms);
            }
            FailureAction::Skip => {
                tracing::info!("Skipping failed action");
            }
            FailureAction::Abort => {
                tracing::warn!("Aborting due to failure");
            }
            FailureAction::Notify { message } => {
                tracing::warn!("Notification: {}", message);
            }
            _ => {}
        }
    }
    
    // Action implementations
    
    async fn send_keyboard_shortcut(&self, shortcut: &str) -> SkillResult<()> {
        tracing::info!("Pressing: {}", shortcut);
        // TODO: Integrate with oasis_hands for actual input
        Ok(())
    }
    
    async fn send_mouse_click(&self, x: i32, y: i32) -> SkillResult<()> {
        tracing::info!("Clicking at ({}, {})", x, y);
        // TODO: Integrate with oasis_hands
        Ok(())
    }
    
    async fn send_text_input(&self, text: &str) -> SkillResult<()> {
        tracing::info!("Typing: {}", text);
        // TODO: Integrate with oasis_hands
        Ok(())
    }
    
    async fn open_application(&self, app: &str) -> SkillResult<()> {
        tracing::info!("Opening application: {}", app);
        // TODO: Use system command to open app
        Ok(())
    }
    
    async fn open_url(&self, url: &str) -> SkillResult<()> {
        tracing::info!("Opening URL: {}", url);
        // TODO: Open URL in default browser
        Ok(())
    }
    
    async fn run_command(&self, cmd: &str) -> SkillResult<()> {
        tracing::info!("Running command: {}", cmd);
        // TODO: Execute shell command safely
        Ok(())
    }
}

impl Default for SkillExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_dry_run_execution() {
        let executor = SkillExecutor::new().dry_run(true);
        let skill = Skill::new("test", "Test skill");
        
        let result = executor.execute(&skill).await.unwrap();
        assert!(result.success);
    }
}
