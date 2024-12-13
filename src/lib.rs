use chrono::{DateTime, Utc};
use globset::{Glob, GlobSetBuilder};
use ignore::Walk;
use log::error;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use thiserror::Error;

pub mod ai;
pub mod ai_enhanced;
pub mod analysis;

#[derive(Debug, Error)]
pub enum DevFlowError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Thread error: {0}")]
    Thread(String),

    #[error("Semantic error: {0}")]
    Semantic(#[from] analysis::semantic::SemanticError),

    #[error("AI error: {0}")]
    AI(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Serialization error: {0}")]
    Serialization(String),
}

pub type Result<T> = std::result::Result<T, DevFlowError>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodeMetrics {
    pub lines_of_code: usize,
    pub blank_lines: usize,
    pub comment_lines: usize,
    pub complexity: f64,
    pub dependencies: Vec<String>,
    pub security_issues: Vec<SecurityIssue>,
    pub last_modified: DateTime<Utc>,
    pub size_bytes: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecurityIssue {
    pub severity: IssueSeverity,
    pub description: String,
    pub line_number: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
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
    pub language_distribution: HashMap<String, usize>,
    pub file_metrics: HashMap<String, CodeMetrics>,
    pub security_summary: Vec<SecurityIssue>,
    pub analysis_timestamp: DateTime<Utc>,
}

impl ProjectInsights {
    #[must_use]
    pub fn new() -> Self {
        Self {
            files_analyzed: 0,
            total_lines: 0,
            language_distribution: HashMap::new(),
            file_metrics: HashMap::new(),
            security_summary: Vec::new(),
            analysis_timestamp: Utc::now(),
        }
    }

    pub fn with_ai_provider(_ai_provider: Arc<dyn ai_enhanced::AIProvider>) -> Self {
        let mut insights = Self::new();
        insights.analysis_timestamp = Utc::now();
        insights
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

    let walker = Walk::new(path)
        .filter_map(|entry| {
            match entry {
                Ok(entry) => {
                    // Only process files
                    if entry.file_type().is_some_and(|ft| ft.is_file()) {
                        Some(entry)
                    } else {
                        None
                    }
                }
                Err(e) => {
                    error!("Error walking directory: {}", e);
                    None
                }
            }
        })
        .collect::<Vec<_>>();

    // Process files in parallel
    walker
        .par_iter()
        .try_for_each(|entry| analyze_file(entry.path(), &insights, config))?;

    // Return the final insights
    let final_insights = insights
        .lock()
        .map_err(|_| DevFlowError::Thread("Failed to acquire lock".into()))?
        .clone();

    Ok(final_insights)
}

/// Walk through the directory and analyze each file
///
/// # Errors
///
/// Returns an error if:
/// - Cannot read directory entries
/// - Cannot get file type of an entry
/// - Cannot analyze a file or subdirectory
pub fn analyze_directory(
    dir_path: &Path,
    insights: &Arc<Mutex<ProjectInsights>>,
    config: &AppConfig,
) -> Result<()> {
    let dir = fs::read_dir(dir_path)?;

    for entry in dir {
        let entry = entry?;
        let path = entry.path();

        if entry.file_type()?.is_dir() {
            analyze_directory(&path, insights, config)?;
        } else if entry.file_type().is_ok_and(|ft| ft.is_file()) {
            analyze_file(&path, insights, config)?;
        }
    }

    Ok(())
}

/// Analyze a single file and update project insights
///
/// # Errors
///
/// Returns an error if:
/// - File metadata cannot be read
/// - File contents cannot be read
/// - Failed to acquire insights lock
/// - Failed to analyze code with LLM
pub fn analyze_file(
    path: &Path,
    insights: &Arc<Mutex<ProjectInsights>>,
    config: &AppConfig,
) -> Result<()> {
    // Skip files that are too large
    let metadata = fs::metadata(path)?;
    if metadata.len() > config.max_file_size as u64 {
        return Ok(());
    }

    // Skip ignored files
    if is_ignored(path, &config.ignored_patterns) {
        return Ok(());
    }

    // Read file contents, handling non-UTF-8 files
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            // Try reading as raw bytes if UTF-8 conversion fails
            match std::fs::read(path) {
                Ok(_) => {
                    // Log the non-UTF-8 file but continue processing
                    error!("Skipping non-UTF-8 file: {}", path.display());
                    return Ok(());
                }
                Err(read_err) => {
                    error!(
                        "Failed to read file {}: {} (Original error: {})",
                        path.display(),
                        read_err,
                        e
                    );
                    return Ok(());
                }
            }
        }
    };

    let lines: Vec<&str> = content.lines().collect();

    // Calculate metrics
    let metrics = calculate_metrics(path, &content, config)?;
    let language = normalize_language_extension(
        path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("unknown"),
    );
    let security_issues = check_security_issues(&lines, config);

    // Acquire lock and update insights
    let mut insights_guard = insights
        .lock()
        .map_err(|_| DevFlowError::Thread("Failed to acquire lock".into()))?;

    // Increment files analyzed
    insights_guard.files_analyzed += 1;

    // Update total lines
    insights_guard.total_lines += lines.len();

    // Update language distribution
    *insights_guard
        .language_distribution
        .entry(language)
        .or_insert(0) += 1;

    // Store file metrics
    insights_guard
        .file_metrics
        .insert(path.to_string_lossy().into_owned(), metrics);

    // Extend security summary
    insights_guard.security_summary.extend(security_issues);
    drop(insights_guard);

    Ok(())
}

fn calculate_metrics(path: &Path, content: &str, config: &AppConfig) -> Result<CodeMetrics> {
    let lines: Vec<&str> = content.lines().collect();
    let mut metrics = CodeMetrics {
        lines_of_code: 0,
        blank_lines: 0,
        comment_lines: 0,
        complexity: 0.0,
        dependencies: Vec::new(),
        security_issues: Vec::new(),
        last_modified: Utc::now(),
        size_bytes: path.metadata()?.len(),
    };

    // Update file metadata
    metrics.last_modified = path.metadata()?.modified()?.into();

    // Process file content
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            metrics.blank_lines += 1;
        } else if trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed.starts_with("/*")
        {
            metrics.comment_lines += 1;
        }
        metrics.lines_of_code += 1;
    }

    // Calculate complexity
    #[allow(clippy::cast_precision_loss)]
    {
        let branches = content.matches("if ").count()
            + content.matches("while ").count()
            + content.matches("for ").count()
            + content.matches("match ").count();
        metrics.complexity = (branches as f64).mul_add(0.1, 1.0);
    }

    // Extract dependencies
    metrics.dependencies = extract_dependencies(content);

    // Check security issues
    metrics.security_issues = check_security_issues(&lines, config);

    Ok(metrics)
}

fn extract_dependencies(content: &str) -> Vec<String> {
    let mut deps = Vec::new();

    // Process each line
    for line in content.lines() {
        let trimmed = line.trim();

        // Rust dependencies
        if trimmed.starts_with("use ") {
            if let Some(dep) = trimmed
                .strip_prefix("use ")
                .and_then(|s| s.split("::").next())
            {
                deps.push(dep.to_string());
            }
        }
        // Python imports
        else if trimmed.starts_with("import ") || trimmed.starts_with("from ") {
            if let Some(dep) = trimmed
                .split_whitespace()
                .nth(1)
                .map(|s| s.split('.').next().unwrap_or(s))
            {
                deps.push(dep.to_string());
            }
        }
        // JavaScript/TypeScript imports
        else if trimmed.starts_with("import ") || trimmed.starts_with("require(") {
            if let Some(dep) = if trimmed.contains("from ") {
                trimmed.split("from ").nth(1).and_then(|s| {
                    s.trim_matches(|c| c == '\'' || c == '"' || c == ';')
                        .split('/')
                        .next()
                })
            } else if trimmed.contains("require(") {
                trimmed.split("require(").nth(1).and_then(|s| {
                    s.trim_matches(|c| c == '\'' || c == '"' || c == ')' || c == ';')
                        .split('/')
                        .next()
                })
            } else {
                None
            } {
                deps.push(dep.to_string());
            }
        }
        // Go imports
        else if trimmed.starts_with("import (") || trimmed.starts_with("import \"") {
            if let Some(dep) = trimmed
                .trim_matches(|c| c == '"' || c == '(' || c == ')')
                .split('/')
                .next()
            {
                deps.push(dep.to_string());
            }
        }
    }

    deps.sort();
    deps.dedup();
    deps
}

fn is_ignored(path: &Path, ignored_patterns: &[String]) -> bool {
    let mut builder = GlobSetBuilder::new();

    for pattern in ignored_patterns {
        if let Ok(glob) = Glob::new(pattern) {
            builder.add(glob);
        }
    }

    if let Ok(set) = builder.build() {
        set.is_match(path)
    } else {
        false
    }
}

fn normalize_language_extension(ext: &str) -> String {
    match ext.to_lowercase().as_str() {
        // JavaScript variants
        "js" | "mjs" | "cjs" => "js".to_string(),
        // TypeScript variants
        "ts" | "tsx" | "mts" => "ts".to_string(),
        // Markdown variants
        "md" | "markdown" => "md".to_string(),
        // Handle special cases
        "1" => "man".to_string(), // Man page files
        "bsd" => "config".to_string(),
        "flow" => "type_def".to_string(),
        "bnf" => "grammar".to_string(),
        // Default case
        _ => ext.to_lowercase(),
    }
}

fn check_security_issues(lines: &[&str], config: &AppConfig) -> Vec<SecurityIssue> {
    let mut issues = Vec::new();

    for (line_num, &line) in lines.iter().enumerate() {
        let line_num = line_num + 1;

        // Check for hardcoded secrets
        if (line.contains("password") || line.contains("secret") || line.contains("api_key"))
            && (line.contains('=') || line.contains(':'))
        {
            issues.push(SecurityIssue {
                severity: IssueSeverity::High,
                description: format!("Potential hardcoded secret found at line {line_num}: {line}"),
                line_number: Some(line_num),
            });
        }

        // Check for SQL injection vulnerabilities
        if (line.contains("SELECT") || line.contains("INSERT") || line.contains("UPDATE"))
            && (line.contains('\"') || line.contains('\''))
        {
            issues.push(SecurityIssue {
                severity: IssueSeverity::High,
                description: format!(
                    "Potential SQL injection vulnerability at line {line_num}: {line}"
                ),
                line_number: Some(line_num),
            });
        }

        // Check for unsafe file operations
        if line.contains("eval(") || line.contains("exec(") {
            issues.push(SecurityIssue {
                severity: IssueSeverity::Critical,
                description: format!("Unsafe code execution found at line {line_num}: {line}"),
                line_number: Some(line_num),
            });
        }
    }

    // Filter out issues based on configuration
    issues
        .into_iter()
        .filter(|issue| issue.severity >= config.min_severity)
        .collect()
}

/// Format project insights into a human-readable report
///
/// # Panics
///
/// Panics if file complexity comparison fails due to NaN values
#[must_use]
pub fn format_report(insights: &ProjectInsights) -> String {
    let mut output = String::new();

    // Header
    output.push_str("📊 DevFlow Pro Analysis Report\n");
    output.push_str("═══════════════════════════════\n\n");

    // Basic Statistics
    output.push_str("📈 Overall Statistics\n");
    output.push_str("──────────────────────\n");
    output.push_str(&format!(
        "Files Analyzed: {}\n",
        insights.file_metrics.len()
    ));
    output.push_str(&format!("Total Lines of Code: {}\n", insights.total_lines));

    #[allow(clippy::cast_precision_loss)]
    let avg_lines = if insights.file_metrics.is_empty() {
        0.0
    } else {
        (insights.total_lines as f64) / (insights.file_metrics.len() as f64)
    };
    output.push_str(&format!("Average Lines per File: {avg_lines:.1}\n\n"));

    // Language Distribution
    output.push_str("🗂  Language Distribution\n");
    output.push_str("──────────────────────\n");
    let mut lang_dist: Vec<_> = insights.language_distribution.iter().collect();
    lang_dist.sort_by(|a, b| b.1.cmp(a.1));
    for (ext, count) in lang_dist {
        output.push_str(&format!("{ext:<8} files: {count:>3}\n"));
    }
    output.push('\n');

    // Top Files by Complexity
    output.push_str("📝 Top Files by Complexity\n");
    output.push_str("──────────────────────\n");
    let mut files: Vec<_> = insights.file_metrics.iter().collect();
    files.sort_by(|a, b| b.1.complexity.partial_cmp(&a.1.complexity).unwrap());
    for (i, (path, metrics)) in files.iter().take(5).enumerate() {
        output.push_str(&format!("{}. {}\n", i + 1, path));
        output.push_str(&format!("   Complexity: {:.1}\n", metrics.complexity));
        output.push_str(&format!("   Lines: {}\n", metrics.lines_of_code));
        output.push_str(&format!("   Blank Lines: {}\n", metrics.blank_lines));
        output.push_str(&format!("   Comments: {}\n", metrics.comment_lines));
        if !metrics.dependencies.is_empty() {
            output.push_str(&format!(
                "   Dependencies: {}\n",
                metrics.dependencies.join(", ")
            ));
        }
        output.push('\n');
    }

    // Code Quality Metrics
    output.push_str("📊 Code Quality Metrics\n");
    output.push_str("──────────────────────\n");
    #[allow(clippy::cast_precision_loss)]
    let comment_ratio = if insights.total_lines > 0 {
        let total_comments = files.iter().map(|(_, m)| m.comment_lines).sum::<usize>();
        (total_comments as f64) / (insights.total_lines as f64) * 100.0
    } else {
        0.0
    };
    output.push_str(&format!("Comment Ratio: {comment_ratio:.1}%\n"));

    #[allow(clippy::cast_precision_loss)]
    let avg_complexity =
        files.iter().map(|(_, m)| m.complexity).sum::<f64>() / (files.len() as f64);
    output.push_str(&format!("Average Complexity: {avg_complexity:.2}\n\n"));

    // File Size Distribution
    output.push_str("📦 File Size Distribution\n");
    output.push_str("──────────────────────\n");
    let mut size_categories = HashMap::new();
    for metrics in insights.file_metrics.values() {
        let category = match metrics.size_bytes {
            0..=1000 => "Small (0-1KB)",
            1001..=10_000 => "Medium (1-10KB)",
            10_001..=100_000 => "Large (10-100KB)",
            _ => "Very Large (>100KB)",
        };
        *size_categories.entry(category).or_insert(0) += 1;
    }
    let categories = [
        "Small (0-1KB)",
        "Medium (1-10KB)",
        "Large (10-100KB)",
        "Very Large (>100KB)",
    ];
    for cat in &categories {
        if let Some(count) = size_categories.get(cat) {
            output.push_str(&format!("{cat}: {count} files\n"));
        }
    }
    output.push('\n');

    // Recently Modified Files
    output.push_str("🕒 Recently Modified Files\n");
    output.push_str("──────────────────────\n");
    let mut recent_files: Vec<_> = insights.file_metrics.iter().collect();
    recent_files.sort_by(|a, b| b.1.last_modified.cmp(&a.1.last_modified));
    for (path, metrics) in recent_files.iter().take(5) {
        output.push_str(&format!(
            "- {}\n  Last Modified: {}\n",
            path,
            metrics.last_modified.format("%Y-%m-%d %H:%M:%S")
        ));
    }
    output.push('\n');

    // Timestamp
    output.push_str(&format!(
        "Analysis completed at: {}\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    ));

    output
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub max_file_size: usize,
    pub ignored_patterns: Vec<String>,
    pub security_patterns: Vec<String>,
    pub min_severity: IssueSeverity,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            max_file_size: 1024 * 1024, // 1MB
            ignored_patterns: vec![
                "**/target/**".to_string(),
                "**/.git/**".to_string(),
                "**/.idea/**".to_string(),
                "**/.vscode/**".to_string(),
                "**/.DS_Store".to_string(),
                "**/node_modules/**".to_string(),
                "**/dist/**".to_string(),
                "**/build/**".to_string(),
            ],
            security_patterns: vec![
                // Command Injection
                String::from("eval\\s*\\("),
                String::from("exec\\s*\\("),
                String::from("system\\s*\\("),
                String::from("shell_exec\\s*\\("),
                // Hardcoded Secrets
                String::from("password\\s*="),
                String::from("api[_-]?key\\s*="),
                String::from("secret\\s*="),
                String::from("token\\s*="),
                // SQL Injection
                String::from("execute\\s*\\("),
                String::from("raw\\s*sql"),
                String::from("\\.query\\s*\\("),
                // File Operations
                String::from("file_get_contents\\s*\\("),
                String::from("fopen\\s*\\("),
                String::from("readFile\\s*\\("),
                // Unsafe Deserialization
                String::from("unserialize\\s*\\("),
                String::from("JSON\\.parse\\s*\\("),
                String::from("fromJson\\s*\\("),
                // XSS Vulnerabilities
                String::from("innerHTML\\s*="),
                String::from("document\\.write\\s*\\("),
                String::from("\\$\\s*\\("),
            ],
            min_severity: IssueSeverity::Low,
        }
    }
}
