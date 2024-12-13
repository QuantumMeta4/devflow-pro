use async_trait::async_trait;
use devflow_pro::{
    ai_enhanced::{AIAnalysisResult, AIProvider, OptimizationCategory, OptimizationSuggestion, SecurityRecommendation},
    analysis::AnalysisPipeline,
    IssueSeverity, Result,
};
use std::sync::Arc;

#[derive(Default, Debug)]
struct MockAIProvider;

#[async_trait]
impl AIProvider for MockAIProvider {
    async fn analyze_code(&self, _content: &str) -> Result<AIAnalysisResult> {
        Ok(AIAnalysisResult {
            code_quality_score: 0.85,
            security_recommendations: vec![SecurityRecommendation {
                severity: IssueSeverity::Medium,
                description: "Potential SQL injection".to_string(),
                suggested_fix: Some("Use parameterized queries".to_string()),
                confidence: 0.9,
            }],
            optimization_suggestions: vec![OptimizationSuggestion {
                category: OptimizationCategory::Performance,
                description: "Consider using parallel processing".to_string(),
                impact_score: 0.7,
                suggested_implementation: Some("Use rayon".to_string()),
            }],
            semantic_complexity: 0.65,
        })
    }

    async fn suggest_fixes(&self, _issues: &[devflow_pro::SecurityIssue]) -> Result<Vec<String>> {
        Ok(vec!["Use prepared statements".to_string()])
    }
}

#[tokio::test]
async fn test_analysis_pipeline() {
    let ai_provider = Arc::new(MockAIProvider::default());
    let pipeline = AnalysisPipeline::new(ai_provider).unwrap();

    pipeline.start_workers(2).unwrap();

    // Create a temporary test file
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    std::fs::write(&test_file, "fn main() { println!(\"Hello, World!\"); }").unwrap();

    // Submit file for analysis
    pipeline.submit(test_file.clone()).unwrap();

    // Wait for analysis to complete
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Check results
    let result = pipeline.get_result(&test_file).unwrap();
    assert!(result.metrics.lines_of_code > 0);
    assert!(result.ai_insights.code_quality_score > 0.0);
}
