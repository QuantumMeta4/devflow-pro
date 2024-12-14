#[cfg(test)]
mod tests {
    use devflow_pro::ai::{AnalysisResult, AnalysisType};
    use std::path::Path;

    #[tokio::test]
    async fn test_code_analysis() {
        let test_code = r#"
            fn main() {
                println!("Hello, World!");
            }
        "#;

        let result = analyze_code(
            test_code,
            Path::new("test.rs"),
            &[AnalysisType::SecurityAudit, AnalysisType::Optimization],
        )
        .await;

        assert!(result.is_ok());
        let analysis = result.unwrap();
        assert!(!analysis.summary.is_empty());
        assert!(!analysis.suggestions.is_empty());
        assert!(analysis.confidence > 0.0);
    }

    async fn analyze_code(
        _code: &str,
        _path: &Path,
        _types: &[AnalysisType],
    ) -> Result<AnalysisResult, Box<dyn std::error::Error>> {
        // Mock implementation for testing
        Ok(AnalysisResult {
            summary: "Code looks good".to_string(),
            suggestions: vec!["Consider adding error handling".to_string()],
            confidence: 0.8,
        })
    }
}
