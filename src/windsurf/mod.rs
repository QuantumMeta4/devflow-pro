use crate::ai::AnalysisType;
use crate::{
    ai_enhanced::{AIAnalysisResult, AIProvider, CodeLLamaProvider},
    analysis::SemanticAnalyzer,
    DevFlowError, ProjectInsights, Result,
};
use chrono::{DateTime, Utc};
use log;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::{Mutex, Semaphore};

pub mod ide;
pub mod ide_main;
pub mod interface;
pub mod mock;
pub mod ui;

pub use ide_main::WindsurfIDE;
pub use interface::WindsurfInterface;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsDisplay {
    pub complexity_score: f64,
    pub quality_score: f64,
}

impl MetricsDisplay {
    pub fn new(analysis_result: &WindsurfAnalysisResult) -> Self {
        Self {
            complexity_score: analysis_result.ai_insights.semantic_complexity,
            quality_score: analysis_result.ai_insights.code_quality_score,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindsurfAnalysisResult {
    pub timestamp: DateTime<Utc>,
    pub context: AnalysisContext,
    pub analysis: ProjectInsights,
    pub ai_insights: AIAnalysisResult,
}

pub mod test_utils {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    pub async fn new_windsurf_plugin_with_mock() -> Result<super::WindsurfPlugin> {
        let config = WindsurfConfig::new(
            4,
            "mock".to_string(),
            "mock".to_string(),
            vec![],
            0.7,
            true,
            true,
        );

        Ok(super::WindsurfPlugin {
            config: Arc::new(Mutex::new(config)),
            ai_provider: Arc::new(super::mock::MockAIProvider::new()),
            semantic_analyzer: Arc::new(Mutex::new(SemanticAnalyzer::default())),
            current_file: Arc::new(Mutex::new(None)),
            semaphore: Arc::new(Semaphore::new(4)),
            metrics_display: Arc::new(Mutex::new(None)),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub column: u32,
    pub offset: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionCategory {
    CodeQuality,
    Security,
    Performance,
    BestPractices,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSuggestion {
    pub suggestion: String,
    pub confidence: f64,
    pub category: SuggestionCategory,
    pub code_snippet: Option<String>,
    pub applies_to_range: Option<Range>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisContext {
    pub file_path: String,
    pub code_content: String,
    pub cursor_position: Option<usize>,
    pub visible_range: Option<(usize, usize)>,
    pub language: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WindsurfConfig {
    pub max_concurrent_analyses: usize,
    pub model_name: String,
    pub api_endpoint: String,
    pub analysis_types: Vec<AnalysisType>,
    pub confidence_threshold: f64,
    pub enable_real_time: bool,
    pub cache_results: bool,
}

impl WindsurfConfig {
    pub fn new(
        max_concurrent_analyses: usize,
        model_name: String,
        api_endpoint: String,
        analysis_types: Vec<AnalysisType>,
        confidence_threshold: f64,
        enable_real_time: bool,
        cache_results: bool,
    ) -> Self {
        Self {
            max_concurrent_analyses,
            model_name,
            api_endpoint,
            analysis_types,
            confidence_threshold,
            enable_real_time,
            cache_results,
        }
    }

    pub fn with_real_time(mut self, enable: bool) -> Self {
        self.enable_real_time = enable;
        self
    }
}

#[derive(Debug, Clone)]
pub struct WindsurfPlugin {
    pub config: Arc<Mutex<WindsurfConfig>>,
    pub ai_provider: Arc<dyn AIProvider + Send + Sync>,
    pub semantic_analyzer: Arc<Mutex<SemanticAnalyzer>>,
    pub current_file: Arc<Mutex<Option<PathBuf>>>,
    pub semaphore: Arc<Semaphore>,
    pub metrics_display: Arc<Mutex<Option<MetricsDisplay>>>,
}

impl WindsurfPlugin {
    pub async fn new(config: Option<WindsurfConfig>) -> Result<Self> {
        let config = match config {
            Some(cfg) => cfg,
            None => WindsurfConfig::new(
                4,
                String::from("codellama/CodeLlama-34b-Instruct-hf"),
                String::from("https://api.together.xyz/v1"),
                vec![],
                0.7,
                true,
                true,
            ),
        };

        let ai_provider: Arc<dyn AIProvider + Send + Sync> =
            if let Ok(api_key) = std::env::var("TOGETHER_API_KEY") {
                log::info!("Using CodeLLamaProvider with Together.ai API");
                Arc::new(CodeLLamaProvider::new(
                    &api_key,
                    &config.api_endpoint,
                    &config.model_name,
                    config.max_concurrent_analyses,
                ))
            } else {
                log::warn!("TOGETHER_API_KEY not found, using MockAIProvider");
                Arc::new(mock::MockAIProvider::new())
            };

        Ok(Self {
            config: Arc::new(Mutex::new(config)),
            ai_provider,
            semantic_analyzer: Arc::new(Mutex::new(SemanticAnalyzer::default())),
            current_file: Arc::new(Mutex::new(None)),
            semaphore: Arc::new(Semaphore::new(4)),
            metrics_display: Arc::new(Mutex::new(None)),
        })
    }

    pub async fn analyze(&self, context: AnalysisContext) -> Result<WindsurfAnalysisResult> {
        // Acquire a permit for concurrent analysis
        let _permit = self.semaphore.acquire().await;

        // Store current file
        let mut current_file = self.current_file.lock().await;
        *current_file = Some(PathBuf::from(context.file_path.clone()));

        // Perform AI analysis
        let ai_result = self.ai_provider.analyze_code(&context.code_content).await?;

        // Update metrics display
        let mut metrics = self.metrics_display.lock().await;
        *metrics = Some(MetricsDisplay {
            complexity_score: ai_result.semantic_complexity,
            quality_score: ai_result.code_quality_score,
        });

        Ok(WindsurfAnalysisResult {
            timestamp: Utc::now(),
            context,
            analysis: ProjectInsights::default(),
            ai_insights: ai_result,
        })
    }

    pub async fn get_suggestions_at(&self, position: Position) -> Result<Vec<ContextSuggestion>> {
        let _permit = self
            .semaphore
            .acquire()
            .await
            .map_err(|e| DevFlowError::AI(format!("Failed to acquire semaphore: {}", e)))?;

        // Get current file
        let current_file = self
            .current_file
            .lock()
            .await
            .clone()
            .ok_or_else(|| DevFlowError::AI("No file currently being analyzed".into()))?;

        // Get semantic context
        let mut analyzer = self.semantic_analyzer.lock().await;
        let analyzer_ref = &mut *analyzer;
        let _semantic_context = analyzer_ref
            .analyze_file(&current_file)
            .map_err(|e| DevFlowError::AI(format!("Semantic analysis failed: {}", e)))?;

        // Get code content from semantic context
        let code_content = std::fs::read_to_string(&current_file)
            .map_err(|e| DevFlowError::AI(format!("Failed to read file: {}", e)))?;

        // Get AI suggestions
        let code_suggestions = AIProvider::analyze_code(&*self.ai_provider, &code_content).await?;

        // Convert to ContextSuggestion format
        let mut suggestions = Vec::new();

        // Handle optimization suggestions
        for suggestion in &code_suggestions.optimization_suggestions {
            suggestions.push(ContextSuggestion {
                suggestion: format!("Optimization: {:?}", suggestion),
                confidence: code_suggestions.code_quality_score,
                category: SuggestionCategory::Performance,
                code_snippet: Some(suggestion.description.clone()),
                applies_to_range: Some(Range {
                    start: position.clone(),
                    end: Position {
                        line: position.line + 1,
                        column: 0,
                        offset: position.offset + 50, // Approximate
                    },
                }),
            });
        }

        // Handle security recommendations
        for suggestion in &code_suggestions.security_recommendations {
            suggestions.push(ContextSuggestion {
                suggestion: format!("Security: {:?}", suggestion),
                confidence: code_suggestions.code_quality_score,
                category: SuggestionCategory::Security,
                code_snippet: Some(suggestion.description.clone()),
                applies_to_range: Some(Range {
                    start: position.clone(),
                    end: Position {
                        line: position.line + 1,
                        column: 0,
                        offset: position.offset + 50, // Approximate
                    },
                }),
            });
        }

        Ok(suggestions)
    }

    pub async fn update_cursor_position(&self, _position: Position) -> Result<()> {
        // Get current file
        let current_file = self
            .current_file
            .lock()
            .await
            .clone()
            .ok_or_else(|| DevFlowError::AI("No file currently being analyzed".into()))?;

        // Update analyzer state with new cursor position
        let mut analyzer = self.semantic_analyzer.lock().await;
        let analyzer_ref = &mut *analyzer;
        let _semantic_context = analyzer_ref
            .analyze_file(&current_file)
            .map_err(|e| DevFlowError::AI(format!("Failed to update cursor position: {}", e)))?;

        Ok(())
    }

    pub async fn get_status_bar_text(&self) -> Result<String> {
        let metrics = self.metrics_display.lock().await;
        if let Some(metrics) = &*metrics {
            Ok(format!(
                "Complexity: {:.2} | Quality: {:.2}",
                metrics.complexity_score, metrics.quality_score
            ))
        } else {
            Ok("No analysis available".to_string())
        }
    }

    pub async fn get_config(&self) -> WindsurfConfig {
        self.config.lock().await.clone()
    }

    pub async fn update_config(&self, config: WindsurfConfig) -> Result<()> {
        *self.config.lock().await = config;
        Ok(())
    }
}
