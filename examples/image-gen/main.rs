// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Image Generation Example
// ═══════════════════════════════════════════════════════════════════════════════

use sentient_image::{
    ImageConfig, ImageProviderType, ImageProvider,
    ImageRequest, ImageSize, ImageQuality, ImageStyle,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 SENTIENT OS - Image Generation Example");
    println!("═══════════════════════════════════════════════════════════\n");

    // === Configuration ===
    println!("📋 1. Configuration");
    println!("─────────────────────────────\n");

    let config = ImageConfig::new()
        .with_provider(ImageProviderType::OpenAI)
        .with_model("dall-e-3");

    println!("Default provider: {:?}", config.default_provider);
    println!("Default model: {}", config.default_model);
    println!("Default size: {}x{}\n", config.default_size.width(), config.default_size.height());

    // === Available Providers ===
    println!("📋 2. Available Providers");
    println!("─────────────────────────────\n");

    println!("Provider        | Models              | Pricing");
    println!("────────────────|─────────────────────|────────────────");
    println!("OpenAI DALL-E   | dall-e-2, dall-e-3  | $0.04-$0.12/img");
    println!("Stability AI    | SDXL, SD 2.1       | $0.002-$0.04/img");
    println!("Flux            | flux-pro, schnell  | $0.003-$0.05/img");
    println!("Ideogram        | v2                 | $0.04-$0.08/img\n");

    // === Image Sizes ===
    println!("📐 3. Image Sizes");
    println!("─────────────────────────────\n");

    println!("Size            | Dimensions  | Type");
    println!("────────────────|─────────────|────────────");
    for size in &[
        ImageSize::Small256,
        ImageSize::Medium512,
        ImageSize::Square1024,
        ImageSize::Landscape1792,
        ImageSize::Portrait1024,
        ImageSize::HD1280,
    ] {
        let typ = if size.is_square() { "Square" }
                  else if size.is_landscape() { "Landscape" }
                  else { "Portrait" };
        println!("{:15} | {:4}x{:<5} | {}", 
            size.to_string_api(), size.width(), size.height(), typ);
    }
    println!();

    // === DALL-E 3 Example ===
    println!("🖼️  4. DALL-E 3 Request");
    println!("─────────────────────────────\n");

    let dalle_request = ImageRequest::dalle3("A serene mountain landscape at sunset with a lake reflection")
        .with_size(ImageSize::Landscape1792)
        .with_quality(ImageQuality::HD)
        .with_style(ImageStyle::Vivid);

    println!("Prompt: {}", dalle_request.prompt);
    println!("Model: {}", dalle_request.model);
    println!("Quality: {:?}", dalle_request.quality);
    println!("Style: {:?}", dalle_request.style);
    println!("Size: {}", dalle_request.size.to_string_api());
    println!();

    // === Stable Diffusion Example ===
    println!("🎭  5. Stable Diffusion XL Request");
    println!("─────────────────────────────\n");

    let sdxl_request = ImageRequest::sdxl("A cyberpunk city at night")
        .with_negative_prompt("blurry, low quality, watermark")
        .with_steps(50)
        .with_cfg_scale(8.0)
        .with_seed(42);

    println!("Prompt: {}", sdxl_request.prompt);
    println!("Negative: {:?}", sdxl_request.negative_prompt);
    println!("Steps: {:?}", sdxl_request.steps);
    println!("CFG Scale: {:?}", sdxl_request.cfg_scale);
    println!();

    // === Multi-Provider ===
    println!("🔄 6. Multi-Provider Setup");
    println!("─────────────────────────────\n");

    println!("Setup multiple providers:\n");
    println!("let provider = ImageProvider::new()");
    println!("    .add_provider(ImageProviderType::OpenAI, \"sk-...\")?");
    println!("    .add_provider(ImageProviderType::StabilityAI, \"sk-...\")?");
    println!("    .with_default(ImageProviderType::OpenAI);\n");

    println!("Generate with default:");
    println!("let image = provider.generate(&request).await?;\n");

    println!("Generate with specific provider:");
    println!("let image = provider.generate_with(ImageProviderType::StabilityAI, &request).await?;\n");

    // === Pricing ===
    println!("💰 7. Pricing Comparison");
    println!("─────────────────────────────\n");

    println!("Model           | Quality | Price/Image | Speed");
    println!("────────────────|─────────|-------------|-------");
    println!("DALL-E 3 HD     | HD      | $0.120      | ~15s");
    println!("DALL-E 3 Std    | Std     | $0.040      | ~10s");
    println!("DALL-E 2        | Std     | $0.020      | ~10s");
    println!("SDXL            | High    | $0.040      | ~6s");
    println!("SD 2.1          | Std     | $0.002      | ~4s");
    println!("Flux Pro        | High    | $0.050      | ~4s");
    println!("Flux Schnell    | Fast    | $0.003      | ~1s\n");

    // === Use Cases ===
    println!("🎯 8. AI Agent Use Cases");
    println!("─────────────────────────────\n");

    println!("Image generation enables AI agents to:");
    println!("  ✅ Create visual content");
    println!("  ✅ Generate UI mockups");
    println!("  ✅ Create marketing materials");
    println!("  ✅ Illustrate concepts");
    println!("  ✅ Generate thumbnails");
    println!("  ✅ Create social media content\n");

    // === Summary ===
    println!("═══════════════════════════════════════════════════════════");
    println!("✅ Example complete!");
    println!("\nTo use Image Generation:");
    println!("1. Get API key from provider (OpenAI, Stability, etc.)");
    println!("2. Set environment: export OPENAI_API_KEY=your_key");
    println!("3. Run: cargo run --example image-gen");

    Ok(())
}
