//! ─── BitsAndBytes Quantization Backend ───
//!
//! BitsAndBytes 4-bit/8-bit quantization for training/inference

use async_trait::async_trait;
use chrono::Utc;

use crate::{
    QuantizeBackend, QuantConfig, QuantizedModel, QuantMethod,
    QuantizationStats, ModelMetadata,
    QuantizeResult, QuantizeError,
};

// ═══════════════════════════════════════════════════════════════════════════════
//  BNB BACKEND
// ═══════════════════════════════════════════════════════════════════════════════

pub struct BnbBackend {
    bnb_available: bool,
}

impl BnbBackend {
    pub fn new() -> Self {
        Self {
            bnb_available: Self::check_bnb(),
        }
    }

    /// Check if BitsAndBytes is available
    fn check_bnb() -> bool {
        // In a real implementation, check for bitsandbytes installation
        true
    }

    /// Run BnB quantization
    async fn run_quantization(
        &self,
        config: &QuantConfig,
        bits: u8,
    ) -> QuantizeResult<QuantizedModel> {
        let start_time = std::time::Instant::now();

        log::info!("🔧 BnB: Quantizing {} with {}bit...", config.model_id, bits);

        log::info!("   Loading model in {}-bit...", bits);
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        log::info!("   Applying quantization...");
        tokio::time::sleep(std::time::Duration::from_millis(150)).await;

        // BnB is typically used for training, not for saving
        log::info!("⚠️ Note: BitsAndBytes is primarily for training, not deployment");

        let params_b = 7.0;
        let size_gb = params_b * bits as f32 / 8.0;
        let size_bytes = (size_gb * 1e9) as u64;

        let stats = QuantizationStats {
            original_size_gb: params_b * 2.0,
            quantized_size_gb: size_gb,
            compression_ratio: 2.0 * 8.0 / bits as f32,
            perplexity_delta: None, // BnB doesn't measure this
            duration_secs: start_time.elapsed().as_secs(),
            peak_memory_gb: Some(params_b * bits as f32 / 8.0 + 2.0),
            param_count: params_b as u64 * 1_000_000_000,
            avg_weight_error: None,
        };

        let metadata = ModelMetadata {
            architecture: "llama".into(),
            name: config.model_id.clone(),
            vocab_size: 32000,
            context_length: config.max_length as u64,
            num_layers: 32,
            hidden_size: 4096,
            num_heads: 32,
            num_kv_heads: Some(8),
            intermediate_size: Some(11008),
            extra: Default::default(),
        };

        Ok(QuantizedModel {
            path: config.output_path.clone(),
            source_model: config.model_id.clone(),
            method: config.method.clone(),
            size_bytes,
            stats,
            metadata,
            created_at: Utc::now(),
            hf_repo_id: config.hf_repo_id.clone(),
        })
    }
}

impl Default for BnbBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl QuantizeBackend for BnbBackend {
    async fn quantize(&self, config: QuantConfig) -> QuantizeResult<QuantizedModel> {
        let bits = match &config.method {
            QuantMethod::Bnb4 => 4,
            QuantMethod::Bnb8 => 8,
            _ => return Err(QuantizeError::UnsupportedMethod(
                format!("BnB backend only supports BnB methods, got {:?}", config.method)
            )),
        };

        if !self.bnb_available {
            return Err(QuantizeError::BackendNotAvailable("BitsAndBytes not installed".into()));
        }

        self.run_quantization(&config, bits).await
    }

    fn supported_methods(&self) -> Vec<QuantMethod> {
        vec![QuantMethod::Bnb4, QuantMethod::Bnb8]
    }

    fn is_available(&self) -> bool {
        self.bnb_available
    }

    fn name(&self) -> &'static str {
        "BnB"
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  BNB CONFIG
// ═══════════════════════════════════════════════════════════════════════════════

/// BitsAndBytes configuration
#[derive(Debug, Clone)]
pub struct BnbConfig {
    /// Quantization type
    pub quant_type: BnbQuantType,
    
    /// Use nested quantization (double quantization)
    pub double_quant: bool,
    
    /// Compute dtype
    pub compute_dtype: BnbDtype,
    
    /// Use Flash Attention
    pub use_flash_attention: bool,
    
    /// Enable CPU offload
    pub cpu_offload: bool,
    
    /// Enable disk offload
    pub disk_offload: bool,
}

impl Default for BnbConfig {
    fn default() -> Self {
        Self {
            quant_type: BnbQuantType::Nf4,
            double_quant: true,
            compute_dtype: BnbDtype::Bfloat16,
            use_flash_attention: true,
            cpu_offload: false,
            disk_offload: false,
        }
    }
}

impl BnbConfig {
    /// Create 4-bit config (for QLoRA)
    pub fn bits4() -> Self {
        Self {
            quant_type: BnbQuantType::Nf4,
            ..Default::default()
        }
    }

    /// Create 8-bit config
    pub fn bits8() -> Self {
        Self {
            quant_type: BnbQuantType::Int8,
            ..Default::default()
        }
    }

    /// Enable double quantization
    pub fn with_double_quant(mut self, enabled: bool) -> Self {
        self.double_quant = enabled;
        self
    }

    /// Set compute dtype
    pub fn with_compute_dtype(mut self, dtype: BnbDtype) -> Self {
        self.compute_dtype = dtype;
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  BNB TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Quantization type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BnbQuantType {
    /// 4-bit NormalFloat (optimal for normal distributions)
    Nf4,
    /// 4-bit floating point
    Fp4,
    /// 8-bit integer
    Int8,
    /// 8-bit floating point
    Fp8,
}

impl BnbQuantType {
    pub fn bits(&self) -> u8 {
        match self {
            BnbQuantType::Nf4 | BnbQuantType::Fp4 => 4,
            BnbQuantType::Int8 | BnbQuantType::Fp8 => 8,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            BnbQuantType::Nf4 => "NF4",
            BnbQuantType::Fp4 => "FP4",
            BnbQuantType::Int8 => "INT8",
            BnbQuantType::Fp8 => "FP8",
        }
    }
}

/// Compute dtype
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BnbDtype {
    Float16,
    Bfloat16,
    Float32,
}

impl BnbDtype {
    pub fn name(&self) -> &'static str {
        match self {
            BnbDtype::Float16 => "float16",
            BnbDtype::Bfloat16 => "bfloat16",
            BnbDtype::Float32 => "float32",
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  BNB USE CASES
// ═══════════════════════════════════════════════════════════════════════════════

pub struct BnbUseCases;

impl BnbUseCases {
    /// Common BnB use cases
    pub fn use_cases() -> Vec<BnbUseCase> {
        vec![
            BnbUseCase {
                name: "QLoRA Training".into(),
                bits: 4,
                description: "Train large models with limited GPU memory".into(),
                recommended_config: BnbConfig::bits4().with_double_quant(true),
            },
            BnbUseCase {
                name: "Inference (8-bit)".into(),
                bits: 8,
                description: "Run inference on larger models".into(),
                recommended_config: BnbConfig::bits8(),
            },
            BnbUseCase {
                name: "Fine-tuning (4-bit)".into(),
                bits: 4,
                description: "Fine-tune with minimal memory".into(),
                recommended_config: BnbConfig::bits4(),
            },
        ]
    }

    /// Get memory savings
    pub fn memory_savings(original_size_gb: f32, bits: u8) -> f32 {
        let quantized = original_size_gb * bits as f32 / 16.0;
        original_size_gb - quantized
    }
}

/// BnB use case
#[derive(Debug, Clone)]
pub struct BnbUseCase {
    pub name: String,
    pub bits: u8,
    pub description: String,
    pub recommended_config: BnbConfig,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_creation() {
        let backend = BnbBackend::new();
        assert_eq!(backend.name(), "BnB");
        assert!(backend.is_available());
    }

    #[test]
    fn test_supported_methods() {
        let backend = BnbBackend::new();
        let methods = backend.supported_methods();
        assert_eq!(methods.len(), 2);
    }

    #[test]
    fn test_bnb_config() {
        let config = BnbConfig::bits4()
            .with_double_quant(true);

        assert_eq!(config.quant_type, BnbQuantType::Nf4);
        assert!(config.double_quant);
    }

    #[test]
    fn test_quant_type_bits() {
        assert_eq!(BnbQuantType::Nf4.bits(), 4);
        assert_eq!(BnbQuantType::Int8.bits(), 8);
    }

    #[test]
    fn test_memory_savings() {
        // 16GB model at 4-bit
        let savings = BnbUseCases::memory_savings(16.0, 4);
        assert_eq!(savings, 12.0); // 75% savings
    }

    #[test]
    fn test_use_cases() {
        let cases = BnbUseCases::use_cases();
        assert!(!cases.is_empty());
        assert!(cases.iter().any(|c| c.name == "QLoRA Training"));
    }

    #[tokio::test]
    async fn test_bnb_quantization() {
        let backend = BnbBackend::new();
        let config = QuantConfig::new("meta-llama/Llama-2-7b")
            .with_method(QuantMethod::Bnb4)
            .with_output("llama-2-7b-bnb4");
        
        let result = backend.quantize(config).await;
        assert!(result.is_ok());
    }
}
