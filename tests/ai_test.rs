#[cfg(test)]
mod tests {
    use devflow_pro::{
        ai_enhanced::{AIProvider, CodeLLamaProvider},
        Result,
    };
    use env_logger;
    use std::env;

    #[tokio::test]
    #[ignore = "Requires TOGETHER_API_KEY environment variable"]
    async fn test_codellama_analysis() -> Result<()> {
        env::set_var("RUST_LOG", "debug");
        env_logger::init_from_env("RUST_LOG");

        // Skip test if API key is not set
        let api_key = match env::var("TOGETHER_API_KEY") {
            Ok(key) => key,
            Err(_) => return Ok(()),
        };

        // Initialize provider
        let provider = CodeLLamaProvider::new(
            &api_key,
            "https://api.together.xyz",
            "codellama/CodeLlama-34b-Instruct-hf",
            4,
        );

        // Read test code
        let code = include_str!("test_code.rs");

        // Analyze code
        let result = provider.analyze_code(code).await?;

        // Print analysis results
        println!("Code Quality Score: {}", result.code_quality_score);
        println!("Semantic Complexity: {}", result.semantic_complexity);

        println!("\nSecurity Recommendations:");
        for rec in &result.security_recommendations {
            println!(
                "- [{:?}] {} (Confidence: {})",
                rec.severity, rec.description, rec.confidence
            );
            if let Some(fix) = &rec.suggested_fix {
                println!("  Fix: {}", fix);
            }
        }

        println!("\nOptimization Suggestions:");
        for opt in &result.optimization_suggestions {
            println!(
                "- [{:?}] {} (Impact: {})",
                opt.category, opt.description, opt.impact_score
            );
            if let Some(implementation) = &opt.suggested_implementation {
                println!("  Implementation: {}", implementation);
            }
        }

        Ok(())
    }
}
