// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Web Search Example
// ═══════════════════════════════════════════════════════════════════════════════

use sentient_search::{WebSearch, SearchOptions, SearchDepth};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 SENTIENT OS - Web Search Example");
    println!("═══════════════════════════════════════\n");

    // Option 1: Tavily (AI-optimized, requires API key)
    // Get free API key at: https://tavily.com
    // let search = WebSearch::tavily("tvly-xxxxx");

    // Option 2: Brave Search (Privacy-focused, requires API key)
    // Get free API key at: https://brave.com/search/api
    // let search = WebSearch::brave("xxxxx");

    // Option 3: DuckDuckGo (Free, no API key required)
    let search = WebSearch::duckduckgo();

    println!("📡 Using DuckDuckGo (free, no API key)\n");

    // Simple search
    println!("🔎 Searching for: \"Rust programming language\"\n");
    let response = search.search("Rust programming language").await?;

    println!("Found {} results:\n", response.results.len());
    
    for (i, result) in response.results.iter().enumerate() {
        println!("{}. {}", i + 1, result.title);
        println!("   URL: {}", result.url);
        println!("   {}", result.snippet);
        println!();
    }

    // Search with options
    println!("═══════════════════════════════════════");
    println!("🔎 Advanced search with options...\n");
    
    let options = SearchOptions {
        max_results: 5,
        include_content: false,
        search_depth: SearchDepth::Basic,
        include_domains: vec!["github.com".to_string()],
        exclude_domains: vec![],
        time_range: None,
        country: None,
        language: None,
    };

    let response = search.search_with_options("sentient os rust", options).await?;
    
    println!("GitHub results for \"sentient os rust\":\n");
    for result in response.results.iter() {
        println!("• {}", result.title);
        println!("  {}", result.url);
    }

    // Get context for LLM
    println!("\n═══════════════════════════════════════");
    println!("📝 Context for LLM:\n");
    let context = response.to_context();
    println!("{}", context);

    println!("═══════════════════════════════════════");
    println!("✅ Search complete!");

    Ok(())
}
