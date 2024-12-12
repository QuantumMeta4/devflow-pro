#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_llama_coder_initialization() {
        let temp_dir = tempdir().unwrap();
        let model_path = temp_dir.path().join("model.gguf");

        let config = LlamaConfig {
            model_path: model_path.into(),
            context_length: 2048,
            temperature: 0.7,
            top_p: 0.9,
            max_tokens: 512,
        };

        // This test will download the model, so it might take some time
        let coder = LlamaCoder::new(config).await;
        assert!(coder.is_ok(), "Failed to initialize LlamaCoder");
    }

    #[tokio::test]
    async fn test_code_analysis() {
        let temp_dir = tempdir().unwrap();
        let model_path = temp_dir.path().join("model.gguf");

        let config = LlamaConfig {
            model_path: model_path.into(),
            context_length: 2048,
            temperature: 0.7,
            top_p: 0.9,
            max_tokens: 512,
        };

        let coder = LlamaCoder::new(config).await.unwrap();
        
        let test_code = r#"
        fn fibonacci(n: u32) -> u32 {
            if n <= 1 {
                return n;
            }
            fibonacci(n - 1) + fibonacci(n - 2)
        }
        "#;

        let result = coder.analyze_code(test_code, AnalysisType::Optimization).await;
        assert!(result.is_ok(), "Code analysis failed");
        
        let analysis = result.unwrap();
        assert!(!analysis.suggestions.is_empty(), "No suggestions generated");
        assert!(!analysis.summary.is_empty(), "No summary generated");
    }
}
