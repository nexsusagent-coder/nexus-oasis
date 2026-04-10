# Multi-Agent Example

Multiple specialized agents working together on complex tasks.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Orchestrator                             │
│                   (Content Team)                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌───────────┐    ┌───────────┐    ┌───────────┐           │
│  │ Researcher│───▶│  Writer   │───▶│  Editor   │           │
│  │           │    │           │    │           │           │
│  │ Gathers   │    │ Creates   │    │ Reviews   │           │
│  │ info      │    │ content   │    │ & improves│           │
│  └───────────┘    └───────────┘    └───────────┘           │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Prerequisites

```bash
# Install Ollama
curl -fsSL https://ollama.com/install.sh | sh
ollama serve &
ollama pull llama3.2:3b
```

## Run

```bash
cd examples/multi-agent
cargo run
```

## Expected Output

```
🧠 SENTIENT OS - Multi-Agent Example
═════════════════════════════════════

📡 Provider: Ollama
🤖 Model: llama3.2:3b

👤 Creating specialized agents...

✅ Created 3 agents:
   📚 Researcher - Gathers information
   ✍️  Writer - Creates content
   🔍 Editor - Reviews and improves

📋 Task: Write a short article about the benefits of Rust

─────────────────────────────────────────────────────────────────────

📚 Researcher: Gathering information...

📊 Research Summary:
Rust is a systems programming language focused on:
1. Memory safety without garbage collection
2. Zero-cost abstractions
3. Fearless concurrency
...

─────────────────────────────────────────────────────────────────────

✍️  Writer: Creating content...

📝 Draft Article:
# The Benefits of Rust Programming Language

Rust has emerged as one of the most loved programming languages...
...

─────────────────────────────────────────────────────────────────────

🔍 Editor: Reviewing and improving...

✨ Final Article:
# The Benefits of Rust Programming Language

*An overview of why developers love Rust*

## Memory Safety Without Garbage Collection
...

─────────────────────────────────────────────────────────────────────

═══════════════════════════════════════
✅ Multi-Agent workflow complete!
   Research → Write → Edit
```

## Key Concepts

### 1. Specialized Agents

Each agent has a specific role:
- **Researcher** - Gathers and summarizes information
- **Writer** - Creates content from research
- **Editor** - Reviews and improves output

### 2. Orchestrator

Coordinates agents and manages workflow:
- Assigns tasks to appropriate agents
- Passes results between agents
- Maintains conversation context

### 3. Pipeline Pattern

Sequential execution:
```
Research → Write → Edit
```

## Extensions

### Add More Agents

```rust
let fact_checker = Agent::new(AgentConfig {
    name: "fact-checker".into(),
    system_prompt: Some("Verify factual accuracy...".into()),
    ..
});

orchestrator.add_agent(AgentRole::Worker, fact_checker);
```

### Parallel Execution

```rust
// Research multiple topics in parallel
let topics = vec!["Rust performance", "Rust safety", "Rust concurrency"];
let results = orchestrator.parallel("researcher", topics).await?;
```

## Next Steps

- [Production Example](../production/) - Real-world application
