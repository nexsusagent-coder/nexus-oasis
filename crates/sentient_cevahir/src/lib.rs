//! # Sentient Cevahir - LLM Cognitive Engine
//!
//! Bu crate, [Cevahir AI](https://github.com/myylogic/cevahir-ai) projesinin
//! SENTIENT OS ile entegrasyonunu sağlar.
//!
//! ## Özellikler
//!
//! - **Neural Network (V-7)**: RoPE, RMSNorm, SwiGLU, KV Cache, MoE, GQA
//! - **Cognitive Strategies**: Direct, Think, Debate, Tree of Thoughts
//! - **Turkish BPE Tokenizer**: Native Türkçe tokenizer

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
//! - **Memory & RAG**: Vector store, semantic cache
//! - **Tool Execution**: Dynamic tool registration

pub mod types;
pub mod config;
pub mod cognitive;
pub mod tokenizer;
pub mod model;
pub mod bridge;
pub mod tools;
pub mod memory;
pub mod error;

#[cfg(feature = "python")]
pub mod python;

// Re-exports
pub use types::*;
pub use config::CevahirConfig;
pub use cognitive::CognitiveManager;
pub use types::{Strategy, CognitiveResult};
pub use tokenizer::TokenizerWrapper;
pub use model::ModelWrapper;
pub use bridge::CevahirBridge;
pub use tools::{ToolDefinition, ToolExecutor};
pub use memory::MemoryAdapter;
pub use error::{CevahirError, Result};

/// Crate version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Cevahir AI kaynak versiyonu
pub const CEVAHIR_VERSION: &str = "V-7";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert!(!CEVAHIR_VERSION.is_empty());
    }
}
