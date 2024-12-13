use devflow_pro::windsurf::{
    interface::WindsurfIntegration, test_utils::new_windsurf_plugin_with_mock, AnalysisContext,
    WindsurfInterface,
};
use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize Windsurf integration with mock plugin
    let mut integration =
        WindsurfIntegration::new_with_plugin(new_windsurf_plugin_with_mock().await?).await?;

    // Get the current file path and content
    let current_file = PathBuf::from("/Users/meta4ickal/devflow-pro/examples/windsurf_ide.rs");
    let current_content = std::fs::read_to_string(&current_file)?;

    // Set current file for analysis
    integration.set_current_file(current_file.clone());

    println!("Starting Windsurf analysis of current file...");

    // Handle text change and wait for analysis
    integration
        .handle_text_change(current_content.clone())
        .await?;

    // Give it a moment to process
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Get analysis results
    let plugin = integration.get_plugin();

    // Get status bar metrics
    let status = plugin.get_status_bar_text().await?;
    println!("Analysis Status: {}", status);

    // Get detailed analysis results
    let context = AnalysisContext {
        file_path: current_file.to_string_lossy().into_owned(),
        code_content: current_content,
        cursor_position: Some(12), // Current cursor position
        visible_range: None,
        language: "rs".to_string(),
    };

    let analysis_result = plugin.analyze(context).await?;

    // Return analysis results as prompt
    println!("\nWindsurf Analysis Results");
    println!("========================");
    println!("\nFile: {}", current_file.display());
    println!("\nMetrics:");
    println!(
        "- Code Quality Score: {:.2}",
        analysis_result.ai_insights.code_quality_score
    );
    println!(
        "- Semantic Complexity: {:.2}",
        analysis_result.ai_insights.semantic_complexity
    );

    println!("\nOptimization Suggestions:");
    if analysis_result
        .ai_insights
        .optimization_suggestions
        .is_empty()
    {
        println!("No optimization suggestions - code looks good!");
    } else {
        for suggestion in &analysis_result.ai_insights.optimization_suggestions {
            println!(
                "- {} (Impact: {:.2})",
                suggestion.description, suggestion.impact_score
            );
            if let Some(impl_suggestion) = &suggestion.suggested_implementation {
                println!("  Suggestion: {}", impl_suggestion);
            }
        }
    }

    println!("\nSecurity Recommendations:");
    if analysis_result
        .ai_insights
        .security_recommendations
        .is_empty()
    {
        println!("No security issues found!");
    } else {
        for recommendation in &analysis_result.ai_insights.security_recommendations {
            println!(
                "- {} (Severity: {:?}, Confidence: {:.2})",
                recommendation.description, recommendation.severity, recommendation.confidence
            );
            if let Some(fix) = &recommendation.suggested_fix {
                println!("  Suggested Fix: {}", fix);
            }
        }
    }

    Ok(())
}
