//! ─── LLM ENTEGRASYON TESTLERİ ───
//!
//! V-GATE üzerinden OpenRouter'a güvenli bağlantı testleri.
//! API anahtarları asla istemcide tutulmaz, V-GATE sunucusundan alınır.

use sentient_common::error::{SENTIENTError, SENTIENTResult};
use sentient_vgate::providers::{LlmRequest, ProviderFactory};
use sentient_vgate::providers::base::ChatMessage;
use sentient_vgate::auth::Provider;
use log;
use std::time::Instant;

/// LLM Bağlantı Test Sonucu
#[derive(Debug, Clone)]
pub struct LlmTestResult {
    pub provider: String,
    pub model: String,
    pub success: bool,
    pub response_time_ms: u64,
    pub tokens_used: Option<u64>,
    pub content_preview: String,
    pub error: Option<String>,
}

impl LlmTestResult {
    pub fn summary(&self) -> String {
        if self.success {
            format!(
                "✅ [{}] {} → {}ms, {} token\n   └─ \"{}\"",
                self.provider,
                self.model,
                self.response_time_ms,
                self.tokens_used.unwrap_or(0),
                self.content_preview.chars().take(100).collect::<String>()
            )
        } else {
            format!(
                "❌ [{}] {} → {}\n   └─ Hata: {}",
                self.provider,
                self.model,
                self.response_time_ms,
                self.error.as_deref().unwrap_or("Bilinmeyen")
            )
        }
    }
}

/// LLM Test Suite
pub struct LlmTestSuite {
    provider: Provider,
    base_url: String,
    test_models: Vec<String>,
}

impl LlmTestSuite {
    /// OpenRouter test suite oluştur
    pub fn openrouter() -> Self {
        Self {
            provider: Provider::OpenRouter,
            base_url: "https://openrouter.ai/api/v1".into(),
            test_models: vec![
                "qwen/qwen3-1.7b:free".into(),
                "google/gemma-3-1b-it:free".into(),
            ],
        }
    }
    
    /// Custom test suite
    pub fn custom(provider: Provider, base_url: String, models: Vec<String>) -> Self {
        Self {
            provider,
            base_url,
            test_models: models,
        }
    }
    
    /// Tek bir model test et
    pub async fn test_model(&self, api_key: &str, model: &str) -> SENTIENTResult<LlmTestResult> {
        let start = Instant::now();
        
        log::info!("🧪  LLM TEST: {} → {} başlatılıyor...", self.provider.as_str(), model);
        
        let provider_impl = ProviderFactory::create(
            self.provider.clone(),
            self.base_url.clone(),
            api_key.into(),
        );
        
        let request = LlmRequest::new(model, vec![
            ChatMessage::system("Sen SENTIENT yapay zeka işletim sisteminin yardımcı asistanısın. Kısa ve öz cevap ver."),
            ChatMessage::user("Merhaba! Kendini kısaca tanıtır mısın?"),
        ])
        .with_max_tokens(150)
        .with_temperature(0.7);
        
        match provider_impl.chat_completion(request).await {
            Ok(response) => {
                let duration = start.elapsed();
                let content = response.content_string();
                let tokens = response.usage.as_ref().map(|u| u.total_tokens);
                
                log::info!(
                    "🧪  LLM TEST: {} → ✓ {}ms, {} token",
                    model,
                    duration.as_millis(),
                    tokens.unwrap_or(0)
                );
                
                Ok(LlmTestResult {
                    provider: self.provider.as_str().into(),
                    model: model.into(),
                    success: true,
                    response_time_ms: duration.as_millis() as u64,
                    tokens_used: tokens,
                    content_preview: content.chars().take(200).collect(),
                    error: None,
                })
            }
            Err(e) => {
                let duration = start.elapsed();
                let error_msg = e.to_sentient_message();
                
                log::warn!("🧪  LLM TEST: {} → ✗ {}", model, error_msg);
                
                Ok(LlmTestResult {
                    provider: self.provider.as_str().into(),
                    model: model.into(),
                    success: false,
                    response_time_ms: duration.as_millis() as u64,
                    tokens_used: None,
                    content_preview: String::new(),
                    error: Some(error_msg),
                })
            }
        }
    }
    
    /// Tüm test modellerini çalıştır
    pub async fn run_all(&self, api_key: &str) -> Vec<LlmTestResult> {
        log::info!("══════════════════════════════════════════════");
        log::info!("  🧪  LLM BAĞLANTI TESTLERİ BAŞLATILIYOR");
        log::info!("══════════════════════════════════════════════");
        log::info!("  Sağlayıcı: {}", self.provider.as_str());
        log::info!("  Test model sayısı: {}", self.test_models.len());
        log::info!("══════════════════════════════════════════════");
        
        let mut results = Vec::new();
        
        for model in &self.test_models {
            match self.test_model(api_key, model).await {
                Ok(result) => results.push(result),
                Err(e) => {
                    results.push(LlmTestResult {
                        provider: self.provider.as_str().into(),
                        model: model.clone(),
                        success: false,
                        response_time_ms: 0,
                        tokens_used: None,
                        content_preview: String::new(),
                        error: Some(e.to_sentient_message()),
                    });
                }
            }
            
            // Rate limiting için bekle
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
        
        // Sonuçları özetle
        let success_count = results.iter().filter(|r| r.success).count();
        let total_count = results.len();
        
        log::info!("══════════════════════════════════════════════");
        log::info!("  🧪  TEST SONUÇLARI: {}/{} başarılı", success_count, total_count);
        log::info!("══════════════════════════════════════════════");
        
        results
    }
    
    /// Modelleri listele
    pub async fn list_models(&self, api_key: &str) -> SENTIENTResult<Vec<String>> {
        let provider_impl = ProviderFactory::create(
            self.provider.clone(),
            self.base_url.clone(),
            api_key.into(),
        );
        
        let models = provider_impl.list_models().await?;
        Ok(models.iter().map(|m| format!("{} ({})", m.id, m.provider)).collect())
    }
}

/// Basit chat istemcisi
pub struct SimpleChat {
    provider: Provider,
    base_url: String,
    api_key: String,
    model: String,
    history: Vec<ChatMessage>,
}

impl SimpleChat {
    pub fn new(provider: Provider, base_url: String, api_key: String, model: String) -> Self {
        Self {
            provider,
            base_url,
            api_key,
            model,
            history: Vec::new(),
        }
    }
    
    /// OpenRouter ile oluştur
    pub fn openrouter(api_key: String, model: Option<String>) -> Self {
        Self::new(
            Provider::OpenRouter,
            "https://openrouter.ai/api/v1".into(),
            api_key,
            model.unwrap_or_else(|| "qwen/qwen3-1.7b:free".into()),
        )
    }
    
    /// Sistem mesajı ekle
    pub fn set_system(&mut self, content: &str) {
        // Varsa eski sistem mesajını sil
        self.history.retain(|m| m.role != "system");
        
        self.history.insert(0, ChatMessage::system(content));
    }
    
    /// Mesaj gönder ve yanıt al
    pub async fn send(&mut self, message: &str) -> SENTIENTResult<String> {
        // Kullanıcı mesajını ekle
        self.history.push(ChatMessage::user(message));
        
        let provider_impl = ProviderFactory::create(
            self.provider.clone(),
            self.base_url.clone(),
            self.api_key.clone(),
        );
        
        let request = LlmRequest::new(&self.model, self.history.clone())
            .with_max_tokens(2048)
            .with_temperature(0.7);
        
        log::debug!("💬  CHAT: {} → {}", self.model, message.chars().take(50).collect::<String>());
        
        let response = provider_impl.chat_completion(request).await?;
        let content = response.content_string();
        
        // Asistan yanıtını geçmişe ekle
        self.history.push(ChatMessage::assistant(&content));
        
        log::debug!(
            "💬  CHAT: Yanıt {} token",
            response.usage.as_ref().map(|u| u.total_tokens).unwrap_or(0)
        );
        
        Ok(content)
    }
    
    /// Geçmişi temizle (sistem mesajı hariç)
    pub fn clear_history(&mut self) {
        self.history.retain(|m| m.role == "system");
    }
    
    /// Geçmiş uzunluğu
    pub fn history_len(&self) -> usize {
        self.history.len()
    }
    
    /// Modeli değiştir
    pub fn set_model(&mut self, model: &str) {
        self.model = model.into();
        log::info!("💬  CHAT: Model değiştirildi → {}", model);
    }
}

/// API anahtarı kontrolü
pub fn check_api_key(api_key: &str) -> SENTIENTResult<bool> {
    if api_key.is_empty() {
        return Err(SENTIENTError::VGate(
            "API anahtarı boş. OPENROUTER_API_KEY ortam değişkenini ayarlayın.".into()
        ));
    }
    
    if api_key.len() < 20 {
        return Err(SENTIENTError::VGate(
            "API anahtarı çok kısa. Geçerli bir OpenRouter API anahtarı kullanın.".into()
        ));
    }
    
    if !api_key.starts_with("sk-or-") {
        log::warn!("⚠️  API anahtarı 'sk-or-' ile başlamıyor. OpenRouter anahtarı bekleniyor.");
    }
    
    Ok(true)
}

/// .env dosyasından API anahtarı oku (V-GATE Envguard kullanarak)
/// 
/// GÜVENLİK: API anahtarları asla kodda sabitlenmez!
/// Her zaman ortam değişkenlerinden okunur.
pub fn load_api_key_from_env() -> SENTIENTResult<String> {
    // Envguard başlat
    sentient_vgate::envguard::init();
    let guard = sentient_vgate::envguard::global();
    
    // API anahtarını güvenli şekilde al
    let key = guard.get_api_key(sentient_vgate::envguard::ApiProvider::OpenRouter)
        .map_err(|e| SENTIENTError::VGate(e.to_string()))?;
    
    check_api_key(&key)?;
    Ok(key)
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_check_api_key_valid() {
        let key = "sk-or-v1-1234567890abcdef1234567890abcdef";
        assert!(check_api_key(key).is_ok());
    }
    
    #[test]
    fn test_check_api_key_empty() {
        let result = check_api_key("");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_check_api_key_short() {
        let result = check_api_key("short");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_llm_test_suite_creation() {
        let suite = LlmTestSuite::openrouter();
        assert_eq!(suite.provider, Provider::OpenRouter);
        assert!(!suite.test_models.is_empty());
    }
    
    #[test]
    fn test_simple_chat_creation() {
        let chat = SimpleChat::openrouter("sk-or-test-key".into(), None);
        assert_eq!(chat.model, "qwen/qwen3-1.7b:free");
    }
    
    #[test]
    fn test_simple_chat_system_message() {
        let mut chat = SimpleChat::openrouter("sk-or-test-key".into(), None);
        chat.set_system("Test sistem mesajı");
        assert_eq!(chat.history.len(), 1);
        assert_eq!(chat.history[0].role, "system");
    }
    
    #[test]
    fn test_llm_test_result_summary_success() {
        let result = LlmTestResult {
            provider: "openrouter".into(),
            model: "test-model".into(),
            success: true,
            response_time_ms: 1500,
            tokens_used: Some(100),
            content_preview: "Merhaba, ben SENTIENT!".into(),
            error: None,
        };
        let summary = result.summary();
        assert!(summary.contains("✅"));
    }
    
    #[test]
    fn test_llm_test_result_summary_failure() {
        let result = LlmTestResult {
            provider: "openrouter".into(),
            model: "test-model".into(),
            success: false,
            response_time_ms: 500,
            tokens_used: None,
            content_preview: "".into(),
            error: Some("Bağlantı hatası".into()),
        };
        let summary = result.summary();
        assert!(summary.contains("❌"));
    }
}
