//! Core types for connectors

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════════
// DOCUMENT - Universal data unit
// ═══════════════════════════════════════════════════════════════════════════════

/// Universal document type returned by all connectors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Unique ID from source
    pub id: String,
    /// Source connector (gmail, calendar, weather, etc.)
    pub source: String,
    /// Document type (email, event, article, notification, etc.)
    pub doc_type: DocumentType,
    /// Title/subject
    pub title: String,
    /// Content/body
    pub content: String,
    /// Timestamp from source
    pub timestamp: DateTime<Utc>,
    /// Author/sender
    pub author: Option<String>,
    /// Tags/labels
    pub tags: Vec<String>,
    /// URL to original
    pub url: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Document {
    pub fn new(source: &str, doc_type: DocumentType, id: &str, title: &str) -> Self {
        Self {
            id: id.to_string(),
            source: source.to_string(),
            doc_type,
            title: title.to_string(),
            content: String::new(),
            timestamp: Utc::now(),
            author: None,
            tags: Vec::new(),
            url: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    pub fn with_author(mut self, author: &str) -> Self {
        self.author = Some(author.to_string());
        self
    }

    pub fn with_url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    pub fn with_metadata(mut self, key: &str, value: serde_json::Value) -> Self {
        self.metadata.insert(key.to_string(), value);
        self
    }
}

/// Document types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DocumentType {
    Email,
    Event,
    Task,
    Article,
    Notification,
    Message,
    Contact,
    File,
    Weather,
    Custom(String),
}

impl std::fmt::Display for DocumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocumentType::Email => write!(f, "email"),
            DocumentType::Event => write!(f, "event"),
            DocumentType::Task => write!(f, "task"),
            DocumentType::Article => write!(f, "article"),
            DocumentType::Notification => write!(f, "notification"),
            DocumentType::Message => write!(f, "message"),
            DocumentType::Contact => write!(f, "contact"),
            DocumentType::File => write!(f, "file"),
            DocumentType::Weather => write!(f, "weather"),
            DocumentType::Custom(s) => write!(f, "{}", s),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// CREDENTIALS - Authentication data
// ═══════════════════════════════════════════════════════════════════════════════

/// Credentials for connector authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    /// Credential type
    pub cred_type: CredentialType,
    /// API key (for simple auth)
    pub api_key: Option<String>,
    /// OAuth2 access token
    pub access_token: Option<String>,
    /// OAuth2 refresh token
    pub refresh_token: Option<String>,
    /// Token expiry
    pub expires_at: Option<DateTime<Utc>>,
    /// Username (for basic auth)
    pub username: Option<String>,
    /// Password (for basic auth)
    pub password: Option<String>,
    /// Additional fields
    pub extra: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CredentialType {
    ApiKey,
    OAuth2,
    Basic,
    Token,
    None,
}

impl Credentials {
    pub fn api_key(key: &str) -> Self {
        Self {
            cred_type: CredentialType::ApiKey,
            api_key: Some(key.to_string()),
            access_token: None,
            refresh_token: None,
            expires_at: None,
            username: None,
            password: None,
            extra: HashMap::new(),
        }
    }

    pub fn oauth2(access: &str, refresh: Option<&str>) -> Self {
        Self {
            cred_type: CredentialType::OAuth2,
            api_key: None,
            access_token: Some(access.to_string()),
            refresh_token: refresh.map(|r| r.to_string()),
            expires_at: None,
            username: None,
            password: None,
            extra: HashMap::new(),
        }
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires_at {
            Utc::now() >= expires
        } else {
            false
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SYNC CONFIG
// ═══════════════════════════════════════════════════════════════════════════════

/// Sync configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    /// Sync interval in seconds
    pub interval_secs: u64,
    /// Maximum items per sync
    pub max_items: usize,
    /// Sync since last sync only
    pub incremental: bool,
    /// Retry on failure
    pub retry_count: u32,
    /// Retry delay in seconds
    pub retry_delay_secs: u64,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            interval_secs: 300, // 5 minutes
            max_items: 100,
            incremental: true,
            retry_count: 3,
            retry_delay_secs: 30,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// EMAIL - Gmail specific types
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    pub id: String,
    pub thread_id: Option<String>,
    pub subject: String,
    pub from: EmailAddress,
    pub to: Vec<EmailAddress>,
    pub cc: Vec<EmailAddress>,
    pub body_text: String,
    pub body_html: Option<String>,
    pub date: DateTime<Utc>,
    pub labels: Vec<String>,
    pub is_read: bool,
    pub is_starred: bool,
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAddress {
    pub name: Option<String>,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub filename: String,
    pub mime_type: String,
    pub size: u64,
    pub url: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// CALENDAR - Google Calendar specific types
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub id: String,
    pub summary: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub is_all_day: bool,
    pub attendees: Vec<Attendee>,
    pub reminders: Vec<Reminder>,
    pub recurrence: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attendee {
    pub email: String,
    pub name: Option<String>,
    pub response_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    pub method: String, // email, popup
    pub minutes_before: u32,
}

// ═══════════════════════════════════════════════════════════════════════════════
// WEATHER - Weather data types
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherData {
    pub location: String,
    pub timestamp: DateTime<Utc>,
    pub temperature: f64,
    pub feels_like: f64,
    pub humidity: u32,
    pub wind_speed: f64,
    pub wind_direction: String,
    pub condition: String,
    pub condition_icon: Option<String>,
    pub precipitation: f64,
    pub visibility: f64,
    pub forecast: Vec<WeatherForecast>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherForecast {
    pub date: DateTime<Utc>,
    pub temp_high: f64,
    pub temp_low: f64,
    pub condition: String,
}

// ═══════════════════════════════════════════════════════════════════════════════
// RSS/NEWS - Feed types
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedItem {
    pub id: String,
    pub feed_url: String,
    pub feed_title: String,
    pub title: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub link: String,
    pub published: Option<DateTime<Utc>>,
    pub author: Option<String>,
    pub categories: Vec<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// GITHUB - Notification types
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubNotification {
    pub id: String,
    pub notification_type: String,
    pub repository: String,
    pub subject: String,
    pub url: Option<String>,
    pub reason: String,
    pub updated_at: DateTime<Utc>,
    pub is_read: bool,
}
