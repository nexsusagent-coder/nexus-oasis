//! LLM Hub Demo

use sentient_llm::{LlmHub, ChatRequest, Message, LlmHubBuilder, RoutingStrategy};

#[tokio::main]
async fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║           SENTIENT LLM Hub - 50+ Models, 13 Providers        ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Create hub with all configured providers
    let hub = LlmHub::from_env().unwrap();
    
    println!("📊 Configured providers: {}", hub.configured_count());
    println!("📊 Available models: {}\n", hub.models().len());

    // List all providers
    println!("🔧 PROVIDERS:");
    for provider in hub.providers() {
        let models_count = provider.models().len();
        println!("   • {} ({}) - {} models", provider.name(), provider.id(), models_count);
    }

    // List free tier models
    println!("\n💰 FREE TIER MODELS:");
    for model in sentient_llm::models::free_tier().iter().take(10) {
        println!("   • {} ({})", model.name, model.provider);
    }

    // Cost comparison
    println!("\n📈 COST COMPARISON (1K prompt + 500 completion):");
    let costs = hub.compare_cost(1000, 500);
    for (model, cost) in costs.iter().take(10) {
        println!("   • {}: ${:.6}", model, cost);
    }

    // Model recommendations
    println!("\n🎯 RECOMMENDATIONS:");
    println!("   • Cheapest: DeepSeek V3 ($0.00014/1K tokens)");
    println!("   • Fastest: Groq LPU inference");
    println!("   • Best Quality: GPT-4o, Claude 4");
    println!("   • Free: Ollama (local), Groq free tier");

    println!("\n✅ LLM Hub ready for use!");
}
