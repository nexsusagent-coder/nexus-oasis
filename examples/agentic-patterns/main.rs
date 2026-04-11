// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Agentic Patterns Example
// ═══════════════════════════════════════════════════════════════════════════════
//  Reasoning patterns for AI agents
// ═══════════════════════════════════════════════════════════════════════════════

use sentient_patterns::{
    PatternType, ReasoningStep, Action, ReasoningTrace,
    patterns::{ReActPattern, ChainOfThoughtPattern, TreeOfThoughtsPattern, PlanAndExecutePattern, SelfReflectionPattern},
    traits::ReasoningPattern,
};

fn main() {
    println!("🧠 SENTIENT OS - Agentic Patterns Example");
    println!("═══════════════════════════════════════════════════════════\n");

    // === Available Patterns ===
    println!("📋 1. Available Patterns");
    println!("─────────────────────────────\n");

    println!("Pattern          | Description                        | Best For");
    println!("─────────────────|────────────────────────────────────|─────────────────");
    println!("ReAct            | Reason + Act interleaved           | Tool use tasks");
    println!("Chain of Thought | Step-by-step reasoning             | Math, logic");
    println!("Tree of Thoughts | Multiple reasoning paths           | Creative problems");
    println!("Plan-and-Execute | Plan first, then execute           | Complex tasks");
    println!("Self-Reflection  | Generate, critique, improve        | Quality critical\n");

    // === ReAct Pattern ===
    println!("🔄 2. ReAct Pattern (Reason + Act)");
    println!("─────────────────────────────\n");

    println!("Format:\n");
    println!("  Thought: I need to find information about X");
    println!("  Action: search[X]");
    println!("  Observation: [search result]");
    println!("  Thought: Now I can answer...");
    println!("  Answer: [final answer]\n");

    let react = ReActPattern::new()
        .with_max_iterations(10);

    println!("Created ReAct pattern:");
    println!("  Name: {}", react.name());
    println!("  Max iterations: 10\n");

    // === Chain of Thought ===
    println!("📝 3. Chain of Thought Pattern");
    println!("─────────────────────────────\n");

    println!("Example:\n");
    println!("  Question: What is 15 * 4 + 7?");
    println!("  Let's think step by step:");
    println!("  Step 1: Multiply 15 * 4 = 60");
    println!("  Step 2: Add 7 to 60 = 67");
    println!("  Answer: 67\n");

    let cot = ChainOfThoughtPattern::new()
        .without_examples()
        .with_max_steps(10);

    println!("Created CoT pattern:");
    println!("  Name: {}", cot.name());
    println!("  Include examples: false\n");

    // === Tree of Thoughts ===
    println!("🌳 4. Tree of Thoughts Pattern");
    println!("─────────────────────────────\n");

    println!("Structure:\n");
    println!("            Problem");
    println!("           /   |   \\");
    println!("        Thought Thought Thought");
    println!("        (0.8)   (0.6)  (0.3)");
    println!("           |");
    println!("        Solution\n");

    let tot = TreeOfThoughtsPattern::new()
        .with_branching(3)
        .with_max_depth(4)
        .with_threshold(0.7);

    println!("Created ToT pattern:");
    println!("  Branching factor: 3");
    println!("  Max depth: 4");
    println!("  Evaluation threshold: 0.7\n");

    // === Plan and Execute ===
    println!("📋 5. Plan-and-Execute Pattern");
    println!("─────────────────────────────\n");

    println!("Example:\n");
    println!("  Goal: Write a blog post about AI");
    println!("  Plan:");
    println!("    1. Research the topic");
    println!("    2. Create an outline");
    println!("    3. Write the introduction");
    println!("    4. Write the body");
    println!("    5. Write the conclusion");
    println!("    6. Edit and polish\n");

    let plan_exec = PlanAndExecutePattern::new()
        .with_max_replans(2);

    println!("Created Plan-Execute pattern:");
    println!("  Name: {}", plan_exec.name());
    println!("  Max replans: 2\n");

    // === Self-Reflection ===
    println!("🪞 6. Self-Reflection Pattern");
    println!("─────────────────────────────\n");

    println!("Cycle:\n");
    println!("  ┌──────────────────┐");
    println!("  │ Generate Answer  │");
    println!("  └────────┬─────────┘");
    println!("           ↓");
    println!("  ┌──────────────────┐");
    println!("  │   Self-Critique  │");
    println!("  └────────┬─────────┘");
    println!("           ↓");
    println!("  ┌──────────────────┐");
    println!("  │    Improve?      │");
    println!("  └────────┬─────────┘");
    println!("           │ No → Done");
    println!("           ↓ Yes");
    println!("      (loop back)\n");

    let reflection = SelfReflectionPattern::new()
        .with_max_iterations(3)
        .with_confidence_threshold(0.8);

    println!("Created Reflection pattern:");
    println!("  Name: {}", reflection.name());
    println!("  Max iterations: 3");
    println!("  Confidence threshold: 0.8\n");

    // === Reasoning Trace ===
    println!("📊 7. Reasoning Trace");
    println!("─────────────────────────────\n");

    let mut trace = ReasoningTrace::new(PatternType::ReAct);
    trace.add_step(ReasoningStep::thought(1, "I need to search for information"));
    trace.add_step(
        ReasoningStep::thought(2, "Found relevant data")
            .with_action(Action::search("Rust programming"))
            .with_observation("Rust is a systems programming language...")
    );
    trace.set_answer("Rust is a systems programming language focused on safety.");

    println!("Trace:");
    println!("  Pattern: {}", trace.pattern);
    println!("  Steps: {}", trace.step_count());
    for step in &trace.steps {
        println!("    Step {}: {}", step.step, step.thought);
    }
    println!("  Answer: {:?}\n", trace.answer);

    // === Use Cases ===
    println!("🎯 8. When to Use Each Pattern");
    println!("─────────────────────────────\n");

    println!("Task Type              | Recommended Pattern");
    println!("───────────────────────|────────────────────");
    println!("Multi-step tool use    | ReAct");
    println!("Math problems          | Chain of Thought");
    println!("Creative writing       | Tree of Thoughts");
    println!("Project planning       | Plan-and-Execute");
    println!("Quality-critical tasks | Self-Reflection");
    println!("Code generation        | Plan-and-Execute + Reflection\n");

    // === Summary ===
    println!("═══════════════════════════════════════════════════════════");
    println!("✅ Example complete!");
    println!("\nThese patterns enable AI agents to:");
    println!("  • Reason systematically");
    println!("  • Use tools effectively");
    println!("  • Explore multiple solutions");
    println!("  • Plan complex tasks");
    println!("  • Self-improve quality");
}
