# 🧪 Testing Strategy

This document outlines SENTIENT's comprehensive testing strategy.

## 📊 Coverage Goals

| Type | Current | Target |
|------|---------|--------|
| Unit Tests | 40% | **80%** |
| Integration Tests | 20% | **70%** |
| E2E Tests | 10% | **50%** |

## 🏗️ Test Architecture

```
tests/
├── unit/              # Unit tests (fast, isolated)
│   ├── core/
│   ├── channels/
│   ├── voice/
│   └── skills/
├── integration/       # Integration tests (services)
│   ├── api/
│   ├── database/
│   └── channels/
├── e2e/              # End-to-end tests (full system)
│   ├── scenarios/
│   └── fixtures/
└── property/         # Property-based tests
    └── generators/
```

## 🚀 Running Tests

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests
cargo test --test '*'

# With coverage
cargo tarpaulin --out Html

# Specific test
cargo test test_agent_message
```

## 🔬 Test Categories

### 1. Unit Tests

Fast, isolated tests for individual components.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_creation() {
        let config = AgentConfig::default();
        let agent = Agent::new(config);
        assert!(agent.is_ok());
    }

    #[test]
    fn test_message_serialization() {
        let msg = Message::user("Hello");
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("Hello"));
    }
}
```

### 2. Integration Tests

Tests with external services (mocked or real).

```rust
#[tokio::test]
async fn test_openai_integration() {
    let mut server = mockito::Server::new();
    let mock = server.mock("POST", "/v1/chat/completions")
        .with_status(200)
        .with_body(r#"{"choices": [{"message": {"content": "Hello"}}]}"#)
        .create();

    let agent = Agent::new(config).await.unwrap();
    let response = agent.send(Message::user("Hi")).await.unwrap();

    mock.assert();
    assert_eq!(response.content, "Hello");
}
```

### 3. Property-Based Tests

Automatically generate test cases.

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_message_doesnt_crash(input in ".*") {
        let msg = Message::user(&input);
        let result = process_message(msg);
        prop_assert!(result.is_ok());
    }

    #[test]
    fn test_json_roundtrip(msg in any::<Message>()) {
        let json = serde_json::to_string(&msg).unwrap();
        let decoded: Message = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(msg, decoded);
    }
}
```

### 4. Load Tests

Performance and stress testing.

```rust
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_concurrent_requests() {
    let agent = Arc::new(Agent::new(config).await.unwrap());

    let handles: Vec<_> = (0..1000)
        .map(|i| {
            let agent = agent.clone();
            tokio::spawn(async move {
                agent.send(Message::user(&format!("Test {}", i))).await
            })
        })
        .collect();

    let results: Vec<_> = join_all(handles).await;
    assert!(results.iter().all(|r| r.is_ok()));
}
```

## 📈 Coverage Tracking

### Generate Coverage Report

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate report
cargo tarpaulin --out Html --out Lcov

# View report
open tarpaulin-report.html
```

### Coverage Badges

[![Coverage Status](https://coveralls.io/repos/github/nexsusagent-coder/SENTIENT_CORE/badge.svg)](https://coveralls.io/github/nexsusagent-coder/SENTIENT_CORE)

## 🔧 Test Utilities

### Test Fixtures

```rust
// tests/fixtures/mod.rs
pub fn create_test_agent() -> Agent {
    Agent::new(AgentConfig {
        api_key: "test-key".to_string(),
        ..Default::default()
    })
}

pub fn mock_llm_response(content: &str) -> Mock {
    server.mock("POST", "/v1/chat/completions")
        .with_body(json!({
            "choices": [{
                "message": {"content": content}
            }]
        }).to_string())
}
```

### Test Builders

```rust
pub struct MessageBuilder {
    role: MessageRole,
    content: String,
}

impl MessageBuilder {
    pub fn user(content: &str) -> Self {
        Self {
            role: MessageRole::User,
            content: content.to_string(),
        }
    }

    pub fn build(self) -> Message {
        Message {
            role: self.role,
            content: self.content,
        }
    }
}
```

## 🎯 Test Matrix

| OS | Rust Version | Status |
|----|--------------|--------|
| Ubuntu | stable | ✅ |
| Ubuntu | beta | ✅ |
| Ubuntu | nightly | ⚠️ |
| macOS | stable | ✅ |
| Windows | stable | ✅ |

## 📊 Current Coverage by Crate

| Crate | Lines | Coverage |
|-------|-------|----------|
| sentient_core | 5,000 | 75% |
| sentient_channels | 8,000 | 60% |
| sentient_voice | 3,000 | 55% |
| sentient_skills | 2,000 | 50% |
| sentient_enterprise | 4,000 | 70% |

---

*Run `cargo test --all` to execute all tests*
