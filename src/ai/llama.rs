use crate::DevFlowError;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::ClientBuilder;
use serde::{Deserialize, Serialize};
use std::{env, time::Duration};
use tokio::time::sleep;

const MAX_RETRIES: u32 = 3;
const RETRY_DELAY_MS: u64 = 1000;
const RATE_LIMIT_DELAY_MS: u64 = 2000;

/// LLM-based code analysis implementation
#[derive(Debug, Clone)]
pub struct Coder {
    client: reqwest::Client,
    api_key: String,
    model: String,
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

impl Coder {
    /// Creates a new `Coder` instance
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - `TOGETHER_API_KEY` environment variable is not set or empty
    /// - Failed to create HTTP client
    pub fn new(config: super::types::LlamaConfig) -> Result<Self, DevFlowError> {
        let api_key = env::var("TOGETHER_API_KEY").map_err(|_| {
            DevFlowError::AI(
                "TOGETHER_API_KEY environment variable not set. Please set your Together.ai API key first.".into()
            )
        })?;

        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| DevFlowError::AI(format!("Failed to create HTTP client: {e}")))?;

        Ok(Self {
            client,
            api_key,
            model: config.model_name,
        })
    }

    /// Analyzes code using specified analysis type
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - API request fails
    /// - Response parsing fails
    /// - Maximum retries exceeded
    pub async fn analyze_code(
        &self,
        code: &str,
        analysis_type: super::types::AnalysisType,
    ) -> Result<super::types::AnalysisResult, DevFlowError> {
        let prompt = match analysis_type {
            super::types::AnalysisType::CodeReview => {
                format!(
                    "You are an expert code reviewer. Analyze this code and provide specific, actionable feedback:\n\n```\n{code}\n```\n\nProvide a concise summary of the code quality and list specific suggestions for improvement."
                )
            }
            super::types::AnalysisType::BugFinding => self.analyze_bugs(code),
            super::types::AnalysisType::SecurityAudit => self.analyze_security(code),
            super::types::AnalysisType::Documentation => self.analyze_docs(code),
            super::types::AnalysisType::Optimization => self.analyze_performance(code),
        };

        let request = TogetherAIRequest {
            model: self.model.clone(),
            prompt,
            max_tokens: 2048,
            temperature: 0.7,
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
                        confidence: confidence.clamp(0.0, 1.0),
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

    #[must_use]
    pub fn analyze_bugs(&self, code: &str) -> String {
        format!("You are a bug-finding expert. Analyze this code for potential bugs and issues:\n\n```\n{code}\n```\n\nList each potential issue with:\n1. Issue description\n2. Severity (Low/Medium/High/Critical)\n3. Impact on code\n4. Suggested fix\n5. Prevention tips\n\nFormat your response in clear sections with bullet points.")
    }

    #[must_use]
    pub fn analyze_security(&self, code: &str) -> String {
        format!("You are a security expert. Analyze this code for security vulnerabilities:\n\n```\n{code}\n```\n\nFor each vulnerability found, provide:\n1. Vulnerability type and CWE ID\n2. Severity (Low/Medium/High/Critical)\n3. Potential impact and attack vectors\n4. Mitigation steps\n5. Best practices to prevent\n\nFormat your response in clear sections with bullet points.")
    }

    #[must_use]
    pub fn analyze_docs(&self, code: &str) -> String {
        format!("You are a technical documentation expert. Generate comprehensive documentation for this code:\n\n```\n{code}\n```\n\nInclude:\n1. Overview and purpose\n2. Key components and architecture\n3. Usage examples with code snippets\n4. Dependencies and requirements\n5. Error handling and edge cases\n\nFormat your response in clear sections with markdown formatting.")
    }

    #[must_use]
    pub fn analyze_performance(&self, code: &str) -> String {
        format!("You are a performance optimization expert. Analyze this code for optimization opportunities:\n\n```\n{code}\n```\n\nFor each optimization opportunity, provide:\n1. Current performance bottleneck\n2. Impact on system resources\n3. Suggested improvement with code example\n4. Expected performance benefit\n5. Implementation complexity\n\nFormat your response in clear sections with bullet points.")
    }

    async fn make_api_request(
        &self,
        request: &TogetherAIRequest,
    ) -> Result<TogetherAIResponse, DevFlowError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.api_key))
                .map_err(|e| DevFlowError::AI(format!("Invalid API key: {e}")))?,
        );

        let response = self
            .client
            .post("https://api.together.xyz/inference")
            .headers(headers)
            .json(request)
            .send()
            .await
            .map_err(|e| DevFlowError::AI(e.to_string()))?;

        let result: TogetherAIResponse = response
            .json()
            .await
            .map_err(|e| DevFlowError::AI(e.to_string()))?;

        if let Some(error) = result.error {
            return Err(DevFlowError::AI(format!(
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

    confidence
}
