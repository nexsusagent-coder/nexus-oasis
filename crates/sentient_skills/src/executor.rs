//! Skill Executor - Skill execution engine
//!
//! Skill'leri yükler, eşleştirir ve çalıştırır

use crate::{Skill, SkillManager, SkillResult, SkillCategory};
use crate::guardrails::GuardrailMiddleware;
use crate::subagent::{SubagentExecutor, SubagentConfig, SubagentTask, SubagentResult};
use std::sync::Arc;
use std::collections::HashMap;
use tracing::{info, debug};

/// Skill Execution Context
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// User input
    pub input: String,
    
    /// Session ID
    pub session_id: Option<String>,
    
    /// User ID
    pub user_id: Option<String>,
    
    /// Additional context
    pub context: HashMap<String, serde_json::Value>,
}

impl ExecutionContext {
    /// Yeni context oluştur
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            session_id: None,
            user_id: None,
            context: HashMap::new(),
        }
    }
    
    /// Session ekle
    pub fn with_session(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }
    
    /// User ekle
    pub fn with_user(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }
    
    /// Context ekle
    pub fn with_context(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.context.insert(key.into(), value);
        self
    }
}

/// Skill Execution Result
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    /// Eşleşen skill
    pub skill: Option<Skill>,
    
    /// Sonuç
    pub output: String,
    
    /// Subagent sonuçları
    pub subagent_results: Vec<SubagentResult>,
    
    /// Başarılı mı
    pub success: bool,
    
    /// Hata mesajı
    pub error: Option<String>,
    
    /// Execution time (ms)
    pub execution_time_ms: u64,
}

impl ExecutionResult {
    /// Başarılı sonuç
    pub fn success(skill: Skill, output: String) -> Self {
        Self {
            skill: Some(skill),
            output,
            subagent_results: Vec::new(),
            success: true,
            error: None,
            execution_time_ms: 0,
        }
    }
    
    /// Skill bulunamadı
    pub fn no_skill(input: &str) -> Self {
        Self {
            skill: None,
            output: format!("No matching skill found for: {}", input),
            subagent_results: Vec::new(),
            success: false,
            error: Some("No matching skill".to_string()),
            execution_time_ms: 0,
        }
    }
    
    /// Hata
    pub fn error(error: impl Into<String>) -> Self {
        Self {
            skill: None,
            output: String::new(),
            subagent_results: Vec::new(),
            success: false,
            error: Some(error.into()),
            execution_time_ms: 0,
        }
    }
}

/// Skill Executor
pub struct SkillExecutor {
    /// Skill manager
    manager: Arc<SkillManager>,
    
    /// Guardrail middleware
    guardrail: GuardrailMiddleware,
    
    /// Subagent executor
    subagent_executor: SubagentExecutor,
    
    /// Auto-load skills
    auto_load: bool,
}

impl SkillExecutor {
    /// Yeni executor oluştur
    pub fn new() -> Self {
        Self {
            manager: Arc::new(SkillManager::new()),
            guardrail: GuardrailMiddleware::with_rules(),
            subagent_executor: SubagentExecutor::new(),
            auto_load: true,
        }
    }
    
    /// Manager ile oluştur
    pub fn with_manager(manager: Arc<SkillManager>) -> Self {
        Self {
            manager,
            guardrail: GuardrailMiddleware::with_rules(),
            subagent_executor: SubagentExecutor::new(),
            auto_load: false,
        }
    }
    
    /// Skill'leri yükle
    pub fn load_skills(&mut self) -> SkillResult<usize> {
        let mut manager = SkillManager::new();
        manager.load_skills()?;
        
        self.manager = Arc::new(manager);
        Ok(self.manager.skill_count())
    }
    
    /// Input'u işle ve skill çalıştır
    pub async fn execute(&self, ctx: ExecutionContext) -> ExecutionResult {
        let start = std::time::Instant::now();
        
        info!("Executing skill for input: {}", ctx.input);
        
        // Eşleşen skill'i bul
        let skill = match self.manager.find_best_match(&ctx.input) {
            Some(s) => s,
            None => {
                debug!("No matching skill found");
                return ExecutionResult::no_skill(&ctx.input);
            }
        };
        
        info!("Matched skill: {}", skill.metadata.name);
        
        // Skill içeriğini hazırla
        let _prompt = self.build_prompt(&skill, &ctx);
        
        // TODO: Gerçek LLM integration
        // Şimdilik simüle edilmiş execution
        let output = format!(
            "Executed skill '{}' with input: {}\n\nSkill content preview:\n{}",
            skill.metadata.name,
            ctx.input,
            skill.summary()
        );
        
        let mut result = ExecutionResult::success(skill, output);
        result.execution_time_ms = start.elapsed().as_millis() as u64;
        
        result
    }
    
    /// Skill ve subagent ile çalıştır
    pub async fn execute_with_subagents(
        &self,
        skill: Skill,
        ctx: ExecutionContext,
        subagent_configs: Vec<SubagentConfig>,
    ) -> ExecutionResult {
        let start = std::time::Instant::now();
        
        // Subagent task'lerini oluştur
        let tasks: Vec<SubagentTask> = subagent_configs
            .into_iter()
            .map(|config| {
                SubagentTask::new(config, ctx.input.clone())
                    .with_context("skill_name", serde_json::json!(skill.metadata.name))
            })
            .collect();
        
        // Paralel çalıştır
        let subagent_results = self.subagent_executor.execute_parallel(tasks).await;
        
        // Sonuçları birleştir
        let output = format!(
            "Skill '{}' executed with {} subagents\n\nResults:\n{}",
            skill.metadata.name,
            subagent_results.len(),
            subagent_results
                .iter()
                .filter_map(|r| r.result.as_ref())
                .map(|s| format!("- {}", s))
                .collect::<Vec<_>>()
                .join("\n")
        );
        
        let mut result = ExecutionResult::success(skill, output);
        result.subagent_results = subagent_results;
        result.execution_time_ms = start.elapsed().as_millis() as u64;
        
        result
    }
    
    /// Prompt oluştur
    fn build_prompt(&self, skill: &Skill, ctx: &ExecutionContext) -> String {
        format!(
            "# Skill: {}\n\n{}\n\n## User Input\n{}\n\n## Instructions\nFollow the skill guidelines above to complete the task.",
            skill.metadata.name,
            skill.content,
            ctx.input
        )
    }
    
    /// Skill'i ada göre çalıştır
    pub async fn execute_by_name(&self, name: &str, ctx: ExecutionContext) -> ExecutionResult {
        match self.manager.get_skill(name) {
            Some(_skill) => self.execute(ctx).await,
            None => ExecutionResult::error(format!("Skill not found: {}", name)),
        }
    }
    
    /// Mevcut skill'leri listele
    pub fn list_skills(&self) -> Vec<String> {
        self.manager.list_skills()
    }
    
    /// Kategori bazlı skill'leri listele
    pub fn list_skills_by_category(&self, category: SkillCategory) -> Vec<Skill> {
        self.manager.get_skills_by_category(category)
    }
    
    /// Skill sayısı
    pub fn skill_count(&self) -> usize {
        self.manager.skill_count()
    }
    
    /// Kategori istatistikleri
    pub fn category_stats(&self) -> HashMap<SkillCategory, usize> {
        self.manager.category_stats()
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
    async fn test_skill_executor() {
        let executor = SkillExecutor::new();
        
        let ctx = ExecutionContext::new("research AI trends");
        let result = executor.execute(ctx).await;
        
        // No skills loaded, should return no_skill
        assert!(!result.success);
    }
    
    #[test]
    fn test_execution_context() {
        let ctx = ExecutionContext::new("test input")
            .with_session("session-123")
            .with_user("user-456")
            .with_context("key", serde_json::json!("value"));
        
        assert_eq!(ctx.input, "test input");
        assert_eq!(ctx.session_id, Some("session-123".to_string()));
        assert_eq!(ctx.user_id, Some("user-456".to_string()));
    }
}
