use async_trait::async_trait;
use devflow_pro::ai_enhanced::{AIAnalysisResult, AIProvider};
use devflow_pro::DevFlowError;

#[derive(Debug)]
struct MockAIProvider;

#[async_trait]
impl AIProvider for MockAIProvider {
    async fn analyze_code(&self, _content: &str) -> Result<AIAnalysisResult, DevFlowError> {
        Ok(AIAnalysisResult {
            code_quality_score: 0.85,
            semantic_complexity: 0.5,
            security_recommendations: vec![],
            optimization_suggestions: vec![],
        })
    }

    async fn suggest_fixes(
        &self,
        _issues: &[devflow_pro::SecurityIssue],
    ) -> Result<Vec<String>, DevFlowError> {
        Ok(vec!["Fix security issue".to_string()])
    }
}

#[tokio::test]
async fn test_ai_analysis() -> anyhow::Result<()> {
    let provider = MockAIProvider;
    let code = "fn main() { let x = 42; }";

    let result = provider.analyze_code(code).await?;
    assert!(result.code_quality_score > 0.0);
    assert!(result.semantic_complexity < 1.0);

    Ok(())
}
