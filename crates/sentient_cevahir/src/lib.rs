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
//! - **Memory & RAG**: Vector store, semantic cache
//! - **Tool Execution**: Dynamic tool registration
//!
//! ## Kullanım
//!
//! ```rust,no_run
//! use sentient_cevahir::{CevahirBridge, CevahirConfig, CognitiveStrategy};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = CevahirConfig::default();
//!     let bridge = CevahirBridge::new(config)?;
//!     
//!     let response = bridge.generate("Merhaba dünya", 128).await?;
//!     println!("Response: {}", response);
//!     
//!     Ok(())
//! }
//! ```

pub mod types;
pub mod config;
pub mod cognitive;
pub mod tokenizer;
pub mod model;
pub mod bridge;
pub mod tools;
pub mod memory;
pub mod error;

// Re-exports
pub use types::*;
pub use config::CevahirConfig;
pub use cognitive::{CognitiveManager};
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
