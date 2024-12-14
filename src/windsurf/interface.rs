use super::{Position, Range, WindsurfPlugin};
use crate::{DevFlowError, Result};
use async_trait::async_trait;
use std::path::PathBuf;

/// Interface for Windsurf IDE integration
#[async_trait]
pub trait IDEInterface: Send + Sync + std::fmt::Debug {
    /// Update the status bar with the given id, text, and tooltip
    ///
    /// # Errors
    /// Returns an error if the update fails
    async fn update_status_bar(
        &self,
        id: &str,
        text: String,
        tooltip: Option<String>,
    ) -> Result<()>;

    /// Handle a text change event
    ///
    /// # Errors
    /// Returns an error if handling fails
    async fn handle_text_change(&self, text: String) -> Result<()>;

    /// Handle a cursor move event
    ///
    /// # Errors
    /// Returns an error if handling fails
    async fn handle_cursor_move(&self, position: Position) -> Result<()>;

    /// Handle a visible range change event
    ///
    /// # Errors
    /// Returns an error if handling fails
    async fn handle_visible_range_change(&self, range: Range) -> Result<()>;

    /// Get the current plugin instance
    #[must_use]
    fn get_plugin(&self) -> &WindsurfPlugin;
}

/// Windsurf IDE integration implementation
#[derive(Debug, Clone)]
pub struct WindsurfIntegration {
    plugin: WindsurfPlugin,
    current_file: PathBuf,
}

impl WindsurfIntegration {
    /// Create a new integration instance
    ///
    /// # Errors
    /// Returns an error if plugin initialization fails
    pub async fn new() -> Result<Self> {
        let plugin = WindsurfPlugin::new(None).await?;
        Ok(Self {
            plugin,
            current_file: PathBuf::new(),
        })
    }

    /// Create a new integration with a custom plugin
    ///
    /// # Errors
    /// Returns an error if plugin initialization fails
    pub async fn new_with_plugin(plugin: WindsurfPlugin) -> Result<Self> {
        Ok(Self {
            plugin,
            current_file: PathBuf::new(),
        })
    }

    /// Get the current plugin instance
    #[must_use]
    pub const fn get_plugin(&self) -> &WindsurfPlugin {
        &self.plugin
    }

    /// Set the current file path
    pub fn set_current_file(&mut self, path: PathBuf) {
        self.current_file = path;
    }
}

#[async_trait]
impl IDEInterface for WindsurfIntegration {
    async fn update_status_bar(
        &self,
        _id: &str,
        _text: String,
        _tooltip: Option<String>,
    ) -> Result<()> {
        Ok(())
    }

    async fn handle_text_change(&self, text: String) -> Result<()> {
        // Create context with new content
        let context = super::AnalysisContext {
            code_content: text,
            file_path: self.current_file.to_string_lossy().into_owned(),
            cursor_position: None,
            visible_range: None,
            language: self
                .current_file
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("unknown")
                .to_string(),
        };

        // Trigger analysis
        let _ = self.plugin.analyze(context).await?;
        Ok(())
    }

    async fn handle_cursor_move(&self, position: Position) -> Result<()> {
        // Update cursor position in plugin
        let context = super::AnalysisContext {
            code_content: String::new(),
            file_path: self.current_file.to_string_lossy().into_owned(),
            cursor_position: Some(
                position
                    .line
                    .try_into()
                    .map_err(|_| DevFlowError::InvalidPosition)?,
            ),
            visible_range: None,
            language: self
                .current_file
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("unknown")
                .to_string(),
        };
        let _ = self.plugin.analyze(context).await?;
        Ok(())
    }

    async fn handle_visible_range_change(&self, range: Range) -> Result<()> {
        // Update visible range in plugin
        let context = super::AnalysisContext {
            code_content: String::new(),
            file_path: self.current_file.to_string_lossy().into_owned(),
            cursor_position: None,
            visible_range: Some((
                range
                    .start
                    .line
                    .try_into()
                    .map_err(|_| DevFlowError::InvalidPosition)?,
                range
                    .end
                    .line
                    .try_into()
                    .map_err(|_| DevFlowError::InvalidPosition)?,
            )),
            language: self
                .current_file
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("unknown")
                .to_string(),
        };
        let _ = self.plugin.analyze(context).await?;
        Ok(())
    }

    fn get_plugin(&self) -> &WindsurfPlugin {
        &self.plugin
    }
}
