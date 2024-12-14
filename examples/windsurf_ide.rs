use devflow_pro::windsurf::{AnalysisContext, IDEInterface, Position, WindsurfIntegration};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new IDE integration
    let integration = WindsurfIntegration::new().await?;

    // Example context
    let context = AnalysisContext {
        code_content: String::from("fn main() { println!(\"Hello, World!\"); }"),
        file_path: PathBuf::from("example.rs").to_string_lossy().into_owned(),
        cursor_position: Some(0),
        visible_range: None,
        language: "rust".to_string(),
    };

    // Handle text change
    integration
        .handle_text_change(context.code_content.clone())
        .await?;

    // Handle cursor move
    integration
        .handle_cursor_move(Position {
            line: 0,
            column: 0,
            offset: 0,
        })
        .await?;

    // Update status bar
    integration
        .update_status_bar(
            "windsurf",
            "DevFlow Pro Ready".to_string(),
            Some("Click for more information".to_string()),
        )
        .await?;

    Ok(())
}
