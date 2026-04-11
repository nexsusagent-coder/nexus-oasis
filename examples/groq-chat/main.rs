// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Groq LPU Example
// ═══════════════════════════════════════════════════════════════════════════════
//  Ultra-fast inference using Groq's LPU (Language Processing Unit)
//  - 500+ tokens/second
//  - Cheaper than OpenAI
//  - OpenAI-compatible API
// ═══════════════════════════════════════════════════════════════════════════════

use sentient_groq::{GroqClient, GroqModel, GroqClientBuilder, ChatMessage, ChatRequest, Tool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ SENTIENT OS - Groq LPU Example");
    println!("══════════════════════════════════════════════════════\n");

    // === Setup ===
    println!("📋 1. Client Setup");
    println!("─────────────────────────────\n");

    // Option 1: From environment variable GROQ_API_KEY
    // let client = GroqClient::from_env()?;

    // Option 2: With API key directly
    // let client = GroqClient::with_key("gsk_xxx")?;

    // Option 3: Builder pattern
    let client = GroqClientBuilder::new()
        .api_key("gsk_test_key") // Replace with your key
        .model(GroqModel::Llama33_70B)
        .timeout(30)
        .build()?;

    println!("✅ Client created");
    println!("Default model: {}", client.default_model());
    println!("Context length: {} tokens\n", client.default_model().context_length());

    // === Available Models ===
    println!("📋 2. Available Models");
    println!("─────────────────────────────\n");

    println!("Model                    | Context  | Input/1M | Output/1M | Speed");
    println!("-------------------------|----------|----------|-----------|-------");
    
    for model in GroqModel::all() {
        let (input, output) = model.pricing();
        println!(
            "{:24} | {:>7} | ${:>6.2} | ${:>8.2} | {}",
            model.id(),
            format!("{}K", model.context_length() / 1000),
            input,
            output,
            if model.supports_vision() { "🚀 Fast+Vision" } else { "🚀 Fast" }
        );
    }
    println!();

    // === Pricing Example ===
    println!("💰 3. Cost Estimation");
    println!("─────────────────────────────\n");

    let input_tokens = 1000;
    let output_tokens = 500;
    let cost = GroqModel::Llama33_70B.estimate_cost(input_tokens, output_tokens);
    
    println!("For {} input + {} output tokens:", input_tokens, output_tokens);
    println!("  Cost: ${:.6}", cost);
    println!("  OpenAI GPT-4 equivalent: ${:.6}", 
        (input_tokens as f64 / 1_000_000.0 * 2.50) + (output_tokens as f64 / 1_000_000.0 * 10.00));
    println!("  Savings: {:.1}x cheaper!\n", 
        ((input_tokens as f64 / 1_000_000.0 * 2.50) + (output_tokens as f64 / 1_000_000.0 * 10.00)) / cost);

    // === Simple Chat ===
    println!("💬 4. Simple Chat");
    println!("─────────────────────────────\n");

    println!("Messages:");
    let messages = vec![
        ChatMessage::system("You are a helpful AI assistant."),
        ChatMessage::user("What is the speed of light?"),
    ];

    for msg in &messages {
        println!("  {:?}: {}", msg.role, msg.content);
    }
    println!();

    // Note: Requires valid API key
    // let response = client.chat_simple(messages).await?;
    // println!("Response: {}", response.content().unwrap_or_default());

    println!("💡 Note: Requires valid GROQ_API_KEY to make actual API calls\n");

    // === Function Calling ===
    println!("🔧 5. Function Calling");
    println!("─────────────────────────────\n");

    let weather_tool = Tool::function(
        "get_weather",
        "Get current weather for a location",
        serde_json::json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "City name"
                },
                "unit": {
                    "type": "string",
                    "enum": ["celsius", "fahrenheit"],
                    "description": "Temperature unit"
                }
            },
            "required": ["location"]
        })
    );

    let request = ChatRequest::new(GroqModel::Llama33_70B, vec![
        ChatMessage::user("What's the weather in Istanbul?"),
    ])
    .with_tools(vec![weather_tool])
    .with_temperature(0.0);

    println!("Request with tool:");
    println!("  Model: {}", request.model);
    println!("  Tools: {} function(s)", request.tools.as_ref().map(|t| t.len()).unwrap_or(0));
    println!();

    // === Streaming ===
    println!("🌊 6. Streaming");
    println!("─────────────────────────────\n");

    println!("Groq supports streaming for real-time responses:");
    println!("  - First token latency: ~100ms");
    println!("  - Throughput: 500+ tokens/sec");
    println!("  - Uses Server-Sent Events (SSE)\n");

    // === Speed Comparison ===
    println!("⚡ 7. Speed Comparison");
    println!("─────────────────────────────\n");

    println!("Provider       | Tokens/sec | First Token");
    println!("---------------|------------|-------------");
    println!("Groq LPU       | 500+       | ~100ms");
    println!("OpenAI GPT-4   | ~50        | ~500ms");
    println!("Claude 3       | ~60        | ~400ms");
    println!("Local (Ollama) | ~30*       | ~200ms");
    println!("\n* Depends on hardware\n");

    // === Summary ===
    println!("══════════════════════════════════════════════════════");
    println!("✅ Example complete!");
    println!("\nTo use Groq:");
    println!("1. Get API key: https://console.groq.com");
    println!("2. Set environment: export GROQ_API_KEY=your_key");
    println!("3. Run: cargo run --example groq-chat");

    Ok(())
}
