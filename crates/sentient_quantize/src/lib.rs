//! ─── SENTIENT OS Model Quantization ───
//!
//! Model quantization for efficient inference:
//! - GGUF (llama.cpp compatible)
//! - GPTQ (AutoGPTQ compatible)
//! - AWQ (Activation-aware Weight Quantization)
//! - BitsAndBytes (4-bit/8-bit)
//!
//! # Example
//! ```ignore

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
//! use sentient_quantize::{Quantizer, QuantConfig, QuantMethod};
//!
//! let quantizer = Quantizer::gguf();
//! let config = QuantConfig::new("meta-llama/Llama-2-7b")
//!     .with_method(QuantMethod::Q4_K_M)
//!     .with_output("llama-2-7b-q4km.gguf");
//! 
//! let result = quantizer.quantize(config).await?;
//! println!("Quantized model saved to: {}", result.output_path);
//! ```

pub mod error;
pub mod types;
pub mod method;
pub mod gguf;
pub mod gptq;
pub mod awq;
pub mod bnb;
pub mod calibration;

pub use error::{QuantizeError, QuantizeResult};
pub use types::{
    QuantConfig, QuantizedModel, QuantizationStats,
    ModelMetadata, Precision,
};
pub use method::{QuantMethod, GgufMethod, GptqMethod, AwqMethod};

use async_trait::async_trait;

// ═══════════════════════════════════════════════════════════════════════════════
//  QUANTIZER TRAIT
// ═══════════════════════════════════════════════════════════════════════════════

/// Trait for quantization backends
#[async_trait]
pub trait QuantizeBackend: Send + Sync {
    /// Quantize model
    async fn quantize(&self, config: QuantConfig) -> QuantizeResult<QuantizedModel>;
    
    /// Get supported quantization methods
    fn supported_methods(&self) -> Vec<QuantMethod>;
    
    /// Check if backend is available
    fn is_available(&self) -> bool;
    
    /// Get backend name
    fn name(&self) -> &'static str;
}

// ═══════════════════════════════════════════════════════════════════════════════
//  QUANTIZER FACADE
// ═══════════════════════════════════════════════════════════════════════════════

/// High-level quantizer interface
pub struct Quantizer {
    backend: Box<dyn QuantizeBackend>,
}

impl Quantizer {
    /// Create GGUF quantizer
    pub fn gguf() -> Self {
        Self {
            backend: Box::new(gguf::GgufBackend::new()),
        }
    }

    /// Create GPTQ quantizer
    pub fn gptq() -> Self {
        Self {
            backend: Box::new(gptq::GptqBackend::new()),
        }
    }

    /// Create AWQ quantizer
    pub fn awq() -> Self {
        Self {
            backend: Box::new(awq::AwqBackend::new()),
        }
    }

    /// Create BitsAndBytes quantizer
    pub fn bnb() -> Self {
        Self {
            backend: Box::new(bnb::BnbBackend::new()),
        }
    }

    /// Quantize model
    pub async fn quantize(&self, config: QuantConfig) -> QuantizeResult<QuantizedModel> {
        log::info!("🔧 QUANTIZE: Starting quantization with {} backend", self.backend.name());
        self.backend.quantize(config).await
    }

    /// Get supported methods
    pub fn supported_methods(&self) -> Vec<QuantMethod> {
        self.backend.supported_methods()
    }

    /// Check availability
    pub fn is_available(&self) -> bool {
        self.backend.is_available()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  MODEL SIZE ESTIMATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Estimate model size after quantization
pub fn estimate_size(params_b: f32, method: &QuantMethod) -> f32 {
    let bits_per_param = match method {
        QuantMethod::Gguf(GgufMethod::Q4_0) => 4.5,
        QuantMethod::Gguf(GgufMethod::Q4_K_M) => 4.8,
        QuantMethod::Gguf(GgufMethod::Q4_K_S) => 4.6,
        QuantMethod::Gguf(GgufMethod::Q5_0) => 5.5,
        QuantMethod::Gguf(GgufMethod::Q5_K_M) => 5.7,
        QuantMethod::Gguf(GgufMethod::Q5_K_S) => 5.5,
        QuantMethod::Gguf(GgufMethod::Q6_K) => 6.6,
        QuantMethod::Gguf(GgufMethod::Q8_0) => 8.5,
        QuantMethod::Gguf(GgufMethod::F16) => 16.0,
        QuantMethod::Gguf(GgufMethod::F32) => 32.0,
        QuantMethod::Gptq(GptqMethod::Gptq4) => 4.5,
        QuantMethod::Gptq(GptqMethod::Gptq8) => 8.5,
        QuantMethod::Awq(AwqMethod::Awq4) => 4.5,
        QuantMethod::Bnb4 => 4.5,
        QuantMethod::Bnb8 => 8.5,
        _ => 16.0,
    };

    params_b * bits_per_param / 8.0 // GB
}

/// Estimate memory requirement for inference
pub fn estimate_memory(params_b: f32, method: &QuantMethod, context_len: usize) -> f32 {
    let model_size = estimate_size(params_b, method);
    let kv_cache = (context_len as f32 * 2.0 * 4096.0 * 2.0) / 1e9; // Rough estimate
    model_size + kv_cache + 1.0 // +1GB for activations
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CONVENIENCE FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// Quick quantize to GGUF Q4_K_M
pub async fn quantize_to_q4km(model_id: &str, output: &str) -> QuantizeResult<QuantizedModel> {
    let config = QuantConfig::new(model_id)
        .with_method(QuantMethod::gguf_q4km())
        .with_output(output);
    
    Quantizer::gguf().quantize(config).await
}

/// Quick quantize to GPTQ 4-bit
pub async fn quantize_to_gptq4(model_id: &str, output: &str) -> QuantizeResult<QuantizedModel> {
    let config = QuantConfig::new(model_id)
        .with_method(QuantMethod::gptq_4bit())
        .with_output(output);
    
    Quantizer::gptq().quantize(config).await
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantizer_constructors() {
        let _gguf = Quantizer::gguf();
        let _gptq = Quantizer::gptq();
        let _awq = Quantizer::awq();
        let _bnb = Quantizer::bnb();
    }

    #[test]
    fn test_size_estimation() {
        // 7B model at Q4_K_M
        let size = estimate_size(7.0, &QuantMethod::gguf_q4km());
        assert!(size > 3.0 && size < 5.0, "Q4_K_M size should be ~4GB, got {}", size);

        // 7B model at Q8_0
        let size = estimate_size(7.0, &QuantMethod::Gguf(GgufMethod::Q8_0));
        assert!(size > 5.0 && size < 8.0, "Q8_0 size should be ~7GB, got {}", size);

        // 70B model at Q4_K_M
        let size = estimate_size(70.0, &QuantMethod::gguf_q4km());
        assert!(size > 35.0 && size < 50.0, "70B Q4_K_M size should be ~40GB, got {}", size);
    }

    #[test]
    fn test_memory_estimation() {
        // 7B Q4_K_M with 4K context
        let mem = estimate_memory(7.0, &QuantMethod::gguf_q4km(), 4096);
        assert!(mem > 4.0, "Memory should be >4GB, got {}", mem);

        // 70B Q4_K_M with 8K context
        let mem = estimate_memory(70.0, &QuantMethod::gguf_q4km(), 8192);
        assert!(mem > 40.0, "Memory should be >40GB, got {}", mem);
    }

    #[test]
    fn test_supported_methods() {
        let gguf = Quantizer::gguf();
        let methods = gguf.supported_methods();
        assert!(!methods.is_empty());
    }

    #[test]
    fn test_precision_bits() {
        assert_eq!(QuantMethod::gguf_q4km().bits(), 4);
        assert_eq!(QuantMethod::Gguf(GgufMethod::Q8_0).bits(), 8);
        assert_eq!(QuantMethod::Gguf(GgufMethod::F16).bits(), 16);
    }
}
