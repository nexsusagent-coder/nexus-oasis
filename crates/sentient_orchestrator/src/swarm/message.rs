//! ─── SWARM MESAJLAŞMA SİSTEMİ ───
//!
//! Ajanlar arası iletişim için mesaj formatları ve kanal yönetimi.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use super::{SwarmAgentId, SwarmTask};

/// ─── SWARM MESSAGE ───
/// 
/// Swarm içindeki tüm iletişim bu formatı kullanır.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmMessage {
    /// Mesaj ID
    pub id: Uuid,
    /// Gönderen
    pub from: SwarmAgentId,
    /// Alıcı (None = broadcast)
    pub to: Option<SwarmAgentId>,
    /// Mesaj tipi
    pub message_type: MessageType,
    /// Öncelik
    pub priority: MessagePriority,
    /// İçerik
    pub content: serde_json::Value,
    /// Meta veriler
    pub metadata: HashMap<String, String>,
    /// Zaman damgası
    pub timestamp: DateTime<Utc>,
    /// Yanıt bekleniyor mu?
    pub requires_response: bool,
    /// İlgili görev ID
    pub task_id: Option<Uuid>,
    /// Konuşma ID (thread)
    pub conversation_id: Option<Uuid>,
}

impl SwarmMessage {
    /// Yeni mesaj oluştur
    pub fn new(from: SwarmAgentId, message_type: MessageType, content: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            from,
            to: None,
            message_type,
            priority: MessagePriority::Normal,
            content,
            metadata: HashMap::new(),
            timestamp: Utc::now(),
            requires_response: false,
            task_id: None,
            conversation_id: None,
        }
    }
    
    /// Alıcı belirt
    pub fn to(mut self, agent_id: SwarmAgentId) -> Self {
        self.to = Some(agent_id);
        self
    }
    
    /// Broadcast (herkese)
    pub fn broadcast(mut self) -> Self {
        self.to = None;
        self
    }
    
    /// Öncelik ayarla
    pub fn with_priority(mut self, priority: MessagePriority) -> Self {
        self.priority = priority;
        self
    }
    
    /// Yanıt bekle
    pub fn expect_response(mut self) -> Self {
        self.requires_response = true;
        self
    }
    
    /// Görev bağla
    pub fn for_task(mut self, task_id: Uuid) -> Self {
        self.task_id = Some(task_id);
        self
    }
    
    /// Meta veri ekle
    pub fn with_meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
    
    /// Yanıt mesajı oluştur
    pub fn reply(&self, content: serde_json::Value) -> Self {
        Self::new(self.to.clone().unwrap_or_else(|| self.from.clone()), MessageType::Response, content)
            .with_priority(self.priority)
            .for_task(self.task_id.unwrap_or_else(Uuid::new_v4))
    }
    
    /// Görev atama mesajı
    pub fn task_assignment(from: SwarmAgentId, to: SwarmAgentId, task: &SwarmTask) -> Self {
        Self::new(from, MessageType::TaskAssignment, serde_json::to_value(task).unwrap_or_default())
            .to(to)
            .with_priority(MessagePriority::High)
            .for_task(task.id)
    }
    
    /// Görev tamamlandı mesajı
    pub fn task_completed(from: SwarmAgentId, task: &SwarmTask) -> Self {
        Self::new(from, MessageType::TaskCompletion, serde_json::json!({
            "task_id": task.id,
            "status": task.status,
            "result": task.result
        }))
        .with_priority(MessagePriority::High)
        .for_task(task.id)
    }
    
    /// Yardım isteği
    pub fn help_request(from: SwarmAgentId, reason: String) -> Self {
        let from_str = from.as_str().to_string();
        Self::new(from.clone(), MessageType::HelpRequest, serde_json::json!({
            "reason": reason,
            "agent_type": from_str
        }))
        .broadcast()
        .expect_response()
        .with_priority(MessagePriority::Urgent)
    }
    
    /// Durum güncellemesi
    pub fn status_update(from: SwarmAgentId, status: &str, details: serde_json::Value) -> Self {
        Self::new(from, MessageType::StatusUpdate, serde_json::json!({
            "status": status,
            "details": details
        }))
        .broadcast()
    }
}

/// ─── MESSAGE TYPE ───

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    // Görev iletişimi
    TaskAssignment,
    TaskAcceptance,
    TaskRejection,
    TaskCompletion,
    TaskFailure,
    TaskQuery,
    
    // Bilgi paylaşımı
    StatusUpdate,
    KnowledgeShare,
    Query,
    Response,
    
    // Koordinasyon
    Handshake,
    Negotiation,
    Synchronization,
    
    // Yardım
    HelpRequest,
    HelpOffer,
    Delegation,
    
    // Kontrol
    Heartbeat,
    Acknowledgment,
    Error,
    
    // Sistem
    Register,
    Deregister,
    Config,
}

impl MessageType {
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::TaskAssignment => "📤",
            Self::TaskAcceptance => "✅",
            Self::TaskRejection => "❌",
            Self::TaskCompletion => "🎉",
            Self::TaskFailure => "💥",
            Self::TaskQuery => "❓",
            Self::StatusUpdate => "📊",
            Self::KnowledgeShare => "💡",
            Self::Query => "🔍",
            Self::Response => "📨",
            Self::Handshake => "🤝",
            Self::Negotiation => "🤔",
            Self::Synchronization => "🔄",
            Self::HelpRequest => "🆘",
            Self::HelpOffer => "🤝",
            Self::Delegation => "👉",
            Self::Heartbeat => "💓",
            Self::Acknowledgment => "👌",
            Self::Error => "⚠️",
            Self::Register => "📝",
            Self::Deregister => "👋",
            Self::Config => "⚙️",
        }
    }
}

/// ─── MESSAGE PRIORITY ───

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MessagePriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Urgent = 3,
    Critical = 4,
}

impl Default for MessagePriority {
    fn default() -> Self {
        Self::Normal
    }
}

/// ─── MESSAGE QUEUE ───
/// 
/// Her ajan için mesaj kuyruğu yönetimi.

#[derive(Debug, Clone)]
pub struct MessageQueue {
    messages: Vec<SwarmMessage>,
    max_size: usize,
}

impl MessageQueue {
    pub fn new(max_size: usize) -> Self {
        Self {
            messages: Vec::new(),
            max_size,
        }
    }
    
    pub fn push(&mut self, message: SwarmMessage) -> Result<(), String> {
        if self.messages.len() >= self.max_size {
            // En eski mesaji cikar
            self.messages.remove(0);
        }
        
        // Oncelige gore sirala (yuksek oncelik sona)
        let pos = self.messages.iter()
            .position(|m| m.priority > message.priority)
            .unwrap_or(self.messages.len());
        
        self.messages.insert(pos, message);
        Ok(())
    }
    
    pub fn pop(&mut self) -> Option<SwarmMessage> {
        // Yuksek oncelikli mesajlari once cikar (sondan)
        self.messages.pop()
    }
    
    pub fn peek(&self) -> Option<&SwarmMessage> {
        self.messages.last()
    }
    
    pub fn len(&self) -> usize {
        self.messages.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
    
    pub fn clear(&mut self) {
        self.messages.clear();
    }
    
    pub fn filter_by_type(&self, msg_type: MessageType) -> Vec<&SwarmMessage> {
        self.messages.iter()
            .filter(|m| m.message_type == msg_type)
            .collect()
    }
    
    pub fn filter_by_sender(&self, sender: &SwarmAgentId) -> Vec<&SwarmMessage> {
        self.messages.iter()
            .filter(|m| &m.from == sender)
            .collect()
    }
}

impl Default for MessageQueue {
    fn default() -> Self {
        Self::new(100)
    }
}

/// ─── CONVERSATION TRACKER ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: Uuid,
    pub participants: Vec<SwarmAgentId>,
    pub messages: Vec<SwarmMessage>,
    pub topic: Option<String>,
    pub started_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
}

impl Conversation {
    pub fn new(participants: Vec<SwarmAgentId>) -> Self {
        Self {
            id: Uuid::new_v4(),
            participants,
            messages: Vec::new(),
            topic: None,
            started_at: Utc::now(),
            last_activity: Utc::now(),
        }
    }
    
    pub fn add_message(&mut self, message: SwarmMessage) {
        self.last_activity = Utc::now();
        self.messages.push(message);
    }
    
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }
    
    pub fn duration_secs(&self) -> i64 {
        (self.last_activity - self.started_at).num_seconds()
    }
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_message_creation() {
        let from = SwarmAgentId::new();
        let msg = SwarmMessage::new(from, MessageType::StatusUpdate, serde_json::json!({"status": "active"}));
        
        assert!(msg.to.is_none());
        assert_eq!(msg.priority, MessagePriority::Normal);
    }
    
    #[test]
    fn test_message_to() {
        let from = SwarmAgentId::new();
        let to = SwarmAgentId::new();
        let msg = SwarmMessage::new(from.clone(), MessageType::TaskAssignment, serde_json::Value::Null)
            .to(to.clone());
        
        assert_eq!(msg.to, Some(to));
    }
    
    #[test]
    fn test_message_queue_push() {
        let mut queue = MessageQueue::new(10);
        
        let msg1 = SwarmMessage::new(SwarmAgentId::new(), MessageType::StatusUpdate, serde_json::Value::Null)
            .with_priority(MessagePriority::Low);
        let msg2 = SwarmMessage::new(SwarmAgentId::new(), MessageType::StatusUpdate, serde_json::Value::Null)
            .with_priority(MessagePriority::High);
        
        queue.push(msg1).expect("operation failed");
        queue.push(msg2).expect("operation failed");
        
        // Yüksek öncelikli önce çıkmalı
        let popped = queue.pop().expect("operation failed");
        assert_eq!(popped.priority, MessagePriority::High);
    }
    
    #[test]
    fn test_message_queue_max_size() {
        let mut queue = MessageQueue::new(3);
        
        for i in 0..5 {
            let msg = SwarmMessage::new(SwarmAgentId::new(), MessageType::StatusUpdate, serde_json::json!(i));
            queue.push(msg).expect("operation failed");
        }
        
        assert_eq!(queue.len(), 3);
    }
    
    #[test]
    fn test_message_reply() {
        let from = SwarmAgentId::new();
        let to = SwarmAgentId::new();
        let original = SwarmMessage::new(from.clone(), MessageType::Query, serde_json::json!("?"))
            .to(to.clone())
            .for_task(Uuid::new_v4());
        
        let reply = original.reply(serde_json::json!("Answer"));
        
        assert_eq!(reply.from, to);
        assert_eq!(reply.message_type, MessageType::Response);
    }
    
    #[test]
    fn test_help_request() {
        let from = SwarmAgentId::new();
        let help = SwarmMessage::help_request(from, "Stuck on task".into());
        
        assert!(help.to.is_none()); // broadcast
        assert!(help.requires_response);
        assert_eq!(help.priority, MessagePriority::Urgent);
    }
    
    #[test]
    fn test_message_type_emoji() {
        assert!(!MessageType::TaskAssignment.emoji().is_empty());
        assert!(!MessageType::Heartbeat.emoji().is_empty());
    }
}
