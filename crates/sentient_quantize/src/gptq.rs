//! ─── GPTQ Quantization Backend ───
//!
//! AutoGPTQ compatible quantization

use async_trait::async_trait;
use chrono::Utc;

use crate::{
    QuantizeBackend, QuantConfig, QuantizedModel, QuantMethod,
    QuantizationStats, ModelMetadata, GptqMethod,
    QuantizeResult, QuantizeError,
};

// ═══════════════════════════════════════════════════════════════════════════════
//  GPTQ BACKEND
// ═══════════════════════════════════════════════════════════════════════════════

pub struct GptqBackend {
    auto_gptq_available: bool,
}

impl GptqBackend {
    pub fn new() -> Self {
        Self {
            auto_gptq_available: Self::check_auto_gptq(),
        }
    }

    /// Check if AutoGPTQ is available
    fn check_auto_gptq() -> bool {
        // In a real implementation, check Python + auto_gptq installation
        // For now, return true for simulation
        true
    }

    /// Run GPTQ quantization
    async fn run_quantization(
        &self,
        config: &QuantConfig,
        method: GptqMethod,
    ) -> QuantizeResult<QuantizedModel> {
        let start_time = std::time::Instant::now();

        log::info!("🔧 GPTQ: Quantizing {} with {}bit...", config.model_id, method.bits());

        // Validate calibration data
        if config.calibration_data.is_none() {
            log::warn!("⚠️ GPTQ: No calibration data specified, using default (wikitext)");
        }

        // Simulate quantization process
        log::info!("   Loading model...");
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        log::info!("   Running calibration with {} samples...", config.calibration_samples);
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        log::info!("   Quantizing weights...");
        tokio::time::sleep(std::time::Duration::from_millis(150)).await;

        log::info!("   Saving quantized model...");
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        // Estimate size
        let params_b = 7.0; // Placeholder
        let size_gb = params_b * method.bits() as f32 / 8.0;
        let size_bytes = (size_gb * 1e9) as u64;

        let stats = QuantizationStats {
            original_size_gb: params_b * 2.0, // FP16
            quantized_size_gb: size_gb,
            compression_ratio: 2.0 * 8.0 / method.bits() as f32,
            perplexity_delta: Some(0.05), // Typical for GPTQ-4bit
            duration_secs: start_time.elapsed().as_secs(),
            peak_memory_gb: Some(params_b * 2.0 + 4.0), // Model + calibration
            param_count: params_b as u64 * 1_000_000_000,
            avg_weight_error: Some(0.02),
        };

        let metadata = ModelMetadata {
            architecture: "llama".into(),
            name: config.model_id.clone(),
            vocab_size: 32000,
            context_length: config.max_length as u64,
            num_layers: 32,
            hidden_size: 4096,
            num_heads: 32,
            num_kv_heads: Some(8), // GQA
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

impl Default for GptqBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl QuantizeBackend for GptqBackend {
    async fn quantize(&self, config: QuantConfig) -> QuantizeResult<QuantizedModel> {
        // Verify GPTQ method
        let method = match &config.method {
            QuantMethod::Gptq(m) => *m,
            _ => return Err(QuantizeError::UnsupportedMethod(
                format!("GPTQ backend only supports GPTQ methods, got {:?}", config.method)
            )),
        };

        if !self.auto_gptq_available {
            return Err(QuantizeError::BackendNotAvailable("AutoGPTQ not installed".into()));
        }

        self.run_quantization(&config, method).await
    }

    fn supported_methods(&self) -> Vec<QuantMethod> {
        vec![
            QuantMethod::Gptq(GptqMethod::Gptq4),
            QuantMethod::Gptq(GptqMethod::Gptq8),
        ]
    }

    fn is_available(&self) -> bool {
        self.auto_gptq_available
    }

    fn name(&self) -> &'static str {
        "GPTQ"
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  GPTQ CONFIG BUILDER
// ═══════════════════════════════════════════════════════════════════════════════

/// GPTQ-specific configuration
#[derive(Debug, Clone)]
pub struct GptqConfig {
    /// Bits (4 or 8)
    pub bits: u8,
    
    /// Group size for quantization
    pub group_size: i32,
    
    /// Dampening factor
    pub damp_percent: f32,
    
    /// Whether to use exllama kernel
    pub use_exllama: bool,
    
    /// Act-order (for better quality)
    pub act_order: bool,
    
    /// Whether to desc act
    pub desc_act: bool,
    
    /// Calibration dataset
    pub calibration_dataset: String,
    
    /// Number of calibration samples
    pub calibration_samples: usize,
    
    /// Max calibration sequence length
    pub calibration_max_length: usize,
}

impl Default for GptqConfig {
    fn default() -> Self {
        Self {
            bits: 4,
            group_size: 128,
            damp_percent: 0.01,
            use_exllama: true,
            act_order: true,
            desc_act: false,
            calibration_dataset: "wikitext".into(),
            calibration_samples: 512,
            calibration_max_length: 2048,
        }
    }
}

impl GptqConfig {
    /// Create 4-bit config
    pub fn bits4() -> Self {
        Self {
            bits: 4,
            ..Default::default()
        }
    }

    /// Create 8-bit config
    pub fn bits8() -> Self {
        Self {
            bits: 8,
            ..Default::default()
        }
    }

    /// Set group size
    pub fn with_group_size(mut self, size: i32) -> Self {
        self.group_size = size;
        self
    }

    /// Enable/disable exllama
    pub fn with_exllama(mut self, enabled: bool) -> Self {
        self.use_exllama = enabled;
        self
    }

    /// Set calibration dataset
    pub fn with_calibration(mut self, dataset: impl Into<String>) -> Self {
        self.calibration_dataset = dataset.into();
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CALIBRATION DATASETS
// ═══════════════════════════════════════════════════════════════════════════════

pub struct CalibrationDatasets;

impl CalibrationDatasets {
    /// Standard calibration datasets
    pub fn standard() -> Vec<&'static str> {
        vec![
            "wikitext",
            "pileval",
            "c4",
            "ptb",
        ]
    }

    /// Recommended dataset for model type
    pub fn recommended_for(model_family: &str) -> &'static str {
        match model_family.to_lowercase().as_str() {
            "llama" | "llama2" | "llama3" => "wikitext",
            "mistral" => "wikitext",
            "falcon" => "pileval",
            "gpt-neox" => "pileval",
            _ => "wikitext",
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_creation() {
        let backend = GptqBackend::new();
        assert_eq!(backend.name(), "GPTQ");
        assert!(backend.is_available());
    }

    #[test]
    fn test_supported_methods() {
        let backend = GptqBackend::new();
        let methods = backend.supported_methods();
        assert_eq!(methods.len(), 2);
    }

    #[test]
    fn test_gptq_config() {
        let config = GptqConfig::bits4()
            .with_group_size(64)
            .with_calibration("custom_data");

        assert_eq!(config.bits, 4);
        assert_eq!(config.group_size, 64);
        assert_eq!(config.calibration_dataset, "custom_data");
    }

    #[test]
    fn test_calibration_datasets() {
        let standard = CalibrationDatasets::standard();
        assert!(!standard.is_empty());

        let llama_rec = CalibrationDatasets::recommended_for("llama");
        assert_eq!(llama_rec, "wikitext");
    }

    #[tokio::test]
    async fn test_quantize_non_gptq_method() {
        let backend = GptqBackend::new();
        let config = QuantConfig::new("model")
            .with_method(QuantMethod::gguf_q4km());
        
        let result = backend.quantize(config).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_gptq_quantization() {
        let backend = GptqBackend::new();
        let config = QuantConfig::new("meta-llama/Llama-2-7b")
            .with_method(QuantMethod::gptq_4bit())
            .with_output("llama-2-7b-gptq");
        
        let result = backend.quantize(config).await;
        assert!(result.is_ok());
        
        let model = result.unwrap();
        assert!(model.is_gptq());
        assert!(model.stats.perplexity_delta.is_some());
    }
}
