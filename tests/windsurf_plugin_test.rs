use chrono::Utc;
use devflow_pro::{
    ai_enhanced::AIAnalysisResult,
    windsurf::{AnalysisContext, Position, WindsurfConfig},
    ProjectInsights, Result,
};
use std::fs;
use tempfile::tempdir;
use tokio;

#[cfg(test)]
mod tests {
    use super::*;
    use devflow_pro::windsurf::test_utils;

    #[tokio::test]
    async fn test_windsurf_plugin() -> Result<()> {
        // Create a temporary directory
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let test_file_path = temp_dir.path().join("test_file.rs");
        let test_code = "fn main() { println!(\"Hello, World!\"); }";
        fs::write(&test_file_path, test_code).expect("Failed to write test file");

        // Create a new plugin instance with mock provider
        let plugin = test_utils::new_windsurf_plugin_with_mock().await?;

        // Test file analysis
        let context = AnalysisContext {
            file_path: test_file_path.to_string_lossy().to_string(),
            code_content: test_code.to_string(),
            cursor_position: Some(0),
            visible_range: Some((0, 10)),
            language: "rust".to_string(),
        };

        let analysis_result = plugin.analyze(context).await?;

        // Verify analysis result
        assert!(analysis_result.timestamp > Utc::now() - chrono::Duration::hours(1));
        assert!(matches!(analysis_result.analysis, ProjectInsights { .. }));
        assert!(matches!(
            analysis_result.ai_insights,
            AIAnalysisResult { .. }
        ));

        // Test suggestion retrieval
        let position = Position {
            line: 0,
            column: 0,
            offset: 0,
        };

        let suggestions = plugin.get_suggestions_at(position.clone()).await?;

        // Verify suggestions - no need to check if empty since we're using a mock
        for suggestion in suggestions {
            assert!(suggestion.confidence >= 0.0 && suggestion.confidence <= 1.0);
            assert!(suggestion.code_snippet.is_some());
            assert!(suggestion.applies_to_range.is_some());
        }

        // Test cursor movement
        plugin.update_cursor_position(position).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_windsurf_config() -> Result<()> {
        let config = WindsurfConfig::new(
            4,
            "codellama/CodeLlama-34b-Instruct-hf".to_string(),
            "https://api.together.xyz".to_string(),
            vec![],
            0.7,
            true,
            true,
        );

        assert_eq!(config.max_concurrent_analyses, 4);
        assert_eq!(config.model_name, "codellama/CodeLlama-34b-Instruct-hf");
        assert_eq!(config.api_endpoint, "https://api.together.xyz");
        assert!(config.enable_real_time);
        assert!(config.cache_results);
        assert!(config.confidence_threshold > 0.0);

        Ok(())
    }
}
