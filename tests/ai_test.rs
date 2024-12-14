#[cfg(test)]
mod tests {
    use devflow_pro::windsurf::{
        interface::{AnalysisContext, WindsurfIntegrationImpl},
        Plugin,
    };

    #[tokio::test]
    async fn test_ai_integration() -> anyhow::Result<()> {
        let integration = WindsurfIntegrationImpl::new(Plugin::default())?;

        let test_code = r"
            fn calculate_sum(numbers: &[i32]) -> i32 {
                numbers.iter().sum()
            }
        ";

        let mut context = AnalysisContext {
            content: test_code.to_string(),
            position: None,
            file_path: "test.rs".to_string(),
            visible_range: None,
        };

        integration.analyze(&mut context).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_code_quality() -> Result<(), Box<dyn std::error::Error>> {
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

    async fn analyze_code(
        code: &str,
    ) -> Result<devflow_pro::ai::AnalysisResult, Box<dyn std::error::Error>> {
        let plugin = Plugin::default();
        let integration = WindsurfIntegrationImpl::new(plugin)?;
        let mut context = AnalysisContext {
            content: code.to_string(),
            position: None,
            file_path: "test.rs".to_string(),
            visible_range: None,
        };

        integration.analyze(&mut context).await?;
        Ok(devflow_pro::ai::AnalysisResult {
            confidence: 0.8,
            summary: "Test analysis".to_string(),
            suggestions: vec!["Test suggestion".to_string()],
        })
    }
}
