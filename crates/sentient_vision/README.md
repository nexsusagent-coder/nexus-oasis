# sentient_vision

**Vision and Multimodal AI** capabilities for SENTIENT OS.

[![Crates.io](https://img.shields.io/crates/v/sentient_vision.svg)](https://crates.io/crates/sentient_vision)
[![Documentation](https://docs.rs/sentient_vision/badge.svg)](https://docs.rs/sentient_vision)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

## Overview

This crate provides comprehensive vision and multimodal AI capabilities:

- 🖼️ **Image Processing**: Load, resize, crop, convert, analyze
- 📝 **OCR**: Extract text from images and documents
- 🤖 **Vision Models**: GPT-4V, Claude 3, Gemini Vision
- 🔢 **Multimodal Embeddings**: CLIP-style image/text embeddings
- 🎨 **Color Extraction**: Dominant color analysis
- 👤 **Face Detection**: Face and landmark detection

## Features

| Feature | Description | Default |
|---------|-------------|---------|
| `api` | HTTP API providers (OpenAI, Claude) | ✅ |
| `pdf` | PDF processing support | ❌ |
| `onnx` | ONNX Runtime for local inference | ❌ |
| `full` | All features enabled | ❌ |

## Installation

```toml
[dependencies]
sentient_vision = { path = "crates/sentient_vision" }

# With all features
sentient_vision = { path = "crates/sentient_vision", features = ["full"] }
```

## Quick Start

### Image Processing

```rust
use sentient_vision::{ImageProcessor, ImageFormat};

let processor = ImageProcessor::new(1024);

// Load image
let image_data = std::fs::read("photo.jpg")?;
let img = processor.load(&image_data)?;

// Process (resize and encode)
let processed = processor.process(&img)?;
println!("Size: {}x{}", processed.size.width, processed.size.height);

// Extract colors
let colors = processor.extract_colors(&img, 5)?;
for color in colors {
    println!("Color: {} ({:.1}%)", color.to_hex(), color.percentage * 100.0);
}

// Convert format
let png_data = processor.encode(&img, ImageFormat::Png)?;
```

### Vision Analysis

```rust
use sentient_vision::{VisionManager, VisionOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = VisionManager::new();
    
    let options = VisionOptions {
        describe: true,
        detect_objects: true,
        extract_text: true,
        ..Default::default()
    };
    
    let image_data = std::fs::read("photo.jpg")?;
    let analysis = manager.analyze(&image_data, &options).await?;
    
    if let Some(desc) = analysis.description {
        println!("Description: {}", desc.description);
    }
    
    for object in analysis.objects {
        println!("Object: {} ({:.0}%)", object.label, object.confidence * 100.0);
    }
    
    Ok(())
}
```

### OCR

```rust
use sentient_vision::{OcrManager, OcrOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = OcrManager::new();
    
    let options = OcrOptions::multi(vec!["en", "tr"])
        .with_min_confidence(0.8);
    
    let image_data = std::fs::read("document.png")?;
    let result = manager.recognize(&image_data, &options).await?;
    
    println!("Text:\n{}", result.full_text);
    println!("Confidence: {:.0}%", result.confidence * 100.0);
    
    Ok(())
}
```

### Multimodal Embeddings

```rust
use sentient_vision::EmbeddingManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = EmbeddingManager::new();
    
    let image_data = std::fs::read("photo.jpg")?;
    let image_emb = manager.embed_image(&image_data).await?;
    
    let text_emb = manager.embed_text("a photo of a cat").await?;
    
    let similarity = image_emb.cosine_similarity(&text_emb);
    println!("Similarity: {:.3}", similarity);
    
    Ok(())
}
```

## Supported Providers

### Vision Models

| Provider | Model | Features |
|----------|-------|----------|
| OpenAI | GPT-4o, GPT-4V | Description, Q&A |
| Anthropic | Claude 3.5 Sonnet | Description, Q&A |
| Google | Gemini Pro Vision | Description, Q&A |
| Local | ONNX Runtime | Object Detection |

### OCR

| Provider | Languages | Features |
|----------|-----------|----------|
| Tesseract | 100+ | Text extraction, Layout |
| EasyOCR | 80+ | Multi-language |
| PaddleOCR | 80+ | Chinese, English |

### Embeddings

| Provider | Model | Dimension |
|----------|-------|-----------|
| OpenAI | text-embedding-3-small | 1536 |
| OpenAI | text-embedding-3-large | 3072 |
| Cohere | embed-multilingual-v3.0 | 1024 |
| CLIP | ViT-B/32 | 512 |

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    sentient_vision                          │
├─────────────────────────────────────────────────────────────┤
│  ┌───────────────┐  ┌───────────────┐  ┌───────────────┐   │
│  │ ImageProcessor│  │  VisionManager│  │   OcrManager  │   │
│  └───────────────┘  └───────────────┘  └───────────────┘   │
│          │                  │                  │            │
│          ▼                  ▼                  ▼            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                  Provider Layer                      │   │
│  │  ┌─────┐ ┌───────┐ ┌────────┐ ┌───────┐ ┌───────┐  │   │
│  │  │OpenAI│ │Claude│ │Gemini  │ │Tesseract│ │CLIP  │  │   │
│  │  └─────┘ └───────┘ └────────┘ └───────┘ └───────┘  │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Integration with SENTIENT OS

```rust
// In sentient_cevahir or sentient_gateway
use sentient_vision::{VisionManager, VisionOptions};

async fn process_multimodal_input(
    image: &[u8],
    text: &str,
) -> Result<String> {
    let vision = VisionManager::new();
    
    // Analyze image
    let analysis = vision.analyze(image, &VisionOptions::default()).await?;
    
    // Combine with text context
    let context = if let Some(desc) = analysis.description {
        format!("[Image: {}]\n\n{}", desc.description, text)
    } else {
        text.to_string()
    };
    
    // Send to LLM...
    Ok(context)
}
```

## License

Apache License 2.0

---

*SENTIENT OS - The Operating System That Thinks*
