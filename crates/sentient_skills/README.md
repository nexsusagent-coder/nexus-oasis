# SENTIENT Skills - DeerFlow Integration

> DeerFlow 2.0'dan esinlenilmiş skill yönetim sistemi

## Özellikler

- 📚 **Skill Loader**: DeerFlow SKILL.md formatını parse eder
- 🎯 **Skill Manager**: Skill registry ve eşleştirme
- 🛡️ **Guardrails Middleware**: Tool call güvenlik katmanı
- 🔄 **Subagent Executor**: Paralel agent execution
- ⚡ **Skill Executor**: Skill execution engine

## Mimari

```
┌─────────────────────────────────────────────────────────────────┐
│                    SENTIENT SKILLS SYSTEM                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   ┌─────────────┐    ┌─────────────┐    ┌─────────────┐       │
│   │   Loader    │───▶│   Manager   │───▶│  Executor   │       │
│   │ (SKILL.md)  │    │  (Registry) │    │  (Engine)   │       │
│   └─────────────┘    └─────────────┘    └─────────────┘       │
│          │                  │                  │                │
│          ▼                  ▼                  ▼                │
│   ┌─────────────────────────────────────────────────────┐     │
│   │              Guardrails Middleware                   │     │
│   │           (Tool Call Security Layer)                │     │
│   └─────────────────────────────────────────────────────┘     │
│          │                                                      │
│          ▼                                                      │
│   ┌─────────────────────────────────────────────────────┐     │
│   │              Subagent Executor                       │     │
│   │         (Parallel Agent Execution)                  │     │
│   └─────────────────────────────────────────────────────┘     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## DeerFlow Skills (Entegre Edildi)

| Skill | Kategori | Açıklama |
|-------|----------|----------|
| `deep-research` | Research | Derin web araştırma metodolojisi |
| `ppt-generation` | Content | Sunum oluşturma |
| `podcast-generation` | Media | Podcast üretimi |
| `video-generation` | Media | Video oluşturma |
| `image-generation` | Media | Görsel üretimi |
| `data-analysis` | Analysis | Veri analizi |
| `frontend-design` | Design | Frontend tasarım |
| `github-deep-research` | Research | GitHub projesi analizi |
| `skill-creator` | Utility | Otomatik skill oluşturucu |

## Kullanım

### Skill Loader

```rust
use sentient_skills::{SkillLoader, Skill};

// Loader oluştur
let mut loader = SkillLoader::new()
    .add_skill_dir("./skills")
    .with_default_dirs();

// Tüm skill'leri yükle
let count = loader.load_all()?;
println!("Loaded {} skills", count);

// Skill'leri al
for skill in loader.get_skills() {
    println!("{}: {}", skill.metadata.name, skill.metadata.description);
}
```

### Skill Manager

```rust
use sentient_skills::SkillManager;

// Manager oluştur
let manager = SkillManager::new().with_defaults();

// Skill ara
let skills = manager.find_matching_skills("research AI trends");

// En iyi eşleşmeyi bul
if let Some(skill) = manager.find_best_match("research AI trends") {
    println!("Found skill: {}", skill.metadata.name);
}

// Kategori bazlı listele
let research_skills = manager.get_skills_by_category(SkillCategory::Research);
```

### Guardrails Middleware

```rust
use sentient_skills::{GuardrailMiddleware, GuardrailProvider, ToolCallRequest};

// Rule-based guardrail
let middleware = GuardrailMiddleware::with_rules()
    .fail_closed(true);

// Tool call değerlendir
let request = ToolCallRequest {
    tool_name: "web_search".to_string(),
    args: HashMap::new(),
    agent_id: None,
    timestamp: chrono::Utc::now(),
};

let decision = middleware.evaluate(&request);
if decision.allow {
    // Tool call'u gerçekleştir
} else {
    // Red mesajı göster
    println!("{}", GuardrailMiddleware::build_denied_message(&request, &decision));
}
```

### Subagent Executor

```rust
use sentient_skills::{SubagentExecutor, SubagentConfig, SubagentTask};

// Executor oluştur
let executor = SubagentExecutor::new()
    .with_max_parallel(5);

// Task oluştur
let task = SubagentTask::new(
    SubagentConfig::new("research-agent")
        .with_timeout(60),
    "Research AI trends in 2024"
);

// Çalıştır
let result = executor.execute(task).await;
println!("Status: {:?}", result.status);

// Paralel execution
let tasks = vec![
    SubagentTask::new(SubagentConfig::new("agent-1"), "Task 1"),
    SubagentTask::new(SubagentConfig::new("agent-2"), "Task 2"),
];
let results = executor.execute_parallel(tasks).await;
```

### Skill Executor

```rust
use sentient_skills::{SkillExecutor, ExecutionContext};

// Executor oluştur
let executor = SkillExecutor::new();

// Context oluştur
let ctx = ExecutionContext::new("research AI trends in 2024")
    .with_session("session-123")
    .with_user("user-456");

// Skill çalıştır
let result = executor.execute(ctx).await;

if result.success {
    println!("Output: {}", result.output);
} else {
    println!("Error: {:?}", result.error);
}
```

## Skill Format (DeerFlow Compatible)

```markdown
---
name: deep-research
description: Deep web research skill
version: 1.0.0
category: research
triggers:
  - type: keyword
    pattern: research
    priority: 1
required_tools:
  - web_search
  - web_fetch
tags:
  - research
  - web
timeout_secs: 300
---

# Deep Research Skill

## Overview
This skill provides systematic methodology for web research.

## When to Use
- User asks "what is X"
- User wants comprehensive research

## Methodology
### Phase 1: Broad Exploration
...
```

## Modüller

| Modül | Satır | Açıklama |
|-------|:-----:|----------|
| `types.rs` | ~250 | Skill veri yapıları |
| `loader.rs` | ~300 | SKILL.md parser |
| `manager.rs` | ~200 | Skill registry |
| `guardrails.rs` | ~350 | Güvenlik katmanı |
| `subagent.rs` | ~400 | Paralel execution |
| `executor.rs` | ~280 | Execution engine |

## Kaynak

- DeerFlow 2.0: https://github.com/bytedance/deer-flow
- MIT License

---

*SENTIENT OS - The Operating System That Thinks*
