use devflow_pro::{
    windsurf::{AnalysisContext, IDEInterface, Position, WindsurfIntegration},
    Result,
};
use std::path::PathBuf;

#[tokio::test]
async fn test_real_windsurf_analysis() -> Result<()> {
    let integration = WindsurfIntegration::new().await?;
    let context = AnalysisContext {
        code_content: String::from("fn main() {}"),
        file_path: PathBuf::from("test.rs").to_string_lossy().into_owned(),
        cursor_position: Some(0),
        visible_range: None,
        language: "rust".to_string(),
    };

    integration
        .handle_text_change(context.code_content.clone())
        .await?;
    integration
        .handle_cursor_move(Position {
            line: 0,
            column: 0,
            offset: 0,
        })
        .await?;

    Ok(())
}
