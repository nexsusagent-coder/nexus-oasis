//! Custom Skill Example
//!
//! Demonstrates how to create and register custom skills for SENTIENT.
//!
//! # Skills
//! - WeatherSkill: Get weather information
//! - CalculatorSkill: Perform calculations
//! - WebSearchSkill: Search the web

use sentient_core::{Agent, AgentConfig, Message, LlmProvider};
use sentient_skills::{Skill, SkillContext, SkillResult, SkillRegistry};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::env;

/// Weather Skill - Gets weather for a location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherSkill {
    pub name: String,
    pub description: String,
}

impl Default for WeatherSkill {
    fn default() -> Self {
        Self {
            name: "weather".to_string(),
            description: "Get current weather for a location. Usage: weather <city>".to_string(),
        }
    }
}

#[async_trait]
impl Skill for WeatherSkill {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn parameters(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "City name or coordinates"
                }
            },
            "required": ["location"]
        }))
    }

    async fn execute(&self, ctx: &SkillContext, args: serde_json::Value) -> anyhow::Result<SkillResult> {
        let location: String = args
            .get("location")
            .and_then(|v| v.as_str())
            .unwrap_or("Istanbul")
            .to_string();

        println!("🌤️ Fetching weather for: {}", location);

        // In production, call a real weather API
        let weather = serde_json::json!({
            "location": location,
            "temperature": 22,
            "condition": "Partly Cloudy",
            "humidity": 65,
            "wind_speed": 12,
            "unit": "celsius"
        });

        Ok(SkillResult::success(weather))
    }
}

/// Calculator Skill - Performs mathematical calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculatorSkill {
    pub name: String,
    pub description: String,
}

impl Default for CalculatorSkill {
    fn default() -> Self {
        Self {
            name: "calculator".to_string(),
            description: "Perform mathematical calculations. Usage: calc <expression>".to_string(),
        }
    }
}

#[async_trait]
impl Skill for CalculatorSkill {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn parameters(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "type": "object",
            "properties": {
                "expression": {
                    "type": "string",
                    "description": "Mathematical expression to evaluate"
                }
            },
            "required": ["expression"]
        }))
    }

    async fn execute(&self, ctx: &SkillContext, args: serde_json::Value) -> anyhow::Result<SkillResult> {
        let expression: String = args
            .get("expression")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("Missing expression"))?;

        println!("🧮 Calculating: {}", expression);

        // Simple calculator - in production use a proper expression parser
        let result = evaluate_expression(&expression)?;

        Ok(SkillResult::success(serde_json::json!({
            "expression": expression,
            "result": result
        })))
    }
}

/// Simple expression evaluator
fn evaluate_expression(expr: &str) -> anyhow::Result<f64> {
    // Very basic - only handles simple arithmetic
    // In production, use evalexpr or similar crate
    let expr = expr.replace(" ", "");
    
    if let Some(pos) = expr.find('+') {
        let a: f64 = expr[..pos].parse()?;
        let b: f64 = expr[pos+1..].parse()?;
        return Ok(a + b);
    }
    if let Some(pos) = expr.find('-') {
        let a: f64 = expr[..pos].parse()?;
        let b: f64 = expr[pos+1..].parse()?;
        return Ok(a - b);
    }
    if let Some(pos) = expr.find('*') {
        let a: f64 = expr[..pos].parse()?;
        let b: f64 = expr[pos+1..].parse()?;
        return Ok(a * b);
    }
    if let Some(pos) = expr.find('/') {
        let a: f64 = expr[..pos].parse()?;
        let b: f64 = expr[pos+1..].parse()?;
        return Ok(a / b);
    }
    
    expr.parse().map_err(|e| anyhow::anyhow!("Invalid expression: {}", e))
}

/// Web Search Skill - Simulates web search
#[derive(Debug, Clone, Default)]
pub struct WebSearchSkill {
    pub name: String,
    pub description: String,
}

#[async_trait]
impl Skill for WebSearchSkill {
    fn name(&self) -> &str { &self.name }
    fn description(&self) -> &str { &self.description }

    fn parameters(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Search query"
                }
            },
            "required": ["query"]
        }))
    }

    async fn execute(&self, ctx: &SkillContext, args: serde_json::Value) -> anyhow::Result<SkillResult> {
        let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
        
        println!("🔍 Searching: {}", query);

        // Simulated search results
        let results = vec![
            format!("Result 1 for: {}", query),
            format!("Result 2 for: {}", query),
            format!("Result 3 for: {}", query),
        ];

        Ok(SkillResult::success(serde_json::json!({
            "query": query,
            "results": results,
            "total": results.len()
        })))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    println!("🛠️ SENTIENT Custom Skill Example\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let api_key = env::var("OPENAI_API_KEY")
        .expect("Please set OPENAI_API_KEY");

    // Create skill registry
    println!("📦 Registering skills...");
    let mut registry = SkillRegistry::new();

    registry.register(WeatherSkill::default())?;
    registry.register(CalculatorSkill::default())?;
    registry.register(WebSearchSkill {
        name: "web_search".to_string(),
        description: "Search the web for information".to_string(),
    })?;

    println!("✅ Registered {} skills:\n", registry.len());
    for (name, skill) in registry.list() {
        println!("  • {}: {}", name, skill.description());
    }
    println!();

    // Create agent with skills
    let config = AgentConfig {
        name: "SkillfulAgent".to_string(),
        llm_provider: LlmProvider::OpenAI,
        llm_model: "gpt-4o".to_string(),
        api_key,
        system_prompt: Some(
            "You are an AI assistant with access to tools. \
             Use the appropriate tool when needed. \
             Available tools: weather, calculator, web_search".to_string()
        ),
        skills: Some(registry.clone()),
        ..Default::default()
    };

    let mut agent = Agent::new(config).await?;

    // Test the skills
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Weather skill
    println!("👤 User: What's the weather in Istanbul?");
    let response = agent
        .send(Message::user("What's the weather in Istanbul?"))
        .await?;
    println!("🤖 Agent: {}\n", response.content);

    // Calculator skill
    println!("👤 User: Calculate 123 + 456");
    let response = agent
        .send(Message::user("Calculate 123 + 456"))
        .await?;
    println!("🤖 Agent: {}\n", response.content);

    // Web search skill
    println!("👤 User: Search for Rust programming language");
    let response = agent
        .send(Message::user("Search for Rust programming language"))
        .await?;
    println!("🤖 Agent: {}\n", response.content);

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("\n✅ Custom skill example completed!");
    println!("\n💡 Tip: Implement the Skill trait to create your own skills!");

    Ok(())
}
