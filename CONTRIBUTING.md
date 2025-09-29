# Contributing to GLIN Chain

Thank you for your interest in contributing to GLIN Chain! We welcome contributions from the community and are grateful for any help you can provide.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)
- [Community](#community)

## 📜 Code of Conduct

This project adheres to a [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to conduct@glin.ai.

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Git
- A GitHub account
- Basic familiarity with Substrate

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
```bash
git clone https://github.com/YOUR-USERNAME/glin-chain.git
cd glin-chain
git remote add upstream https://github.com/glin-ai/glin-chain.git
```

## 🛠️ Development Setup

### Install Dependencies

```bash
# Ubuntu/Debian
sudo apt update && sudo apt install -y \
  git clang curl libssl-dev llvm libudev-dev make protobuf-compiler

# macOS
brew install protobuf

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build the Project

```bash
cargo build --release
```

### Run Tests

```bash
cargo test
```

## 🤝 How to Contribute

### Types of Contributions

We welcome many types of contributions:

- **Bug Fixes**: Found a bug? Please open an issue first, then submit a PR
- **Features**: New features should be discussed in an issue before implementation
- **Documentation**: Help improve our docs, README, or code comments
- **Tests**: Add missing tests or improve existing ones
- **Performance**: Optimize code for better performance
- **Refactoring**: Improve code structure and readability

### Finding Issues

Look for issues labeled:
- `good first issue` - Great for newcomers
- `help wanted` - We need help with these
- `documentation` - Documentation improvements
- `bug` - Known bugs to fix

## 🔄 Pull Request Process

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

### 2. Make Your Changes

- Write clear, concise commit messages
- Follow our [coding standards](#coding-standards)
- Add tests for new functionality
- Update documentation as needed

### 3. Commit Your Changes

```bash
git add .
git commit -m "feat: add new validation mechanism"
```

We use conventional commits:
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `test:` Test additions or changes
- `refactor:` Code refactoring
- `perf:` Performance improvements
- `chore:` Maintenance tasks

### 4. Push to Your Fork

```bash
git push origin feature/your-feature-name
```

### 5. Create a Pull Request

1. Go to the original repository on GitHub
2. Click "New Pull Request"
3. Select your fork and branch
4. Fill out the PR template
5. Submit for review

### PR Requirements

- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Documentation is updated
- [ ] Commit messages follow conventions
- [ ] PR description explains changes

## 📝 Coding Standards

### Rust Style

- Use `rustfmt` for formatting:
```bash
cargo fmt --all
```

- Use `clippy` for linting:
```bash
cargo clippy -- -D warnings
```

### Code Organization

```rust
// Good: Clear module organization
pub mod validation {
    pub fn validate_gradient() { ... }
}

// Good: Clear function names
pub fn calculate_reward_distribution() { ... }

// Good: Comprehensive documentation
/// Calculates the reward for a provider based on their contribution.
///
/// # Arguments
/// * `gradients` - Number of gradients submitted
/// * `quality_score` - Quality score from 0 to 1000
///
/// # Returns
/// The calculated reward amount in GLIN tokens
pub fn calculate_provider_reward(gradients: u64, quality_score: u32) -> Balance {
    // Implementation
}
```

### Best Practices

- Write descriptive variable names
- Keep functions small and focused
- Add comments for complex logic
- Use proper error handling
- Avoid unwrap() in production code

## 🧪 Testing Guidelines

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reward_calculation() {
        let reward = calculate_provider_reward(100, 950);
        assert_eq!(reward, expected_amount);
    }
}
```

### Integration Tests

Place integration tests in `tests/` directory:
```rust
// tests/integration_test.rs
#[test]
fn test_full_task_lifecycle() {
    // Test complete task flow
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific pallet tests
cargo test -p pallet-task-registry

# Run with output
cargo test -- --nocapture
```

## 📚 Documentation

### Code Documentation

- Document all public APIs
- Use rustdoc format
- Include examples where helpful

```rust
/// Creates a new federated learning task.
///
/// # Example
/// ```
/// let task = create_task(
///     b"Image Classification",
///     ModelType::ResNet,
///     1000_000_000_000,
/// )?;
/// ```
pub fn create_task(...) -> Result<...> { ... }
```

### Updating Documentation

- Update README.md for user-facing changes
- Update inline documentation
- Add to changelog for significant changes

## 💬 Community

### Communication Channels

- **Discord**: [Join our server](https://discord.gg/glin-ai)
- **GitHub Discussions**: For feature requests and questions
- **Twitter**: [@glin_ai](https://twitter.com/glin_ai)

### Getting Help

- Check existing issues and discussions
- Ask in Discord #dev-help channel
- Review documentation and examples

## 🎯 Development Workflow

### Typical Workflow

1. **Sync with upstream**:
```bash
git fetch upstream
git checkout main
git merge upstream/main
```

2. **Create feature branch**:
```bash
git checkout -b feature/new-feature
```

3. **Develop and test**:
```bash
cargo build
cargo test
cargo fmt
cargo clippy
```

4. **Commit and push**:
```bash
git add .
git commit -m "feat: add new feature"
git push origin feature/new-feature
```

5. **Create PR and iterate**

## 🏆 Recognition

Contributors will be:
- Listed in CONTRIBUTORS.md
- Mentioned in release notes
- Eligible for GLIN token grants (coming soon)

## ⚖️ License

By contributing, you agree that your contributions will be licensed under the Apache License 2.0.

## 🙏 Thank You!

Your contributions make GLIN Chain better for everyone. We appreciate your time and effort in improving the project!

---

Questions? Reach out on [Discord](https://discord.gg/glin-ai) or open an issue.