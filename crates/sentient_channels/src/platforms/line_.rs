//! LINE Channel Integration
//!
//! Supports LINE Messaging API with:
//! - Text, Image, Video, Audio messages
//! - Flex Messages (rich layouts)
//! - Quick Reply buttons
//! - Rich Menu
//! - Push notifications

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use reqwest::Client;

/// LINE channel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineConfig {
    /// Channel Access Token
    pub channel_access_token: String,

    /// Channel Secret
    pub channel_secret: String,
}

impl Default for LineConfig {
    fn default() -> Self {
        Self {
            channel_access_token: String::new(),
            channel_secret: String::new(),
        }
    }
}

/// LINE message types
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum LineMessage {
    Text {
        text: String,
    },
    Image {
        original_content_url: String,
        preview_image_url: String,
    },
    Video {
        original_content_url: String,
        preview_image_url: String,
    },
    Audio {
        original_content_url: String,
        duration: u32,
    },
    Location {
        title: String,
        address: String,
        latitude: f64,
        longitude: f64,
    },
    Sticker {
        package_id: String,
        sticker_id: String,
    },
    Flex {
        alt_text: String,
        contents: FlexMessage,
    },
}

/// Flex Message container
#[derive(Debug, Clone, Serialize)]
pub struct FlexMessage {
    #[serde(rename = "type")]
    pub container_type: String,
    pub contents: Vec<FlexComponent>,
}

/// Flex Message component
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum FlexComponent {
    Box {
        layout: String,
        contents: Vec<FlexComponent>,
        #[serde(skip_serializing_if = "Option::is_none")]
        flex: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        background_color: Option<String>,
    },
    Text {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        size: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        weight: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        color: Option<String>,
    },
    Image {
        url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        size: Option<String>,
    },
    Button {
        action: LineAction,
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<String>,
    },
    Separator,
    Spacer,
}

/// LINE action
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum LineAction {
    Message { label: String, text: String },
    Uri { label: String, uri: String },
    Postback { label: String, data: String, display_text: Option<String> },
}

/// Flex Message builder
pub struct FlexBuilder {
    components: Vec<FlexComponent>,
}

impl FlexBuilder {
    /// Create a new Flex message builder
    pub fn new() -> Self {
        Self {
            components: vec![],
        }
    }

    /// Add text component
    pub fn text(mut self, text: &str, size: Option<&str>, weight: Option<&str>) -> Self {
        self.components.push(FlexComponent::Text {
            text: text.to_string(),
            size: size.map(|s| s.to_string()),
            weight: weight.map(|w| w.to_string()),
            color: None,
        });
        self
    }

    /// Add image component
    pub fn image(mut self, url: &str, size: Option<&str>) -> Self {
        self.components.push(FlexComponent::Image {
            url: url.to_string(),
            size: size.map(|s| s.to_string()),
        });
        self
    }

    /// Add button with message action
    pub fn button_message(mut self, label: &str, message: &str) -> Self {
        self.components.push(FlexComponent::Button {
            action: LineAction::Message {
                label: label.to_string(),
                text: message.to_string(),
            },
            style: Some("primary".to_string()),
        });
        self
    }

    /// Add button with URI action
    pub fn button_uri(mut self, label: &str, uri: &str) -> Self {
        self.components.push(FlexComponent::Button {
            action: LineAction::Uri {
                label: label.to_string(),
                uri: uri.to_string(),
            },
            style: None,
        });
        self
    }

    /// Add separator
    pub fn separator(mut self) -> Self {
        self.components.push(FlexComponent::Separator);
        self
    }

    /// Build into a bubble container
    pub fn build_bubble(self, alt_text: &str) -> LineMessage {
        LineMessage::Flex {
            alt_text: alt_text.to_string(),
            contents: FlexMessage {
                container_type: "bubble".to_string(),
                contents: self.components,
            },
        }
    }

    /// Build into a carousel container
    pub fn build_carousel(self, alt_text: &str) -> LineMessage {
        LineMessage::Flex {
            alt_text: alt_text.to_string(),
            contents: FlexMessage {
                container_type: "carousel".to_string(),
                contents: self.components,
            },
        }
    }
}

impl Default for FlexBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// LINE webhook event
#[derive(Debug, Clone, Deserialize)]
pub struct LineWebhookEvent {
    pub destination: String,
    pub events: Vec<LineEvent>,
}

/// LINE event
#[derive(Debug, Clone, Deserialize)]
pub struct LineEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(rename = "replyToken")]
    pub reply_token: Option<String>,
    pub timestamp: i64,
    pub source: LineSource,
    pub message: Option<LineEventMessage>,
}

/// LINE event source
#[derive(Debug, Clone, Deserialize)]
pub struct LineSource {
    #[serde(rename = "type")]
    pub source_type: String,
    pub user_id: Option<String>,
    pub group_id: Option<String>,
    pub room_id: Option<String>,
}

/// LINE event message
#[derive(Debug, Clone, Deserialize)]
pub struct LineEventMessage {
    id: String,
    #[serde(rename = "type")]
    pub message_type: String,
    pub text: Option<String>,
}

/// LINE push message request
#[derive(Debug, Serialize)]
struct PushMessageRequest {
    to: String,
    messages: Vec<LineMessage>,
}

/// LINE reply message request
#[derive(Debug, Serialize)]
struct ReplyMessageRequest {
    #[serde(rename = "replyToken")]
    reply_token: String,
    messages: Vec<LineMessage>,
}

/// LINE channel implementation
pub struct LineChannel {
    config: LineConfig,
    client: Client,
}

impl LineChannel {
    /// Create a new LINE channel
    pub fn new(config: LineConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    /// Push message to user
    pub async fn push_message(
        &self,
        to: &str,
        messages: Vec<LineMessage>,
    ) -> Result<(), LineError> {
        let url = "https://api.line.me/v2/bot/message/push";

        let request = PushMessageRequest {
            to: to.to_string(),
            messages,
        };

        let response = self.client
            .post(url)
            .bearer_auth(&self.config.channel_access_token)
            .json(&request)
            .send()
            .await
            .map_err(|e| LineError::RequestError(e.to_string()))?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(LineError::ApiError(error));
        }

        Ok(())
    }

    /// Reply to a message
    pub async fn reply_message(
        &self,
        reply_token: &str,
        messages: Vec<LineMessage>,
    ) -> Result<(), LineError> {
        let url = "https://api.line.me/v2/bot/message/reply";

        let request = ReplyMessageRequest {
            reply_token: reply_token.to_string(),
            messages,
        };

        let response = self.client
            .post(url)
            .bearer_auth(&self.config.channel_access_token)
            .json(&request)
            .send()
            .await
            .map_err(|e| LineError::RequestError(e.to_string()))?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(LineError::ApiError(error));
        }

        Ok(())
    }

    /// Send text message
    pub async fn send_text(&self, to: &str, text: &str) -> Result<(), LineError> {
        self.push_message(to, vec![LineMessage::Text {
            text: text.to_string(),
        }]).await
    }

    /// Send Flex message
    pub async fn send_flex(
        &self,
        to: &str,
        alt_text: &str,
        flex: FlexMessage,
    ) -> Result<(), LineError> {
        self.push_message(to, vec![LineMessage::Flex {
            alt_text: alt_text.to_string(),
            contents: flex,
        }]).await
    }

    /// Get user profile
    pub async fn get_profile(&self, user_id: &str) -> Result<LineProfile, LineError> {
        let url = format!(
            "https://api.line.me/v2/bot/profile/{}",
            user_id
        );

        let response = self.client
            .get(&url)
            .bearer_auth(&self.config.channel_access_token)
            .send()
            .await
            .map_err(|e| LineError::RequestError(e.to_string()))?;

        let profile = response
            .json()
            .await
            .map_err(|e| LineError::ParseError(e.to_string()))?;

        Ok(profile)
    }

    /// Verify webhook signature
    pub fn verify_signature(&self, body: &str, signature: &str) -> bool {
        use hmac::{Hmac, Mac};
        use sha256::Sha256;

        type HmacSha256 = Hmac<Sha256>;

        let mut mac = match HmacSha256::new_from_slice(self.config.channel_secret.as_bytes()) {
            Ok(m) => m,
            Err(_) => return false,
        };

        mac.update(body.as_bytes());
        let result = mac.finalize();
        let computed = base64::encode(result.into_bytes());

        computed == signature
    }
}

/// LINE user profile
#[derive(Debug, Clone, Deserialize)]
pub struct LineProfile {
    pub user_id: String,
    pub display_name: String,
    pub picture_url: Option<String>,
    pub status_message: Option<String>,
}

/// LINE error types
#[derive(Debug, thiserror::Error)]
pub enum LineError {
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
    fn test_flex_builder() {
        let message = FlexBuilder::new()
            .text("Hello, LINE!", Some("xl"), Some("bold"))
            .separator()
            .button_message("Click me", "clicked!")
            .build_bubble("Hello message");

        match message {
            LineMessage::Flex { alt_text, .. } => {
                assert_eq!(alt_text, "Hello message");
            }
            _ => panic!("Expected Flex message"),
        }
    }
}
