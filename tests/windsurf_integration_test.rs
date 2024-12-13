use devflow_pro::{
    ai::types::AnalysisType,
    windsurf::{AnalysisContext, Position, WindsurfConfig, WindsurfPlugin},
    Result,
};
use std::fs;
use tempfile::tempdir;
use tokio;

#[tokio::test]
async fn test_real_windsurf_analysis() -> Result<()> {
    // Create a temporary directory and test file
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let test_file_path = temp_dir.path().join("test.rs");

    // This is a sample code that should trigger various suggestions
    let test_code = r#"
    fn process_data(data: &str) -> String {
        // Potential security issue: using unwrap
        let parsed = data.parse::<i32>().unwrap();
        
        // Potential performance issue: unnecessary string allocation
        let result = String::from(data) + &parsed.to_string();
        
        // Potential code quality issue: no error handling
        std::fs::write("output.txt", &result).unwrap();
        
        result
    }
    "#;

    fs::write(&test_file_path, test_code).expect("Failed to write test file");

    // Create configuration for real API
    let config = WindsurfConfig::new(
        4,
        "codellama/CodeLlama-34b-Instruct-hf".to_string(),
        "https://api.together.xyz".to_string(),
        vec![
            AnalysisType::CodeReview,
            AnalysisType::SecurityAudit,
            AnalysisType::Optimization,
        ],
        0.7,
        true,
        true,
    );

    // Create plugin instance with real API
    let plugin = WindsurfPlugin::new(Some(config)).await?;

    // Test file analysis
    let context = AnalysisContext {
        file_path: test_file_path.to_string_lossy().to_string(),
        code_content: test_code.to_string(),
        cursor_position: Some(0),
        visible_range: Some((0, test_code.lines().count())),
        language: "rust".to_string(),
    };

    println!("Starting analysis...");
    let analysis_result = plugin.analyze(context).await?;

    // Print analysis results
    println!("\nAnalysis Results:");
    println!(
        "Code Quality Score: {}",
        analysis_result.ai_insights.code_quality_score
    );
    println!(
        "Semantic Complexity: {}",
        analysis_result.ai_insights.semantic_complexity
    );

    println!("\nOptimization Suggestions:");
    for suggestion in &analysis_result.ai_insights.optimization_suggestions {
        println!("- {}", suggestion.description);
    }

    println!("\nSecurity Recommendations:");
    for suggestion in &analysis_result.ai_insights.security_recommendations {
        println!("- {}", suggestion.description);
    }

    // Test getting suggestions at a specific position
    let position = Position {
        line: 3, // Line with unwrap
        column: 0,
        offset: test_code.find("unwrap").unwrap_or(0) as u32,
    };

    println!("\nGetting suggestions at position...");
    let suggestions = plugin.get_suggestions_at(position).await?;

    println!("\nContext Suggestions:");
    for suggestion in &suggestions {
        println!("Category: {:?}", suggestion.category);
        println!("Confidence: {}", suggestion.confidence);
        println!("Suggestion: {}", suggestion.suggestion);
        if let Some(snippet) = &suggestion.code_snippet {
            println!("Code Snippet: {}", snippet);
        }
        println!();
    }

    Ok(())
}
