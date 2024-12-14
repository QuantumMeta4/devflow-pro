use anyhow::Result;
use async_trait::async_trait;
use devflow_pro::windsurf::{
    interface::{AnalysisContext, IDEInterface, WindsurfIntegration},
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

#[async_trait]
impl WindsurfIntegration for TestIntegration {
    async fn initialize(&self) -> Result<()> {
        println!("Initializing test integration");
        Ok(())
    }
}

#[tokio::test]
async fn test_integration() -> Result<()> {
    let integration = TestIntegration::new();
    integration.initialize().await?;

    let context = AnalysisContext {
        content: String::from("fn main() {}"),
        position: Some(Position {
            line: 0,
            character: 0,
        }),
        file_path: "test.rs".into(),
        visible_range: None,
    };

    integration.handle_text_change(&context.content).await?;
    integration
        .handle_cursor_move(context.position.unwrap())
        .await?;

    Ok(())
}
