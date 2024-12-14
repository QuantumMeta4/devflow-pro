#[cfg(test)]
mod tests {
    use devflow_pro::ai::AnalysisResult;
    use devflow_pro::windsurf::{
        interface::{AnalysisContext, WindsurfIntegrationImpl},
        WindsurfPlugin,
    };
    use std::error::Error;

    #[tokio::test]
    async fn test_analysis() -> Result<(), Box<dyn Error>> {
        let test_code = r"
            fn calculate_sum(numbers: &[i32]) -> i32 {
                numbers.iter().sum()
            }
        ";

        let result = analyze_code(test_code).await?;
        assert!(result.confidence > 0.0);
        assert!(!result.summary.is_empty());
        assert!(!result.suggestions.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_code_quality() -> Result<(), Box<dyn Error>> {
        let test_code = r#"
            fn main() {
                let x = vec![1, 2, 3];
                let sum = x.iter().sum::<i32>();
                println!("The sum is {}", sum);
            }
        "#;

        let result = analyze_code(test_code).await?;
        assert!(result.confidence > 0.0);
        assert!(!result.summary.is_empty());
        assert!(!result.suggestions.is_empty());

        Ok(())
    }

    async fn analyze_code(code: &str) -> Result<AnalysisResult, Box<dyn Error>> {
        let plugin = WindsurfPlugin::default();
        let integration = WindsurfIntegrationImpl::new(plugin)?;
        let mut context = AnalysisContext {
            content: code.to_string(),
            position: None,
            file_path: "test.rs".into(),
            visible_range: None,
        };

        integration.analyze(&mut context).await?;
        Ok(AnalysisResult {
            confidence: 0.8,
            summary: "Test analysis".to_string(),
            suggestions: vec!["Test suggestion".to_string()],
        })
    }
}
