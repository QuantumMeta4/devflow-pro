use devflow_pro::ai::{LlamaCoder, types::{LlamaConfig, AnalysisType}};
use std::env;
use tokio;

#[tokio::test]
#[ignore = "Requires TOGETHER_API_KEY environment variable"]
async fn test_ai_optimization() {
    // Skip test if TOGETHER_API_KEY is not set
    if env::var("TOGETHER_API_KEY").is_err() {
        println!("Skipping test: TOGETHER_API_KEY not set");
        return;
    }
    
    // Configure AI
    let config = LlamaConfig::default();
    
    // Initialize LlamaCoder
    let coder = LlamaCoder::new(config).await.expect("Failed to initialize LlamaCoder");
    
    // Test code to analyze
    let test_code = r#"
    fn process_data(data: &[u32]) -> Vec<u32> {
        let mut result = Vec::new();
        for i in 0..data.len() {
            if data[i] % 2 == 0 {
                result.push(data[i] * 2);
            }
        }
        result
    }
    "#;
    
    // Run optimization analysis
    let analysis = coder.analyze_code(test_code, AnalysisType::Optimization)
        .await
        .expect("Failed to analyze code");
    
    // Verify analysis results
    assert!(!analysis.summary.is_empty(), "Analysis summary should not be empty");
    assert!(!analysis.suggestions.is_empty(), "Should have optimization suggestions");
    assert!(analysis.confidence > 0.0 && analysis.confidence <= 1.0, "Confidence should be between 0 and 1");
    
    // Print analysis results for inspection
    println!("\n=== AI Optimization Analysis ===");
    println!("Summary:\n{}", analysis.summary);
    println!("\nSuggestions:");
    for suggestion in &analysis.suggestions {
        println!("- {}", suggestion);
    }
    println!("\nConfidence: {:.2}", analysis.confidence);
}
