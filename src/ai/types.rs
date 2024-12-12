use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlamaConfig {
    pub model_name: String,
    pub context_length: usize,
    pub temperature: f32,
    pub top_p: f32,
    pub max_tokens: u32,
    pub stop_sequences: Vec<String>,
}

impl Default for LlamaConfig {
    fn default() -> Self {
        Self {
            model_name: "mistralai/Mistral-7B-Instruct-v0.1".to_string(),
            context_length: 4096,
            temperature: 0.7,
            top_p: 0.95,
            max_tokens: 1000,
            stop_sequences: vec!["```".to_string()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalysisType {
    CodeReview,
    BugFinding,
    SecurityAudit,
    Documentation,
    Optimization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub summary: String,
    pub suggestions: Vec<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityVulnerability {
    pub cwe_id: Option<String>,
    pub description: String,
    pub severity: Severity,
    pub impact: String,
    pub mitigation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    pub description: String,
    pub impact: String,
    pub code_example: Option<String>,
    pub expected_benefit: String,
    pub complexity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationSection {
    pub title: String,
    pub content: String,
    pub code_examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BugReport {
    pub description: String,
    pub severity: Severity,
    pub impact: String,
    pub fix: String,
    pub prevention: String,
}
