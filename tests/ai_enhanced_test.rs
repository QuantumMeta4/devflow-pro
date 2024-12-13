use async_trait::async_trait;
use devflow_pro::{
    ai_enhanced::{AIAnalysisResult, AIProvider, OptimizationCategory, OptimizationSuggestion, SecurityRecommendation},
    analysis::{AnalysisPipeline, SemanticAnalyzer},
    IssueSeverity, Result,
};
use std::sync::Arc;
use crossbeam::channel;
use tempfile;
use std::time::Duration;
use env_logger;
use log;

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
    let _ = env_logger::builder().is_test(true).try_init();
    
    let ai_provider = Arc::new(MockAIProvider::default());
    let semantic_analyzer = Arc::new(SemanticAnalyzer::new());
    let pipeline = AnalysisPipeline::new(semantic_analyzer, ai_provider);

    // Create channels for sending files to workers
    let (sender, receiver) = channel::unbounded();
    pipeline.start_workers(2, receiver);

    // Create a temporary test file
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    log::debug!("Creating test file at {:?}", test_file);
    std::fs::write(&test_file, "fn main() { println!(\"Hello, World!\"); }").unwrap();

    // Send file for analysis
    log::debug!("Sending file for analysis");
    sender.send(test_file.clone()).expect("Failed to send file for analysis");

    // Wait for analysis to complete with timeout
    let start = std::time::Instant::now();
    let timeout = Duration::from_secs(10); // Increased timeout
    let mut processed = false;

    while !processed {
        if start.elapsed() > timeout {
            panic!("Timeout waiting for file analysis after {} seconds", timeout.as_secs());
        }
        
        processed = pipeline.is_file_processed(&test_file);
        if !processed {
            let stats = pipeline.get_stats();
            log::debug!("Current stats: processed={}, errors={}", stats.files_processed, stats.errors);
            if stats.errors > 0 {
                panic!("Analysis failed with {} errors", stats.errors);
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    // Check results
    log::debug!("Analysis completed, checking results");
    let result = pipeline.get_analysis_result(&test_file).expect("Failed to get analysis result");
    
    // Check if analysis completed successfully
    if let Some(ai_insights) = result.ai_insights {
        log::debug!("Got AI insights: score={}", ai_insights.code_quality_score);
        assert!(ai_insights.code_quality_score > 0.0);
        assert_eq!(ai_insights.security_recommendations.len(), 1);
        assert_eq!(ai_insights.optimization_suggestions.len(), 1);
    } else {
        let stats = pipeline.get_stats();
        if stats.errors > 0 {
            panic!("Analysis failed with errors, no AI insights available");
        }
    }

    // Check pipeline stats
    let stats = pipeline.get_stats();
    log::debug!("Final stats: processed={}, errors={}", stats.files_processed, stats.errors);
    assert_eq!(stats.files_processed, 1);
    assert_eq!(stats.errors, 0);
}
