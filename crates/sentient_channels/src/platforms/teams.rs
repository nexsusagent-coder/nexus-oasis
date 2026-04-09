//! Microsoft Teams Channel Integration
//!
//! Supports Microsoft Teams bot API with:
//! - Adaptive Cards
//! - Task Modules
//! - Messaging Extensions
//! - Conversation updates

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use reqwest::Client;

/// Teams channel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsConfig {
    /// Microsoft App ID
    pub app_id: String,

    /// Microsoft App Password (client secret)
    pub app_password: String,

    /// Tenant ID (for single-tenant apps)
    pub tenant_id: Option<String>,

    /// Service URL for API calls
    pub service_url: String,
}

impl Default for TeamsConfig {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            app_password: String::new(),
            tenant_id: None,
            service_url: "https://smba.trafficmanager.net/amer/".to_string(),
        }
    }
}

/// Teams message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsMessage {
    /// Message type
    #[serde(rename = "type")]
    pub message_type: String,

    /// Message ID
    pub id: Option<String>,

    /// Timestamp
    pub timestamp: Option<String>,

    /// From (sender)
    pub from: Option<TeamsUser>,

    /// Conversation
    pub conversation: Option<TeamsConversation>,

    /// Text content
    pub text: Option<String>,

    /// Attachments (Adaptive Cards)
    pub attachments: Vec<TeamsAttachment>,
}

impl Default for TeamsMessage {
    fn default() -> Self {
        Self {
            message_type: "message".to_string(),
            id: None,
            timestamp: None,
            from: None,
            conversation: None,
            text: None,
            attachments: vec![],
        }
    }
}

/// Teams user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsUser {
    pub id: String,
    pub name: Option<String>,
    #[serde(rename = "aadObjectId")]
    pub aad_object_id: Option<String>,
}

/// Teams conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsConversation {
    pub id: String,
    pub name: Option<String>,
    #[serde(rename = "conversationType")]
    pub conversation_type: Option<String>,
    pub tenant_id: Option<String>,
}

/// Teams attachment (Adaptive Card)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsAttachment {
    #[serde(rename = "contentType")]
    pub content_type: String,

    #[serde(rename = "contentUrl")]
    pub content_url: Option<String>,

    pub content: Option<serde_json::Value>,
}

impl TeamsAttachment {
    /// Create an Adaptive Card attachment
    pub fn adaptive_card(card: serde_json::Value) -> Self {
        Self {
            content_type: "application/vnd.microsoft.card.adaptive".to_string(),
            content_url: None,
            content: Some(card),
        }
    }
}

/// Adaptive Card builder
pub struct AdaptiveCardBuilder {
    card: serde_json::Value,
}

impl AdaptiveCardBuilder {
    /// Create a new Adaptive Card
    pub fn new() -> Self {
        Self {
            card: serde_json::json!({
                "$schema": "http://adaptivecards.io/schemas/adaptive-card.json",
                "type": "AdaptiveCard",
                "version": "1.4",
                "body": []
            }),
        }
    }

    /// Add text block
    pub fn text(mut self, text: &str, size: Option<&str>) -> Self {
        let size = size.unwrap_or("default");
        if let Some(body) = self.card.get_mut("body").and_then(|b| b.as_array_mut()) {
            body.push(serde_json::json!({
                "type": "TextBlock",
                "text": text,
                "size": size
            }));
        }
        self
    }

    /// Add image
    pub fn image(mut self, url: &str, alt_text: Option<&str>) -> Self {
        if let Some(body) = self.card.get_mut("body").and_then(|b| b.as_array_mut()) {
            body.push(serde_json::json!({
                "type": "Image",
                "url": url,
                "altText": alt_text.unwrap_or("")
            }));
        }
        self
    }

    /// Add action (button)
    pub fn action(mut self, title: &str, action_type: &str, data: serde_json::Value) -> Self {
        if let Some(actions) = self.card.get_mut("actions").and_then(|a| a.as_array_mut()) {
            actions.push(serde_json::json!({
                "type": action_type,
                "title": title,
                "data": data
            }));
        } else {
            self.card["actions"] = serde_json::json!([{
                "type": action_type,
                "title": title,
                "data": data
            }]);
        }
        self
    }

    /// Add input field
    pub fn input_text(mut self, id: &str, placeholder: &str) -> Self {
        if let Some(body) = self.card.get_mut("body").and_then(|b| b.as_array_mut()) {
            body.push(serde_json::json!({
                "type": "Input.Text",
                "id": id,
                "placeholder": placeholder
            }));
        }
        self
    }

    /// Add choice set (dropdown)
    pub fn choice_set(mut self, id: &str, choices: Vec<(&str, &str)>, placeholder: &str) -> Self {
        if let Some(body) = self.card.get_mut("body").and_then(|b| b.as_array_mut()) {
            body.push(serde_json::json!({
                "type": "Input.ChoiceSet",
                "id": id,
                "choices": choices.iter().map(|(title, value)| {
                    serde_json::json!({"title": title, "value": value})
                }).collect::<Vec<_>>(),
                "placeholder": placeholder
            }));
        }
        self
    }

    /// Build the card
    pub fn build(self) -> serde_json::Value {
        self.card
    }
}

impl Default for AdaptiveCardBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Teams channel implementation
pub struct TeamsChannel {
    config: TeamsConfig,
    client: Client,
    access_token: Option<String>,
}

impl TeamsChannel {
    /// Create a new Teams channel
    pub fn new(config: TeamsConfig) -> Self {
        Self {
            config,
            client: Client::new(),
            access_token: None,
        }
    }

    /// Get access token from Microsoft
    pub async fn get_access_token(&mut self) -> Result<String, TeamsError> {
        let token_url = if let Some(ref tenant_id) = self.config.tenant_id {
            format!(
                "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
                tenant_id
            )
        } else {
            "https://login.microsoftonline.com/common/oauth2/v2.0/token".to_string()
        };

        let response = self.client
            .post(&token_url)
            .form(&[
                ("grant_type", "client_credentials"),
                ("client_id", &self.config.app_id),
                ("client_secret", &self.config.app_password),
                ("scope", "https://api.botframework.com/.default"),
            ])
            .send()
            .await
            .map_err(|e| TeamsError::RequestError(e.to_string()))?;

        let token: AccessTokenResponse = response
            .json()
            .await
            .map_err(|e| TeamsError::ParseError(e.to_string()))?;

        self.access_token = Some(token.access_token.clone());
        Ok(token.access_token)
    }

    /// Send message to a conversation
    pub async fn send_message(
        &mut self,
        conversation_id: &str,
        text: &str,
    ) -> Result<String, TeamsError> {
        let token = if let Some(ref token) = self.access_token {
            token.clone()
        } else {
            self.get_access_token().await?
        };

        let url = format!(
            "{}v3/conversations/{}/activities",
            self.config.service_url, conversation_id
        );

        let message = TeamsMessage {
            text: Some(text.to_string()),
            ..Default::default()
        };

        let response = self.client
            .post(&url)
            .bearer_auth(&token)
            .json(&message)
            .send()
            .await
            .map_err(|e| TeamsError::RequestError(e.to_string()))?;

        let result: ActivityResponse = response
            .json()
            .await
            .map_err(|e| TeamsError::ParseError(e.to_string()))?;

        Ok(result.id)
    }

    /// Send Adaptive Card
    pub async fn send_card(
        &mut self,
        conversation_id: &str,
        card: serde_json::Value,
    ) -> Result<String, TeamsError> {
        let token = if let Some(ref token) = self.access_token {
            token.clone()
        } else {
            self.get_access_token().await?
        };

        let url = format!(
            "{}v3/conversations/{}/activities",
            self.config.service_url, conversation_id
        );

        let message = TeamsMessage {
            attachments: vec![TeamsAttachment::adaptive_card(card)],
            ..Default::default()
        };

        let response = self.client
            .post(&url)
            .bearer_auth(&token)
            .json(&message)
            .send()
            .await
            .map_err(|e| TeamsError::RequestError(e.to_string()))?;

        let result: ActivityResponse = response
            .json()
            .await
            .map_err(|e| TeamsError::ParseError(e.to_string()))?;

        Ok(result.id)
    }

    /// Reply to a message
    pub async fn reply(
        &mut self,
        conversation_id: &str,
        reply_to_id: &str,
        text: &str,
    ) -> Result<String, TeamsError> {
        let token = if let Some(ref token) = self.access_token {
            token.clone()
        } else {
            self.get_access_token().await?
        };

        let url = format!(
            "{}v3/conversations/{}/activities/{}",
            self.config.service_url, conversation_id, reply_to_id
        );

        let message = TeamsMessage {
            text: Some(text.to_string()),
            ..Default::default()
        };

        let response = self.client
            .post(&url)
            .bearer_auth(&token)
            .json(&message)
            .send()
            .await
            .map_err(|e| TeamsError::RequestError(e.to_string()))?;

        let result: ActivityResponse = response
            .json()
            .await
            .map_err(|e| TeamsError::ParseError(e.to_string()))?;

        Ok(result.id)
    }
}

#[derive(Debug, Deserialize)]
struct AccessTokenResponse {
    access_token: String,
    expires_in: i64,
}

#[derive(Debug, Deserialize)]
struct ActivityResponse {
    id: String,
}

/// Teams error types
#[derive(Debug, thiserror::Error)]
pub enum TeamsError {
    #[error("Request error: {0}")]
    RequestError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_card_builder() {
        let card = AdaptiveCardBuilder::new()
            .text("Hello, Teams!", Some("large"))
            .input_text("user_input", "Type something...")
            .action("Submit", "Action.Submit", serde_json::json!({"action": "submit"}))
            .build();

        assert_eq!(card["type"], "AdaptiveCard");
    }
}
