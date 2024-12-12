use devflow_pro::ai::llama::LlamaCoder;
use devflow_pro::ai::types::{LlamaConfig, AnalysisType};

#[tokio::main]
async fn main() {
    // Create a test configuration
    let config = LlamaConfig {
        model_name: "mistralai/Mistral-7B-Instruct-v0.1".to_string(),
        context_length: 4096,
        temperature: 0.7,
        top_p: 0.95,
        max_tokens: 100,
        stop_sequences: vec!["```".to_string()],
    };

    // Initialize the LlamaCoder
    match LlamaCoder::new(config).await {
        Ok(llama) => {
            println!("✅ Successfully initialized LlamaCoder");
            
            // Test simple code analysis
            let test_code = "fn hello() { println!(\"Hello, World!\"); }";
            match llama.analyze_code(test_code, AnalysisType::CodeReview).await {
                Ok(result) => {
                    println!("✅ API call successful!");
                    println!("Confidence: {}", result.confidence);
                    println!("\nSuggestions:");
                    for suggestion in result.suggestions {
                        println!("- {}", suggestion);
                    }
                }
                Err(e) => println!("❌ API call failed: {}", e),
            }
        }
        Err(e) => println!("❌ Failed to initialize LlamaCoder: {}", e),
    }
}
