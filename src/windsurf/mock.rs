use crate::ai_enhanced::{
    AIAnalysisResult, AIProvider, OptimizationCategory, OptimizationSuggestion,
    SecurityRecommendation,
};
use crate::{IssueSeverity, Result, SecurityIssue};
use async_trait::async_trait;

/// AI provider implementation for testing purposes
#[derive(Debug)]
pub struct TestProvider;

impl TestProvider {
    /// Creates a new test provider instance.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for TestProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AIProvider for TestProvider {
    async fn analyze_code(&self, content: &str) -> Result<AIAnalysisResult> {
        Ok(AIAnalysisResult {
            code_quality_score: Self::analyze_code_quality(content),
            security_recommendations: Self::generate_security_recommendations(content),
            optimization_suggestions: Self::generate_optimization_suggestions(content),
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

impl TestProvider {
    fn analyze_code_quality(code: &str) -> f64 {
        let lines: Vec<&str> = code.lines().collect();
        let total_lines = lines.len() as f32;

        if total_lines == 0.0 {
            return 0.0;
        }

        let comment_lines = lines.iter().filter(|l| l.trim().starts_with("//")).count() as f32;
        let long_lines = lines.iter().filter(|l| l.len() > 100).count() as f32;
        let empty_lines = lines.iter().filter(|l| l.trim().is_empty()).count() as f32;

        let comment_ratio = comment_lines / total_lines;
        let long_lines_ratio = long_lines / total_lines;
        let empty_lines_ratio = empty_lines / total_lines;

        let quality_score = 100.0
            * (
                0.4 * (1.0 - long_lines_ratio) +  // Prefer shorter lines
            0.4 * comment_ratio +              // More comments is better
            0.2 * (1.0 - empty_lines_ratio)
                // Some empty lines are good, but not too many
            );

        quality_score as f64
    }

    fn generate_security_recommendations(code: &str) -> Vec<SecurityRecommendation> {
        let mut recommendations = Vec::new();

        // Simple pattern matching for common security issues
        if code.contains("unsafe") {
            recommendations.push(SecurityRecommendation {
                severity: IssueSeverity::High,
                description: "Use of unsafe block detected".to_string(),
                suggested_fix: Some("Consider using safe alternatives".to_string()),
                confidence: 0.9,
            });
        }

        if code.contains("unwrap") {
            recommendations.push(SecurityRecommendation {
                severity: IssueSeverity::Medium,
                description: "Use of unwrap() detected".to_string(),
                suggested_fix: Some("Handle errors explicitly".to_string()),
                confidence: 0.8,
            });
        }

        recommendations
    }

    fn generate_optimization_suggestions(code: &str) -> Vec<OptimizationSuggestion> {
        let mut suggestions = Vec::new();

        // Simple pattern matching for common optimization opportunities
        if code.contains("clone") {
            suggestions.push(OptimizationSuggestion {
                category: OptimizationCategory::Performance,
                description: "Unnecessary clone detected".to_string(),
                impact_score: 0.5,
                suggested_implementation: Some("Consider using references".to_string()),
            });
        }

        if code.contains("Vec::new") {
            suggestions.push(OptimizationSuggestion {
                category: OptimizationCategory::Memory,
                description: "Vec allocation without capacity".to_string(),
                impact_score: 0.3,
                suggested_implementation: Some("Consider using Vec::with_capacity".to_string()),
            });
        }

        suggestions
    }
}
