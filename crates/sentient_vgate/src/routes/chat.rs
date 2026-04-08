//! ─── CHAT COMPLETION ROUTES ───
//!
//! OpenAI uyumlu chat completion endpoint'i.
//! Tüm istekler Guardrails ve koruma katmanlarından geçer.

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use log;

use crate::VGateState;
use crate::providers::base::{LlmRequest, ChatMessage};
use crate::middleware::security::RequestValidator;

/// ─── İstek Yapısı ───

#[derive(Debug, Deserialize)]
pub struct ChatCompletionRequest {
    /// Model adı (provider/model formatı veya direkt model)
    pub model: String,
    /// Sohbet mesajları
    pub messages: Vec<MessageInput>,
    /// Maksimum token sayısı
    #[serde(default)]
    pub max_tokens: Option<u32>,
    /// Sıcaklık (0.0 - 2.0)
    #[serde(default)]
    pub temperature: Option<f32>,
    /// Top-p sampling
    #[serde(default)]
    pub top_p: Option<f32>,
    /// Streaming modu
    #[serde(default)]
    pub stream: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct MessageInput {
    pub role: String,
    pub content: String,
}

impl From<MessageInput> for ChatMessage {
    fn from(msg: MessageInput) -> Self {
        ChatMessage {
            role: msg.role,
            content: msg.content,
        }
    }
}

/// ─── Yanıt Yapısı ───

#[derive(Debug, Serialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

#[derive(Debug, Serialize)]
pub struct Choice {
    pub index: u32,
    pub message: ResponseMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ResponseMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

/// ─── Hata Yanıtı ───

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    pub message: String,
    pub r#type: String,
    pub code: String,
}

/// ─── Handler ───

pub async fn chat_completions(
    state: State<Arc<Mutex<VGateState>>>,
    Json(request): Json<ChatCompletionRequest>,
) -> Response {
    log::info!("🚪  CHAT: İstek alındı → {} mesaj", request.messages.len());
    
    // 1. İsteği doğrula
    let model = match RequestValidator::validate_model(&request.model) {
        Ok(m) => m,
        Err(e) => return error_response(StatusCode::BAD_REQUEST, &e),
    };

    // 2. Mesajları doğrula
    let mut messages = Vec::new();
    for msg in request.messages {
        match RequestValidator::validate_message(&msg.content) {
            Ok(_) => messages.push(ChatMessage::from(msg)),
            Err(e) => return error_response(StatusCode::BAD_REQUEST, &e),
        }
    }

    if messages.is_empty() {
        return error_response(StatusCode::BAD_REQUEST, "En az bir mesaj gerekli");
    }

    // 3. Token limitini doğrula
    let max_tokens = match RequestValidator::validate_max_tokens(request.max_tokens) {
        Ok(t) => Some(t),
        Err(e) => return error_response(StatusCode::BAD_REQUEST, &e),
    };

    // 4. Sıcaklık doğrula
    let temperature = match RequestValidator::validate_temperature(request.temperature) {
        Ok(t) => Some(t),
        Err(e) => return error_response(StatusCode::BAD_REQUEST, &e),
    };

    // 5. Guardrails giden kontrol
    let state_guard = state.lock().await;
    let guardrails = &state_guard.guardrails;
    
    for msg in &messages {
        let result = guardrails.check_input(&msg.content);
        if !result.is_clean() {
            log::warn!("🚪  CHAT: Giden istek Guardrails tarafından engellendi");
            drop(state_guard);
            return error_response(
                StatusCode::FORBIDDEN,
                "İstek güvenlik filtresine takıldı. Potansiyel tehlikeli içerik algılandı.",
            );
        }
    }
    
    // 6. Sağlayıcıyı belirle
    let (provider, model_name) = crate::auth::RequestValidator::parse_model(&model)
        .unwrap_or((crate::auth::Provider::OpenRouter, model.clone()));
    
    // API anahtarını al
    let api_key = match state_guard.auth.get_api_key(&provider).await {
        Ok(key) => key,
        Err(e) => {
            drop(state_guard);
            return error_response(StatusCode::SERVICE_UNAVAILABLE, &e.to_sentient_message());
        }
    };
    
    let base_url = state_guard.auth.get_base_url(&provider).await;
    drop(state_guard);

    // 7. Sağlayıcı oluştur ve isteği gönder
    let provider_impl = crate::providers::ProviderFactory::create(
        provider,
        base_url,
        api_key,
    );

    let llm_request = LlmRequest::new(&model_name, messages)
        .with_max_tokens(max_tokens.unwrap_or(4096))
        .with_temperature(temperature.unwrap_or(0.7));

    // İsteği gönder
    match provider_impl.chat_completion(llm_request).await {
        Ok(response) => {
            let content = response.content_string();
            
            // 8. Gelen yanıt Guardrails kontrolü
            let state_guard = state.lock().await;
            let check = state_guard.guardrails.check_output(&content);
            
            if !check.is_clean() {
                log::warn!("🚪  CHAT: Gelen yanıt Guardrails tarafından engellendi");
                drop(state_guard);
                return error_response(
                    StatusCode::FORBIDDEN,
                    "Yanıt güvenlik filtresine takıldı.",
                );
            }
            drop(state_guard);

            // 9. Başarılı yanıt
            log::info!(
                "🚪  CHAT: Yanıt gönderildi → {} token",
                response.usage.as_ref().map(|u| u.total_tokens).unwrap_or(0)
            );

            let completion_response = ChatCompletionResponse {
                id: response.id,
                object: "chat.completion".into(),
                created: response.created,
                model: response.model,
                choices: response.choices.iter().map(|c| Choice {
                    index: c.index,
                    message: ResponseMessage {
                        role: c.message.role.clone(),
                        content: content.clone(),
                    },
                    finish_reason: c.finish_reason.clone(),
                }).collect(),
                usage: response.usage.map(|u| Usage {
                    prompt_tokens: u.prompt_tokens,
                    completion_tokens: u.completion_tokens,
                    total_tokens: u.total_tokens,
                }),
            };

            (StatusCode::OK, Json(completion_response)).into_response()
        }
        Err(e) => {
            log::error!("🚪  CHAT HATA → {}", e.summary());
            error_response(StatusCode::BAD_GATEWAY, &e.to_sentient_message())
        }
    }
}

/// Hata yanıtı oluştur
fn error_response(status: StatusCode, message: &str) -> Response {
    (
        status,
        Json(ErrorResponse {
            error: ErrorDetail {
                message: message.into(),
                r#type: "vgate_error".into(),
                code: status.as_u16().to_string(),
            },
        }),
    ).into_response()
}
