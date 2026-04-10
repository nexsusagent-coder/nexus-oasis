# sentient_rag

**Native RAG (Retrieval-Augmented Generation) Engine** for SENTIENT OS.

[![Crates.io](https://img.shields.io/crates/v/sentient_rag.svg)](https://crates.io/crates/sentient_rag)
[![Documentation](https://docs.rs/sentient_rag/badge.svg)](https://docs.rs/sentient_rag)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

## Overview

This crate provides a complete RAG system for SENTIENT OS:

- 📝 **Document Chunking**: Multiple strategies (fixed, sentence, paragraph, recursive, code)
- 🧮 **Embeddings**: Local (fastembed) and remote (OpenAI, etc.) embedding generation
- 💾 **Vector Store**: In-memory and LanceDB for persistent storage
- 🔍 **Semantic Search**: Cosine, Euclidean, Dot Product distance metrics
- 🔄 **RAG Pipeline**: Complete end-to-end RAG workflow

## Features

| Feature | Description | Default |
|---------|-------------|---------|
| `chunking` | Document chunking | ✅ |
| `memory-store` | In-memory vector store | ✅ |
| `embeddings` | Local embedding with fastembed | ❌ |
| `remote-embeddings` | Remote embedding APIs | ❌ |
| `vector-store` | LanceDB persistent storage | ❌ |
| `full` | All features enabled | ❌ |

## Installation

```toml
[dependencies]
sentient_rag = { path = "crates/sentient_rag" }

# With all features
sentient_rag = { path = "crates/sentient_rag", features = ["full"] }
```

## Quick Start

### Basic Usage

```rust
use sentient_rag::{RagPipeline, Document};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create pipeline with defaults
    let pipeline = RagPipeline::default_pipeline()?;

    // Ingest documents
    let doc = Document::new("The quick brown fox jumps over the lazy dog.");
    pipeline.ingest(&doc).await?;

    // Query
    let response = pipeline.query("What does the fox do?").await?;
    println!("Response: {}", response.response);

    Ok(())
}
```

### Custom Configuration

```rust
use sentient_rag::{
    RagPipeline, ChunkingStrategy, DistanceMetric,
    ChunkingConfig, EmbeddingConfig, IndexConfig,
};

let pipeline = RagPipeline::builder()
    .chunk_size(512)
    .chunk_overlap(50)
    .chunking_strategy(ChunkingStrategy::Recursive)
    .embedding_dimension(384)
    .distance_metric(DistanceMetric::Cosine)
    .build()
    .await?;
```

## Document Chunking

### Strategies

| Strategy | Description | Use Case |
|----------|-------------|----------|
| `FixedSize` | Fixed character size | Simple, uniform chunks |
| `Sentence` | Sentence boundaries | Natural text |
| `Paragraph` | Paragraph boundaries | Articles, essays |
| `Recursive` | Try larger separators first | General purpose |
| `Code` | Code-aware splitting | Source code |

### Example

```rust
use sentient_rag::{Chunker, ChunkingConfig, ChunkingStrategy};

let config = ChunkingConfig {
    strategy: ChunkingStrategy::Code,
    chunk_size: 512,
    overlap: 50,
    ..Default::default()
};

let chunker = Chunker::new(config);
let chunks = chunker.chunk_text(r#"
fn main() {
    println!("Hello, world!");
}
"#)?;
```

## Embeddings

### Local (fastembed)

```rust
use sentient_rag::embedder::{LocalEmbedder, EmbeddingConfig};

#[cfg(feature = "embeddings")]
{
    let config = EmbeddingConfig {
        model: "all-MiniLM-L6-v2".to_string(),
        dimension: 384,
        ..Default::default()
    };

    let embedder = LocalEmbedder::new(config)?;
    let embedding = embedder.embed("Hello, world!").await?;
}
```

### Remote (OpenAI)

```rust
use sentient_rag::embedder::RemoteEmbedder;

#[cfg(feature = "remote-embeddings")]
{
    let embedder = RemoteEmbedder::openai("your-api-key");
    let embedding = embedder.embed("Hello, world!").await?;
}
```

## Vector Store

### In-Memory

```rust
use sentient_rag::store::{MemoryStore, StoreBuilder, IndexConfig};

let store = MemoryStore::new(IndexConfig {
    dimension: 384,
    ..Default::default()
});
```

### LanceDB (Persistent)

```rust
use sentient_rag::store::StoreBuilder;

#[cfg(feature = "vector-store")]
{
    let store = StoreBuilder::new()
        .lance("./data/vectors")
        .dimension(384)
        .build()
        .await?;
}
```

## Retrieval

```rust
use sentient_rag::{Retriever, SearchQuery};

// Basic search
let results = retriever.search("query text").await?;

// With filters
let mut filters = std::collections::HashMap::new();
filters.insert("category".to_string(), "tech".to_string());
let results = retriever.search_filtered("query", filters, 5).await?;

// Hybrid search (keyword + semantic)
let results = retriever.hybrid_search("query", 0.3, 5).await?;
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    sentient_rag                             │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Chunker   │  │  Embedder   │  │VectorStore  │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
│         │               │                │                  │
│         └───────────────┼────────────────┘                  │
│                         │                                   │
│                         ▼                                   │
│               ┌─────────────────┐                          │
│               │    Retriever    │                          │
│               └─────────────────┘                          │
│                         │                                   │
│                         ▼                                   │
│               ┌─────────────────┐                          │
│               │  RAG Pipeline   │                          │
│               └─────────────────┘                          │
└─────────────────────────────────────────────────────────────┘
```

## Distance Metrics

| Metric | Description | Formula |
|--------|-------------|---------|
| `Cosine` | Cosine similarity | 1 - (a·b / ‖a‖‖b‖) |
| `Euclidean` | L2 distance | √Σ(aᵢ - bᵢ)² |
| `DotProduct` | Dot product similarity | a·b |
| `Manhattan` | L1 distance | Σ\|aᵢ - bᵢ\| |

## License

Apache License 2.0

---

*SENTIENT OS - The Operating System That Thinks*
