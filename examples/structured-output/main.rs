// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Structured Output Example
// ═══════════════════════════════════════════════════════════════════════════════
//  Reliable structured outputs from LLMs using JSON Schema
//  - Schema generation
//  - Function calling
//  - Instructor-style extraction
// ═══════════════════════════════════════════════════════════════════════════════

use sentient_schema::{SchemaBuilder, FunctionDef, StructuredLLM};
use serde::{Deserialize, Serialize};
use sentient_schema::JsonSchema;

/// Example structured output
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct Person {
    name: String,
    age: u32,
    occupation: String,
    skills: Vec<String>,
}

/// Another example
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct WeatherInfo {
    location: String,
    temperature: f32,
    condition: String,
    humidity: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 SENTIENT OS - Structured Output Example");
    println!("════════════════════════════════════════════\n");

    // === Schema Generation ===
    println!("📋 1. Schema Generation");
    println!("─────────────────────────────\n");

    // Static schema from derive
    let person_schema = sentient_schema::Schema::from_type::<Person>();
    println!("Person Schema (derive):");
    println!("{}\n", serde_json::to_string_pretty(&person_schema.schema)?);

    // Dynamic schema with builder
    let dynamic_schema = SchemaBuilder::object("Product")
        .required_string("name", Some("Product name"))
        .number("price", Some("Price in USD"))
        .enum_values("category", &["electronics", "clothing", "food"], None)
        .array("tags", "string", None)
        .build();

    println!("Product Schema (builder):");
    println!("{}\n", serde_json::to_string_pretty(&dynamic_schema.schema)?);

    // === Function Calling ===
    println!("🔧 2. Function Calling");
    println!("─────────────────────────────\n");

    let weather_func = FunctionDef::new("get_weather", "Get current weather for a location")
        .string_param("location", "City name", true)
        .enum_param("unit", "Temperature unit", &["celsius", "fahrenheit"], false);

    println!("OpenAI Function Format:");
    println!("{}\n", serde_json::to_string_pretty(&weather_func.to_openai())?);

    println!("Anthropic Tool Format:");
    println!("{}\n", serde_json::to_string_pretty(&weather_func.to_anthropic())?);

    // === Structured Extraction (Local) ===
    println!("🤖 3. Structured Extraction");
    println!("─────────────────────────────\n");

    // Using Ollama (local, free)
    let llm = StructuredLLM::ollama(None);

    println!("Using Ollama (local, requires Ollama running)");
    println!("Model: llama3.2 (or configure with .with_model())\n");

    // Check if available
    if llm.is_available().await {
        println!("✅ Ollama is running!");

        // Extract structured data
        println!("\nExtracting person data from prompt...\n");

        // Note: This would work with a running Ollama instance
        // let person: Person = llm.extract_with_retry(
        //     "Extract person info: John is a 30 year old software engineer who knows Rust and Python",
        //     3
        // ).await?;
    } else {
        println!("⚠️  Ollama not running. Start with: ollama serve");
        println!("   Or use OpenAI/Anthropic with API keys:\n");
        println!("   let llm = StructuredLLM::openai(\"sk-...\");");
        println!("   let llm = StructuredLLM::anthropic(\"sk-ant-...\");");
    }

    // === Validation Example ===
    println!("\n✅ 4. Schema Validation");
    println!("─────────────────────────────\n");

    use sentient_schema::extractor::SchemaValidator;

    let schema = sentient_schema::Schema::from_type::<Person>().schema;
    let valid_data = serde_json::json!({
        "name": "Alice",
        "age": 28,
        "occupation": "Engineer",
        "skills": ["Rust", "Python"]
    });

    let invalid_data = serde_json::json!({
        "age": "not a number"
    });

    println!("Valid data: {:?}",
        SchemaValidator::validate(&valid_data, &schema));
    println!("Invalid data: {:?}",
        SchemaValidator::validate(&invalid_data, &schema));

    println!("\n════════════════════════════════════════════");
    println!("✅ Example complete!");

    Ok(())
}
