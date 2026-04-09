//! Yapılandırma tipleri

use serde::{Deserialize, Serialize};
use crate::types::Strategy;

/// Ana yapılandırma
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CevahirConfig {
    // === Device ===
    /// Cihaz (cpu/cuda)
    pub device: String,
    /// Random seed
    pub seed: Option<u64>,
    /// Log seviyesi
    pub log_level: String,
    
    // === Model ===
    /// Vocabulary boyutu
    pub vocab_size: usize,
    /// Embedding boyutu
    pub embed_dim: usize,
    /// Sequence projection boyutu
    pub seq_proj_dim: usize,
    /// Attention head sayısı
    pub num_heads: usize,
    /// Katman sayısı
    pub num_layers: usize,
    /// FFN boyutu (None = 4x embed_dim)
    pub ffn_dim: Option<usize>,
    /// Dropout
    pub dropout: f32,
    /// Learning rate
    pub learning_rate: f64,
    
    // === V-4 Features ===
    /// RoPE kullan
    pub use_rope: bool,
    /// RMSNorm kullan
    pub use_rmsnorm: bool,
    /// SwiGLU kullan
    pub use_swiglu: bool,
    /// KV Cache kullan
    pub use_kv_cache: bool,
    /// MoE kullan
    pub use_moe: bool,
    /// Expert sayısı (MoE için)
    pub num_experts: usize,
    /// Top-k (MoE için)
    pub moe_top_k: usize,
    
    // === V-5 Features ===
    /// KV head sayısı (GQA için)
    pub num_kv_heads: Option<usize>,
    /// Sliding window
    pub sliding_window: Option<usize>,
    /// RoPE scaling tipi
    pub rope_scaling_type: String,
    /// RoPE scaling faktörü
    pub rope_scaling_factor: f32,
    
    // === V-6 Features ===
    /// PyTorch SDPA kullan
    pub use_pytorch_sdpa: bool,
    /// QK-Norm kullan
    pub use_qk_norm: bool,
    /// Parallel residual
    pub parallel_residual: bool,
    /// Logit soft cap
    pub logit_soft_cap: f32,
    
    // === V-7 Features ===
    /// Stochastic depth
    pub drop_path_rate: f32,
    
    // === Cognitive ===
    /// Varsayılan strateji
    pub default_strategy: Strategy,
    /// Maksimum düşünme adımı
    pub max_thinking_steps: usize,
    /// Bellek aktif
    pub enable_memory: bool,
    /// Tool kullanımı aktif
    pub enable_tools: bool,
    
    // === Paths ===
    /// Vocabulary dosyası yolu
    pub vocab_path: String,
    /// Merges dosyası yolu
    pub merges_path: String,
    /// Model ağırlıkları yolu
    pub model_path: Option<String>,
    
    // === Training ===
    /// Eğitim modu
    pub training_mode: bool,
    /// Batch size
    pub batch_size: usize,
    /// Gradient checkpointing
    pub use_gradient_checkpointing: bool,
    /// Weight tying
    pub tie_weights: bool,
    
    // === Quantization ===
    /// Quantization tipi
    pub quantization_type: String,
    
    // === TensorBoard ===
    /// TensorBoard aktif
    pub use_tensorboard: bool,
    /// TensorBoard log dizini
    pub tb_log_dir: String,
}

impl Default for CevahirConfig {
    fn default() -> Self {
        Self {
            // Device
            device: "cpu".to_string(),
            seed: Some(42),
            log_level: "INFO".to_string(),
            
            // Model
            vocab_size: 60000,
            embed_dim: 512,
            seq_proj_dim: 512,
            num_heads: 8,
            num_layers: 8,
            ffn_dim: None,  // Auto: 4x embed_dim
            dropout: 0.15,
            learning_rate: 1e-4,
            
            // V-4 Features
            use_rope: true,
            use_rmsnorm: true,
            use_swiglu: true,
            use_kv_cache: true,
            use_moe: false,
            num_experts: 8,
            moe_top_k: 2,
            
            // V-5 Features
            num_kv_heads: None,
            sliding_window: None,
            rope_scaling_type: "none".to_string(),
            rope_scaling_factor: 1.0,
            
            // V-6 Features
            use_pytorch_sdpa: true,
            use_qk_norm: false,
            parallel_residual: false,
            logit_soft_cap: 30.0,
            
            // V-7 Features
            drop_path_rate: 0.0,
            
            // Cognitive
            default_strategy: Strategy::Think,
            max_thinking_steps: 5,
            enable_memory: true,
            enable_tools: true,
            
            // Paths
            vocab_path: "data/vocab_lib/vocab.json".to_string(),
            merges_path: "data/merges_lib/merges.txt".to_string(),
            model_path: None,
            
            // Training
            training_mode: false,
            batch_size: 32,
            use_gradient_checkpointing: true,
            tie_weights: true,
            
            // Quantization
            quantization_type: "none".to_string(),
            
            // TensorBoard
            use_tensorboard: false,
            tb_log_dir: "runs/cevahir".to_string(),
        }
    }
}

impl CevahirConfig {
    /// GPU için optimize edilmiş yapılandırma
    pub fn gpu() -> Self {
        Self {
            device: "cuda".to_string(),
            num_layers: 12,
            embed_dim: 768,
            num_heads: 12,
            ..Default::default()
        }
    }
    
    /// Küçük model (hızlı inference)
    pub fn small() -> Self {
        Self {
            num_layers: 4,
            embed_dim: 256,
            num_heads: 4,
            vocab_size: 30000,
            ..Default::default()
        }
    }
    
    /// Büyük model (yüksek kalite)
    pub fn large() -> Self {
        Self {
            num_layers: 24,
            embed_dim: 1024,
            num_heads: 16,
            ffn_dim: Some(4096),
            ..Default::default()
        }
    }
    
    /// Python dict'ine dönüştür
    pub fn to_python_dict(&self) -> std::collections::HashMap<String, serde_json::Value> {
        let mut map = std::collections::HashMap::new();
        
        map.insert("device".to_string(), serde_json::json!(self.device));
        map.insert("vocab_size".to_string(), serde_json::json!(self.vocab_size));
        map.insert("embed_dim".to_string(), serde_json::json!(self.embed_dim));
        map.insert("seq_proj_dim".to_string(), serde_json::json!(self.seq_proj_dim));
        map.insert("num_heads".to_string(), serde_json::json!(self.num_heads));
        map.insert("num_layers".to_string(), serde_json::json!(self.num_layers));
        map.insert("dropout".to_string(), serde_json::json!(self.dropout));
        map.insert("learning_rate".to_string(), serde_json::json!(self.learning_rate));
        map.insert("use_rope".to_string(), serde_json::json!(self.use_rope));
        map.insert("use_rmsnorm".to_string(), serde_json::json!(self.use_rmsnorm));
        map.insert("use_swiglu".to_string(), serde_json::json!(self.use_swiglu));
        map.insert("use_kv_cache".to_string(), serde_json::json!(self.use_kv_cache));
        map.insert("use_moe".to_string(), serde_json::json!(self.use_moe));
        map.insert("pe_mode".to_string(), serde_json::json!(if self.use_rope { "rope" } else { "sinusoidal" }));
        map.insert("use_gradient_checkpointing".to_string(), serde_json::json!(self.use_gradient_checkpointing));
        map.insert("tie_weights".to_string(), serde_json::json!(self.tie_weights));
        map.insert("quantization_type".to_string(), serde_json::json!(self.quantization_type));
        
        if let Some(seed) = self.seed {
            map.insert("seed".to_string(), serde_json::json!(seed));
        }
        if let Some(ffn_dim) = self.ffn_dim {
            map.insert("ffn_dim".to_string(), serde_json::json!(ffn_dim));
        }
        if let Some(num_kv_heads) = self.num_kv_heads {
            map.insert("num_kv_heads".to_string(), serde_json::json!(num_kv_heads));
        }
        if let Some(sliding_window) = self.sliding_window {
            map.insert("sliding_window".to_string(), serde_json::json!(sliding_window));
        }
        
        map
    }
}
