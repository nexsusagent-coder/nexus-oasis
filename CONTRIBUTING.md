# 🤝 Contributing to SENTIENT OS

Thank you for your interest in contributing to SENTIENT OS! This document provides guidelines and instructions for contributing.

## 📜 Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md).

## 🛠️ Development Setup

### Prerequisites

- **Rust 1.75+**: Install via [rustup](https://rustup.rs/)
- **Ollama**: For local LLM testing
- **Git**: Version control

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
cd SENTIENT_CORE

# Build
cargo build

# Run tests
cargo test

# Run clippy
cargo clippy -- -W clippy::all

# Format check
cargo fmt --all -- --check
```

## 📋 How to Contribute

### 1. Report Bugs

- Check [existing issues](https://github.com/nexsusagent-coder/SENTIENT_CORE/issues)
- Use the bug report template
- Include reproduction steps

### 2. Suggest Features

- Check [existing discussions](https://github.com/nexsusagent-coder/SENTIENT_CORE/discussions)
- Use the feature request template
- Explain the use case

### 3. Submit Pull Requests

1. **Fork** the repository
2. **Create a branch**: `git checkout -b feature/my-feature`
3. **Make changes** following our coding standards
4. **Test**: `cargo test`
5. **Lint**: `cargo clippy && cargo fmt`
6. **Commit**: Use conventional commits
7. **Push**: `git push origin feature/my-feature`
8. **Open PR**: Describe your changes

## 📐 Coding Standards

### Rust Style

```rust
// ✅ Good
pub fn process_data(input: &str) -> Result<String, Error> {
    let sanitized = sanitize(input)?;
    let processed = transform(&sanitized)?;
    Ok(processed)
}

// ❌ Bad
pub fn process_data(input: &str) -> String {
    let sanitized = sanitize(input).unwrap(); // Never use unwrap!
    transform(&sanitized).unwrap()
}
```

### Guidelines

1. **No `unwrap()`**: Use `.expect("descriptive message")` or `?`
2. **No panics**: Handle errors gracefully
3. **Document public APIs**: Use `///` doc comments
4. **Write tests**: Unit tests for all public functions
5. **Keep it simple**: Avoid premature optimization

### Code Structure

```
crates/
├── crate_name/
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       ├── lib.rs      # Public API
│       ├── error.rs    # Error types
│       ├── types.rs    # Public types
│       └── internal/   # Internal modules
```

## 🧪 Testing

### Run All Tests

```bash
cargo test --workspace --all-features
```

### Run Specific Tests

```bash
cargo test -p sentient_core
cargo test test_agent_creation
```

### Test Coverage

We aim for >80% coverage on critical paths.

## 📝 Commit Messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add speaker diarization
fix: resolve memory leak in agent loop
docs: update README with new examples
test: add tests for RAG pipeline
refactor: simplify orchestrator logic
chore: update dependencies
```

## 🔀 Pull Request Process

1. **Small PRs**: Keep changes focused
2. **Description**: Explain what and why
3. **Tests**: Include tests for new features
4. **Docs**: Update documentation
5. **CI**: Ensure all checks pass

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation

## Testing
- [ ] Unit tests added
- [ ] Integration tests added
- [ ] Manual testing done

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No new warnings
```

## 🏷️ Release Process

1. Update `CHANGELOG.md`
2. Update version in `Cargo.toml`
3. Create git tag: `git tag v4.1.0`
4. Push tag: `git push origin v4.1.0`
5. GitHub Actions handles the rest

## 🗺️ Project Structure

| Directory | Description |
|-----------|-------------|
| `crates/` | Rust crates |
| `examples/` | Example applications |
| `integrations/` | Third-party integrations |
| `dashboard/` | Web dashboard |
| `ide/` | IDE plugins |
| `docs/` | Documentation |

## 📚 Resources

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Async Book](https://rust-lang.github.io/async-book/)

## ❓ Questions?

- [GitHub Discussions](https://github.com/nexsusagent-coder/SENTIENT_CORE/discussions)
- [Discord](https://discord.gg/sentient)

## 📄 License

By contributing, you agree that your contributions will be licensed under the Apache 2.0 License.

---

Thank you for contributing to SENTIENT OS! 🧠
