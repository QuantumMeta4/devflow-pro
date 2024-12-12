use devflow_pro::ai::types::{AnalysisType, LlamaConfig};
use devflow_pro::ai::Coder;

#[tokio::main]
async fn main() {
    // Create a test configuration
    let config = LlamaConfig::default();

    // Initialize the Coder
    match Coder::new(config) {
        Ok(llama) => {
            println!("✅ Successfully initialized Coder");

            // Test a simple code analysis
            let test_code = "fn main() { println!(\"Hello, World!\"); }";
            let result = llama
                .analyze_code(test_code, AnalysisType::CodeReview)
                .await;
            match result {
                Ok(analysis) => println!("Analysis result: {analysis:#?}"),
                Err(e) => println!("❌ API call failed: {e}"),
            }
        }
        Err(e) => println!("❌ Failed to initialize Coder: {e}"),
    }
}
