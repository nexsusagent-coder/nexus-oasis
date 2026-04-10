# sentient_finetuning

**Fine-tuning support for SENTIENT OS** - LoRA, QLoRA, dataset preparation.

[![Crates.io](https://img.shields.io/crates/v/sentient_finetuning.svg)](https://crates.io/crates/sentient_finetuning)
[![Documentation](https://docs.rs/sentient_finetuning/badge.svg)](https://docs.rs/sentient_finetuning)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

## Overview

This crate provides comprehensive fine-tuning support for SENTIENT OS:

- 📁 **Dataset Handling**: Load/save datasets in multiple formats (JSON, JSONL, CSV)
- 🎯 **Training Engine**: Async training with progress monitoring
- 🔧 **Fine-tuning Methods**: LoRA, QLoRA, Full fine-tuning, Prefix/Prompt tuning
- 💾 **Checkpointing**: Save/resume training from checkpoints
- ⚙️ **Hyperparameters**: Comprehensive hyperparameter configuration

## Features

| Feature | Description | Default |
|---------|-------------|---------|
| `lora` | LoRA fine-tuning with candle | ❌ |
| `qlora` | QLoRA (quantized LoRA) | ❌ |
| `full` | All features enabled | ❌ |

## Installation

```toml
[dependencies]
sentient_finetuning = { path = "crates/sentient_finetuning" }

# With LoRA support
sentient_finetuning = { path = "crates/sentient_finetuning", features = ["lora"] }
```

## Quick Start

### Load Dataset

```rust
use sentient_finetuning::{DatasetLoader, DatasetSaver, Dataset};

// Load from JSONL
let dataset = DatasetLoader::new("data.jsonl")
    .with_input_field("prompt")
    .with_output_field("response")
    .load()?;

// Or create programmatically
let mut dataset = Dataset::new("my_dataset");
dataset.add_sample(TrainingSample::new("Question?", "Answer."));
```

### Configure Training

```rust
use sentient_finetuning::{
    TrainingConfig, Hyperparameters, FinetuningMethod, LoraConfig,
};

let config = TrainingConfig::new("gemma-4-4b")
    .with_output_dir("./output")
    .with_method(FinetuningMethod::Lora)
    .with_hyperparameters(
        Hyperparameters::new()
            .with_learning_rate(1e-4)
            .with_batch_size(8)
            .with_epochs(3)
    );
```

### Run Training

```rust
use sentient_finetuning::TrainingEngine;

let mut engine = TrainingEngine::new(config, dataset);
let handle = engine.handle();

// Monitor training in background
tokio::spawn(async move {
    let mut events = handle.subscribe();
    while let Ok(event) = events.recv().await {
        println!("Event: {:?}", event);
    }
});

// Train
let model = engine.train().await?;
println!("Trained model: {}", model.id);
```

## Supported Formats

| Format | Extension | Description |
|--------|-----------|-------------|
| JSON Array | `.json` | Array of objects |
| JSONL | `.jsonl` | One JSON per line |
| CSV | `.csv` | Comma-separated |
| Conversation | `.json` | ShareGPT-style conversations |

## Fine-tuning Methods

### LoRA (Low-Rank Adaptation)

```rust
use sentient_finetuning::{FinetuningMethod, LoraConfig};

let lora_config = LoraConfig::new(16)  // rank = 16
    .with_target_modules(vec![
        "q_proj".to_string(),
        "v_proj".to_string(),
    ])
    .with_dropout(0.05);

let config = TrainingConfig::new("model")
    .with_method(FinetuningMethod::Lora);
```

### QLoRA (Quantized LoRA)

```rust
let config = TrainingConfig::new("model")
    .with_method(FinetuningMethod::Qlora);
```

## Hyperparameters

```rust
let hp = Hyperparameters::new()
    .with_learning_rate(2e-4)
    .with_batch_size(16)
    .with_epochs(5);

// Advanced
let hp = Hyperparameters {
    learning_rate: 2e-4,
    batch_size: 16,
    epochs: 5,
    warmup_steps: 100,
    weight_decay: 0.01,
    gradient_accumulation_steps: 4,
    max_grad_norm: 1.0,
    lr_scheduler: LrScheduler::Cosine,
    optimizer: Optimizer::AdamW,
    seed: Some(42),
};
```

## Learning Rate Schedulers

| Scheduler | Description |
|-----------|-------------|
| `Constant` | No decay |
| `Linear` | Linear decay |
| `Cosine` | Cosine annealing |
| `CosineWithRestarts` | Cosine with warm restarts |
| `Polynomial` | Polynomial decay |
| `InverseSqrt` | Inverse square root decay |

## Optimizers

| Optimizer | Description |
|-----------|-------------|
| `Adam` | Adam optimizer |
| `AdamW` | Adam with weight decay (default) |
| `Sgd` | Stochastic gradient descent |
| `AdaFactor` | Memory-efficient optimizer |
| `LAMB` | Large batch optimizer |

## Training Events

```rust
match event {
    TrainingEvent::Started { config, total_steps } => { }
    TrainingEvent::Progress { metrics } => {
        println!("Step {}/{} | Loss: {:.4}",
            metrics.step, metrics.total_steps, metrics.train_loss);
    }
    TrainingEvent::Validation { step, val_loss } => { }
    TrainingEvent::CheckpointSaved { path, step } => { }
    TrainingEvent::Completed { final_loss, duration_secs } => { }
    TrainingEvent::Failed { error } => { }
    _ => { }
}
```

## Dataset Statistics

```rust
use sentient_finetuning::DatasetUtils;

let stats = DatasetUtils::stats(&dataset);
println!("Samples: {}", stats.total_samples);
println!("Avg input length: {:.1}", stats.avg_input_len);
println!("Avg output length: {:.1}", stats.avg_output_len);
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                   sentient_finetuning                       │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │DatasetLoader │  │DatasetSaver  │  │DatasetUtils  │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│         │                                                   │
│         ▼                                                   │
│  ┌──────────────────────────────────────────────────┐      │
│  │              TrainingEngine                       │      │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐  │      │
│  │  │ Config     │  │ Hyperparams│  │ State      │  │      │
│  │  └────────────┘  └────────────┘  └────────────┘  │      │
│  └──────────────────────────────────────────────────┘      │
│         │                                                   │
│         ▼                                                   │
│  ┌──────────────┐                                          │
│  │TrainingHandle│◄── Events, Metrics, Cancel               │
│  └──────────────┘                                          │
└─────────────────────────────────────────────────────────────┘
```

## License

Apache License 2.0

---

*SENTIENT OS - The Operating System That Thinks*
