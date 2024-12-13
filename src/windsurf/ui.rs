use super::{Position, WindsurfAnalysisResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Default for Range {
    fn default() -> Self {
        Self {
            start: Position {
                line: 0,
                column: 0,
                offset: 0,
            },
            end: Position {
                line: 0,
                column: 0,
                offset: 0,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsDisplay {
    pub complexity_score: f64,
    pub quality_score: f64,
}

impl MetricsDisplay {
    pub fn new(analysis_result: &WindsurfAnalysisResult) -> Self {
        Self {
            complexity_score: analysis_result.ai_insights.semantic_complexity,
            quality_score: analysis_result.ai_insights.code_quality_score,
        }
    }

    pub fn to_status_bar_text(&self) -> String {
        format!(
            "Complexity: {:.1} | Quality: {:.1}",
            self.complexity_score, self.quality_score
        )
    }
}
