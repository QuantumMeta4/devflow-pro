use chrono::{DateTime, Utc};
use ignore::Walk;
use log::{error, warn};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::Path,
    sync::{Arc, Mutex},
};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, DevFlowError>;

#[derive(Error, Debug)]
pub enum DevFlowError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Analysis error: {0}")]
    Analysis(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Thread error: {0}")]
    Thread(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodeMetrics {
    pub lines_of_code: usize,
    pub comment_lines: usize,
    pub complexity: f64,
    pub dependencies: Vec<String>,
    pub security_issues: Vec<SecurityIssue>,
    pub last_modified: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecurityIssue {
    pub severity: IssueSeverity,
    pub description: String,
    pub line_number: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IssueSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProjectInsights {
    pub files_analyzed: usize,
    pub total_lines: usize,
    pub language_stats: HashMap<String, usize>,
    pub metrics_by_file: HashMap<String, CodeMetrics>,
    pub security_summary: Vec<SecurityIssue>,
    pub analysis_timestamp: DateTime<Utc>,
}

impl ProjectInsights {
    #[must_use]
    pub fn new() -> Self {
        Self {
            files_analyzed: 0,
            total_lines: 0,
            language_stats: HashMap::new(),
            metrics_by_file: HashMap::new(),
            security_summary: Vec::new(),
            analysis_timestamp: Utc::now(),
        }
    }
}

/// Analyzes a codebase at the given path using the provided configuration.
///
/// # Arguments
/// * `path` - The path to the codebase to analyze
/// * `config` - Configuration for the analysis
///
/// # Returns
/// * `Result<ProjectInsights>` - Analysis results or an error
///
/// # Errors
/// This function can return the following errors:
/// * `DevFlowError::Io` - If there are file system access issues
/// * `DevFlowError::Thread` - If there are issues with parallel processing
/// * `DevFlowError::InvalidPath` - If the provided path is invalid
pub fn analyze_codebase(path: &Path, config: &AppConfig) -> Result<ProjectInsights> {
    let insights = Arc::new(Mutex::new(ProjectInsights::new()));

    #[allow(clippy::redundant_closure_for_method_calls)]
    let walker = Walk::new(path)
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.file_type().is_some_and(|ft| ft.is_file())
                && !is_ignored(entry.path(), &config.ignored_patterns)
        })
        .collect::<Vec<_>>();

    walker
        .par_iter()
        .try_for_each(|entry| analyze_file(entry.path(), &insights, config))?;

    Arc::try_unwrap(insights)
        .map_err(|_| DevFlowError::Thread("Failed to unwrap Arc".into()))?
        .into_inner()
        .map_err(|_| DevFlowError::Thread("Failed to acquire lock".into()))
}

fn analyze_file(
    path: &Path,
    insights: &Arc<Mutex<ProjectInsights>>,
    config: &AppConfig,
) -> Result<()> {
    let content = fs::read_to_string(path).map_err(DevFlowError::Io)?;

    if content.len() > config.max_file_size {
        warn!("File too large to analyze: {:?}", path);
        return Ok(());
    }

    let metrics = calculate_metrics(path, &content, config)?;
    let file_path = path.to_string_lossy().into_owned();

    let mut insights = insights
        .lock()
        .map_err(|_| DevFlowError::Thread("Failed to acquire lock".into()))?;

    insights.files_analyzed += 1;
    insights.total_lines += metrics.lines_of_code;

    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("unknown")
        .to_lowercase();

    *insights.language_stats.entry(extension).or_insert(0) += 1;
    insights.metrics_by_file.insert(file_path, metrics.clone());
    insights.security_summary.extend(metrics.security_issues);

    drop(insights);
    Ok(())
}

fn calculate_metrics(path: &Path, content: &str, config: &AppConfig) -> Result<CodeMetrics> {
    let lines: Vec<&str> = content.lines().collect();
    let lines_of_code = lines.len();
    let comment_lines = count_comments(&lines);
    let complexity = calculate_complexity(&lines);
    let dependencies = extract_dependencies(&lines);
    let security_issues = check_security_issues(&lines, config);
    let last_modified = fs::metadata(path)
        .map_err(DevFlowError::Io)?
        .modified()
        .map_err(DevFlowError::Io)?
        .into();

    Ok(CodeMetrics {
        lines_of_code,
        comment_lines,
        complexity,
        dependencies,
        security_issues,
        last_modified,
    })
}

fn is_ignored(path: &Path, ignored_patterns: &[String]) -> bool {
    let path_str = path.to_string_lossy();
    ignored_patterns
        .iter()
        .any(|pattern| path_str.contains(pattern))
}

fn count_comments(lines: &[&str]) -> usize {
    lines
        .iter()
        .filter(|line| {
            let trimmed = line.trim();
            trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*')
        })
        .count()
}

fn calculate_complexity(lines: &[&str]) -> f64 {
    let control_flow_keywords = [
        "if", "else", "while", "for", "match", "loop", "break", "continue", "return",
    ];

    let mut complexity = 1.0;
    for line in lines {
        let line = line.trim();
        for keyword in &control_flow_keywords {
            if line.starts_with(keyword) {
                complexity += 1.0;
            }
        }
    }
    complexity
}

fn extract_dependencies(lines: &[&str]) -> Vec<String> {
    let mut deps = Vec::new();
    for line in lines {
        if line.trim().starts_with("use ") {
            if let Some(dep) = line
                .trim()
                .strip_prefix("use ")
                .and_then(|s| s.split("::").next())
            {
                deps.push(dep.to_string());
            }
        }
    }
    deps.sort();
    deps.dedup();
    deps
}

fn check_security_issues(lines: &[&str], config: &AppConfig) -> Vec<SecurityIssue> {
    let mut issues = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        for pattern in &config.security_patterns {
            if line.contains(pattern) {
                issues.push(SecurityIssue {
                    severity: IssueSeverity::High,
                    description: format!("Found security pattern: {pattern}"),
                    line_number: Some(i + 1),
                });
            }
        }
    }

    issues
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub max_file_size: usize,
    pub ignored_patterns: Vec<String>,
    pub security_patterns: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            max_file_size: 1024 * 1024, // 1MB
            ignored_patterns: vec![
                String::from(".git"),
                String::from("target"),
                String::from("node_modules"),
            ],
            security_patterns: vec![
                String::from("eval\\s*\\("),
                String::from("exec\\s*\\("),
                String::from("password\\s*="),
            ],
        }
    }
}
