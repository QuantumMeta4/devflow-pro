use devflow_pro::ai::types::{AnalysisType, LlamaConfig};
use devflow_pro::ai::Coder;

#[tokio::main]
async fn main() {
    let config = LlamaConfig::default();
    match Coder::new(config.clone()) {
        Ok(coder) => {
            // Test complex code sample
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

            // Run different types of analysis
            let analyses = vec![
                ("Code Review", AnalysisType::CodeReview),
                ("Bug Finding", AnalysisType::BugFinding),
                ("Security Audit", AnalysisType::SecurityAudit),
                ("Documentation", AnalysisType::Documentation),
                ("Optimization", AnalysisType::Optimization),
            ];

            for (label, analysis_type) in analyses {
                println!("\n🔍 Running {label}");
                match coder.analyze_code(test_code, analysis_type).await {
                    Ok(result) => {
                        println!("✅ Analysis successful!");
                        println!("Confidence: {}", result.confidence);
                        println!("\nSuggestions:");
                        for suggestion in result.suggestions {
                            println!("- {suggestion}");
                        }
                    }
                    Err(e) => println!("❌ Analysis failed: {e}"),
                }
            }
        }
        Err(e) => println!("❌ Failed to initialize Coder: {e}"),
    }
}
