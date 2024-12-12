use devflow_pro::ai::types::{AnalysisType, LlamaConfig};
use devflow_pro::ai::LlamaCoder;
use std::env;

#[tokio::test]
#[ignore = "Requires TOGETHER_API_KEY environment variable"]
async fn test_code_optimization() {
    // Skip test if TOGETHER_API_KEY is not set
    if env::var("TOGETHER_API_KEY").is_err() {
        println!("Skipping test: TOGETHER_API_KEY not set");
        return;
    }

    // Initialize LlamaCoder
    let coder = LlamaCoder::new(LlamaConfig::default())
        .expect("Failed to initialize LlamaCoder");

    // Test code to analyze
    let test_code = r"
    fn process_data(data: &[u32]) -> Vec<u32> {
        let mut result = Vec::new();
        for i in 0..data.len() {
            if data[i] % 2 == 0 {
                result.push(data[i]);
            }
        }
        result
    }
    ";

    // Run optimization analysis
    let analysis = coder
        .analyze_code(test_code, AnalysisType::CodeReview)
        .await
        .expect("Failed to analyze code");

    // Verify analysis results
    assert!(!analysis.suggestions.is_empty(), "Expected optimization suggestions");
    assert!(
        analysis.confidence > 0.0,
        "Expected non-zero confidence score"
    );

    // Print analysis results for inspection
    println!("\n=== AI Optimization Analysis ===");
    println!("Summary:\n{}", analysis.summary);
    println!("\nSuggestions:");
    for suggestion in &analysis.suggestions {
        println!("- {}", suggestion);
    }
    println!("\nConfidence: {:.2}", analysis.confidence);
}
