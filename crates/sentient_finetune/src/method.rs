//! ─── Fine-tuning Methods ───

use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════════
//  FINETUNE METHOD
// ═══════════════════════════════════════════════════════════════════════════════

/// Fine-tuning method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FineTuneMethod {
    #[serde(rename = "lora")]
    Lora,
    
    #[serde(rename = "qlora")]
    Qlora,
    
    #[serde(rename = "full")]
    Full,
    
    #[serde(rename = "prefix_tuning")]
    PrefixTuning,
    
    #[serde(rename = "prompt_tuning")]
    PromptTuning,
}

impl Default for FineTuneMethod {
    fn default() -> Self {
        Self::Lora
    }
}

impl FineTuneMethod {
    /// Get method display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Lora => "LoRA",
            Self::Qlora => "QLoRA",
            Self::Full => "Full Fine-tuning",
            Self::PrefixTuning => "Prefix Tuning",
            Self::PromptTuning => "Prompt Tuning",
        }
    }

    /// Get memory efficiency (higher is better, 0-100)
    pub fn memory_efficiency(&self) -> u8 {
        match self {
            Self::Full => 10,
            Self::Lora => 70,
            Self::Qlora => 95,
            Self::PrefixTuning => 85,
            Self::PromptTuning => 90,
        }
    }

    /// Check if method requires base model to be frozen
    pub fn freezes_base(&self) -> bool {
        !matches!(self, Self::Full)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  LORA CONFIG
// ═══════════════════════════════════════════════════════════════════════════════

/// LoRA (Low-Rank Adaptation) configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoraConfig {
    /// Rank (r)
    #[serde(default = "default_r")]
    pub r: u8,
    
    /// Alpha scaling factor
    #[serde(default = "default_alpha")]
    pub alpha: u16,
    
    /// Dropout probability
    #[serde(default = "default_dropout")]
    pub dropout: f32,
    
    /// Target modules
    #[serde(default = "default_target_modules")]
    pub target_modules: Vec<String>,
    
    /// Bias type
    #[serde(default)]
    pub bias: LoraBias,
    
    /// Fan-in-fan-out
    #[serde(default)]
    pub fan_in_fan_out: bool,
    
    /// Modules to save (not merged)
    #[serde(default)]
    pub modules_to_save: Vec<String>,
    
    /// Initialize LoRA weights
    #[serde(default)]
    pub init_weights: LoraInitMethod,
}

fn default_r() -> u8 { 8 }
fn default_alpha() -> u16 { 16 }
fn default_dropout() -> f32 { 0.05 }
fn default_target_modules() -> Vec<String> {
    vec!["q_proj".into(), "v_proj".into()]
}

impl Default for LoraConfig {
    fn default() -> Self {
        Self {
            r: default_r(),
            alpha: default_alpha(),
            dropout: default_dropout(),
            target_modules: default_target_modules(),
            bias: LoraBias::default(),
            fan_in_fan_out: false,
            modules_to_save: vec![],
            init_weights: LoraInitMethod::default(),
        }
    }
}

impl LoraConfig {
    /// Create new LoRA config
    pub fn new(r: u8) -> Self {
        Self {
            r,
            alpha: (r as u16) * 2,
            ..Default::default()
        }
    }

    /// Set rank
    pub fn rank(mut self, r: u8) -> Self {
        self.r = r;
        self
    }

    /// Set alpha
    pub fn alpha(mut self, alpha: u16) -> Self {
        self.alpha = alpha;
        self
    }

    /// Set target modules
    pub fn target_modules(mut self, modules: Vec<&str>) -> Self {
        self.target_modules = modules.into_iter().map(|s| s.into()).collect();
        self
    }

    /// Set dropout
    pub fn dropout(mut self, dropout: f32) -> Self {
        self.dropout = dropout;
        self
    }

    /// Estimate number of trainable parameters
    pub fn estimate_params(&self, hidden_size: usize, num_layers: usize) -> usize {
        // Each LoRA module has 2 * r * hidden_size parameters (A and B matrices)
        let params_per_module = 2 * self.r as usize * hidden_size;
        let num_modules = self.target_modules.len();
        params_per_module * num_modules * num_layers
    }
}

/// LoRA bias type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoraBias {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "lora_only")]
    LoraOnly,
}

impl Default for LoraBias {
    fn default() -> Self {
        Self::None
    }
}

/// LoRA initialization method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoraInitMethod {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "gaussian")]
    Gaussian,
    #[serde(rename = "kaiming")]
    Kaiming,
    #[serde(rename = "orthogonal")]
    Orthogonal,
}

impl Default for LoraInitMethod {
    fn default() -> Self {
        Self::Default
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  QLORA CONFIG
// ═══════════════════════════════════════════════════════════════════════════════

/// QLoRA (Quantized LoRA) configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QloraConfig {
    /// Base LoRA config
    #[serde(flatten)]
    pub lora: LoraConfig,
    
    /// Quantization bits (4 or 8)
    #[serde(default = "default_bits")]
    pub bits: u8,
    
    /// Quantization type
    #[serde(default)]
    pub quant_type: QuantType,
    
    /// Use double quantization
    #[serde(default = "default_double_quant")]
    pub double_quant: bool,
    
    /// Compute dtype
    #[serde(default)]
    pub compute_dtype: ComputeDtype,
    
    /// Use paged optimizers
    #[serde(default = "default_paged")]
    pub use_paged_optimizer: bool,
    
    /// LoRA rank (for convenience)
    pub r: u8,
    
    /// LoRA alpha
    pub alpha: u16,
}

fn default_bits() -> u8 { 4 }
fn default_double_quant() -> bool { true }
fn default_paged() -> bool { true }

impl Default for QloraConfig {
    fn default() -> Self {
        Self {
            lora: LoraConfig::default(),
            bits: default_bits(),
            quant_type: QuantType::default(),
            double_quant: default_double_quant(),
            compute_dtype: ComputeDtype::default(),
            use_paged_optimizer: default_paged(),
            r: default_r(),
            alpha: default_alpha(),
        }
    }
}

impl QloraConfig {
    /// Create new QLoRA config
    pub fn new(r: u8) -> Self {
        Self {
            lora: LoraConfig::new(r),
            r,
            alpha: (r as u16) * 2,
            ..Default::default()
        }
    }

    /// Set quantization bits
    pub fn bits(mut self, bits: u8) -> Self {
        self.bits = bits;
        self
    }

    /// Estimate memory requirement (GB)
    pub fn estimate_memory_gb(&self, model_params_b: f32) -> f32 {
        // 4-bit quantization: ~0.5 bytes per parameter
        // Plus LoRA adapters: small overhead
        let base_memory = model_params_b * 0.5;
        let lora_overhead = 0.1; // Approximate
        base_memory + lora_overhead
    }
}

/// Quantization type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuantType {
    #[serde(rename = "nf4")]
    NF4,
    #[serde(rename = "fp4")]
    FP4,
    #[serde(rename = "int4")]
    Int4,
    #[serde(rename = "int8")]
    Int8,
}

impl Default for QuantType {
    fn default() -> Self {
        Self::NF4
    }
}

/// Compute dtype
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComputeDtype {
    #[serde(rename = "float16")]
    Float16,
    #[serde(rename = "bfloat16")]
    BFloat16,
    #[serde(rename = "float32")]
    Float32,
}

impl Default for ComputeDtype {
    fn default() -> Self {
        Self::BFloat16
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  FULL FINE-TUNING CONFIG
// ═══════════════════════════════════════════════════════════════════════════════

/// Full fine-tuning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullConfig {
    /// Freeze embedding layer
    #[serde(default)]
    pub freeze_embeddings: bool,
    
    /// Freeze layer norm
    #[serde(default)]
    pub freeze_layer_norm: bool,
    
    /// Freeze specific layers (indices)
    #[serde(default)]
    pub freeze_layers: Vec<usize>,
    
    /// Use gradient checkpointing
    #[serde(default = "default_gradient_checkpointing")]
    pub gradient_checkpointing: bool,
    
    /// Use DeepSpeed
    #[serde(default)]
    pub use_deepspeed: bool,
    
    /// DeepSpeed config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deepspeed_config: Option<DeepSpeedConfig>,
}

fn default_gradient_checkpointing() -> bool { true }

impl Default for FullConfig {
    fn default() -> Self {
        Self {
            freeze_embeddings: false,
            freeze_layer_norm: false,
            freeze_layers: vec![],
            gradient_checkpointing: default_gradient_checkpointing(),
            use_deepspeed: false,
            deepspeed_config: None,
        }
    }
}

/// DeepSpeed configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepSpeedConfig {
    /// Zero stage (0, 1, 2, or 3)
    pub zero_stage: u8,
    
    /// Offload optimizer to CPU
    #[serde(default)]
    pub offload_optimizer: bool,
    
    /// Offload parameters to CPU
    #[serde(default)]
    pub offload_param: bool,
    
    /// Gradient accumulation steps
    #[serde(default)]
    pub gradient_accumulation_steps: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finetune_method_default() {
        assert_eq!(FineTuneMethod::default(), FineTuneMethod::Lora);
    }

    #[test]
    fn test_lora_config_default() {
        let config = LoraConfig::default();
        assert_eq!(config.r, 8);
        assert_eq!(config.alpha, 16);
        assert_eq!(config.dropout, 0.05);
    }

    #[test]
    fn test_lora_config_builder() {
        let config = LoraConfig::new(16)
            .alpha(32)
            .dropout(0.1)
            .target_modules(vec!["q_proj", "v_proj", "k_proj", "o_proj"]);

        assert_eq!(config.r, 16);
        assert_eq!(config.alpha, 32);
        assert_eq!(config.dropout, 0.1);
        assert_eq!(config.target_modules.len(), 4);
    }

    #[test]
    fn test_qlora_config() {
        let config = QloraConfig::new(8);
        assert_eq!(config.bits, 4);
        assert!(config.double_quant);
        assert!(config.use_paged_optimizer);
    }

    #[test]
    fn test_memory_efficiency() {
        assert!(FineTuneMethod::Qlora.memory_efficiency() > FineTuneMethod::Lora.memory_efficiency());
        assert!(FineTuneMethod::Lora.memory_efficiency() > FineTuneMethod::Full.memory_efficiency());
    }

    #[test]
    fn test_estimate_memory() {
        let config = QloraConfig::new(8);
        let memory = config.estimate_memory_gb(7.0); // 7B model
        assert!(memory > 0.0 && memory < 10.0);
    }

    #[test]
    fn test_estimate_params() {
        let config = LoraConfig::default();
        // 2 modules * 2 matrices * rank 8 * hidden 4096 * 32 layers
        let params = config.estimate_params(4096, 32);
        assert!(params > 0);
    }
}
