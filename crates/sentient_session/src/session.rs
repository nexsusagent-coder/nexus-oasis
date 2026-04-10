//! ─── SESSION ───
//!
//! Oturum yapısı ve yönetimi

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::SessionError;

// ═══════════════════════════════════════════════════════════════════════════════
// SESSION - Ana yapı
// ═══════════════════════════════════════════════════════════════════════════════

/// Oturum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Benzersiz ID
    pub id: Uuid,
    /// Üst oturum ID
    pub parent_id: Option<Uuid>,
    /// Yapılandırma
    pub config: SessionConfig,
    /// Durum
    pub state: SessionState,
    /// Tip
    pub session_type: SessionType,
    /// Mesaj geçmişi
    pub messages: Vec<Message>,
    /// Bağlam
    pub context: SessionContext,
    /// Sıkıştırılmış bağlam
    pub compacted_context: Option<String>,
    /// Token sayısı
    pub token_count: u64,
    /// Başlangıç zamanı
    pub started_at: Option<DateTime<Utc>>,
    /// Bitiş zamanı
    pub ended_at: Option<DateTime<Utc>>,
    /// Metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Session {
    /// Yeni oturum oluştur
    pub fn new(config: SessionConfig) -> Self {
        Self {
            id: Uuid::new_v4(),
            parent_id: None,
            config,
            state: SessionState::Pending,
            session_type: SessionType::Default,
            messages: Vec::new(),
            context: SessionContext::default(),
            compacted_context: None,
            token_count: 0,
            started_at: None,
            ended_at: None,
            metadata: HashMap::new(),
        }
    }
    
    /// Oturumu başlat
    pub fn start(&mut self) -> Result<(), SessionError> {
        if self.state != SessionState::Pending {
            return Err(SessionError::Invalid("Oturum zaten başlatılmış".into()));
        }
        self.state = SessionState::Active;
        self.started_at = Some(Utc::now());
        Ok(())
    }
    
    /// Oturumu sonlandır
    pub fn end(&mut self) -> Result<(), SessionError> {
        if self.state != SessionState::Active && self.state != SessionState::Paused {
            return Err(SessionError::Invalid("Oturum aktif değil".into()));
        }
        self.state = SessionState::Ended;
        self.ended_at = Some(Utc::now());
        Ok(())
    }
    
    /// Oturumu devam ettir
    pub fn resume(&mut self) -> Result<(), SessionError> {
        if self.state != SessionState::Ended {
            return Err(SessionError::Invalid("Oturum sonlandırılmamış".into()));
        }
        self.state = SessionState::Active;
        self.ended_at = None;
        Ok(())
    }
    
    /// Oturumu duraklat
    pub fn pause(&mut self) -> Result<(), SessionError> {
        if self.state != SessionState::Active {
            return Err(SessionError::Invalid("Oturum aktif değil".into()));
        }
        self.state = SessionState::Paused;
        Ok(())
    }
    
    /// Aktif mi?
    pub fn is_active(&self) -> bool {
        self.state == SessionState::Active
    }
    
    /// Mesaj ekle
    pub fn add_message(&mut self, message: Message) {
        self.token_count += message.token_count as u64;
        self.messages.push(message);
    }
    
    /// Süre (saniye)
    pub fn duration_secs(&self) -> u64 {
        match (self.started_at, self.ended_at) {
            (Some(start), Some(end)) => (end - start).num_seconds() as u64,
            (Some(start), None) if self.state == SessionState::Active => {
                (Utc::now() - start).num_seconds() as u64
            }
            _ => 0,
        }
    }
    
    /// Mesaj sayısı
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SESSION CONFIG
// ═══════════════════════════════════════════════════════════════════════════════

/// Oturum yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Oturum adı
    pub name: String,
    /// Maksimum token
    pub max_tokens: u64,
    /// Sıkıştırma eşiği
    pub compaction_threshold: u64,
    /// Otomatik sıkıştırma
    pub auto_compact: bool,
    /// Model
    pub model: String,
    /// Sistem promptu
    pub system_prompt: Option<String>,
    /// Temperature
    pub temperature: f32,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            name: "default".into(),
            max_tokens: 128000,
            compaction_threshold: 100000,
            auto_compact: true,
            model: "qwen/qwen3-1.7b:free".into(),
            system_prompt: None,
            temperature: 0.7,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SESSION STATE & TYPE
// ═══════════════════════════════════════════════════════════════════════════════

/// Oturum durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionState {
    Pending,
    Active,
    Paused,
    Ended,
    Error,
}

/// Oturum tipi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionType {
    Default,
    ReAct,
    Swarm,
    Research,
    Development,
    Interactive,
}

// ═══════════════════════════════════════════════════════════════════════════════
// MESSAGE
// ═══════════════════════════════════════════════════════════════════════════════

/// Mesaj
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub role: MessageRole,
    pub content: String,
    pub token_count: usize,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Message {
    pub fn user(content: impl Into<String>, token_count: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: MessageRole::User,
            content: content.into(),
            token_count,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        }
    }
    
    pub fn assistant(content: impl Into<String>, token_count: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: MessageRole::Assistant,
            content: content.into(),
            token_count,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        }
    }
    
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: MessageRole::System,
            content: content.into(),
            token_count: 0,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        }
    }
    
    pub fn tool_call(tool_name: &str, args: &serde_json::Value, token_count: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: MessageRole::ToolCall,
            content: serde_json::to_string(&serde_json::json!({
                "tool": tool_name,
                "args": args
            })).unwrap_or_default(),
            token_count,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        }
    }
    
    pub fn tool_result(tool_name: &str, result: &serde_json::Value, token_count: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: MessageRole::ToolResult,
            content: serde_json::to_string(&serde_json::json!({
                "tool": tool_name,
                "result": result
            })).unwrap_or_default(),
            token_count,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageRole {
    System,
    User,
    Assistant,
    ToolCall,
    ToolResult,
}

// ═══════════════════════════════════════════════════════════════════════════════
// SESSION CONTEXT
// ═══════════════════════════════════════════════════════════════════════════════

/// Oturum bağlamı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContext {
    /// Aktif hedef
    pub current_goal: Option<String>,
    /// Aktif görevler
    pub active_tasks: Vec<String>,
    /// Çalışma dizini
    pub working_directory: String,
    /// Ortam değişkenleri
    pub env: HashMap<String, String>,
    /// Kullanıcı tercihleri
    pub user_preferences: HashMap<String, String>,
    /// Dosya bağlamı
    pub file_context: Vec<FileContext>,
    /// Önceki sonuçlar
    pub previous_results: Vec<serde_json::Value>,
}

impl Default for SessionContext {
    fn default() -> Self {
        Self {
            current_goal: None,
            active_tasks: Vec::new(),
            working_directory: std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| "/".into()),
            env: HashMap::new(),
            user_preferences: HashMap::new(),
            file_context: Vec::new(),
            previous_results: Vec::new(),
        }
    }
}

/// Dosya bağlamı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContext {
    pub path: String,
    pub content_preview: String,
    pub line_count: usize,
    pub language: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_session_creation() {
        let session = Session::new(SessionConfig::default());
        assert_eq!(session.state, SessionState::Pending);
    }
    
    #[test]
    fn test_session_lifecycle() {
        let mut session = Session::new(SessionConfig::default());
        
        session.start().expect("operation failed");
        assert_eq!(session.state, SessionState::Active);
        
        session.pause().expect("operation failed");
        assert_eq!(session.state, SessionState::Paused);
        
        session.end().expect("operation failed");
        assert_eq!(session.state, SessionState::Ended);
    }
    
    #[test]
    fn test_message_addition() {
        let mut session = Session::new(SessionConfig::default());
        session.add_message(Message::user("Merhaba", 5));
        session.add_message(Message::assistant("Merhaba!", 10));
        
        assert_eq!(session.message_count(), 2);
        assert_eq!(session.token_count, 15);
    }
}
