//! ─── Gmail API Client ───

use reqwest::Client;

use crate::models::*;
use crate::{EmailResult, EmailError};

/// Gmail API client
pub struct GmailClient {
    client: Client,
    access_token: String,
    base_url: String,
}

impl GmailClient {
    /// Create new Gmail client
    pub fn new(access_token: String) -> Self {
        Self {
            client: Client::new(),
            access_token,
            base_url: "https://gmail.googleapis.com/gmail/v1".to_string(),
        }
    }
    
    /// Get unread emails
    pub async fn get_unread(&self, limit: u32) -> EmailResult<Vec<Email>> {
        let url = format!(
            "{}/users/me/messages?labelIds=UNREAD&maxResults={}",
            self.base_url, limit
        );
        
        let response = self.client
            .get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(EmailError::ApiError(format!("Gmail API error: {}", response.status())));
        }
        
        let json: serde_json::Value = response.json().await?;
        let empty = vec![];
        let messages = json["messages"].as_array().unwrap_or(&empty);
        
        let mut emails = Vec::new();
        for msg in messages.iter().take(limit as usize) {
            if let Some(id) = msg["id"].as_str() {
                if let Ok(email) = self.get_email(id).await {
                    emails.push(email);
                }
            }
        }
        
        Ok(emails)
    }
    
    /// Get emails from folder
    pub async fn get_folder(&self, folder: EmailFolder, limit: u32) -> EmailResult<Vec<Email>> {
        let label = folder_to_gmail_label(folder);
        let url = format!(
            "{}/users/me/messages?labelIds={}&maxResults={}",
            self.base_url, label, limit
        );
        
        let response = self.client
            .get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(EmailError::ApiError(format!("Gmail API error: {}", response.status())));
        }
        
        let json: serde_json::Value = response.json().await?;
        let empty = vec![];
        let messages = json["messages"].as_array().unwrap_or(&empty);
        
        let mut emails = Vec::new();
        for msg in messages.iter().take(limit as usize) {
            if let Some(id) = msg["id"].as_str() {
                if let Ok(email) = self.get_email(id).await {
                    emails.push(email);
                }
            }
        }
        
        Ok(emails)
    }
    
    /// Get single email by ID
    pub async fn get_email(&self, id: &str) -> EmailResult<Email> {
        let url = format!("{}/users/me/messages/{}", self.base_url, id);
        
        let response = self.client
            .get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(EmailError::NotFound(id.to_string()));
        }
        
        let json: serde_json::Value = response.json().await?;
        self.parse_gmail_message(&json)
    }
    
    /// Search emails
    pub async fn search(&self, query: &EmailQuery) -> EmailResult<Vec<Email>> {
        let mut q = String::new();
        
        if let Some(subject) = &query.subject {
            q.push_str(&format!("subject:{} ", subject));
        }
        if let Some(from) = &query.from {
            q.push_str(&format!("from:{} ", from));
        }
        if let Some(body) = &query.body {
            q.push_str(&format!("{} ", body));
        }
        if query.is_unread.unwrap_or(false) {
            q.push_str("is:unread ");
        }
        if query.has_attachments.unwrap_or(false) {
            q.push_str("has:attachment ");
        }
        
        let limit = query.limit.unwrap_or(20);
        let url = format!(
            "{}/users/me/messages?q={}&maxResults={}",
            self.base_url, 
            urlencoding::encode(&q),
            limit
        );
        
        let response = self.client
            .get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(EmailError::ApiError(format!("Gmail API error: {}", response.status())));
        }
        
        let json: serde_json::Value = response.json().await?;
        let empty = vec![];
        let messages = json["messages"].as_array().unwrap_or(&empty);
        
        let mut emails = Vec::new();
        for msg in messages.iter().take(limit as usize) {
            if let Some(id) = msg["id"].as_str() {
                if let Ok(email) = self.get_email(id).await {
                    emails.push(email);
                }
            }
        }
        
        Ok(emails)
    }
    
    /// Mark email as read
    pub async fn mark_read(&self, id: &str) -> EmailResult<()> {
        let url = format!(
            "{}/users/me/messages/{}/modify",
            self.base_url, id
        );
        
        let body = serde_json::json!({
            "removeLabelIds": ["UNREAD"]
        });
        
        let response = self.client
            .post(&url)
            .bearer_auth(&self.access_token)
            .json(&body)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(EmailError::ApiError(format!("Failed to mark as read: {}", response.status())));
        }
        
        Ok(())
    }
    
    /// Move email to folder
    pub async fn move_to(&self, id: &str, folder: EmailFolder) -> EmailResult<()> {
        let url = format!(
            "{}/users/me/messages/{}/modify",
            self.base_url, id
        );
        
        let label = folder_to_gmail_label(folder);
        let body = serde_json::json!({
            "addLabelIds": [label]
        });
        
        let response = self.client
            .post(&url)
            .bearer_auth(&self.access_token)
            .json(&body)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(EmailError::ApiError(format!("Failed to move email: {}", response.status())));
        }
        
        Ok(())
    }
    
    /// Parse Gmail message JSON
    fn parse_gmail_message(&self, json: &serde_json::Value) -> EmailResult<Email> {
        let id = json["id"].as_str().unwrap_or("unknown").to_string();
        let payload = &json["payload"];
        let empty_headers = vec![];
        let headers = payload["headers"].as_array().unwrap_or(&empty_headers);
        
        let mut subject = String::new();
        let mut from = EmailAddress::new("unknown@example.com");
        let mut to: Vec<EmailAddress> = vec![];
        let mut date = chrono::Utc::now();
        
        for header in headers {
            let name = header["name"].as_str().unwrap_or("");
            let value = header["value"].as_str().unwrap_or("");
            
            match name.to_lowercase().as_str() {
                "subject" => subject = value.to_string(),
                "from" => from = parse_email_address(value),
                "to" => to = value.split(',').map(|s| parse_email_address(s.trim())).collect(),
                "date" => {
                    if let Ok(parsed) = chrono::DateTime::parse_from_rfc2822(value) {
                        date = parsed.with_timezone(&chrono::Utc);
                    }
                }
                _ => {}
            }
        }
        
        // Get body
        let (body_text, body_html) = self.extract_body(payload);
        
        // Get labels/flags
        let labels = json["labelIds"].as_array();
        let flags = EmailFlags {
            seen: labels.map_or(true, |l| !l.iter().any(|x| x.as_str() == Some("UNREAD"))),
            flagged: labels.map_or(false, |l| l.iter().any(|x| x.as_str() == Some("STARRED"))),
            ..Default::default()
        };
        
        Ok(Email {
            id,
            subject,
            from,
            to,
            cc: vec![],
            date,
            body_text,
            body_html,
            folder: EmailFolder::Inbox,
            flags,
            attachments: vec![],
            thread_id: json["threadId"].as_str().map(|s| s.to_string()),
            priority: None,
        })
    }
    
    /// Extract body from payload
    fn extract_body(&self, payload: &serde_json::Value) -> (Option<String>, Option<String>) {
        let parts = payload["parts"].as_array();
        
        let mut text_body = None;
        let mut html_body = None;
        
        if let Some(parts) = parts {
            for part in parts {
                let mime_type = part["mimeType"].as_str().unwrap_or("");
                let data = part["body"]["data"].as_str().unwrap_or("");
                
                if let Ok(decoded) = base64_decode_urlsafe(data) {
                    match mime_type {
                        "text/plain" => text_body = Some(decoded),
                        "text/html" => html_body = Some(decoded),
                        _ => {}
                    }
                }
            }
        } else {
            // Simple message
            let data = payload["body"]["data"].as_str().unwrap_or("");
            if let Ok(decoded) = base64_decode_urlsafe(data) {
                text_body = Some(decoded);
            }
        }
        
        (text_body, html_body)
    }
}

// Helper functions
fn folder_to_gmail_label(folder: EmailFolder) -> &'static str {
    match folder {
        EmailFolder::Inbox => "INBOX",
        EmailFolder::Sent => "SENT",
        EmailFolder::Drafts => "DRAFT",
        EmailFolder::Spam => "SPAM",
        EmailFolder::Trash => "TRASH",
        EmailFolder::Important => "IMPORTANT",
        EmailFolder::Archive => "ARCHIVE",
        EmailFolder::Custom(_) => "INBOX",
    }
}

fn parse_email_address(s: &str) -> EmailAddress {
    // Parse "Name <email@domain.com>" or just "email@domain.com"
    if s.contains('<') && s.contains('>') {
        let start = s.find('<').unwrap_or(0) + 1;
        let end = s.find('>').unwrap_or(s.len());
        let addr = &s[start..end];
        let name = s[..start - 2].trim();
        EmailAddress::with_name(name, addr)
    } else {
        EmailAddress::new(s.trim())
    }
}

fn base64_decode_urlsafe(s: &str) -> EmailResult<String> {
    let s = s.replace('-', "+").replace('_', "/");
    let padded = if s.len() % 4 != 0 {
        format!("{}{}", s, "=".repeat(4 - s.len() % 4))
    } else {
        s
    };
    
    let bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &padded)
        .map_err(|e| EmailError::ParseError(e.to_string()))?;
    
    String::from_utf8(bytes).map_err(|e| EmailError::ParseError(e.to_string()))
}

// URL encoding helper
mod urlencoding {
    pub fn encode(s: &str) -> String {
        s.chars()
            .map(|c| match c {
                ' ' => "%20".to_string(),
                '+' => "%2B".to_string(),
                '&' => "%26".to_string(),
                '=' => "%3D".to_string(),
                _ => c.to_string(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_email_address() {
        let addr = parse_email_address("John Doe <john@example.com>");
        assert_eq!(addr.address, "john@example.com");
        assert_eq!(addr.name, Some("John Doe".to_string()));
        
        let simple = parse_email_address("test@example.com");
        assert_eq!(simple.address, "test@example.com");
    }
}
