//! ─── LLM Hub Registry ───
//!
//! Central registry for all LLM providers with intelligent routing

use std::collections::HashMap;
use std::sync::Arc;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::{LlmProvider, ProviderInfo};
use crate::models;

// ═══════════════════════════════════════════════════════════════════════════════
//  LLM HUB - Central Registry
// ═══════════════════════════════════════════════════════════════════════════════

/// Central hub for all LLM providers
pub struct LlmHub {
    providers: HashMap<String, Arc<dyn LlmProvider>>,
    default_model: String,
    routing_strategy: RoutingStrategy,
}

/// Routing strategy for model selection
#[derive(Debug, Clone, Copy)]
pub enum RoutingStrategy {
    /// Use default model always
    Default,
    /// Route to cheapest provider
    Cheapest,
    /// Route to fastest provider
    Fastest,
    /// Route to best quality
    BestQuality,
    /// Route to free tier if available
    FreeTierFirst,
}

impl Default for RoutingStrategy {
    fn default() -> Self {
        Self::Default
    }
}

impl LlmHub {
    /// Create new empty hub
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            default_model: "gpt-4o-mini".into(),
            routing_strategy: RoutingStrategy::default(),
        }
    }

    /// Create hub with all providers from environment variables
    pub fn from_env() -> LlmResult<Self> {
        let mut hub = Self::new();

        // ═══════════════════════════════════════════════════════════
        //  DIRECT PROVIDERS
        // ═══════════════════════════════════════════════════════════
        if let Ok(p) = crate::providers::OpenAIProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        if let Ok(p) = crate::providers::AnthropicProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        if let Ok(p) = crate::providers::GoogleProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        if let Ok(p) = crate::providers::MistralProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        if let Ok(p) = crate::providers::DeepSeekProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        if let Ok(p) = crate::providers::XAIProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        if let Ok(p) = crate::providers::CohereProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        if let Ok(p) = crate::providers::PerplexityProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        if let Ok(p) = crate::providers::GroqProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        if let Ok(p) = crate::providers::TogetherProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        if let Ok(p) = crate::providers::FireworksProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        if let Ok(p) = crate::providers::ReplicateProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        if let Ok(p) = crate::providers::AI21Provider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        // Ollama doesn't need API key
        if let Ok(p) = crate::providers::OllamaProvider::new() {
            hub = hub.register(Arc::new(p));
        }

        // ═══════════════════════════════════════════════════════════
        //  AGGREGATOR PROVIDERS (200+ models each)
        // ═══════════════════════════════════════════════════════════
        if let Ok(p) = crate::providers::OpenRouterProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        if let Ok(p) = crate::providers::GlhfProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        if let Ok(p) = crate::providers::NovitaProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        if let Ok(p) = crate::providers::HyperbolicProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        if let Ok(p) = crate::providers::SiliconFlowProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        if let Ok(p) = crate::providers::CerebrasProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        if let Ok(p) = crate::providers::LiteLLMProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }

        // ═══════════════════════════════════════════════════════════
        //  ENTERPRISE PROVIDERS
        // ═══════════════════════════════════════════════════════════
        if let Ok(p) = crate::providers::NvidiaProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        if let Ok(p) = crate::providers::SambaNovaProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        if let Ok(p) = crate::providers::DeepInfraProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        if let Ok(p) = crate::providers::AzureOpenAIProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        if let Ok(p) = crate::providers::BedrockProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        if let Ok(p) = crate::providers::VertexAIProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }

        // ═══════════════════════════════════════════════════════════
        //  LOCAL PROVIDERS
        // ═══════════════════════════════════════════════════════════
        if let Ok(p) = crate::providers::VLLMProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }
        if let Ok(p) = crate::providers::LmStudioProvider::from_env() {
            hub = hub.register(Arc::new(p));
        }

        Ok(hub)
    }

    /// Register a provider
    pub fn register(mut self, provider: Arc<dyn LlmProvider>) -> Self {
        self.providers.insert(provider.id().to_string(), provider);
        self
    }

    /// Set default model
    pub fn with_default_model(mut self, model: impl Into<String>) -> Self {
        self.default_model = model.into();
        self
    }

    /// Set routing strategy
    pub fn with_routing(mut self, strategy: RoutingStrategy) -> Self {
        self.routing_strategy = strategy;
        self
    }

    /// Get all registered providers
    pub fn providers(&self) -> Vec<&Arc<dyn LlmProvider>> {
        self.providers.values().collect()
    }

    /// Get provider by ID
    pub fn get_provider(&self, id: &str) -> Option<&Arc<dyn LlmProvider>> {
        self.providers.get(id)
    }

    /// Get all available models
    pub fn models(&self) -> Vec<ModelInfo> {
        self.providers.values().flat_map(|p| p.models()).collect()
    }

    /// Get model info by ID
    pub fn get_model(&self, model_id: &str) -> Option<ModelInfo> {
        self.providers.values()
            .find_map(|p| p.get_model(model_id))
    }

    /// Find provider for model
    fn find_provider_for_model(&self, model_id: &str) -> LlmResult<&Arc<dyn LlmProvider>> {
        // First, try exact match
        for provider in self.providers.values() {
            if provider.get_model(model_id).is_some() {
                return Ok(provider);
            }
        }

        // Try fuzzy matching
        let model_lower = model_id.to_lowercase();
        for provider in self.providers.values() {
            for model in provider.models() {
                if model.id.to_lowercase().contains(&model_lower)
                    || model_lower.contains(&model.id.to_lowercase())
                {
                    return Ok(provider);
                }
            }
        }

        Err(LlmError::ModelNotFound(model_id.into()))
    }

    /// Chat with automatic provider routing
    pub async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let model_id = if request.model.is_empty() {
            &self.default_model
        } else {
            &request.model
        };

        let provider = self.find_provider_for_model(model_id)?;

        // Create new request with resolved model ID
        let resolved_model = provider.get_model(model_id)
            .map(|m| m.id.clone())
            .unwrap_or_else(|| model_id.clone());

        let mut resolved_request = request;
        resolved_request.model = resolved_model;

        provider.chat(resolved_request).await
    }

    /// Chat with streaming
    pub async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> LlmResult<impl futures::Stream<Item = LlmResult<StreamChunk>>> {
        let model_id = if request.model.is_empty() {
            &self.default_model
        } else {
            &request.model
        };

        let provider = self.find_provider_for_model(model_id)?;
        provider.chat_stream(request).await
    }

    /// Get cheapest provider for model
    pub fn get_cheapest_provider(&self, model_id: &str) -> Option<&Arc<dyn LlmProvider>> {
        let mut cheapest: Option<(&Arc<dyn LlmProvider>, f64)> = None;

        for provider in self.providers.values() {
            if let Some(model) = provider.get_model(model_id) {
                let cost = model.input_cost_per_1k + model.output_cost_per_1k;
                if cheapest.is_none() || cost < cheapest.unwrap().1 {
                    cheapest = Some((provider, cost));
                }
            }
        }

        cheapest.map(|(p, _)| p)
    }

    /// Get fastest provider for model
    pub fn get_fastest_provider(&self, model_id: &str) -> Option<&Arc<dyn LlmProvider>> {
        let mut fastest: Option<(&Arc<dyn LlmProvider>, u8)> = None;

        for provider in self.providers.values() {
            if let Some(model) = provider.get_model(model_id) {
                if fastest.is_none() || model.speed_rating > fastest.unwrap().1 {
                    fastest = Some((provider, model.speed_rating));
                }
            }
        }

        fastest.map(|(p, _)| p)
    }

    /// Get best quality provider for model
    pub fn get_best_quality_provider(&self, model_id: &str) -> Option<&Arc<dyn LlmProvider>> {
        let mut best: Option<(&Arc<dyn LlmProvider>, u8)> = None;

        for provider in self.providers.values() {
            if let Some(model) = provider.get_model(model_id) {
                if best.is_none() || model.quality_rating > best.unwrap().1 {
                    best = Some((provider, model.quality_rating));
                }
            }
        }

        best.map(|(p, _)| p)
    }

    /// Get free tier providers
    pub fn get_free_tier_providers(&self) -> Vec<&Arc<dyn LlmProvider>> {
        self.providers.values()
            .filter(|p| {
                p.models().iter().any(|m| m.free_tier)
            })
            .collect()
    }

    /// Count tokens for any model
    pub fn count_tokens(&self, text: &str, model: &str) -> LlmResult<usize> {
        let provider = self.find_provider_for_model(model)?;
        provider.count_tokens(text, model)
    }

    /// Get configured provider count
    pub fn configured_count(&self) -> usize {
        self.providers.len()
    }

    /// Check if any provider is configured
    pub fn is_any_configured(&self) -> bool {
        !self.providers.is_empty()
    }

    /// Compare models by cost
    pub fn compare_cost(&self, prompt_tokens: u32, completion_tokens: u32) -> Vec<(String, f64)> {
        let mut costs: Vec<(String, f64)> = self.models()
            .into_iter()
            .map(|m| {
                let cost = (prompt_tokens as f64 * m.input_cost_per_1k / 1000.0)
                         + (completion_tokens as f64 * m.output_cost_per_1k / 1000.0);
                (format!("{} ({})", m.name, m.provider), cost)
            })
            .collect();

        costs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        costs
    }
}

impl Default for LlmHub {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  BUILDER PATTERN
// ═══════════════════════════════════════════════════════════════════════════════

/// Builder for LlmHub
pub struct LlmHubBuilder {
    hub: LlmHub,
}

impl LlmHubBuilder {
    pub fn new() -> Self {
        Self {
            hub: LlmHub::new(),
        }
    }

    pub fn openai(mut self, api_key: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::OpenAIProvider::new(api_key)?));
        Ok(self)
    }

    pub fn anthropic(mut self, api_key: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::AnthropicProvider::new(api_key)?));
        Ok(self)
    }

    pub fn google(mut self, api_key: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::GoogleProvider::new(api_key)?));
        Ok(self)
    }

    pub fn mistral(mut self, api_key: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::MistralProvider::new(api_key)?));
        Ok(self)
    }

    pub fn deepseek(mut self, api_key: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::DeepSeekProvider::new(api_key)?));
        Ok(self)
    }

    pub fn xai(mut self, api_key: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::XAIProvider::new(api_key)?));
        Ok(self)
    }

    pub fn cohere(mut self, api_key: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::CohereProvider::new(api_key)?));
        Ok(self)
    }

    pub fn perplexity(mut self, api_key: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::PerplexityProvider::new(api_key)?));
        Ok(self)
    }

    pub fn groq(mut self, api_key: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::GroqProvider::new(api_key)?));
        Ok(self)
    }

    pub fn together(mut self, api_key: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::TogetherProvider::new(api_key)?));
        Ok(self)
    }

    pub fn fireworks(mut self, api_key: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::FireworksProvider::new(api_key)?));
        Ok(self)
    }

    pub fn replicate(mut self, api_key: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::ReplicateProvider::new(api_key)?));
        Ok(self)
    }

    pub fn ai21(mut self, api_key: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::AI21Provider::new(api_key)?));
        Ok(self)
    }

    pub fn ollama(mut self) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::OllamaProvider::new()?));
        Ok(self)
    }

    // ═══════════════════════════════════════════════════════════
    //  AGGREGATOR PROVIDERS
    // ═══════════════════════════════════════════════════════════

    pub fn openrouter(mut self, api_key: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::OpenRouterProvider::new(api_key)?));
        Ok(self)
    }

    pub fn glhf(mut self, api_key: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::GlhfProvider::new(api_key)?));
        Ok(self)
    }

    pub fn novita(mut self, api_key: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::NovitaProvider::new(api_key)?));
        Ok(self)
    }

    pub fn hyperbolic(mut self, api_key: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::HyperbolicProvider::new(api_key)?));
        Ok(self)
    }

    pub fn siliconflow(mut self, api_key: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::SiliconFlowProvider::new(api_key)?));
        Ok(self)
    }

    pub fn cerebras(mut self, api_key: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::CerebrasProvider::new(api_key)?));
        Ok(self)
    }

    pub fn litellm(mut self, api_key: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::LiteLLMProvider::new(api_key)?));
        Ok(self)
    }

    // ═══════════════════════════════════════════════════════════
    //  ENTERPRISE PROVIDERS
    // ═══════════════════════════════════════════════════════════

    pub fn nvidia(mut self, api_key: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::NvidiaProvider::new(api_key)?));
        Ok(self)
    }

    pub fn sambanova(mut self, api_key: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::SambaNovaProvider::new(api_key)?));
        Ok(self)
    }

    pub fn deepinfra(mut self, api_key: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::DeepInfraProvider::new(api_key)?));
        Ok(self)
    }

    // ═══════════════════════════════════════════════════════════
    //  LOCAL PROVIDERS
    // ═══════════════════════════════════════════════════════════

    pub fn vllm(mut self) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::VLLMProvider::from_env()?));
        Ok(self)
    }

    pub fn vllm_with_url(mut self, base_url: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::VLLMProvider::new(base_url)?));
        Ok(self)
    }

    pub fn lmstudio(mut self) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::LmStudioProvider::new()?));
        Ok(self)
    }

    // ═══════════════════════════════════════════════════════════
    //  CLOUD ENTERPRISE PROVIDERS
    // ═══════════════════════════════════════════════════════════

    pub fn azure(mut self, api_key: impl Into<String>, endpoint: impl Into<String>, deployment: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::AzureOpenAIProvider::new(api_key, endpoint, deployment)?));
        Ok(self)
    }

    pub fn bedrock(mut self, access_key: impl Into<String>, secret_key: impl Into<String>, region: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::BedrockProvider::new(access_key, secret_key, region)?));
        Ok(self)
    }

    pub fn vertex(mut self, access_token: impl Into<String>, project_id: impl Into<String>, location: impl Into<String>) -> LlmResult<Self> {
        self.hub = self.hub.register(Arc::new(crate::providers::VertexAIProvider::new(access_token, project_id, location)?));
        Ok(self)
    }

    pub fn default_model(mut self, model: impl Into<String>) -> Self {
        self.hub.default_model = model.into();
        self
    }

    pub fn routing(mut self, strategy: RoutingStrategy) -> Self {
        self.hub.routing_strategy = strategy;
        self
    }

    pub fn build(self) -> LlmHub {
        self.hub
    }
}

impl Default for LlmHubBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hub_creation() {
        let hub = LlmHub::new();
        assert_eq!(hub.configured_count(), 0);
    }

    #[test]
    fn test_hub_builder() {
        let hub = LlmHubBuilder::new()
            .ollama().unwrap()
            .default_model("llama3.2")
            .build();

        assert_eq!(hub.configured_count(), 1);
        assert_eq!(hub.default_model, "llama3.2");
    }

    #[test]
    fn test_models_from_hub() {
        let hub = LlmHubBuilder::new()
            .ollama().unwrap()
            .build();

        let models = hub.models();
        assert!(!models.is_empty());
    }

    #[test]
    fn test_cost_comparison() {
        let hub = LlmHubBuilder::new()
            .ollama().unwrap()
            .build();

        let costs = hub.compare_cost(1000, 500);
        // Ollama models are free
        assert!(costs.iter().all(|(_, c)| *c == 0.0));
    }
}
