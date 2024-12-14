use anyhow::Result;
use async_trait::async_trait;
use devflow_pro::windsurf::{
    interface::{AnalysisContext, IDEInterface},
    Position, Range, WindsurfPlugin,
};

struct TestIntegration {
    plugin: WindsurfPlugin,
}

impl TestIntegration {
    fn new() -> Self {
        Self {
            plugin: WindsurfPlugin::default(),
        }
    }
}

#[async_trait]
impl IDEInterface for TestIntegration {
    async fn handle_text_change(&self, content: &str) -> Result<()> {
        println!("Handling text change: {content}");
        Ok(())
    }

    async fn handle_cursor_move(&self, position: Position) -> Result<()> {
        println!("Handling cursor move: {position:?}");
        Ok(())
    }

    async fn handle_visible_range_change(&self, range: Range) -> Result<()> {
        println!("Handling visible range change: {range:?}");
        Ok(())
    }

    async fn toggle_real_time_analysis(&self) -> Result<()> {
        println!("Toggling real-time analysis");
        Ok(())
    }

    fn get_plugin(&self) -> &WindsurfPlugin {
        &self.plugin
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let integration = TestIntegration::new();

    let analysis_ctx = AnalysisContext {
        content: String::from("fn main() {}"),
        position: Some(Position {
            line: 0,
            character: 0,
        }),
        file_path: "test.rs".into(),
        visible_range: None,
    };

    println!("Analysis context: {analysis_ctx:?}");
    integration
        .handle_text_change(&analysis_ctx.content)
        .await?;
    integration
        .handle_cursor_move(analysis_ctx.position.unwrap())
        .await?;

    Ok(())
}
