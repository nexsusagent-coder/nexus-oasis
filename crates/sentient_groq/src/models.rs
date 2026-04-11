// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Groq Models
// ═══════════════════════════════════════════════════════════════════════════════
//  Available models on Groq LPU
//  Pricing: https://groq.com/pricing/
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};

/// Available Groq models
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroqModel {
    // Llama family
    #[serde(rename = "llama-3.3-70b-versatile")]
    Llama33_70B,
    
    #[serde(rename = "llama-3.1-8b-instant")]
    Llama31_8B,
    
    #[serde(rename = "llama-guard-3-8b")]
    LlamaGuard3_8B,
    
    // Mixtral family
    #[serde(rename = "mixtral-8x7b-32768")]
    Mixtral_8x7B,
    
    // Gemma family
    #[serde(rename = "gemma2-9b-it")]
    Gemma2_9B,
    
    // DeepSeek
    #[serde(rename = "deepseek-r1-distill-llama-70b")]
    DeepSeek_R1_70B,
    
    // Qwen
    #[serde(rename = "qwen-2.5-32b")]
    Qwen2_5_32B,
    
    // Compound (beta)
    #[serde(rename = "compound-beta")]
    CompoundBeta,
}

impl GroqModel {
    /// Get model ID string for API
    pub fn id(&self) -> &'static str {
        match self {
            Self::Llama33_70B => "llama-3.3-70b-versatile",
            Self::Llama31_8B => "llama-3.1-8b-instant",
            Self::LlamaGuard3_8B => "llama-guard-3-8b",
            Self::Mixtral_8x7B => "mixtral-8x7b-32768",
            Self::Gemma2_9B => "gemma2-9b-it",
            Self::DeepSeek_R1_70B => "deepseek-r1-distill-llama-70b",
            Self::Qwen2_5_32B => "qwen-2.5-32b",
            Self::CompoundBeta => "compound-beta",
        }
    }

    /// Get model family
    pub fn family(&self) -> GroqModelFamily {
        match self {
            Self::Llama33_70B | Self::Llama31_8B | Self::LlamaGuard3_8B => GroqModelFamily::Llama,
            Self::Mixtral_8x7B => GroqModelFamily::Mixtral,
            Self::Gemma2_9B => GroqModelFamily::Gemma,
            Self::DeepSeek_R1_70B => GroqModelFamily::DeepSeek,
            Self::Qwen2_5_32B => GroqModelFamily::Qwen,
            Self::CompoundBeta => GroqModelFamily::Compound,
        }
    }

    /// Get context window size
    pub fn context_length(&self) -> usize {
        match self {
            Self::Llama33_70B => 128_000,
            Self::Llama31_8B => 128_000,
            Self::LlamaGuard3_8B => 8_192,
            Self::Mixtral_8x7B => 32_768,
            Self::Gemma2_9B => 8_192,
            Self::DeepSeek_R1_70B => 128_000,
            Self::Qwen2_5_32B => 128_000,
            Self::CompoundBeta => 128_000,
        }
    }

    /// Get pricing per 1M tokens (input, output)
    pub fn pricing(&self) -> (f64, f64) {
        match self {
            Self::Llama33_70B => (0.59, 0.79),
            Self::Llama31_8B => (0.05, 0.08),
            Self::LlamaGuard3_8B => (0.05, 0.08),
            Self::Mixtral_8x7B => (0.24, 0.24),
            Self::Gemma2_9B => (0.20, 0.20),
            Self::DeepSeek_R1_70B => (0.75, 0.99),
            Self::Qwen2_5_32B => (0.30, 0.40),
            Self::CompoundBeta => (0.00, 0.00), // Free during beta
        }
    }

    /// Estimate cost for given tokens
    pub fn estimate_cost(&self, input_tokens: usize, output_tokens: usize) -> f64 {
        let (input_price, output_price) = self.pricing();
        let input_cost = (input_tokens as f64 / 1_000_000.0) * input_price;
        let output_cost = (output_tokens as f64 / 1_000_000.0) * output_price;
        input_cost + output_cost
    }

    /// Check if model supports vision
    pub fn supports_vision(&self) -> bool {
        matches!(self, Self::Llama33_70B | Self::CompoundBeta)
    }

    /// Check if model supports function calling
    pub fn supports_function_calling(&self) -> bool {
        matches!(
            self,
            Self::Llama33_70B |
            Self::Llama31_8B |
            Self::Mixtral_8x7B |
            Self::Gemma2_9B
        )
    }

    /// Get all available models
    pub fn all() -> Vec<Self> {
        vec![
            Self::Llama33_70B,
            Self::Llama31_8B,
            Self::LlamaGuard3_8B,
            Self::Mixtral_8x7B,
            Self::Gemma2_9B,
            Self::DeepSeek_R1_70B,
            Self::Qwen2_5_32B,
            Self::CompoundBeta,
        ]
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "llama-3.3-70b-versatile" | "llama-3.3-70b" | "llama33-70b" => Some(Self::Llama33_70B),
            "llama-3.1-8b-instant" | "llama-3.1-8b" | "llama31-8b" => Some(Self::Llama31_8B),
            "llama-guard-3-8b" | "llama-guard" => Some(Self::LlamaGuard3_8B),
            "mixtral-8x7b-32768" | "mixtral-8x7b" | "mixtral" => Some(Self::Mixtral_8x7B),
            "gemma2-9b-it" | "gemma2-9b" | "gemma" => Some(Self::Gemma2_9B),
            "deepseek-r1-distill-llama-70b" | "deepseek-r1" | "deepseek" => Some(Self::DeepSeek_R1_70B),
            "qwen-2.5-32b" | "qwen-2.5" | "qwen" => Some(Self::Qwen2_5_32B),
            "compound-beta" | "compound" => Some(Self::CompoundBeta),
            _ => None,
        }
    }
}

impl std::fmt::Display for GroqModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id())
    }
}

/// Model family
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroqModelFamily {
    Llama,
    Mixtral,
    Gemma,
    DeepSeek,
    Qwen,
    Compound,
}

impl std::fmt::Display for GroqModelFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Llama => write!(f, "Llama"),
            Self::Mixtral => write!(f, "Mixtral"),
            Self::Gemma => write!(f, "Gemma"),
            Self::DeepSeek => write!(f, "DeepSeek"),
            Self::Qwen => write!(f, "Qwen"),
            Self::Compound => write!(f, "Compound"),
        }
    }
}

/// Model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub owned_by: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_id() {
        assert_eq!(GroqModel::Llama33_70B.id(), "llama-3.3-70b-versatile");
        assert_eq!(GroqModel::Mixtral_8x7B.id(), "mixtral-8x7b-32768");
    }

    #[test]
    fn test_model_context_length() {
        assert_eq!(GroqModel::Llama33_70B.context_length(), 128_000);
        assert_eq!(GroqModel::Mixtral_8x7B.context_length(), 32_768);
    }

    #[test]
    fn test_model_pricing() {
        let (input, output) = GroqModel::Llama33_70B.pricing();
        assert_eq!(input, 0.59);
        assert_eq!(output, 0.79);
    }

    #[test]
    fn test_estimate_cost() {
        let cost = GroqModel::Llama33_70B.estimate_cost(1000, 500);
        // 1000 input * 0.59/1M + 500 output * 0.79/1M
        let expected = (1000.0 / 1_000_000.0 * 0.59) + (500.0 / 1_000_000.0 * 0.79);
        assert!((cost - expected).abs() < 0.0001);
    }

    #[test]
    fn test_model_from_str() {
        assert_eq!(GroqModel::from_str("llama33-70b"), Some(GroqModel::Llama33_70B));
        assert_eq!(GroqModel::from_str("mixtral"), Some(GroqModel::Mixtral_8x7B));
        assert_eq!(GroqModel::from_str("unknown"), None);
    }

    #[test]
    fn test_model_features() {
        assert!(GroqModel::Llama33_70B.supports_function_calling());
        assert!(GroqModel::Llama33_70B.supports_vision());
    }
}
