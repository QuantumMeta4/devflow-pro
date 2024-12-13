use super::interface::WindsurfIntegration;
use super::{
    ide::{commands, IDEContext},
    Position, Range,
};
use crate::Result;
use std::sync::Arc;

pub struct WindsurfIDE {
    context: Arc<IDEContext>,
}

impl WindsurfIDE {
    pub async fn new() -> Result<Self> {
        // Initialize Windsurf interface
        let windsurf = Arc::new(WindsurfIntegration::new().await?);

        // Create IDE context
        let context = Arc::new(IDEContext::new(windsurf));

        Ok(Self { context })
    }

    pub async fn start(&self) -> Result<()> {
        // Register commands
        self.register_commands().await?;

        // Initialize status bar
        self.context
            .update_status_bar(
                "windsurf.metrics",
                "Windsurf: Ready".to_string(),
                Some("Click for detailed metrics".to_string()),
            )
            .await?;

        Ok(())
    }

    async fn register_commands(&self) -> Result<()> {
        // In a real IDE, we'd register these with the IDE's command system
        // For now we'll just store them in our context
        let context = self.context.clone();

        // Register show metrics command
        tokio::spawn(async move {
            commands::show_metrics_details(context.clone())
                .await
                .unwrap();
        });

        Ok(())
    }

    // Event handlers that the IDE should call
    pub async fn handle_text_change(&self, text: String) -> Result<()> {
        self.context.handle_text_change(text).await
    }

    pub async fn handle_cursor_move(&self, position: Position) -> Result<()> {
        self.context.handle_cursor_move(position).await
    }

    pub async fn handle_visible_range_change(&self, range: Range) -> Result<()> {
        self.context.handle_visible_range_change(range).await
    }
}
