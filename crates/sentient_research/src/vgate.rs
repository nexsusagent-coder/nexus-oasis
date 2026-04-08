//! Research V-GATE Köprüsü
//! Araştırma modülü için LLM iletişim katmanı

use crate::error::{ResearchError, ResearchResult};
use serde::{Deserialize, Serialize};

/// Research V-GATE istemcisi
#[derive(Clone)]
pub struct ResearchVGate {
    /// V-GATE URL
    url: String,
    /// HTTP istemcisi
    client: reqwest::Client,
}

/// LLM İsteği
#[derive(Debug, Serialize)]
pub struct LLMRequest {
    /// Model adı
    pub model: String,
    /// Mesajlar
    pub messages: Vec<Message>,
    /// Sıcaklık
    pub temperature: f32,
    /// Maksimum token
    pub max_tokens: u32,
}

/// Mesaj
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    /// Rol
    pub role: String,
    /// İçerik
    pub content: String,
}

/// LLM Yanıtı
#[derive(Debug, Deserialize)]
pub struct LLMResponse {
    /// ID
    pub id: Option<String>,
    /// Seçimler
    pub choices: Vec<Choice>,
    /// Kullanım
    pub usage: Option<Usage>,
}

/// Seçim
#[derive(Debug, Deserialize)]
pub struct Choice {
    /// Index
    pub index: u32,
    /// Mesaj
    pub message: Message,
    /// Finish reason
    pub finish_reason: Option<String>,
}

/// Kullanım istatistikleri
#[derive(Debug, Deserialize)]
pub struct Usage {
    /// Prompt token sayısı
    pub prompt_tokens: u32,
    /// Completion token sayısı
    pub completion_tokens: u32,
    /// Toplam token sayısı
    pub total_tokens: u32,
}

impl ResearchVGate {
    /// Yeni V-GATE istemcisi oluştur
    pub fn new(url: &str) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        
        Self {
            url: url.to_string(),
            client,
        }
    }
    
    /// LLM isteği gönder
    pub async fn complete(&self, prompt: &str, system_prompt: Option<&str>) -> ResearchResult<String> {
        log::info!("🔬 RESEARCH-VGATE: LLM isteği gönderiliyor...");
        
        let mut messages = vec![];
        
        if let Some(system) = system_prompt {
            messages.push(Message {
                role: "system".into(),
                content: system.into(),
            });
        }
        
        messages.push(Message {
            role: "user".into(),
            content: prompt.into(),
        });
        
        let request = LLMRequest {
            model: "qwen/qwen3-235b-a22b:free".into(),
            messages,
            temperature: 0.7,
            max_tokens: 4096,
        };
        
        let response = self.client
            .post(format!("{}/v1/chat/completions", self.url))
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(ResearchError::VGateError {
                status,
                message: body,
            });
        }
        
        let llm_response: LLMResponse = response.json().await?;
        
        if let Some(choice) = llm_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(ResearchError::VGateError {
                status: 500,
                message: "No response choices".into(),
            })
        }
    }
    
    /// Araştırma promptu oluştur
    pub fn create_research_prompt(&self, query: &str, context: &str) -> String {
        format!(
            r#"Aşağıdaki sorgu için araştırma yap:

Sorgu: {}
Bağlam: {}

Lütfen:
1. Ana noktaları belirle
2. Alt sorular oluştur
3. Kaynak önerileri sun
4. Özet hazırla

Yanıt:"#,
            query, context
        )
    }
    
    /// Özet promptu oluştur
    pub fn create_summary_prompt(&self, content: &str) -> String {
        format!(
            r#"Aşağıdaki içeriği özetle:

{}

Özet kuralları:
- Ana fikirleri koru
- Kısa ve öz ol
- Maddeler halinde sun

Özet:"#,
            content
        )
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vgate_creation() {
        let vgate = ResearchVGate::new("http://127.0.0.1:1071");
        assert!(!vgate.url.is_empty());
    }
    
    #[test]
    fn test_research_prompt_creation() {
        let vgate = ResearchVGate::new("http://127.0.0.1:1071");
        let prompt = vgate.create_research_prompt("Test query", "Test context");
        
        assert!(prompt.contains("Test query"));
        assert!(prompt.contains("Test context"));
    }
    
    #[test]
    fn test_message_serialization() {
        let message = Message {
            role: "user".into(),
            content: "Test content".into(),
        };
        
        let json = serde_json::to_string(&message).unwrap();
        assert!(json.contains("user"));
        assert!(json.contains("Test content"));
    }
}
