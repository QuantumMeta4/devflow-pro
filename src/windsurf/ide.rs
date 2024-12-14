use super::{Arc, IDEInterface, Position, Range};
use crate::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusBarItem {
    pub id: String,
    pub text: String,
    pub tooltip: Option<String>,
    pub command: Option<String>,
}

#[derive(Debug)]
pub struct IDE {
    status_bar: Mutex<Vec<StatusBarItem>>,
    windsurf: Arc<dyn IDEInterface>,
}

impl IDE {
    #[must_use]
    pub fn new(windsurf: Arc<dyn IDEInterface>) -> Self {
        Self {
            status_bar: Mutex::new(Vec::new()),
            windsurf,
        }
    }

    /// Updates the status bar with the given text and optional tooltip.
    ///
    /// # Errors
    ///
    /// Returns an error if the status bar update fails or if the mutex lock cannot be acquired.
    pub async fn update_status_bar(
        &self,
        id: &str,
        text: String,
        tooltip: Option<String>,
    ) -> Result<()> {
        let mut status_bar = self.status_bar.lock().await;
        if let Some(item) = status_bar.iter_mut().find(|item| item.id == id) {
            item.text = text;
            item.tooltip = tooltip;
        } else {
            status_bar.push(StatusBarItem {
                id: id.to_string(),
                text,
                tooltip,
                command: Some("windsurf.showMetricsDetails".to_string()),
            });
        }
        drop(status_bar);
        Ok(())
    }

    /// Handles text changes in the editor.
    ///
    /// # Errors
    ///
    /// Returns an error if the text change cannot be processed.
    pub async fn handle_text_change(&self, text: String) -> Result<()> {
        self.windsurf.handle_text_change(text).await
    }

    /// Handles cursor movement in the editor.
    ///
    /// # Errors
    ///
    /// Returns an error if the cursor position cannot be updated.
    pub async fn handle_cursor_move(&self, position: Position) -> Result<()> {
        self.windsurf.handle_cursor_move(position).await
    }

    /// Handles visible range changes in the editor.
    ///
    /// # Errors
    ///
    /// Returns an error if the visible range cannot be updated.
    pub async fn handle_visible_range_change(&self, range: Range) -> Result<()> {
        self.windsurf.handle_visible_range_change(range).await
    }
}

pub mod commands {
    use super::{Arc, Result, IDE};

    /// Shows detailed metrics information.
    ///
    /// # Errors
    ///
    /// Returns an error if the metrics cannot be displayed.
    pub async fn show_metrics_details(ide: Arc<IDE>) -> Result<()> {
        let metrics_text = format!("{ide:?}");
        println!("Detailed Metrics:\n{metrics_text}");
        Ok(())
    }

    /// Toggles real-time analysis.
    ///
    /// # Errors
    ///
    /// Returns an error if the real-time analysis state cannot be toggled.
    pub async fn toggle_real_time_analysis(ide: Arc<IDE>) -> Result<()> {
        let plugin = ide.windsurf.get_plugin();
        let config = plugin.get_config().await;
        let enable_real_time = !config.enable_real_time;
        let new_config = config.with_real_time(enable_real_time);
        plugin.update_config(new_config).await
    }
}
