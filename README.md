# DevFlow Pro: Multi-Language Static Code Analysis Toolkit 🔍🛡️

![Build Status](https://github.com/QuantumMeta4/devflow-pro/actions/workflows/rust.yml/badge.svg)
![Version](https://img.shields.io/badge/version-1.0.3-blue)
![License](https://img.shields.io/badge/License-MIT-yellow)
![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange)
![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)

## 🌟 Overview

DevFlow Pro is an advanced, high-performance code analysis toolkit designed to provide comprehensive insights into software projects across multiple programming languages. Built with Rust's powerful ecosystem, it offers developers and engineering teams a sophisticated tool for assessing code quality, complexity, and security.

### 🚀 Why DevFlow Pro?

- **Blazing Fast Performance**: Leveraging Rust's efficiency for maximum speed
- **AI-Powered Insights**: Advanced code analysis using cutting-edge AI models
- **Comprehensive Analysis**: Deep dive into code quality, security, and performance
- **Multi-Language Support**: Analyzing 10+ programming languages
- **Parallel Processing**: Efficient analysis of large codebases
- **Security-First Approach**: Advanced vulnerability detection

## 📦 Quick Installation

### Prerequisites
- Rust 1.70+
- 64-bit operating system

### Install Methods

#### 1. Using Cargo (Recommended)
```bash
cargo install devflow-pro
```

#### 2. From Source
```bash
git clone https://github.com/QuantumMeta4/devflow-pro.git
cd devflow-pro
cargo build --release
```

## 🔍 Quick Start

### Basic Analysis
```bash
# Analyze current project
devflow-pro -p . 

# Enable AI-powered analysis
devflow-pro -p . --ai
```

## 🌈 Key Features

### 📊 Comprehensive Analysis
- **Code Metrics**: Lines of Code, Complexity, Dependencies
- **Performance Tracking**: Memory Usage, Processing Speed
- **Security Scanning**: Vulnerability Detection across multiple categories

### 🤖 AI-Enhanced Capabilities (Open Source codellama integration)
- Intelligent Code Reviews
- Security Vulnerability Detection
- Performance Optimization Suggestions
- Architecture Recommendations
- Best Practices Evaluation

### 🔐 Supported Languages
- **Full Support**: 
  - Rust
  - Python
  - JavaScript/TypeScript
  - Go

- **Partial Support**:
  - Java/Kotlin
  - C/C++
  - Ruby
  - PHP
  - C#
  - Swift
  - Shell Scripts

## 📋 Usage Examples

### Security Analysis
```bash
# Run comprehensive security audit
devflow-pro security ./my-project

# Focus on specific vulnerability types
devflow-pro security ./my-project --type injection
```

### AI-Powered Insights
```bash
# Get AI code review
devflow-pro ai review ./my-project

# Optimize code performance
devflow-pro ai optimize ./my-project
```

## 🛠️ Advanced Configuration

```toml
# config.toml
[analysis]
max_file_size = 1048576  # 1MB
min_severity = "low"     # low, medium, high, critical

[ignore]
patterns = [
  "**/target/**",
  "**/.git/**",
  "**/node_modules/**"
]

[security]
enabled = true
patterns = [
  "password\\s*=",
  "api[_-]?key\\s*="
]
```

## 🔍 Analysis Features

#### Static Analysis
- **Code Complexity**: Cyclomatic complexity calculation
- **Dependency Analysis**: Track imports and module relationships
- **Security Scanning**: Built-in patterns for common vulnerabilities
  - Command Injection
  - Hardcoded Secrets
  - SQL Injection
  - Unsafe File Operations
  - XSS Vulnerabilities
  - Unsafe Deserialization

#### Semantic Analysis
- **AST Parsing**: Deep code structure analysis
- **Function Analysis**: Track function complexity and relationships
- **Import Analysis**: Detailed dependency graphs
- **Complexity Metrics**: Per-function and per-file complexity scoring

#### Performance Analysis
- **File Size Distribution**: Categorization of files by size
- **Language Distribution**: Breakdown of codebase by language
- **Recent Changes**: Track recently modified files
- **Comment Ratio**: Code documentation coverage

## 📊 Output Formats

#### JSON Output
```bash
devflow-pro -p . --format json
```

```json
{
  "files_analyzed": 100,
  "total_lines": 5000,
  "language_distribution": {
    "rust": 45,
    "python": 30,
    "javascript": 25
  },
  "security_summary": [
    {
      "severity": "high",
      "description": "Potential hardcoded secret",
      "file": "config.js",
      "line": 42
    }
  ]
}
```

#### Human-Readable Report
```bash
devflow-pro -p . --format text
```

## ⚙️ Configuration

### Environment Variables
```bash
# API Configuration
TOGETHER_API_KEY=your_api_key_here

Get API here: https://www.together.ai


# Optional Settings
DEVFLOW_LOG_LEVEL=info
DEVFLOW_MAX_THREADS=4
```

### 🔧 Environment Variables

- `TOGETHER_API_KEY`: API key for AI-powered analysis
- `DEVFLOW_LOG_LEVEL`: Set logging verbosity (debug, info, warn, error)
- `DEVFLOW_CONFIG_PATH`: Custom path to configuration file

## 🤝 Contributing

We welcome contributions! Please read our [Contributing Guidelines](CONTRIBUTING.md) to get started.

## 📜 Code of Conduct

Please note that we have a [Code of Conduct](CODE_OF_CONDUCT.md) that we expect all contributors to follow.

## 📊 Project Status

- **Current Release**: v1.0.3 (Stable)
- **Maintenance**: Active Development
- **Community**: Start-Up

## 📜 License

Distributed under the MIT License. See `LICENSE` for more information.

## 🌐 Connect With Us

- 📧 Email: OmegaPhiAI@gmail.com
- 💬 Discord: [Join our Community](Coming Soon)
- 🐦 Twitter: [@Meta4ickal](https://twitter.com/meta4ickal)

---



## 🎯 Use Cases

1. **CI/CD Integration**
   ```bash
   # In your CI pipeline
   devflow-pro -p . --min-severity high --format json > analysis.json
   ```

2. **Pre-commit Hooks**
   ```bash
   # .git/hooks/pre-commit
   devflow-pro -p . --security-only
   ```

3. **Code Review Automation**
   ```bash
   # Generate detailed review report
   devflow-pro -p . --ai --format markdown > review.md
   ```
<div align="center">
Crafted with ❤️ by the Omega Phi Team
</div>