use super::{Position, Range, WindsurfInterface};
use crate::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusBarItem {
    pub id: String,
    pub text: String,
    pub tooltip: Option<String>,
    pub command: Option<String>,
}

#[derive(Debug, Clone)]
pub struct IDEContext {
    pub status_bar: Arc<Mutex<Vec<StatusBarItem>>>,
    pub windsurf: Arc<dyn WindsurfInterface>,
}

impl IDEContext {
    pub fn new(windsurf: Arc<dyn WindsurfInterface>) -> Self {
        Self {
            status_bar: Arc::new(Mutex::new(Vec::new())),
            windsurf,
        }
    }

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
        Ok(())
    }

    pub async fn handle_text_change(&self, text: String) -> Result<()> {
        // Forward to Windsurf interface
        self.windsurf.handle_text_change(text).await?;

        // Get updated metrics
        let metrics_text = self.windsurf.get_plugin().get_status_bar_text().await?;

        // Update status bar
        self.update_status_bar(
            "windsurf.metrics",
            metrics_text.clone(),
            Some("Click for detailed metrics".to_string()),
        )
        .await?;

        Ok(())
    }

    pub async fn handle_cursor_move(&self, position: Position) -> Result<()> {
        self.windsurf.handle_cursor_move(position).await
    }

    pub async fn handle_visible_range_change(&self, _range: Range) -> Result<()> {
        // Store visible range for context-aware analysis
        Ok(())
    }
}

// Command handlers
pub mod commands {
    use super::*;

    pub async fn show_metrics_details(context: Arc<IDEContext>) -> Result<()> {
        // This would open a detailed metrics view in the IDE
        // For now we'll just print the metrics
        let metrics_text = context.windsurf.get_plugin().get_status_bar_text().await?;
        println!("Detailed Metrics:\n{}", metrics_text);
        Ok(())
    }

    pub async fn toggle_real_time_analysis(context: Arc<IDEContext>) -> Result<()> {
        // Toggle the real-time analysis setting
        let plugin = context.windsurf.get_plugin();
        let mut config = plugin.get_config().await;
        config.enable_real_time = !config.enable_real_time;
        plugin.update_config(config).await?;
        Ok(())
    }
}
