//! ─── LLM Models Registry ───
//!
//! Complete registry of all LLM models across providers
//! Updated: 2025

use crate::types::ModelInfo;

// ═══════════════════════════════════════════════════════════════════════════════
//  OPENAI MODELS
// ═══════════════════════════════════════════════════════════════════════════════

/// OpenAI models
pub fn openai_models() -> Vec<ModelInfo> {
    vec![
        // GPT-4o family (latest)
        ModelInfo {
            id: "gpt-4o".into(),
            name: "GPT-4o".into(),
            provider: "OpenAI".into(),
            context_window: 128_000,
            max_output_tokens: 16_384,
            input_cost_per_1k: 0.0025,
            output_cost_per_1k: 0.01,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-04".into()),
            quality_rating: 5,
            speed_rating: 4,
            is_reasoning: false,
            free_tier: true,
        },
        ModelInfo {
            id: "gpt-4o-mini".into(),
            name: "GPT-4o Mini".into(),
            provider: "OpenAI".into(),
            context_window: 128_000,
            max_output_tokens: 16_384,
            input_cost_per_1k: 0.00015,
            output_cost_per_1k: 0.0006,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2023-10".into()),
            quality_rating: 4,
            speed_rating: 5,
            is_reasoning: false,
            free_tier: true,
        },
        ModelInfo {
            id: "gpt-4o-audio-preview".into(),
            name: "GPT-4o Audio".into(),
            provider: "OpenAI".into(),
            context_window: 128_000,
            max_output_tokens: 16_384,
            input_cost_per_1k: 0.0025,
            output_cost_per_1k: 0.01,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-04".into()),
            quality_rating: 5,
            speed_rating: 4,
            is_reasoning: false,
            free_tier: false,
        },
        // GPT-4 Turbo
        ModelInfo {
            id: "gpt-4-turbo".into(),
            name: "GPT-4 Turbo".into(),
            provider: "OpenAI".into(),
            context_window: 128_000,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.01,
            output_cost_per_1k: 0.03,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2023-12".into()),
            quality_rating: 5,
            speed_rating: 3,
            is_reasoning: false,
            free_tier: false,
        },
        // GPT-3.5
        ModelInfo {
            id: "gpt-3.5-turbo".into(),
            name: "GPT-3.5 Turbo".into(),
            provider: "OpenAI".into(),
            context_window: 16_385,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.0005,
            output_cost_per_1k: 0.0015,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2021-09".into()),
            quality_rating: 3,
            speed_rating: 5,
            is_reasoning: false,
            free_tier: true,
        },
        // Reasoning models (o-series)
        ModelInfo {
            id: "o1".into(),
            name: "o1".into(),
            provider: "OpenAI".into(),
            context_window: 200_000,
            max_output_tokens: 100_000,
            input_cost_per_1k: 0.015,
            output_cost_per_1k: 0.06,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: false,
            supports_json: false,
            training_cutoff: Some("2024-06".into()),
            quality_rating: 5,
            speed_rating: 2,
            is_reasoning: true,
            free_tier: false,
        },
        ModelInfo {
            id: "o1-mini".into(),
            name: "o1 Mini".into(),
            provider: "OpenAI".into(),
            context_window: 128_000,
            max_output_tokens: 65_536,
            input_cost_per_1k: 0.0011,
            output_cost_per_1k: 0.0044,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: false,
            supports_json: false,
            training_cutoff: Some("2024-06".into()),
            quality_rating: 4,
            speed_rating: 4,
            is_reasoning: true,
            free_tier: false,
        },
        ModelInfo {
            id: "o1-pro".into(),
            name: "o1 Pro".into(),
            provider: "OpenAI".into(),
            context_window: 200_000,
            max_output_tokens: 100_000,
            input_cost_per_1k: 0.15,
            output_cost_per_1k: 0.60,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: false,
            supports_json: false,
            training_cutoff: Some("2024-06".into()),
            quality_rating: 5,
            speed_rating: 1,
            is_reasoning: true,
            free_tier: false,
        },
        ModelInfo {
            id: "o3-mini".into(),
            name: "o3 Mini".into(),
            provider: "OpenAI".into(),
            context_window: 200_000,
            max_output_tokens: 100_000,
            input_cost_per_1k: 0.0011,
            output_cost_per_1k: 0.0044,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            supports_json: false,
            training_cutoff: Some("2024-10".into()),
            quality_rating: 5,
            speed_rating: 4,
            is_reasoning: true,
            free_tier: false,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ANTHROPIC MODELS
// ═══════════════════════════════════════════════════════════════════════════════

/// Anthropic Claude models
pub fn anthropic_models() -> Vec<ModelInfo> {
    vec![
        // Claude 4 (latest)
        ModelInfo {
            id: "claude-opus-4-20250514".into(),
            name: "Claude Opus 4".into(),
            provider: "Anthropic".into(),
            context_window: 200_000,
            max_output_tokens: 32_000,
            input_cost_per_1k: 0.015,
            output_cost_per_1k: 0.075,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2025-02".into()),
            quality_rating: 5,
            speed_rating: 3,
            is_reasoning: false,
            free_tier: false,
        },
        ModelInfo {
            id: "claude-sonnet-4-20250514".into(),
            name: "Claude Sonnet 4".into(),
            provider: "Anthropic".into(),
            context_window: 200_000,
            max_output_tokens: 16_000,
            input_cost_per_1k: 0.003,
            output_cost_per_1k: 0.015,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2025-02".into()),
            quality_rating: 5,
            speed_rating: 4,
            is_reasoning: false,
            free_tier: false,
        },
        // Claude 3.5
        ModelInfo {
            id: "claude-3-5-sonnet-20241022".into(),
            name: "Claude 3.5 Sonnet (v2)".into(),
            provider: "Anthropic".into(),
            context_window: 200_000,
            max_output_tokens: 8_192,
            input_cost_per_1k: 0.003,
            output_cost_per_1k: 0.015,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-07".into()),
            quality_rating: 5,
            speed_rating: 5,
            is_reasoning: false,
            free_tier: false,
        },
        ModelInfo {
            id: "claude-3-5-haiku-20241022".into(),
            name: "Claude 3.5 Haiku".into(),
            provider: "Anthropic".into(),
            context_window: 200_000,
            max_output_tokens: 8_192,
            input_cost_per_1k: 0.001,
            output_cost_per_1k: 0.005,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-07".into()),
            quality_rating: 4,
            speed_rating: 5,
            is_reasoning: false,
            free_tier: false,
        },
        // Claude 3
        ModelInfo {
            id: "claude-3-opus-20240229".into(),
            name: "Claude 3 Opus".into(),
            provider: "Anthropic".into(),
            context_window: 200_000,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.015,
            output_cost_per_1k: 0.075,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2023-08".into()),
            quality_rating: 5,
            speed_rating: 2,
            is_reasoning: false,
            free_tier: false,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  GOOGLE MODELS
// ═══════════════════════════════════════════════════════════════════════════════

/// Google Gemini models
pub fn google_models() -> Vec<ModelInfo> {
    vec![
        // Gemini 2.0
        ModelInfo {
            id: "gemini-2.0-flash".into(),
            name: "Gemini 2.0 Flash".into(),
            provider: "Google".into(),
            context_window: 1_048_576,
            max_output_tokens: 8_192,
            input_cost_per_1k: 0.0001,
            output_cost_per_1k: 0.0004,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-06".into()),
            quality_rating: 4,
            speed_rating: 5,
            is_reasoning: false,
            free_tier: true,
        },
        ModelInfo {
            id: "gemini-2.0-pro-exp-02-05".into(),
            name: "Gemini 2.0 Pro".into(),
            provider: "Google".into(),
            context_window: 1_048_576,
            max_output_tokens: 8_192,
            input_cost_per_1k: 0.00125,
            output_cost_per_1k: 0.005,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-06".into()),
            quality_rating: 5,
            speed_rating: 4,
            is_reasoning: false,
            free_tier: true,
        },
        // Gemini 1.5
        ModelInfo {
            id: "gemini-1.5-pro".into(),
            name: "Gemini 1.5 Pro".into(),
            provider: "Google".into(),
            context_window: 2_097_152,
            max_output_tokens: 8_192,
            input_cost_per_1k: 0.00125,
            output_cost_per_1k: 0.005,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-01".into()),
            quality_rating: 5,
            speed_rating: 3,
            is_reasoning: false,
            free_tier: true,
        },
        ModelInfo {
            id: "gemini-1.5-flash".into(),
            name: "Gemini 1.5 Flash".into(),
            provider: "Google".into(),
            context_window: 1_048_576,
            max_output_tokens: 8_192,
            input_cost_per_1k: 0.000075,
            output_cost_per_1k: 0.0003,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-01".into()),
            quality_rating: 4,
            speed_rating: 5,
            is_reasoning: false,
            free_tier: true,
        },
        // Gemma (open)
        ModelInfo {
            id: "gemma-2-27b-it".into(),
            name: "Gemma 2 27B".into(),
            provider: "Google".into(),
            context_window: 8_192,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.00008,
            output_cost_per_1k: 0.00008,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-06".into()),
            quality_rating: 4,
            speed_rating: 4,
            is_reasoning: false,
            free_tier: true,
        },
        ModelInfo {
            id: "gemma-2-9b-it".into(),
            name: "Gemma 2 9B".into(),
            provider: "Google".into(),
            context_window: 8_192,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.00003,
            output_cost_per_1k: 0.00003,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-06".into()),
            quality_rating: 3,
            speed_rating: 5,
            is_reasoning: false,
            free_tier: true,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  MISTRAL MODELS
// ═══════════════════════════════════════════════════════════════════════════════

/// Mistral AI models
pub fn mistral_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "mistral-large-latest".into(),
            name: "Mistral Large 2".into(),
            provider: "Mistral".into(),
            context_window: 128_000,
            max_output_tokens: 8_192,
            input_cost_per_1k: 0.002,
            output_cost_per_1k: 0.006,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-07".into()),
            quality_rating: 5,
            speed_rating: 4,
            is_reasoning: false,
            free_tier: false,
        },
        ModelInfo {
            id: "mistral-medium-latest".into(),
            name: "Mistral Medium".into(),
            provider: "Mistral".into(),
            context_window: 32_000,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.0007,
            output_cost_per_1k: 0.0021,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-01".into()),
            quality_rating: 4,
            speed_rating: 4,
            is_reasoning: false,
            free_tier: false,
        },
        ModelInfo {
            id: "mistral-small-latest".into(),
            name: "Mistral Small".into(),
            provider: "Mistral".into(),
            context_window: 32_000,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.0001,
            output_cost_per_1k: 0.0003,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-01".into()),
            quality_rating: 3,
            speed_rating: 5,
            is_reasoning: false,
            free_tier: true,
        },
        ModelInfo {
            id: "codestral-latest".into(),
            name: "Codestral".into(),
            provider: "Mistral".into(),
            context_window: 32_000,
            max_output_tokens: 8_192,
            input_cost_per_1k: 0.0002,
            output_cost_per_1k: 0.0006,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-03".into()),
            quality_rating: 4,
            speed_rating: 5,
            is_reasoning: false,
            free_tier: false,
        },
        // Open models
        ModelInfo {
            id: "open-mixtral-8x22b".into(),
            name: "Mixtral 8x22B".into(),
            provider: "Mistral".into(),
            context_window: 65_536,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.00065,
            output_cost_per_1k: 0.00065,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-01".into()),
            quality_rating: 4,
            speed_rating: 4,
            is_reasoning: false,
            free_tier: true,
        },
        ModelInfo {
            id: "open-mixtral-8x7b".into(),
            name: "Mixtral 8x7B".into(),
            provider: "Mistral".into(),
            context_window: 32_000,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.00024,
            output_cost_per_1k: 0.00024,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2023-12".into()),
            quality_rating: 3,
            speed_rating: 5,
            is_reasoning: false,
            free_tier: true,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DEEPSEEK MODELS
// ═══════════════════════════════════════════════════════════════════════════════

/// DeepSeek models
pub fn deepseek_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "deepseek-chat".into(),
            name: "DeepSeek V3".into(),
            provider: "DeepSeek".into(),
            context_window: 64_000,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.00007,  // Extremely cheap!
            output_cost_per_1k: 0.00028,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-07".into()),
            quality_rating: 5,
            speed_rating: 5,
            is_reasoning: false,
            free_tier: true,
        },
        ModelInfo {
            id: "deepseek-reasoner".into(),
            name: "DeepSeek R1".into(),
            provider: "DeepSeek".into(),
            context_window: 64_000,
            max_output_tokens: 8_192,
            input_cost_per_1k: 0.00055,
            output_cost_per_1k: 0.00219,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-11".into()),
            quality_rating: 5,
            speed_rating: 3,
            is_reasoning: true,
            free_tier: true,
        },
        ModelInfo {
            id: "deepseek-coder".into(),
            name: "DeepSeek Coder V2".into(),
            provider: "DeepSeek".into(),
            context_window: 128_000,
            max_output_tokens: 16_384,
            input_cost_per_1k: 0.00014,
            output_cost_per_1k: 0.00028,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-06".into()),
            quality_rating: 5,
            speed_rating: 5,
            is_reasoning: false,
            free_tier: true,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  XAI (GROK) MODELS
// ═══════════════════════════════════════════════════════════════════════════════

/// xAI (Grok) models
pub fn xai_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "grok-2-latest".into(),
            name: "Grok 2".into(),
            provider: "xAI".into(),
            context_window: 131_072,
            max_output_tokens: 8_192,
            input_cost_per_1k: 0.002,
            output_cost_per_1k: 0.01,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-08".into()),
            quality_rating: 5,
            speed_rating: 4,
            is_reasoning: false,
            free_tier: false,
        },
        ModelInfo {
            id: "grok-2-vision-latest".into(),
            name: "Grok 2 Vision".into(),
            provider: "xAI".into(),
            context_window: 32_768,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.002,
            output_cost_per_1k: 0.01,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-08".into()),
            quality_rating: 5,
            speed_rating: 4,
            is_reasoning: false,
            free_tier: false,
        },
        ModelInfo {
            id: "grok-beta".into(),
            name: "Grok Beta".into(),
            provider: "xAI".into(),
            context_window: 131_072,
            max_output_tokens: 8_192,
            input_cost_per_1k: 0.005,
            output_cost_per_1k: 0.015,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-03".into()),
            quality_rating: 4,
            speed_rating: 4,
            is_reasoning: false,
            free_tier: false,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  COHERE MODELS
// ═══════════════════════════════════════════════════════════════════════════════

/// Cohere models
pub fn cohere_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "command-r-plus-08-2024".into(),
            name: "Command R+".into(),
            provider: "Cohere".into(),
            context_window: 128_000,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.0025,
            output_cost_per_1k: 0.01,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-08".into()),
            quality_rating: 5,
            speed_rating: 3,
            is_reasoning: false,
            free_tier: true,
        },
        ModelInfo {
            id: "command-r-08-2024".into(),
            name: "Command R".into(),
            provider: "Cohere".into(),
            context_window: 128_000,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.00015,
            output_cost_per_1k: 0.0006,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-08".into()),
            quality_rating: 4,
            speed_rating: 5,
            is_reasoning: false,
            free_tier: true,
        },
        ModelInfo {
            id: "command".into(),
            name: "Command".into(),
            provider: "Cohere".into(),
            context_window: 4_096,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.001,
            output_cost_per_1k: 0.002,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-01".into()),
            quality_rating: 3,
            speed_rating: 4,
            is_reasoning: false,
            free_tier: true,
        },
        ModelInfo {
            id: "command-light".into(),
            name: "Command Light".into(),
            provider: "Cohere".into(),
            context_window: 4_096,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.0003,
            output_cost_per_1k: 0.0006,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-01".into()),
            quality_rating: 2,
            speed_rating: 5,
            is_reasoning: false,
            free_tier: true,
        },
        ModelInfo {
            id: "aya-exa-32b".into(),
            name: "Aya Exa 32B".into(),
            provider: "Cohere".into(),
            context_window: 128_000,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.0005,
            output_cost_per_1k: 0.0015,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-08".into()),
            quality_rating: 4,
            speed_rating: 4,
            is_reasoning: false,
            free_tier: true,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  META LLAMA (via Together, Fireworks, etc.)
// ═══════════════════════════════════════════════════════════════════════════════

/// Meta Llama models (via various providers)
pub fn llama_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "meta-llama/Llama-3.3-70B-Instruct-Turbo".into(),
            name: "Llama 3.3 70B".into(),
            provider: "Together".into(),
            context_window: 128_000,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.00088,
            output_cost_per_1k: 0.00088,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-12".into()),
            quality_rating: 5,
            speed_rating: 4,
            is_reasoning: false,
            free_tier: true,
        },
        ModelInfo {
            id: "meta-llama/Meta-Llama-3.1-405B-Instruct-Turbo".into(),
            name: "Llama 3.1 405B".into(),
            provider: "Together".into(),
            context_window: 131_072,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.005,
            output_cost_per_1k: 0.005,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-07".into()),
            quality_rating: 5,
            speed_rating: 3,
            is_reasoning: false,
            free_tier: false,
        },
        ModelInfo {
            id: "meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo".into(),
            name: "Llama 3.1 8B".into(),
            provider: "Together".into(),
            context_window: 131_072,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.0001,
            output_cost_per_1k: 0.0001,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-07".into()),
            quality_rating: 3,
            speed_rating: 5,
            is_reasoning: false,
            free_tier: true,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PERPLEXITY MODELS
// ═══════════════════════════════════════════════════════════════════════════════

/// Perplexity models (online LLMs with web search)
pub fn perplexity_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "sonar".into(),
            name: "Sonar".into(),
            provider: "Perplexity".into(),
            context_window: 127_000,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.0002,
            output_cost_per_1k: 0.0002,
            supports_vision: false,
            supports_tools: false,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-12".into()),
            quality_rating: 4,
            speed_rating: 4,
            is_reasoning: false,
            free_tier: false,
        },
        ModelInfo {
            id: "sonar-pro".into(),
            name: "Sonar Pro".into(),
            provider: "Perplexity".into(),
            context_window: 200_000,
            max_output_tokens: 8_192,
            input_cost_per_1k: 0.0005,
            output_cost_per_1k: 0.0005,
            supports_vision: false,
            supports_tools: false,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-12".into()),
            quality_rating: 5,
            speed_rating: 3,
            is_reasoning: false,
            free_tier: false,
        },
        ModelInfo {
            id: "sonar-reasoning".into(),
            name: "Sonar Reasoning".into(),
            provider: "Perplexity".into(),
            context_window: 127_000,
            max_output_tokens: 8_192,
            input_cost_per_1k: 0.001,
            output_cost_per_1k: 0.005,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-12".into()),
            quality_rating: 5,
            speed_rating: 3,
            is_reasoning: true,
            free_tier: false,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  GROQ MODELS (LPU - Fastest inference)
// ═══════════════════════════════════════════════════════════════════════════════

/// Groq models (LPU inference - fastest)
pub fn groq_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "llama-3.3-70b-versatile".into(),
            name: "Llama 3.3 70B (Groq)".into(),
            provider: "Groq".into(),
            context_window: 131_072,
            max_output_tokens: 8_192,
            input_cost_per_1k: 0.00059,
            output_cost_per_1k: 0.00079,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-12".into()),
            quality_rating: 5,
            speed_rating: 5,
            is_reasoning: false,
            free_tier: true,
        },
        ModelInfo {
            id: "llama-3.1-8b-instant".into(),
            name: "Llama 3.1 8B (Groq)".into(),
            provider: "Groq".into(),
            context_window: 131_072,
            max_output_tokens: 8_192,
            input_cost_per_1k: 0.0000,
            output_cost_per_1k: 0.0000,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-07".into()),
            quality_rating: 3,
            speed_rating: 5,
            is_reasoning: false,
            free_tier: true,
        },
        ModelInfo {
            id: "mixtral-8x7b-32768".into(),
            name: "Mixtral 8x7B (Groq)".into(),
            provider: "Groq".into(),
            context_window: 32_768,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.00024,
            output_cost_per_1k: 0.00024,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2023-12".into()),
            quality_rating: 4,
            speed_rating: 5,
            is_reasoning: false,
            free_tier: true,
        },
        ModelInfo {
            id: "gemma2-9b-it".into(),
            name: "Gemma 2 9B (Groq)".into(),
            provider: "Groq".into(),
            context_window: 8_192,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.00008,
            output_cost_per_1k: 0.00008,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-06".into()),
            quality_rating: 3,
            speed_rating: 5,
            is_reasoning: false,
            free_tier: true,
        },
        ModelInfo {
            id: "deepseek-r1-distill-llama-70b".into(),
            name: "DeepSeek R1 70B (Groq)".into(),
            provider: "Groq".into(),
            context_window: 131_072,
            max_output_tokens: 8_192,
            input_cost_per_1k: 0.00075,
            output_cost_per_1k: 0.00099,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-11".into()),
            quality_rating: 5,
            speed_rating: 5,
            is_reasoning: true,
            free_tier: true,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  FIREWORKS MODELS
// ═══════════════════════════════════════════════════════════════════════════════

/// Fireworks AI models
pub fn fireworks_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "accounts/fireworks/models/llama-v3p3-70b-instruct".into(),
            name: "Llama 3.3 70B (Fireworks)".into(),
            provider: "Fireworks".into(),
            context_window: 131_072,
            max_output_tokens: 16_384,
            input_cost_per_1k: 0.0009,
            output_cost_per_1k: 0.0009,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-12".into()),
            quality_rating: 5,
            speed_rating: 4,
            is_reasoning: false,
            free_tier: true,
        },
        ModelInfo {
            id: "accounts/fireworks/models/qwen2p5-72b-instruct".into(),
            name: "Qwen 2.5 72B".into(),
            provider: "Fireworks".into(),
            context_window: 131_072,
            max_output_tokens: 8_192,
            input_cost_per_1k: 0.0009,
            output_cost_per_1k: 0.0009,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-09".into()),
            quality_rating: 5,
            speed_rating: 4,
            is_reasoning: false,
            free_tier: true,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  AI21 MODELS
// ═══════════════════════════════════════════════════════════════════════════════

/// AI21 models
pub fn ai21_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "jamba-1-5-large".into(),
            name: "Jamba 1.5 Large".into(),
            provider: "AI21".into(),
            context_window: 256_000,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.002,
            output_cost_per_1k: 0.008,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-07".into()),
            quality_rating: 4,
            speed_rating: 3,
            is_reasoning: false,
            free_tier: false,
        },
        ModelInfo {
            id: "jamba-1-5-mini".into(),
            name: "Jamba 1.5 Mini".into(),
            provider: "AI21".into(),
            context_window: 256_000,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.0002,
            output_cost_per_1k: 0.0004,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-07".into()),
            quality_rating: 3,
            speed_rating: 5,
            is_reasoning: false,
            free_tier: true,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  REPLICATE MODELS
// ═══════════════════════════════════════════════════════════════════════════════

/// Replicate models (various open source)
pub fn replicate_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "meta/llama-3.3-70b-instruct".into(),
            name: "Llama 3.3 70B (Replicate)".into(),
            provider: "Replicate".into(),
            context_window: 128_000,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.0006,
            output_cost_per_1k: 0.0006,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2024-12".into()),
            quality_rating: 5,
            speed_rating: 3,
            is_reasoning: false,
            free_tier: false,
        },
        ModelInfo {
            id: "mistralai/mixtral-8x7b-instruct-v0.1".into(),
            name: "Mixtral 8x7B (Replicate)".into(),
            provider: "Replicate".into(),
            context_window: 32_000,
            max_output_tokens: 4_096,
            input_cost_per_1k: 0.0003,
            output_cost_per_1k: 0.0003,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: Some("2023-12".into()),
            quality_rating: 4,
            speed_rating: 3,
            is_reasoning: false,
            free_tier: false,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ALL MODELS
// ═══════════════════════════════════════════════════════════════════════════════

/// Get all available models
pub fn all_models() -> Vec<ModelInfo> {
    let mut models = Vec::new();
    models.extend(openai_models());
    models.extend(anthropic_models());
    models.extend(google_models());
    models.extend(mistral_models());
    models.extend(deepseek_models());
    models.extend(xai_models());
    models.extend(cohere_models());
    models.extend(llama_models());
    models.extend(perplexity_models());
    models.extend(groq_models());
    models.extend(fireworks_models());
    models.extend(ai21_models());
    models.extend(replicate_models());
    models
}

/// Get models by provider
pub fn by_provider(provider: &str) -> Vec<ModelInfo> {
    all_models()
        .into_iter()
        .filter(|m| m.provider.to_lowercase() == provider.to_lowercase())
        .collect()
}

/// Get models sorted by quality (best first)
pub fn by_quality() -> Vec<ModelInfo> {
    let mut models = all_models();
    models.sort_by(|a, b| b.quality_rating.cmp(&a.quality_rating));
    models
}

/// Get models sorted by speed (fastest first)
pub fn by_speed() -> Vec<ModelInfo> {
    let mut models = all_models();
    models.sort_by(|a, b| b.speed_rating.cmp(&a.speed_rating));
    models
}

/// Get models sorted by cost (cheapest first for input)
pub fn by_cost() -> Vec<ModelInfo> {
    let mut models = all_models();
    models.sort_by(|a, b| a.input_cost_per_1k.partial_cmp(&b.input_cost_per_1k).unwrap());
    models
}

/// Get free tier models only
pub fn free_tier() -> Vec<ModelInfo> {
    all_models()
        .into_iter()
        .filter(|m| m.free_tier)
        .collect()
}

/// Get reasoning models
pub fn reasoning_models() -> Vec<ModelInfo> {
    all_models()
        .into_iter()
        .filter(|m| m.is_reasoning)
        .collect()
}

/// Get models with vision support
pub fn vision_models() -> Vec<ModelInfo> {
    all_models()
        .into_iter()
        .filter(|m| m.supports_vision)
        .collect()
}

/// Find model by ID
pub fn find(id: &str) -> Option<ModelInfo> {
    all_models().into_iter().find(|m| m.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_models_count() {
        let models = all_models();
        assert!(models.len() >= 40, "Should have at least 40 models");
    }

    #[test]
    fn test_openai_models() {
        let models = openai_models();
        assert!(!models.is_empty());
        assert!(models.iter().any(|m| m.id == "gpt-4o"));
    }

    #[test]
    fn test_anthropic_models() {
        let models = anthropic_models();
        assert!(!models.is_empty());
        assert!(models.iter().any(|m| m.id.contains("claude")));
    }

    #[test]
    fn test_find_model() {
        let model = find("gpt-4o");
        assert!(model.is_some());
        assert_eq!(model.unwrap().provider, "OpenAI");
    }

    #[test]
    fn test_by_quality() {
        let models = by_quality();
        assert!(!models.is_empty());
        assert!(models[0].quality_rating >= models[models.len()-1].quality_rating);
    }

    #[test]
    fn test_free_tier() {
        let models = free_tier();
        assert!(!models.is_empty());
        assert!(models.iter().all(|m| m.free_tier));
    }

    #[test]
    fn test_deepseek_cheapest() {
        let models = deepseek_models();
        let deepseek_chat = models.iter().find(|m| m.id == "deepseek-chat");
        assert!(deepseek_chat.is_some());
        assert!(deepseek_chat.unwrap().input_cost_per_1k < 0.0001);
    }
}
