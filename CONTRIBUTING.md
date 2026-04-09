# 🤝 Contributing to SENTIENT

First off, thank you for considering contributing to SENTIENT! It's people like you that make SENTIENT such a great tool.

## 📜 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Commit Guidelines](#commit-guidelines)

---

## 📜 Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

---

## 🚀 Getting Started

### Prerequisites

- **Rust** 1.75+ (`rustup install stable`)
- **Cargo** (comes with Rust)
- **Git**
- **Docker** (for integration tests)
- **Just** (task runner, optional but recommended)

### Fork and Clone

```bash
# Fork the repo on GitHub, then:
git clone https://github.com/YOUR_USERNAME/SENTIENT_CORE.git
cd SENTIENT_CORE
git remote add upstream https://github.com/nexsusagent-coder/SENTIENT_CORE.git
```

### Build

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run all tests including integration
cargo test --all-features
```

---

## 🛠️ How to Contribute

### 🐛 Reporting Bugs

Before creating a bug report, please check existing issues. When creating a bug report, include:

- **Title**: Clear, descriptive title
- **Description**: What happened vs. what you expected
- **Steps to Reproduce**: Minimal code example
- **Environment**: OS, Rust version, SENTIENT version
- **Logs**: Relevant log output with `RUST_LOG=debug`

### 💡 Suggesting Features

We love feature suggestions! Please include:

- **Use Case**: Why is this feature needed?
- **Proposed Solution**: How should it work?
- **Alternatives**: What alternatives have you considered?

### 📝 Improving Documentation

Documentation improvements are always welcome:

- Fix typos or unclear explanations
- Add code examples
- Translate documentation
- Write tutorials

### 🔧 Submitting Code

1. **Create a branch**: `git checkout -b feature/my-feature`
2. **Make changes**: Follow coding standards
3. **Add tests**: All new code needs tests
4. **Run tests**: `cargo test`
5. **Format code**: `cargo fmt`
6. **Lint**: `cargo clippy`
7. **Commit**: Follow commit guidelines
8. **Push**: `git push origin feature/my-feature`
9. **Open PR**: Fill out PR template

---

## 🏗️ Development Setup

### Project Structure

```
SENTIENT_CORE/
├── crates/                 # All crates (50+)
│   ├── sentient_core/      # Core types and traits
│   ├── sentient_cli/       # CLI application
│   ├── sentient_channels/  # Channel integrations
│   ├── sentient_voice/     # Voice processing
│   ├── sentient_memory/    # Memory management
│   └── ...
├── apps/                   # Native applications
│   ├── desktop/            # Tauri desktop app
│   └── mobile/             # iOS + Android
├── docs/                   # Documentation
├── npm/                    # npm package
└── tests/                  # Integration tests
```

### Running Tests

```bash
# Unit tests
cargo test

# Specific crate
cargo test -p sentient_channels

# With coverage
cargo tarpaulin --out Html

# Integration tests
cargo test --test integration

# Doc tests
cargo test --doc
```

### Debugging

```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Specific module
RUST_LOG=sentient_channels=debug cargo run

# Trace all
RUST_LOG=trace cargo run
```

---

## 📋 Pull Request Process

1. **Update Documentation**: Add/update docs in `docs/`
2. **Add Tests**: Maintain or improve test coverage
3. **Update CHANGELOG**: Add entry to CHANGELOG.md
4. **PR Title**: Follow conventional commits format
5. **Link Issues**: Link related issues
6. **Request Review**: Request review from maintainers

### PR Checklist

- [ ] Code compiles without errors
- [ ] All tests pass
- [ ] New code has tests
- [ ] Documentation updated
- [ ] CHANGELOG updated
- [ ] Commit messages follow guidelines
- [ ] PR title follows format

---

## 📏 Coding Standards

### Rust Style

```bash
# Format code
cargo fmt

# Check with clippy
cargo clippy -- -D warnings
```

### Code Organization

```rust
// Good: Module structure
mod channels {
    pub mod telegram;
    pub mod discord;
    pub mod slack;
}

// Good: Public API
pub struct Channel {
    // Private fields
    inner: ChannelInner,
}

impl Channel {
    /// Creates a new channel.
    pub fn new(config: Config) -> Result<Self> {
        // ...
    }
}
```

### Error Handling

```rust
// Use thiserror for custom errors
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChannelError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    
    #[error("Authentication error")]
    AuthError(#[from] AuthError),
}
```

### Documentation

```rust
/// Sends a message to the channel.
///
/// # Arguments
///
/// * `message` - The message to send
///
/// # Returns
///
/// Returns the message ID on success.
///
/// # Errors
///
/// Returns an error if the message fails to send.
///
/// # Example
///
/// ```
/// let channel = Channel::new(config)?;
/// let id = channel.send("Hello!").await?;
/// ```
pub async fn send(&self, message: &str) -> Result<MessageId> {
    // ...
}
```

---

## 📝 Commit Guidelines

We follow [Conventional Commits](https://www.conventionalcommits.org/):

### Format

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types

| Type | Description |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `style` | Code style (formatting, etc.) |
| `refactor` | Code refactoring |
| `perf` | Performance improvement |
| `test` | Adding/updating tests |
| `chore` | Maintenance tasks |
| `ci` | CI/CD changes |

### Examples

```bash
# Feature
feat(channels): add WeChat integration

# Bug fix
fix(voice): resolve wake word detection delay

# Documentation
docs(api): update channel configuration docs

# Breaking change
feat(core)!: change Agent trait interface

BREAKING CHANGE: The Agent trait now requires async methods.
```

---

## 🏷️ Labels

| Label | Description |
|-------|-------------|
| `good first issue` | Good for newcomers |
| `help wanted` | Extra attention needed |
| `bug` | Something isn't working |
| `enhancement` | New feature or request |
| `documentation` | Improvements to docs |
| `performance` | Performance related |
| `security` | Security related |

---

## 🙏 Recognition

Contributors are recognized in:

- **README.md**: All contributors listed
- **CHANGELOG.md**: Significant contributions noted
- **Release Notes**: Major contributions highlighted

---

## ❓ Questions?

- **GitHub Discussions**: For general questions
- **GitHub Issues**: For bugs and features
- **Discord**: [Join our server](https://discord.gg/sentient)

---

Thank you for contributing! 🎉
