//! ─── WEBHOOK PROVIDERS ───
//!
//! Desteklenen dış servis provider'ları:
//! - GitHub (push, pull_request, issues, vb.)
//! - Stripe (payment, invoice, vb.)
//! - n8n (custom workflows)
//! - Slack (events, commands)
//! - Generic (herhangi bir JSON webhook)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ─── WEBHOOK PROVIDER ───

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebhookProvider {
    GitHub,
    Stripe,
    N8n,
    Slack,
    Telegram,
    Discord,
    Custom(String),
}

impl std::fmt::Display for WebhookProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GitHub => write!(f, "github"),
            Self::Stripe => write!(f, "stripe"),
            Self::N8n => write!(f, "n8n"),
            Self::Slack => write!(f, "slack"),
            Self::Telegram => write!(f, "telegram"),
            Self::Discord => write!(f, "discord"),
            Self::Custom(name) => write!(f, "{}", name),
        }
    }
}

impl std::str::FromStr for WebhookProvider {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "github" => Ok(Self::GitHub),
            "stripe" => Ok(Self::Stripe),
            "n8n" => Ok(Self::N8n),
            "slack" => Ok(Self::Slack),
            "telegram" => Ok(Self::Telegram),
            "discord" => Ok(Self::Discord),
            other => Ok(Self::Custom(other.to_string())),
        }
    }
}

/// ─── WEBHOOK PAYLOAD (TRAIT) ───

pub trait WebhookPayload: Send + Sync {
    /// Provider tipi
    fn provider(&self) -> WebhookProvider;
    
    /// Event tipi
    fn event_type(&self) -> &str;
    
    /// Ham veri
    fn raw(&self) -> &str;
    
    /// JSON olarak
    fn as_json(&self) -> serde_json::Result<serde_json::Value>;
    
    /// Özet
    fn summary(&self) -> String;
}

/// ─── GITHUB PAYLOAD ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubPayload {
    /// Event tipi (push, pull_request, issues, vb.)
    #[serde(rename = "X-GitHub-Event")]
    pub event: String,
    
    /// Delivery ID
    #[serde(rename = "X-GitHub-Delivery")]
    pub delivery_id: String,
    
    /// Repository
    pub repository: Option<GitHubRepository>,
    
    /// Sender
    pub sender: Option<GitHubUser>,
    
    /// Action (varsa)
    pub action: Option<String>,
    
    /// Ref (push için)
    #[serde(default)]
    pub r#ref: Option<String>,
    
    /// Commits (push için)
    #[serde(default)]
    pub commits: Vec<GitHubCommit>,
    
    /// Pull request
    pub pull_request: Option<GitHubPullRequest>,
    
    /// Issue
    pub issue: Option<GitHubIssue>,
    
    /// Ham veri
    #[serde(skip)]
    pub raw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubRepository {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub private: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubUser {
    pub login: String,
    pub id: u64,
    pub html_url: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubCommit {
    pub id: String,
    pub message: String,
    pub timestamp: String,
    pub author: GitHubUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubPullRequest {
    pub id: u64,
    pub number: u64,
    pub state: String,
    pub title: String,
    pub body: Option<String>,
    pub html_url: String,
    pub user: GitHubUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubIssue {
    pub id: u64,
    pub number: u64,
    pub state: String,
    pub title: String,
    pub body: Option<String>,
    pub html_url: String,
    pub user: GitHubUser,
}

impl WebhookPayload for GitHubPayload {
    fn provider(&self) -> WebhookProvider {
        WebhookProvider::GitHub
    }
    
    fn event_type(&self) -> &str {
        &self.event
    }
    
    fn raw(&self) -> &str {
        &self.raw
    }
    
    fn as_json(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::to_value(self)
    }
    
    fn summary(&self) -> String {
        match self.event.as_str() {
            "push" => {
                let branch = self.r#ref.as_ref()
                    .map(|r| r.strip_prefix("refs/heads/").unwrap_or(r))
                    .unwrap_or("unknown");
                let commit_count = self.commits.len();
                format!("Push to {} ({} commits)", branch, commit_count)
            }
            "pull_request" => {
                let action = self.action.as_deref().unwrap_or("unknown");
                let pr_num = self.pull_request.as_ref()
                    .map(|pr| pr.number)
                    .unwrap_or(0);
                format!("PR #{} {}", pr_num, action)
            }
            "issues" => {
                let action = self.action.as_deref().unwrap_or("unknown");
                let issue_num = self.issue.as_ref()
                    .map(|i| i.number)
                    .unwrap_or(0);
                format!("Issue #{} {}", issue_num, action)
            }
            _ => format!("GitHub event: {}", self.event)
        }
    }
}

/// ─── STRIPE PAYLOAD ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripePayload {
    /// Event ID
    pub id: String,
    
    /// Event type
    #[serde(rename = "type")]
    pub event_type: String,
    
    /// Created timestamp
    pub created: i64,
    
    /// Data
    pub data: StripeData,
    
    /// Ham veri
    #[serde(skip)]
    pub raw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripeData {
    pub object: serde_json::Value,
}

impl WebhookPayload for StripePayload {
    fn provider(&self) -> WebhookProvider {
        WebhookProvider::Stripe
    }
    
    fn event_type(&self) -> &str {
        &self.event_type
    }
    
    fn raw(&self) -> &str {
        &self.raw
    }
    
    fn as_json(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::to_value(self)
    }
    
    fn summary(&self) -> String {
        match self.event_type.as_str() {
            "payment_intent.succeeded" => "Payment succeeded".into(),
            "payment_intent.payment_failed" => "Payment failed".into(),
            "invoice.paid" => "Invoice paid".into(),
            "invoice.payment_failed" => "Invoice payment failed".into(),
            "customer.created" => "Customer created".into(),
            "checkout.session.completed" => "Checkout completed".into(),
            _ => format!("Stripe event: {}", self.event_type)
        }
    }
}

/// ─── N8N PAYLOAD ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct N8nPayload {
    /// Workflow ID
    #[serde(default)]
    pub workflow_id: Option<String>,
    
    /// Workflow name
    #[serde(default)]
    pub workflow_name: Option<String>,
    
    /// Execution ID
    #[serde(default)]
    pub execution_id: Option<String>,
    
    /// Event type
    #[serde(default)]
    pub event: Option<String>,
    
    /// Custom data
    #[serde(flatten)]
    pub data: HashMap<String, serde_json::Value>,
    
    /// Ham veri
    #[serde(skip)]
    pub raw: String,
}

impl WebhookPayload for N8nPayload {
    fn provider(&self) -> WebhookProvider {
        WebhookProvider::N8n
    }
    
    fn event_type(&self) -> &str {
        self.event.as_deref().unwrap_or("trigger")
    }
    
    fn raw(&self) -> &str {
        &self.raw
    }
    
    fn as_json(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::to_value(self)
    }
    
    fn summary(&self) -> String {
        match (&self.workflow_name, &self.event) {
            (Some(name), Some(event)) => format!("n8n: {} - {}", name, event),
            (Some(name), None) => format!("n8n: {}", name),
            (None, Some(event)) => format!("n8n workflow: {}", event),
            _ => "n8n webhook trigger".into()
        }
    }
}

/// ─── SLACK PAYLOAD ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackPayload {
    /// Event type
    #[serde(rename = "type")]
    pub event_type: String,
    
    /// Team ID
    #[serde(default)]
    pub team_id: Option<String>,
    
    /// User ID
    #[serde(default)]
    pub user_id: Option<String>,
    
    /// Channel
    #[serde(default)]
    pub channel: Option<String>,
    
    /// Text (message içeriği)
    #[serde(default)]
    pub text: Option<String>,
    
    /// Event data
    #[serde(default)]
    pub event: Option<SlackEvent>,
    
    /// Challenge (URL verification için)
    #[serde(default)]
    pub challenge: Option<String>,
    
    /// Ham veri
    #[serde(skip)]
    pub raw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub user: Option<String>,
    pub channel: Option<String>,
    pub text: Option<String>,
    pub ts: Option<String>,
}

impl WebhookPayload for SlackPayload {
    fn provider(&self) -> WebhookProvider {
        WebhookProvider::Slack
    }
    
    fn event_type(&self) -> &str {
        &self.event_type
    }
    
    fn raw(&self) -> &str {
        &self.raw
    }
    
    fn as_json(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::to_value(self)
    }
    
    fn summary(&self) -> String {
        match (self.event_type.as_str(), &self.text) {
            ("url_verification", _) => "Slack URL verification".into(),
            ("event_callback", Some(text)) => format!("Slack message: {}", 
                text.chars().take(50).collect::<String>()),
            ("event_callback", None) => "Slack event".into(),
            _ => format!("Slack: {}", self.event_type)
        }
    }
}

/// ─── GENERIC PAYLOAD ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericPayload {
    /// Provider adı
    pub provider: String,
    
    /// Event tipi
    #[serde(default)]
    pub event_type: String,
    
    /// Tüm veri
    #[serde(flatten)]
    pub data: HashMap<String, serde_json::Value>,
    
    /// Ham veri
    #[serde(skip)]
    pub raw: String,
}

impl WebhookPayload for GenericPayload {
    fn provider(&self) -> WebhookProvider {
        WebhookProvider::Custom(self.provider.clone())
    }
    
    fn event_type(&self) -> &str {
        &self.event_type
    }
    
    fn raw(&self) -> &str {
        &self.raw
    }
    
    fn as_json(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::to_value(&self.data)
    }
    
    fn summary(&self) -> String {
        format!("{}: {}", self.provider, self.event_type)
    }
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    
    #[test]
    fn test_provider_from_str() {
        assert_eq!(WebhookProvider::from_str("github").unwrap(), WebhookProvider::GitHub);
        assert_eq!(WebhookProvider::from_str("stripe").unwrap(), WebhookProvider::Stripe);
        assert_eq!(WebhookProvider::from_str("custom_service").unwrap(), 
            WebhookProvider::Custom("custom_service".into()));
    }
    
    #[test]
    fn test_github_payload_parsing() {
        let json = r#"{
            "X-GitHub-Event": "push",
            "X-GitHub-Delivery": "12345",
            "ref": "refs/heads/main",
            "commits": [],
            "repository": {
                "id": 1,
                "name": "test",
                "full_name": "user/test",
                "html_url": "https://github.com/user/test",
                "private": false
            }
        }"#;
        
        let payload: GitHubPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.event, "push");
        assert_eq!(payload.r#ref, Some("refs/heads/main".into()));
    }
    
    #[test]
    fn test_stripe_payload_parsing() {
        let json = r#"{
            "id": "evt_123",
            "type": "payment_intent.succeeded",
            "created": 1234567890,
            "data": {
                "object": {}
            }
        }"#;
        
        let payload: StripePayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.event_type, "payment_intent.succeeded");
        assert_eq!(payload.summary(), "Payment succeeded");
    }
    
    #[test]
    fn test_n8n_payload_parsing() {
        let json = r#"{
            "workflow_id": "wf_123",
            "workflow_name": "Test Workflow",
            "event": "completed",
            "result": "success"
        }"#;
        
        let payload: N8nPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.workflow_name, Some("Test Workflow".into()));
        assert_eq!(payload.summary(), "n8n: Test Workflow - completed");
    }
}
