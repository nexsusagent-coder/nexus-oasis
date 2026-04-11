//! ─── Quantization Methods ───

use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════════
//  QUANT METHOD
// ═══════════════════════════════════════════════════════════════════════════════

/// Quantization method
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuantMethod {
    /// GGUF quantization (llama.cpp)
    Gguf(GgufMethod),
    
    /// GPTQ quantization
    Gptq(GptqMethod),
    
    /// AWQ quantization
    Awq(AwqMethod),
    
    /// BitsAndBytes 4-bit
    Bnb4,
    
    /// BitsAndBytes 8-bit
    Bnb8,
}

impl QuantMethod {
    // ═════════════════════════════════════════════════════════════════════════
    //  GGUF SHORTCUTS
    // ═════════════════════════════════════════════════════════════════════════

    pub fn gguf_q4_0() -> Self {
        Self::Gguf(GgufMethod::Q4_0)
    }

    pub fn gguf_q4km() -> Self {
        Self::Gguf(GgufMethod::Q4_K_M)
    }

    pub fn gguf_q4ks() -> Self {
        Self::Gguf(GgufMethod::Q4_K_S)
    }

    pub fn gguf_q5km() -> Self {
        Self::Gguf(GgufMethod::Q5_K_M)
    }

    pub fn gguf_q8_0() -> Self {
        Self::Gguf(GgufMethod::Q8_0)
    }

    pub fn gguf_f16() -> Self {
        Self::Gguf(GgufMethod::F16)
    }

    // ═════════════════════════════════════════════════════════════════════════
    //  GPTQ SHORTCUTS
    // ═════════════════════════════════════════════════════════════════════════

    pub fn gptq_4bit() -> Self {
        Self::Gptq(GptqMethod::Gptq4)
    }

    pub fn gptq_8bit() -> Self {
        Self::Gptq(GptqMethod::Gptq8)
    }

    // ═════════════════════════════════════════════════════════════════════════
    //  AWQ SHORTCUTS
    // ═════════════════════════════════════════════════════════════════════════

    pub fn awq_4bit() -> Self {
        Self::Awq(AwqMethod::Awq4)
    }

    // ═════════════════════════════════════════════════════════════════════════
    //  PROPERTIES
    // ═════════════════════════════════════════════════════════════════════════

    /// Get effective bits
    pub fn bits(&self) -> u8 {
        match self {
            QuantMethod::Gguf(m) => m.bits(),
            QuantMethod::Gptq(m) => m.bits(),
            QuantMethod::Awq(m) => m.bits(),
            QuantMethod::Bnb4 => 4,
            QuantMethod::Bnb8 => 8,
        }
    }

    /// Get display name
    pub fn name(&self) -> String {
        match self {
            QuantMethod::Gguf(m) => format!("GGUF-{}", m.name()),
            QuantMethod::Gptq(m) => format!("GPTQ-{}", m.name()),
            QuantMethod::Awq(m) => format!("AWQ-{}", m.name()),
            QuantMethod::Bnb4 => "BnB-4bit".into(),
            QuantMethod::Bnb8 => "BnB-8bit".into(),
        }
    }

    /// Check if requires calibration
    pub fn requires_calibration(&self) -> bool {
        matches!(self, QuantMethod::Gptq(_) | QuantMethod::Awq(_))
    }

    /// Get recommended use case
    pub fn recommended_for(&self) -> &'static str {
        match self {
            QuantMethod::Gguf(GgufMethod::Q4_0) => "Fast inference, lowest quality",
            QuantMethod::Gguf(GgufMethod::Q4_K_M) => "Balanced quality/speed (recommended)",
            QuantMethod::Gguf(GgufMethod::Q4_K_S) => "Faster than Q4_K_M, slightly lower quality",
            QuantMethod::Gguf(GgufMethod::Q5_0) => "Good quality 5-bit",
            QuantMethod::Gguf(GgufMethod::Q5_K_M) => "High quality, larger size",
            QuantMethod::Gguf(GgufMethod::Q5_K_S) => "Good quality, medium size",
            QuantMethod::Gguf(GgufMethod::Q6_K) => "Very high quality, large size",
            QuantMethod::Gguf(GgufMethod::Q8_0) => "Near-original quality",
            QuantMethod::Gguf(GgufMethod::F16) => "Full precision (debugging)",
            QuantMethod::Gguf(GgufMethod::F32) => "Full FP32 (debugging)",
            QuantMethod::Gptq(GptqMethod::Gptq4) => "vLLM/AutoGPTQ deployment",
            QuantMethod::Gptq(GptqMethod::Gptq8) => "Higher quality GPTQ",
            QuantMethod::Awq(AwqMethod::Awq4) => "Fast AWQ inference",
            QuantMethod::Awq(AwqMethod::Awq8) => "Higher quality AWQ",
            QuantMethod::Bnb4 => "Training/LoRA with QLoRA",
            QuantMethod::Bnb8 => "Training with reduced memory",
        }
    }
}

impl Default for QuantMethod {
    fn default() -> Self {
        Self::gguf_q4km()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  GGUF METHOD
// ═══════════════════════════════════════════════════════════════════════════════

/// GGUF quantization methods
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GgufMethod {
    /// 4-bit, block quantization (legacy)
    #[serde(rename = "Q4_0")]
    Q4_0,
    
    /// 4-bit, K-quantization medium
    #[serde(rename = "Q4_K_M")]
    Q4_K_M,
    
    /// 4-bit, K-quantization small
    #[serde(rename = "Q4_K_S")]
    Q4_K_S,
    
    /// 5-bit, block quantization (legacy)
    #[serde(rename = "Q5_0")]
    Q5_0,
    
    /// 5-bit, K-quantization medium
    #[serde(rename = "Q5_K_M")]
    Q5_K_M,
    
    /// 5-bit, K-quantization small
    #[serde(rename = "Q5_K_S")]
    Q5_K_S,
    
    /// 6-bit, K-quantization
    #[serde(rename = "Q6_K")]
    Q6_K,
    
    /// 8-bit, block quantization
    #[serde(rename = "Q8_0")]
    Q8_0,
    
    /// 16-bit float
    #[serde(rename = "F16")]
    F16,
    
    /// 32-bit float
    #[serde(rename = "F32")]
    F32,
}

impl GgufMethod {
    /// Get effective bits per weight
    pub fn bits(&self) -> u8 {
        match self {
            GgufMethod::Q4_0 => 4,
            GgufMethod::Q4_K_M => 4,
            GgufMethod::Q4_K_S => 4,
            GgufMethod::Q5_0 => 5,
            GgufMethod::Q5_K_M => 5,
            GgufMethod::Q5_K_S => 5,
            GgufMethod::Q6_K => 6,
            GgufMethod::Q8_0 => 8,
            GgufMethod::F16 => 16,
            GgufMethod::F32 => 32,
        }
    }

    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            GgufMethod::Q4_0 => "Q4_0",
            GgufMethod::Q4_K_M => "Q4_K_M",
            GgufMethod::Q4_K_S => "Q4_K_S",
            GgufMethod::Q5_0 => "Q5_0",
            GgufMethod::Q5_K_M => "Q5_K_M",
            GgufMethod::Q5_K_S => "Q5_K_S",
            GgufMethod::Q6_K => "Q6_K",
            GgufMethod::Q8_0 => "Q8_0",
            GgufMethod::F16 => "F16",
            GgufMethod::F32 => "F32",
        }
    }

    /// Get file extension suffix
    pub fn file_suffix(&self) -> &'static str {
        match self {
            GgufMethod::Q4_0 => "Q4_0",
            GgufMethod::Q4_K_M => "Q4_K_M",
            GgufMethod::Q4_K_S => "Q4_K_S",
            GgufMethod::Q5_0 => "Q5_0",
            GgufMethod::Q5_K_M => "Q5_K_M",
            GgufMethod::Q5_K_S => "Q5_K_S",
            GgufMethod::Q6_K => "Q6_K",
            GgufMethod::Q8_0 => "Q8_0",
            GgufMethod::F16 => "F16",
            GgufMethod::F32 => "F32",
        }
    }

    /// List all GGUF methods
    pub fn all() -> Vec<Self> {
        vec![
            GgufMethod::Q4_0,
            GgufMethod::Q4_K_M,
            GgufMethod::Q4_K_S,
            GgufMethod::Q5_0,
            GgufMethod::Q5_K_M,
            GgufMethod::Q5_K_S,
            GgufMethod::Q6_K,
            GgufMethod::Q8_0,
            GgufMethod::F16,
            GgufMethod::F32,
        ]
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  GPTQ METHOD
// ═══════════════════════════════════════════════════════════════════════════════

/// GPTQ quantization methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GptqMethod {
    /// 4-bit GPTQ
    #[serde(rename = "gptq-4bit")]
    Gptq4,
    
    /// 8-bit GPTQ
    #[serde(rename = "gptq-8bit")]
    Gptq8,
}

impl GptqMethod {
    pub fn bits(&self) -> u8 {
        match self {
            GptqMethod::Gptq4 => 4,
            GptqMethod::Gptq8 => 8,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            GptqMethod::Gptq4 => "4bit",
            GptqMethod::Gptq8 => "8bit",
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  AWQ METHOD
// ═══════════════════════════════════════════════════════════════════════════════

/// AWQ quantization methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AwqMethod {
    /// 4-bit AWQ
    #[serde(rename = "awq-4bit")]
    Awq4,
    
    /// 8-bit AWQ (less common)
    #[serde(rename = "awq-8bit")]
    Awq8,
}

impl AwqMethod {
    pub fn bits(&self) -> u8 {
        match self {
            AwqMethod::Awq4 => 4,
            AwqMethod::Awq8 => 8,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            AwqMethod::Awq4 => "4bit",
            AwqMethod::Awq8 => "8bit",
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
    fn test_gguf_bits() {
        assert_eq!(GgufMethod::Q4_0.bits(), 4);
        assert_eq!(GgufMethod::Q4_K_M.bits(), 4);
        assert_eq!(GgufMethod::Q8_0.bits(), 8);
        assert_eq!(GgufMethod::F16.bits(), 16);
    }

    #[test]
    fn test_quant_method_shortcuts() {
        let q4km = QuantMethod::gguf_q4km();
        assert!(matches!(q4km, QuantMethod::Gguf(GgufMethod::Q4_K_M)));

        let gptq = QuantMethod::gptq_4bit();
        assert!(matches!(gptq, QuantMethod::Gptq(GptqMethod::Gptq4)));
    }

    #[test]
    fn test_requires_calibration() {
        assert!(!QuantMethod::gguf_q4km().requires_calibration());
        assert!(QuantMethod::gptq_4bit().requires_calibration());
        assert!(QuantMethod::awq_4bit().requires_calibration());
    }

    #[test]
    fn test_recommended_for() {
        let rec = QuantMethod::gguf_q4km().recommended_for();
        assert!(rec.contains("Balanced"));
    }

    #[test]
    fn test_gguf_all() {
        let all = GgufMethod::all();
        assert_eq!(all.len(), 10);
    }

    #[test]
    fn test_serialization() {
        let method = QuantMethod::gguf_q4km();
        let json = serde_json::to_string(&method).unwrap();
        assert!(json.contains("Q4_K_M"));
        
        let parsed: QuantMethod = serde_json::from_str(&json).unwrap();
        assert_eq!(method, parsed);
    }
}
