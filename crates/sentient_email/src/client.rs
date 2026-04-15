//! ─── Email Client ───

use std::sync::Arc;

use crate::models::*;
use crate::gmail::GmailClient;
use crate::imap_client::ImapClient;
use crate::smtp_client::SmtpClient;
use crate::summarize::EmailSummarizer;
use crate::actions::ActionDetector;
use crate::{EmailResult, EmailError};

/// Email client configuration
#[derive(Debug, Clone)]
pub struct EmailConfig {
    /// Provider type
    pub provider: EmailProvider,
    
    /// OAuth2 access token (for Gmail)
    pub access_token: Option<String>,
    
    /// IMAP settings
    pub imap_host: Option<String>,
    pub imap_port: Option<u16>,
    pub imap_username: Option<String>,
    pub imap_password: Option<String>,
    
    /// SMTP settings
    pub smtp_host: Option<String>,
    pub smtp_port: Option<u16>,
    pub smtp_username: Option<String>,
    pub smtp_password: Option<String>,
    
    /// AI summarization enabled
    pub enable_summarization: bool,
    
    /// Action detection enabled
    pub enable_action_detection: bool,
}

impl Default for EmailConfig {
    fn default() -> Self {
        Self {
            provider: EmailProvider::Gmail,
            access_token: None,
            imap_host: None,
            imap_port: None,
            imap_username: None,
            imap_password: None,
            smtp_host: None,
            smtp_port: None,
            smtp_username: None,
            smtp_password: None,
            enable_summarization: true,
            enable_action_detection: true,
        }
    }
}

/// Email provider type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmailProvider {
    Gmail,
    Outlook,
    Yahoo,
    Imap,  // Generic IMAP
    ProtonMail,
}

/// Main email client
pub struct EmailClient {
    config: EmailConfig,
    gmail: Option<GmailClient>,
    imap: Option<ImapClient>,
    smtp: Option<SmtpClient>,
    summarizer: EmailSummarizer,
    action_detector: ActionDetector,
}

impl EmailClient {
    /// Create new email client with config
    pub fn new(config: EmailConfig) -> EmailResult<Self> {
        let summarizer = EmailSummarizer::new();
        let action_detector = ActionDetector::new();
        
        let mut client = Self {
            config: config.clone(),
            gmail: None,
            imap: None,
            smtp: None,
            summarizer,
            action_detector,
        };
        
        // Initialize provider clients
        match client.config.provider {
            EmailProvider::Gmail => {
                if let Some(token) = &client.config.access_token {
                    client.gmail = Some(GmailClient::new(token.clone()));
                }
            }
            EmailProvider::Imap | EmailProvider::Outlook | EmailProvider::Yahoo | EmailProvider::ProtonMail => {
                if let (Some(host), Some(user), Some(pass)) = (
                    &client.config.imap_host,
                    &client.config.imap_username,
                    &client.config.imap_password,
                ) {
                    client.imap = Some(ImapClient::new(
                        host.clone(),
                        client.config.imap_port.unwrap_or(993),
                        user.clone(),
                        pass.clone(),
                    ));
                }
            }
        }
        
        // Initialize SMTP for sending
        if let (Some(host), Some(user), Some(pass)) = (
            &client.config.smtp_host,
            &client.config.smtp_username,
            &client.config.smtp_password,
        ) {
            client.smtp = Some(SmtpClient::new(
                host.clone(),
                config.smtp_port.unwrap_or(587),
                user.clone(),
                pass.clone(),
            ));
        }
        
        Ok(client)
    }
    
    /// Create Gmail client from environment
    pub async fn gmail_from_env() -> EmailResult<Self> {
        let access_token = std::env::var("GMAIL_ACCESS_TOKEN")
            .map_err(|_| EmailError::AuthFailed("GMAIL_ACCESS_TOKEN not set".into()))?;
        
        Self::new(EmailConfig {
            provider: EmailProvider::Gmail,
            access_token: Some(access_token),
            ..Default::default()
        })
    }
    
    /// Get unread emails
    pub async fn get_unread(&self, limit: u32) -> EmailResult<Vec<Email>> {
        match (&self.gmail, &self.imap) {
            (Some(gmail), _) => gmail.get_unread(limit).await,
            (_, Some(imap)) => imap.get_unread(limit).await,
            _ => Err(EmailError::ConnectionError("No email provider configured".into())),
        }
    }
    
    /// Get emails from folder
    pub async fn get_folder(&self, folder: EmailFolder, limit: u32) -> EmailResult<Vec<Email>> {
        match (&self.gmail, &self.imap) {
            (Some(gmail), _) => gmail.get_folder(folder, limit).await,
            (_, Some(imap)) => imap.get_folder(folder, limit).await,
            _ => Err(EmailError::ConnectionError("No email provider configured".into())),
        }
    }
    
    /// Search emails
    pub async fn search(&self, query: &EmailQuery) -> EmailResult<Vec<Email>> {
        match (&self.gmail, &self.imap) {
            (Some(gmail), _) => gmail.search(query).await,
            (_, Some(imap)) => imap.search(query).await,
            _ => Err(EmailError::ConnectionError("No email provider configured".into())),
        }
    }
    
    /// Get single email by ID
    pub async fn get_email(&self, id: &str) -> EmailResult<Email> {
        match (&self.gmail, &self.imap) {
            (Some(gmail), _) => gmail.get_email(id).await,
            (_, Some(imap)) => imap.get_email(id).await,
            _ => Err(EmailError::ConnectionError("No email provider configured".into())),
        }
    }
    
    /// Send email
    pub async fn send(&self, email: &ComposeEmail) -> EmailResult<String> {
        match &self.smtp {
            Some(smtp) => smtp.send(email).await,
            None => Err(EmailError::ConnectionError("SMTP not configured".into())),
        }
    }
    
    /// Mark email as read
    pub async fn mark_read(&self, id: &str) -> EmailResult<()> {
        match (&self.gmail, &self.imap) {
            (Some(gmail), _) => gmail.mark_read(id).await,
            (_, Some(imap)) => imap.mark_read(id).await,
            _ => Err(EmailError::ConnectionError("No email provider configured".into())),
        }
    }
    
    /// Move email to folder
    pub async fn move_to(&self, id: &str, folder: EmailFolder) -> EmailResult<()> {
        match (&self.gmail, &self.imap) {
            (Some(gmail), _) => gmail.move_to(id, folder).await,
            (_, Some(imap)) => imap.move_to(id, folder).await,
            _ => Err(EmailError::ConnectionError("No email provider configured".into())),
        }
    }
    
    /// Summarize emails with AI
    pub async fn summarize_emails(&self, emails: &[Email]) -> EmailResult<crate::summarize::EmailSummary> {
        if !self.config.enable_summarization {
            return Err(EmailError::SummarizationError("Summarization disabled".into()));
        }
        
        self.summarizer.summarize(emails).await
    }
    
    /// Detect actions from email
    pub async fn detect_actions(&self, email: &Email) -> Vec<crate::actions::EmailAction> {
        if !self.config.enable_action_detection {
            return vec![];
        }
        
        self.action_detector.detect(email)
    }
    
    /// Get quick summary of inbox
    pub async fn inbox_summary(&self) -> EmailResult<String> {
        let unread = self.get_unread(20).await?;
        
        if unread.is_empty() {
            return Ok("Inbox is empty! 🎉".to_string());
        }
        
        let summary = self.summarize_emails(&unread).await?;
        
        Ok(format!(
            "📬 {} unread emails\n{}\n\nUrgent: {}\nNeeds reply: {}",
            unread.len(),
            summary.overview,
            summary.urgent_count,
            summary.needs_reply_count
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_default() {
        let config = EmailConfig::default();
        assert_eq!(config.provider, EmailProvider::Gmail);
        assert!(config.enable_summarization);
    }
}
