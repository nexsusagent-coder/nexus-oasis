# 🧠 Contributing to SENTIENT OS

Thank you for your interest in contributing to SENTIENT OS - The Operating System That Thinks!

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Coding Standards](#coding-standards)
- [Commit Messages](#commit-messages)
- [Pull Requests](#pull-requests)

---

## Code of Conduct

Be respectful, inclusive, and constructive. We welcome contributions from everyone.

---

## Getting Started

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/sentient-os.git
   cd sentient-os
   ```
3. Run setup:
   ```bash
   ./setup.sh
   ```

---

## Development Setup

### Prerequisites

- Rust 1.75+ (`rustup default stable`)
- SQLite 3
- Docker (optional)
- Python 3.10+ (optional, for integrations)

### Build

```bash
# Debug build
make build-dev

# Release build
make build

# Run tests
make test
```

---

## Making Changes

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Your Changes

- Follow the [coding standards](#coding-standards)
- Add tests for new functionality
- Update documentation

### 3. Test Your Changes

```bash
# Run tests
make test

# Run clippy
make clippy

# Format code
make fmt
```

---

## Coding Standards

### Rust

- Use `rustfmt` for formatting
- Follow Clippy recommendations
- Document public APIs with `///` doc comments
- Use `Result<T, E>` for error handling
- Prefer `?` operator over `unwrap()`

### File Headers

All Rust files should start with:

```rust
//! ═════════════════════════════════════════════════════════════════
//!  MODULE NAME - Brief Description
//! ═════════════════════════════════════════════════════════════════
```

### Module Structure

```
crates/sentient_module/
├── Cargo.toml
├── src/
│   ├── lib.rs        # Public API
│   ├── module.rs     # Implementation
│   └── tests.rs      # Tests
└── README.md         # Module docs
```

---

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): description

[optional body]

[optional footer]
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance

### Examples

```
feat(vgate): add retry logic for failed requests

fix(memory): resolve race condition in cache

docs(readme): update installation instructions
```

---

## Pull Requests

### Checklist

- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] New code has tests
- [ ] Documentation updated
- [ ] Commit messages follow convention
- [ ] Branch is up-to-date with main

### Process

1. Push your branch
2. Open a Pull Request
3. Wait for review
4. Address feedback
5. Merge when approved

---

## 🐛 Reporting Bugs

Open an issue with:

- Clear description
- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment (OS, Rust version)

---

## 💡 Feature Requests

Open an issue with:

- Clear description
- Use case
- Proposed solution (optional)

---

## 📚 Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [SENTIENT Knowledge Base](./knowledge_base/)

---

<div align="center">

**Thank you for contributing to SENTIENT OS! 🧠**

</div>
