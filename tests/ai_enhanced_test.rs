use std::time::Duration;

use async_trait::async_trait;
use crossbeam::channel;

use devflow_pro::ai_enhanced::{
    AIAnalysisResult, AIProvider, OptimizationCategory, OptimizationSuggestion,
    SecurityRecommendation,
};
use devflow_pro::analysis::Pipeline;
use devflow_pro::{DevFlowError, IssueSeverity};

#[derive(Default, Debug)]
#[allow(dead_code)]
struct MockAIProvider;

#[async_trait]
impl AIProvider for MockAIProvider {
    async fn analyze_code(&self, _content: &str) -> Result<AIAnalysisResult, DevFlowError> {
        Ok(AIAnalysisResult {
            code_quality_score: 0.85,
            security_recommendations: vec![SecurityRecommendation {
                description: "Potential SQL injection".to_string(),
                severity: IssueSeverity::Medium,
                suggested_fix: Some("Use parameterized queries".to_string()),
                confidence: 0.9,
            }],
            optimization_suggestions: vec![OptimizationSuggestion {
                description: "Consider using parallel processing".to_string(),
                category: OptimizationCategory::Performance,
                impact_score: 0.7,
                suggested_implementation: Some("Use rayon".to_string()),
            }],
            semantic_complexity: 0.65,
        })
    }

    async fn suggest_fixes(
        &self,
        _issues: &[devflow_pro::SecurityIssue],
    ) -> Result<Vec<String>, DevFlowError> {
        Ok(vec!["Use prepared statements".to_string()])
    }
}

#[tokio::test]
async fn test_analysis_pipeline() {
    let _ = env_logger::try_init();

    // Create a temporary directory for test files
    let dir = tempfile::tempdir().unwrap();
    let test_file = dir.path().join("test.rs");

    // Write test content to file
    let test_content = r#"
        fn main() {
            println!("Hello, world!");
        }
    "#;
    std::fs::write(&test_file, test_content).unwrap();

    // Create and start the pipeline
    let pipeline = Pipeline::new();
    let (sender, receiver) = channel::unbounded();
    pipeline.start_workers(2, &receiver);

    // Send the test file
    sender.send(test_file.clone()).unwrap();
    drop(sender); // Signal that no more files will be sent

    // Wait for processing to complete
    let timeout = Duration::from_secs(5); // Reduced timeout
    let start = std::time::Instant::now();

    while !pipeline.is_file_processed(&test_file) {
        assert!(start.elapsed() <= timeout, "Test timed out");
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    // Verify results
    let stats = pipeline.get_stats();
    assert_eq!(
        stats
            .files_processed
            .load(std::sync::atomic::Ordering::SeqCst),
        1
    );
    assert_eq!(stats.errors.load(std::sync::atomic::Ordering::SeqCst), 0);

    // Clean up
    dir.close().unwrap();
}
