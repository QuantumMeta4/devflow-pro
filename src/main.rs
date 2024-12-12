use clap::Parser;
use devflow_pro::{analyze_codebase, AppConfig, DevFlowError, ProjectInsights, Result};
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

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Output format (json or text)
    #[arg(short, long, default_value = "text")]
    format: String,
}

fn main() {
    env_logger::init();

    match run() {
        Ok(_) => (),
        Err(e) => {
            error!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn run() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        info!("Starting analysis of {:?}", args.path);
    }

    if !args.path.exists() {
        return Err(DevFlowError::InvalidPath(
            format!("Path does not exist: {:?}", args.path)
        ));
    }

    let config = AppConfig {
        max_file_size: args.max_file_size,
        ..AppConfig::default()
    };

    let insights = analyze_codebase(&args.path, config)?;

    match args.output {
        Some(path) => {
            let json = serde_json::to_string_pretty(&insights)
                .map_err(|e| DevFlowError::Serialization(e.to_string()))?;
            fs::write(&path, json)?;
            if args.verbose {
                info!("Analysis results written to: {:?}", path);
            }
        }
        None => {
            if args.format == "json" {
                println!("{}", serde_json::to_string_pretty(&insights)
                    .map_err(|e| DevFlowError::Serialization(e.to_string()))?);
            } else {
                print_formatted_insights(&insights);
            }
        }
    }

    if args.verbose {
        info!("Analysis completed successfully");
    }

    Ok(())
}

fn print_formatted_insights(insights: &ProjectInsights) {
    println!("\n📊 DevFlow Pro Analysis Report");
    println!("═══════════════════════════════\n");

    println!("📈 Overall Statistics");
    println!("──────────────────────");
    println!("Files Analyzed: {}", insights.files_analyzed);
    println!("Total Lines of Code: {}", insights.total_lines);
    println!();

    println!("🗂  Language Distribution");
    println!("──────────────────────");
    for (lang, count) in &insights.language_stats {
        println!("  {} files: {}", lang, count);
    }
    println!();

    println!("📝 Top Files by Complexity");
    println!("──────────────────────");
    let mut files: Vec<_> = insights.metrics_by_file.iter().collect();
    files.sort_by(|a, b| b.1.complexity.partial_cmp(&a.1.complexity).unwrap());
    for (path, metrics) in files.iter().take(5) {
        println!("  {} (Complexity: {:.1})", path, metrics.complexity);
        println!("    Lines: {}, Comments: {}", metrics.lines_of_code, metrics.comment_lines);
        if !metrics.dependencies.is_empty() {
            println!("    Dependencies: {}", metrics.dependencies.join(", "));
        }
        println!();
    }

    if !insights.security_summary.is_empty() {
        println!("⚠️  Security Issues");
        println!("──────────────────────");
        for issue in &insights.security_summary {
            println!("  • {}", issue.description);
            if let Some(line) = issue.line_number {
                println!("    Line: {}", line);
            }
            println!();
        }
    }

    println!("Analysis completed at: {}\n", insights.analysis_timestamp);
}
