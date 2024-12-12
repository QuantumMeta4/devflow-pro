use crate::ai::llama::LlamaCoder;
use crate::ai::types::{LlamaConfig, AnalysisType};

pub async fn check_api_connection() -> Result<String, String> {
    dotenv::dotenv().ok();
    
    // Create a test configuration
    let config = LlamaConfig {
        model: "togethercomputer/llama-2-7b".to_string(),
        max_tokens: 10,
        temperature: 0.7,
    };

    // Try to initialize the client
    let llama = match LlamaCoder::new(config) {
        Ok(client) => client,
        Err(e) => return Err(format!("Failed to initialize API client: {}", e)),
    };

    // Try a simple API call
    let test_code = "fn test() {}";
    match llama.analyze_code(test_code, AnalysisType::CodeReview).await {
        Ok(_) => Ok("API connection successful! The integration is working properly.".to_string()),
        Err(e) => Err(format!("API connection failed: {}", e)),
    }
}
