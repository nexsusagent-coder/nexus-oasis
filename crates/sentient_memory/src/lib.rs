//! ─── SENTIENT MEMORY (BİLİŞSEL BELLEK / HİPOKAMPÜS) ───
//!
//! RAG (Retrieval-Augmented Generation) destekli, vektör tabanlı uzun süreli hafıza.
//! - Episodik Bellek: Deneyimler ve olaylar
//! - Semantik Bellek: Bilgiler ve gerçekler
//! - Prosedürel Bellek: Beceriler ve yöntemler
//!
//! MemOS Mimarisi:
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────┐
//! │                         MEMOS                                       │
//! ├─────────────────────────────────────────────────────────────────────┤
//! │                                                                     │
//! │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐                    │
//! │  │  CUBE A     │ │  CUBE B     │ │  CUBE C     │                    │
//! │  │  (User 1)   │ │  (User 2)   │ │  (Agent)    │                    │
//! │  └──────┬──────┘ └──────┬──────┘ └──────┬──────┘                    │
//! │         │               │               │                          │
//! │         └───────────────┼───────────────┘                          │
//! │                         ▼                                          │
//! │              ┌─────────────────────┐                               │
//! │              │  HYBRID SEARCH      │                               │
//! │              │  FTS5 + Vector      │                               │
//! │              └──────────┬──────────┘                               │
//! │                         │                                          │
//! │                         ▼                                          │
//! │              ┌─────────────────────┐                               │
//! │              │   MEMSCHEDULER     │                               │
//! │              │  Async Task Queue  │                               │
//! │              └─────────────────────┘                               │
//! │                                                                     │
//! └─────────────────────────────────────────────────────────────────────┘
//! ```

pub mod types;
pub mod cube;
pub mod embeddings;
pub mod vector_index;
pub mod rag;
pub mod knowledge_graph;
pub mod consolidation;
pub mod decay;
pub mod scheduler;
pub mod fts;
pub mod memos;
pub mod tools;
pub mod compression;
pub mod migration;
pub mod distributed;

// Re-exports
pub use types::*;
pub use cube::MemoryCube;
pub use embeddings::EmbeddingEngine;
pub use vector_index::VectorIndex;
pub use rag::RagEngine;
pub use knowledge_graph::KnowledgeGraph;
pub use consolidation::MemoryConsolidator;
pub use decay::MemoryDecay;
pub use scheduler::{MemScheduler, SchedulerConfig, TaskPriority, TaskStatus, ScheduledTask, MemTask};
pub use fts::{FtsEngine, FtsOptions, FtsResult, HybridSearchEngine, HybridWeights, HybridResult};
pub use memos::{MemOS, MemOSConfig, MemOSStats, CubeMeta, CubeType};
pub use compression::{MemoryCompressor, CompressedEntry};
pub use migration::{MigrationManager, MigrationResult, Migration};
pub use distributed::{DistributedMemoryManager, ReplicationConfig, MemoryNode, NodeStatus, ConsistencyLevel, ClusterStats};

// New Tools Re-exports
pub use tools::{MemoryTool, MemoryToolInput, MemoryToolOutput, MemoryEntrySummary, ToolContext};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        assert!(true);
    }
}
