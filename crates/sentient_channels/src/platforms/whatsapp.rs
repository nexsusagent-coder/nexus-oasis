//! ─── WhatsApp Business API Integration ───
//!
//! Supports:
//! - WhatsApp Business API (Cloud API)
//! - Webhook for incoming messages
//! - Template messages
//! - Media messages

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::{Channel, ChannelError, ChannelMessage, MessageContent, ChannelType};

/// WhatsApp Business API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppConfig {
    /// Phone Number ID (from Meta Business Suite)
    pub phone_number_id: String,
    
    /// WhatsApp Business Account ID
    pub business_account_id: String,
    
    /// Permanent access token
    pub access_token: String,
    
    /// Webhook verify token
    pub verify_token: String,
    
    /// App secret (for signature verification)
    pub app_secret: String,
}

/// WhatsApp channel
pub struct WhatsAppChannel {
    config: WhatsAppConfig,
    client: Client,
    base_url: String,
}

impl WhatsAppChannel {
    pub fn new(config: WhatsAppConfig) -> Self {
        Self {
            config,
            client: Client::new(),
            base_url: "https://graph.facebook.com/v18.0".into(),
        }
    }
    
    /// Send text message
    pub async fn send_text(&self, to: &str, text: &str) -> Result<String, ChannelError> {
        let url = format!("{}/{}/messages", self.base_url, self.config.phone_number_id);
        
        let body = serde_json::json!({
            "messaging_product": "whatsapp",
            "recipient_type": "individual",
            "to": to,
            "type": "text",
            "text": {
                "preview_url": false,
                "body": text
            }
        });
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.access_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(ChannelError::ApiError(error));
        }
        
        let result: WhatsAppResponse = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        Ok(result.messages.first().map(|m| m.id.clone()).unwrap_or_default())
    }
    
    /// Send image message
    pub async fn send_image(&self, to: &str, image_url: &str, caption: Option<&str>) -> Result<String, ChannelError> {
        let url = format!("{}/{}/messages", self.base_url, self.config.phone_number_id);
        
        let mut body = serde_json::json!({
            "messaging_product": "whatsapp",
            "recipient_type": "individual",
            "to": to,
            "type": "image",
            "image": {
                "link": image_url
            }
        });
        
        if let Some(cap) = caption {
            body["image"]["caption"] = serde_json::json!(cap);
        }
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.access_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let result: WhatsAppResponse = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        Ok(result.messages.first().map(|m| m.id.clone()).unwrap_or_default())
    }
    
    /// Send template message
    pub async fn send_template(&self, to: &str, template_name: &str, language: &str, components: Option<Vec<TemplateComponent>>) -> Result<String, ChannelError> {
        let url = format!("{}/{}/messages", self.base_url, self.config.phone_number_id);
        
        let mut body = serde_json::json!({
            "messaging_product": "whatsapp",
            "recipient_type": "individual",
            "to": to,
            "type": "template",
            "template": {
                "name": template_name,
                "language": {
                    "code": language
                }
            }
        });
        
        if let Some(comps) = components {
            body["template"]["components"] = serde_json::to_value(comps)
                .map_err(|e| ChannelError::Parse(e.to_string()))?;
        }
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.access_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let result: WhatsAppResponse = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        Ok(result.messages.first().map(|m| m.id.clone()).unwrap_or_default())
    }
    
    /// Mark message as read
    pub async fn mark_as_read(&self, message_id: &str) -> Result<(), ChannelError> {
        let url = format!("{}/{}/messages", self.base_url, self.config.phone_number_id);
        
        let body = serde_json::json!({
            "messaging_product": "whatsapp",
            "status": "read",
            "message_id": message_id
        });
        
        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.access_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        Ok(())
    }
    
    /// Parse webhook payload
    pub fn parse_webhook(&self, payload: &str) -> Result<Vec<WhatsAppWebhookMessage>, ChannelError> {
        let webhook: WhatsAppWebhook = serde_json::from_str(payload)
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        let mut messages = Vec::new();
        
        for entry in webhook.entry {
            for change in entry.changes {
                if let Some(value) = change.value {
                    for msg in value.messages {
                        messages.push(msg);
                    }
                }
            }
        }
        
        Ok(messages)
    }
    
    /// Verify webhook signature
    pub fn verify_signature(&self, payload: &[u8], signature: &str) -> bool {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        
        type HmacSha256 = Hmac<Sha256>;
        
        let mut mac = match HmacSha256::new_from_slice(self.config.app_secret.as_bytes()) {
            Ok(m) => m,
            Err(_) => return false,
        };
        
        mac.update(payload);
        let result = mac.finalize();
        let computed = format!("sha256={}", hex::encode(result.into_bytes()));
        
        computed == signature
    }
}

#[async_trait]
impl Channel for WhatsAppChannel {
    fn channel_type(&self) -> ChannelType {
        ChannelType::WhatsApp
    }
    
    async fn send(&self, message: ChannelMessage) -> Result<String, ChannelError> {
        match message.content {
            MessageContent::Text(text) => self.send_text(&message.recipient, &text).await,
            MessageContent::Image { url, caption } => self.send_image(&message.recipient, &url, caption.as_deref()).await,
            _ => Err(ChannelError::UnsupportedContentType),
        }
    }
    
    async fn receive(&self) -> Result<Vec<ChannelMessage>, ChannelError> {
        // WhatsApp uses webhooks, so this would be handled by the webhook endpoint
        Ok(Vec::new())
    }
    
    fn is_connected(&self) -> bool {
        true // WhatsApp is always "connected" via API
    }
}

/// WhatsApp API response
#[derive(Debug, Deserialize)]
struct WhatsAppResponse {
    messages: Vec<WhatsAppMessageId>,
}

#[derive(Debug, Deserialize)]
struct WhatsAppMessageId {
    id: String,
}

/// Template component
#[derive(Debug, Serialize)]
pub struct TemplateComponent {
    #[serde(rename = "type")]
    pub component_type: String,
    pub parameters: Vec<TemplateParameter>,
}

#[derive(Debug, Serialize)]
pub struct TemplateParameter {
    #[serde(rename = "type")]
    pub param_type: String,
    pub text: Option<String>,
}

/// Webhook payload
#[derive(Debug, Deserialize)]
pub struct WhatsAppWebhook {
    entry: Vec<WebhookEntry>,
}

#[derive(Debug, Deserialize)]
struct WebhookEntry {
    id: String,
    changes: Vec<WebhookChange>,
}

#[derive(Debug, Deserialize)]
struct WebhookChange {
    value: Option<WebhookValue>,
}

#[derive(Debug, Deserialize)]
struct WebhookValue {
    messages: Vec<WhatsAppWebhookMessage>,
}

/// Incoming WhatsApp message
#[derive(Debug, Deserialize)]
pub struct WhatsAppWebhookMessage {
    pub from: String,
    pub id: String,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub text: Option<TextBody>,
    pub image: Option<MediaBody>,
    pub audio: Option<MediaBody>,
    pub video: Option<MediaBody>,
    pub document: Option<MediaBody>,
    pub timestamp: String,
}

#[derive(Debug, Deserialize)]
pub struct TextBody {
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct MediaBody {
    pub id: String,
    pub mime_type: Option<String>,
    pub caption: Option<String>,
}

impl From<WhatsAppWebhookMessage> for ChannelMessage {
    fn from(msg: WhatsAppWebhookMessage) -> Self {
        let content = match msg.msg_type.as_str() {
            "text" => msg.text.map(|t| MessageContent::Text(t.body)).unwrap_or(MessageContent::Text(String::new())),
            "image" => msg.image.map(|i| MessageContent::Image {
                url: i.id,
                caption: i.caption,
            }).unwrap_or(MessageContent::Unknown),
            "audio" => msg.audio.map(|a| MessageContent::Audio { url: a.id }).unwrap_or(MessageContent::Unknown),
            _ => MessageContent::Unknown,
        };
        
        Self {
            id: msg.id,
            channel: ChannelType::WhatsApp,
            sender: msg.from,
            recipient: String::new(),
            content,
            timestamp: chrono::Utc::now(),
            metadata: None,
        }
    }
}
