use devflow_pro::windsurf::{
    interface::{AnalysisContext, WindsurfIntegrationImpl},
    Position, WindsurfPlugin,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let plugin = WindsurfPlugin::default();
    let integration = WindsurfIntegrationImpl::new(plugin)?;

    let code_content = r#"
        fn main() {
            let result = vec![1, 2, 3].iter().sum::<i32>();
            println!("{}", result);
        }
    "#;

    let mut context = AnalysisContext {
        content: code_content.to_string(),
        position: Some(Position {
            line: 0,
            character: 0,
        }),
        file_path: "test.rs".into(),
        visible_range: None,
    };

    integration.analyze(&mut context).await?;
    println!("Analysis complete");

    Ok(())
}
