// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - E2B Code Sandbox Example
// ═══════════════════════════════════════════════════════════════════════════════
//  Secure code execution using E2B's Firecracker microVMs
//  - Isolated environments
//  - Multiple languages
//  - AI-safe code execution
// ═══════════════════════════════════════════════════════════════════════════════

use sentient_sandbox::{
    SandboxBuilder, SandboxConfig, CodeSnippet,
    BuiltinTemplate, RunCommandRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔒 SENTIENT OS - E2B Code Sandbox Example");
    println!("═══════════════════════════════════════════════════════════\n");

    // === Configuration ===
    println!("📋 1. Configuration");
    println!("─────────────────────────────\n");

    // Option 1: From environment variable E2B_API_KEY
    // let config = SandboxConfig::from_env()?;

    // Option 2: With API key directly
    // let config = SandboxConfig::new("e2b_xxx");

    // Option 3: Builder pattern
    let config = SandboxConfig::new("e2b_test_key") // Replace with your key
        .with_template("python-3.11")
        .with_timeout(300);

    println!("✅ Config created");
    println!("Default template: {}", config.default_template);
    println!("Timeout: {} seconds\n", config.timeout_secs);

    // === Available Templates ===
    println!("📋 2. Available Templates");
    println!("─────────────────────────────\n");

    println!("Template ID           | Language    | Packages");
    println!("----------------------|-------------|-------------------------");
    
    for template in BuiltinTemplate::all() {
        let packages = template.packages().join(", ");
        let pkgs = if packages.is_empty() { "-".to_string() } else { packages };
        println!("{:21} | {:11} | {}", 
            template.id(),
            template.name(),
            pkgs
        );
    }
    println!();

    // === Create Sandbox ===
    println!("🏗️  3. Sandbox Creation");
    println!("─────────────────────────────\n");

    println!("Creating sandbox with Python 3.11 template...\n");

    // Note: Requires valid E2B_API_KEY
    // let sandbox = SandboxBuilder::new(config)
    //     .builtin_template(BuiltinTemplate::Python311)
    //     .timeout(300)
    //     .create()
    //     .await?;
    // 
    // println!("Sandbox ID: {}", sandbox.id());
    // println!("Template: {}", sandbox.template());

    println!("💡 Note: Requires valid E2B_API_KEY to create actual sandbox\n");

    // === Code Execution ===
    println!("💻 4. Code Execution");
    println!("─────────────────────────────\n");

    // Python example
    let python_code = r#"
import json
import sys

# Simple data processing
data = [1, 2, 3, 4, 5]
result = {
    "sum": sum(data),
    "avg": sum(data) / len(data),
    "max": max(data),
    "min": min(data)
}

print(json.dumps(result, indent=2))
"#;

    println!("Python Code:");
    println!("─────────────────────────────");
    println!("{}", python_code);
    println!("─────────────────────────────\n");

    // Execute
    // let result = sandbox.run_python(python_code).await?;
    // println!("Output: {}", result.output());
    // println!("Duration: {}ms", result.duration_ms);

    // JavaScript example
    let js_code = r#"
const data = [1, 2, 3, 4, 5];
const result = {
    sum: data.reduce((a, b) => a + b, 0),
    avg: data.reduce((a, b) => a + b, 0) / data.length,
    max: Math.max(...data),
    min: Math.min(...data)
};
console.log(JSON.stringify(result, null, 2));
"#;

    println!("JavaScript Code:");
    println!("─────────────────────────────");
    println!("{}", js_code);
    println!("─────────────────────────────\n");

    // === File Operations ===
    println!("📁 5. File Operations");
    println!("─────────────────────────────\n");

    println!("Available operations:");
    println!("  sandbox.write_file(\"test.py\", \"print('hello')\")");
    println!("  sandbox.read_file(\"test.py\")");
    println!("  sandbox.list_dir(\"/home\")");
    println!("  sandbox.delete_file(\"test.py\")\n");

    // === Terminal Commands ===
    println!("🖥️  6. Terminal Commands");
    println!("─────────────────────────────\n");

    println!("Available commands:");
    println!("  sandbox.shell(\"ls -la\")");
    println!("  sandbox.pip_install(\"numpy\")");
    println!("  sandbox.npm_install(\"lodash\")\n");

    // === AI Agent Use Case ===
    println!("🤖 7. AI Agent Use Cases");
    println!("─────────────────────────────\n");

    println!("E2B Sandbox enables AI agents to:");
    println!("  ✅ Run generated code safely");
    println!("  ✅ Execute data analysis");
    println!("  ✅ Install packages on demand");
    println!("  ✅ Process files");
    println!("  ✅ Run web scraping scripts");
    println!("  ✅ Execute system commands\n");

    // === Pricing ===
    println!("💰 8. Pricing");
    println!("─────────────────────────────\n");

    println!("E2B Pricing:");
    println!("  Free tier: 1,000 sandbox hours/month");
    println!("  Pro tier: $0.02/hour");
    println!("  Enterprise: Custom pricing\n");

    // === Summary ===
    println!("═══════════════════════════════════════════════════════════");
    println!("✅ Example complete!");
    println!("\nTo use E2B Sandbox:");
    println!("1. Sign up: https://e2b.dev");
    println!("2. Get API key: https://e2b.dev/dashboard");
    println!("3. Set environment: export E2B_API_KEY=your_key");
    println!("4. Run: cargo run --example code-sandbox");

    Ok(())
}
