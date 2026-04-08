//! ─── WEBHOOK EVENT ───
//!
//! Webhook'tan gelen veriyi normalize edilmiş event'e dönüştürür.
//! Event Listener tarafından işlenmek üzere hazırlar.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use super::providers::WebhookProvider;

/// ─── WEBHOOK EVENT ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    /// Event ID
    pub id: Uuid,
    
    /// Provider
    pub provider: WebhookProvider,
    
    /// Event tipi
    pub event_type: EventType,
    
    /// Provider'dan gelen ham event tipi
    pub raw_event_type: String,
    
    /// Öncelik
    pub priority: EventPriority,
    
    /// Aksiyon
    pub action: EventAction,
    
    /// Kaynak
    pub source: String,
    
    /// Hedef (kime/action)
    pub target: Option<String>,
    
    /// Mesaj/Özet
    pub message: String,
    
    /// Ham veri (JSON)
    pub payload: serde_json::Value,
    
    /// Metadata
    pub metadata: HashMap<String, String>,
    
    /// Zaman damgası
    pub timestamp: DateTime<Utc>,
    
    /// İşlendi mi?
    pub processed: bool,
    
    /// Oluşturulan görev ID'si
    pub task_id: Option<Uuid>,
}

impl WebhookEvent {
    /// Yeni event oluştur
    pub fn new(
        provider: WebhookProvider,
        event_type: EventType,
        raw_event_type: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            provider,
            event_type,
            raw_event_type: raw_event_type.into(),
            priority: EventPriority::Normal,
            action: EventAction::Notify,
            source: String::new(),
            target: None,
            message: String::new(),
            payload: serde_json::json!({}),
            metadata: HashMap::new(),
            timestamp: Utc::now(),
            processed: false,
            task_id: None,
        }
    }
    
    /// GitHub push event
    pub fn github_push(repo: &str, branch: &str, commits: usize) -> Self {
        Self::new(WebhookProvider::GitHub, EventType::CodePush, "push")
            .with_source(repo)
            .with_target(branch)
            .with_message(format!("{} branch'ine {} commit push edildi", branch, commits))
            .with_action(EventAction::Analyze)
            .with_metadata("repo", repo)
            .with_metadata("branch", branch)
    }
    
    /// GitHub PR event
    pub fn github_pr(repo: &str, pr_number: u64, action: &str, title: &str) -> Self {
        Self::new(WebhookProvider::GitHub, EventType::PullRequest, "pull_request")
            .with_source(repo)
            .with_target(format!("#{}", pr_number))
            .with_message(format!("PR #{} {}: {}", pr_number, action, title))
            .with_action(EventAction::Review)
            .with_metadata("repo", repo)
            .with_metadata("pr_number", &pr_number.to_string())
            .with_metadata("action", action)
    }
    
    /// GitHub Issue event
    pub fn github_issue(repo: &str, issue_number: u64, action: &str, title: &str) -> Self {
        Self::new(WebhookProvider::GitHub, EventType::Issue, "issues")
            .with_source(repo)
            .with_target(format!("#{}", issue_number))
            .with_message(format!("Issue #{} {}: {}", issue_number, action, title))
            .with_action(EventAction::Respond)
            .with_metadata("repo", repo)
            .with_metadata("issue_number", &issue_number.to_string())
            .with_metadata("action", action)
    }
    
    /// Stripe payment event
    pub fn stripe_payment(event_type: &str, amount: Option<f64>, currency: Option<&str>) -> Self {
        let event = if event_type.contains("succeeded") {
            EventType::PaymentSuccess
        } else if event_type.contains("failed") {
            EventType::PaymentFailed
        } else {
            EventType::PaymentEvent
        };
        
        let message = match (amount, currency) {
            (Some(a), Some(c)) => format!("Ödeme: {} {} - {}", a, c, event_type),
            _ => format!("Ödeme olayı: {}", event_type),
        };
        
        Self::new(WebhookProvider::Stripe, event, event_type)
            .with_message(message)
            .with_action(EventAction::Log)
            .with_priority(EventPriority::High)
    }
    
    /// n8n workflow event
    pub fn n8n_workflow(workflow_name: &str, event: &str, data: serde_json::Value) -> Self {
        Self::new(WebhookProvider::N8n, EventType::WorkflowTrigger, event)
            .with_source(workflow_name)
            .with_message(format!("n8n: {} - {}", workflow_name, event))
            .with_payload(data)
            .with_action(EventAction::Execute)
    }
    
    /// Slack message event
    pub fn slack_message(channel: &str, user: &str, text: &str) -> Self {
        Self::new(WebhookProvider::Slack, EventType::Message, "message")
            .with_source(channel)
            .with_target(user)
            .with_message(format!("Slack [{}]: {}", channel, text))
            .with_action(EventAction::Respond)
            .with_metadata("channel", channel)
            .with_metadata("user", user)
    }
    
    // Builder pattern
    
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = source.into();
        self
    }
    
    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }
    
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }
    
    pub fn with_action(mut self, action: EventAction) -> Self {
        self.action = action;
        self
    }
    
    pub fn with_priority(mut self, priority: EventPriority) -> Self {
        self.priority = priority;
        self
    }
    
    pub fn with_payload(mut self, payload: serde_json::Value) -> Self {
        self.payload = payload;
        self
    }
    
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
    
    /// Event özeti
    pub fn summary(&self) -> String {
        format!(
            "[{}] {} {}: {}",
            self.provider,
            self.event_type,
            self.source,
            self.message.chars().take(80).collect::<String>()
        )
    }
    
    /// SENTIENT için görev açıklaması oluştur
    pub fn to_task_description(&self) -> String {
        match self.action {
            EventAction::Analyze => {
                format!(
                    "{} kaynağından gelen {} olayını analiz et. {}",
                    self.provider, self.event_type, self.message
                )
            }
            EventAction::Review => {
                format!(
                    "{} üzerindeki {} öğesini incele. {}",
                    self.source, self.event_type, self.message
                )
            }
            EventAction::Respond => {
                format!(
                    "{} kaynağından gelen mesaja yanıt oluştur. {}",
                    self.provider, self.message
                )
            }
            EventAction::Execute => {
                format!(
                    "{} workflow'undan gelen tetiklemeyi işle. {}",
                    self.provider, self.message
                )
            }
            EventAction::Notify => {
                format!("Bildirim: {}", self.message)
            }
            EventAction::Log => {
                format!("Kaydet: {}", self.message)
            }
            EventAction::Ignore => {
                format!("Yoksay: {}", self.message)
            }
        }
    }
}

/// ─── EVENT TYPE ───

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    // Kod olayları
    CodePush,
    PullRequest,
    Issue,
    Release,
    
    // Ödeme olayları
    PaymentEvent,
    PaymentSuccess,
    PaymentFailed,
    InvoiceEvent,
    
    // Workflow olayları
    WorkflowTrigger,
    WorkflowComplete,
    WorkflowFailed,
    
    // İletişim olayları
    Message,
    Command,
    Mention,
    
    // Sistem olayları
    SystemAlert,
    HealthCheck,
    CustomEvent,
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CodePush => write!(f, "CodePush"),
            Self::PullRequest => write!(f, "PullRequest"),
            Self::Issue => write!(f, "Issue"),
            Self::Release => write!(f, "Release"),
            Self::PaymentEvent => write!(f, "Payment"),
            Self::PaymentSuccess => write!(f, "PaymentSuccess"),
            Self::PaymentFailed => write!(f, "PaymentFailed"),
            Self::InvoiceEvent => write!(f, "Invoice"),
            Self::WorkflowTrigger => write!(f, "WorkflowTrigger"),
            Self::WorkflowComplete => write!(f, "WorkflowComplete"),
            Self::WorkflowFailed => write!(f, "WorkflowFailed"),
            Self::Message => write!(f, "Message"),
            Self::Command => write!(f, "Command"),
            Self::Mention => write!(f, "Mention"),
            Self::SystemAlert => write!(f, "SystemAlert"),
            Self::HealthCheck => write!(f, "HealthCheck"),
            Self::CustomEvent => write!(f, "Custom"),
        }
    }
}

/// ─── EVENT PRIORITY ───

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl Default for EventPriority {
    fn default() -> Self {
        Self::Normal
    }
}

/// ─── EVENT ACTION ───

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventAction {
    /// Analiz et
    Analyze,
    
    /// İncele/Review
    Review,
    
    /// Yanıt ver
    Respond,
    
    /// Çalıştır
    Execute,
    
    /// Bildir
    Notify,
    
    /// Kaydet
    Log,
    
    /// Yoksay
    Ignore,
}

impl Default for EventAction {
    fn default() -> Self {
        Self::Notify
    }
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_github_push_event() {
        let event = WebhookEvent::github_push("user/repo", "main", 3);
        
        assert_eq!(event.provider, WebhookProvider::GitHub);
        assert_eq!(event.event_type, EventType::CodePush);
        assert!(event.message.contains("3 commit"));
        assert_eq!(event.action, EventAction::Analyze);
    }
    
    #[test]
    fn test_github_pr_event() {
        let event = WebhookEvent::github_pr("user/repo", 42, "opened", "Fix bug");
        
        assert_eq!(event.event_type, EventType::PullRequest);
        assert!(event.message.contains("#42"));
        assert_eq!(event.action, EventAction::Review);
    }
    
    #[test]
    fn test_stripe_payment_event() {
        let event = WebhookEvent::stripe_payment("payment_intent.succeeded", Some(99.99), Some("USD"));
        
        assert_eq!(event.event_type, EventType::PaymentSuccess);
        assert_eq!(event.priority, EventPriority::High);
    }
    
    #[test]
    fn test_n8n_workflow_event() {
        let event = WebhookEvent::n8n_workflow(
            "Data Sync",
            "completed",
            serde_json::json!({"records": 100})
        );
        
        assert_eq!(event.provider, WebhookProvider::N8n);
        assert_eq!(event.event_type, EventType::WorkflowTrigger);
        assert_eq!(event.action, EventAction::Execute);
    }
    
    #[test]
    fn test_event_summary() {
        let event = WebhookEvent::github_push("user/repo", "main", 5);
        let summary = event.summary();
        
        assert!(summary.contains("github"));
        assert!(summary.contains("CodePush"));
    }
    
    #[test]
    fn test_to_task_description() {
        let event = WebhookEvent::github_pr("user/repo", 1, "opened", "Test PR");
        let desc = event.to_task_description();
        
        assert!(desc.contains("incele"));
        assert!(desc.contains("PullRequest"));
    }
}
