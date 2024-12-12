use devflow_pro::ai::llama::LlamaCoder;
use devflow_pro::ai::types::{AnalysisType, LlamaConfig};

#[tokio::main]
async fn main() {
    println!("🔍 Testing AI Code Analysis Integration\n");

    // Create test configuration
    let config = LlamaConfig::default();

    println!("1️⃣ Testing Code Review");
    println!("────────────────────");

    // Initialize the LlamaCoder
    match LlamaCoder::new(config.clone()).await {
        Ok(llama) => {
            // Test complex code sample
            let test_code = r#"
            fn fibonacci(n: u32) -> Result<u32, String> {
                if n >= 93 {
                    return Err("Value too large for u32".to_string());
                }
                if n <= 1 {
                    return Ok(n);
                }
                match (fibonacci(n - 1), fibonacci(n - 2)) {
                    (Ok(a), Ok(b)) => Ok(a + b),
                    (Err(e), _) | (_, Err(e)) => Err(e),
                }
            }
            
            fn main() {
                let results: Vec<_> = (0..100)
                    .map(|n| fibonacci(n))
                    .take_while(|r| r.is_ok())
                    .collect();
                println!("Fibonacci sequence: {:?}", results);
            }
            "#;

            // Test different types of analysis
            let analysis_types = vec![
                (AnalysisType::CodeReview, "Code Review"),
                (AnalysisType::SecurityAudit, "Security Audit"),
                (AnalysisType::Optimization, "Optimization"),
            ];

            for (analysis_type, label) in analysis_types {
                println!("\n🔍 Running {}", label);
                match llama.analyze_code(test_code, analysis_type).await {
                    Ok(result) => {
                        println!("✅ Analysis successful");
                        println!("Confidence: {:.2}", result.confidence);
                        println!("\nSuggestions:");
                        for suggestion in result.suggestions {
                            println!("- {}", suggestion);
                        }
                    }
                    Err(e) => println!("❌ Analysis failed: {}", e),
                }
            }
        }
        Err(e) => println!("❌ Failed to initialize LlamaCoder: {}", e),
    }
}
