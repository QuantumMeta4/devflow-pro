use crate::ai_enhanced::{
    AIAnalysisResult, AIProvider, OptimizationCategory, OptimizationSuggestion,
    SecurityRecommendation,
};
use crate::windsurf::{
    interface::{Interface, WindsurfIntegration},
    Config, Plugin, Position, Range,
};
use crate::{DevFlowError, IssueSeverity, SecurityIssue};
use anyhow::Result;
use async_trait::async_trait;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

/// Mock implementation of `WindsurfIntegration` for testing.
#[derive(Default, Debug)]
pub struct MockIntegration {
    text_changes: Arc<Mutex<Vec<String>>>,
    cursor_moves: Arc<Mutex<Vec<Position>>>,
    visible_ranges: Arc<Mutex<Vec<Range>>>,
    real_time_enabled: Arc<Mutex<bool>>,
    plugin: Plugin,
    config: Arc<Mutex<Config>>,
    current_file: Arc<Mutex<Option<PathBuf>>>,
}

impl MockIntegration {
    /// Creates a new `MockIntegration` instance with the given plugin.
    #[must_use]
    pub fn new(plugin: Plugin) -> Self {
        Self {
            plugin,
            text_changes: Arc::new(Mutex::new(Vec::new())),
            cursor_moves: Arc::new(Mutex::new(Vec::new())),
            visible_ranges: Arc::new(Mutex::new(Vec::new())),
            real_time_enabled: Arc::new(Mutex::new(false)),
            config: Arc::new(Mutex::new(Config::default())),
            current_file: Arc::new(Mutex::new(None)),
        }
    }

    /// Gets the recorded text changes.
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned.
    #[must_use]
    pub fn get_text_changes(&self) -> Vec<String> {
        self.text_changes.lock().unwrap().clone()
    }

    /// Gets the recorded cursor moves.
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned.
    #[must_use]
    pub fn get_cursor_moves(&self) -> Vec<Position> {
        self.cursor_moves.lock().unwrap().clone()
    }

    /// Gets the recorded visible ranges.
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned.
    #[must_use]
    pub fn get_visible_ranges(&self) -> Vec<Range> {
        self.visible_ranges.lock().unwrap().clone()
    }
}

#[async_trait]
impl Interface for MockIntegration {
    async fn handle_text_change(&self, content: &str) -> Result<()> {
        self.text_changes.lock().unwrap().push(content.to_string());
        Ok(())
    }

    async fn handle_cursor_move(&self, line: u32, character: u32) -> Result<()> {
        self.cursor_moves
            .lock()
            .unwrap()
            .push(Position { line, character });
        Ok(())
    }

    async fn handle_visible_range_change(&self, range: Range) -> Result<()> {
        self.visible_ranges.lock().unwrap().push(range);
        Ok(())
    }

    async fn toggle_real_time_analysis(&self) -> Result<()> {
        let mut enabled = self.real_time_enabled.lock().unwrap();
        *enabled = !*enabled;
        drop(enabled);
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
impl WindsurfIntegration for MockIntegration {
    async fn initialize(&self) -> Result<()> {
        Ok(())
    }
}

/// Mock implementation of the AI provider for testing.
#[derive(Debug)]
pub struct TestProvider;

impl TestProvider {
    fn generate_security_recommendations(_content: &str) -> Vec<SecurityRecommendation> {
        vec![SecurityRecommendation {
            severity: IssueSeverity::Low,
            description: "Test security recommendation".to_string(),
            suggested_fix: Some("Test suggestion".to_string()),
            confidence: 0.8,
        }]
    }

    fn generate_optimization_suggestions(_content: &str) -> Vec<OptimizationSuggestion> {
        vec![OptimizationSuggestion {
            category: OptimizationCategory::Performance,
            description: "Test optimization suggestion".to_string(),
            impact_score: 0.7,
            suggested_implementation: Some("Test suggestion".to_string()),
        }]
    }
}

#[async_trait]
impl AIProvider for TestProvider {
    async fn analyze_code(&self, _content: &str) -> Result<AIAnalysisResult, DevFlowError> {
        Ok(AIAnalysisResult {
            code_quality_score: 0.8,
            semantic_complexity: 0.5,
            security_recommendations: Self::generate_security_recommendations(_content),
            optimization_suggestions: Self::generate_optimization_suggestions(_content),
        })
    }

    async fn suggest_fixes(&self, _issues: &[SecurityIssue]) -> Result<Vec<String>, DevFlowError> {
        Ok(vec!["Test fix suggestion".to_string()])
    }
}
