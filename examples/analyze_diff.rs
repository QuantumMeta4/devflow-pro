use devflow_pro::windsurf::{AnalysisContext, WindsurfConfig, WindsurfPlugin};
use std::process::Command;
use std::str;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Windsurf plugin
    println!("Initializing Windsurf plugin...");
    let config = WindsurfConfig::new(
        4,
        "codellama/CodeLlama-34b-Instruct-hf".to_string(),
        "https://api.together.xyz".to_string(),
        vec![],
        0.7,
        true,
        true,
    );
    let plugin = WindsurfPlugin::new(Some(config)).await?;

    // Get git diff
    let output = Command::new("git").args(["diff", "HEAD~1"]).output()?;
    let diff_output = str::from_utf8(&output.stdout)?;

    // Parse diff to get modified files and their changes
    let mut current_file = String::new();
    let mut current_content = String::new();
    let mut files_to_analyze = Vec::new();

    for line in diff_output.lines() {
        if line.starts_with("diff --git") {
            // Save previous file if exists
            if !current_file.is_empty() && !current_content.is_empty() {
                files_to_analyze.push((current_file.clone(), current_content.clone()));
            }
            // Extract new filename
            let parts: Vec<&str> = line.split(' ').collect();
            current_file = parts[2][2..].to_string(); // Remove 'a/' prefix
            current_content.clear();
        } else if line.starts_with('+') && !line.starts_with("+++") {
            // Collect added/modified lines
            current_content.push_str(&line[1..]);
            current_content.push('\n');
        }
    }
    // Add last file
    if !current_file.is_empty() && !current_content.is_empty() {
        files_to_analyze.push((current_file.clone(), current_content.clone()));
    }

    // Analyze each modified file
    for (file_path, content) in files_to_analyze {
        if file_path.ends_with(".rs") || file_path.ends_with(".toml") {
            println!("\n=== Analyzing changes in {} ===", file_path);

            let context = AnalysisContext {
                file_path: file_path.clone(),
                code_content: content,
                cursor_position: None,
                visible_range: None,
                language: if file_path.ends_with(".rs") {
                    "rust"
                } else {
                    "toml"
                }
                .to_string(),
            };

            match plugin.analyze(context).await {
                Ok(analysis_result) => {
                    println!("\nAnalysis Results:");
                    println!(
                        "Code Quality Score: {}",
                        analysis_result.ai_insights.code_quality_score
                    );
                    println!(
                        "Semantic Complexity: {}",
                        analysis_result.ai_insights.semantic_complexity
                    );

                    if !analysis_result
                        .ai_insights
                        .optimization_suggestions
                        .is_empty()
                    {
                        println!("\nOptimization Suggestions:");
                        for suggestion in &analysis_result.ai_insights.optimization_suggestions {
                            println!("- {}", suggestion.description);
                        }
                    }

                    if !analysis_result
                        .ai_insights
                        .security_recommendations
                        .is_empty()
                    {
                        println!("\nSecurity Recommendations:");
                        for suggestion in &analysis_result.ai_insights.security_recommendations {
                            println!("- {}", suggestion.description);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error analyzing {}: {}", file_path, e);
                }
            }
        }
    }

    Ok(())
}
