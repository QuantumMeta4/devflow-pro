use super::{Position, Range, WindsurfPlugin};
use crate::Result;
use async_trait::async_trait;
use std::path::PathBuf;

#[async_trait]
pub trait WindsurfInterface: Send + Sync + std::fmt::Debug {
    async fn handle_text_change(&self, text: String) -> Result<()>;
    async fn handle_cursor_move(&self, position: Position) -> Result<()>;
    async fn handle_visible_range_change(&self, range: Range) -> Result<()>;
    fn get_plugin(&self) -> &WindsurfPlugin;
}

#[derive(Debug, Clone)]
pub struct WindsurfIntegration {
    plugin: WindsurfPlugin,
    current_file: PathBuf,
}

impl WindsurfIntegration {
    pub async fn new() -> Result<Self> {
        let plugin = WindsurfPlugin::new(None).await?;
        Ok(Self {
            plugin,
            current_file: PathBuf::new(),
        })
    }

    pub async fn new_with_plugin(plugin: WindsurfPlugin) -> Result<Self> {
        Ok(Self {
            plugin,
            current_file: PathBuf::new(),
        })
    }

    pub fn get_plugin(&self) -> &WindsurfPlugin {
        &self.plugin
    }

    pub fn set_current_file(&mut self, path: PathBuf) {
        self.current_file = path;
    }
}

#[async_trait]
impl WindsurfInterface for WindsurfIntegration {
    async fn handle_text_change(&self, text: String) -> Result<()> {
        // Create context with new content
        let context = super::AnalysisContext {
            file_path: self.current_file.to_string_lossy().into_owned(),
            code_content: text,
            cursor_position: None,
            visible_range: None,
            language: self
                .current_file
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("unknown")
                .to_string(),
        };

        // Trigger reanalysis
        self.plugin.analyze(context).await?;
        Ok(())
    }

    async fn handle_cursor_move(&self, position: Position) -> Result<()> {
        self.plugin.update_cursor_position(position).await
    }

    async fn handle_visible_range_change(&self, _range: Range) -> Result<()> {
        // Store visible range for context-aware analysis
        Ok(())
    }

    fn get_plugin(&self) -> &WindsurfPlugin {
        &self.plugin
    }
}
