use clap::Parser;
use devflow_pro::ai::{
    types::{AnalysisType, LlamaConfig},
    Coder,
};
use devflow_pro::{
    analyze_codebase, AppConfig, DevFlowError, IssueSeverity, ProjectInsights, Result,
};
use log::{error, info};
use std::{fs, path::PathBuf, process};

#[derive(Parser, Debug)]
#[command(
    name = "DevFlow Pro",
    version = "1.0.0",
    author = "DevFlow Team",
    about = "A comprehensive static code analysis tool",
    long_about = None
)]
struct Args {
    /// Path to analyze (defaults to current directory)
    #[arg(short, long, default_value = ".")]
    path: PathBuf,

    /// Output file path (JSON format)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Maximum file size to analyze in bytes
    #[arg(long, default_value = "1048576")] // 1MB
    max_file_size: usize,

    /// Minimum severity level for issues (low, medium, high, critical)
    #[arg(long, default_value = "low")]
    min_severity: String,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Output format (json or text)
    #[arg(short, long, default_value = "text")]
    format: String,

    /// Log file path
    #[arg(short, long)]
    log_file: Option<PathBuf>,

    /// Ignore patterns
    #[arg(short, long)]
    ignore: Option<Vec<String>>,

    /// Security patterns
    #[arg(short, long)]
    security_patterns: Option<Vec<String>>,

    /// Enable AI analysis
    #[arg(long)]
    ai: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(
        if Args::parse().verbose {
            "debug"
        } else {
            "info"
        },
    ))
    .init();

    info!("Starting analysis of {}", Args::parse().path.display());

    // Run the static analysis
    match run().await {
        Ok(insights) => {
            print_formatted_insights(&insights);
            Ok(())
        }
        Err(e) => {
            error!("Error: {}", e);
            process::exit(1);
        }
    }
}

async fn run() -> Result<ProjectInsights> {
    let args = Args::parse();

    if let Some(log_file) = &args.log_file {
        if let Err(e) = fs::File::create(log_file) {
            return Err(DevFlowError::Io(e));
        }
    }

    let target_dir = args.path.canonicalize()?;

    if !target_dir.exists() {
        return Err(DevFlowError::InvalidPath(format!("Path does not exist: {target_dir:?}")));
    }

    let mut ignored_patterns = vec![
        "**/node_modules/**".to_string(),
        "**/target/**".to_string(),
        "**/dist/**".to_string(),
        "**/build/**".to_string(),
        "**/.git/**".to_string(),
    ];
    if let Some(patterns) = args.ignore {
        ignored_patterns.extend(patterns);
    }

    let config = AppConfig {
        ignored_patterns,
        max_file_size: args.max_file_size,
        min_severity: match args.min_severity.as_str() {
            "medium" => IssueSeverity::Medium,
            "high" => IssueSeverity::High,
            "critical" => IssueSeverity::Critical,
            _ => IssueSeverity::Low,
        },
        security_patterns: args.security_patterns.unwrap_or_default(),
    };

    let insights = analyze_codebase(&target_dir, &config)?;

    // Run AI analysis if enabled
    if args.ai {
        let llama = Coder::new(LlamaConfig::default())?;

        // Get the top 5 most complex files for AI analysis
        let mut files: Vec<_> = insights.file_metrics.iter().collect();
        files.sort_by(|a, b| b.1.complexity.partial_cmp(&a.1.complexity).unwrap());

        for (path, metrics) in files.iter().take(5) {
            if let Ok(content) = fs::read_to_string(path) {
                info!("Running AI analysis on: {}", path);

                // Extract a representative sample of the code
                let sample = if content.len() > 5000 {
                    let lines: Vec<&str> = content.lines().collect();
                    let total_lines = lines.len();
                    let sample_lines = std::cmp::min(50, total_lines);
                    let stride = if total_lines > sample_lines {
                        total_lines / sample_lines
                    } else {
                        1
                    };

                    let mut sample = String::new();
                    for i in (0..total_lines).step_by(stride).take(sample_lines) {
                        sample.push_str(lines[i]);
                        sample.push('\n');
                    }
                    sample
                } else {
                    content.clone()
                };

                // Run different types of analysis
                let review = llama
                    .analyze_code(&sample, AnalysisType::CodeReview)
                    .await?;
                let security = llama
                    .analyze_code(&sample, AnalysisType::SecurityAudit)
                    .await?;
                let optimization = llama
                    .analyze_code(&sample, AnalysisType::Optimization)
                    .await?;

                println!("\n🤖 AI Analysis for {path}");
                println!("═══════════════════════════════\n");

                println!("📁 File Info");
                println!("──────────");
                println!("Lines of Code: {}", metrics.lines_of_code);
                println!("Complexity: {:.1}", metrics.complexity);
                println!("Comments: {}\n", metrics.comment_lines);

                println!("📝 Code Review");
                println!("──────────────");
                println!("{}\n", review.summary);

                println!("🔒 Security Analysis");
                println!("──────────────────");
                println!("{}\n", security.summary);

                println!("⚡ Optimization Suggestions");
                println!("────────────────────────");
                println!("{}\n", optimization.summary);
            }
        }
    }

    match args.output {
        Some(path) => {
            let json = serde_json::to_string_pretty(&insights)
                .map_err(|e| DevFlowError::Serialization(e.to_string()))?;
            fs::write(&path, json.as_bytes())?;
            if args.verbose {
                info!("Analysis results written to: {:?}", path);
            }
        }
        None => {
            if args.format == "json" {
                let json = serde_json::to_string_pretty(&insights)
                    .map_err(|e| DevFlowError::Serialization(e.to_string()))?;
                println!("{json}");
            } else {
                print_formatted_insights(&insights);
            }
        }
    }

    if args.verbose {
        info!("Analysis completed successfully");
    }

    Ok(insights)
}

fn print_formatted_insights(insights: &ProjectInsights) {
    println!("\n📊 DevFlow Pro Analysis Report");
    println!("═══════════════════════════════\n");

    println!("📈 Overall Statistics");
    println!("──────────────────────");
    println!(
        "Files Analyzed: {files_analyzed}",
        files_analyzed = insights.files_analyzed
    );
    println!(
        "Total Lines of Code: {total_lines}",
        total_lines = insights.total_lines
    );
    println!();

    print_language_stats(insights);

    println!("📝 Top Files by Complexity");
    println!("──────────────────────");
    let mut files: Vec<_> = insights.file_metrics.iter().collect();
    files.sort_by(|a, b| b.1.complexity.partial_cmp(&a.1.complexity).unwrap());
    for (path, metrics) in files.iter().take(5) {
        println!("  {path} (Complexity: {:.1})", metrics.complexity);
        println!(
            "    Lines: {lines}, Comments: {comments}",
            lines = metrics.lines_of_code,
            comments = metrics.comment_lines
        );
        if !metrics.dependencies.is_empty() {
            println!(
                "    Dependencies: {deps}",
                deps = metrics.dependencies.join(", ")
            );
        }
        println!();
    }

    if !insights.security_summary.is_empty() {
        println!("⚠️  Security Issues");
        println!("──────────────────────");
        for issue in &insights.security_summary {
            println!("  • {desc}", desc = issue.description);
            if let Some(line) = issue.line_number {
                println!("    Line: {line}");
            }
            println!();
        }
    }

    println!(
        "Analysis completed at: {timestamp}",
        timestamp = insights.analysis_timestamp
    );
}

fn print_language_stats(insights: &ProjectInsights) {
    println!("🗂  Language Distribution");
    println!("──────────────────────");
    let mut lang_dist: Vec<_> = insights.language_distribution.iter().collect();
    lang_dist.sort_by(|a, b| b.1.cmp(a.1));
    for (lang, count) in lang_dist {
        println!("  {lang} files: {count}");
    }
    println!();
}
