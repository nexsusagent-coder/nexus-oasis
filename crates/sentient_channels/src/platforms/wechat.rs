//! WeChat Channel Integration
//!
//! Supports WeChat Work (企业微信) and WeChat Official Account API with:
//! - Text messages
//! - Image, Voice, Video messages
//! - Rich media (articles)
//! - Menu management
//! - OAuth for user info

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use reqwest::Client;

/// WeChat configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeChatConfig {
    /// App ID (应用ID)
    pub app_id: String,

    /// App Secret (应用密钥)
    pub app_secret: String,

    /// Token for message encryption
    pub token: Option<String>,

    /// Encoding AES Key (43 characters)
    pub encoding_aes_key: Option<String>,

    /// Is Enterprise WeChat?
    pub is_enterprise: bool,

    /// Enterprise ID (企业ID) for WeChat Work
    pub corp_id: Option<String>,
}

impl Default for WeChatConfig {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            app_secret: String::new(),
            token: None,
            encoding_aes_key: None,
            is_enterprise: false,
            corp_id: None,
        }
    }
}

/// WeChat message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "MsgType")]
#[serde(rename_all = "lowercase")]
pub enum WeChatMessageType {
    Text {
        content: String,
    },
    Image {
        #[serde(rename = "MediaId")]
        media_id: String,
    },
    Voice {
        #[serde(rename = "MediaId")]
        media_id: String,
    },
    Video {
        #[serde(rename = "MediaId")]
        media_id: String,
        #[serde(rename = "ThumbMediaId")]
        thumb_media_id: String,
        title: Option<String>,
        description: Option<String>,
    },
    News {
        #[serde(rename = "ArticleCount")]
        article_count: u32,
        articles: Vec<WeChatArticle>,
    },
}

/// WeChat article for news messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeChatArticle {
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "PicUrl")]
    pub pic_url: Option<String>,
    #[serde(rename = "Url")]
    pub url: String,
}

/// WeChat incoming message
#[derive(Debug, Clone, Deserialize)]
pub struct WeChatIncomingMessage {
    /// Developer WeChat ID
    #[serde(rename = "ToUserName")]
    pub to_user_name: String,

    /// Sender WeChat ID
    #[serde(rename = "FromUserName")]
    pub from_user_name: String,

    /// Message creation time (timestamp)
    #[serde(rename = "CreateTime")]
    pub create_time: i64,

    /// Message type
    #[serde(rename = "MsgType")]
    pub msg_type: String,

    /// Text content (for text messages)
    pub content: Option<String>,

    /// Media ID (for media messages)
    #[serde(rename = "MediaId")]
    pub media_id: Option<String>,

    /// Message ID
    #[serde(rename = "MsgId")]
    pub msg_id: Option<i64>,
}

/// WeChat outgoing message
#[derive(Debug, Clone, Serialize)]
pub struct WeChatOutgoingMessage {
    /// Receiver WeChat ID
    #[serde(rename = "touser")]
    pub to_user: String,

    /// Message type
    #[serde(rename = "msgtype")]
    pub msg_type: String,

    /// Message content
    #[serde(flatten)]
    pub content: WeChatMessageType,
}

/// WeChat access token response
#[derive(Debug, Deserialize)]
struct AccessTokenResponse {
    access_token: String,
    expires_in: i64,
    errcode: Option<i32>,
    errmsg: Option<String>,
}

/// WeChat API response
#[derive(Debug, Deserialize)]
struct ApiResponse {
    errcode: i32,
    errmsg: String,
    msgid: Option<i64>,
}

/// WeChat channel implementation
pub struct WeChatChannel {
    config: WeChatConfig,
    client: Client,
    access_token: Option<String>,
}

impl WeChatChannel {
    /// Create a new WeChat channel
    pub fn new(config: WeChatConfig) -> Self {
        Self {
            config,
            client: Client::new(),
            access_token: None,
        }
    }

    /// Get access token
    pub async fn get_access_token(&mut self) -> Result<String, WeChatError> {
        let url = if self.config.is_enterprise {
            format!(
                "https://qyapi.weixin.qq.com/cgi-bin/gettoken?corpid={}&corpsecret={}",
                self.config.corp_id.as_ref().unwrap_or(&self.config.app_id),
                self.config.app_secret
            )
        } else {
            format!(
                "https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={}&secret={}",
                self.config.app_id,
                self.config.app_secret
            )
        };

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| WeChatError::RequestError(e.to_string()))?;

        let result: AccessTokenResponse = response
            .json()
            .await
            .map_err(|e| WeChatError::ParseError(e.to_string()))?;

        if let Some(errcode) = result.errcode {
            if errcode != 0 {
                return Err(WeChatError::ApiError {
                    code: errcode,
                    message: result.errmsg.unwrap_or_default(),
                });
            }
        }

        self.access_token = Some(result.access_token.clone());
        Ok(result.access_token)
    }

    /// Send text message
    pub async fn send_text(
        &mut self,
        to_user: &str,
        text: &str,
    ) -> Result<i64, WeChatError> {
        let token = if let Some(ref token) = self.access_token {
            token.clone()
        } else {
            self.get_access_token().await?
        };

        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/message/custom/send?access_token={}",
            token
        );

        let message = WeChatOutgoingMessage {
            to_user: to_user.to_string(),
            msg_type: "text".to_string(),
            content: WeChatMessageType::Text {
                content: text.to_string(),
            },
        };

        let response = self.client
            .post(&url)
            .json(&message)
            .send()
            .await
            .map_err(|e| WeChatError::RequestError(e.to_string()))?;

        let result: ApiResponse = response
            .json()
            .await
            .map_err(|e| WeChatError::ParseError(e.to_string()))?;

        if result.errcode != 0 {
            return Err(WeChatError::ApiError {
                code: result.errcode,
                message: result.errmsg,
            });
        }

        Ok(result.msgid.unwrap_or(0))
    }

    /// Send news message (articles)
    pub async fn send_news(
        &mut self,
        to_user: &str,
        articles: Vec<WeChatArticle>,
    ) -> Result<i64, WeChatError> {
        let token = if let Some(ref token) = self.access_token {
            token.clone()
        } else {
            self.get_access_token().await?
        };

        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/message/custom/send?access_token={}",
            token
        );

        let message = WeChatOutgoingMessage {
            to_user: to_user.to_string(),
            msg_type: "news".to_string(),
            content: WeChatMessageType::News {
                article_count: articles.len() as u32,
                articles,
            },
        };

        let response = self.client
            .post(&url)
            .json(&message)
            .send()
            .await
            .map_err(|e| WeChatError::RequestError(e.to_string()))?;

        let result: ApiResponse = response
            .json()
            .await
            .map_err(|e| WeChatError::ParseError(e.to_string()))?;

        if result.errcode != 0 {
            return Err(WeChatError::ApiError {
                code: result.errcode,
                message: result.errmsg,
            });
        }

        Ok(result.msgid.unwrap_or(0))
    }

    /// Upload media
    pub async fn upload_media(
        &mut self,
        media_type: &str,
        data: Vec<u8>,
    ) -> Result<String, WeChatError> {
        let token = if let Some(ref token) = self.access_token {
            token.clone()
        } else {
            self.get_access_token().await?
        };

        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/media/upload?access_token={}&type={}",
            token, media_type
        );

        let part = reqwest::multipart::Part::bytes(data)
            .file_name("media".to_string())
            .mime_str("application/octet-stream")
            .map_err(|e| WeChatError::UploadError(e.to_string()))?;

        let form = reqwest::multipart::Form::new().part("media", part);

        let response = self.client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| WeChatError::RequestError(e.to_string()))?;

        #[derive(Debug, Deserialize)]
        struct UploadResponse {
            media_id: String,
            #[serde(default)]
            errcode: i32,
            #[serde(default)]
            errmsg: String,
        }

        let result: UploadResponse = response
            .json()
            .await
            .map_err(|e| WeChatError::ParseError(e.to_string()))?;

        if result.errcode != 0 {
            return Err(WeChatError::ApiError {
                code: result.errcode,
                message: result.errmsg,
            });
        }

        Ok(result.media_id)
    }

    /// Verify webhook signature
    pub fn verify_signature(
        &self,
        signature: &str,
        timestamp: &str,
        nonce: &str,
    ) -> bool {
        use sha1::{Sha1, Digest};

        let token = match &self.config.token {
            Some(t) => t,
            None => return false,
        };

        let mut parts = vec![token, timestamp, nonce];
        parts.sort();

        let combined = parts.join("");
        let mut hasher = Sha1::new();
        hasher.update(combined.as_bytes());
        let result = hasher.finalize();
        let computed = hex::encode(result);

        computed == signature
    }
}

/// WeChat error types
#[derive(Debug, thiserror::Error)]
pub enum WeChatError {
    #[error("Request error: {0}")]
    RequestError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("API error {code}: {message}")]
    ApiError { code: i32, message: String },

    #[error("Upload error: {0}")]
    UploadError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wechat_article() {
        let article = WeChatArticle {
            title: "Test Article".to_string(),
            description: Some("Test description".to_string()),
            pic_url: Some("https://example.com/image.jpg".to_string()),
            url: "https://example.com/article".to_string(),
        };

        assert_eq!(article.title, "Test Article");
    }
}
