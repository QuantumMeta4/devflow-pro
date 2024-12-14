use crate::windsurf::{Position, Range};
use anyhow::Result;
use async_trait::async_trait;
use std::{future::Future, path::PathBuf, pin::Pin};
use tokio::sync::Mutex;

/// Interface for IDE command handlers.
pub type CommandHandler = Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Command structure for IDE commands.
pub struct Command {
    pub id: String,
    pub handler: CommandHandler,
}

impl Command {
    /// Creates a new command with the given ID and handler.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        handler: impl Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            handler: Box::new(handler),
        }
    }
}

/// Analysis context for Windsurf operations.
#[derive(Debug, Clone)]
pub struct AnalysisContext {
    /// Path to the file being analyzed.
    pub file_path: String,
    /// Content of the file.
    pub content: String,
    /// Current cursor position.
    pub position: Option<Position>,
    /// Visible range in the editor.
    pub visible_range: Option<Range>,
}

/// Interface for IDE functionality.
#[async_trait]
pub trait IDEInterface: Send + Sync {
    /// Handles text changes in the editor.
    async fn handle_text_change(&self, content: &str) -> Result<()>;

    /// Handles cursor movement in the editor.
    async fn handle_cursor_move(&self, position: Position) -> Result<()>;

    /// Handles visible range changes in the editor.
    async fn handle_visible_range_change(&self, range: Range) -> Result<()>;

    /// Toggles real-time analysis.
    async fn toggle_real_time_analysis(&self) -> Result<()>;

    /// Get the current plugin instance.
    #[must_use]
    fn get_plugin(&self) -> &super::WindsurfPlugin;
}

/// Windsurf integration trait.
#[async_trait]
pub trait WindsurfIntegration: IDEInterface {
    /// Initializes the Windsurf integration.
    async fn initialize(&self) -> Result<()>;
}

/// Windsurf IDE integration implementation.
pub struct WindsurfIntegrationImpl {
    pub config: Mutex<super::WindsurfConfig>,
    pub current_file: Mutex<Option<PathBuf>>,
    plugin: super::WindsurfPlugin,
}

#[async_trait]
impl IDEInterface for WindsurfIntegrationImpl {
    async fn handle_text_change(&self, content: &str) -> Result<()> {
        let file_path = self
            .current_file
            .lock()
            .await
            .as_ref()
            .map_or_else(String::new, |p| p.to_string_lossy().into_owned());

        let mut analysis_ctx = AnalysisContext {
            file_path,
            content: content.to_string(),
            position: None,
            visible_range: None,
        };

        self.analyze(&mut analysis_ctx).await
    }

    async fn handle_cursor_move(&self, position: Position) -> Result<()> {
        let file_path = self
            .current_file
            .lock()
            .await
            .as_ref()
            .map_or_else(String::new, |p| p.to_string_lossy().into_owned());

        let mut analysis_ctx = AnalysisContext {
            file_path,
            content: String::new(),
            position: Some(position),
            visible_range: None,
        };

        self.analyze(&mut analysis_ctx).await
    }

    async fn handle_visible_range_change(&self, range: Range) -> Result<()> {
        let file_path = self
            .current_file
            .lock()
            .await
            .as_ref()
            .map_or_else(String::new, |p| p.to_string_lossy().into_owned());

        let mut analysis_ctx = AnalysisContext {
            file_path,
            content: String::new(),
            position: None,
            visible_range: Some(range),
        };

        self.analyze(&mut analysis_ctx).await
    }

    async fn toggle_real_time_analysis(&self) -> Result<()> {
        let mut config = self.config.lock().await;
        config.real_time_enabled = !config.real_time_enabled;
        drop(config);
        Ok(())
    }

    fn get_plugin(&self) -> &super::WindsurfPlugin {
        &self.plugin
    }
}

#[async_trait]
impl WindsurfIntegration for WindsurfIntegrationImpl {
    async fn initialize(&self) -> Result<()> {
        // Initialize any required resources
        Ok(())
    }
}

impl WindsurfIntegrationImpl {
    /// Creates a new Windsurf integration instance.
    ///
    /// # Errors
    ///
    /// Returns an error if initialization fails.
    pub fn new(plugin: super::WindsurfPlugin) -> Result<Self> {
        Ok(Self {
            config: Mutex::new(super::WindsurfConfig::default()),
            current_file: Mutex::new(None),
            plugin,
        })
    }

    /// Analyzes the current context.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Failed to acquire lock on configuration
    /// - Analysis operation fails
    pub async fn analyze(&self, _context: &mut AnalysisContext) -> Result<()> {
        if !self.config.lock().await.real_time_enabled {
            return Ok(());
        }
        Ok(())
    }
}
