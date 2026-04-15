//! ─── Email Models ───

use serde::{Deserialize, Serialize};

/// Email message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    /// Unique identifier
    pub id: String,
    
    /// Subject line
    pub subject: String,
    
    /// Sender
    pub from: EmailAddress,
    
    /// Recipients
    pub to: Vec<EmailAddress>,
    
    /// CC recipients
    pub cc: Vec<EmailAddress>,
    
    /// Date received
    pub date: chrono::DateTime<chrono::Utc>,
    
    /// Plain text body
    pub body_text: Option<String>,
    
    /// HTML body
    pub body_html: Option<String>,
    
    /// Folder/Label
    pub folder: EmailFolder,
    
    /// Email flags
    pub flags: EmailFlags,
    
    /// Attachments
    pub attachments: Vec<EmailAttachment>,
    
    /// Thread ID (for conversations)
    pub thread_id: Option<String>,
    
    /// Priority (1-5, lower = higher priority)
    pub priority: Option<u8>,
}

impl Email {
    /// Create a new email
    pub fn new(id: &str, subject: &str, from: EmailAddress) -> Self {
        Self {
            id: id.to_string(),
            subject: subject.to_string(),
            from,
            to: vec![],
            cc: vec![],
            date: chrono::Utc::now(),
            body_text: None,
            body_html: None,
            folder: EmailFolder::Inbox,
            flags: EmailFlags::default(),
            attachments: vec![],
            thread_id: None,
            priority: None,
        }
    }
    
    /// Check if email is unread
    pub fn is_unread(&self) -> bool {
        !self.flags.seen
    }
    
    /// Check if email is flagged/starred
    pub fn is_flagged(&self) -> bool {
        self.flags.flagged
    }
    
    /// Check if email has attachments
    pub fn has_attachments(&self) -> bool {
        !self.attachments.is_empty()
    }
    
    /// Get preview text (first 200 chars)
    pub fn preview(&self) -> String {
        self.body_text
            .as_ref()
            .map(|b| b.chars().take(200).collect())
            .unwrap_or_default()
    }
}

/// Email address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAddress {
    /// Display name
    pub name: Option<String>,
    
    /// Email address
    pub address: String,
}

impl EmailAddress {
    pub fn new(address: &str) -> Self {
        Self {
            name: None,
            address: address.to_string(),
        }
    }
    
    pub fn with_name(name: &str, address: &str) -> Self {
        Self {
            name: Some(name.to_string()),
            address: address.to_string(),
        }
    }
    
    pub fn display(&self) -> String {
        match &self.name {
            Some(n) => format!("{} <{}>", n, self.address),
            None => self.address.clone(),
        }
    }
}

/// Email folder/label
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmailFolder {
    Inbox,
    Sent,
    Drafts,
    Spam,
    Trash,
    Archive,
    Important,
    Custom(u8),
}

impl std::fmt::Display for EmailFolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inbox => write!(f, "INBOX"),
            Self::Sent => write!(f, "SENT"),
            Self::Drafts => write!(f, "DRAFTS"),
            Self::Spam => write!(f, "SPAM"),
            Self::Trash => write!(f, "TRASH"),
            Self::Archive => write!(f, "ARCHIVE"),
            Self::Important => write!(f, "IMPORTANT"),
            Self::Custom(id) => write!(f, "CUSTOM_{}", id),
        }
    }
}

/// Email flags
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct EmailFlags {
    pub seen: bool,
    pub answered: bool,
    pub flagged: bool,
    pub deleted: bool,
    pub draft: bool,
    pub recent: bool,
}

/// Email attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAttachment {
    /// Filename
    pub filename: String,
    
    /// MIME type
    pub content_type: String,
    
    /// Size in bytes
    pub size: u64,
    
    /// Content ID (for inline images)
    pub content_id: Option<String>,
    
    /// Attachment data (if downloaded)
    pub data: Option<Vec<u8>>,
}

/// Email search query
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmailQuery {
    /// Search in subject
    pub subject: Option<String>,
    
    /// Search in body
    pub body: Option<String>,
    
    /// From address
    pub from: Option<String>,
    
    /// To address
    pub to: Option<String>,
    
    /// Folder to search
    pub folder: Option<EmailFolder>,
    
    /// Date range
    pub after: Option<chrono::DateTime<chrono::Utc>>,
    pub before: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Has attachments
    pub has_attachments: Option<bool>,
    
    /// Is unread
    pub is_unread: Option<bool>,
    
    /// Is flagged
    pub is_flagged: Option<bool>,
    
    /// Limit results
    pub limit: Option<u32>,
}

/// Email compose request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeEmail {
    /// Recipients
    pub to: Vec<EmailAddress>,
    
    /// CC recipients
    pub cc: Vec<EmailAddress>,
    
    /// BCC recipients
    pub bcc: Vec<EmailAddress>,
    
    /// Subject
    pub subject: String,
    
    /// Body text
    pub body: String,
    
    /// Reply to email ID
    pub reply_to: Option<String>,
    
    /// Forward email ID
    pub forward_from: Option<String>,
    
    /// Attachments
    pub attachments: Vec<EmailAttachment>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_email_creation() {
        let email = Email::new(
            "123",
            "Test",
            EmailAddress::new("test@example.com")
        );
        
        assert!(email.is_unread());
        assert!(!email.has_attachments());
    }
    
    #[test]
    fn test_address_display() {
        let addr = EmailAddress::with_name("John", "john@example.com");
        assert_eq!(addr.display(), "John <john@example.com>");
    }
}
