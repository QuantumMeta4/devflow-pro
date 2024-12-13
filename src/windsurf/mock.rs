use crate::ai_enhanced::{
    AIAnalysisResult, AIProvider, OptimizationCategory, OptimizationSuggestion,
    SecurityRecommendation,
};
use crate::{IssueSeverity, Result, SecurityIssue};
use async_trait::async_trait;

#[derive(Debug)]
pub struct MockAIProvider;

impl MockAIProvider {
    pub fn new() -> Self {
        Self
    }

    fn analyze_code_quality(&self, code: &str) -> f64 {
        let lines: Vec<&str> = code.lines().collect();
        let total_lines = lines.len() as f64;
        if total_lines == 0.0 {
            return 0.0;
        }

        let comment_lines = lines.iter().filter(|l| l.trim().starts_with("//")).count() as f64;
        let long_lines = lines.iter().filter(|l| l.len() > 100).count() as f64;
        let empty_lines = lines.iter().filter(|l| l.trim().is_empty()).count() as f64;

        let comment_ratio = comment_lines / total_lines;
        let long_lines_penalty = 1.0 - (long_lines / total_lines);
        let empty_lines_ratio = empty_lines / total_lines;

        let base_score = 70.0;
        let comment_score = comment_ratio * 15.0;
        let length_score = long_lines_penalty * 10.0;
        let structure_score = (1.0 - (empty_lines_ratio - 0.1).abs()) * 5.0;

        (base_score + comment_score + length_score + structure_score).min(100.0)
    }

    fn generate_security_recommendations(&self, code: &str) -> Vec<SecurityRecommendation> {
        let mut recommendations = Vec::new();

        if code.contains("unsafe") {
            recommendations.push(SecurityRecommendation {
                severity: IssueSeverity::High,
                description: "Use of unsafe block detected".to_string(),
                suggested_fix: Some(
                    "Consider using safe alternatives or add safety documentation".to_string(),
                ),
                confidence: 0.9,
            });
        }

        if code.contains(".unwrap()") {
            recommendations.push(SecurityRecommendation {
                severity: IssueSeverity::Medium,
                description: "Use of unwrap() detected".to_string(),
                suggested_fix: Some(
                    "Replace with proper error handling using match or if let".to_string(),
                ),
                confidence: 0.8,
            });
        }

        if code.contains("Box::leak") {
            recommendations.push(SecurityRecommendation {
                severity: IssueSeverity::High,
                description: "Memory leak risk detected".to_string(),
                suggested_fix: Some(
                    "Ensure proper memory management or use Arc/Rc instead".to_string(),
                ),
                confidence: 0.85,
            });
        }

        recommendations
    }

    fn generate_optimization_suggestions(&self, code: &str) -> Vec<OptimizationSuggestion> {
        let mut suggestions = Vec::new();

        if code.contains(".clone()") {
            suggestions.push(OptimizationSuggestion {
                category: OptimizationCategory::Performance,
                description: "Unnecessary clone detected".to_string(),
                impact_score: 0.7,
                suggested_implementation: Some(
                    "Consider using references or implementing Copy".to_string(),
                ),
            });
        }

        if code.contains("Vec::new()") {
            suggestions.push(OptimizationSuggestion {
                category: OptimizationCategory::Memory,
                description: "Vector without capacity hint".to_string(),
                impact_score: 0.5,
                suggested_implementation: Some(
                    "Use Vec::with_capacity() when size is known".to_string(),
                ),
            });
        }

        if code.contains("push_str") || code.contains(" + ") {
            suggestions.push(OptimizationSuggestion {
                category: OptimizationCategory::Performance,
                description: "String concatenation detected".to_string(),
                impact_score: 0.6,
                suggested_implementation: Some(
                    "Consider using string formatting or a string builder".to_string(),
                ),
            });
        }

        suggestions
    }
}

impl Default for MockAIProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AIProvider for MockAIProvider {
    async fn analyze_code(&self, content: &str) -> Result<AIAnalysisResult> {
        Ok(AIAnalysisResult {
            code_quality_score: self.analyze_code_quality(content),
            security_recommendations: self.generate_security_recommendations(content),
            optimization_suggestions: self.generate_optimization_suggestions(content),
            semantic_complexity: content.lines().count() as f64 * 0.1,
        })
    }

    async fn suggest_fixes(&self, issues: &[SecurityIssue]) -> Result<Vec<String>> {
        Ok(issues
            .iter()
            .map(|issue| {
                format!(
                    "Fix for {:?} severity issue: {}",
                    issue.severity, issue.description
                )
            })
            .collect())
    }
}
