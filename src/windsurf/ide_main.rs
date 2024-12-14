use super::interface::WindsurfIntegration;
use super::{
    ide::{commands, IDE},
    Position, Range,
};
use crate::Result;
use std::sync::Arc;

/// Main IDE implementation that provides the interface between Windsurf and the editor.
///
/// This struct manages the IDE context and handles all editor-related events such as
/// text changes, cursor movements, and visible range updates.
pub struct WindsurfIDE {
    context: Arc<IDE>,
}

impl WindsurfIDE {
    /// Creates a new `WindsurfIDE` instance.
    ///
    /// # Errors
    ///
    /// Returns an error if initialization fails.
    pub async fn new() -> Result<Self> {
        // Initialize Windsurf interface
        let windsurf = Arc::new(WindsurfIntegration::new().await?);

        // Create IDE context
        let context = Arc::new(IDE::new(windsurf));

        let ide = Self { context };
        ide.register_commands()?;
        Ok(ide)
    }

    /// Starts the IDE interface.
    ///
    /// # Errors
    ///
    /// Returns an error if startup fails.
    pub fn start(&self) -> Result<()> {
        // In a real IDE, we'd initialize the UI and start listening for events
        println!("Starting Windsurf IDE...");
        Ok(())
    }

    /// Registers IDE commands with the command system.
    ///
    /// This function sets up command handlers for features like showing metrics
    /// and managing real-time analysis.
    ///
    /// # Errors
    ///
    /// Returns an error if command registration fails.
    fn register_commands(&self) -> Result<()> {
        // In a real IDE, we'd register these with the IDE's command system
        let context = self.context.clone();

        tokio::spawn(async move {
            if let Err(e) = commands::show_metrics_details(context.clone()).await {
                eprintln!("Failed to show metrics: {e}");
            }
        });

        Ok(())
    }

    /// Handles text changes in the editor.
    ///
    /// # Errors
    ///
    /// Returns an error if the text change cannot be processed.
    pub async fn handle_text_change(&self, text: String) -> Result<()> {
        self.context.handle_text_change(text).await
    }

    /// Handles cursor movement in the editor.
    ///
    /// # Errors
    ///
    /// Returns an error if the cursor position cannot be updated.
    pub async fn handle_cursor_move(&self, position: Position) -> Result<()> {
        self.context.handle_cursor_move(position).await
    }

    /// Handles visible range changes in the editor.
    ///
    /// # Errors
    ///
    /// Returns an error if the visible range cannot be updated.
    pub async fn handle_visible_range_change(&self, range: Range) -> Result<()> {
        self.context.handle_visible_range_change(range).await
    }
}
