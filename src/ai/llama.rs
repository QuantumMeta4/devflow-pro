use crate::DevFlowError;
use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{env, time::Duration};
use tokio::time::sleep;

const MAX_RETRIES: u32 = 3;
const RETRY_DELAY_MS: u64 = 1000;
const RATE_LIMIT_DELAY_MS: u64 = 2000;

#[derive(Debug, Clone)]
pub struct LlamaCoder {
    client: Client,
    api_key: String,
    config: super::types::LlamaConfig,
}

#[derive(Debug, Serialize)]
struct TogetherAIRequest {
    model: String,
    prompt: String,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Debug, Deserialize)]
struct TogetherAIResponse {
    #[serde(default)]
    output: Option<TogetherAIOutput>,
    #[serde(default)]
    error: Option<TogetherAIError>,
}

#[derive(Debug, Deserialize)]
struct TogetherAIError {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
}

#[derive(Debug, Deserialize)]
struct TogetherAIOutput {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    text: String,
}

impl LlamaCoder {
    pub async fn new(config: super::types::LlamaConfig) -> Result<Self, DevFlowError> {
        dotenv().ok();
        let api_key = match env::var("TOGETHER_API_KEY") {
            Ok(key) if !key.is_empty() => key,
            _ => {
                return Err(DevFlowError::Config(
                    "TOGETHER_API_KEY not found or empty in environment. Please set it with: export TOGETHER_API_KEY=your_key".into()
                ))
            }
        };

        Ok(Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .map_err(|e| DevFlowError::Ai(format!("Failed to create HTTP client: {}", e)))?,
            api_key,
            config,
        })
    }

    pub async fn analyze_code(
        &self,
        code: &str,
        analysis_type: super::types::AnalysisType,
    ) -> Result<super::types::AnalysisResult, DevFlowError> {
        let prompt = match analysis_type {
            super::types::AnalysisType::CodeReview => {
                format!(
                    "You are an expert code reviewer. Analyze this code and provide specific, actionable feedback:\n\n```\n{}\n```\n\nProvide a concise analysis focusing on:\n1. Code quality and maintainability\n2. Performance and efficiency\n3. Best practices and patterns\n4. Potential issues or bugs\n5. Suggestions for improvement\n\nFormat your response in clear sections with bullet points.",
                    code
                )
            }
            super::types::AnalysisType::BugFinding => {
                format!(
                    "You are a bug-finding expert. Analyze this code for potential bugs and issues:\n\n```\n{}\n```\n\nList each potential issue with:\n1. Issue description\n2. Severity (Low/Medium/High/Critical)\n3. Impact on code\n4. Suggested fix\n5. Prevention tips\n\nFormat your response in clear sections with bullet points.",
                    code
                )
            }
            super::types::AnalysisType::SecurityAudit => {
                format!(
                    "You are a security expert. Analyze this code for security vulnerabilities:\n\n```\n{}\n```\n\nFor each vulnerability found, provide:\n1. Vulnerability type and CWE ID\n2. Severity (Low/Medium/High/Critical)\n3. Potential impact and attack vectors\n4. Mitigation steps\n5. Best practices to prevent\n\nFormat your response in clear sections with bullet points.",
                    code
                )
            }
            super::types::AnalysisType::Documentation => {
                format!(
                    "You are a technical documentation expert. Generate comprehensive documentation for this code:\n\n```\n{}\n```\n\nInclude:\n1. Overview and purpose\n2. Key components and architecture\n3. Usage examples with code snippets\n4. Dependencies and requirements\n5. Error handling and edge cases\n\nFormat your response in clear sections with markdown formatting.",
                    code
                )
            }
            super::types::AnalysisType::Optimization => {
                format!(
                    "You are a performance optimization expert. Analyze this code for optimization opportunities:\n\n```\n{}\n```\n\nFor each optimization opportunity, provide:\n1. Current performance bottleneck\n2. Impact on system resources\n3. Suggested improvement with code example\n4. Expected performance benefit\n5. Implementation complexity\n\nFormat your response in clear sections with bullet points.",
                    code
                )
            }
        };

        let request = TogetherAIRequest {
            model: self.config.model_name.clone(),
            prompt,
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
        };

        let mut retries = 0;

        loop {
            match self.make_api_request(&request).await {
                Ok(result) => {
                    let summary = result
                        .output
                        .and_then(|o| o.choices.first().map(|c| c.text.trim().to_string()))
                        .unwrap_or_else(|| "No analysis available".to_string());

                    let suggestions = extract_suggestions(&summary);

                    // Ensure we have at least one suggestion
                    let suggestions = if suggestions.is_empty() {
                        // Extract potential suggestions from the summary
                        summary
                            .lines()
                            .filter(|line| {
                                let line = line.trim().to_lowercase();
                                line.contains("suggest")
                                    || line.contains("recommend")
                                    || line.contains("could")
                                    || line.contains("should")
                                    || line.contains("improve")
                            })
                            .map(|s| s.trim().to_string())
                            .collect()
                    } else {
                        suggestions
                    };

                    let confidence = calculate_confidence(&summary);

                    return Ok(super::types::AnalysisResult {
                        summary,
                        suggestions,
                        confidence,
                    });
                }
                Err(e) => {
                    if retries >= MAX_RETRIES {
                        return Err(e);
                    }
                    if e.to_string().contains("rate limiting") {
                        sleep(Duration::from_millis(RATE_LIMIT_DELAY_MS)).await;
                    } else {
                        sleep(Duration::from_millis(RETRY_DELAY_MS)).await;
                    }
                    retries += 1;
                }
            }
        }
    }

    async fn make_api_request(
        &self,
        request: &TogetherAIRequest,
    ) -> Result<TogetherAIResponse, DevFlowError> {
        let response = self
            .client
            .post("https://api.together.xyz/inference")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(request)
            .send()
            .await
            .map_err(|e| DevFlowError::Ai(e.to_string()))?;

        let result: TogetherAIResponse = response
            .json()
            .await
            .map_err(|e| DevFlowError::Ai(e.to_string()))?;

        if let Some(error) = result.error {
            return Err(DevFlowError::Ai(format!(
                "{} ({})",
                error.message, error.error_type
            )));
        }

        Ok(result)
    }
}

fn extract_suggestions(analysis: &str) -> Vec<String> {
    let mut suggestions = Vec::new();

    // Extract suggestions from bullet points and numbered lists
    for line in analysis.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('•')
            || trimmed.starts_with('-')
            || (trimmed.starts_with(char::is_numeric) && trimmed.contains('.'))
        {
            suggestions.push(
                trimmed
                    .trim_start_matches(|c: char| !c.is_alphabetic())
                    .trim()
                    .to_string(),
            );
        }
    }

    // If no bullet points found, try to extract suggestions from paragraphs
    if suggestions.is_empty() {
        for line in analysis.lines() {
            let trimmed = line.trim();
            if trimmed.len() > 10
                && (trimmed.contains("suggest")
                    || trimmed.contains("recommend")
                    || trimmed.contains("could")
                    || trimmed.contains("should")
                    || trimmed.contains("improve"))
            {
                suggestions.push(trimmed.to_string());
            }
        }
    }

    suggestions
}

fn calculate_confidence(analysis: &str) -> f64 {
    let factors: [(bool, f64); 8] = [
        (analysis.contains("definitely"), 0.2),
        (analysis.contains("likely"), 0.15),
        (analysis.contains("possibly"), 0.1),
        (analysis.contains("might"), 0.05),
        (analysis.contains("code example"), 0.15),
        (analysis.contains("specific"), 0.1),
        (analysis.contains("recommend"), 0.1),
        (analysis.contains("suggest"), 0.1),
    ];

    let base_confidence: f64 = 0.5;
    let confidence = factors.iter().fold(
        base_confidence,
        |acc, (found, weight)| {
            if *found {
                acc + weight
            } else {
                acc
            }
        },
    );

    if confidence > 1.0 {
        1.0
    } else if confidence < 0.0 {
        0.0
    } else {
        confidence
    }
}
