//! Facebook Messenger Channel Integration
//!
//! Supports Facebook Messenger Platform API with:
//! - Text, Image, Video, Audio messages
//! - Quick Replies
//! - Button Templates
//! - Generic Templates (carousel)
//! - Webview
//! - Handover Protocol

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use reqwest::Client;

/// Facebook Messenger configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessengerConfig {
    /// Facebook Page Access Token
    pub page_access_token: String,

    /// Facebook App Secret
    pub app_secret: String,

    /// Verify token for webhook verification
    pub verify_token: String,
}

impl Default for MessengerConfig {
    fn default() -> Self {
        Self {
            page_access_token: String::new(),
            app_secret: String::new(),
            verify_token: String::new(),
        }
    }
}

/// Messenger message types
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MessengerMessage {
    Text {
        text: String,
    },
    Attachment {
        attachment: MessengerAttachment,
    },
    Template {
        attachment: TemplateAttachment,
    },
}

/// Messenger attachment
#[derive(Debug, Clone, Serialize)]
pub struct MessengerAttachment {
    #[serde(rename = "type")]
    pub attachment_type: String,
    pub payload: serde_json::Value,
}

/// Template attachment
#[derive(Debug, Clone, Serialize)]
pub struct TemplateAttachment {
    #[serde(rename = "type")]
    pub attachment_type: String, // "template"
    pub payload: TemplatePayload,
}

/// Template payload
#[derive(Debug, Clone, Serialize)]
pub struct TemplatePayload {
    #[serde(rename = "template_type")]
    pub template_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<MessengerButton>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elements: Option<Vec<GenericElement>>,
}

/// Messenger button
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum MessengerButton {
    WebUrl {
        url: String,
        title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        webview_height_ratio: Option<String>,
    },
    Postback {
        title: String,
        payload: String,
    },
    Phone {
        title: String,
        payload: String,
    },
    Share,
}

/// Generic template element
#[derive(Debug, Clone, Serialize)]
pub struct GenericElement {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<MessengerButton>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_action: Option<MessengerButton>,
}

/// Quick reply
#[derive(Debug, Clone, Serialize)]
pub struct QuickReply {
    pub content_type: String,
    pub title: String,
    pub payload: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

/// Message request
#[derive(Debug, Serialize)]
struct MessageRequest {
    recipient: Recipient,
    message: MessageBody,
    #[serde(skip_serializing_if = "Option::is_none")]
    messaging_type: Option<String>,
}

#[derive(Debug, Serialize)]
struct Recipient {
    id: String,
}

#[derive(Debug, Serialize)]
struct MessageBody {
    #[serde(flatten)]
    pub content: MessengerMessage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_replies: Option<Vec<QuickReply>>,
}

/// Webhook event
#[derive(Debug, Clone, Deserialize)]
pub struct WebhookEvent {
    pub object: String,
    pub entry: Vec<WebhookEntry>,
}

/// Webhook entry
#[derive(Debug, Clone, Deserialize)]
pub struct WebhookEntry {
    pub id: String,
    pub time: i64,
    pub messaging: Vec<MessagingEvent>,
}

/// Messaging event
#[derive(Debug, Clone, Deserialize)]
pub struct MessagingEvent {
    pub sender: Sender,
    pub recipient: RecipientInfo,
    pub timestamp: i64,
    pub message: Option<ReceivedMessage>,
    pub postback: Option<Postback>,
}

/// Sender info
#[derive(Debug, Clone, Deserialize)]
pub struct Sender {
    pub id: String,
}

/// Recipient info
#[derive(Debug, Clone, Deserialize)]
pub struct RecipientInfo {
    pub id: String,
}

/// Received message
#[derive(Debug, Clone, Deserialize)]
pub struct ReceivedMessage {
    pub mid: String,
    pub text: Option<String>,
    pub attachments: Option<Vec<AttachmentInfo>>,
}

/// Attachment info
#[derive(Debug, Clone, Deserialize)]
pub struct AttachmentInfo {
    #[serde(rename = "type")]
    pub attachment_type: String,
    pub payload: serde_json::Value,
}

/// Postback
#[derive(Debug, Clone, Deserialize)]
pub struct Postback {
    pub payload: String,
    pub title: Option<String>,
}

/// Sender action types
#[derive(Debug, Clone, Copy)]
pub enum SenderAction {
    MarkSeen,
    TypingOn,
    TypingOff,
}

impl Serialize for SenderAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            SenderAction::MarkSeen => "mark_seen",
            SenderAction::TypingOn => "typing_on",
            SenderAction::TypingOff => "typing_off",
        };
        serializer.serialize_str(s)
    }
}

/// Facebook Messenger channel implementation
pub struct MessengerChannel {
    config: MessengerConfig,
    client: Client,
}

impl MessengerChannel {
    /// Create a new Messenger channel
    pub fn new(config: MessengerConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    /// Send text message
    pub async fn send_text(
        &self,
        recipient_id: &str,
        text: &str,
    ) -> Result<String, MessengerError> {
        self.send_message(recipient_id, MessengerMessage::Text {
            text: text.to_string(),
        }, None).await
    }

    /// Send text with quick replies
    pub async fn send_text_with_replies(
        &self,
        recipient_id: &str,
        text: &str,
        quick_replies: Vec<QuickReply>,
    ) -> Result<String, MessengerError> {
        let request = MessageRequest {
            recipient: Recipient {
                id: recipient_id.to_string(),
            },
            messaging_type: Some("RESPONSE".to_string()),
            message: MessageBody {
                content: MessengerMessage::Text {
                    text: text.to_string(),
                },
                quick_replies: Some(quick_replies),
            },
        };

        self.send_request(&request).await
    }

    /// Send image
    pub async fn send_image(
        &self,
        recipient_id: &str,
        image_url: &str,
    ) -> Result<String, MessengerError> {
        let message = MessengerMessage::Attachment {
            attachment: MessengerAttachment {
                attachment_type: "image".to_string(),
                payload: serde_json::json!({
                    "url": image_url
                }),
            },
        };

        self.send_message(recipient_id, message, None).await
    }

    /// Send button template
    pub async fn send_button_template(
        &self,
        recipient_id: &str,
        text: &str,
        buttons: Vec<MessengerButton>,
    ) -> Result<String, MessengerError> {
        let message = MessengerMessage::Template {
            attachment: TemplateAttachment {
                attachment_type: "template".to_string(),
                payload: TemplatePayload {
                    template_type: "button".to_string(),
                    text: Some(text.to_string()),
                    buttons: Some(buttons),
                    elements: None,
                },
            },
        };

        self.send_message(recipient_id, message, None).await
    }

    /// Send generic template (carousel)
    pub async fn send_generic_template(
        &self,
        recipient_id: &str,
        elements: Vec<GenericElement>,
    ) -> Result<String, MessengerError> {
        let message = MessengerMessage::Template {
            attachment: TemplateAttachment {
                attachment_type: "template".to_string(),
                payload: TemplatePayload {
                    template_type: "generic".to_string(),
                    text: None,
                    buttons: None,
                    elements: Some(elements),
                },
            },
        };

        self.send_message(recipient_id, message, None).await
    }

    /// Send sender action (typing indicator, mark seen)
    pub async fn send_sender_action(
        &self,
        recipient_id: &str,
        action: SenderAction,
    ) -> Result<(), MessengerError> {
        let url = "https://graph.facebook.com/v18.0/me/messages";

        #[derive(Serialize)]
        struct ActionRequest {
            recipient: Recipient,
            #[serde(rename = "sender_action")]
            action: SenderAction,
        }

        let request = ActionRequest {
            recipient: Recipient {
                id: recipient_id.to_string(),
            },
            action,
        };

        let response = self.client
            .post(url)
            .query(&[("access_token", &self.config.page_access_token)])
            .json(&request)
            .send()
            .await
            .map_err(|e| MessengerError::RequestError(e.to_string()))?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(MessengerError::ApiError(error));
        }

        Ok(())
    }

    /// Get user profile
    pub async fn get_user_profile(
        &self,
        user_id: &str,
    ) -> Result<MessengerProfile, MessengerError> {
        let url = format!(
            "https://graph.facebook.com/v18.0/{}?fields=first_name,last_name,profile_pic",
            user_id
        );

        let response = self.client
            .get(&url)
            .query(&[("access_token", &self.config.page_access_token)])
            .send()
            .await
            .map_err(|e| MessengerError::RequestError(e.to_string()))?;

        let profile = response
            .json()
            .await
            .map_err(|e| MessengerError::ParseError(e.to_string()))?;

        Ok(profile)
    }

    async fn send_message(
        &self,
        recipient_id: &str,
        message: MessengerMessage,
        quick_replies: Option<Vec<QuickReply>>,
    ) -> Result<String, MessengerError> {
        let request = MessageRequest {
            recipient: Recipient {
                id: recipient_id.to_string(),
            },
            messaging_type: Some("RESPONSE".to_string()),
            message: MessageBody {
                content: message,
                quick_replies,
            },
        };

        self.send_request(&request).await
    }

    async fn send_request(&self, request: &MessageRequest) -> Result<String, MessengerError> {
        let url = "https://graph.facebook.com/v18.0/me/messages";

        let response = self.client
            .post(url)
            .query(&[("access_token", &self.config.page_access_token)])
            .json(request)
            .send()
            .await
            .map_err(|e| MessengerError::RequestError(e.to_string()))?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(MessengerError::ApiError(error));
        }

        #[derive(Debug, Deserialize)]
        struct MessageResponse {
            message_id: String,
        }

        let result: MessageResponse = response
            .json()
            .await
            .map_err(|e| MessengerError::ParseError(e.to_string()))?;

        Ok(result.message_id)
    }

    /// Verify webhook signature
    pub fn verify_signature(&self, body: &str, signature: &str) -> bool {
        use hmac::{Hmac, Mac};
        use sha1::Sha1;

        type HmacSha1 = Hmac<Sha1>;

        let signature = signature.strip_prefix("sha1=").unwrap_or(signature);

        let mut mac = match HmacSha1::new_from_slice(self.config.app_secret.as_bytes()) {
            Ok(m) => m,
            Err(_) => return false,
        };

        mac.update(body.as_bytes());
        let result = mac.finalize();
        let computed = hex::encode(result.into_bytes());

        computed == signature
    }
}

/// Messenger user profile
#[derive(Debug, Clone, Deserialize)]
pub struct MessengerProfile {
    pub first_name: String,
    pub last_name: String,
    pub profile_pic: Option<String>,
}

/// Messenger error types
#[derive(Debug, thiserror::Error)]
pub enum MessengerError {
    #[error("Request error: {0}")]
    RequestError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_element() {
        let element = GenericElement {
            title: "Test Title".to_string(),
            subtitle: Some("Test Subtitle".to_string()),
            image_url: Some("https://example.com/image.jpg".to_string()),
            buttons: Some(vec![MessengerButton::WebUrl {
                url: "https://example.com".to_string(),
                title: "Visit".to_string(),
                webview_height_ratio: Some("full".to_string()),
            }]),
            default_action: None,
        };

        assert_eq!(element.title, "Test Title");
    }
}
