#[cfg(test)]
mod tests {
    use devflow_pro::windsurf::{
        interface::{AnalysisContext, WindsurfIntegrationImpl},
        Plugin,
    };
    use std::error::Error;

    #[tokio::test]
    async fn test_plugin_analysis() -> Result<(), Box<dyn Error>> {
        let integration = WindsurfIntegrationImpl::new(Plugin::default())?;

        let test_code = r"
            fn calculate_sum(numbers: &[i32]) -> i32 {
                numbers.iter().sum()
            }
        ";

        let mut context = AnalysisContext {
            content: test_code.to_string(),
            position: None,
            file_path: "test.rs".into(),
            visible_range: None,
        };

        integration.analyze(&mut context).await?;
        Ok(())
    }
}
