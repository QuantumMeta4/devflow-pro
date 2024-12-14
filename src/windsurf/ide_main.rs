use crate::windsurf::interface::{Command, Integration, Position, Range};
use anyhow::Result;
use std::sync::Arc;

/// Plugin for IDE integration.
#[derive(Debug, Clone)]
pub struct Plugin {
    pub name: String,
    pub version: String,
}

impl Default for Plugin {
    fn default() -> Self {
        Self {
            name: "Windsurf".to_string(),
            version: "0.1.0".to_string(),
        }
    }
}

/// The main IDE struct that handles integration with the editor.
pub struct WindsurfIDE {
    context: Arc<dyn Integration>,
}

impl WindsurfIDE {
    /// Creates a new `WindsurfIDE` instance.
    ///
    /// # Errors
    ///
    /// Returns an error if IDE initialization fails.
    #[must_use = "This function returns a new WindsurfIDE instance that should be used"]
    pub fn new(windsurf: impl Into<Arc<dyn Integration>>) -> Result<Self> {
        let context = windsurf.into();
        let ide = Self { context };
        ide.register_commands();
        Ok(ide)
    }

    /// Registers IDE commands.
    ///
    /// This function sets up command handlers for features like showing metrics
    /// and managing real-time analysis.
    fn register_commands(&self) {
        // In a real IDE, we'd register these with the IDE's command system
        let context = self.context.clone();
        let _toggle_command = Command::new("windsurf.toggleRealTimeAnalysis", move || {
            let context = context.clone();
            Box::pin(async move {
                if let Err(e) = context.toggle_real_time_analysis().await {
                    eprintln!("Failed to toggle real-time analysis: {e}");
                }
            })
        });
    }

    /// Handles text changes in the editor.
    ///
    /// # Errors
    ///
    /// Returns an error if handling text changes fails.
    pub async fn handle_text_change(&self, content: &str) -> Result<()> {
        self.context.handle_text_change(content).await
    }

    /// Handles cursor movement in the editor.
    ///
    /// # Errors
    ///
    /// Returns an error if handling cursor movement fails.
    pub async fn handle_cursor_move(&self, position: Position) -> Result<()> {
        self.context
            .handle_cursor_move(position.line, position.character)
            .await
    }

    /// Handles visible range changes in the editor.
    ///
    /// # Errors
    ///
    /// Returns an error if handling range changes fails.
    pub async fn handle_visible_range_change(&self, range: Range) -> Result<()> {
        self.context.handle_visible_range_change(range).await
    }
}
