//! Gmail connector - Google Gmail API

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    Connector, ConnectorError, ConnectorResult, ConnectorStatus, Credentials,
    CredentialType, Document, DocumentType, Email, SyncConfig, SyncResult,
    OAuthConfig, OAuthManager, OAuthToken,
};

/// Gmail connector
pub struct GmailConnector {
    token: Option<OAuthToken>,
    status: ConnectorStatus,
    last_sync: Option<DateTime<Utc>>,
    config: SyncConfig,
    client: reqwest::Client,
}

impl GmailConnector {
    pub fn new() -> Self {
        Self {
            token: None,
            status: ConnectorStatus::Disconnected,
            last_sync: None,
            config: SyncConfig::default(),
            client: reqwest::Client::new(),
        }
    }

    fn auth_header(&self) -> ConnectorResult<String> {
        let token = self.token.as_ref()
            .ok_or_else(|| ConnectorError::AuthFailed("Not authenticated".to_string()))?;
        
        if token.is_expired() {
            return Err(ConnectorError::TokenExpired);
        }
        
        Ok(format!("Bearer {}", token.access_token))
    }

    /// List messages
    pub async fn list_messages(&self, max_results: u32) -> ConnectorResult<Vec<GmailMessageRef>> {
        let auth = self.auth_header()?;
        
        let url = format!(
            "https://gmail.googleapis.com/gmail/v1/users/me/messages?maxResults={}",
            max_results
        );

        let response = self.client
            .get(&url)
            .header("Authorization", &auth)
            .send()
            .await?;

        if response.status().is_success() {
            let list: GmailMessageList = response.json().await?;
            Ok(list.messages.unwrap_or_default())
        } else {
            Err(ConnectorError::ApiError(format!("Gmail API error: {}", response.status())))
        }
    }

    /// Get a single message
    pub async fn get_message(&self, id: &str) -> ConnectorResult<Email> {
        let auth = self.auth_header()?;
        
        let url = format!(
            "https://gmail.googleapis.com/gmail/v1/users/me/messages/{}?format=full",
            id
        );

        let response = self.client
            .get(&url)
            .header("Authorization", &auth)
            .send()
            .await?;

        if response.status().is_success() {
            let msg: GmailMessage = response.json().await?;
            Ok(self.parse_message(&msg))
        } else {
            Err(ConnectorError::ApiError(format!("Gmail API error: {}", response.status())))
        }
    }

    /// Search messages
    pub async fn search_messages(&self, query: &str, max_results: u32) -> ConnectorResult<Vec<GmailMessageRef>> {
        let auth = self.auth_header()?;
        
        let url = format!(
            "https://gmail.googleapis.com/gmail/v1/users/me/messages?q={}&maxResults={}",
            urlencoding::encode(query),
            max_results
        );

        let response = self.client
            .get(&url)
            .header("Authorization", &auth)
            .send()
            .await?;

        if response.status().is_success() {
            let list: GmailMessageList = response.json().await?;
            Ok(list.messages.unwrap_or_default())
        } else {
            Err(ConnectorError::ApiError(format!("Gmail API error: {}", response.status())))
        }
    }

    fn parse_message(&self, msg: &GmailMessage) -> Email {
        let headers = msg.payload.headers.iter()
            .map(|h| (h.name.to_lowercase(), h.value.clone()))
            .collect::<std::collections::HashMap<String, String>>();

        Email {
            id: msg.id.clone(),
            thread_id: Some(msg.thread_id.clone()),
            subject: headers.get("subject").cloned().unwrap_or_default(),
            from: self.parse_email_address(headers.get("from").unwrap_or(&String::new())),
            to: headers.get("to").map(|s| vec![self.parse_email_address(s)]).unwrap_or_default(),
            cc: Vec::new(),
            body_text: self.extract_body(&msg.payload, "text/plain"),
            body_html: self.extract_body(&msg.payload, "text/html").into(),
            date: chrono::DateTime::from_timestamp_millis(msg.internal_date as i64)
                .unwrap_or(Utc::now()),
            labels: msg.label_ids.clone(),
            is_read: !msg.label_ids.contains(&"UNREAD".to_string()),
            is_starred: msg.label_ids.contains(&"STARRED".to_string()),
            attachments: Vec::new(),
        }
    }

    fn parse_email_address(&self, s: &str) -> crate::EmailAddress {
        // Parse "Name <email@domain.com>" or just "email@domain.com"
        let s = s.trim();
        if let Some(start) = s.find('<') {
            if let Some(end) = s.find('>') {
                return crate::EmailAddress {
                    name: Some(s[..start].trim().to_string()),
                    email: s[start+1..end].to_string(),
                };
            }
        }
        crate::EmailAddress {
            name: None,
            email: s.to_string(),
        }
    }

    fn extract_body(&self, payload: &GmailPayload, mime_type: &str) -> String {
        if payload.mime_type == mime_type {
            if let Some(ref data) = payload.body.data {
                return self.decode_base64(data);
            }
        }
        
        if let Some(ref parts) = payload.parts {
            for part in parts {
                let body = self.extract_body(part, mime_type);
                if !body.is_empty() {
                    return body;
                }
            }
        }
        
        String::new()
    }

    fn decode_base64(&self, s: &str) -> String {
        // Gmail uses URL-safe base64 without padding
        let s = s.replace('-', "+").replace('_', "/");
        let padded = match s.len() % 4 {
            2 => format!("{}==", s),
            3 => format!("{}=", s),
            _ => s,
        };
        
        use base64::Engine;
        base64::engine::general_purpose::STANDARD
            .decode(&padded)
            .ok()
            .and_then(|bytes| String::from_utf8(bytes).ok())
            .unwrap_or_default()
    }
}

impl Default for GmailConnector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Connector for GmailConnector {
    fn connector_id(&self) -> &str { "gmail" }
    fn connector_name(&self) -> &str { "Gmail" }
    fn category(&self) -> &str { "email" }
    fn status(&self) -> ConnectorStatus { self.status.clone() }
    fn required_credentials(&self) -> Vec<String> { vec!["oauth2".to_string()] }

    async fn connect(&mut self, credentials: Credentials) -> ConnectorResult<()> {
        self.status = ConnectorStatus::Connecting;
        
        match credentials.cred_type {
            CredentialType::OAuth2 => {
                if let Some(access) = credentials.access_token {
                    let mut token = OAuthToken::new(&access, credentials.expires_at
                        .map(|e| (e - Utc::now()).num_seconds() as u64)
                        .unwrap_or(3600));
                    
                    if let Some(refresh) = credentials.refresh_token {
                        token = token.with_refresh(&refresh);
                    }
                    
                    self.token = Some(token);
                    self.status = ConnectorStatus::Connected;
                    Ok(())
                } else {
                    self.status = ConnectorStatus::Error("Missing access token".to_string());
                    Err(ConnectorError::AuthFailed("OAuth2 access token required".to_string()))
                }
            }
            _ => {
                self.status = ConnectorStatus::Error("Invalid credential type".to_string());
                Err(ConnectorError::AuthFailed("OAuth2 required for Gmail".to_string()))
            }
        }
    }

    async fn disconnect(&mut self) -> ConnectorResult<()> {
        self.token = None;
        self.status = ConnectorStatus::Disconnected;
        Ok(())
    }

    async fn test_connection(&self) -> ConnectorResult<bool> {
        let auth = self.auth_header()?;
        
        let response = self.client
            .get("https://gmail.googleapis.com/gmail/v1/users/me/profile")
            .header("Authorization", &auth)
            .send()
            .await?;

        Ok(response.status().is_success())
    }

    async fn sync(&self, since: Option<DateTime<Utc>>, config: &SyncConfig) -> ConnectorResult<SyncResult> {
        let mut result = SyncResult::new(self.connector_id());
        
        let messages = self.list_messages(config.max_items as u32).await?;
        result.items_synced = messages.len();
        result.items_new = messages.len(); // TODO: track actually new
        
        Ok(result)
    }

    async fn fetch(&self, query: &str, limit: usize) -> ConnectorResult<Vec<Document>> {
        let refs = if query.is_empty() {
            self.list_messages(limit as u32).await?
        } else {
            self.search_messages(query, limit as u32).await?
        };
        
        let mut docs = Vec::new();
        for r in refs.iter().take(limit) {
            match self.get_message(&r.id).await {
                Ok(email) => {
                    docs.push(Document::new("gmail", DocumentType::Email, &email.id, &email.subject)
                        .with_content(&email.body_text)
                        .with_author(&email.from.email)
                        .with_tag(if email.is_read { "read" } else { "unread" }));
                }
                Err(e) => log::warn!("Failed to fetch email {}: {}", r.id, e),
            }
        }
        
        Ok(docs)
    }

    async fn get_document(&self, id: &str) -> ConnectorResult<Option<Document>> {
        let email = self.get_message(id).await?;
        Ok(Some(Document::new("gmail", DocumentType::Email, &email.id, &email.subject)
            .with_content(&email.body_text)
            .with_author(&email.from.email)))
    }

    async fn search(&self, query: &str, limit: usize) -> ConnectorResult<Vec<Document>> {
        self.fetch(query, limit).await
    }

    fn last_sync(&self) -> Option<DateTime<Utc>> { self.last_sync }
    fn set_config(&mut self, config: SyncConfig) { self.config = config; }
    fn config(&self) -> &SyncConfig { &self.config }
}

// Gmail API types
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GmailMessageList {
    messages: Option<Vec<GmailMessageRef>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GmailMessageRef {
    id: String,
    thread_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GmailMessage {
    id: String,
    thread_id: String,
    label_ids: Vec<String>,
    snippet: String,
    #[serde(default)]
    internal_date: i64,
    payload: GmailPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GmailPayload {
    #[serde(rename = "mimeType")]
    mime_type: String,
    headers: Vec<GmailHeader>,
    body: GmailBody,
    parts: Option<Vec<GmailPayload>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GmailHeader {
    name: String,
    value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GmailBody {
    data: Option<String>,
}

mod urlencoding {
    pub fn encode(s: &str) -> String {
        url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
    }
}
