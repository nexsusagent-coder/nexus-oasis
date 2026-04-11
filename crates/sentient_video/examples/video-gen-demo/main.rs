//! ─── Video Generation Comprehensive Demo ───
//!
//! Demonstrates all video generation providers and features

use sentient_video::{
    VideoClient, VideoRequest, VideoBuilder, VideoModel,
    AspectRatio, VideoResolution, VideoStyle, CameraMotion,
    ProviderInfo, TemplateLibrary, TemplateCategory,
};

#[tokio::main]
async fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║       SENTIENT OS - Video Generation Demo                ║");
    println!("║       2025 Edition - All Providers                       ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    // ══════════════════════════════════════════════════════════════════════════
    //  SECTION 1: PROVIDER COMPARISON
    // ══════════════════════════════════════════════════════════════════════════
    
    println!("📊 ═══ PROVIDER COMPARISON ═══");
    println!();

    let providers = VideoClient::all_providers();
    
    println!("| Provider | Free Tier | Text→Video | Image→Video | Max Dur | Speed |");
    println!("|----------|-----------|------------|-------------|---------|-------|");
    
    for p in &providers {
        let t2v = if p.supports_text_to_video { "✅" } else { "❌" };
        let i2v = if p.supports_image_to_video { "✅" } else { "❌" };
        let free = if p.free_tier { "✅" } else { "❌" };
        let speed = format!("~{}s", p.avg_generation_time_seconds as i32);
        
        println!("| {:8} | {:9} | {:10} | {:11} | {:5}s   | {:5} |", 
            p.name, free, t2v, i2v, p.max_duration_seconds as i32, speed);
    }
    println!();

    // ══════════════════════════════════════════════════════════════════════════
    //  SECTION 2: PRICING (2025)
    // ══════════════════════════════════════════════════════════════════════════
    
    println!("💰 ═══ PRICING (2025) ═══");
    println!();

    println!("┌─────────────┬─────────────────────┬───────────────────────┐");
    println!("│ Provider    │ Free Tier           │ Cost/Second           │");
    println!("├─────────────┼─────────────────────┼───────────────────────┤");
    println!("│ Runway      │ 125 credits (new)   │ $0.05 - $0.20         │");
    println!("│ Pika        │ 250 credits/month   │ $0.02 - $0.03         │");
    println!("│ Luma AI     │ 30 videos/month     │ $0.04                 │");
    println!("│ Kling AI    │ 66 credits/day (!)  │ $0.02 - $0.025        │");
    println!("│ Haiper      │ 150 credits/month   │ $0.02                 │");
    println!("│ Stability   │ 150 credits         │ $0.02 - $0.03         │");
    println!("└─────────────┴─────────────────────┴───────────────────────┘");
    println!();

    // ══════════════════════════════════════════════════════════════════════════
    //  SECTION 3: MODEL COMPARISON
    // ══════════════════════════════════════════════════════════════════════════
    
    println!("🎬 ═══ AVAILABLE MODELS ═══");
    println!();

    // By Quality
    println!("📈 By Quality (Best First):");
    for model in VideoModel::by_quality().iter().take(5) {
        let quality = "⭐".repeat(model.quality_rating as usize);
        println!("   {} - {} ({})", model.name, quality, model.provider);
    }
    println!();

    // By Speed
    println!("⚡ By Speed (Fastest First):");
    for model in VideoModel::by_speed().iter().take(5) {
        let speed = "🚀".repeat(model.speed_rating as usize);
        println!("   {} - {} (~{}s)", model.name, speed, model.avg_generation_time as i32);
    }
    println!();

    // By Cost
    println!("💵 By Cost (Cheapest First):");
    for model in VideoModel::by_cost().iter().take(5) {
        println!("   {} - ${:.3}/second", model.name, model.cost_per_second);
    }
    println!();

    // ══════════════════════════════════════════════════════════════════════════
    //  SECTION 4: COST CALCULATION
    // ══════════════════════════════════════════════════════════════════════════
    
    println!("📊 ═══ COST CALCULATION ═══");
    println!();

    let duration = 5.0;
    println!("For a {} second video:", duration);
    println!();

    for model in VideoModel::all() {
        let cost = model.calculate_cost(duration);
        println!("   {:20} ${:.2}", model.name, cost);
    }
    println!();

    // ══════════════════════════════════════════════════════════════════════════
    //  SECTION 5: VIDEO BUILDER USAGE
    // ══════════════════════════════════════════════════════════════════════════
    
    println!("🛠️ ═══ VIDEO BUILDER USAGE ═══");
    println!();

    // Simple request
    let simple = VideoRequest::text_to_video("A cat playing piano");
    println!("Simple Request:");
    println!("   Prompt: {}", simple.prompt);
    println!();

    // Advanced builder
    let advanced = VideoBuilder::new("A majestic dragon soaring over mountains at sunset")
        .duration(5.0)
        .aspect_ratio(AspectRatio::Landscape16x9)
        .resolution(VideoResolution::FullHD)
        .style(VideoStyle::Cinematic)
        .camera_motion(CameraMotion::Drone)
        .seed(42)
        .negative_prompt("blurry, low quality, bad anatomy")
        .build();

    println!("Advanced Request:");
    println!("   Prompt: {}", advanced.prompt);
    println!("   Duration: {}s", advanced.duration.unwrap_or(0.0));
    println!("   Aspect: {:?}", advanced.aspect_ratio);
    println!("   Style: {:?}", advanced.style);
    println!("   Camera: {:?}", advanced.camera_motion);
    println!();

    // Image-to-video
    let i2v = VideoRequest::image_to_video(
        "Animate with gentle motion",
        "https://example.com/image.jpg"
    );
    println!("Image-to-Video:");
    println!("   Type: {:?}", i2v.generation_type());
    println!();

    // ══════════════════════════════════════════════════════════════════════════
    //  SECTION 6: STYLES AND CAMERA MOTION
    // ══════════════════════════════════════════════════════════════════════════
    
    println!("🎨 ═══ AVAILABLE STYLES ═══");
    println!();

    let styles = [
        VideoStyle::Cinematic,
        VideoStyle::Anime,
        VideoStyle::ThreeDAnimation,
        VideoStyle::Realistic,
        VideoStyle::Cyberpunk,
        VideoStyle::Fantasy,
        VideoStyle::SciFi,
        VideoStyle::Noir,
    ];

    for style in &styles {
        println!("   {:15} → {}", format!("{:?}", style), style.prompt_suffix());
    }
    println!();

    println!("📷 ═══ CAMERA MOTIONS ═══");
    println!();

    let motions = [
        CameraMotion::Static,
        CameraMotion::PanLeft,
        CameraMotion::PanRight,
        CameraMotion::ZoomIn,
        CameraMotion::ZoomOut,
        CameraMotion::DollyIn,
        CameraMotion::Drone,
        CameraMotion::OrbitLeft,
    ];

    for motion in &motions {
        println!("   {:15} → {}", format!("{:?}", motion), motion.prompt_suffix());
    }
    println!();

    // ══════════════════════════════════════════════════════════════════════════
    //  SECTION 7: ASPECT RATIOS
    // ══════════════════════════════════════════════════════════════════════════
    
    println!("📐 ═══ ASPECT RATIOS ═══");
    println!();

    let ratios = [
        AspectRatio::Landscape16x9,
        AspectRatio::Portrait9x16,
        AspectRatio::Square,
        AspectRatio::UltraWide21x9,
        AspectRatio::Cinematic2_39_1,
    ];

    for ratio in &ratios {
        let (w, h) = ratio.dimensions(720);
        println!("   {:20} {}x{} - {}", format!("{:?}", ratio), w, h, ratio.name());
    }
    println!();

    // ══════════════════════════════════════════════════════════════════════════
    //  SECTION 8: TEMPLATES
    // ══════════════════════════════════════════════════════════════════════════
    
    println!("📚 ═══ TEMPLATE LIBRARY ═══");
    println!();

    let library = TemplateLibrary::new();
    let templates = library.list();
    
    println!("{} templates available:", templates.len());
    println!();

    // By category
    for category in [TemplateCategory::Marketing, TemplateCategory::Social, TemplateCategory::Nature] {
        let count = library.by_category(category).len();
        println!("   {:?}: {} templates", category, count);
    }
    println!();

    // Example usage
    println!("Example Template Usage:");
    if let Some(template) = library.get("product-showcase") {
        use std::collections::HashMap;
        
        let mut values = HashMap::new();
        values.insert("product", "luxury smartphone");
        
        let request = template.create_request(&values);
        println!("   Template: {}", template.name);
        println!("   Prompt: {}", request.prompt);
    }
    println!();

    // ══════════════════════════════════════════════════════════════════════════
    //  SECTION 9: CLIENT INITIALIZATION
    // ══════════════════════════════════════════════════════════════════════════
    
    println!("🔌 ═══ CLIENT INITIALIZATION ═══");
    println!();

    println!("Runway (Best Quality):");
    println!("   let client = VideoClient::runway(\"your-api-key\");");
    println!();

    println!("Pika (Fast & Affordable):");
    println!("   let client = VideoClient::pika(\"your-api-key\");");
    println!();

    println!("Luma AI (Near Sora Quality):");
    println!("   let client = VideoClient::luma(\"your-api-key\");");
    println!();

    println!("Kling AI (Best Free Tier - 66/day!):");
    println!("   let client = VideoClient::kling(\"your-api-key\");");
    println!();

    println!("Haiper (Fast Generation):");
    println!("   let client = VideoClient::haiper(\"your-api-key\");");
    println!();

    println!("Stability AI (Image-to-Video Only):");
    println!("   let client = VideoClient::stability(\"your-api-key\");");
    println!();

    // ══════════════════════════════════════════════════════════════════════════
    //  SECTION 10: GENERATION FLOW
    // ══════════════════════════════════════════════════════════════════════════
    
    println!("🔄 ═══ GENERATION FLOW ═══");
    println!();

    println!("1. Simple Generation:");
    println!("   let client = VideoClient::kling(\"api-key\");");
    println!("   let request = VideoRequest::text_to_video(\"A sunset\");");
    println!("   let video = client.generate(request).await?;");
    println!("   println!(\"URL: {{}}\", video.url);");
    println!();

    println!("2. With Builder:");
    println!("   let request = VideoBuilder::new(\"A dragon flying\")");
    println!("       .duration(5.0)");
    println!("       .style(VideoStyle::Fantasy)");
    println!("       .build();");
    println!("   let video = client.generate(request).await?;");
    println!();

    println!("3. Status Checking:");
    println!("   let job = client.status(\"job-id\").await?;");
    println!("   match job.status {{");
    println!("       VideoStatus::Completed => println!(\"Done!\"),");
    println!("       VideoStatus::Processing => println!(\"Processing...\"),");
    println!("       _ => {{}},");
    println!("   }}");
    println!();

    // ══════════════════════════════════════════════════════════════════════════
    //  SECTION 11: RECOMMENDATIONS
    // ══════════════════════════════════════════════════════════════════════════
    
    println!("💡 ═══ RECOMMENDATIONS ═══");
    println!();

    println!("Best for Quality: Kling AI v1.5, Luma Dream Machine");
    println!("Best for Speed: Pika 2.0, Haiper v2");
    println!("Best Free Tier: Kling AI (66 credits/day = ~22 videos/day)");
    println!("Best Value: Pika ($8/month for 500 videos)");
    println!("Best for Social Media: Pika (vertical format support)");
    println!("Best for Products: Runway Gen-3 (professional quality)");
    println!("Best for Animation: Any (use style presets)");
    println!();

    println!("✅ Demo complete!");
}
