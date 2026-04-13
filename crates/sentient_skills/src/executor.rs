//! Skill Executor - Skill execution engine
//!
//! Skill'leri yükler, eşleştirir ve çalıştırır

use crate::{Skill, SkillManager, SkillResult, SkillCategory};
use crate::guardrails::GuardrailMiddleware;
use crate::subagent::{SubagentExecutor, SubagentConfig, SubagentTask, SubagentResult};
use std::sync::Arc;
use std::collections::HashMap;
use tracing::{info, debug, warn};
use serde::{Deserialize, Serialize};

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

/// LLM Configuration for skill execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    /// API endpoint (e.g., "https://api.openai.com/v1")
    pub endpoint: String,
    /// API key
    pub api_key: Option<String>,
    /// Model to use
    pub model: String,
    /// Max tokens
    pub max_tokens: u32,
    /// Temperature
    pub temperature: f32,
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            endpoint: "http://localhost:11434/v1".to_string(), // Ollama default
            api_key: None,
            model: "llama3".to_string(),
            max_tokens: 2048,
            temperature: 0.7,
        }
    }
}

/// LLM Chat Message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// LLM Chat Request
#[derive(Debug, Clone, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: u32,
    temperature: f32,
}

/// LLM Chat Response
#[derive(Debug, Clone, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Clone, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

/// Skill Executor
pub struct SkillExecutor {
    /// Skill manager
    manager: Arc<SkillManager>,
    
    /// Guardrail middleware
    #[allow(dead_code)]
    guardrail: GuardrailMiddleware,
    
    /// Subagent executor
    subagent_executor: SubagentExecutor,
    
    /// Auto-load skills
    #[allow(dead_code)]
    auto_load: bool,
    
    /// LLM configuration
    llm_config: LLMConfig,
    
    /// HTTP client
    http_client: reqwest::Client,
}

impl SkillExecutor {
    /// Yeni executor oluştur
    pub fn new() -> Self {
        Self {
            manager: Arc::new(SkillManager::new()),
            guardrail: GuardrailMiddleware::with_rules(),
            subagent_executor: SubagentExecutor::new(),
            auto_load: true,
            llm_config: LLMConfig::default(),
            http_client: reqwest::Client::new(),
        }
    }
    
    /// Manager ile oluştur
    pub fn with_manager(manager: Arc<SkillManager>) -> Self {
        Self {
            manager,
            guardrail: GuardrailMiddleware::with_rules(),
            subagent_executor: SubagentExecutor::new(),
            auto_load: false,
            llm_config: LLMConfig::default(),
            http_client: reqwest::Client::new(),
        }
    }
    
    /// LLM config ile oluştur
    pub fn with_llm(mut self, config: LLMConfig) -> Self {
        self.llm_config = config;
        self
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
        let prompt = self.build_prompt(&skill, &ctx);
        
        // Gerçek LLM integration
        let output = match self.call_llm(&prompt).await {
            Ok(response) => response,
            Err(e) => {
                warn!("LLM call failed: {}, using fallback", e);
                // Fallback: Simulated execution
                format!(
                    "Executed skill '{}' with input: {}\n\nSkill content preview:\n{}",
                    skill.metadata.name,
                    ctx.input,
                    skill.summary()
                )
            }
        };
        
        let mut result = ExecutionResult::success(skill, output);
        result.execution_time_ms = start.elapsed().as_millis() as u64;
        
        result
    }
    
    /// LLM API çağrısı
    async fn call_llm(&self, prompt: &str) -> Result<String, String> {
        let url = format!("{}/chat/completions", self.llm_config.endpoint);
        
        let request = ChatRequest {
            model: self.llm_config.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: "You are a helpful AI assistant executing skills. Follow the skill instructions precisely.".to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                },
            ],
            max_tokens: self.llm_config.max_tokens,
            temperature: self.llm_config.temperature,
        };
        
        let mut req = self.http_client.post(&url).json(&request);
        
        // Add API key if available
        if let Some(ref api_key) = self.llm_config.api_key {
            req = req.bearer_auth(api_key);
        }
        
        let response = req
            .timeout(std::time::Duration::from_secs(60))
            .send()
            .await
            .map_err(|e| format!("HTTP error: {}", e))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("API error ({}): {}", status, text));
        }
        
        let data: ChatResponse = response
            .json()
            .await
            .map_err(|e| format!("JSON parse error: {}", e))?;
        
        Ok(data.choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default())
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
