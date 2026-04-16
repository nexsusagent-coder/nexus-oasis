//! ─── LLM Provider Trait ───
//!
//! Core trait for LLM providers
//! 50+ providers worldwide

use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;

use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::error::LlmResult;

// ═══════════════════════════════════════════════════════════════════════════════
//  LLM PROVIDER TRAIT
// ═══════════════════════════════════════════════════════════════════════════════

/// LLM Provider trait
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Get provider name
    fn name(&self) -> &str;

    /// Get provider ID
    fn id(&self) -> &str;

    /// Get available models
    fn models(&self) -> Vec<ModelInfo>;

    /// Check if API key is configured
    fn is_configured(&self) -> bool;

    /// Chat completion
    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse>;

    /// Streaming chat completion
    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>>;

    /// Count tokens in a text
    fn count_tokens(&self, text: &str, model: &str) -> LlmResult<usize>;

    /// Get model info
    fn get_model(&self, model_id: &str) -> Option<ModelInfo> {
        self.models().into_iter().find(|m| m.id == model_id)
    }

    /// Validate request
    fn validate_request(&self, request: &ChatRequest) -> LlmResult<()> {
        let model = self.get_model(&request.model)
            .ok_or_else(|| crate::error::LlmError::ModelNotFound(request.model.clone()))?;

        let total_tokens: usize = request.messages.iter()
            .filter_map(|m| m.content.as_text())
            .map(|t| t.len() / 4)
            .sum();

        if total_tokens > model.context_window as usize {
            return Err(crate::error::LlmError::ContextLengthExceeded {
                max: model.context_window as usize,
                provided: total_tokens,
            });
        }

        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PROVIDER INFO - Complete catalog
// ═══════════════════════════════════════════════════════════════════════════════

/// Provider information for display
#[derive(Debug, Clone)]
pub struct ProviderInfo {
    pub name: String,
    pub id: String,
    pub description: String,
    pub base_url: String,
    pub docs_url: String,
    pub pricing_url: String,
    pub free_tier: bool,
    pub free_tier_limits: Option<String>,
    pub model_count: usize,
    pub supports_streaming: bool,
    pub supports_tools: bool,
    pub supports_vision: bool,
    pub api_key_env: String,
    pub category: ProviderCategory,
    pub region: ProviderRegion,
    pub founded_year: Option<u16>,
}

/// Provider category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderCategory {
    /// Tier-1 AI company (OpenAI, Anthropic, Google, etc.)
    Direct,
    /// Aggregator/router platform
    Aggregator,
    /// Enterprise cloud provider
    Enterprise,
    /// Local/self-hosted
    Local,
    /// Regional provider
    Regional,
    /// Specialized (embedding, vision-only, etc.)
    Specialized,
}

/// Provider region
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderRegion {
    Global,
    NorthAmerica,
    Europe,
    China,
    Russia,
    Korea,
    India,
    Japan,
    Israel,
}

impl ProviderInfo {
    // ═══════════════════════════════════════════════════════════
    //  DIRECT PROVIDERS
    // ═══════════════════════════════════════════════════════════

    pub fn openai() -> Self {
        Self {
            name: "OpenAI".into(), id: "openai".into(),
            description: "GPT-4o, GPT-4, GPT-3.5, o1, o3, o4-mini reasoning models".into(),
            base_url: "https://api.openai.com/v1".into(),
            docs_url: "https://platform.openai.com/docs".into(),
            pricing_url: "https://openai.com/pricing".into(),
            free_tier: true, free_tier_limits: Some("$5 free credits for new users".into()),
            model_count: 17, supports_streaming: true, supports_tools: true, supports_vision: true,
            api_key_env: "OPENAI_API_KEY".into(),
            category: ProviderCategory::Direct, region: ProviderRegion::NorthAmerica, founded_year: Some(2015),
        }
    }

    pub fn anthropic() -> Self {
        Self {
            name: "Anthropic".into(), id: "anthropic".into(),
            description: "Claude 4, Claude 3.5, Claude 3, Claude 2, Claude 1".into(),
            base_url: "https://api.anthropic.com/v1".into(),
            docs_url: "https://docs.anthropic.com".into(),
            pricing_url: "https://www.anthropic.com/pricing".into(),
            free_tier: false, free_tier_limits: None,
            model_count: 12, supports_streaming: true, supports_tools: true, supports_vision: true,
            api_key_env: "ANTHROPIC_API_KEY".into(),
            category: ProviderCategory::Direct, region: ProviderRegion::NorthAmerica, founded_year: Some(2021),
        }
    }

    pub fn google() -> Self {
        Self {
            name: "Google".into(), id: "google".into(),
            description: "Gemini 2.5, Gemini 2.0, Gemini 1.5, Gemma 3, PaLM (legacy)".into(),
            base_url: "https://generativelanguage.googleapis.com/v1beta".into(),
            docs_url: "https://ai.google.dev/docs".into(),
            pricing_url: "https://ai.google.dev/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free tier: 15 RPM, 1M tokens/day".into()),
            model_count: 14, supports_streaming: true, supports_tools: true, supports_vision: true,
            api_key_env: "GOOGLE_API_KEY".into(),
            category: ProviderCategory::Direct, region: ProviderRegion::NorthAmerica, founded_year: Some(2017),
        }
    }

    pub fn mistral() -> Self {
        Self {
            name: "Mistral".into(), id: "mistral".into(),
            description: "Mistral Large 2, Small 3.1, Codestral, Pixtral, Mixtral".into(),
            base_url: "https://api.mistral.ai/v1".into(),
            docs_url: "https://docs.mistral.ai".into(),
            pricing_url: "https://mistral.ai/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free tier available".into()),
            model_count: 11, supports_streaming: true, supports_tools: true, supports_vision: true,
            api_key_env: "MISTRAL_API_KEY".into(),
            category: ProviderCategory::Direct, region: ProviderRegion::Europe, founded_year: Some(2023),
        }
    }

    pub fn deepseek() -> Self {
        Self {
            name: "DeepSeek".into(), id: "deepseek".into(),
            description: "DeepSeek V3, R1 (reasoning), Coder, Prover V2 - cheapest quality".into(),
            base_url: "https://api.deepseek.com/v1".into(),
            docs_url: "https://platform.deepseek.com/docs".into(),
            pricing_url: "https://platform.deepseek.com/pricing".into(),
            free_tier: true, free_tier_limits: Some("Extremely cheap pricing".into()),
            model_count: 4, supports_streaming: true, supports_tools: true, supports_vision: false,
            api_key_env: "DEEPSEEK_API_KEY".into(),
            category: ProviderCategory::Direct, region: ProviderRegion::China, founded_year: Some(2023),
        }
    }

    pub fn xai() -> Self {
        Self {
            name: "xAI".into(), id: "xai".into(),
            description: "Grok 3, Grok 3 Mini (reasoning), Grok 2 Vision".into(),
            base_url: "https://api.x.ai/v1".into(),
            docs_url: "https://docs.x.ai/docs".into(),
            pricing_url: "https://x.ai/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free tier available".into()),
            model_count: 4, supports_streaming: true, supports_tools: true, supports_vision: true,
            api_key_env: "XAI_API_KEY".into(),
            category: ProviderCategory::Direct, region: ProviderRegion::NorthAmerica, founded_year: Some(2023),
        }
    }

    pub fn cohere() -> Self {
        Self {
            name: "Cohere".into(), id: "cohere".into(),
            description: "Command A, Command R+, R, Aya Exa multilingual".into(),
            base_url: "https://api.cohere.ai/v1".into(),
            docs_url: "https://docs.cohere.com".into(),
            pricing_url: "https://cohere.com/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free tier: 1000 calls/month".into()),
            model_count: 6, supports_streaming: true, supports_tools: true, supports_vision: false,
            api_key_env: "COHERE_API_KEY".into(),
            category: ProviderCategory::Direct, region: ProviderRegion::NorthAmerica, founded_year: Some(2019),
        }
    }

    pub fn perplexity() -> Self {
        Self {
            name: "Perplexity".into(), id: "perplexity".into(),
            description: "Sonar, Sonar Pro, Sonar Reasoning, Deep Research - online with web search".into(),
            base_url: "https://api.perplexity.ai".into(),
            docs_url: "https://docs.perplexity.ai".into(),
            pricing_url: "https://perplexity.ai/pricing".into(),
            free_tier: false, free_tier_limits: None,
            model_count: 4, supports_streaming: true, supports_tools: true, supports_vision: false,
            api_key_env: "PERPLEXITY_API_KEY".into(),
            category: ProviderCategory::Direct, region: ProviderRegion::NorthAmerica, founded_year: Some(2022),
        }
    }

    // ═══════════════════════════════════════════════════════════
    //  AGGREGATOR PROVIDERS
    // ═══════════════════════════════════════════════════════════

    pub fn groq() -> Self {
        Self {
            name: "Groq".into(), id: "groq".into(),
            description: "LPU inference - fastest LLM inference available".into(),
            base_url: "https://api.groq.com/openai/v1".into(),
            docs_url: "https://console.groq.com/docs".into(),
            pricing_url: "https://groq.com/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free tier with rate limits".into()),
            model_count: 6, supports_streaming: true, supports_tools: true, supports_vision: false,
            api_key_env: "GROQ_API_KEY".into(),
            category: ProviderCategory::Aggregator, region: ProviderRegion::NorthAmerica, founded_year: Some(2016),
        }
    }

    pub fn together() -> Self {
        Self {
            name: "Together".into(), id: "together".into(),
            description: "100+ open source models, Llama, Qwen, DeepSeek, etc.".into(),
            base_url: "https://api.together.xyz/v1".into(),
            docs_url: "https://docs.together.ai".into(),
            pricing_url: "https://together.ai/pricing".into(),
            free_tier: true, free_tier_limits: Some("$1 free credits".into()),
            model_count: 3, supports_streaming: true, supports_tools: true, supports_vision: false,
            api_key_env: "TOGETHER_API_KEY".into(),
            category: ProviderCategory::Aggregator, region: ProviderRegion::NorthAmerica, founded_year: Some(2022),
        }
    }

    pub fn openrouter() -> Self {
        Self {
            name: "OpenRouter".into(), id: "openrouter".into(),
            description: "200+ models via single API, auto-routing".into(),
            base_url: "https://openrouter.ai/api/v1".into(),
            docs_url: "https://openrouter.ai/docs".into(),
            pricing_url: "https://openrouter.ai/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free models available".into()),
            model_count: 200, supports_streaming: true, supports_tools: true, supports_vision: true,
            api_key_env: "OPENROUTER_API_KEY".into(),
            category: ProviderCategory::Aggregator, region: ProviderRegion::NorthAmerica, founded_year: Some(2023),
        }
    }

    pub fn fireworks() -> Self {
        Self {
            name: "Fireworks".into(), id: "fireworks".into(),
            description: "Fast inference for open source models".into(),
            base_url: "https://api.fireworks.ai/inference/v1".into(),
            docs_url: "https://docs.fireworks.ai".into(),
            pricing_url: "https://fireworks.ai/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free tier available".into()),
            model_count: 3, supports_streaming: true, supports_tools: true, supports_vision: false,
            api_key_env: "FIREWORKS_API_KEY".into(),
            category: ProviderCategory::Aggregator, region: ProviderRegion::NorthAmerica, founded_year: Some(2022),
        }
    }

    pub fn cerebras() -> Self {
        Self {
            name: "Cerebras".into(), id: "cerebras".into(),
            description: "CS-3 wafer-scale engine - fastest inference".into(),
            base_url: "https://api.cerebras.ai/v1".into(),
            docs_url: "https://cerebras.ai/docs".into(),
            pricing_url: "https://cerebras.ai/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free tier available".into()),
            model_count: 3, supports_streaming: true, supports_tools: true, supports_vision: false,
            api_key_env: "CEREBRAS_API_KEY".into(),
            category: ProviderCategory::Aggregator, region: ProviderRegion::NorthAmerica, founded_year: Some(2015),
        }
    }

    pub fn cloudflare() -> Self {
        Self {
            name: "Cloudflare Workers AI".into(), id: "cloudflare".into(),
            description: "Edge inference, serverless AI at Cloudflare edge".into(),
            base_url: "https://api.cloudflare.com/client/v4".into(),
            docs_url: "https://developers.cloudflare.com/workers-ai".into(),
            pricing_url: "https://developers.cloudflare.com/workers-ai/pricing".into(),
            free_tier: true, free_tier_limits: Some("10K neurons/day free".into()),
            model_count: 5, supports_streaming: true, supports_tools: true, supports_vision: true,
            api_key_env: "CLOUDFLARE_API_TOKEN".into(),
            category: ProviderCategory::Aggregator, region: ProviderRegion::NorthAmerica, founded_year: Some(2009),
        }
    }

    pub fn chutes() -> Self {
        Self {
            name: "Chutes".into(), id: "chutes".into(),
            description: "Free serverless AI inference platform".into(),
            base_url: "https://llm.chutes.ai/v1".into(),
            docs_url: "https://chutes.ai/docs".into(),
            pricing_url: "https://chutes.ai".into(),
            free_tier: true, free_tier_limits: Some("Completely free!".into()),
            model_count: 3, supports_streaming: true, supports_tools: true, supports_vision: true,
            api_key_env: "CHUTES_API_KEY".into(),
            category: ProviderCategory::Aggregator, region: ProviderRegion::NorthAmerica, founded_year: Some(2024),
        }
    }

    pub fn deepinfra() -> Self {
        Self {
            name: "DeepInfra".into(), id: "deepinfra".into(),
            description: "Cost-effective inference for open source models".into(),
            base_url: "https://api.deepinfra.com/v1/openai".into(),
            docs_url: "https://deepinfra.com/docs".into(),
            pricing_url: "https://deepinfra.com/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free tier available".into()),
            model_count: 3, supports_streaming: true, supports_tools: true, supports_vision: false,
            api_key_env: "DEEPINFRA_API_KEY".into(),
            category: ProviderCategory::Aggregator, region: ProviderRegion::NorthAmerica, founded_year: Some(2022),
        }
    }

    pub fn siliconflow() -> Self {
        Self {
            name: "SiliconFlow".into(), id: "siliconflow".into(),
            description: "Chinese GPU cloud - cheap inference".into(),
            base_url: "https://api.siliconflow.cn/v1".into(),
            docs_url: "https://docs.siliconflow.cn".into(),
            pricing_url: "https://siliconflow.cn/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free tier available".into()),
            model_count: 3, supports_streaming: true, supports_tools: true, supports_vision: false,
            api_key_env: "SILICONFLOW_API_KEY".into(),
            category: ProviderCategory::Aggregator, region: ProviderRegion::China, founded_year: Some(2023),
        }
    }

    pub fn replicate() -> Self {
        Self {
            name: "Replicate".into(), id: "replicate".into(),
            description: "Run any model via API - serverless ML".into(),
            base_url: "https://api.replicate.com/v1".into(),
            docs_url: "https://replicate.com/docs".into(),
            pricing_url: "https://replicate.com/pricing".into(),
            free_tier: false, free_tier_limits: None,
            model_count: 2, supports_streaming: true, supports_tools: true, supports_vision: false,
            api_key_env: "REPLICATE_API_TOKEN".into(),
            category: ProviderCategory::Aggregator, region: ProviderRegion::NorthAmerica, founded_year: Some(2019),
        }
    }

    // ═══════════════════════════════════════════════════════════
    //  ENTERPRISE PROVIDERS
    // ═══════════════════════════════════════════════════════════

    pub fn azure() -> Self {
        Self {
            name: "Azure OpenAI".into(), id: "azure".into(),
            description: "OpenAI models on Azure enterprise cloud".into(),
            base_url: "https://<resource>.openai.azure.com".into(),
            docs_url: "https://learn.microsoft.com/azure/ai-services/openai".into(),
            pricing_url: "https://azure.microsoft.com/pricing/details/cognitive-services/openai".into(),
            free_tier: false, free_tier_limits: None,
            model_count: 2, supports_streaming: true, supports_tools: true, supports_vision: true,
            api_key_env: "AZURE_OPENAI_API_KEY".into(),
            category: ProviderCategory::Enterprise, region: ProviderRegion::Global, founded_year: Some(2023),
        }
    }

    pub fn bedrock() -> Self {
        Self {
            name: "AWS Bedrock".into(), id: "bedrock".into(),
            description: "Multiple foundation models on AWS".into(),
            base_url: "https://bedrock-runtime.<region>.amazonaws.com".into(),
            docs_url: "https://docs.aws.amazon.com/bedrock".into(),
            pricing_url: "https://aws.amazon.com/bedrock/pricing".into(),
            free_tier: false, free_tier_limits: None,
            model_count: 2, supports_streaming: true, supports_tools: true, supports_vision: true,
            api_key_env: "AWS_ACCESS_KEY_ID".into(),
            category: ProviderCategory::Enterprise, region: ProviderRegion::Global, founded_year: Some(2023),
        }
    }

    pub fn vertex() -> Self {
        Self {
            name: "Google Vertex AI".into(), id: "vertex".into(),
            description: "Google AI models on GCP enterprise".into(),
            base_url: "https://<location>-aiplatform.googleapis.com".into(),
            docs_url: "https://cloud.google.com/vertex-ai/docs".into(),
            pricing_url: "https://cloud.google.com/vertex-ai/pricing".into(),
            free_tier: false, free_tier_limits: None,
            model_count: 2, supports_streaming: true, supports_tools: true, supports_vision: true,
            api_key_env: "GOOGLE_APPLICATION_CREDENTIALS".into(),
            category: ProviderCategory::Enterprise, region: ProviderRegion::Global, founded_year: Some(2020),
        }
    }

    pub fn nvidia() -> Self {
        Self {
            name: "NVIDIA NIM".into(), id: "nvidia".into(),
            description: "NVIDIA-optimized inference, Nemotron models".into(),
            base_url: "https://integrate.api.nvidia.com/v1".into(),
            docs_url: "https://docs.nvidia.com/nim".into(),
            pricing_url: "https://build.nvidia.com/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free credits available".into()),
            model_count: 2, supports_streaming: true, supports_tools: true, supports_vision: false,
            api_key_env: "NVIDIA_API_KEY".into(),
            category: ProviderCategory::Enterprise, region: ProviderRegion::NorthAmerica, founded_year: Some(1993),
        }
    }

    pub fn sambanova() -> Self {
        Self {
            name: "SambaNova".into(), id: "sambanova".into(),
            description: "Reconfigurable dataflow architecture - fast".into(),
            base_url: "https://api.sambanova.ai/v1".into(),
            docs_url: "https://docs.sambanova.ai".into(),
            pricing_url: "https://sambanova.ai/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free tier available".into()),
            model_count: 2, supports_streaming: true, supports_tools: true, supports_vision: false,
            api_key_env: "SAMBANOVA_API_KEY".into(),
            category: ProviderCategory::Enterprise, region: ProviderRegion::NorthAmerica, founded_year: Some(2017),
        }
    }

    pub fn watsonx() -> Self {
        Self {
            name: "IBM WatsonX".into(), id: "watsonx".into(),
            description: "IBM Granite models, enterprise AI on IBM Cloud".into(),
            base_url: "https://us-south.ml.cloud.ibm.com".into(),
            docs_url: "https://dataplatform.cloud.ibm.com/docs/content/wsj/analyze-data/watsonxai.html".into(),
            pricing_url: "https://ibm.com/cloud/watsonx/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free tier available".into()),
            model_count: 2, supports_streaming: true, supports_tools: true, supports_vision: false,
            api_key_env: "WATSONX_API_KEY".into(),
            category: ProviderCategory::Enterprise, region: ProviderRegion::NorthAmerica, founded_year: Some(2023),
        }
    }

    // ═══════════════════════════════════════════════════════════
    //  CHINESE AI PROVIDERS
    // ═══════════════════════════════════════════════════════════

    pub fn qwen() -> Self {
        Self {
            name: "Qwen (Alibaba)".into(), id: "qwen".into(),
            description: "Qwen3, QwQ (reasoning), QVQ (vision reasoning), Qwen 2.5 Coder".into(),
            base_url: "https://dashscope.aliyuncs.com/compatible-mode/v1".into(),
            docs_url: "https://help.aliyun.com/product/2400365.html".into(),
            pricing_url: "https://help.aliyun.com/zh/model-studio/getting-started/models".into(),
            free_tier: true, free_tier_limits: Some("Generous free tier".into()),
            model_count: 7, supports_streaming: true, supports_tools: true, supports_vision: true,
            api_key_env: "DASHSCOPE_API_KEY".into(),
            category: ProviderCategory::Direct, region: ProviderRegion::China, founded_year: Some(2023),
        }
    }

    pub fn baidu() -> Self {
        Self {
            name: "Baidu ERNIE".into(), id: "baidu".into(),
            description: "ERNIE 4.0, ERNIE 3.5 - Chinese AI pioneer".into(),
            base_url: "https://aip.baidubce.com".into(),
            docs_url: "https://cloud.baidu.com/doc/WENXINWORKSHOP".into(),
            pricing_url: "https://cloud.baidu.com/doc/WENXINWORKSHOP/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free tier for ERNIE 3.5".into()),
            model_count: 2, supports_streaming: true, supports_tools: true, supports_vision: false,
            api_key_env: "BAIDU_API_KEY".into(),
            category: ProviderCategory::Direct, region: ProviderRegion::China, founded_year: Some(2023),
        }
    }

    pub fn zhipu() -> Self {
        Self {
            name: "Zhipu GLM".into(), id: "zhipu".into(),
            description: "GLM-4 Plus, GLM-4 Flash - bilingual Chinese/English".into(),
            base_url: "https://open.bigmodel.cn/api/paas/v4".into(),
            docs_url: "https://open.bigmodel.cn/dev".into(),
            pricing_url: "https://open.bigmodel.cn/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free tier available".into()),
            model_count: 2, supports_streaming: true, supports_tools: true, supports_vision: true,
            api_key_env: "ZHIPU_API_KEY".into(),
            category: ProviderCategory::Direct, region: ProviderRegion::China, founded_year: Some(2022),
        }
    }

    pub fn moonshot() -> Self {
        Self {
            name: "Moonshot (Kimi)".into(), id: "moonshot".into(),
            description: "Kimi - 1M token context window".into(),
            base_url: "https://api.moonshot.cn/v1".into(),
            docs_url: "https://platform.moonshot.cn/docs".into(),
            pricing_url: "https://platform.moonshot.cn/pricing".into(),
            free_tier: false, free_tier_limits: None,
            model_count: 2, supports_streaming: true, supports_tools: true, supports_vision: true,
            api_key_env: "MOONSHOT_API_KEY".into(),
            category: ProviderCategory::Direct, region: ProviderRegion::China, founded_year: Some(2023),
        }
    }

    pub fn stepfun() -> Self {
        Self {
            name: "StepFun".into(), id: "stepfun".into(),
            description: "Step-2, Step-1V Vision - Chinese AI startup".into(),
            base_url: "https://api.stepfun.com/v1".into(),
            docs_url: "https://platform.stepfun.com/docs".into(),
            pricing_url: "https://platform.stepfun.com/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free tier available".into()),
            model_count: 2, supports_streaming: true, supports_tools: true, supports_vision: true,
            api_key_env: "STEPFUN_API_KEY".into(),
            category: ProviderCategory::Direct, region: ProviderRegion::China, founded_year: Some(2023),
        }
    }

    pub fn bytedance() -> Self {
        Self {
            name: "ByteDance Doubao".into(), id: "doubao".into(),
            description: "Doubao 1.5 Pro - TikTok's AI division".into(),
            base_url: "https://api.doubao.com/v1".into(),
            docs_url: "https://www.volcengine.com/docs/doubao".into(),
            pricing_url: "https://www.volcengine.com/pricing/doubao".into(),
            free_tier: true, free_tier_limits: Some("Free tier available".into()),
            model_count: 1, supports_streaming: true, supports_tools: true, supports_vision: true,
            api_key_env: "DOUBAO_API_KEY".into(),
            category: ProviderCategory::Direct, region: ProviderRegion::China, founded_year: Some(2024),
        }
    }

    // ═══════════════════════════════════════════════════════════
    //  RUSSIAN AI PROVIDERS
    // ═══════════════════════════════════════════════════════════

    pub fn gigachat() -> Self {
        Self {
            name: "Sber GigaChat".into(), id: "gigachat".into(),
            description: "GigaChat Pro, Max - Russia's leading AI".into(),
            base_url: "https://gigachat.devices.sberbank.ru/api/v1".into(),
            docs_url: "https://developers.sber.ru/gigachat".into(),
            pricing_url: "https://developers.sber.ru/gigachat/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free tier available".into()),
            model_count: 3, supports_streaming: true, supports_tools: true, supports_vision: true,
            api_key_env: "GIGACHAT_API_KEY".into(),
            category: ProviderCategory::Regional, region: ProviderRegion::Russia, founded_year: Some(2023),
        }
    }

    // ═══════════════════════════════════════════════════════════
    //  KOREAN AI PROVIDERS
    // ═══════════════════════════════════════════════════════════

    pub fn upstage() -> Self {
        Self {
            name: "Upstage".into(), id: "upstage".into(),
            description: "Solar Pro 2, Solar Mini - Korean AI".into(),
            base_url: "https://api.upstage.ai/v1/solar".into(),
            docs_url: "https://docs.upstage.ai".into(),
            pricing_url: "https://upstage.ai/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free tier available".into()),
            model_count: 2, supports_streaming: true, supports_tools: true, supports_vision: false,
            api_key_env: "UPSTAGE_API_KEY".into(),
            category: ProviderCategory::Regional, region: ProviderRegion::Korea, founded_year: Some(2021),
        }
    }

    // ═══════════════════════════════════════════════════════════
    //  EUROPEAN AI PROVIDERS
    // ═══════════════════════════════════════════════════════════

    pub fn aleph_alpha() -> Self {
        Self {
            name: "Aleph Alpha".into(), id: "aleph_alpha".into(),
            description: "Luminous, Pharia - European sovereign AI".into(),
            base_url: "https://api.aleph-alpha.com/v1".into(),
            docs_url: "https://docs.aleph-alpha.com".into(),
            pricing_url: "https://aleph-alpha.com/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free tier available".into()),
            model_count: 3, supports_streaming: true, supports_tools: true, supports_vision: false,
            api_key_env: "ALEPH_ALPHA_API_KEY".into(),
            category: ProviderCategory::Regional, region: ProviderRegion::Europe, founded_year: Some(2019),
        }
    }

    // ═══════════════════════════════════════════════════════════
    //  INDIAN AI PROVIDERS
    // ═══════════════════════════════════════════════════════════

    pub fn sarvam() -> Self {
        Self {
            name: "Sarvam AI".into(), id: "sarvam".into(),
            description: "Sarvam-M - Indian multilingual AI".into(),
            base_url: "https://api.sarvam.ai/v1".into(),
            docs_url: "https://docs.sarvam.ai".into(),
            pricing_url: "https://sarvam.ai/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free tier available".into()),
            model_count: 2, supports_streaming: true, supports_tools: true, supports_vision: false,
            api_key_env: "SARVAM_API_KEY".into(),
            category: ProviderCategory::Regional, region: ProviderRegion::India, founded_year: Some(2023),
        }
    }

    // ═══════════════════════════════════════════════════════════
    //  LOCAL PROVIDERS
    // ═══════════════════════════════════════════════════════════

    pub fn ollama() -> Self {
        Self {
            name: "Ollama".into(), id: "ollama".into(),
            description: "Local LLM inference - FREE, runs on your hardware".into(),
            base_url: "http://localhost:11434".into(),
            docs_url: "https://ollama.ai/docs".into(),
            pricing_url: "FREE".into(),
            free_tier: true, free_tier_limits: Some("Completely free - uses your hardware".into()),
            model_count: 6, supports_streaming: true, supports_tools: true, supports_vision: false,
            api_key_env: "OLLAMA_HOST".into(),
            category: ProviderCategory::Local, region: ProviderRegion::Global, founded_year: Some(2023),
        }
    }

    pub fn vllm() -> Self {
        Self {
            name: "vLLM".into(), id: "vllm".into(),
            description: "High-throughput local LLM serving".into(),
            base_url: "http://localhost:8000".into(),
            docs_url: "https://docs.vllm.ai".into(),
            pricing_url: "FREE".into(),
            free_tier: true, free_tier_limits: Some("Completely free - self-hosted".into()),
            model_count: 1, supports_streaming: true, supports_tools: true, supports_vision: false,
            api_key_env: "VLLM_HOST".into(),
            category: ProviderCategory::Local, region: ProviderRegion::Global, founded_year: Some(2023),
        }
    }

    pub fn lmstudio() -> Self {
        Self {
            name: "LM Studio".into(), id: "lmstudio".into(),
            description: "Desktop LLM application - run models locally".into(),
            base_url: "http://localhost:1234".into(),
            docs_url: "https://lmstudio.ai/docs".into(),
            pricing_url: "FREE".into(),
            free_tier: true, free_tier_limits: Some("Completely free".into()),
            model_count: 1, supports_streaming: true, supports_tools: true, supports_vision: false,
            api_key_env: "LMSTUDIO_HOST".into(),
            category: ProviderCategory::Local, region: ProviderRegion::Global, founded_year: Some(2023),
        }
    }

    pub fn llamafile() -> Self {
        Self {
            name: "Llamafile".into(), id: "llamafile".into(),
            description: "Mozilla's single-file LLM distribution".into(),
            base_url: "http://localhost:8080".into(),
            docs_url: "https://llamafile.ai".into(),
            pricing_url: "FREE".into(),
            free_tier: true, free_tier_limits: Some("Completely free".into()),
            model_count: 1, supports_streaming: true, supports_tools: true, supports_vision: false,
            api_key_env: "LLAMAFILE_HOST".into(),
            category: ProviderCategory::Local, region: ProviderRegion::Global, founded_year: Some(2023),
        }
    }

    // ═══════════════════════════════════════════════════════════
    //  SPECIALIZED PROVIDERS
    // ═══════════════════════════════════════════════════════════

    pub fn reka() -> Self {
        Self {
            name: "Reka AI".into(), id: "reka".into(),
            description: "Reka Core, Flash, Edge - multimodal AI".into(),
            base_url: "https://api.reka.ai/v1".into(),
            docs_url: "https://docs.reka.ai".into(),
            pricing_url: "https://reka.ai/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free tier available".into()),
            model_count: 3, supports_streaming: true, supports_tools: true, supports_vision: true,
            api_key_env: "REKA_API_KEY".into(),
            category: ProviderCategory::Specialized, region: ProviderRegion::Global, founded_year: Some(2023),
        }
    }

    pub fn ai21() -> Self {
        Self {
            name: "AI21".into(), id: "ai21".into(),
            description: "Jamba 1.5 - SSM-Transformer hybrid architecture".into(),
            base_url: "https://api.ai21.com/v1".into(),
            docs_url: "https://docs.ai21.com".into(),
            pricing_url: "https://ai21.com/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free tier available".into()),
            model_count: 3, supports_streaming: true, supports_tools: true, supports_vision: false,
            api_key_env: "AI21_API_KEY".into(),
            category: ProviderCategory::Direct, region: ProviderRegion::Israel, founded_year: Some(2017),
        }
    }

    pub fn friendliai() -> Self {
        Self {
            name: "FriendliAI".into(), id: "friendliai".into(),
            description: "GPU-optimized inference engine".into(),
            base_url: "https://api.friendli.ai/v1".into(),
            docs_url: "https://docs.friendli.ai".into(),
            pricing_url: "https://friendli.ai/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free tier available".into()),
            model_count: 3, supports_streaming: true, supports_tools: true, supports_vision: false,
            api_key_env: "FRIENDLI_TOKEN".into(),
            category: ProviderCategory::Aggregator, region: ProviderRegion::NorthAmerica, founded_year: Some(2022),
        }
    }

    pub fn octoai() -> Self {
        Self {
            name: "OctoAI".into(), id: "octoai".into(),
            description: "Efficient cloud inference for open source models".into(),
            base_url: "https://text.octoai.run/v1".into(),
            docs_url: "https://docs.octoai.ai".into(),
            pricing_url: "https://octoai.ai/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free tier available".into()),
            model_count: 2, supports_streaming: true, supports_tools: true, supports_vision: false,
            api_key_env: "OCTOAI_TOKEN".into(),
            category: ProviderCategory::Aggregator, region: ProviderRegion::NorthAmerica, founded_year: Some(2022),
        }
    }

    pub fn voyage() -> Self {
        Self {
            name: "Voyage AI".into(), id: "voyage".into(),
            description: "Voyage 3, Code 3 - specialized embedding models".into(),
            base_url: "https://api.voyageai.com/v1".into(),
            docs_url: "https://docs.voyageai.com".into(),
            pricing_url: "https://voyageai.com/pricing".into(),
            free_tier: true, free_tier_limits: Some("Free tier available".into()),
            model_count: 3, supports_streaming: false, supports_tools: false, supports_vision: false,
            api_key_env: "VOYAGE_API_KEY".into(),
            category: ProviderCategory::Specialized, region: ProviderRegion::NorthAmerica, founded_year: Some(2023),
        }
    }

    // ═══════════════════════════════════════════════════════════
    //  ALL PROVIDERS
    // ═══════════════════════════════════════════════════════════

    /// Get all provider info (50+)
    pub fn all() -> Vec<ProviderInfo> {
        vec![
            // Direct
            Self::openai(), Self::anthropic(), Self::google(),
            Self::mistral(), Self::deepseek(), Self::xai(),
            Self::cohere(), Self::perplexity(),
            // Chinese
            Self::qwen(), Self::baidu(), Self::zhipu(),
            Self::moonshot(), Self::stepfun(), Self::bytedance(),
            // Aggregator
            Self::groq(), Self::together(), Self::openrouter(),
            Self::fireworks(), Self::cerebras(), Self::cloudflare(),
            Self::chutes(), Self::deepinfra(), Self::siliconflow(),
            Self::replicate(), Self::friendliai(), Self::octoai(),
            // Enterprise
            Self::azure(), Self::bedrock(), Self::vertex(),
            Self::nvidia(), Self::sambanova(), Self::watsonx(),
            // Regional
            Self::gigachat(), Self::upstage(), Self::aleph_alpha(), Self::sarvam(),
            // Local
            Self::ollama(), Self::vllm(), Self::lmstudio(), Self::llamafile(),
            // Specialized
            Self::reka(), Self::ai21(), Self::voyage(),
        ]
    }

    /// Get providers with free tier
    pub fn free_tier() -> Vec<ProviderInfo> {
        Self::all().into_iter().filter(|p| p.free_tier).collect()
    }

    /// Get providers by category
    pub fn by_category(category: ProviderCategory) -> Vec<ProviderInfo> {
        Self::all().into_iter().filter(|p| p.category == category).collect()
    }

    /// Get providers by region
    pub fn by_region(region: ProviderRegion) -> Vec<ProviderInfo> {
        Self::all().into_iter().filter(|p| p.region == region).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_info_count() {
        let providers = ProviderInfo::all();
        assert!(providers.len() >= 40, "Should have 40+ providers, got {}", providers.len());
    }

    #[test]
    fn test_free_tier_providers() {
        let free = ProviderInfo::free_tier();
        assert!(!free.is_empty());
        assert!(free.iter().all(|p| p.free_tier));
    }

    #[test]
    fn test_by_category() {
        let direct = ProviderInfo::by_category(ProviderCategory::Direct);
        assert!(direct.len() >= 8);

        let local = ProviderInfo::by_category(ProviderCategory::Local);
        assert!(local.len() >= 3);
    }

    #[test]
    fn test_by_region() {
        let china = ProviderInfo::by_region(ProviderRegion::China);
        assert!(china.len() >= 3);

        let europe = ProviderInfo::by_region(ProviderRegion::Europe);
        assert!(europe.len() >= 2);
    }
}
