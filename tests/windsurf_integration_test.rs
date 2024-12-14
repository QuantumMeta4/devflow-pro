use anyhow::Result;
use async_trait::async_trait;
use devflow_pro::windsurf::{
    interface::{Integration, Interface},
    Config, Plugin,
};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

struct TestIntegration {
    config: Arc<Mutex<Config>>,
    current_file: Arc<Mutex<Option<PathBuf>>>,
    plugin: Plugin,
}

impl TestIntegration {
    fn new(plugin: Plugin) -> Self {
        Self {
            config: Arc::new(Mutex::new(Config::default())),
            current_file: Arc::new(Mutex::new(None)),
            plugin,
        }
    }
}

#[async_trait]
impl Interface for TestIntegration {
    async fn handle_text_change(&self, _content: &str) -> Result<()> {
        Ok(())
    }

    async fn handle_cursor_move(&self, _line: u32, _character: u32) -> Result<()> {
        Ok(())
    }

    async fn handle_visible_range_change(
        &self,
        _range: devflow_pro::windsurf::interface::Range,
    ) -> Result<()> {
        Ok(())
    }

    async fn toggle_real_time_analysis(&self) -> Result<()> {
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
impl Integration for TestIntegration {
    async fn initialize(&self) -> Result<()> {
        Ok(())
    }
}

#[tokio::test]
async fn test_integration() -> Result<()> {
    let plugin = Plugin::default();
    let integration = Arc::new(TestIntegration::new(plugin));
    integration.initialize().await?;

    // Test text change handling
    integration.handle_text_change("test content").await?;

    // Test cursor move handling
    integration.handle_cursor_move(0, 0).await?;

    // Test configuration
    let config = Config::default();
    integration.set_config(config).await?;
    integration.get_config().await?;

    // Test file handling
    let path = PathBuf::from("test.rs");
    integration.set_current_file(Some(path.clone())).await?;
    let current_file = integration.get_current_file().await?;
    assert_eq!(current_file.as_ref(), Some(&path));

    Ok(())
}
