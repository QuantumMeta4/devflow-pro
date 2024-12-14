# Contributing to DevFlow Pro 🚀

First off, thank you for considering contributing to DevFlow Pro! It's people like you that make DevFlow Pro such a great tool.

## 📋 Table of Contents
- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Process](#development-process)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)
- [Community](#community)

## 📜 Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to OmegaPhiAI@gmail.com.

## 🚀 Getting Started

1. **Fork the Repository**
   ```bash
   git clone https://github.com/YOUR-USERNAME/devflow-pro.git
   cd devflow-pro
   ```

2. **Set Up Development Environment**
   ```bash
   # Install Rust (if not already installed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Install development dependencies
   cargo build
   ```

3. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

## 💻 Development Process

### Environment Setup
- Rust 1.70 or higher
- A modern IDE with Rust support (VS Code + rust-analyzer recommended)
- Together AI API key for AI-related features

### Local Development Workflow
1. Make your changes in a dedicated branch
2. Run the complete test suite in this order:
   ```bash
   # Format your code
   cargo fmt
   
   # Run the linter
   cargo clippy  --all-targets  --all-features  --  -D warnings  -W clippy::pedantic  -W clippy::nursery
   
   # Build the project
   cargo build
   
   # Run all tests
   cargo test
   
   # Run DevFlow Pro own analysis
   cargo run -- analyze ./src
   ```
3. Ensure all tests pass:
   - ✅ `cargo fmt` shows no formatting issues
   - ✅ `cargo clippy` shows no warnings
   - ✅ `cargo test` passes all tests
   - ✅ `cargo build` completes successfully
   - ✅ GitHub Actions CI pipeline is green
4. Update documentation for any changed features
5. Add tests for new features

### Pre-Pull Request Checklist
- [ ] Code is formatted with `cargo fmt`
- [ ] No warnings from `cargo clippy`
- [ ] All tests pass with `cargo test`
- [ ] Project builds with `cargo build`
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated
- [ ] GitHub Actions CI pipeline is passing
- [ ] New tests added for new features

## 🔄 Pull Request Process

1. **Before Submitting**
   - Update documentation
   - Add tests for new features
   - Run the full test suite
   - Update the CHANGELOG.md
   - Ensure CI passes

2. **PR Guidelines**
   - Use the PR template
   - Link related issues
   - Include before/after screenshots for UI changes
   - Describe your changes in detail

3. **Commit Message Format**
   ```
   type(scope): description
   
   [optional body]
   
   [optional footer]
   ```
   Types: feat, fix, docs, style, refactor, test, chore

## 📝 Coding Standards

### Rust Guidelines
- Follow Rust idioms and best practices
- Use `rustfmt` for formatting
- Address all Clippy warnings
- Maintain high test coverage

### Code Style
- Clear, descriptive variable names
- Document public APIs
- Keep functions focused and small
- Use meaningful error messages

## 🧪 Testing Guidelines

1. **Unit Tests**
   - Write tests for new functionality
   - Maintain existing tests
   - Use meaningful test names

2. **Integration Tests**
   - Test real-world scenarios
   - Cover error cases
   - Test API endpoints thoroughly

3. **Performance Tests**
   - Benchmark critical operations
   - Test with large codebases
   - Profile memory usage

## 📚 Documentation

1. **Code Documentation**
   - Document all public APIs
   - Include examples in doc comments
   - Keep documentation up to date

2. **Project Documentation**
   - Update README.md for new features
   - Maintain CHANGELOG.md
   - Document configuration options

## 👥 Community

- Join our [Discord](Coming Soon)
- Follow us on [Twitter](https://twitter.com/meta4ickal)
- Subscribe to our newsletter (Coming Soon)

## 🎯 Focus Areas

We're particularly interested in contributions in these areas:
- Language support expansion
- Performance optimizations
- Security analysis improvements
- AI integration enhancements
- Documentation improvements

## 🏆 Recognition

Contributors will be:
- Listed in our CONTRIBUTORS.md file
- Mentioned in release notes
- Featured on our website (Coming Soon)

## ❓ Questions?

Feel free to:
- Open an issue for questions
- Join our Discord community
- Email us at OmegaPhiAI@gmail.com

---

Remember, the best way to contribute is to:
1. Start small
2. Ask questions
3. Follow the guidelines
4. Be patient and respectful

Thank you for contributing to DevFlow Pro! 🎉
