use devflow_pro::windsurf::{
    interface::{AnalysisContext, Interface},
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

#[async_trait::async_trait]
impl Interface for TestIntegration {
    async fn handle_text_change(&self, content: &str) -> anyhow::Result<()> {
        println!("Handling text change: {content}");
        Ok(())
    }

    async fn handle_cursor_move(&self, line: u32, character: u32) -> anyhow::Result<()> {
        println!("Handling cursor move: ({line}, {character})");
        Ok(())
    }

    async fn handle_visible_range_change(&self, range: Range) -> anyhow::Result<()> {
        println!("Handling visible range change: {range:?}");
        Ok(())
    }

    fn get_plugin(&self) -> &Plugin {
        &self.plugin
    }

    async fn get_current_file(&self) -> anyhow::Result<Option<PathBuf>> {
        Ok(self.current_file.lock().unwrap().clone())
    }

    async fn set_current_file(&self, path: Option<PathBuf>) -> anyhow::Result<()> {
        *self.current_file.lock().unwrap() = path;
        Ok(())
    }

    async fn get_config(&self) -> anyhow::Result<Config> {
        Ok(self.config.lock().unwrap().clone())
    }

    async fn set_config(&self, config: Config) -> anyhow::Result<()> {
        *self.config.lock().unwrap() = config;
        Ok(())
    }

    async fn toggle_real_time_analysis(&self) -> anyhow::Result<()> {
        println!("Toggling real-time analysis");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let integration = TestIntegration::new();

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
        visible_range: Some(Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: 4,
                character: 0,
            },
        }),
    };

    integration.handle_text_change(&context.content).await?;
    if let Some(pos) = context.position {
        integration
            .handle_cursor_move(pos.line, pos.character)
            .await?;
    }
    if let Some(range) = context.visible_range {
        integration.handle_visible_range_change(range).await?;
    }

    Ok(())
}
