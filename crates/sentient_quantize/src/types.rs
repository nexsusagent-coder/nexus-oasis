//! ─── Quantization Types ───

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::QuantMethod;

// ═══════════════════════════════════════════════════════════════════════════════
//  QUANT CONFIG
// ═══════════════════════════════════════════════════════════════════════════════

/// Quantization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantConfig {
    /// Source model (HuggingFace ID or local path)
    pub model_id: String,
    
    /// Quantization method
    pub method: QuantMethod,
    
    /// Output path
    pub output_path: String,
    
    /// Calibration dataset (for GPTQ/AWQ)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calibration_data: Option<String>,
    
    /// Number of calibration samples
    pub calibration_samples: usize,
    
    /// Maximum sequence length
    pub max_length: usize,
    
    /// Device to use
    pub device: String,
    
    /// Whether to upload to HuggingFace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hf_repo_id: Option<String>,
    
    /// Additional metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl QuantConfig {
    /// Create new config
    pub fn new(model_id: impl Into<String>) -> Self {
        Self {
            model_id: model_id.into(),
            method: QuantMethod::gguf_q4km(),
            output_path: "quantized_model".into(),
            calibration_data: None,
            calibration_samples: 512,
            max_length: 2048,
            device: "cuda".into(),
            hf_repo_id: None,
            metadata: HashMap::new(),
        }
    }

    /// Set quantization method
    pub fn with_method(mut self, method: QuantMethod) -> Self {
        self.method = method;
        self
    }

    /// Set output path
    pub fn with_output(mut self, path: impl Into<String>) -> Self {
        self.output_path = path.into();
        self
    }

    /// Set calibration dataset
    pub fn with_calibration(mut self, data: impl Into<String>) -> Self {
        self.calibration_data = Some(data.into());
        self
    }

    /// Set number of calibration samples
    pub fn with_samples(mut self, samples: usize) -> Self {
        self.calibration_samples = samples;
        self
    }

    /// Set max sequence length
    pub fn with_max_length(mut self, length: usize) -> Self {
        self.max_length = length;
        self
    }

    /// Set device
    pub fn with_device(mut self, device: impl Into<String>) -> Self {
        self.device = device.into();
        self
    }

    /// Upload to HuggingFace
    pub fn upload_to(mut self, repo_id: impl Into<String>) -> Self {
        self.hf_repo_id = Some(repo_id.into());
        self
    }
}

impl Default for QuantConfig {
    fn default() -> Self {
        Self::new("model")
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  QUANTIZED MODEL
// ═══════════════════════════════════════════════════════════════════════════════

/// Quantized model result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantizedModel {
    /// Output path
    pub path: String,
    
    /// Original model ID
    pub source_model: String,
    
    /// Quantization method used
    pub method: QuantMethod,
    
    /// Model size in bytes
    pub size_bytes: u64,
    
    /// Quantization statistics
    pub stats: QuantizationStats,
    
    /// Model metadata
    pub metadata: ModelMetadata,
    
    /// Creation time
    pub created_at: DateTime<Utc>,
    
    /// HuggingFace repo ID (if uploaded)
    pub hf_repo_id: Option<String>,
}

impl QuantizedModel {
    /// Get size in GB
    pub fn size_gb(&self) -> f32 {
        self.size_bytes as f32 / 1e9
    }

    /// Get compression ratio
    pub fn compression_ratio(&self) -> f32 {
        self.stats.original_size_gb / self.size_gb()
    }

    /// Check if GGUF format
    pub fn is_gguf(&self) -> bool {
        matches!(self.method, QuantMethod::Gguf(_))
    }

    /// Check if GPTQ format
    pub fn is_gptq(&self) -> bool {
        matches!(self.method, QuantMethod::Gptq(_))
    }

    /// Check if AWQ format
    pub fn is_awq(&self) -> bool {
        matches!(self.method, QuantMethod::Awq(_))
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  QUANTIZATION STATS
// ═══════════════════════════════════════════════════════════════════════════════

/// Quantization statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantizationStats {
    /// Original model size in GB
    pub original_size_gb: f32,
    
    /// Quantized size in GB
    pub quantized_size_gb: f32,
    
    /// Compression ratio
    pub compression_ratio: f32,
    
    /// Perplexity change (if measured)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perplexity_delta: Option<f32>,
    
    /// Quantization time in seconds
    pub duration_secs: u64,
    
    /// Peak GPU memory in GB
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peak_memory_gb: Option<f32>,
    
    /// Number of parameters
    pub param_count: u64,
    
    /// Average weight error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_weight_error: Option<f32>,
}

impl Default for QuantizationStats {
    fn default() -> Self {
        Self {
            original_size_gb: 0.0,
            quantized_size_gb: 0.0,
            compression_ratio: 1.0,
            perplexity_delta: None,
            duration_secs: 0,
            peak_memory_gb: None,
            param_count: 0,
            avg_weight_error: None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  MODEL METADATA
// ═══════════════════════════════════════════════════════════════════════════════

/// Model metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    /// Model architecture
    pub architecture: String,
    
    /// Model name
    pub name: String,
    
    /// Vocabulary size
    pub vocab_size: u64,
    
    /// Context length
    pub context_length: u64,
    
    /// Number of layers
    pub num_layers: u32,
    
    /// Hidden size
    pub hidden_size: u32,
    
    /// Number of attention heads
    pub num_heads: u32,
    
    /// Number of key-value heads (for GQA)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_kv_heads: Option<u32>,
    
    /// Intermediate size (FFN)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermediate_size: Option<u32>,
    
    /// Additional properties
    #[serde(default)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Default for ModelMetadata {
    fn default() -> Self {
        Self {
            architecture: "unknown".into(),
            name: "unknown".into(),
            vocab_size: 0,
            context_length: 2048,
            num_layers: 0,
            hidden_size: 0,
            num_heads: 0,
            num_kv_heads: None,
            intermediate_size: None,
            extra: HashMap::new(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PRECISION
// ═══════════════════════════════════════════════════════════════════════════════

/// Precision type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Precision {
    #[serde(rename = "fp32")]
    Fp32,
    #[serde(rename = "fp16")]
    Fp16,
    #[serde(rename = "bf16")]
    Bf16,
    #[serde(rename = "int8")]
    Int8,
    #[serde(rename = "int4")]
    Int4,
    #[serde(rename = "nf4")]
    Nf4,
    #[serde(rename = "fp4")]
    Fp4,
}

impl Precision {
    /// Get bits per weight
    pub fn bits(&self) -> u8 {
        match self {
            Precision::Fp32 => 32,
            Precision::Fp16 => 16,
            Precision::Bf16 => 16,
            Precision::Int8 => 8,
            Precision::Int4 => 4,
            Precision::Nf4 => 4,
            Precision::Fp4 => 4,
        }
    }

    /// Get name
    pub fn name(&self) -> &'static str {
        match self {
            Precision::Fp32 => "FP32",
            Precision::Fp16 => "FP16",
            Precision::Bf16 => "BF16",
            Precision::Int8 => "INT8",
            Precision::Int4 => "INT4",
            Precision::Nf4 => "NF4",
            Precision::Fp4 => "FP4",
        }
    }
}

impl Default for Precision {
    fn default() -> Self {
        Precision::Fp16
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quant_config() {
        let config = QuantConfig::new("meta-llama/Llama-2-7b")
            .with_method(QuantMethod::gguf_q4km())
            .with_output("llama-2-7b-q4km.gguf");

        assert_eq!(config.model_id, "meta-llama/Llama-2-7b");
        assert!(config.output_path.contains("llama"));
    }

    #[test]
    fn test_precision_bits() {
        assert_eq!(Precision::Fp32.bits(), 32);
        assert_eq!(Precision::Fp16.bits(), 16);
        assert_eq!(Precision::Int4.bits(), 4);
        assert_eq!(Precision::Nf4.bits(), 4);
    }

    #[test]
    fn test_quantized_model() {
        let model = QuantizedModel {
            path: "model.gguf".into(),
            source_model: "llama-2-7b".into(),
            method: QuantMethod::gguf_q4km(),
            size_bytes: 4_000_000_000,
            stats: QuantizationStats {
                original_size_gb: 14.0,
                quantized_size_gb: 4.0,
                compression_ratio: 3.5,
                ..Default::default()
            },
            metadata: ModelMetadata::default(),
            created_at: Utc::now(),
            hf_repo_id: None,
        };

        assert_eq!(model.size_gb(), 4.0);
        assert!(model.is_gguf());
        assert!(!model.is_gptq());
    }

    #[test]
    fn test_stats_default() {
        let stats = QuantizationStats::default();
        assert_eq!(stats.compression_ratio, 1.0);
        assert_eq!(stats.duration_secs, 0);
    }
}
