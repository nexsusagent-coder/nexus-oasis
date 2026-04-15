//! ─── SENTIENT EMAIL INTEGRATION ───
//!
//! JARVIS-like email management system
//!
//! # Features
//! - **Gmail API**: OAuth2 authentication, full email access
//! - **IMAP/SMTP**: Generic provider support
//! - **Smart Summarization**: AI-powered email summaries
//! - **Action Detection**: "Reply needed", "Urgent", "Calendar event"
//!
//! # Example
//! ```rust,ignore
//! use sentient_email::{EmailClient, EmailConfig};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = EmailClient::gmail_from_env().await?;
//!     
//!     // Get recent emails
//!     let emails = client.get_unread(10).await?;
//!     
//!     // Summarize with AI
//!     let summary = client.summarize_emails(&emails).await?;
//!     println!("Email summary: {}", summary);
//! }
//! ```

pub mod client;
pub mod gmail;
pub mod imap_client;
pub mod smtp_client;
pub mod models;
pub mod summarize;
pub mod actions;

pub use client::{EmailClient, EmailConfig, EmailProvider};
pub use models::{Email, EmailAddress, EmailFolder, EmailAttachment, EmailFlags};
pub use summarize::{EmailSummarizer, EmailSummary, SummaryConfig};
pub use actions::{ActionDetector, EmailAction, ActionType};

pub mod prelude {
    pub use crate::{EmailClient, Email, EmailSummary, EmailAction};
}

/// Result type for email operations
pub type EmailResult<T> = Result<T, EmailError>;

/// Error type
#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("Authentication failed: {0}")]
    AuthFailed(String),
    
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    #[error("API error: {0}")]
    ApiError(String),
    
    #[error("Email not found: {0}")]
    NotFound(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Summarization error: {0}")]
    SummarizationError(String),
    
    #[error(transparent)]
    Io(#[from] std::io::Error),
    
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    
    #[error(transparent)]
    Http(#[from] reqwest::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_email_model() {
        let email = Email {
            id: "test-123".into(),
            subject: "Test Subject".into(),
            from: EmailAddress { name: Some("Sender".into()), address: "sender@example.com".into() },
            to: vec![EmailAddress { name: None, address: "me@example.com".into() }],
            cc: vec![],
            date: chrono::Utc::now(),
            body_text: Some("Hello".into()),
            body_html: None,
            folder: EmailFolder::Inbox,
            flags: EmailFlags::default(),
            attachments: vec![],
            thread_id: None,
            priority: None,
        };
        
        assert_eq!(email.id, "test-123");
    }
}
