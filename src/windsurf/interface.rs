use super::{Config, Plugin};
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
pub trait Interface: Send + Sync {
    /// Handles text changes in the editor.
    async fn handle_text_change(&self, content: &str) -> Result<()>;

    /// Handles cursor movement in the editor.
    async fn handle_cursor_move(&self, line: u32, character: u32) -> Result<()>;

    /// Gets the current file path.
    async fn get_current_file(&self) -> Result<Option<PathBuf>>;

    /// Sets the current file path.
    async fn set_current_file(&self, path: Option<PathBuf>) -> Result<()>;

    /// Gets the current configuration.
    async fn get_config(&self) -> Result<Config>;

    /// Sets the current configuration.
    async fn set_config(&self, config: Config) -> Result<()>;

    /// Gets the plugin instance.
    fn get_plugin(&self) -> &Plugin;

    /// Toggles real-time analysis.
    async fn toggle_real_time_analysis(&self) -> Result<()>;

    /// Handles visible range changes in the editor.
    async fn handle_visible_range_change(&self, range: Range) -> Result<()>;
}

/// Windsurf integration trait.
#[async_trait]
pub trait WindsurfIntegration: Interface {
    /// Initializes the Windsurf integration.
    async fn initialize(&self) -> Result<()>;
}

/// Windsurf IDE integration implementation.
pub struct WindsurfIntegrationImpl {
    pub config: Mutex<Config>,
    pub current_file: Mutex<Option<PathBuf>>,
    plugin: Plugin,
}

#[async_trait]
impl Interface for WindsurfIntegrationImpl {
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

    async fn handle_cursor_move(&self, line: u32, character: u32) -> Result<()> {
        let file_path = self
            .current_file
            .lock()
            .await
            .as_ref()
            .map_or_else(String::new, |p| p.to_string_lossy().into_owned());

        let mut analysis_ctx = AnalysisContext {
            file_path,
            content: String::new(),
            position: Some(Position { line, character }),
            visible_range: None,
        };

        self.analyze(&mut analysis_ctx).await
    }

    async fn get_current_file(&self) -> Result<Option<PathBuf>> {
        Ok(self.current_file.lock().await.clone())
    }

    async fn set_current_file(&self, path: Option<PathBuf>) -> Result<()> {
        *self.current_file.lock().await = path;
        Ok(())
    }

    async fn get_config(&self) -> Result<Config> {
        Ok(self.config.lock().await.clone())
    }

    async fn set_config(&self, config: Config) -> Result<()> {
        *self.config.lock().await = config;
        Ok(())
    }

    fn get_plugin(&self) -> &Plugin {
        &self.plugin
    }

    async fn toggle_real_time_analysis(&self) -> Result<()> {
        let mut config = self.config.lock().await;
        config.real_time_enabled = !config.real_time_enabled;
        drop(config);
        Ok(())
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
    pub fn new(plugin: Plugin) -> Result<Self> {
        Ok(Self {
            config: Mutex::new(Config::default()),
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
