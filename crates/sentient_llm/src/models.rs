//! ─── LLM Models Registry ───
//!
//! Complete registry of ALL LLM models across providers worldwide
//! From oldest (GPT-2 era 2019) to latest (2026)
//! 200+ models, 50+ providers, Free & Paid
//!
//! Last updated: 2026-04-16

use crate::types::ModelInfo;

// ═══════════════════════════════════════════════════════════════════════════════
//  OPENAI MODELS (2019 → 2026)
// ═══════════════════════════════════════════════════════════════════════════════

/// OpenAI models - from GPT-2 to o3
pub fn openai_models() -> Vec<ModelInfo> {
    vec![
        // ── Legacy (Historical) ──
        ModelInfo {
            id: "gpt-2".into(), name: "GPT-2 (Legacy)".into(), provider: "OpenAI".into(),
            context_window: 1_024, max_output_tokens: 1_024,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: false, supports_streaming: true, supports_json: false,
            training_cutoff: Some("2019-02".into()), quality_rating: 1, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "gpt-3".into(), name: "GPT-3 Davinci (Legacy)".into(), provider: "OpenAI".into(),
            context_window: 4_096, max_output_tokens: 4_096,
            input_cost_per_1k: 0.02, output_cost_per_1k: 0.02,
            supports_vision: false, supports_tools: false, supports_streaming: true, supports_json: false,
            training_cutoff: Some("2020-06".into()), quality_rating: 2, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "gpt-3.5-turbo".into(), name: "GPT-3.5 Turbo".into(), provider: "OpenAI".into(),
            context_window: 16_385, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0005, output_cost_per_1k: 0.0015,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2021-09".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "gpt-3.5-turbo-16k".into(), name: "GPT-3.5 Turbo 16K".into(), provider: "OpenAI".into(),
            context_window: 16_385, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0003, output_cost_per_1k: 0.0004,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2021-09".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // ── GPT-4 Series ──
        ModelInfo {
            id: "gpt-4".into(), name: "GPT-4".into(), provider: "OpenAI".into(),
            context_window: 8_192, max_output_tokens: 8_192,
            input_cost_per_1k: 0.03, output_cost_per_1k: 0.06,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2021-09".into()), quality_rating: 4, speed_rating: 2,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "gpt-4-32k".into(), name: "GPT-4 32K".into(), provider: "OpenAI".into(),
            context_window: 32_768, max_output_tokens: 8_192,
            input_cost_per_1k: 0.06, output_cost_per_1k: 0.12,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2021-09".into()), quality_rating: 4, speed_rating: 2,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "gpt-4-turbo".into(), name: "GPT-4 Turbo".into(), provider: "OpenAI".into(),
            context_window: 128_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.01, output_cost_per_1k: 0.03,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2023-12".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: false, free_tier: false,
        },
        // ── GPT-4o Series (2024) ──
        ModelInfo {
            id: "gpt-4o".into(), name: "GPT-4o".into(), provider: "OpenAI".into(),
            context_window: 128_000, max_output_tokens: 16_384,
            input_cost_per_1k: 0.0025, output_cost_per_1k: 0.01,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-04".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "gpt-4o-mini".into(), name: "GPT-4o Mini".into(), provider: "OpenAI".into(),
            context_window: 128_000, max_output_tokens: 16_384,
            input_cost_per_1k: 0.00015, output_cost_per_1k: 0.0006,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2023-10".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // ── Reasoning o-series ──
        ModelInfo {
            id: "o1-preview".into(), name: "o1 Preview".into(), provider: "OpenAI".into(),
            context_window: 128_000, max_output_tokens: 32_768,
            input_cost_per_1k: 0.015, output_cost_per_1k: 0.06,
            supports_vision: false, supports_tools: true, supports_streaming: false, supports_json: false,
            training_cutoff: Some("2024-06".into()), quality_rating: 5, speed_rating: 2,
            is_reasoning: true, free_tier: false,
        },
        ModelInfo {
            id: "o1".into(), name: "o1".into(), provider: "OpenAI".into(),
            context_window: 200_000, max_output_tokens: 100_000,
            input_cost_per_1k: 0.015, output_cost_per_1k: 0.06,
            supports_vision: true, supports_tools: true, supports_streaming: false, supports_json: false,
            training_cutoff: Some("2024-06".into()), quality_rating: 5, speed_rating: 2,
            is_reasoning: true, free_tier: false,
        },
        ModelInfo {
            id: "o1-mini".into(), name: "o1 Mini".into(), provider: "OpenAI".into(),
            context_window: 128_000, max_output_tokens: 65_536,
            input_cost_per_1k: 0.0011, output_cost_per_1k: 0.0044,
            supports_vision: false, supports_tools: true, supports_streaming: false, supports_json: false,
            training_cutoff: Some("2024-06".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: true, free_tier: false,
        },
        ModelInfo {
            id: "o1-pro".into(), name: "o1 Pro".into(), provider: "OpenAI".into(),
            context_window: 200_000, max_output_tokens: 100_000,
            input_cost_per_1k: 0.15, output_cost_per_1k: 0.60,
            supports_vision: true, supports_tools: true, supports_streaming: false, supports_json: false,
            training_cutoff: Some("2024-06".into()), quality_rating: 5, speed_rating: 1,
            is_reasoning: true, free_tier: false,
        },
        ModelInfo {
            id: "o3-mini".into(), name: "o3 Mini".into(), provider: "OpenAI".into(),
            context_window: 200_000, max_output_tokens: 100_000,
            input_cost_per_1k: 0.0011, output_cost_per_1k: 0.0044,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: false,
            training_cutoff: Some("2024-10".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: true, free_tier: false,
        },
        ModelInfo {
            id: "o3".into(), name: "o3".into(), provider: "OpenAI".into(),
            context_window: 200_000, max_output_tokens: 100_000,
            input_cost_per_1k: 0.015, output_cost_per_1k: 0.06,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: false,
            training_cutoff: Some("2025-04".into()), quality_rating: 5, speed_rating: 2,
            is_reasoning: true, free_tier: false,
        },
        ModelInfo {
            id: "o4-mini".into(), name: "o4 Mini".into(), provider: "OpenAI".into(),
            context_window: 200_000, max_output_tokens: 100_000,
            input_cost_per_1k: 0.0011, output_cost_per_1k: 0.0044,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: false,
            training_cutoff: Some("2025-11".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: true, free_tier: false,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ANTHROPIC MODELS (2023 → 2026)
// ═══════════════════════════════════════════════════════════════════════════════

/// Anthropic Claude models
pub fn anthropic_models() -> Vec<ModelInfo> {
    vec![
        // ── Claude 1 (Legacy) ──
        ModelInfo {
            id: "claude-1".into(), name: "Claude 1 (Legacy)".into(), provider: "Anthropic".into(),
            context_window: 9_000, max_output_tokens: 2_048,
            input_cost_per_1k: 0.008, output_cost_per_1k: 0.024,
            supports_vision: false, supports_tools: false, supports_streaming: true, supports_json: false,
            training_cutoff: Some("2022-12".into()), quality_rating: 2, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "claude-instant-1".into(), name: "Claude Instant 1 (Legacy)".into(), provider: "Anthropic".into(),
            context_window: 9_000, max_output_tokens: 2_048,
            input_cost_per_1k: 0.0008, output_cost_per_1k: 0.0024,
            supports_vision: false, supports_tools: false, supports_streaming: true, supports_json: false,
            training_cutoff: Some("2022-12".into()), quality_rating: 2, speed_rating: 5,
            is_reasoning: false, free_tier: false,
        },
        // ── Claude 2 ──
        ModelInfo {
            id: "claude-2".into(), name: "Claude 2".into(), provider: "Anthropic".into(),
            context_window: 100_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.008, output_cost_per_1k: 0.024,
            supports_vision: false, supports_tools: false, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2023-04".into()), quality_rating: 3, speed_rating: 3,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "claude-2.1".into(), name: "Claude 2.1".into(), provider: "Anthropic".into(),
            context_window: 200_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.008, output_cost_per_1k: 0.024,
            supports_vision: false, supports_tools: false, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2023-08".into()), quality_rating: 4, speed_rating: 3,
            is_reasoning: false, free_tier: false,
        },
        // ── Claude 3 ──
        ModelInfo {
            id: "claude-3-haiku-20240307".into(), name: "Claude 3 Haiku".into(), provider: "Anthropic".into(),
            context_window: 200_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.00025, output_cost_per_1k: 0.00125,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2023-08".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "claude-3-sonnet-20240229".into(), name: "Claude 3 Sonnet".into(), provider: "Anthropic".into(),
            context_window: 200_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.003, output_cost_per_1k: 0.015,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2023-08".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "claude-3-opus-20240229".into(), name: "Claude 3 Opus".into(), provider: "Anthropic".into(),
            context_window: 200_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.015, output_cost_per_1k: 0.075,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2023-08".into()), quality_rating: 5, speed_rating: 2,
            is_reasoning: false, free_tier: false,
        },
        // ── Claude 3.5 ──
        ModelInfo {
            id: "claude-3-5-sonnet-20241022".into(), name: "Claude 3.5 Sonnet (v2)".into(), provider: "Anthropic".into(),
            context_window: 200_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.003, output_cost_per_1k: 0.015,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-07".into()), quality_rating: 5, speed_rating: 5,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "claude-3-5-haiku-20241022".into(), name: "Claude 3.5 Haiku".into(), provider: "Anthropic".into(),
            context_window: 200_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.001, output_cost_per_1k: 0.005,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-07".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: false,
        },
        // ── Claude 4 (2025) ──
        ModelInfo {
            id: "claude-opus-4-20250514".into(), name: "Claude Opus 4".into(), provider: "Anthropic".into(),
            context_window: 200_000, max_output_tokens: 32_000,
            input_cost_per_1k: 0.015, output_cost_per_1k: 0.075,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-02".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "claude-sonnet-4-20250514".into(), name: "Claude Sonnet 4".into(), provider: "Anthropic".into(),
            context_window: 200_000, max_output_tokens: 16_000,
            input_cost_per_1k: 0.003, output_cost_per_1k: 0.015,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-02".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  GOOGLE MODELS (2022 → 2026)
// ═══════════════════════════════════════════════════════════════════════════════

/// Google Gemini models
pub fn google_models() -> Vec<ModelInfo> {
    vec![
        // ── Legacy (PaLM) ──
        ModelInfo {
            id: "text-bison-001".into(), name: "PaLM 2 Bison (Legacy)".into(), provider: "Google".into(),
            context_window: 8_192, max_output_tokens: 1_024,
            input_cost_per_1k: 0.001, output_cost_per_1k: 0.002,
            supports_vision: false, supports_tools: false, supports_streaming: true, supports_json: false,
            training_cutoff: Some("2022-06".into()), quality_rating: 2, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        // ── Gemini 1.0 ──
        ModelInfo {
            id: "gemini-1.0-pro".into(), name: "Gemini 1.0 Pro".into(), provider: "Google".into(),
            context_window: 32_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0005, output_cost_per_1k: 0.0015,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2023-12".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        // ── Gemini 1.5 ──
        ModelInfo {
            id: "gemini-1.5-pro".into(), name: "Gemini 1.5 Pro".into(), provider: "Google".into(),
            context_window: 2_097_152, max_output_tokens: 8_192,
            input_cost_per_1k: 0.00125, output_cost_per_1k: 0.005,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-01".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "gemini-1.5-flash".into(), name: "Gemini 1.5 Flash".into(), provider: "Google".into(),
            context_window: 1_048_576, max_output_tokens: 8_192,
            input_cost_per_1k: 0.000075, output_cost_per_1k: 0.0003,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-01".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // ── Gemini 2.0 ──
        ModelInfo {
            id: "gemini-2.0-flash".into(), name: "Gemini 2.0 Flash".into(), provider: "Google".into(),
            context_window: 1_048_576, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0004,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-06".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "gemini-2.0-flash-thinking-exp".into(), name: "Gemini 2.0 Flash Thinking".into(), provider: "Google".into(),
            context_window: 1_048_576, max_output_tokens: 65_536,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0004,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-06".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: true, free_tier: true,
        },
        ModelInfo {
            id: "gemini-2.0-pro-exp-02-05".into(), name: "Gemini 2.0 Pro".into(), provider: "Google".into(),
            context_window: 1_048_576, max_output_tokens: 8_192,
            input_cost_per_1k: 0.00125, output_cost_per_1k: 0.005,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-06".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        // ── Gemini 2.5 (2025) ──
        ModelInfo {
            id: "gemini-2.5-pro-preview-05-06".into(), name: "Gemini 2.5 Pro".into(), provider: "Google".into(),
            context_window: 1_048_576, max_output_tokens: 65_536,
            input_cost_per_1k: 0.00125, output_cost_per_1k: 0.01,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-01".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: true, free_tier: false,
        },
        ModelInfo {
            id: "gemini-2.5-flash-preview-04-17".into(), name: "Gemini 2.5 Flash".into(), provider: "Google".into(),
            context_window: 1_048_576, max_output_tokens: 65_536,
            input_cost_per_1k: 0.00015, output_cost_per_1k: 0.0006,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-01".into()), quality_rating: 5, speed_rating: 5,
            is_reasoning: true, free_tier: true,
        },
        // ── Gemma (Open) ──
        ModelInfo {
            id: "gemma-2-27b-it".into(), name: "Gemma 2 27B".into(), provider: "Google".into(),
            context_window: 8_192, max_output_tokens: 4_096,
            input_cost_per_1k: 0.00008, output_cost_per_1k: 0.00008,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-06".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "gemma-2-9b-it".into(), name: "Gemma 2 9B".into(), provider: "Google".into(),
            context_window: 8_192, max_output_tokens: 4_096,
            input_cost_per_1k: 0.00003, output_cost_per_1k: 0.00003,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-06".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "gemma-3-27b-it".into(), name: "Gemma 3 27B".into(), provider: "Google".into(),
            context_window: 128_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0001,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-02".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  MISTRAL AI MODELS (2023 → 2026)
// ═══════════════════════════════════════════════════════════════════════════════

/// Mistral AI models
pub fn mistral_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "mistral-tiny".into(), name: "Mistral Tiny (Legacy)".into(), provider: "Mistral".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0001,
            supports_vision: false, supports_tools: false, supports_streaming: true, supports_json: false,
            training_cutoff: Some("2023-06".into()), quality_rating: 2, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "mistral-small-latest".into(), name: "Mistral Small 3.1".into(), provider: "Mistral".into(),
            context_window: 128_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0003,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-02".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "mistral-medium-latest".into(), name: "Mistral Medium".into(), provider: "Mistral".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0007, output_cost_per_1k: 0.0021,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-01".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "mistral-large-latest".into(), name: "Mistral Large 2".into(), provider: "Mistral".into(),
            context_window: 128_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.002, output_cost_per_1k: 0.006,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-07".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "codestral-latest".into(), name: "Codestral".into(), provider: "Mistral".into(),
            context_window: 256_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0002, output_cost_per_1k: 0.0006,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-03".into()), quality_rating: 5, speed_rating: 5,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "mistral-embed".into(), name: "Mistral Embed".into(), provider: "Mistral".into(),
            context_window: 8_192, max_output_tokens: 0,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: false, supports_streaming: false, supports_json: false,
            training_cutoff: Some("2023-12".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "open-mistral-7b".into(), name: "Mistral 7B (Open)".into(), provider: "Mistral".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0001,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2023-09".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "open-mixtral-8x7b".into(), name: "Mixtral 8x7B".into(), provider: "Mistral".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.00024, output_cost_per_1k: 0.00024,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2023-12".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "open-mixtral-8x22b".into(), name: "Mixtral 8x22B".into(), provider: "Mistral".into(),
            context_window: 65_536, max_output_tokens: 4_096,
            input_cost_per_1k: 0.00065, output_cost_per_1k: 0.00065,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-01".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "pixtral-large-latest".into(), name: "Pixtral Large (Vision)".into(), provider: "Mistral".into(),
            context_window: 128_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.002, output_cost_per_1k: 0.006,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-09".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: false, free_tier: false,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DEEPSEEK MODELS (2024 → 2026)
// ═══════════════════════════════════════════════════════════════════════════════

/// DeepSeek models
pub fn deepseek_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "deepseek-coder".into(), name: "DeepSeek Coder V2".into(), provider: "DeepSeek".into(),
            context_window: 128_000, max_output_tokens: 16_384,
            input_cost_per_1k: 0.00014, output_cost_per_1k: 0.00028,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-06".into()), quality_rating: 5, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "deepseek-chat".into(), name: "DeepSeek V3".into(), provider: "DeepSeek".into(),
            context_window: 64_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.00007, output_cost_per_1k: 0.00028,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-07".into()), quality_rating: 5, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "deepseek-reasoner".into(), name: "DeepSeek R1".into(), provider: "DeepSeek".into(),
            context_window: 64_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.00055, output_cost_per_1k: 0.00219,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-11".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: true, free_tier: true,
        },
        ModelInfo {
            id: "deepseek-prover-v2".into(), name: "DeepSeek Prover V2".into(), provider: "DeepSeek".into(),
            context_window: 64_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.00055, output_cost_per_1k: 0.00219,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-04".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: true, free_tier: true,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  XAI GROK MODELS (2024 → 2026)
// ═══════════════════════════════════════════════════════════════════════════════

/// xAI (Grok) models
pub fn xai_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "grok-beta".into(), name: "Grok Beta (Legacy)".into(), provider: "xAI".into(),
            context_window: 131_072, max_output_tokens: 8_192,
            input_cost_per_1k: 0.005, output_cost_per_1k: 0.015,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-03".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "grok-2-latest".into(), name: "Grok 2".into(), provider: "xAI".into(),
            context_window: 131_072, max_output_tokens: 8_192,
            input_cost_per_1k: 0.002, output_cost_per_1k: 0.01,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-08".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "grok-3".into(), name: "Grok 3".into(), provider: "xAI".into(),
            context_window: 131_072, max_output_tokens: 16_384,
            input_cost_per_1k: 0.003, output_cost_per_1k: 0.015,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-02".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "grok-3-mini".into(), name: "Grok 3 Mini (Reasoning)".into(), provider: "xAI".into(),
            context_window: 131_072, max_output_tokens: 16_384,
            input_cost_per_1k: 0.0003, output_cost_per_1k: 0.0005,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-02".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: true, free_tier: true,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  META LLAMA MODELS (2023 → 2026)
// ═══════════════════════════════════════════════════════════════════════════════

/// Meta Llama models
pub fn llama_models() -> Vec<ModelInfo> {
    vec![
        // Llama 1 (Legacy)
        ModelInfo {
            id: "llama-7b".into(), name: "LLaMA 7B (Legacy)".into(), provider: "Meta".into(),
            context_window: 2_048, max_output_tokens: 2_048,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: false, supports_streaming: true, supports_json: false,
            training_cutoff: Some("2023-02".into()), quality_rating: 2, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // Llama 2
        ModelInfo {
            id: "llama-2-70b".into(), name: "Llama 2 70B".into(), provider: "Meta".into(),
            context_window: 4_096, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0006, output_cost_per_1k: 0.0006,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2023-07".into()), quality_rating: 3, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        // Llama 3
        ModelInfo {
            id: "llama-3-8b".into(), name: "Llama 3 8B".into(), provider: "Meta".into(),
            context_window: 8_192, max_output_tokens: 4_096,
            input_cost_per_1k: 0.00005, output_cost_per_1k: 0.00005,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2023-12".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "llama-3-70b".into(), name: "Llama 3 70B".into(), provider: "Meta".into(),
            context_window: 8_192, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0008, output_cost_per_1k: 0.0008,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2023-12".into()), quality_rating: 4, speed_rating: 3,
            is_reasoning: false, free_tier: true,
        },
        // Llama 3.1
        ModelInfo {
            id: "meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo".into(), name: "Llama 3.1 8B".into(), provider: "Together".into(),
            context_window: 131_072, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0001,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-07".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo".into(), name: "Llama 3.1 70B".into(), provider: "Together".into(),
            context_window: 131_072, max_output_tokens: 4_096,
            input_cost_per_1k: 0.00088, output_cost_per_1k: 0.00088,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-07".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "meta-llama/Meta-Llama-3.1-405B-Instruct-Turbo".into(), name: "Llama 3.1 405B".into(), provider: "Together".into(),
            context_window: 131_072, max_output_tokens: 4_096,
            input_cost_per_1k: 0.005, output_cost_per_1k: 0.005,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-07".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: false, free_tier: false,
        },
        // Llama 3.2 (Vision)
        ModelInfo {
            id: "llama-3.2-11b-vision".into(), name: "Llama 3.2 11B Vision".into(), provider: "Meta".into(),
            context_window: 131_072, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0001,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-09".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "llama-3.2-90b-vision".into(), name: "Llama 3.2 90B Vision".into(), provider: "Meta".into(),
            context_window: 131_072, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0009, output_cost_per_1k: 0.0009,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-09".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: false, free_tier: true,
        },
        // Llama 3.3
        ModelInfo {
            id: "meta-llama/Llama-3.3-70B-Instruct-Turbo".into(), name: "Llama 3.3 70B".into(), provider: "Together".into(),
            context_window: 128_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.00088, output_cost_per_1k: 0.00088,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-12".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        // Llama 4 (2025)
        ModelInfo {
            id: "llama-4-maverick-17b-128e".into(), name: "Llama 4 Maverick 400B (MoE)".into(), provider: "Meta".into(),
            context_window: 1_048_576, max_output_tokens: 16_384,
            input_cost_per_1k: 0.0015, output_cost_per_1k: 0.0015,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-04".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "llama-4-scout-17b-16e".into(), name: "Llama 4 Scout 109B (MoE)".into(), provider: "Meta".into(),
            context_window: 10_485_760, max_output_tokens: 16_384,
            input_cost_per_1k: 0.001, output_cost_per_1k: 0.001,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-04".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  COHERE MODELS (2022 → 2026)
// ═══════════════════════════════════════════════════════════════════════════════

/// Cohere models
pub fn cohere_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "command".into(), name: "Command (Legacy)".into(), provider: "Cohere".into(),
            context_window: 4_096, max_output_tokens: 4_096,
            input_cost_per_1k: 0.001, output_cost_per_1k: 0.002,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2022-12".into()), quality_rating: 3, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "command-light".into(), name: "Command Light".into(), provider: "Cohere".into(),
            context_window: 4_096, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0003, output_cost_per_1k: 0.0006,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2023-01".into()), quality_rating: 2, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "command-r-08-2024".into(), name: "Command R".into(), provider: "Cohere".into(),
            context_window: 128_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.00015, output_cost_per_1k: 0.0006,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-08".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "command-r-plus-08-2024".into(), name: "Command R+".into(), provider: "Cohere".into(),
            context_window: 128_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0025, output_cost_per_1k: 0.01,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-08".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "command-a-03-2025".into(), name: "Command A".into(), provider: "Cohere".into(),
            context_window: 256_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0025, output_cost_per_1k: 0.01,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-03".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "aya-exa-32b".into(), name: "Aya Exa 32B".into(), provider: "Cohere".into(),
            context_window: 128_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0005, output_cost_per_1k: 0.0015,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-08".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PERPLEXITY MODELS
// ═══════════════════════════════════════════════════════════════════════════════

pub fn perplexity_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "sonar".into(), name: "Sonar".into(), provider: "Perplexity".into(),
            context_window: 127_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0002, output_cost_per_1k: 0.0002,
            supports_vision: false, supports_tools: false, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-12".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "sonar-pro".into(), name: "Sonar Pro".into(), provider: "Perplexity".into(),
            context_window: 200_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0005, output_cost_per_1k: 0.0005,
            supports_vision: false, supports_tools: false, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-12".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "sonar-reasoning".into(), name: "Sonar Reasoning".into(), provider: "Perplexity".into(),
            context_window: 127_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.001, output_cost_per_1k: 0.005,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-12".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: true, free_tier: false,
        },
        ModelInfo {
            id: "sonar-deep-research".into(), name: "Sonar Deep Research".into(), provider: "Perplexity".into(),
            context_window: 127_000, max_output_tokens: 16_384,
            input_cost_per_1k: 0.002, output_cost_per_1k: 0.01,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-03".into()), quality_rating: 5, speed_rating: 1,
            is_reasoning: true, free_tier: false,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  GROQ MODELS (LPU - Fastest)
// ═══════════════════════════════════════════════════════════════════════════════

pub fn groq_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "llama-3.3-70b-versatile".into(), name: "Llama 3.3 70B (Groq)".into(), provider: "Groq".into(),
            context_window: 131_072, max_output_tokens: 32_768,
            input_cost_per_1k: 0.00059, output_cost_per_1k: 0.00079,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-12".into()), quality_rating: 5, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "llama-3.1-8b-instant".into(), name: "Llama 3.1 8B (Groq)".into(), provider: "Groq".into(),
            context_window: 131_072, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-07".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "mixtral-8x7b-32768".into(), name: "Mixtral 8x7B (Groq)".into(), provider: "Groq".into(),
            context_window: 32_768, max_output_tokens: 4_096,
            input_cost_per_1k: 0.00024, output_cost_per_1k: 0.00024,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2023-12".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "gemma2-9b-it".into(), name: "Gemma 2 9B (Groq)".into(), provider: "Groq".into(),
            context_window: 8_192, max_output_tokens: 4_096,
            input_cost_per_1k: 0.00008, output_cost_per_1k: 0.00008,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-06".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "deepseek-r1-distill-llama-70b".into(), name: "DeepSeek R1 70B (Groq)".into(), provider: "Groq".into(),
            context_window: 131_072, max_output_tokens: 32_768,
            input_cost_per_1k: 0.00075, output_cost_per_1k: 0.00099,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-11".into()), quality_rating: 5, speed_rating: 5,
            is_reasoning: true, free_tier: true,
        },
        ModelInfo {
            id: "qwen-qwq-32b".into(), name: "QwQ 32B (Groq)".into(), provider: "Groq".into(),
            context_window: 131_072, max_output_tokens: 32_768,
            input_cost_per_1k: 0.00029, output_cost_per_1k: 0.00039,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-02".into()), quality_rating: 5, speed_rating: 5,
            is_reasoning: true, free_tier: true,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  AI21 MODELS
// ═══════════════════════════════════════════════════════════════════════════════

pub fn ai21_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "j1-jumbo".into(), name: "Jurassic-1 Jumbo (Legacy)".into(), provider: "AI21".into(),
            context_window: 2_048, max_output_tokens: 2_048,
            input_cost_per_1k: 0.0125, output_cost_per_1k: 0.0125,
            supports_vision: false, supports_tools: false, supports_streaming: true, supports_json: false,
            training_cutoff: Some("2021-12".into()), quality_rating: 2, speed_rating: 3,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "jamba-1-5-large".into(), name: "Jamba 1.5 Large".into(), provider: "AI21".into(),
            context_window: 256_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.002, output_cost_per_1k: 0.008,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-07".into()), quality_rating: 4, speed_rating: 3,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "jamba-1-5-mini".into(), name: "Jamba 1.5 Mini".into(), provider: "AI21".into(),
            context_window: 256_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0002, output_cost_per_1k: 0.0004,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-07".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  FIREWORKS MODELS
// ═══════════════════════════════════════════════════════════════════════════════

pub fn fireworks_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "accounts/fireworks/models/llama-v3p3-70b-instruct".into(), name: "Llama 3.3 70B (Fireworks)".into(), provider: "Fireworks".into(),
            context_window: 131_072, max_output_tokens: 16_384,
            input_cost_per_1k: 0.0009, output_cost_per_1k: 0.0009,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-12".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "accounts/fireworks/models/qwen2p5-72b-instruct".into(), name: "Qwen 2.5 72B".into(), provider: "Fireworks".into(),
            context_window: 131_072, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0009, output_cost_per_1k: 0.0009,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-09".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "accounts/fireworks/models/deepseek-r1".into(), name: "DeepSeek R1 (Fireworks)".into(), provider: "Fireworks".into(),
            context_window: 131_072, max_output_tokens: 16_384,
            input_cost_per_1k: 0.008, output_cost_per_1k: 0.008,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-11".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: true, free_tier: false,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  REPLICATE MODELS
// ═══════════════════════════════════════════════════════════════════════════════

pub fn replicate_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "meta/llama-3.3-70b-instruct".into(), name: "Llama 3.3 70B (Replicate)".into(), provider: "Replicate".into(),
            context_window: 128_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0006, output_cost_per_1k: 0.0006,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-12".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "mistralai/mixtral-8x7b-instruct-v0.1".into(), name: "Mixtral 8x7B (Replicate)".into(), provider: "Replicate".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0003, output_cost_per_1k: 0.0003,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2023-12".into()), quality_rating: 4, speed_rating: 3,
            is_reasoning: false, free_tier: false,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  QWEN / ALIBABA MODELS
// ═══════════════════════════════════════════════════════════════════════════════

pub fn qwen_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "qwen2.5-72b-instruct".into(), name: "Qwen 2.5 72B".into(), provider: "Qwen".into(),
            context_window: 131_072, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0004, output_cost_per_1k: 0.0012,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-09".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "qwen2.5-coder-32b-instruct".into(), name: "Qwen 2.5 Coder 32B".into(), provider: "Qwen".into(),
            context_window: 131_072, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0002, output_cost_per_1k: 0.0006,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-11".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "qwen3-235b-a22b".into(), name: "Qwen3 235B-A22B (MoE)".into(), provider: "Qwen".into(),
            context_window: 131_072, max_output_tokens: 16_384,
            input_cost_per_1k: 0.0014, output_cost_per_1k: 0.0042,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-03".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: true, free_tier: true,
        },
        ModelInfo {
            id: "qwen3-32b".into(), name: "Qwen3 32B".into(), provider: "Qwen".into(),
            context_window: 131_072, max_output_tokens: 16_384,
            input_cost_per_1k: 0.0005, output_cost_per_1k: 0.0015,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-03".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: true, free_tier: true,
        },
        ModelInfo {
            id: "qwq-32b".into(), name: "QwQ 32B (Reasoning)".into(), provider: "Qwen".into(),
            context_window: 131_072, max_output_tokens: 16_384,
            input_cost_per_1k: 0.0005, output_cost_per_1k: 0.0015,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-02".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: true, free_tier: true,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CHINESE AI PROVIDERS
// ═══════════════════════════════════════════════════════════════════════════════

pub fn chinese_models() -> Vec<ModelInfo> {
    vec![
        // Baidu Ernie
        ModelInfo {
            id: "ernie-4.0-8k".into(), name: "ERNIE 4.0".into(), provider: "Baidu".into(),
            context_window: 8_192, max_output_tokens: 4_096,
            input_cost_per_1k: 0.012, output_cost_per_1k: 0.012,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-06".into()), quality_rating: 4, speed_rating: 3,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "ernie-3.5-8k".into(), name: "ERNIE 3.5".into(), provider: "Baidu".into(),
            context_window: 8_192, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0012, output_cost_per_1k: 0.0012,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2023-12".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // Zhipu GLM
        ModelInfo {
            id: "glm-4-plus".into(), name: "GLM-4 Plus".into(), provider: "Zhipu".into(),
            context_window: 128_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0035, output_cost_per_1k: 0.0035,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-10".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "glm-4-flash".into(), name: "GLM-4 Flash".into(), provider: "Zhipu".into(),
            context_window: 128_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0001,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-10".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // Moonshot (Kimi)
        ModelInfo {
            id: "moonshot-v1-128k".into(), name: "Moonshot V1 128K".into(), provider: "Moonshot".into(),
            context_window: 128_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.001, output_cost_per_1k: 0.001,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-03".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "kimi-latest".into(), name: "Kimi Latest".into(), provider: "Moonshot".into(),
            context_window: 1_048_576, max_output_tokens: 8_192,
            input_cost_per_1k: 0.002, output_cost_per_1k: 0.002,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-01".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: false, free_tier: false,
        },
        // Yi (01.AI)
        ModelInfo {
            id: "yi-lightning".into(), name: "Yi Lightning".into(), provider: "Yi".into(),
            context_window: 16_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0001,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-06".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // MiniMax
        ModelInfo {
            id: "abab6.5s-chat".into(), name: "MiniMax ABAB 6.5s".into(), provider: "MiniMax".into(),
            context_window: 128_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.001, output_cost_per_1k: 0.001,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-06".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        // StepFun
        ModelInfo {
            id: "step-2-16k".into(), name: "Step-2 16K".into(), provider: "StepFun".into(),
            context_window: 16_384, max_output_tokens: 8_192,
            input_cost_per_1k: 0.004, output_cost_per_1k: 0.016,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-10".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: false, free_tier: false,
        },
        // ByteDance Doubao
        ModelInfo {
            id: "doubao-1.5-pro".into(), name: "Doubao 1.5 Pro".into(), provider: "ByteDance".into(),
            context_window: 128_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0005, output_cost_per_1k: 0.0015,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-01".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  RUSSIAN AI PROVIDERS
// ═══════════════════════════════════════════════════════════════════════════════

pub fn russian_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "GigaChat-Pro".into(), name: "GigaChat Pro".into(), provider: "GigaChat".into(),
            context_window: 131_072, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0007, output_cost_per_1k: 0.002,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-06".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "GigaChat-Max".into(), name: "GigaChat Max".into(), provider: "GigaChat".into(),
            context_window: 131_072, max_output_tokens: 16_384,
            input_cost_per_1k: 0.0015, output_cost_per_1k: 0.005,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-10".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "yandexgpt-5".into(), name: "YandexGPT 5".into(), provider: "Yandex".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.001, output_cost_per_1k: 0.003,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-12".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  KOREAN AI PROVIDERS
// ═══════════════════════════════════════════════════════════════════════════════

pub fn korean_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "solar-pro2-preview".into(), name: "Solar Pro 2".into(), provider: "Upstage".into(),
            context_window: 65_536, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0003, output_cost_per_1k: 0.0009,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-10".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "solar-mini".into(), name: "Solar Mini".into(), provider: "Upstage".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.00005, output_cost_per_1k: 0.00015,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-06".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "hyperclovax-seed".into(), name: "HyperCLOVA X".into(), provider: "Naver".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0007, output_cost_per_1k: 0.002,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-06".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  EUROPEAN AI PROVIDERS
// ═══════════════════════════════════════════════════════════════════════════════

pub fn european_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "luminous-supreme-control".into(), name: "Luminous Supreme Control".into(), provider: "Aleph Alpha".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.03, output_cost_per_1k: 0.03,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-01".into()), quality_rating: 4, speed_rating: 3,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "pharia-1-llm-7b-control".into(), name: "Pharia 1 LLM 7B".into(), provider: "Aleph Alpha".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0002, output_cost_per_1k: 0.0002,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-06".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "mistral-large-latest".into(), name: "Mistral Large 2 (EU)".into(), provider: "Mistral".into(),
            context_window: 128_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.002, output_cost_per_1k: 0.006,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-07".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  INDIAN AI PROVIDERS
// ═══════════════════════════════════════════════════════════════════════════════

pub fn indian_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "sarvam-m".into(), name: "Sarvam-M".into(), provider: "Sarvam".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0002, output_cost_per_1k: 0.0006,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-10".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ENTERPRISE / CLOUD PROVIDERS
// ═══════════════════════════════════════════════════════════════════════════════

pub fn enterprise_models() -> Vec<ModelInfo> {
    vec![
        // AWS Bedrock
        ModelInfo {
            id: "anthropic.claude-3-5-sonnet-20241022-v2:0".into(), name: "Claude 3.5 Sonnet (Bedrock)".into(), provider: "Bedrock".into(),
            context_window: 200_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.003, output_cost_per_1k: 0.015,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-07".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "meta.llama3-3-70b-instruct-v1:0".into(), name: "Llama 3.3 70B (Bedrock)".into(), provider: "Bedrock".into(),
            context_window: 128_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0008, output_cost_per_1k: 0.0008,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-12".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        // Azure OpenAI
        ModelInfo {
            id: "gpt-4o-azure".into(), name: "GPT-4o (Azure)".into(), provider: "Azure".into(),
            context_window: 128_000, max_output_tokens: 16_384,
            input_cost_per_1k: 0.0025, output_cost_per_1k: 0.01,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-04".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        // GCP Vertex AI
        ModelInfo {
            id: "gemini-2.0-flash-vertex".into(), name: "Gemini 2.0 Flash (Vertex)".into(), provider: "Vertex".into(),
            context_window: 1_048_576, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0004,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-06".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: false,
        },
        // IBM WatsonX
        ModelInfo {
            id: "ibm/granite-3.3-8b-instruct".into(), name: "Granite 3.3 8B".into(), provider: "WatsonX".into(),
            context_window: 128_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0002, output_cost_per_1k: 0.0002,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-02".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // NVIDIA NIM
        ModelInfo {
            id: "nvidia/llama-3.3-nemotron-super-49b-v1".into(), name: "Llama 3.3 Nemotron 49B".into(), provider: "NVIDIA".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0008, output_cost_per_1k: 0.0008,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-03".into()), quality_rating: 5, speed_rating: 5,
            is_reasoning: false, free_tier: false,
        },
        // Oracle OCI GenAI
        ModelInfo {
            id: "oci-cohere-command-r-plus".into(), name: "Command R+ (OCI)".into(), provider: "OCI".into(),
            context_window: 128_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0025, output_cost_per_1k: 0.01,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-08".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: false, free_tier: false,
        },
        // SambaNova
        ModelInfo {
            id: "Meta-Llama-3.3-70B-Instruct".into(), name: "Llama 3.3 70B (SambaNova)".into(), provider: "SambaNova".into(),
            context_window: 131_072, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0007, output_cost_per_1k: 0.0007,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-12".into()), quality_rating: 5, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  AGGREGATOR / ROUTER PROVIDERS
// ═══════════════════════════════════════════════════════════════════════════════

pub fn aggregator_models() -> Vec<ModelInfo> {
    vec![
        // OpenRouter
        ModelInfo {
            id: "openrouter/auto".into(), name: "OpenRouter Auto".into(), provider: "OpenRouter".into(),
            context_window: 128_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0025, output_cost_per_1k: 0.01,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-01".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        // Cerebras
        ModelInfo {
            id: "llama3.3-70b".into(), name: "Llama 3.3 70B (Cerebras)".into(), provider: "Cerebras".into(),
            context_window: 128_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0006, output_cost_per_1k: 0.0006,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-12".into()), quality_rating: 5, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // Cloudflare
        ModelInfo {
            id: "@cf/meta/llama-3.3-70b-instruct-fp8-fast".into(), name: "Llama 3.3 70B (CF)".into(), provider: "Cloudflare".into(),
            context_window: 65_536, max_output_tokens: 4_096,
            input_cost_per_1k: 0.00075, output_cost_per_1k: 0.00075,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-12".into()), quality_rating: 5, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // DeepInfra
        ModelInfo {
            id: "meta-llama/Llama-3.3-70B-Instruct".into(), name: "Llama 3.3 70B (DeepInfra)".into(), provider: "DeepInfra".into(),
            context_window: 131_072, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0005, output_cost_per_1k: 0.0005,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-12".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        // SiliconFlow
        ModelInfo {
            id: "deepseek-ai/DeepSeek-V3".into(), name: "DeepSeek V3 (SiliconFlow)".into(), provider: "SiliconFlow".into(),
            context_window: 64_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0003,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-07".into()), quality_rating: 5, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  LOCAL / SELF-HOSTED PROVIDERS
// ═══════════════════════════════════════════════════════════════════════════════

pub fn local_models() -> Vec<ModelInfo> {
    vec![
        // Ollama
        ModelInfo {
            id: "llama3.3:70b".into(), name: "Llama 3.3 70B (Ollama)".into(), provider: "Ollama".into(),
            context_window: 131_072, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-12".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "llama3.1:8b".into(), name: "Llama 3.1 8B (Ollama)".into(), provider: "Ollama".into(),
            context_window: 131_072, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-07".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "mistral:7b".into(), name: "Mistral 7B (Ollama)".into(), provider: "Ollama".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2023-09".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "qwen3:32b".into(), name: "Qwen3 32B (Ollama)".into(), provider: "Ollama".into(),
            context_window: 131_072, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-03".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: true, free_tier: true,
        },
        ModelInfo {
            id: "deepseek-r1:70b".into(), name: "DeepSeek R1 70B (Ollama)".into(), provider: "Ollama".into(),
            context_window: 131_072, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-11".into()), quality_rating: 5, speed_rating: 2,
            is_reasoning: true, free_tier: true,
        },
        ModelInfo {
            id: "codestral:22b".into(), name: "Codestral 22B (Ollama)".into(), provider: "Ollama".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-03".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        // vLLM
        ModelInfo {
            id: "vllm-default".into(), name: "vLLM Default".into(), provider: "vLLM".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: None, quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        // LM Studio
        ModelInfo {
            id: "lmstudio-default".into(), name: "LM Studio Default".into(), provider: "LMStudio".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: None, quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        // Llamafile
        ModelInfo {
            id: "llamafile-default".into(), name: "Llamafile Default".into(), provider: "Llamafile".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: None, quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        // Cevahir AI (SENTIENT's own engine)
        ModelInfo {
            id: "cevahir-v7".into(), name: "Cevahir V-7".into(), provider: "CevahirAI".into(),
            context_window: 8_192, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-06".into()), quality_rating: 3, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ADDITIONAL SPECIALIZED PROVIDERS
// ═══════════════════════════════════════════════════════════════════════════════

pub fn specialized_models() -> Vec<ModelInfo> {
    vec![
        // Stability AI
        ModelInfo {
            id: "stablelm-2-12b-chat".into(), name: "StableLM 2 12B".into(), provider: "Stability".into(),
            context_window: 16_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0003, output_cost_per_1k: 0.0003,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-03".into()), quality_rating: 3, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        // Reka
        ModelInfo {
            id: "reka-core-20250401".into(), name: "Reka Core".into(), provider: "Reka".into(),
            context_window: 128_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.003, output_cost_per_1k: 0.015,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-01".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "reka-flash-20250401".into(), name: "Reka Flash".into(), provider: "Reka".into(),
            context_window: 128_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0008, output_cost_per_1k: 0.002,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-01".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "reka-edge-20250401".into(), name: "Reka Edge".into(), provider: "Reka".into(),
            context_window: 64_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0002, output_cost_per_1k: 0.0006,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-12".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // Hugging Face
        ModelInfo {
            id: "HuggingFaceH4/zephyr-7b-beta".into(), name: "Zephyr 7B".into(), provider: "HuggingFace".into(),
            context_window: 4_096, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0001,
            supports_vision: false, supports_tools: false, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2023-10".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "mistralai/Mistral-7B-Instruct-v0.3".into(), name: "Mistral 7B v0.3 (HF)".into(), provider: "HuggingFace".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0001,
            supports_vision: false, supports_tools: false, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2023-12".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // Novita
        ModelInfo {
            id: "deepseek/deepseek-r1".into(), name: "DeepSeek R1 (Novita)".into(), provider: "Novita".into(),
            context_window: 131_072, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0007, output_cost_per_1k: 0.0028,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-11".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: true, free_tier: true,
        },
        // Hyperbolic
        ModelInfo {
            id: "meta-llama/Meta-Llama-3.1-70B-Instruct".into(), name: "Llama 3.1 70B (Hyperbolic)".into(), provider: "Hyperbolic".into(),
            context_window: 131_072, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0004, output_cost_per_1k: 0.0004,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-07".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        // Lepton
        ModelInfo {
            id: "llama3-70b".into(), name: "Llama 3 70B (Lepton)".into(), provider: "Lepton".into(),
            context_window: 8_192, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0006, output_cost_per_1k: 0.0006,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2023-12".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        // RunPod
        ModelInfo {
            id: "runpod-llama3-70b".into(), name: "Llama 3 70B (RunPod)".into(), provider: "RunPod".into(),
            context_window: 8_192, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0005, output_cost_per_1k: 0.0005,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2023-12".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        // Modal
        ModelInfo {
            id: "modal-llama3-70b".into(), name: "Llama 3 70B (Modal)".into(), provider: "Modal".into(),
            context_window: 8_192, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0004, output_cost_per_1k: 0.0004,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2023-12".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        // Chutes (Free!)
        ModelInfo {
            id: "chutesai/Llama-4-Maverick-17B-128E-Instruct".into(), name: "Llama 4 Maverick (Chutes)".into(), provider: "Chutes".into(),
            context_window: 1_048_576, max_output_tokens: 16_384,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-04".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "chutesai/DeepSeek-R1".into(), name: "DeepSeek R1 (Chutes)".into(), provider: "Chutes".into(),
            context_window: 131_072, max_output_tokens: 16_384,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-11".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: true, free_tier: true,
        },
        ModelInfo {
            id: "chutesai/Qwen3-235B-A22B".into(), name: "Qwen3 235B (Chutes)".into(), provider: "Chutes".into(),
            context_window: 131_072, max_output_tokens: 16_384,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-03".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: true, free_tier: true,
        },
        // FriendliAI
        ModelInfo {
            id: "meta-llama-3.3-70b-instruct-friendli".into(), name: "Llama 3.3 70B (Friendli)".into(), provider: "FriendliAI".into(),
            context_window: 131_072, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0007, output_cost_per_1k: 0.0007,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-12".into()), quality_rating: 5, speed_rating: 5,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "qwen2.5-72b-instruct-friendli".into(), name: "Qwen 2.5 72B (Friendli)".into(), provider: "FriendliAI".into(),
            context_window: 131_072, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0007, output_cost_per_1k: 0.0007,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-09".into()), quality_rating: 5, speed_rating: 5,
            is_reasoning: false, free_tier: false,
        },
        // OctoAI
        ModelInfo {
            id: "octoai-llama3.1-70b".into(), name: "Llama 3.1 70B (OctoAI)".into(), provider: "OctoAI".into(),
            context_window: 131_072, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0009, output_cost_per_1k: 0.0009,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-07".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "octoai-llama3.1-8b".into(), name: "Llama 3.1 8B (OctoAI)".into(), provider: "OctoAI".into(),
            context_window: 131_072, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0001,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-07".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // Voyage AI (Embeddings)
        ModelInfo {
            id: "voyage-3".into(), name: "Voyage 3".into(), provider: "Voyage".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.00006, output_cost_per_1k: 0.00006,
            supports_vision: false, supports_tools: false, supports_streaming: false, supports_json: true,
            training_cutoff: Some("2024-06".into()), quality_rating: 5, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "voyage-code-3".into(), name: "Voyage Code 3".into(), provider: "Voyage".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.00006, output_cost_per_1k: 0.00006,
            supports_vision: false, supports_tools: false, supports_streaming: false, supports_json: true,
            training_cutoff: Some("2024-08".into()), quality_rating: 5, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // Open-source models (available via Ollama/vLLM/Together)
        ModelInfo {
            id: "phi-4".into(), name: "Phi-4 (14B)".into(), provider: "Microsoft".into(),
            context_window: 16_384, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0001,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-11".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "phi-4-mini".into(), name: "Phi-4 Mini".into(), provider: "Microsoft".into(),
            context_window: 128_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.00005, output_cost_per_1k: 0.00005,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-02".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "phi-3.5-moe-instruct".into(), name: "Phi-3.5 MoE".into(), provider: "Microsoft".into(),
            context_window: 128_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0001,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-08".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "Yi-Coder-9B-Chat".into(), name: "Yi Coder 9B".into(), provider: "Yi".into(),
            context_window: 128_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0001,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-09".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "internlm3-8b-instruct".into(), name: "InternLM 3 8B".into(), provider: "InternLM".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0001,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-10".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "deepseek-r1-distill-qwen-32b".into(), name: "DeepSeek R1 Distill Qwen 32B".into(), provider: "DeepSeek".into(),
            context_window: 131_072, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0003,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-11".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: true, free_tier: true,
        },
        ModelInfo {
            id: "deepseek-r1-distill-llama-8b".into(), name: "DeepSeek R1 Distill Llama 8B".into(), provider: "DeepSeek".into(),
            context_window: 131_072, max_output_tokens: 4_096,
            input_cost_per_1k: 0.00001, output_cost_per_1k: 0.00003,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-11".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: true, free_tier: true,
        },
        ModelInfo {
            id: "command-a-03-2025".into(), name: "Command A (Cohere)".into(), provider: "Cohere".into(),
            context_window: 256_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0025, output_cost_per_1k: 0.01,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-03".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        // Nemotron
        ModelInfo {
            id: "nvidia/llama-3.1-nemotron-70b-instruct".into(), name: "Nemotron 70B".into(), provider: "NVIDIA".into(),
            context_window: 131_072, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0008, output_cost_per_1k: 0.0008,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-10".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        // Granite (IBM)
        ModelInfo {
            id: "ibm/granite-3.3-8b-instruct".into(), name: "Granite 3.3 8B".into(), provider: "WatsonX".into(),
            context_window: 128_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0002, output_cost_per_1k: 0.0002,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-02".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "ibm/granite-3.3-34b-instruct".into(), name: "Granite 3.3 34B".into(), provider: "WatsonX".into(),
            context_window: 128_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0006, output_cost_per_1k: 0.0006,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-02".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        // Ollama local models
        ModelInfo {
            id: "phi4:14b".into(), name: "Phi-4 14B (Ollama)".into(), provider: "Ollama".into(),
            context_window: 16_384, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-11".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "gemma3:27b".into(), name: "Gemma 3 27B (Ollama)".into(), provider: "Ollama".into(),
            context_window: 128_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-02".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "starcoder2:15b".into(), name: "StarCoder2 15B (Ollama)".into(), provider: "Ollama".into(),
            context_window: 16_384, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-02".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "llama3.2:3b".into(), name: "Llama 3.2 3B (Ollama)".into(), provider: "Ollama".into(),
            context_window: 131_072, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-07".into()), quality_rating: 2, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // ── Llama 4 Scout (2025) - 10M context, MoE 109B ──
        ModelInfo {
            id: "llama4:scout".into(), name: "Llama 4 Scout 109B MoE (Ollama)".into(), provider: "Ollama".into(),
            context_window: 10_485_760, max_output_tokens: 16_384,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-04".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: false, free_tier: true,
        },
        // ── Llama 4 Maverick (2025) - 1M context, MoE 400B ──
        ModelInfo {
            id: "llama4:maverick".into(), name: "Llama 4 Maverick 400B MoE (Ollama)".into(), provider: "Ollama".into(),
            context_window: 1_048_576, max_output_tokens: 16_384,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-04".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: false, free_tier: true,
        },
        // ── Qwen3 MoE - çok hafif, 3B aktif parametre ──
        ModelInfo {
            id: "qwen3:30b-a3b".into(), name: "Qwen3 30B-A3B MoE (Ollama)".into(), provider: "Ollama".into(),
            context_window: 131_072, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-03".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: true, free_tier: true,
        },
        // ── Gemma 3 4B - küçük ama güçlü ──
        ModelInfo {
            id: "gemma3:4b".into(), name: "Gemma 3 4B (Ollama)".into(), provider: "Ollama".into(),
            context_window: 128_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-02".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // ── Gemma 3 12B ──
        ModelInfo {
            id: "gemma3:12b".into(), name: "Gemma 3 12B (Ollama)".into(), provider: "Ollama".into(),
            context_window: 128_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-02".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        // ── Phi-4 Mini - 4GB VRAM ile çalışır ──
        ModelInfo {
            id: "phi4-mini:5b".into(), name: "Phi-4 Mini 5B (Ollama)".into(), provider: "Ollama".into(),
            context_window: 128_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-02".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // ── Mistral Small 3.1 24B - vision destekli ──
        ModelInfo {
            id: "mistral-small3.1:24b".into(), name: "Mistral Small 3.1 24B (Ollama)".into(), provider: "Ollama".into(),
            context_window: 128_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-02".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        // ── Command R (Cohere) - açık kaynak ──
        ModelInfo {
            id: "command-r:35b".into(), name: "Command R 35B (Ollama)".into(), provider: "Ollama".into(),
            context_window: 128_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-08".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        // ── DeepSeek R1 8B distill - en küçük reasoning ──
        ModelInfo {
            id: "deepseek-r1:8b".into(), name: "DeepSeek R1 Distill 8B (Ollama)".into(), provider: "Ollama".into(),
            context_window: 131_072, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-11".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: true, free_tier: true,
        },
        // ── DeepSeek R1 14B distill ──
        ModelInfo {
            id: "deepseek-r1:14b".into(), name: "DeepSeek R1 Distill 14B (Ollama)".into(), provider: "Ollama".into(),
            context_window: 131_072, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-11".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: true, free_tier: true,
        },
        // ── Qwen2.5 Coder 7B - küçük kod modeli ──
        ModelInfo {
            id: "qwen2.5-coder:7b".into(), name: "Qwen 2.5 Coder 7B (Ollama)".into(), provider: "Ollama".into(),
            context_window: 131_072, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-11".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // ── Qwen2.5 Coder 14B ──
        ModelInfo {
            id: "qwen2.5-coder:14b".into(), name: "Qwen 2.5 Coder 14B (Ollama)".into(), provider: "Ollama".into(),
            context_window: 131_072, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-11".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        // ── Granite 3.3 8B (IBM) - açık kaynak ──
        ModelInfo {
            id: "granite3.3:8b".into(), name: "Granite 3.3 8B (Ollama)".into(), provider: "Ollama".into(),
            context_window: 128_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-02".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // ── InternLM 3 8B - Çin açık kaynak ──
        ModelInfo {
            id: "internlm3:8b".into(), name: "InternLM 3 8B (Ollama)".into(), provider: "Ollama".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-10".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // ── Yi Coder 9B - Çin kod modeli ──
        ModelInfo {
            id: "yi-coder:9b".into(), name: "Yi Coder 9B (Ollama)".into(), provider: "Ollama".into(),
            context_window: 128_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-09".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // ── DBRX Instruct (Databricks) - açık kaynak MoE ──
        ModelInfo {
            id: "dbrx:132b".into(), name: "DBRX 132B MoE (Ollama)".into(), provider: "Ollama".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-03".into()), quality_rating: 4, speed_rating: 3,
            is_reasoning: false, free_tier: true,
        },
        // ── Llama 3.2 1B - en küçük Llama ──
        ModelInfo {
            id: "llama3.2:1b".into(), name: "Llama 3.2 1B (Ollama)".into(), provider: "Ollama".into(),
            context_window: 131_072, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-07".into()), quality_rating: 1, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // ── Pixtral 12B - Mistral'ın vision modeli ──
        ModelInfo {
            id: "pixtral:12b".into(), name: "Pixtral 12B Vision (Ollama)".into(), provider: "Ollama".into(),
            context_window: 128_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0, output_cost_per_1k: 0.0,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-09".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        // Together AI additional models
        ModelInfo {
            id: "togethercomputer/StripedHyena-Nous-7B".into(), name: "StripedHyena 7B (Together)".into(), provider: "Together".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0001,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-01".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "Qwen/Qwen2.5-Coder-32B-Instruct".into(), name: "Qwen 2.5 Coder 32B (Together)".into(), provider: "Together".into(),
            context_window: 131_072, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0004, output_cost_per_1k: 0.0004,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-11".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "deepseek-ai/deepseek-r1".into(), name: "DeepSeek R1 (Together)".into(), provider: "Together".into(),
            context_window: 131_072, max_output_tokens: 8_192,
            input_cost_per_1k: 0.006, output_cost_per_1k: 0.006,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-11".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: true, free_tier: false,
        },
        ModelInfo {
            id: "mistralai/Mixtral-8x22B-Instruct-v0.1".into(), name: "Mixtral 8x22B (Together)".into(), provider: "Together".into(),
            context_window: 65_536, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0006, output_cost_per_1k: 0.0006,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-01".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        // OpenRouter additional
        ModelInfo {
            id: "openrouter/google/gemini-2.5-pro-preview".into(), name: "Gemini 2.5 Pro (OpenRouter)".into(), provider: "OpenRouter".into(),
            context_window: 1_048_576, max_output_tokens: 65_536,
            input_cost_per_1k: 0.00125, output_cost_per_1k: 0.01,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-01".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: true, free_tier: false,
        },
        ModelInfo {
            id: "openrouter/anthropic/claude-sonnet-4".into(), name: "Claude Sonnet 4 (OpenRouter)".into(), provider: "OpenRouter".into(),
            context_window: 200_000, max_output_tokens: 16_384,
            input_cost_per_1k: 0.003, output_cost_per_1k: 0.015,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-02".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "openrouter/x-ai/grok-3".into(), name: "Grok 3 (OpenRouter)".into(), provider: "OpenRouter".into(),
            context_window: 131_072, max_output_tokens: 16_384,
            input_cost_per_1k: 0.003, output_cost_per_1k: 0.015,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-02".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "openrouter/deepseek/deepseek-r1".into(), name: "DeepSeek R1 (OpenRouter)".into(), provider: "OpenRouter".into(),
            context_window: 131_072, max_output_tokens: 16_384,
            input_cost_per_1k: 0.00055, output_cost_per_1k: 0.00219,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-11".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: true, free_tier: true,
        },
        ModelInfo {
            id: "openrouter/meta-llama/llama-4-maverick".into(), name: "Llama 4 Maverick (OpenRouter)".into(), provider: "OpenRouter".into(),
            context_window: 1_048_576, max_output_tokens: 16_384,
            input_cost_per_1k: 0.0015, output_cost_per_1k: 0.002,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-04".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        // Cerebras models
        ModelInfo {
            id: "llama3.1-8b".into(), name: "Llama 3.1 8B (Cerebras)".into(), provider: "Cerebras".into(),
            context_window: 128_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0001,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-07".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "qwen-2.5-coder-32b".into(), name: "Qwen 2.5 Coder 32B (Cerebras)".into(), provider: "Cerebras".into(),
            context_window: 128_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0004, output_cost_per_1k: 0.0004,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-11".into()), quality_rating: 5, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // DeepInfra additional
        ModelInfo {
            id: "deepseek-ai/DeepSeek-R1".into(), name: "DeepSeek R1 (DeepInfra)".into(), provider: "DeepInfra".into(),
            context_window: 131_072, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0008, output_cost_per_1k: 0.0032,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-11".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: true, free_tier: false,
        },
        ModelInfo {
            id: "Qwen/Qwen3-235B-A22B".into(), name: "Qwen3 235B (DeepInfra)".into(), provider: "DeepInfra".into(),
            context_window: 131_072, max_output_tokens: 16_384,
            input_cost_per_1k: 0.0014, output_cost_per_1k: 0.0042,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-03".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: true, free_tier: false,
        },
        // Japanese AI
        ModelInfo {
            id: "rinna/japanese-gpt-neox-3.6b".into(), name: "Rinna 3.6B (Japanese)".into(), provider: "Rinna".into(),
            context_window: 2_048, max_output_tokens: 1_024,
            input_cost_per_1k: 0.00005, output_cost_per_1k: 0.00005,
            supports_vision: false, supports_tools: false, supports_streaming: true, supports_json: false,
            training_cutoff: Some("2022-12".into()), quality_rating: 2, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        ModelInfo {
            id: "cyberagent/calm3-22b-chat".into(), name: "CALM 3 22B (Japanese)".into(), provider: "CyberAgent".into(),
            context_window: 32_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0003, output_cost_per_1k: 0.0003,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-06".into()), quality_rating: 4, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        // Arabic AI
        ModelInfo {
            id: "inceptionai/jais-30b-chat".into(), name: "JAIS 30B (Arabic)".into(), provider: "InceptionAI".into(),
            context_window: 8_192, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0005, output_cost_per_1k: 0.0005,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2023-06".into()), quality_rating: 3, speed_rating: 4,
            is_reasoning: false, free_tier: true,
        },
        // French AI (LightOn)
        ModelInfo {
            id: "lighton/paradigm-3b".into(), name: "Paradigm 3B (French)".into(), provider: "LightOn".into(),
            context_window: 4_096, max_output_tokens: 2_048,
            input_cost_per_1k: 0.00005, output_cost_per_1k: 0.00005,
            supports_vision: false, supports_tools: false, supports_streaming: true, supports_json: false,
            training_cutoff: Some("2023-03".into()), quality_rating: 2, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // Amazon Titan
        ModelInfo {
            id: "amazon.titan-text-express-v1".into(), name: "Amazon Titan Express".into(), provider: "Bedrock".into(),
            context_window: 8_000, max_output_tokens: 3_000,
            input_cost_per_1k: 0.0008, output_cost_per_1k: 0.0008,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-04".into()), quality_rating: 3, speed_rating: 5,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "amazon.nova-pro-v1:0".into(), name: "Amazon Nova Pro".into(), provider: "Bedrock".into(),
            context_window: 300_000, max_output_tokens: 4_096,
            input_cost_per_1k: 0.0008, output_cost_per_1k: 0.0032,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2025-01".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        // Cohere Embed
        ModelInfo {
            id: "embed-v4.0".into(), name: "Cohere Embed V4".into(), provider: "Cohere".into(),
            context_window: 128_000, max_output_tokens: 0,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: false, supports_streaming: false, supports_json: false,
            training_cutoff: Some("2024-10".into()), quality_rating: 5, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // OpenAI Embed
        ModelInfo {
            id: "text-embedding-3-large".into(), name: "OpenAI Embed 3 Large".into(), provider: "OpenAI".into(),
            context_window: 8_191, max_output_tokens: 0,
            input_cost_per_1k: 0.00013, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: false, supports_streaming: false, supports_json: false,
            training_cutoff: Some("2024-04".into()), quality_rating: 5, speed_rating: 5,
            is_reasoning: false, free_tier: false,
        },
        ModelInfo {
            id: "text-embedding-3-small".into(), name: "OpenAI Embed 3 Small".into(), provider: "OpenAI".into(),
            context_window: 8_191, max_output_tokens: 0,
            input_cost_per_1k: 0.00002, output_cost_per_1k: 0.0,
            supports_vision: false, supports_tools: false, supports_streaming: false, supports_json: false,
            training_cutoff: Some("2024-04".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: false,
        },
        // GPT-4o audio
        ModelInfo {
            id: "gpt-4o-audio-preview".into(), name: "GPT-4o Audio".into(), provider: "OpenAI".into(),
            context_window: 128_000, max_output_tokens: 16_384,
            input_cost_per_1k: 0.0025, output_cost_per_1k: 0.01,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-04".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        // SiliconFlow additional
        ModelInfo {
            id: "Pro/deepseek-ai/DeepSeek-R1".into(), name: "DeepSeek R1 (SiliconFlow Pro)".into(), provider: "SiliconFlow".into(),
            context_window: 131_072, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0008, output_cost_per_1k: 0.0032,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2024-11".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: true, free_tier: false,
        },
        // ═══════════════════════════════════════════════════════════
        //  2026 GÜNCEL - En Yeni Modeller (Nisan 2026)
        // ═══════════════════════════════════════════════════════════

        // ── OpenAI o4-mini (Mart 2026) ──
        ModelInfo {
            id: "o4-mini".into(), name: "o4 Mini (2026)".into(), provider: "OpenAI".into(),
            context_window: 200_000, max_output_tokens: 100_000,
            input_cost_per_1k: 0.0011, output_cost_per_1k: 0.0044,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: false,
            training_cutoff: Some("2025-11".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: true, free_tier: false,
        },
        // ── Gemini 2.5 Pro (Mart 2026 güncelleme) ──
        ModelInfo {
            id: "gemini-2.5-pro-latest".into(), name: "Gemini 2.5 Pro (2026)".into(), provider: "Google".into(),
            context_window: 1_048_576, max_output_tokens: 65_536,
            input_cost_per_1k: 0.00125, output_cost_per_1k: 0.01,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2026-02".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: true, free_tier: false,
        },
        // ── Gemini 2.5 Flash (Mart 2026 güncelleme) ──
        ModelInfo {
            id: "gemini-2.5-flash-latest".into(), name: "Gemini 2.5 Flash (2026)".into(), provider: "Google".into(),
            context_window: 1_048_576, max_output_tokens: 65_536,
            input_cost_per_1k: 0.00015, output_cost_per_1k: 0.0006,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2026-02".into()), quality_rating: 5, speed_rating: 5,
            is_reasoning: true, free_tier: true,
        },
        // ── Claude Opus 4.1 (Şubat 2026) ──
        ModelInfo {
            id: "claude-opus-4-1-20260215".into(), name: "Claude Opus 4.1 (2026)".into(), provider: "Anthropic".into(),
            context_window: 200_000, max_output_tokens: 32_000,
            input_cost_per_1k: 0.015, output_cost_per_1k: 0.075,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2026-01".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: true, free_tier: false,
        },
        // ── Grok 4 (Mart 2026) ──
        ModelInfo {
            id: "grok-4".into(), name: "Grok 4 (2026)".into(), provider: "xAI".into(),
            context_window: 262_144, max_output_tokens: 32_768,
            input_cost_per_1k: 0.005, output_cost_per_1k: 0.025,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2026-02".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        // ── DeepSeek V4 (Şubat 2026) ──
        ModelInfo {
            id: "deepseek-v4".into(), name: "DeepSeek V4 (2026)".into(), provider: "DeepSeek".into(),
            context_window: 256_000, max_output_tokens: 16_384,
            input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0004,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2026-01".into()), quality_rating: 5, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
        // ── DeepSeek R2 (Mart 2026) - güncel reasoning ──
        ModelInfo {
            id: "deepseek-r2".into(), name: "DeepSeek R2 (2026)".into(), provider: "DeepSeek".into(),
            context_window: 256_000, max_output_tokens: 32_768,
            input_cost_per_1k: 0.0008, output_cost_per_1k: 0.0032,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2026-02".into()), quality_rating: 5, speed_rating: 3,
            is_reasoning: true, free_tier: true,
        },
        // ── Qwen4 Max (Mart 2026) ──
        ModelInfo {
            id: "qwen4-max".into(), name: "Qwen4 Max (2026)".into(), provider: "Qwen".into(),
            context_window: 1_048_576, max_output_tokens: 32_768,
            input_cost_per_1k: 0.002, output_cost_per_1k: 0.008,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2026-03".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: true, free_tier: false,
        },
        // ── Mistral Large 3 (2026) ──
        ModelInfo {
            id: "mistral-large-3".into(), name: "Mistral Large 3 (2026)".into(), provider: "Mistral".into(),
            context_window: 256_000, max_output_tokens: 16_384,
            input_cost_per_1k: 0.003, output_cost_per_1k: 0.009,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2026-02".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        // ── Command R2 (Cohere 2026) ──
        ModelInfo {
            id: "command-r2".into(), name: "Command R2 (2026)".into(), provider: "Cohere".into(),
            context_window: 256_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.003, output_cost_per_1k: 0.012,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2026-01".into()), quality_rating: 5, speed_rating: 4,
            is_reasoning: false, free_tier: false,
        },
        // ── Sonar Deep Research v2 (Perplexity 2026) ──
        ModelInfo {
            id: "sonar-deep-research-v2".into(), name: "Sonar Deep Research v2 (2026)".into(), provider: "Perplexity".into(),
            context_window: 200_000, max_output_tokens: 32_768,
            input_cost_per_1k: 0.003, output_cost_per_1k: 0.015,
            supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2026-03".into()), quality_rating: 5, speed_rating: 1,
            is_reasoning: true, free_tier: false,
        },
        // ── Granite 4.0 (IBM 2026) - açık kaynak ──
        ModelInfo {
            id: "ibm/granite-4.0-8b".into(), name: "Granite 4.0 8B (2026)".into(), provider: "WatsonX".into(),
            context_window: 128_000, max_output_tokens: 8_192,
            input_cost_per_1k: 0.0002, output_cost_per_1k: 0.0002,
            supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
            training_cutoff: Some("2026-01".into()), quality_rating: 4, speed_rating: 5,
            is_reasoning: false, free_tier: true,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ALL MODELS - AGGREGATED
// ═══════════════════════════════════════════════════════════════════════════════

/// Get all available models (200+)
pub fn all_models() -> Vec<ModelInfo> {
    let mut models = Vec::new();
    models.extend(openai_models());
    models.extend(anthropic_models());
    models.extend(google_models());
    models.extend(mistral_models());
    models.extend(deepseek_models());
    models.extend(xai_models());
    models.extend(llama_models());
    models.extend(cohere_models());
    models.extend(perplexity_models());
    models.extend(groq_models());
    models.extend(ai21_models());
    models.extend(fireworks_models());
    models.extend(replicate_models());
    models.extend(qwen_models());
    models.extend(chinese_models());
    models.extend(russian_models());
    models.extend(korean_models());
    models.extend(european_models());
    models.extend(indian_models());
    models.extend(enterprise_models());
    models.extend(aggregator_models());
    models.extend(local_models());
    models.extend(specialized_models());
    models
}

/// Get models by provider
pub fn by_provider(provider: &str) -> Vec<ModelInfo> {
    all_models().into_iter()
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
    all_models().into_iter().filter(|m| m.free_tier).collect()
}

/// Get reasoning models
pub fn reasoning_models() -> Vec<ModelInfo> {
    all_models().into_iter().filter(|m| m.is_reasoning).collect()
}

/// Get models with vision support
pub fn vision_models() -> Vec<ModelInfo> {
    all_models().into_iter().filter(|m| m.supports_vision).collect()
}

/// Get models sorted by era (oldest first)
pub fn by_era() -> Vec<ModelInfo> {
    let mut models = all_models();
    models.sort_by(|a, b| {
        let a_cutoff = a.training_cutoff.as_ref().map(|s| s.as_str()).unwrap_or("9999-99");
        let b_cutoff = b.training_cutoff.as_ref().map(|s| s.as_str()).unwrap_or("9999-99");
        a_cutoff.cmp(b_cutoff)
    });
    models
}

/// Get legacy/historical models (pre-2024)
pub fn legacy_models() -> Vec<ModelInfo> {
    all_models().into_iter()
        .filter(|m| m.training_cutoff.as_ref().map(|s| s.as_str() < "2024").unwrap_or(false))
        .collect()
}

/// Get cutting-edge models (2025+)
pub fn cutting_edge_models() -> Vec<ModelInfo> {
    all_models().into_iter()
        .filter(|m| m.training_cutoff.as_ref().map(|s| s.as_str() >= "2025").unwrap_or(false))
        .collect()
}

/// Find model by ID
pub fn find(id: &str) -> Option<ModelInfo> {
    all_models().into_iter().find(|m| m.id == id)
}

/// Count total models
pub fn count() -> usize {
    all_models().len()
}

/// Count unique providers
pub fn provider_count() -> usize {
    let models = all_models();
    let mut providers: Vec<&str> = models.iter().map(|m| m.provider.as_str()).collect();
    providers.sort();
    providers.dedup();
    providers.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_models_count() {
        let models = all_models();
        assert!(models.len() >= 200, "Should have 200+ models, got {}", models.len());
    }

    #[test]
    fn test_provider_count() {
        let count = provider_count();
        assert!(count >= 40, "Should have 40+ unique providers, got {}", count);
    }

    #[test]
    fn test_openai_models() {
        let models = openai_models();
        assert!(models.len() >= 15, "OpenAI should have 15+ models");
        assert!(models.iter().any(|m| m.id == "gpt-4o"));
        assert!(models.iter().any(|m| m.id == "o3"));
    }

    #[test]
    fn test_anthropic_models() {
        let models = anthropic_models();
        assert!(models.len() >= 10);
        assert!(models.iter().any(|m| m.id.contains("claude")));
    }

    #[test]
    fn test_google_models() {
        let models = google_models();
        assert!(models.len() >= 10);
        assert!(models.iter().any(|m| m.id.contains("gemini")));
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
    fn test_legacy_models() {
        let models = legacy_models();
        assert!(!models.is_empty());
        assert!(models.iter().any(|m| m.id.contains("gpt-2") || m.id.contains("gpt-3")));
    }

    #[test]
    fn test_cutting_edge_models() {
        let models = cutting_edge_models();
        assert!(!models.is_empty());
    }

    #[test]
    fn test_chinese_models() {
        let models = chinese_models();
        assert!(models.len() >= 5);
    }

    #[test]
    fn test_local_models() {
        let models = local_models();
        assert!(models.len() >= 5);
        assert!(models.iter().all(|m| m.input_cost_per_1k == 0.0));
    }

    #[test]
    fn test_reasoning_models() {
        let models = reasoning_models();
        assert!(models.len() >= 10);
    }

    #[test]
    fn test_vision_models() {
        let models = vision_models();
        assert!(!models.is_empty());
        assert!(models.iter().all(|m| m.supports_vision));
    }
}
