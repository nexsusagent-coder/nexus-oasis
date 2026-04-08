//! ─── LLM SAĞLAYICI TEMEL SINIFLARI ───
//!
//! Tüm LLM sağlayıcıları için ortak arayüz ve veri yapıları.

use sentient_common::error::{SENTIENTError, SENTIENTResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// ─── LLM İstek Yapısı ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
}

impl LlmRequest {
    pub fn new(model: impl Into<String>, messages: Vec<ChatMessage>) -> Self {
        Self {
            model: model.into(),
            messages,
            max_tokens: None,
            temperature: None,
            top_p: None,
            stream: Some(false),
            stop: None,
            frequency_penalty: None,
            presence_penalty: None,
        }
    }

    pub fn with_max_tokens(mut self, tokens: u32) -> Self {
        self.max_tokens = Some(tokens);
        self
    }

    pub fn with_temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    pub fn with_stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    /// Kullanıcı mesajı ekle
    pub fn add_user(mut self, content: impl Into<String>) -> Self {
        self.messages.push(ChatMessage {
            role: "user".into(),
            content: content.into(),
        });
        self
    }

    /// Sistem mesajı ekle
    pub fn add_system(mut self, content: impl Into<String>) -> Self {
        self.messages.push(ChatMessage {
            role: "system".into(),
            content: content.into(),
        });
        self
    }

    /// Asistan mesajı ekle
    pub fn add_assistant(mut self, content: impl Into<String>) -> Self {
        self.messages.push(ChatMessage {
            role: "assistant".into(),
            content: content.into(),
        });
        self
    }
}

/// ─── Sohbet Mesajı ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".into(),
            content: content.into(),
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".into(),
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".into(),
            content: content.into(),
        }
    }
}

/// ─── LLM Yanıt Yapısı ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    pub id: String,
    pub model: String,
    pub choices: Vec<ResponseChoice>,
    pub usage: Option<UsageStats>,
    pub created: u64,
}

impl LlmResponse {
    /// Yanıt içeriğini al
    pub fn content(&self) -> Option<&str> {
        self.choices.first().and_then(|c| c.message.content.as_deref())
    }

    /// Yanıt içeriğini String olarak al
    pub fn content_string(&self) -> String {
        self.content().unwrap_or("").to_string()
    }

    /// Toplam token kullanımını al
    pub fn total_tokens(&self) -> u64 {
        self.usage.as_ref().map(|u| u.total_tokens).unwrap_or(0)
    }
}

/// ─── Yanıt Seçeneği ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseChoice {
    pub index: u32,
    pub message: ResponseMessage,
    pub finish_reason: Option<String>,
}

/// ─── Yanıt Mesajı ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMessage {
    pub role: String,
    #[serde(default)]
    pub content: Option<String>,
}

/// ─── Kullanım İstatistikleri ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStats {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

/// ─── Sağlayıcı Arayüzü ───

#[async_trait]
pub trait LlmProvider {
    /// Sohbet tamamlama isteği gönder
    async fn chat_completion(&self, request: LlmRequest) -> SENTIENTResult<LlmResponse>;

    /// Modelleri listele
    async fn list_models(&self) -> SENTIENTResult<Vec<ModelInfo>>;

    /// Sağlayıcı adı
    fn name(&self) -> &str;
}

/// ─── Model Bilgisi ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub context_length: Option<u64>,
    pub pricing: Option<ModelPricing>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPricing {
    pub prompt_price: f64,  // 1K token başına $
    pub completion_price: f64,
}

/// ─── API Hatası ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub error: ApiErrorDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorDetail {
    pub message: String,
    pub r#type: String,
    pub code: Option<String>,
}

impl From<ApiError> for SENTIENTError {
    fn from(err: ApiError) -> Self {
        SENTIENTError::VGate(format!("API Hatası: {}", err.error.message))
    }
}
