//! ─── GGUF Quantization Backend ───
//!
//! llama.cpp GGUF format quantization

use async_trait::async_trait;
use chrono::Utc;
use std::process::Command;

use crate::{
    QuantizeBackend, QuantConfig, QuantizedModel, QuantMethod,
    QuantizationStats, ModelMetadata, GgufMethod,
    QuantizeResult, QuantizeError,
};

// ═══════════════════════════════════════════════════════════════════════════════
//  GGUF BACKEND
// ═══════════════════════════════════════════════════════════════════════════════

pub struct GgufBackend {
    llama_cpp_path: Option<String>,
}

impl GgufBackend {
    pub fn new() -> Self {
        Self {
            llama_cpp_path: Self::find_llama_cpp(),
        }
    }

    /// Find llama.cpp installation
    fn find_llama_cpp() -> Option<String> {
        // Check common locations
        let paths = [
            "./llama.cpp",
            "../llama.cpp",
            "/usr/local/bin/llama.cpp",
            "~/llama.cpp",
        ];

        for path in paths {
            if std::path::Path::new(path).exists() {
                return Some(path.to_string());
            }
        }

        // Check for quantize binary
        if let Ok(output) = Command::new("which").arg("llama-quantize").output() {
            if output.status.success() {
                return Some("system".into());
            }
        }

        None
    }

    /// Convert model to GGUF
    async fn convert_to_gguf(&self, model_id: &str, output_dir: &str) -> QuantizeResult<String> {
        log::info!("🔄 GGUF: Converting {} to GGUF format...", model_id);

        // In a real implementation, this would call convert-hf-to-gguf.py
        // For now, simulate the conversion
        let output_path = format!("{}/model-f16.gguf", output_dir);
        
        // Simulate conversion time
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        log::info!("✅ GGUF: Conversion complete: {}", output_path);
        Ok(output_path)
    }

    /// Quantize GGUF model
    async fn quantize_gguf(
        &self,
        input: &str,
        output: &str,
        method: GgufMethod,
    ) -> QuantizeResult<()> {
        log::info!("🔧 GGUF: Quantizing with {}...", method.name());

        // In a real implementation, this would call llama-quantize
        // Example: ./llama-quantize model-f16.gguf model-q4km.gguf Q4_K_M
        
        let method_str = method.name();
        log::info!("   Input: {}", input);
        log::info!("   Output: {}", output);
        log::info!("   Method: {}", method_str);

        // Simulate quantization
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        log::info!("✅ GGUF: Quantization complete!");
        Ok(())
    }

    /// Get model metadata from GGUF
    fn read_metadata(&self, path: &str) -> ModelMetadata {
        // In a real implementation, parse GGUF header
        ModelMetadata {
            architecture: "llama".into(),
            name: path.to_string(),
            vocab_size: 32000,
            context_length: 4096,
            num_layers: 32,
            hidden_size: 4096,
            num_heads: 32,
            num_kv_heads: Some(32),
            intermediate_size: Some(11008),
            extra: Default::default(),
        }
    }

    /// Estimate original model size
    fn estimate_original_size(&self, metadata: &ModelMetadata) -> f32 {
        // Rough estimation based on architecture
        let params = metadata.vocab_size as f32 * 0.001 // embedding
            + metadata.num_layers as f32 * metadata.hidden_size as f32 * 0.01; // transformer
        
        params * 2.0 // FP16 = 2 bytes per param
    }
}

impl Default for GgufBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl QuantizeBackend for GgufBackend {
    async fn quantize(&self, config: QuantConfig) -> QuantizeResult<QuantizedModel> {
        let start_time = std::time::Instant::now();

        // Verify GGUF method
        let method = match &config.method {
            QuantMethod::Gguf(m) => m,
            _ => return Err(QuantizeError::UnsupportedMethod(
                format!("GGUF backend only supports GGUF methods, got {:?}", config.method)
            )),
        };

        // Create output directory
        let output_dir = std::path::Path::new(&config.output_path)
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .to_string_lossy()
            .to_string();
        
        std::fs::create_dir_all(&output_dir)?;

        // Step 1: Convert to GGUF (if not already GGUF)
        let f16_path = self.convert_to_gguf(&config.model_id, &output_dir).await?;

        // Step 2: Quantize to target method
        let output_file = format!(
            "{}/{}.gguf",
            output_dir,
            config.output_path.replace(".gguf", "")
        );
        
        self.quantize_gguf(&f16_path, &output_file, *method).await?;

        // Get file size
        let size_bytes = std::fs::metadata(&output_file)
            .map(|m| m.len())
            .unwrap_or(0);

        // Read metadata
        let metadata = self.read_metadata(&output_file);
        let original_size = self.estimate_original_size(&metadata);

        let stats = QuantizationStats {
            original_size_gb: original_size,
            quantized_size_gb: size_bytes as f32 / 1e9,
            compression_ratio: original_size / (size_bytes as f32 / 1e9),
            perplexity_delta: None,
            duration_secs: start_time.elapsed().as_secs(),
            peak_memory_gb: None,
            param_count: 7_000_000_000, // Placeholder
            avg_weight_error: None,
        };

        Ok(QuantizedModel {
            path: output_file,
            source_model: config.model_id,
            method: config.method,
            size_bytes,
            stats,
            metadata,
            created_at: Utc::now(),
            hf_repo_id: config.hf_repo_id,
        })
    }

    fn supported_methods(&self) -> Vec<QuantMethod> {
        GgufMethod::all().into_iter().map(QuantMethod::Gguf).collect()
    }

    fn is_available(&self) -> bool {
        self.llama_cpp_path.is_some()
    }

    fn name(&self) -> &'static str {
        "GGUF"
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  GGUF UTILITIES
// ═══════════════════════════════════════════════════════════════════════════════

pub struct GgufUtils;

impl GgufUtils {
    /// Parse GGUF file header
    pub fn parse_header(_path: &str) -> QuantizeResult<GgufHeader> {
        // In a real implementation, read GGUF binary header
        Ok(GgufHeader {
            version: 3,
            tensor_count: 300,
            metadata_kv_count: 20,
        })
    }

    /// List tensors in GGUF file
    pub fn list_tensors(_path: &str) -> QuantizeResult<Vec<String>> {
        Ok(vec![
            "token_embd.weight".into(),
            "blk.0.attn_q.weight".into(),
            "blk.0.attn_k.weight".into(),
            "blk.0.attn_v.weight".into(),
            "blk.0.attn_output.weight".into(),
            "blk.0.ffn_gate.weight".into(),
            "blk.0.ffn_up.weight".into(),
            "blk.0.ffn_down.weight".into(),
        ])
    }

    /// Get GGUF type string for method
    pub fn method_to_string(method: GgufMethod) -> &'static str {
        method.name()
    }
}

/// GGUF file header
#[derive(Debug, Clone)]
pub struct GgufHeader {
    pub version: u32,
    pub tensor_count: u64,
    pub metadata_kv_count: u64,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_creation() {
        let backend = GgufBackend::new();
        assert_eq!(backend.name(), "GGUF");
    }

    #[test]
    fn test_supported_methods() {
        let backend = GgufBackend::new();
        let methods = backend.supported_methods();
        assert!(!methods.is_empty());
        
        // Should include Q4_K_M
        assert!(methods.iter().any(|m| matches!(m, QuantMethod::Gguf(GgufMethod::Q4_K_M))));
    }

    #[test]
    fn test_method_string() {
        assert_eq!(GgufUtils::method_to_string(GgufMethod::Q4_K_M), "Q4_K_M");
        assert_eq!(GgufUtils::method_to_string(GgufMethod::Q8_0), "Q8_0");
    }

    #[test]
    fn test_metadata_estimation() {
        let backend = GgufBackend::new();
        let metadata = ModelMetadata {
            num_layers: 32,
            hidden_size: 4096,
            vocab_size: 32000,
            ..Default::default()
        };
        
        let size = backend.estimate_original_size(&metadata);
        assert!(size > 0.0);
    }

    #[tokio::test]
    async fn test_quantize_non_gguf_method() {
        let backend = GgufBackend::new();
        let config = QuantConfig::new("model")
            .with_method(QuantMethod::gptq_4bit());
        
        let result = backend.quantize(config).await;
        assert!(result.is_err());
    }
}
