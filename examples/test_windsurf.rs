use devflow_pro::windsurf::{
    interface::{AnalysisContext, WindsurfIntegrationImpl},
    Plugin, Position,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let integration = WindsurfIntegrationImpl::new(Plugin::default())?;

    let test_code = r"
        fn calculate_sum(numbers: &[i32]) -> i32 {
            numbers.iter().sum()
        }
    ";

    let mut context = AnalysisContext {
        content: test_code.to_string(),
        position: Some(Position {
            line: 1,
            character: 0,
        }),
        file_path: "test.rs".into(),
        visible_range: None,
    };

    integration.analyze(&mut context).await?;
    Ok(())
}
