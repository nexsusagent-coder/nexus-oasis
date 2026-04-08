//! Custom Provider Settings - Universal LLM Gateway
//! Herhangi bir OpenAI/Anthropic uyumlu API'ye bağlantı desteği

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Custom Provider Yapılandırması
/// OpenAI ve Anthropic API formatını destekleyen HERHANGİ bir provider'a bağlanabilir
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomProviderConfig {
    /// Provider adı (kullanıcı tanımlı)
    pub name: String,
    
    /// Base URL (ör: https://api.openai.com/v1, https://api.anthropic.com, https://api.together.xyz/v1)
    pub base_url: String,
    
    /// API Key
    pub api_key: Option<String>,
    
    /// API Formatı (openai, anthropic)
    pub api_format: ApiFormat,
    
    /// Kullanılabilir modeller
    pub models: Vec<ModelInfo>,
    
    /// Varsayılan model
    pub default_model: Option<String>,
    
    /// Ek header'lar
    pub extra_headers: HashMap<String, String>,
    
    /// Timeout (saniye)
    pub timeout: u64,
    
    /// Aktif mi?
    pub enabled: bool,
    
    /// Rate limit (requests per minute)
    pub rate_limit: Option<u32>,
    
    /// Max retries
    pub max_retries: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApiFormat {
    /// OpenAI formatı: /chat/completions endpoint
    OpenAI,
    /// Anthropic formatı: /messages endpoint
    Anthropic,
    /// Custom endpoint
    Custom { endpoint: String },
}

impl Default for ApiFormat {
    fn default() -> Self {
        Self::OpenAI
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub context_length: usize,
    pub pricing: Option<PricingInfo>,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingInfo {
    pub input_per_1k: f64,
    pub output_per_1k: f64,
}

impl CustomProviderConfig {
    /// Yeni custom provider oluştur
    pub fn new(name: &str, base_url: &str) -> Self {
        Self {
            name: name.to_string(),
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key: None,
            api_format: ApiFormat::OpenAI,
            models: vec![],
            default_model: None,
            extra_headers: HashMap::new(),
            timeout: 120,
            enabled: true,
            rate_limit: None,
            max_retries: 3,
        }
    }
    
    /// API key ekle
    pub fn with_api_key(mut self, key: &str) -> Self {
        self.api_key = Some(key.to_string());
        self
    }
    
    /// API formatı ayarla
    pub fn with_format(mut self, format: ApiFormat) -> Self {
        self.api_format = format;
        self
    }
    
    /// Model ekle
    pub fn with_model(mut self, model: ModelInfo) -> Self {
        self.models.push(model);
        self
    }
    
    /// Chat completions URL'ini al
    pub fn chat_url(&self) -> String {
        match &self.api_format {
            ApiFormat::OpenAI => format!("{}/chat/completions", self.base_url),
            ApiFormat::Anthropic => format!("{}/v1/messages", self.base_url),
            ApiFormat::Custom { endpoint } => format!("{}{}", self.base_url, endpoint),
        }
    }
    
    /// Models URL'ini al
    pub fn models_url(&self) -> String {
        format!("{}/models", self.base_url)
    }
}

/// Önceden tanımlı popüler provider'lar
pub const PREDEFINED_PROVIDERS: &[(&str, &str, ApiFormat)] = &[
    // OpenAI Compatible
    ("Together AI", "https://api.together.xyz/v1", ApiFormat::OpenAI),
    ("Groq", "https://api.groq.com/openai/v1", ApiFormat::OpenAI),
    ("Fireworks AI", "https://api.fireworks.ai/inference/v1", ApiFormat::OpenAI),
    ("Perplexity AI", "https://api.perplexity.ai", ApiFormat::OpenAI),
    ("DeepSeek", "https://api.deepseek.com/v1", ApiFormat::OpenAI),
    ("Mistral AI", "https://api.mistral.ai/v1", ApiFormat::OpenAI),
    ("Replicate", "https://api.replicate.com/v1", ApiFormat::OpenAI),
    ("Anyscale", "https://api.endpoints.anyscale.com/v1", ApiFormat::OpenAI),
    ("Lepton AI", "https://api.lepton.ai/v1", ApiFormat::OpenAI),
    ("OctoAI", "https://api.octoai.cloud/v1", ApiFormat::OpenAI),
    ("SiliconFlow", "https://api.siliconflow.cn/v1", ApiFormat::OpenAI),
    ("OpenRouter", "https://openrouter.ai/api/v1", ApiFormat::OpenAI),
    
    // Anthropic Compatible
    ("Anthropic", "https://api.anthropic.com", ApiFormat::Anthropic),
    
    // Local
    ("Ollama (Local)", "http://localhost:11434/v1", ApiFormat::OpenAI),
    ("LM Studio (Local)", "http://localhost:1234/v1", ApiFormat::OpenAI),
    ("vLLM (Local)", "http://localhost:8000/v1", ApiFormat::OpenAI),
    ("LocalAI", "http://localhost:8080/v1", ApiFormat::OpenAI),
    
    // Cloud Providers
    ("AWS Bedrock", "https://bedrock-runtime.{region}.amazonaws.com", ApiFormat::OpenAI),
    ("Google Vertex AI", "https://{region}-aiplatform.googleapis.com/v1", ApiFormat::OpenAI),
    ("Azure OpenAI", "https://{resource}.openai.azure.com/openai/deployments", ApiFormat::OpenAI),
    
    // Chinese Providers
    ("Alibaba Qwen", "https://dashscope.aliyuncs.com/api/v1", ApiFormat::OpenAI),
    ("Baidu Ernie", "https://aip.baidubce.com/rpc/2.0/ai_custom/v1", ApiFormat::OpenAI),
    ("Zhipu AI", "https://open.bigmodel.cn/api/paas/v4", ApiFormat::OpenAI),
    ("Moonshot", "https://api.moonshot.cn/v1", ApiFormat::OpenAI),
    ("Baichuan", "https://api.baichuan-ai.com/v1", ApiFormat::OpenAI),
    
    // European Providers
    ("Mistral (EU)", "https://api.mistral.ai/v1", ApiFormat::OpenAI),
    ("DeepInfra", "https://api.deepinfra.com/v1/openai", ApiFormat::OpenAI),
    ("Nebius AI", "https://api.nebius.ai/v1", ApiFormat::OpenAI),
];

impl std::fmt::Display for ApiFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiFormat::OpenAI => write!(f, "OpenAI Compatible"),
            ApiFormat::Anthropic => write!(f, "Anthropic Compatible"),
            ApiFormat::Custom { endpoint } => write!(f, "Custom ({})", endpoint),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_custom_provider_url() {
        let provider = CustomProviderConfig::new("Test", "https://api.example.com/v1")
            .with_format(ApiFormat::OpenAI);
        
        assert_eq!(provider.chat_url(), "https://api.example.com/v1/chat/completions");
    }
    
    #[test]
    fn test_anthropic_format() {
        let provider = CustomProviderConfig::new("Anthropic", "https://api.anthropic.com")
            .with_format(ApiFormat::Anthropic);
        
        assert_eq!(provider.chat_url(), "https://api.anthropic.com/v1/messages");
    }
}
