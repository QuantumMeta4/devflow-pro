# DevFlow Pro: Multi-Language Static AI Code Analysis Toolkit 🔍🛡️

![Build Status](https://github.com/QuantumMeta4/devflow-pro/actions/workflows/rust.yml/badge.svg)
![Version](https://img.shields.io/github/v/release/QuantumMeta4/devflow-pro?label=version)
![License](https://img.shields.io/github/license/QuantumMeta4/devflow-pro)
![Rust Version](https://img.shields.io/github/actions/workflow/status/QuantumMeta4/devflow-pro/rust.yml?label=rust&logo=rust)
![PRs Welcome](https://img.shields.io/github/issues-pr-raw/QuantumMeta4/devflow-pro?label=PRs%20welcome)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/QuantumMeta4/devflow-pro?label=latest%20release)
![GitHub issues](https://img.shields.io/github/issues/QuantumMeta4/devflow-pro)
![GitHub pull requests](https://img.shields.io/github/issues-pr/QuantumMeta4/devflow-pro)
![GitHub last commit](https://img.shields.io/github/last-commit/QuantumMeta4/devflow-pro)
![GitHub Repo stars](https://img.shields.io/github/stars/QuantumMeta4/devflow-pro?style=social)

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

### Command Line Usage

```bash
# Basic code analysis
devflow-pro analyze ./src

# Analyze with specific focus
devflow-pro analyze ./src --focus security,performance

# Generate detailed report
devflow-pro analyze ./src --output report.json --format json

# Analyze with custom rules
devflow-pro analyze ./src --rules-file .devflow/rules.toml

# Security scan
devflow-pro security ./src --severity high

# Performance analysis
devflow-pro perf ./src --threshold 0.8

# Generate documentation
devflow-pro docs ./src --output docs/

# Run tests with analysis
devflow-pro test --analyze
```

### Configuration Options

```bash
# Global options
--verbose                 Enable verbose output
--log-file <FILE>        Write logs to file
--config <FILE>          Use custom config file
--ignore <PATTERN>       Ignore files/directories
--max-file-size <SIZE>   Set maximum file size to analyze

# Analysis options
--focus <AREAS>          Focus on specific analysis areas
--min-severity <LEVEL>   Set minimum severity level
--format <FORMAT>        Set output format (text/json)
--output <FILE>         Write output to file

# Security options
--security-level <LEVEL> Set security analysis level
--vuln-db <PATH>        Custom vulnerability database

# Performance options
--perf-threshold <NUM>   Set performance threshold
--profile               Enable performance profiling
```

### Example Workflows

#### 1. Basic Project Analysis
```bash
# Initial analysis
devflow-pro analyze ./src --verbose

# Generate report
devflow-pro analyze ./src --output analysis.json --format json
```

#### 2. Security Audit
```bash
# Full security scan
devflow-pro security ./src --severity high --output security.json

# Focused scan
devflow-pro security ./src --focus injection,xss --verbose
```

#### 3. Performance Optimization
```bash
# Performance analysis
devflow-pro perf ./src --threshold 0.8 --profile

# Memory analysis
devflow-pro perf ./src --focus memory --verbose
```

#### 4. Documentation
```bash
# Generate docs
devflow-pro docs ./src --output docs/

# Update existing docs
devflow-pro docs ./src --update --verbose
```

### Basic Analysis in Windsurf IDE
```rust
// Ask Cascade to analyze your code
"Please analyze this file for security issues"
"Can you optimize this code?"
"Check for potential memory leaks"

// Get real-time suggestions
"What improvements can be made at line 42?"
"Suggest optimizations for this function"
```

### 🤖 Using DevFlow Pro with Cascade AI Agent Workflow

DevFlow Pro can be seamlessly integrated with Cascade AI for enhanced code analysis. Here's how to use it:

#### Basic Usage with Cascade AI
```bash
# Basic analysis with AI integration
env TOGETHER_API_KEY=<your-api-key> devflow-pro --path /path/to/project --ai --verbose --format text --output stdout

# Example with all options
env TOGETHER_API_KEY=<your-api-key> devflow-pro \
  --path /path/to/project \
  --ai \
  --verbose \
  --format text \
  --output stdout \
  --security-patterns "unwrap,panic,unsafe" \
  --ignore "target/*,*.bak" \
  --max-file-size 1000000
```

#### Advanced Features in Cascade AI Workflow

1. **AI-Enhanced Security Analysis**
```bash
# Security-focused analysis
env TOGETHER_API_KEY=<your-api-key> devflow-pro \
  --path /path/to/project \
  --ai \
  --security-patterns "sql_injection,xss,csrf" \
  --format json \
  --output security_report.json
```

2. **Performance Analysis with AI**
```bash
# Performance optimization analysis
env TOGETHER_API_KEY=<your-api-key> devflow-pro \
  --path /path/to/project \
  --ai \
  --focus performance \
  --verbose \
  --output perf_report.txt
```

3. **Code Quality Assessment**
```bash
# Code quality analysis
env TOGETHER_API_KEY=<your-api-key> devflow-pro \
  --path /path/to/project \
  --ai \
  --focus quality \
  --format markdown \
  --output quality_report.md
```

4. **Documentation Generation**
```bash
# Generate AI-enhanced documentation
env TOGETHER_API_KEY=<your-api-key> devflow-pro \
  --path /path/to/project \
  --ai \
  --focus docs \
  --output ./docs
```

#### Integration with Other Tools

1. **CI/CD Pipeline Integration**
```yaml
# GitHub Actions example
steps:
  - name: Run DevFlow Pro Analysis
    env:
      TOGETHER_API_KEY: ${{ secrets.TOGETHER_API_KEY }}
    run: |
      devflow-pro --path . --ai --verbose --format json --output analysis.json
```

2. **Pre-commit Hook**
```bash
#!/bin/bash
env TOGETHER_API_KEY=<your-api-key> devflow-pro \
  --path . \
  --ai \
  --focus security,quality \
  --format text
```

### 🤖 Using DevFlow Pro with Cascade AI in Windsurf IDE

When using DevFlow Pro through Cascade AI in Windsurf IDE, you can leverage natural language interactions for powerful code analysis. Here's how to get the most out of it:

#### 1. Code Analysis and Security
```rust
// Request comprehensive analysis
User: "Analyze this entire file for potential issues"
Cascade: *Performs full analysis covering security, performance, and best practices*

// Focus on security
User: "Check this authentication function for security vulnerabilities"
Cascade: *Analyzes security aspects like input validation, encryption, and access control*

// Request specific security checks
User: "Look for SQL injection vulnerabilities in this query function"
Cascade: *Focuses on SQL-related security issues*
```

#### 2. Performance Optimization
```rust
// General optimization
User: "What performance improvements can be made in this file?"
Cascade: *Analyzes code for performance bottlenecks and suggests optimizations*

// Specific function optimization
User: "How can I make this data processing function faster?"
Cascade: *Provides targeted performance improvements for the function*

// Memory optimization
User: "Check for memory leaks in this resource handling code"
Cascade: *Analyzes memory management and suggests improvements*
```

#### 3. Code Quality and Best Practices
```rust
// Code review
User: "Review this module for Rust best practices"
Cascade: *Checks code against Rust idioms and community standards*

// Specific improvements
User: "How can I make this error handling more idiomatic?"
Cascade: *Suggests Rust-specific error handling improvements*

// Architecture review
User: "Is this the best way to structure these components?"
Cascade: *Analyzes architectural decisions and suggests improvements*
```

#### 4. Testing and Documentation
```rust
// Test generation
User: "Help me write tests for this API endpoint"
Cascade: *Generates comprehensive test cases with examples*

// Documentation
User: "Generate documentation for this module"
Cascade: *Creates documentation following Rust documentation standards*

// Test coverage
User: "What areas of this code need better test coverage?"
Cascade: *Analyzes test coverage and suggests additional tests*
```

#### 5. Real-time Assistance
```rust
// Context-aware help
User: "What's wrong with this line of code?"
Cascade: *Analyzes current line and provides specific guidance*

// Code completion
User: "How should I complete this function?"
Cascade: *Suggests appropriate code completion based on context*

// Error resolution
User: "Why am I getting this compiler error?"
Cascade: *Explains error and suggests fixes*
```

#### Best Practices for Cascade AI Interaction

1. **Be Specific**
   - ✅ "Check this authentication function for SQL injection"
   - ❌ "Is this secure?"

2. **Provide Context**
   - ✅ "This function handles user uploads. Are there any security issues?"
   - ❌ "Check for issues"

3. **Ask for Explanations**
   - ✅ "Why is this approach better than using a Vec?"
   - ❌ "Fix this"

4. **Request Examples**
   - ✅ "Show me how to implement this using async/await"
   - ❌ "Make it async"

5. **Iterate and Refine**
   - ✅ "Now optimize the error handling in the part we just changed"
   - ❌ "Make everything better"

### 🌈 Key Features

### 📊 Comprehensive Analysis
- **Code Metrics**: Lines of Code, Complexity, Dependencies
- **Performance Tracking**: Memory Usage, Processing Speed
- **Security Scanning**: Vulnerability Detection across multiple categories

### 🤖 AI-Enhanced Capabilities (Together.xyz CodeLlama Integration)
- Intelligent Code Reviews using Llama Vision 11B + FLUX.1 [schnell] model
- Real-time Security Vulnerability Detection
- Context-aware Performance Optimization Suggestions
- Smart Architecture Recommendations
- Best Practices Evaluation with confidence scoring
- Semantic Code Understanding

### 🔑 Together.xyz API Setup (Required)

#### 1. Get API Key
1. Visit [Together.xyz](https://www.together.xyz)
2. Create an account or sign in
3. Navigate to API settings
4. Generate a new API key
5. Copy your API key

#### 2. Set Your API Key
Before using DevFlow Pro, you **must** set your Together.xyz API key as an environment variable:

```bash
# Linux/macOS
export TOGETHER_API_KEY=your_api_key_here

# Windows (PowerShell)
$env:TOGETHER_API_KEY="your_api_key_here"

# To make it permanent, add to your shell's config file (.bashrc, .zshrc, etc.)
echo 'export TOGETHER_API_KEY=your_api_key_here' >> ~/.bashrc  # or ~/.zshrc
```

> ⚠️ **Important**: The application will not function without a valid Together.xyz API key. This is a security measure to ensure each user manages their own API access.

### 🌊 Windsurf IDE Integration

DevFlow Pro seamlessly integrates with Windsurf IDE through the Cascade AI assistant. This integration provides:

#### Features
- Real-time code analysis
- Context-aware suggestions
- Intelligent code completion
- Security vulnerability detection
- Performance optimization recommendations

#### Setup
1. Install Windsurf IDE
2. Configure your Together.xyz API key as described above
3. Enable the DevFlow Pro plugin in Windsurf settings

#### Usage
In Windsurf IDE, you can interact with Cascade AI using natural language:

```rust
// Code Analysis
"Analyze this file for security issues"
"Check for performance bottlenecks"
"Review this function for best practices"

// Real-time Assistance
"What improvements can be made here?"
"How can I optimize this code?"
"Suggest better error handling"

// Project Management
"Help me document this module"
"Generate unit tests for this class"
"Review my PR changes"
```

### 🤖 Windsurf and Cascade AI Integration

DevFlow Pro is powered by Together.xyz's Llama Vision 11B + FLUX.1 [schnell] model, and integrates direct with Windsurf IDE. This integration allows you to use DevFlow Pro through the Cascade AI assistant and soon as a extension. This integration provides a seamless experience with a custom DevFlow Pro environment for the Windsurf IDE and Cascade AI agent workflow. This provides intelligent code analysis, efficienct Cascade usage, and CodeLlama enhanced integretion with whichever agent is currently active.

#### Features
- **Semantic Code Understanding**: Analyzes code context and purpose
- **Security Analysis**: Identifies potential vulnerabilities
- **Performance Optimization**: Suggests improvements for better efficiency
- **Best Practices**: Recommends Rust idioms and patterns
- **Documentation**: Helps generate and improve documentation
- **Testing**: Assists in writing comprehensive tests

#### Configuration
```toml
# config/default.toml
[ai]
model = "codellama-34b-instruct"
temperature = 0.7
max_tokens = 2048
confidence_threshold = 0.8

[analysis]
security_level = "high"
performance_threshold = 0.9
```

#### Best Practices
1. **Be Specific**: Provide clear, focused requests
2. **Include Context**: Share relevant background information
3. **Iterate**: Break complex tasks into smaller steps
4. **Review**: Always verify AI suggestions
5. **Security**: Never share sensitive information in prompts

### 📊 Analysis Features

#### Code Quality
```bash
# Basic analysis
devflow-pro analyze ./src

# Detailed analysis with AI insights
devflow-pro analyze ./src --ai --verbose --format text --output stdout

# Security-focused analysis
devflow-pro analyze ./src --security-level high

# Performance analysis
devflow-pro analyze ./src --performance-metrics
```

#### Custom Rules
```toml
# .devflow/rules.toml
[rules]
max_complexity = 15
min_test_coverage = 80
required_docs = true
security_checks = ["sql-injection", "xss", "csrf"]
```

### 🚀 Advanced Features

#### 1. Custom Analysis Rules
```toml
# .devflow/rules.toml
[rules]
max_complexity = 10
min_coverage = 80
banned_functions = ["unsafe_operation", "deprecated_api"]

[security]
severity_threshold = "high"
scan_dependencies = true

[performance]
memory_threshold = "100MB"
cpu_threshold = "50%"
```

#### 2. Analysis Profiles
```bash
# Development profile
devflow-pro --profile dev --quick-scan

# Production profile
devflow-pro --profile prod --deep-scan

# Custom profile
devflow-pro --profile custom --config my-rules.toml
```

#### 3. Plugin System
```rust
// Custom plugin example
#[devflow_plugin]
fn custom_analyzer(code: &str) -> Analysis {
    // Custom analysis logic
}
```

### 🛠️ Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details on:

- Setting up your development environment
- Code style guidelines
- Commit message conventions
- Pull request process
- Testing requirements
- Documentation standards

Before contributing, please:

1. Read our [Contributing Guide](CONTRIBUTING.md)
2. Fork the repository
3. Create a feature branch
4. Make your changes following our guidelines
5. Run the full test suite:
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   devflow-pro analyze ./src
   ```
6. Submit a PR with a clear description of your changes

### 📝 License

MIT License - See [LICENSE](LICENSE) for details.

## 🌐 Connect With Us

- 📧 Email: OmegaPhiAI@gmail.com
- 💬 Discord: [Join our Community](Coming Soon)
- 🐦 Twitter: [@Meta4ickal](https://twitter.com/meta4ickal)

---
<div align="center">
Crafted with ❤️ by the Omega Phi Team
</div>
