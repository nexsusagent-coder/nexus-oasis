//! ─── SENTIENT V-GATE MODEL REGISTRY ───
//!
//! Desteklenen tüm LLM modellerinin merkezi kayıt defteri.
//! Gemma 4 öncelikli entegrasyon.

use serde::{Deserialize, Serialize};

/// ─── Model Tanımı (Const-uyumlu) ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDefinition {
    pub id: &'static str,
    pub display_name: &'static str,
    pub provider: &'static str,
    pub context_length: u64,
    pub max_output_tokens: u32,
    pub supports_vision: bool,
    pub supports_audio: bool,
    pub supports_function_calling: bool,
    pub supports_thinking: bool,
    pub is_free: bool,
    #[serde(skip)]
    pub recommended_for: &'static [&'static str],
}

// ═══════════════════════════════════════════════════════════════════════════════
//  GEMMA 4 MODEL AİLESİ (ÖNCELİKLİ)
// ═══════════════════════════════════════════════════════════════════════════════

/// Gemma 4 modelleri - Google DeepMind
pub const GEMMA4_MODELS: &[ModelDefinition] = &[
    // ═══════════════════════════════════════════════════════════════════════════
    //  GEMMA 4 31B - VARSAYILAN MODEL (EN GÜÇLÜ)
    // ═══════════════════════════════════════════════════════════════════════════
    ModelDefinition {
        id: "google/gemma-4-31b-it:free",
        display_name: "Gemma 4 31B (Free)",
        provider: "openrouter",
        context_length: 262_144,  // 256K
        max_output_tokens: 16_384,
        supports_vision: true,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: true,
        is_free: true,
        recommended_for: &["general", "reasoning", "coding", "math", "multimodal", "long_context"],
    },
    ModelDefinition {
        id: "google/gemma-4-31b-it",
        display_name: "Gemma 4 31B",
        provider: "openrouter",
        context_length: 262_144,
        max_output_tokens: 16_384,
        supports_vision: true,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: true,
        is_free: false,
        recommended_for: &["production"],
    },
    
    // ═══════════════════════════════════════════════════════════════════════════
    //  GEMMA 4 26B A4B - MIXTURE OF EXPERTS (HIZLI)
    // ═══════════════════════════════════════════════════════════════════════════
    ModelDefinition {
        id: "google/gemma-4-26b-a4b-it:free",
        display_name: "Gemma 4 26B MoE (Free)",
        provider: "openrouter",
        context_length: 262_144,  // 256K
        max_output_tokens: 16_384,
        supports_vision: true,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: true,
        is_free: true,
        recommended_for: &["fast_inference", "agents", "function_calling"],
    },
    ModelDefinition {
        id: "google/gemma-4-26b-a4b-it",
        display_name: "Gemma 4 26B MoE",
        provider: "openrouter",
        context_length: 262_144,
        max_output_tokens: 16_384,
        supports_vision: true,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: true,
        is_free: false,
        recommended_for: &["production"],
    },
    
    // ═══════════════════════════════════════════════════════════════════════════
    //  GEMMA 4 E4B - EDGE (LAPTOP/EDGE CİHAZLAR)
    // ═══════════════════════════════════════════════════════════════════════════
    ModelDefinition {
        id: "google/gemma-4-e4b-it",
        display_name: "Gemma 4 E4B (Edge)",
        provider: "openrouter",
        context_length: 131_072,  // 128K
        max_output_tokens: 8_192,
        supports_vision: true,
        supports_audio: true,  // Audio desteği var!
        supports_function_calling: true,
        supports_thinking: true,
        is_free: false,
        recommended_for: &["edge", "laptop", "audio", "speech_to_text"],
    },
    
    // ═══════════════════════════════════════════════════════════════════════════
    //  GEMMA 4 E2B - MOBILE (EN HAFİF)
    // ═══════════════════════════════════════════════════════════════════════════
    ModelDefinition {
        id: "google/gemma-4-e2b-it",
        display_name: "Gemma 4 E2B (Mobile)",
        provider: "openrouter",
        context_length: 131_072,  // 128K
        max_output_tokens: 4_096,
        supports_vision: true,
        supports_audio: true,  // Audio desteği var!
        supports_function_calling: true,
        supports_thinking: true,
        is_free: false,
        recommended_for: &["mobile", "on_device", "audio"],
    },
];

// ═══════════════════════════════════════════════════════════════════════════════
//  DİĞER MODELLER
// ═══════════════════════════════════════════════════════════════════════════════

/// Qwen modelleri - Alibaba
pub const QWEN_MODELS: &[ModelDefinition] = &[
    ModelDefinition {
        id: "qwen/qwen3-235b-a22b-instruct",
        display_name: "Qwen3 235B MoE",
        provider: "openrouter",
        context_length: 131_072,
        max_output_tokens: 16_384,
        supports_vision: false,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: true,
        is_free: false,
        recommended_for: &["advanced_reasoning"],
    },
    ModelDefinition {
        id: "qwen/qwen3-32b-instruct",
        display_name: "Qwen3 32B",
        provider: "openrouter",
        context_length: 131_072,
        max_output_tokens: 8_192,
        supports_vision: false,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: true,
        is_free: false,
        recommended_for: &["coding"],
    },
    ModelDefinition {
        id: "qwen/qwen3-30b-a3b-instruct",
        display_name: "Qwen3 30B MoE",
        provider: "openrouter",
        context_length: 131_072,
        max_output_tokens: 8_192,
        supports_vision: false,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: true,
        is_free: false,
        recommended_for: &["fast_inference"],
    },
];

/// OpenAI modelleri
pub const OPENAI_MODELS: &[ModelDefinition] = &[
    ModelDefinition {
        id: "openai/gpt-4o",
        display_name: "GPT-4o",
        provider: "openrouter",
        context_length: 128_000,
        max_output_tokens: 16_384,
        supports_vision: true,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: false,
        is_free: false,
        recommended_for: &["general"],
    },
    ModelDefinition {
        id: "openai/gpt-4o-mini",
        display_name: "GPT-4o Mini",
        provider: "openrouter",
        context_length: 128_000,
        max_output_tokens: 16_384,
        supports_vision: true,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: false,
        is_free: false,
        recommended_for: &["fast_inference"],
    },
    ModelDefinition {
        id: "openai/o3-mini",
        display_name: "o3-mini",
        provider: "openrouter",
        context_length: 200_000,
        max_output_tokens: 100_000,
        supports_vision: false,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: true,
        is_free: false,
        recommended_for: &["reasoning"],
    },
];

/// Anthropic modelleri
pub const ANTHROPIC_MODELS: &[ModelDefinition] = &[
    ModelDefinition {
        id: "anthropic/claude-3.7-sonnet",
        display_name: "Claude 3.7 Sonnet",
        provider: "openrouter",
        context_length: 200_000,
        max_output_tokens: 16_384,
        supports_vision: true,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: true,
        is_free: false,
        recommended_for: &["coding", "reasoning"],
    },
    ModelDefinition {
        id: "anthropic/claude-3.5-sonnet",
        display_name: "Claude 3.5 Sonnet",
        provider: "openrouter",
        context_length: 200_000,
        max_output_tokens: 8_192,
        supports_vision: true,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: false,
        is_free: false,
        recommended_for: &["general"],
    },
    ModelDefinition {
        id: "anthropic/claude-3.5-haiku",
        display_name: "Claude 3.5 Haiku",
        provider: "openrouter",
        context_length: 200_000,
        max_output_tokens: 8_192,
        supports_vision: true,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: false,
        is_free: false,
        recommended_for: &["fast_inference"],
    },
];

/// Meta Llama modelleri
pub const LLAMA_MODELS: &[ModelDefinition] = &[
    ModelDefinition {
        id: "meta-llama/llama-3.3-70b-instruct",
        display_name: "Llama 3.3 70B",
        provider: "openrouter",
        context_length: 131_072,
        max_output_tokens: 8_192,
        supports_vision: false,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: false,
        is_free: false,
        recommended_for: &["general", "coding"],
    },
    ModelDefinition {
        id: "meta-llama/llama-3.2-90b-vision-instruct",
        display_name: "Llama 3.2 90B Vision",
        provider: "openrouter",
        context_length: 131_072,
        max_output_tokens: 8_192,
        supports_vision: true,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: false,
        is_free: false,
        recommended_for: &["multimodal"],
    },
    ModelDefinition {
        id: "meta-llama/llama-3.1-405b-instruct",
        display_name: "Llama 3.1 405B",
        provider: "openrouter",
        context_length: 131_072,
        max_output_tokens: 16_384,
        supports_vision: false,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: false,
        is_free: false,
        recommended_for: &["advanced_reasoning"],
    },
];

/// Mistral modelleri
pub const MISTRAL_MODELS: &[ModelDefinition] = &[
    ModelDefinition {
        id: "mistralai/mistral-large-2412",
        display_name: "Mistral Large",
        provider: "openrouter",
        context_length: 131_072,
        max_output_tokens: 8_192,
        supports_vision: false,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: false,
        is_free: false,
        recommended_for: &["general"],
    },
    ModelDefinition {
        id: "mistralai/mixtral-8x22b-instruct",
        display_name: "Mixtral 8x22B",
        provider: "openrouter",
        context_length: 65_536,
        max_output_tokens: 8_192,
        supports_vision: false,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: false,
        is_free: false,
        recommended_for: &["fast_inference"],
    },
    ModelDefinition {
        id: "mistralai/pixtral-large-2411",
        display_name: "Pixtral Large",
        provider: "openrouter",
        context_length: 131_072,
        max_output_tokens: 8_192,
        supports_vision: true,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: false,
        is_free: false,
        recommended_for: &["multimodal"],
    },
];

/// DeepSeek modelleri
pub const DEEPSEEK_MODELS: &[ModelDefinition] = &[
    ModelDefinition {
        id: "deepseek/deepseek-r1",
        display_name: "DeepSeek R1",
        provider: "openrouter",
        context_length: 164_000,
        max_output_tokens: 32_768,
        supports_vision: false,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: true,
        is_free: false,
        recommended_for: &["reasoning", "math"],
    },
    ModelDefinition {
        id: "deepseek/deepseek-chat",
        display_name: "DeepSeek Chat",
        provider: "openrouter",
        context_length: 65_536,
        max_output_tokens: 8_192,
        supports_vision: false,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: false,
        is_free: false,
        recommended_for: &["general"],
    },
];

/// Groq modelleri (hızlı inference)
pub const GROQ_MODELS: &[ModelDefinition] = &[
    ModelDefinition {
        id: "groq/llama-3.3-70b-versatile",
        display_name: "Llama 3.3 70B (Groq)",
        provider: "groq",
        context_length: 131_072,
        max_output_tokens: 8_192,
        supports_vision: false,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: false,
        is_free: false,
        recommended_for: &["fast_inference"],
    },
    ModelDefinition {
        id: "groq/mixtral-8x7b-32768",
        display_name: "Mixtral 8x7B (Groq)",
        provider: "groq",
        context_length: 32_768,
        max_output_tokens: 8_192,
        supports_vision: false,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: false,
        is_free: false,
        recommended_for: &["fast_inference"],
    },
];

/// Cohere modelleri
pub const COHERE_MODELS: &[ModelDefinition] = &[
    ModelDefinition {
        id: "cohere/command-r-plus",
        display_name: "Command R+",
        provider: "openrouter",
        context_length: 131_072,
        max_output_tokens: 8_192,
        supports_vision: false,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: false,
        is_free: false,
        recommended_for: &["rag", "general"],
    },
    ModelDefinition {
        id: "cohere/command-r",
        display_name: "Command R",
        provider: "openrouter",
        context_length: 131_072,
        max_output_tokens: 8_192,
        supports_vision: false,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: false,
        is_free: false,
        recommended_for: &["rag"],
    },
];

/// Google Gemini modelleri
pub const GEMINI_MODELS: &[ModelDefinition] = &[
    ModelDefinition {
        id: "google/gemini-2.0-flash-exp",
        display_name: "Gemini 2.0 Flash",
        provider: "openrouter",
        context_length: 1_048_576,  // 1M!
        max_output_tokens: 8_192,
        supports_vision: true,
        supports_audio: true,
        supports_function_calling: true,
        supports_thinking: false,
        is_free: false,
        recommended_for: &["multimodal", "long_context"],
    },
    ModelDefinition {
        id: "google/gemini-exp-1206",
        display_name: "Gemini Exp",
        provider: "openrouter",
        context_length: 2_097_152,  // 2M!
        max_output_tokens: 8_192,
        supports_vision: true,
        supports_audio: true,
        supports_function_calling: true,
        supports_thinking: false,
        is_free: false,
        recommended_for: &["extreme_context"],
    },
];

/// Yerel modeller (Ollama)
pub const LOCAL_MODELS: &[ModelDefinition] = &[
    ModelDefinition {
        id: "local/gemma4:31b",
        display_name: "Gemma 4 31B (Local)",
        provider: "local",
        context_length: 262_144,
        max_output_tokens: 16_384,
        supports_vision: true,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: true,
        is_free: true,
        recommended_for: &["default", "local"],
    },
    ModelDefinition {
        id: "local/llama3.3:70b",
        display_name: "Llama 3.3 70B (Local)",
        provider: "local",
        context_length: 131_072,
        max_output_tokens: 8_192,
        supports_vision: false,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: false,
        is_free: true,
        recommended_for: &["local"],
    },
    ModelDefinition {
        id: "local/qwen2.5:72b",
        display_name: "Qwen 2.5 72B (Local)",
        provider: "local",
        context_length: 131_072,
        max_output_tokens: 8_192,
        supports_vision: false,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: false,
        is_free: true,
        recommended_for: &["local", "coding"],
    },
    ModelDefinition {
        id: "local/deepseek-r1:67b",
        display_name: "DeepSeek R1 67B (Local)",
        provider: "local",
        context_length: 131_072,
        max_output_tokens: 16_384,
        supports_vision: false,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: true,
        is_free: true,
        recommended_for: &["local", "reasoning"],
    },
    ModelDefinition {
        id: "local/mistral:24b",
        display_name: "Mistral 24B (Local)",
        provider: "local",
        context_length: 131_072,
        max_output_tokens: 8_192,
        supports_vision: false,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: false,
        is_free: true,
        recommended_for: &["local", "fast_inference"],
    },
];

/// Ücretsiz modeller (OpenRouter free tier)
pub const FREE_MODELS: &[ModelDefinition] = &[
    ModelDefinition {
        id: "google/gemma-4-31b-it:free",
        display_name: "Gemma 4 31B (Free)",
        provider: "openrouter",
        context_length: 262_144,
        max_output_tokens: 16_384,
        supports_vision: true,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: true,
        is_free: true,
        recommended_for: &["default"],
    },
    ModelDefinition {
        id: "google/gemma-4-26b-a4b-it:free",
        display_name: "Gemma 4 26B MoE (Free)",
        provider: "openrouter",
        context_length: 262_144,
        max_output_tokens: 16_384,
        supports_vision: true,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: true,
        is_free: true,
        recommended_for: &["fast"],
    },
    ModelDefinition {
        id: "meta-llama/llama-3.3-70b-instruct:free",
        display_name: "Llama 3.3 70B (Free)",
        provider: "openrouter",
        context_length: 131_072,
        max_output_tokens: 8_192,
        supports_vision: false,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: false,
        is_free: true,
        recommended_for: &["general"],
    },
    ModelDefinition {
        id: "qwen/qwen3-235b-a22b-instruct:free",
        display_name: "Qwen3 235B MoE (Free)",
        provider: "openrouter",
        context_length: 131_072,
        max_output_tokens: 8_192,
        supports_vision: false,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: true,
        is_free: true,
        recommended_for: &["advanced"],
    },
    ModelDefinition {
        id: "deepseek/deepseek-r1:free",
        display_name: "DeepSeek R1 (Free)",
        provider: "openrouter",
        context_length: 164_000,
        max_output_tokens: 16_384,
        supports_vision: false,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: true,
        is_free: true,
        recommended_for: &["reasoning", "math"],
    },
    ModelDefinition {
        id: "mistralai/mistral-small-24b-2501:free",
        display_name: "Mistral Small 24B (Free)",
        provider: "openrouter",
        context_length: 131_072,
        max_output_tokens: 8_192,
        supports_vision: false,
        supports_audio: false,
        supports_function_calling: true,
        supports_thinking: false,
        is_free: true,
        recommended_for: &["fast_inference"],
    },
];

// ═══════════════════════════════════════════════════════════════════════════════
//  VARSAYILAN MODEL
// ═══════════════════════════════════════════════════════════════════════════════

/// SENTIENT OS varsayılan modeli
pub const DEFAULT_MODEL: &str = "google/gemma-4-31b-it:free";

/// Varsayılan model tanımını al
pub const fn get_default_model() -> &'static ModelDefinition {
    &GEMMA4_MODELS[0]  // Gemma 4 31B Free
}

/// Model ID'den model tanımını bul
pub fn find_model(model_id: &str) -> Option<&'static ModelDefinition> {
    // Önce Gemma 4'te ara
    for model in GEMMA4_MODELS {
        if model.id == model_id {
            return Some(model);
        }
    }
    // Diğerlerinde ara
    for model in QWEN_MODELS {
        if model.id == model_id {
            return Some(model);
        }
    }
    for model in OPENAI_MODELS {
        if model.id == model_id {
            return Some(model);
        }
    }
    for model in ANTHROPIC_MODELS {
        if model.id == model_id {
            return Some(model);
        }
    }
    for model in LLAMA_MODELS {
        if model.id == model_id {
            return Some(model);
        }
    }
    for model in MISTRAL_MODELS {
        if model.id == model_id {
            return Some(model);
        }
    }
    for model in DEEPSEEK_MODELS {
        if model.id == model_id {
            return Some(model);
        }
    }
    for model in GROQ_MODELS {
        if model.id == model_id {
            return Some(model);
        }
    }
    for model in COHERE_MODELS {
        if model.id == model_id {
            return Some(model);
        }
    }
    for model in GEMINI_MODELS {
        if model.id == model_id {
            return Some(model);
        }
    }
    for model in LOCAL_MODELS {
        if model.id == model_id {
            return Some(model);
        }
    }
    for model in FREE_MODELS {
        if model.id == model_id {
            return Some(model);
        }
    }
    None
}

/// Tüm modelleri listele
pub fn all_models() -> Vec<&'static ModelDefinition> {
    GEMMA4_MODELS.iter()
        .chain(QWEN_MODELS.iter())
        .chain(OPENAI_MODELS.iter())
        .chain(ANTHROPIC_MODELS.iter())
        .chain(LLAMA_MODELS.iter())
        .chain(MISTRAL_MODELS.iter())
        .chain(DEEPSEEK_MODELS.iter())
        .chain(GROQ_MODELS.iter())
        .chain(COHERE_MODELS.iter())
        .chain(GEMINI_MODELS.iter())
        .chain(LOCAL_MODELS.iter())
        .chain(FREE_MODELS.iter())
        .collect()
}

/// Ücretsiz modelleri listele
pub fn free_models() -> Vec<&'static ModelDefinition> {
    FREE_MODELS.iter().collect()
}

/// Thinking mode destekleyen modeller
pub fn thinking_capable_models() -> Vec<&'static ModelDefinition> {
    all_models().into_iter()
        .filter(|m| m.supports_thinking)
        .collect()
}

/// Vision destekleyen modeller
pub fn vision_capable_models() -> Vec<&'static ModelDefinition> {
    all_models().into_iter()
        .filter(|m| m.supports_vision)
        .collect()
}

/// Audio destekleyen modeller
pub fn audio_capable_models() -> Vec<&'static ModelDefinition> {
    all_models().into_iter()
        .filter(|m| m.supports_audio)
        .collect()
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_model() {
        assert_eq!(DEFAULT_MODEL, "google/gemma-4-31b-it:free");
    }

    #[test]
    fn test_find_model() {
        let model = find_model("google/gemma-4-31b-it:free");
        assert!(model.is_some());
        assert_eq!(model.unwrap().context_length, 262_144);
    }

    #[test]
    fn test_gemma4_count() {
        assert_eq!(GEMMA4_MODELS.len(), 6);
    }

    #[test]
    fn test_free_models() {
        let free = free_models();
        assert!(free.iter().all(|m| m.is_free));
    }

    #[test]
    fn test_thinking_models() {
        let thinking = thinking_capable_models();
        assert!(thinking.len() > 0);
        assert!(thinking.iter().all(|m| m.supports_thinking));
    }

    #[test]
    fn test_vision_models() {
        let vision = vision_capable_models();
        assert!(vision.len() > 0);
        assert!(vision.iter().all(|m| m.supports_vision));
    }

    #[test]
    fn test_audio_models() {
        let audio = audio_capable_models();
        // Gemma 4 E2B ve E4B audio destekliyor
        assert!(audio.len() >= 2);
    }
}
