# AI Configuration Guide

## Setting up Together AI API Key

DevFlow Pro uses Together AI for its AI-powered analysis features. To use these features, you'll need to configure the API key securely.

### Local Development

For local development, set the `TOGETHER_API_KEY` environment variable:

```bash
# Add to your shell configuration file (.bashrc, .zshrc, etc.)
export TOGETHER_API_KEY=your_key_here
```

### GitHub Actions

For GitHub Actions workflows, follow these steps to configure the API key:

1. Go to your GitHub repository settings
2. Navigate to "Secrets and variables" → "Actions"
3. Click "New repository secret"
4. Add a new secret with:
   - Name: `TOGETHER_API_KEY`
   - Value: Your Together AI API key

The workflow will automatically use this secret when running tests and AI analysis features.

### Configuration File

The AI features can be configured in your project's configuration file:

```toml
# ~/.config/devflow-pro/config.toml

[ai]
enabled = true
model_type = "advanced"  # basic, standard, advanced
temperature = 0.7       # 0.0 to 1.0
max_tokens = 2000

# Analysis Preferences
detail_level = "high"   # low, medium, high
focus_areas = [
    "security",
    "performance",
    "best-practices"
]

# Output Configuration
output_format = "detailed"  # summary, detailed, technical
include_examples = true
suggest_fixes = true
```

### Security Best Practices

1. Never commit API keys to version control
2. Use environment variables for local development
3. Use GitHub Secrets for CI/CD workflows
4. Regularly rotate your API keys
5. Set appropriate access permissions for team members

### Troubleshooting

If you encounter issues with the AI features:

1. Verify the API key is correctly set in your environment
2. Check the GitHub Actions secrets are properly configured
3. Ensure the configuration file has the correct permissions
4. Review the logs for any authentication errors
