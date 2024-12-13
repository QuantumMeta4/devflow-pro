use devflow_pro::windsurf::{AnalysisContext, Position, WindsurfConfig, WindsurfPlugin};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration
    let config = WindsurfConfig::new(
        4,
        "codellama/CodeLlama-34b-Instruct-hf".to_string(),
        "https://api.together.xyz".to_string(),
        vec![],
        0.7,
        true,
        true,
    );

    // Create plugin instance
    println!("Initializing Windsurf plugin...");
    let plugin = WindsurfPlugin::new(Some(config)).await?;

    // Read the test file
    let file_path = PathBuf::from("examples/test_analysis.rs");
    let code_content = std::fs::read_to_string(&file_path)?;

    // Create analysis context
    let context = AnalysisContext {
        file_path: file_path.to_string_lossy().to_string(),
        code_content: code_content.clone(),
        cursor_position: Some(0),
        visible_range: Some((0, code_content.lines().count())),
        language: "rust".to_string(),
    };

    // Perform analysis
    println!("\nAnalyzing code...");
    let analysis_result = plugin.analyze(context).await?;

    // Print analysis results
    println!("\n=== Analysis Results ===");
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

    // Get suggestions for specific positions
    println!("\n=== Getting suggestions for specific positions ===");

    // Test unwrap position
    let unwrap_position = Position {
        line: 5,
        column: 51,
        offset: code_content.find("unwrap").unwrap_or(0) as u32,
    };

    println!("\nSuggestions at first unwrap:");
    let suggestions = plugin.get_suggestions_at(unwrap_position).await?;
    for suggestion in &suggestions {
        println!("\nCategory: {:?}", suggestion.category);
        println!("Confidence: {}", suggestion.confidence);
        println!("Suggestion: {}", suggestion.suggestion);
        if let Some(snippet) = &suggestion.code_snippet {
            println!("Code Snippet: {}", snippet);
        }
    }

    Ok(())
}
