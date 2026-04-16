//! ─── SENTIENT LLM ───
//!
//! Comprehensive LLM Model Hub - ALL providers, ALL models, unified API
//! 200+ models, 50+ providers, from GPT-2 (2019) to o4/Gemini 2.5 (2026)
//!
//! # Supported Provider Categories
//!
//! ## Tier-1 AI Companies
//! - **OpenAI**: GPT-4o, GPT-4 Turbo, GPT-3.5, o1, o3, o4-mini
//! - **Anthropic**: Claude 4, Claude 3.5, Claude 3, Claude 2, Claude 1
//! - **Google**: Gemini 2.5, Gemini 2.0, Gemini 1.5, Gemma 3
//! - **Mistral**: Mistral Large 2, Small 3.1, Codestral, Pixtral
//! - **DeepSeek**: DeepSeek V3, R1, Coder (cheapest!)
//! - **xAI**: Grok 3, Grok 3 Mini (Reasoning), Grok 2
//! - **Cohere**: Command A, Command R+, Aya Exa
//! - **Perplexity**: Sonar, Sonar Pro, Deep Research
//!
//! ## Aggregator Platforms
//! - **OpenRouter**: 200+ models via single API
//! - **Groq**: LPU inference - fastest!
//! - **Together**: 100+ open source models
//! - **Fireworks**: Fast open source inference
//! - **Cerebras**: Wafer-scale fastest inference
//! - **Cloudflare Workers AI**: Edge inference
//! - **Chutes**: Free serverless AI inference
//! - **SiliconFlow**: Chinese GPU cloud
//!
//! ## Enterprise / Cloud
//! - **Azure OpenAI**, **AWS Bedrock**, **GCP Vertex AI**
//! - **NVIDIA NIM**, **SambaNova**, **IBM WatsonX**
//! - **DeepInfra**, **OCI GenAI**
//!
//! ## Chinese AI
//! - **Qwen (Alibaba)**: Qwen3, QwQ, QVQ
//! - **Baidu ERNIE**, **Zhipu GLM**, **Moonshot Kimi**
//! - **MiniMax**, **StepFun**, **ByteDance Doubao**
//!
//! ## Regional AI
//! - **GigaChat** (Russia), **Upstage** (Korea), **Aleph Alpha** (EU)
//! - **Sarvam AI** (India), **Reka AI** (multimodal)
//!
//! ## Local / Self-Hosted
//! - **Ollama**: Local LLM inference (FREE!)
//! - **vLLM**: High-throughput serving
//! - **LM Studio**: Desktop LLM
//! - **Llamafile**: Single-file distribution
//! - **Cevahir AI**: SENTIENT's own LLM engine
//!
//! # Example
//! ```rust,ignore
//! use sentient_llm::{LlmHub, ChatRequest, Message};
//!
//! #[tokio::main]
//! async fn main() {
//!     let hub = LlmHub::from_env().unwrap();
//!
//!     let response = hub.chat(ChatRequest {
//!         model: "gpt-4o".into(),
//!         messages: vec![Message::user("Hello!")],
//!         ..Default::default()
//!     }).await.unwrap();
//!
//!     println!("{}", response.choices[0].message.content.as_text().unwrap());
//! }
//! ```

pub mod error;
pub mod types;
pub mod models;
pub mod provider;
pub mod providers;
pub mod registry;
pub mod cache;
pub mod distributed;
pub mod router;
pub mod streaming;
pub mod cost_tracker;

pub use error::{LlmError, LlmResult};
pub use types::{
    ChatRequest, ChatResponse, Message, Role, Content, ContentPart,
    ToolCall, FunctionCall, Tool, FunctionDefinition,
    Usage, Choice, StreamChunk, StreamChoice, Delta,
    ModelInfo, ResponseFormat, ToolChoice,
};
pub use provider::{LlmProvider, ProviderInfo};
pub use registry::{LlmHub, LlmHubBuilder, RoutingStrategy};
pub use cache::{ModelCache, CacheConfig, CacheStats, CacheEntry};
pub use distributed::{
    DistributedCluster, DistributedClient, DistributedConfig,
    NodeConfig, NodeInfo, NodeStatus, NodeStats,
    LoadBalanceStrategy, ShardingStrategy,
};
pub use router::{SmartRouter, ComplexityTier, RoutingDecision, ModelTier, RouterConfig, RouterStats};

// ═══════════════════════════════════════════════════════════════════════════════
//  RE-EXPORT: DIRECT PROVIDERS
// ═══════════════════════════════════════════════════════════════════════════════

pub use providers::{
    OpenAIProvider, AnthropicProvider, GoogleProvider, MistralProvider,
    DeepSeekProvider, XAIProvider, CohereProvider, PerplexityProvider,
    GroqProvider, TogetherProvider, FireworksProvider, ReplicateProvider,
    AI21Provider, OllamaProvider,
};

// ═══════════════════════════════════════════════════════════════════════════════
//  RE-EXPORT: AGGREGATOR PROVIDERS
// ═══════════════════════════════════════════════════════════════════════════════

pub use providers::{
    OpenRouterProvider, GlhfProvider, NovitaProvider, HyperbolicProvider,
    SiliconFlowProvider, CerebrasProvider, LiteLLMProvider, HuggingFaceProvider,
    CloudflareAIProvider, ChutesProvider,
};

// ═══════════════════════════════════════════════════════════════════════════════
//  RE-EXPORT: ENTERPRISE PROVIDERS
// ═══════════════════════════════════════════════════════════════════════════════

pub use providers::{
    NvidiaProvider, SambaNovaProvider, DeepInfraProvider,
    AzureOpenAIProvider, BedrockProvider, VertexAIProvider,
};

// ═══════════════════════════════════════════════════════════════════════════════
//  RE-EXPORT: LOCAL PROVIDERS
// ═══════════════════════════════════════════════════════════════════════════════

pub use providers::{
    VLLMProvider, LmStudioProvider, LlamafileProvider,
};

// ═══════════════════════════════════════════════════════════════════════════════
//  RE-EXPORT: CHINESE AI PROVIDERS
// ═══════════════════════════════════════════════════════════════════════════════

pub use providers::{
    ZhipuProvider, MoonshotProvider, YiProvider,
    BaiduErnieProvider, MiniMaxProvider,
    QwenProvider, StepFunProvider,
};

// ═══════════════════════════════════════════════════════════════════════════════
//  RE-EXPORT: REGIONAL PROVIDERS
// ═══════════════════════════════════════════════════════════════════════════════

pub use providers::{
    GigaChatProvider,      // Russia
    UpstageProvider,       // Korea
    AlephAlphaProvider,    // EU
    SarvamProvider,        // India
};

// ═══════════════════════════════════════════════════════════════════════════════
//  RE-EXPORT: SPECIALIZED PROVIDERS
// ═══════════════════════════════════════════════════════════════════════════════

pub use providers::{
    StabilityProvider, WatsonXProvider, LeptonProvider,
    RunPodProvider, ModalProvider, CharacterAIProvider,
    RekaProvider, FriendliAIProvider, OctoAIProvider, VoyageProvider,
};

// Re-export embedding and reranking (cross-module)
pub mod embedding {
    //! Embedding types and utilities (see sentient_embed crate)
    pub use sentient_embed::{Embedding, EmbeddingModel, EmbeddingHub, EmbeddingRequest};
}

pub mod reranking {
    //! Reranking types and utilities (see sentient_rerank crate)
    pub use sentient_rerank::{RerankDocument, RerankResult, RerankModel, RerankEngine};
}

/// Prelude for common imports
pub mod prelude {
    pub use crate::{LlmHub, ChatRequest, ChatResponse, Message, Role, LlmProvider};
    pub use crate::models;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_models_count() {
        let models = models::all_models();
        assert!(models.len() >= 200, "Should have 200+ models, got {}", models.len());
    }

    #[test]
    fn test_provider_count() {
        let count = models::provider_count();
        assert!(count >= 40, "Should have 40+ unique providers, got {}", count);
    }

    #[test]
    fn test_free_tier_models() {
        let free = models::free_tier();
        assert!(!free.is_empty());
        assert!(free.iter().all(|m| m.free_tier));
    }

    #[test]
    fn test_cheapest_provider() {
        let by_cost = models::by_cost();
        let cheapest = &by_cost[0];
        // DeepSeek, Ollama or free models should be at the top
        assert!(cheapest.input_cost_per_1k <= 0.0001 || cheapest.provider == "Ollama");
    }

    #[test]
    fn test_reasoning_models() {
        let reasoning = models::reasoning_models();
        assert!(reasoning.len() >= 10, "Should have 10+ reasoning models");
        assert!(reasoning.iter().any(|m| m.id.contains("o1") || m.id.contains("r1") || m.id.contains("qwq")));
    }

    #[test]
    fn test_vision_models() {
        let vision = models::vision_models();
        assert!(!vision.is_empty());
        assert!(vision.iter().all(|m| m.supports_vision));
    }

    #[test]
    fn test_legacy_models() {
        let legacy = models::legacy_models();
        assert!(!legacy.is_empty());
        assert!(legacy.iter().any(|m| m.id.contains("gpt-2") || m.id.contains("gpt-3")));
    }

    #[test]
    fn test_cutting_edge() {
        let cutting = models::cutting_edge_models();
        assert!(!cutting.is_empty());
    }

    #[test]
    fn test_chinese_models() {
        let chinese = models::chinese_models();
        assert!(chinese.len() >= 5);
    }

    #[test]
    fn test_enterprise_models() {
        let enterprise = models::enterprise_models();
        assert!(enterprise.len() >= 5);
    }

    #[test]
    fn test_local_models_all_free() {
        let local = models::local_models();
        assert!(local.iter().all(|m| m.input_cost_per_1k == 0.0));
    }
}
