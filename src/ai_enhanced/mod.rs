use crate::{DevFlowError, Result};
use async_trait::async_trait;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{fmt, sync::Arc};
use tokio::sync::Semaphore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIAnalysisResult {
    pub code_quality_score: f64,
    pub security_recommendations: Vec<SecurityRecommendation>,
    pub optimization_suggestions: Vec<OptimizationSuggestion>,
    pub semantic_complexity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRecommendation {
    pub severity: crate::IssueSeverity,
    pub description: String,
    pub suggested_fix: Option<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    pub category: OptimizationCategory,
    pub description: String,
    pub impact_score: f64,
    pub suggested_implementation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationCategory {
    Performance,
    Memory,
    Security,
    Maintainability,
}

impl fmt::Display for OptimizationCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptimizationCategory::Performance => write!(f, "Performance"),
            OptimizationCategory::Memory => write!(f, "Memory"),
            OptimizationCategory::Security => write!(f, "Security"),
            OptimizationCategory::Maintainability => write!(f, "Maintainability"),
        }
    }
}

#[async_trait]
pub trait AIProvider: Send + Sync + std::fmt::Debug {
    async fn analyze_code(&self, content: &str) -> Result<AIAnalysisResult>;
    async fn suggest_fixes(&self, issues: &[crate::SecurityIssue]) -> Result<Vec<String>>;
}

#[derive(Debug)]
pub struct CodeLLamaProvider {
    api_key: String,
    base_url: String,
    model: String,
    semaphore: Arc<Semaphore>,
}

impl CodeLLamaProvider {
    #[must_use]
    pub fn new(api_key: &str, base_url: &str, model: &str, max_concurrent: usize) -> Self {
        Self {
            api_key: api_key.to_string(),
            base_url: base_url.to_string(),
            model: model.to_string(),
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    async fn send_ai_request(&self, prompt: &str) -> Result<String> {
        let _permit = self
            .semaphore
            .acquire()
            .await
            .map_err(|e| DevFlowError::AI(format!("Failed to acquire semaphore: {e}")))?;

        let url = format!("{}/v1/chat/completions", self.base_url);
        let client = reqwest::Client::new();

        let request_body = json!({
            "model": self.model,
            "messages": [
                {"role": "system", "content": "You are an expert code analysis AI specializing in Rust. Provide detailed, actionable feedback with concrete examples."},
                {"role": "user", "content": prompt}
            ],
            "temperature": 0.2,
            "max_tokens": 2000
        });

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| DevFlowError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(DevFlowError::AI(format!(
                "API request failed: {}",
                response.status()
            )));
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| DevFlowError::Network(e.to_string()))?;

        Ok(response_text)
    }

    fn parse_ai_response(&self, response: &str) -> Result<AIAnalysisResult> {
        #[derive(Deserialize)]
        struct AIResponse {
            choices: Vec<Choice>,
        }

        #[derive(Deserialize)]
        struct Choice {
            message: Message,
        }

        #[derive(Deserialize)]
        struct Message {
            content: String,
        }

        let response: AIResponse = serde_json::from_str(response)
            .map_err(|e| DevFlowError::AI(format!("Failed to parse AI response: {}", e)))?;

        let content = response
            .choices
            .first()
            .ok_or_else(|| DevFlowError::AI("Empty response from AI".to_string()))?
            .message
            .content
            .trim();

        let analysis: serde_json::Value = serde_json::from_str(content)
            .map_err(|e| DevFlowError::AI(format!("Failed to parse analysis JSON: {}", e)))?;

        Ok(AIAnalysisResult {
            code_quality_score: analysis["quality_score"].as_f64().unwrap_or(0.0),
            security_recommendations: analysis["security_issues"]
                .as_array()
                .unwrap_or(&Vec::new())
                .iter()
                .map(|issue| SecurityRecommendation {
                    severity: match issue["severity"].as_str().unwrap_or("LOW") {
                        "HIGH" => crate::IssueSeverity::High,
                        "MEDIUM" => crate::IssueSeverity::Medium,
                        _ => crate::IssueSeverity::Low,
                    },
                    description: issue["description"].as_str().unwrap_or("").to_string(),
                    suggested_fix: issue["fix"].as_str().map(String::from),
                    confidence: issue["confidence"].as_f64().unwrap_or(0.0),
                })
                .collect(),
            optimization_suggestions: analysis["optimizations"]
                .as_array()
                .unwrap_or(&Vec::new())
                .iter()
                .map(|opt| OptimizationSuggestion {
                    category: match opt["category"].as_str().unwrap_or("PERFORMANCE") {
                        "MEMORY" => OptimizationCategory::Memory,
                        "SECURITY" => OptimizationCategory::Security,
                        "MAINTAINABILITY" => OptimizationCategory::Maintainability,
                        _ => OptimizationCategory::Performance,
                    },
                    description: opt["description"].as_str().unwrap_or("").to_string(),
                    impact_score: opt["impact"].as_f64().unwrap_or(0.0),
                    suggested_implementation: opt["implementation"].as_str().map(String::from),
                })
                .collect(),
            semantic_complexity: analysis["complexity"].as_f64().unwrap_or(0.0),
        })
    }

    fn parse_fix_suggestions(&self, response: &str) -> Result<Vec<String>> {
        let response: serde_json::Value = serde_json::from_str(response)
            .map_err(|e| DevFlowError::AI(format!("Failed to parse fix suggestions: {}", e)))?;

        Ok(response["suggestions"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .filter_map(|s| s.as_str())
            .map(String::from)
            .collect())
    }
}

#[async_trait]
impl AIProvider for CodeLLamaProvider {
    async fn analyze_code(&self, content: &str) -> Result<AIAnalysisResult> {
        let prompt = format!(
            r#"Analyze the following Rust code and provide a detailed analysis in JSON format. Include:
1. A quality score from 0-100
2. A semantic complexity score
3. Security issues with severity (HIGH/MEDIUM/LOW), description, suggested fix, and confidence score
4. Optimization suggestions with category (PERFORMANCE/MEMORY/SECURITY/MAINTAINABILITY), description, impact score, and suggested implementation

IMPORTANT: Return ONLY a valid JSON object with NO additional text or notes. The response must be EXACTLY in this format:
{{
    "quality_score": float,
    "complexity": float,
    "security_issues": [
        {{
            "severity": "HIGH"|"MEDIUM"|"LOW",
            "description": string,
            "fix": string,
            "confidence": float
        }}
    ],
    "optimizations": [
        {{
            "category": "PERFORMANCE"|"MEMORY"|"SECURITY"|"MAINTAINABILITY",
            "description": string,
            "impact": float,
            "implementation": string
        }}
    ]
}}

Here's the code to analyze:

{}
"#,
            content
        );

        let response = self.send_ai_request(&prompt).await?;
        log::debug!("AI Response: {}", response);
        self.parse_ai_response(&response)
    }

    async fn suggest_fixes(&self, issues: &[crate::SecurityIssue]) -> Result<Vec<String>> {
        if issues.is_empty() {
            return Ok(Vec::new());
        }

        let issues_json = serde_json::to_string(issues)
            .map_err(|e| DevFlowError::AI(format!("Failed to serialize issues: {}", e)))?;

        let prompt = format!(
            r#"Given these security issues in JSON format:
{issues_json}

Provide specific code fixes in JSON format:
{{
    "suggestions": [
        "<detailed fix suggestion with code example>"
    ]
}}"#
        );

        let response = self.send_ai_request(&prompt).await?;
        self.parse_fix_suggestions(&response)
    }
}
