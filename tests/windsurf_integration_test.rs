use anyhow::Result;
use async_trait::async_trait;
use devflow_pro::windsurf::{
    interface::{AnalysisContext, Interface, WindsurfIntegration},
    Config, Plugin, Position, Range,
};
use std::path::PathBuf;
use std::sync::Mutex;

struct TestIntegration {
    plugin: Plugin,
    config: Mutex<Config>,
    current_file: Mutex<Option<PathBuf>>,
}

impl TestIntegration {
    fn new() -> Self {
        Self {
            plugin: Plugin::default(),
            config: Mutex::new(Config::default()),
            current_file: Mutex::new(None),
        }
    }
}

#[async_trait]
impl Interface for TestIntegration {
    async fn handle_text_change(&self, content: &str) -> Result<()> {
        println!("Handling text change: {content}");
        Ok(())
    }

    async fn handle_cursor_move(&self, line: u32, character: u32) -> Result<()> {
        println!("Handling cursor move: line {line}, character {character}");
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

    fn get_plugin(&self) -> &Plugin {
        &self.plugin
    }

    async fn get_current_file(&self) -> Result<Option<PathBuf>> {
        Ok(self.current_file.lock().unwrap().clone())
    }

    async fn set_current_file(&self, path: Option<PathBuf>) -> Result<()> {
        *self.current_file.lock().unwrap() = path;
        Ok(())
    }

    async fn get_config(&self) -> Result<Config> {
        Ok(self.config.lock().unwrap().clone())
    }

    async fn set_config(&self, config: Config) -> Result<()> {
        *self.config.lock().unwrap() = config;
        Ok(())
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

    let test_code = r"
        fn calculate_sum(numbers: &[i32]) -> i32 {
            numbers.iter().sum()
        }
    ";

    let context = AnalysisContext {
        content: test_code.to_string(),
        position: Some(Position {
            line: 1,
            character: 0,
        }),
        file_path: "test.rs".into(),
        visible_range: None,
    };

    integration.handle_text_change(&context.content).await?;
    if let Some(pos) = context.position {
        integration
            .handle_cursor_move(pos.line, pos.character)
            .await?;
    }

    Ok(())
}
