use crate::{
    ai_enhanced::{
        AIAnalysisResult, AIProvider, OptimizationCategory, OptimizationSuggestion,
        SecurityRecommendation,
    },
    windsurf::{
        interface::{IDEInterface, WindsurfIntegration},
        Position, Range, WindsurfPlugin,
    },
    DevFlowError, IssueSeverity, SecurityIssue,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Mock implementation of `WindsurfIntegration` for testing.
#[derive(Debug)]
pub struct MockWindsurfIntegration {
    text_changes: Arc<Mutex<Vec<String>>>,
    cursor_moves: Arc<Mutex<Vec<Position>>>,
    visible_ranges: Arc<Mutex<Vec<Range>>>,
    real_time_enabled: Arc<Mutex<bool>>,
    plugin: WindsurfPlugin,
}

impl MockWindsurfIntegration {
    /// Creates a new mock integration instance.
    #[must_use]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            text_changes: Arc::new(Mutex::new(Vec::new())),
            cursor_moves: Arc::new(Mutex::new(Vec::new())),
            visible_ranges: Arc::new(Mutex::new(Vec::new())),
            real_time_enabled: Arc::new(Mutex::new(true)),
            plugin: WindsurfPlugin {
                name: "Mock Windsurf".to_string(),
                version: "0.1.0".to_string(),
            },
        })
    }

    /// Gets the recorded text changes.
    pub async fn get_text_changes(&self) -> Vec<String> {
        self.text_changes.lock().await.clone()
    }

    /// Gets the recorded cursor moves.
    pub async fn get_cursor_moves(&self) -> Vec<Position> {
        self.cursor_moves.lock().await.clone()
    }

    /// Gets the recorded visible ranges.
    pub async fn get_visible_ranges(&self) -> Vec<Range> {
        self.visible_ranges.lock().await.clone()
    }
}

#[async_trait]
impl IDEInterface for MockWindsurfIntegration {
    async fn handle_text_change(&self, content: &str) -> Result<()> {
        self.text_changes.lock().await.push(content.to_string());
        Ok(())
    }

    async fn handle_cursor_move(&self, position: Position) -> Result<()> {
        self.cursor_moves.lock().await.push(position);
        Ok(())
    }

    async fn handle_visible_range_change(&self, range: Range) -> Result<()> {
        self.visible_ranges.lock().await.push(range);
        Ok(())
    }

    async fn toggle_real_time_analysis(&self) -> Result<()> {
        let mut enabled = self.real_time_enabled.lock().await;
        *enabled = !*enabled;
        drop(enabled);
        Ok(())
    }

    fn get_plugin(&self) -> &WindsurfPlugin {
        &self.plugin
    }
}

#[async_trait]
impl WindsurfIntegration for MockWindsurfIntegration {
    async fn initialize(&self) -> Result<()> {
        Ok(())
    }
}

/// Test provider for mock implementations.
#[derive(Debug)]
pub struct TestProvider;

impl TestProvider {
    #[allow(clippy::cast_precision_loss)]
    fn calculate_quality_score(content: &str) -> f64 {
        content.len() as f64 * 0.1
    }

    fn generate_security_recommendations(content: &str) -> Vec<SecurityRecommendation> {
        vec![SecurityRecommendation {
            severity: IssueSeverity::Low,
            description: format!(
                "Mock security recommendation for {} bytes of code",
                content.len()
            ),
            suggested_fix: Some("Fix suggestion".to_string()),
            confidence: 0.8,
        }]
    }

    fn generate_optimization_suggestions(content: &str) -> Vec<OptimizationSuggestion> {
        vec![OptimizationSuggestion {
            category: OptimizationCategory::Performance,
            description: format!(
                "Mock optimization suggestion for {} bytes of code",
                content.len()
            ),
            impact_score: 0.7,
            suggested_implementation: Some("Implementation suggestion".to_string()),
        }]
    }
}

#[async_trait]
impl AIProvider for TestProvider {
    async fn analyze_code(&self, content: &str) -> Result<AIAnalysisResult, DevFlowError> {
        Ok(AIAnalysisResult {
            code_quality_score: Self::calculate_quality_score(content),
            #[allow(clippy::cast_precision_loss)]
            semantic_complexity: (content.lines().count() as f64) * 0.1,
            security_recommendations: Self::generate_security_recommendations(content),
            optimization_suggestions: Self::generate_optimization_suggestions(content),
        })
    }

    async fn suggest_fixes(&self, issues: &[SecurityIssue]) -> Result<Vec<String>, DevFlowError> {
        Ok(issues.iter().map(|i| format!("Fix for {i:?}")).collect())
    }
}
