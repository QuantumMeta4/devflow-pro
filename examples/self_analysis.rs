use devflow_pro::ai::{types::*, LlamaCoder};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the LlamaCoder with default config
    let llama = LlamaCoder::new(LlamaConfig::default()).await?;

    // Get the source code to analyze
    let code = r#"
    use crate::DevFlowError;
    use dotenv::dotenv;
    use reqwest::Client;
    use serde::{Deserialize, Serialize};
    use std::env;

    #[derive(Debug, Clone)]
    pub struct LlamaCoder {
        client: Client,
        api_key: String,
    }

    impl LlamaCoder {
        pub async fn new(_config: super::types::LlamaConfig) -> Result<Self, DevFlowError> {
            dotenv().ok();
            let api_key = env::var("TOGETHER_API_KEY")
                .map_err(|_| DevFlowError::Config("TOGETHER_API_KEY not found in environment".into()))?;

            Ok(Self {
                client: Client::new(),
                api_key,
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
                        "Please review the following code and provide suggestions for improvement:\n\n{}",
                        code
                    )
                }
                super::types::AnalysisType::BugFinding => {
                    format!(
                        "Please analyze the following code for potential bugs and issues:\n\n{}",
                        code
                    )
                }
                super::types::AnalysisType::SecurityAudit => {
                    format!(
                        "Please analyze the following code for security vulnerabilities:\n\n{}",
                        code
                    )
                }
                super::types::AnalysisType::Documentation => {
                    format!(
                        "Please generate documentation for the following code:\n\n{}",
                        code
                    )
                }
                super::types::AnalysisType::Optimization => {
                    format!(
                        "Please suggest performance optimizations for the following code:\n\n{}",
                        code
                    )
                }
            };

            let request = TogetherAIRequest {
                model: "codellama/CodeLlama-34b-Instruct-hf".to_string(),
                prompt,
                max_tokens: 1000,
                temperature: 0.7,
            };

            let response = self
                .client
                .post("https://api.together.xyz/inference")
                .header("Authorization", format!("Bearer {}", self.api_key))
                .json(&request)
                .send()
                .await
                .map_err(|e| DevFlowError::Ai(e.to_string()))?;

            let result: TogetherAIResponse = response
                .json()
                .await
                .map_err(|e| DevFlowError::Ai(e.to_string()))?;

            Ok(super::types::AnalysisResult {
                summary: result.output.choices[0].text.clone(),
                suggestions: vec![],
                confidence: 0.8,
            })
        }
    }
    "#;

    // Analyze the code using different analysis types
    println!("🔍 Running code analysis...\n");

    // Code Review
    println!("📝 Code Review:");
    let review = llama.analyze_code(code, AnalysisType::CodeReview).await?;
    println!("{}\n", review.summary);

    // Security Audit
    println!("🔒 Security Audit:");
    let security = llama
        .analyze_code(code, AnalysisType::SecurityAudit)
        .await?;
    println!("{}\n", security.summary);

    // Documentation
    println!("📚 Documentation:");
    let docs = llama
        .analyze_code(code, AnalysisType::Documentation)
        .await?;
    println!("{}\n", docs.summary);

    Ok(())
}
