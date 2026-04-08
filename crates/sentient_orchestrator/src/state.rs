//! ─── AJAN DURUMU VE BAĞLAM ───
//!
//! Agent'ın çalışma zamanı durumu ve bağlam yönetimi.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

/// ─── AGENT STATE ───
/// 
/// Ajanın anlık durumu - döngünün hangi aşamasında olduğunu takip eder.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentState {
    /// Boşta, yeni görev bekliyor
    Idle,
    /// Hedefi analiz ediyor
    Analyzing,
    /// Plan yapıyor
    Planning,
    /// Araç kullanıyor
    Acting,
    /// Sonucu değerlendiriyor
    Evaluating,
    /// Başarıyla tamamlandı
    Completed,
    /// Hata durumunda
    Error,
    /// Zaman aşımı
    Timeout,
    /// İptal edildi
    Cancelled,
}

impl AgentState {
    /// Durum göstergesi (emoji)
    pub fn indicator(&self) -> &'static str {
        match self {
            Self::Idle => "💤",
            Self::Analyzing => "🤔",
            Self::Planning => "📋",
            Self::Acting => "🎬",
            Self::Evaluating => "🔍",
            Self::Completed => "✅",
            Self::Error => "❌",
            Self::Timeout => "⏰",
            Self::Cancelled => "🛑",
        }
    }
    
    /// Durum adı
    pub fn name(&self) -> &'static str {
        match self {
            Self::Idle => "Boşta",
            Self::Analyzing => "Analiz Ediliyor",
            Self::Planning => "Planlanıyor",
            Self::Acting => "Eylem Yapılıyor",
            Self::Evaluating => "Değerlendiriliyor",
            Self::Completed => "Tamamlandı",
            Self::Error => "Hata",
            Self::Timeout => "Zaman Aşımı",
            Self::Cancelled => "İptal",
        }
    }
    
    /// Aktif mi?
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Analyzing | Self::Planning | Self::Acting | Self::Evaluating)
    }
    
    /// Terminal mi? (döngüden çıkmalı)
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Error | Self::Timeout | Self::Cancelled)
    }
}

/// ─── AGENT CONTEXT ───
/// 
/// Ajanın çalışma zamanı bağlamı - tüm geçici veriler burada.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentContext {
    /// Aktif hedef
    pub current_goal: Option<crate::goal::Goal>,
    /// Geçerli plan
    pub current_plan: Option<crate::planner::ExecutionPlan>,
    /// Tamamlanan görevler
    pub completed_tasks: Vec<Uuid>,
    /// Bekleyen görevler
    pub pending_tasks: Vec<Uuid>,
    /// Başarısız görevler
    pub failed_tasks: Vec<Uuid>,
    /// Kullanıcı değişkenleri
    pub variables: HashMap<String, serde_json::Value>,
    /// Sohbet geçmişi (LLM için)
    pub conversation_history: Vec<ConversationMessage>,
    /// Son araç çağrısı
    pub last_tool_call: Option<ToolCall>,
    /// Son araç sonucu
    pub last_tool_result: Option<ToolResult>,
    /// İterasyon sayısı
    pub iteration: u32,
    /// Toplam token kullanımı
    pub total_tokens: u64,
    /// Başlangıç zamanı
    pub started_at: chrono::DateTime<chrono::Utc>,
}

impl Default for AgentContext {
    fn default() -> Self {
        Self {
            current_goal: None,
            current_plan: None,
            completed_tasks: Vec::new(),
            pending_tasks: Vec::new(),
            failed_tasks: Vec::new(),
            variables: HashMap::new(),
            conversation_history: Vec::new(),
            last_tool_call: None,
            last_tool_result: None,
            iteration: 0,
            total_tokens: 0,
            started_at: chrono::Utc::now(),
        }
    }
}

impl AgentContext {
    /// Yeni bağlam oluştur
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Hedef ayarla
    pub fn set_goal(&mut self, goal: crate::goal::Goal) {
        self.current_goal = Some(goal);
        self.started_at = chrono::Utc::now();
    }
    
    /// Plan ayarla
    pub fn set_plan(&mut self, plan: crate::planner::ExecutionPlan) {
        self.pending_tasks = plan.tasks.iter().map(|t| t.id).collect();
        self.current_plan = Some(plan);
    }
    
    /// Görev tamamlandı işaretle
    pub fn complete_task(&mut self, task_id: Uuid) {
        self.pending_tasks.retain(|&id| id != task_id);
        self.completed_tasks.push(task_id);
    }
    
    /// Görev başarısız işaretle
    pub fn fail_task(&mut self, task_id: Uuid) {
        self.pending_tasks.retain(|&id| id != task_id);
        self.failed_tasks.push(task_id);
    }
    
    /// Sohbet mesajı ekle
    pub fn add_message(&mut self, role: MessageRole, content: String) {
        self.conversation_history.push(ConversationMessage {
            role,
            content,
            timestamp: chrono::Utc::now(),
        });
    }
    
    /// Kullanıcı mesajı ekle
    pub fn user_message(&mut self, content: String) {
        self.add_message(MessageRole::User, content);
    }
    
    /// Asistan mesajı ekle
    pub fn assistant_message(&mut self, content: String) {
        self.add_message(MessageRole::Assistant, content);
    }
    
    /// Sistem mesajı ekle
    pub fn system_message(&mut self, content: String) {
        self.add_message(MessageRole::System, content);
    }
    
    /// Değişken ayarla
    pub fn set_var(&mut self, key: String, value: serde_json::Value) {
        self.variables.insert(key, value);
    }
    
    /// Değişken al
    pub fn get_var(&self, key: &str) -> Option<&serde_json::Value> {
        self.variables.get(key)
    }
    
    /// İterasyon artır
    pub fn increment_iteration(&mut self) {
        self.iteration += 1;
    }
    
    /// Token ekle
    pub fn add_tokens(&mut self, tokens: u64) {
        self.total_tokens += tokens;
    }
    
    /// Geçen süre
    pub fn elapsed(&self) -> chrono::Duration {
        chrono::Utc::now().signed_duration_since(self.started_at)
    }
    
    /// Özet rapor
    pub fn summary(&self) -> ContextSummary {
        ContextSummary {
            total_iterations: self.iteration,
            completed_tasks: self.completed_tasks.len(),
            failed_tasks: self.failed_tasks.len(),
            pending_tasks: self.pending_tasks.len(),
            total_tokens: self.total_tokens,
            elapsed_secs: self.elapsed().num_seconds() as u64,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSummary {
    pub total_iterations: u32,
    pub completed_tasks: usize,
    pub failed_tasks: usize,
    pub pending_tasks: usize,
    pub total_tokens: u64,
    pub elapsed_secs: u64,
}

/// ─── CONVERSATION MESSAGE ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

impl MessageRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::System => "system",
            Self::User => "user",
            Self::Assistant => "assistant",
            Self::Tool => "tool",
        }
    }
}

/// ─── TOOL CALL ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ToolCall {
    pub fn new(name: impl Into<String>, arguments: serde_json::Value) -> Self {
        Self {
            id: format!("call_{}", uuid::Uuid::new_v4()),
            name: name.into(),
            arguments,
            timestamp: chrono::Utc::now(),
        }
    }
}

/// ─── TOOL RESULT ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub call_id: String,
    pub success: bool,
    pub output: serde_json::Value,
    pub error: Option<String>,
    pub duration_ms: u64,
}

impl ToolResult {
    pub fn success(call_id: String, output: serde_json::Value, duration_ms: u64) -> Self {
        Self {
            call_id,
            success: true,
            output,
            error: None,
            duration_ms,
        }
    }
    
    pub fn error(call_id: String, error: String, duration_ms: u64) -> Self {
        Self {
            call_id,
            success: false,
            output: serde_json::Value::Null,
            error: Some(error),
            duration_ms,
        }
    }
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_agent_state_indicators() {
        assert!(!AgentState::Idle.indicator().is_empty());
        assert!(!AgentState::Acting.indicator().is_empty());
    }
    
    #[test]
    fn test_agent_state_active() {
        assert!(!AgentState::Idle.is_active());
        assert!(AgentState::Acting.is_active());
        assert!(AgentState::Planning.is_active());
    }
    
    #[test]
    fn test_agent_state_terminal() {
        assert!(AgentState::Completed.is_terminal());
        assert!(AgentState::Error.is_terminal());
        assert!(!AgentState::Acting.is_terminal());
    }
    
    #[test]
    fn test_context_creation() {
        let ctx = AgentContext::new();
        assert_eq!(ctx.iteration, 0);
        assert!(ctx.conversation_history.is_empty());
    }
    
    #[test]
    fn test_context_messages() {
        let mut ctx = AgentContext::new();
        ctx.user_message("Merhaba".into());
        ctx.assistant_message("Selam!".into());
        
        assert_eq!(ctx.conversation_history.len(), 2);
        assert_eq!(ctx.conversation_history[0].role, MessageRole::User);
    }
    
    #[test]
    fn test_context_variables() {
        let mut ctx = AgentContext::new();
        ctx.set_var("test".into(), serde_json::json!(42));
        
        assert_eq!(ctx.get_var("test"), Some(&serde_json::json!(42)));
        assert_eq!(ctx.get_var("missing"), None);
    }
    
    #[test]
    fn test_tool_call_creation() {
        let call = ToolCall::new("browser_navigate", serde_json::json!({"url": "https://example.com"}));
        assert!(call.id.starts_with("call_"));
        assert_eq!(call.name, "browser_navigate");
    }
    
    #[test]
    fn test_tool_result_success() {
        let result = ToolResult::success("call_123".into(), serde_json::json!("OK"), 100);
        assert!(result.success);
        assert!(result.error.is_none());
    }
    
    #[test]
    fn test_tool_result_error() {
        let result = ToolResult::error("call_123".into(), "Failed".into(), 50);
        assert!(!result.success);
        assert!(result.error.is_some());
    }
}
