use super::*;
use tokio;

#[tokio::test]
async fn test_llama_api_connection() {
    // Load environment variables
    dotenv::dotenv().ok();
    
    // Create a basic config
    let config = super::types::LlamaConfig {
        model: "togethercomputer/llama-2-7b".to_string(),
        max_tokens: 100,
        temperature: 0.7,
    };

    // Initialize the LlamaCoder
    let llama = LlamaCoder::new(config).expect("Failed to create LlamaCoder");

    // Test a simple code analysis
    let test_code = "fn main() { println!(\"Hello, World!\"); }";
    let result = llama.analyze_code(test_code, super::types::AnalysisType::CodeReview)
        .await
        .expect("API request failed");

    // Verify we got a response
    assert!(!result.suggestions.is_empty(), "Should receive analysis suggestions");
    assert!(result.confidence > 0.0, "Should have non-zero confidence");
}

#[tokio::test]
async fn test_api_error_handling() {
    // Test with invalid API key
    let config = super::types::LlamaConfig {
        model: "invalid-model".to_string(),
        max_tokens: 100,
        temperature: 0.7,
    };

    let llama = LlamaCoder::new(config).expect("Failed to create LlamaCoder");
    let test_code = "print('test')";
    
    let result = llama.analyze_code(test_code, super::types::AnalysisType::CodeReview).await;
    assert!(result.is_err(), "Should return an error for invalid model");
}
