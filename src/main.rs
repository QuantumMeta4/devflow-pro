use std::{env, fs, path::{Path, PathBuf}, time::Instant, collections::HashMap};

#[derive(Default)]
struct ProjectInsights {
    metrics: Metrics,
    tech_stack: TechStack,
    quality: CodeQuality,
    patterns: ArchitecturePatterns,
    recommendations: Vec<Recommendation>,
}

#[derive(Default)]
struct Metrics {
    lines_of_code: usize,
    files: usize,
    complexity: f32,
    test_coverage: f32,
}

#[derive(Default)]
struct TechStack {
    languages: HashMap<String, usize>,
    frameworks: Vec<String>,
    databases: Vec<String>,
    cloud_services: Vec<String>,
}

#[derive(Default)]
struct CodeQuality {
    duplications: Vec<(String, f32)>,
    security_issues: Vec<String>,
    performance_hotspots: Vec<String>,
    best_practices: Vec<String>,
}

#[derive(Default)]
struct ArchitecturePatterns {
    design_patterns: Vec<String>,
    anti_patterns: Vec<String>,
    architecture_style: String,
}

struct Recommendation {
    category: String,
    priority: u8,
    description: String,
    impact: String,
    effort: String,
}

fn main() {
    let start = Instant::now();
    println!("🚀 DevFlow Pro - Advanced Codebase Analysis");
    let path = env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let insights = analyze_codebase(Path::new(&path));
    display_insights(&insights);
    generate_detailed_report(&insights);
    suggest_optimizations(&insights);
    println!("\n✨ Analysis completed in {:.2}s", start.elapsed().as_secs_f32());
}

fn analyze_codebase(root: &Path) -> ProjectInsights {
    let mut insights = ProjectInsights::default();
    analyze_structure(root, &mut insights);
    detect_code_patterns(&mut insights);
    evaluate_quality(&mut insights);
    generate_recommendations(&mut insights);
    insights
}

fn analyze_structure(dir: &Path, insights: &mut ProjectInsights) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if !is_excluded_directory(&path) {
                    analyze_structure(&path, insights);
                }
            } else {
                analyze_file(&path, insights);
            }
        }
    }
}

fn is_excluded_directory(path: &Path) -> bool {
    let excluded = ["node_modules", "target", "dist", "build", ".git", "vendor"];
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|n| excluded.contains(&n))
        .unwrap_or(false)
}

fn analyze_file(path: &PathBuf, insights: &mut ProjectInsights) {
    if let Ok(content) = fs::read_to_string(path) {
        insights.metrics.files += 1;
        insights.metrics.lines_of_code += content.lines().count();
        if path.to_string_lossy().contains("test") {
            insights.metrics.test_coverage += 1.0;
        }
        detect_language(path, &content, insights);
        analyze_complexity(&content, insights);
        detect_frameworks(path, &content, insights);
        find_security_issues(&content, insights);
        detect_architecture_patterns(&content, insights);
        detect_databases(&content, insights);
        detect_cloud_services(&content, insights);
        analyze_code_duplication(&content, insights);
        detect_performance_issues(&content, insights);
    }
}

fn detect_performance_issues(content: &str, insights: &mut ProjectInsights) {
    let patterns = [
        (".*\\*.*", "Inefficient wildcard operation"),
        ("Thread\\.sleep", "Blocking thread sleep"),
        ("for.*for.*for", "Nested triple loop"),
    ];
    
    for (pattern, issue) in patterns {
        if content.contains(pattern) {
            insights.quality.performance_hotspots.push(issue.to_string());
        }
    }
}

fn detect_language(path: &Path, _content: &str, insights: &mut ProjectInsights) {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let language = match ext {
        "rs" => Some("Rust"),
        "go" => Some("Go"),
        "js" | "jsx" => Some("JavaScript"),
        "ts" | "tsx" => Some("TypeScript"),
        "py" => Some("Python"),
        "java" => Some("Java"),
        "rb" => Some("Ruby"),
        "php" => Some("PHP"),
        "swift" => Some("Swift"),
        "kt" => Some("Kotlin"),
        "cpp" | "hpp" | "cc" | "h" => Some("C++"),
        "c" => Some("C"),
        "cs" => Some("C#"),
        _ => None
    };
    if let Some(lang) = language {
        *insights.tech_stack.languages.entry(lang.to_string()).or_insert(0) += 1;
    }
}

fn analyze_complexity(content: &str, insights: &mut ProjectInsights) {
    let line_count = content.lines().count();
    let nested_count = content.matches("{").count();
    let conditional_count = content.matches("if").count() + 
                          content.matches("match").count() +
                          content.matches("while").count();
    if line_count > 0 {
        insights.metrics.complexity += (nested_count as f32 + conditional_count as f32) / (line_count as f32);
    }
}

fn detect_frameworks(path: &Path, content: &str, insights: &mut ProjectInsights) {
    let frameworks = match path.file_name().and_then(|n| n.to_str()) {
        Some("package.json") => detect_js_frameworks(content),
        Some("Cargo.toml") => detect_rust_frameworks(content),
        Some("go.mod") => detect_go_frameworks(content),
        Some("requirements.txt") => detect_python_frameworks(content),
        Some("composer.json") => detect_php_frameworks(content),
        Some("pom.xml") => detect_java_frameworks(content),
        _ => vec![]
    };
    insights.tech_stack.frameworks.extend(frameworks);
}

fn detect_js_frameworks(content: &str) -> Vec<String> {
    let patterns = [("react", "React"), ("vue", "Vue.js"), ("angular", "Angular")];
    patterns.iter()
        .filter(|(p, _)| content.contains(p))
        .map(|(_, f)| f.to_string())
        .collect()
}

fn detect_rust_frameworks(content: &str) -> Vec<String> {
    let patterns = [("rocket", "Rocket"), ("actix", "Actix"), ("tokio", "Tokio")];
    patterns.iter()
        .filter(|(p, _)| content.contains(p))
        .map(|(_, f)| f.to_string())
        .collect()
}

fn detect_python_frameworks(content: &str) -> Vec<String> {
    let patterns = [("django", "Django"), ("flask", "Flask"), ("fastapi", "FastAPI")];
    patterns.iter()
        .filter(|(p, _)| content.contains(p))
        .map(|(_, f)| f.to_string())
        .collect()
}

fn detect_go_frameworks(content: &str) -> Vec<String> {
    let patterns = [("gin-gonic", "Gin"), ("fiber", "Fiber")];
    patterns.iter()
        .filter(|(p, _)| content.contains(p))
        .map(|(_, f)| f.to_string())
        .collect()
}

fn detect_php_frameworks(content: &str) -> Vec<String> {
    let patterns = [("laravel", "Laravel"), ("symfony", "Symfony")];
    patterns.iter()
        .filter(|(p, _)| content.contains(p))
        .map(|(_, f)| f.to_string())
        .collect()
}

fn detect_java_frameworks(content: &str) -> Vec<String> {
    let patterns = [("springframework", "Spring"), ("hibernate", "Hibernate")];
    patterns.iter()
        .filter(|(p, _)| content.contains(p))
        .map(|(_, f)| f.to_string())
        .collect()
}

fn detect_databases(content: &str, insights: &mut ProjectInsights) {
    let patterns = [("mongodb", "MongoDB"), ("postgresql", "PostgreSQL")];
    for (pattern, db) in patterns {
        if content.to_lowercase().contains(pattern) {
            insights.tech_stack.databases.push(db.to_string());
        }
    }
}

fn detect_cloud_services(content: &str, insights: &mut ProjectInsights) {
    let patterns = [("aws", "AWS"), ("azure", "Azure"), ("gcp", "Google Cloud")];
    for (pattern, service) in patterns {
        if content.to_lowercase().contains(pattern) {
            insights.tech_stack.cloud_services.push(service.to_string());
        }
    }
}

fn analyze_code_duplication(content: &str, insights: &mut ProjectInsights) {
    let lines: Vec<&str> = content.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if line.len() > 30 {
            let duplicates = lines[i+1..].iter()
                .filter(|&&other| other == *line)
                .count();
            if duplicates > 0 {
                insights.quality.duplications.push((line.to_string(), duplicates as f32));
            }
        }
    }
}

fn find_security_issues(content: &str, insights: &mut ProjectInsights) {
    let checks = [("eval(", "eval() usage"), ("innerHTML", "XSS risk")];
    for (pattern, issue) in checks {
        if content.contains(pattern) {
            insights.quality.security_issues.push(format!("⚠️ {}", issue));
        }
    }
}


fn detect_architecture_patterns(content: &str, insights: &mut ProjectInsights) {
    let patterns = [("interface", "Interface"), ("abstract", "Abstract Factory")];
    for (pattern, design) in patterns {
        if content.to_lowercase().contains(pattern) {
            insights.patterns.design_patterns.push(design.to_string());
        }
    }

    let anti_patterns = [
        ("goto", "Goto Statement"),
        ("global", "Global State"),
        ("singleton", "Singleton Abuse")
    ];
    
    for (pattern, anti) in anti_patterns {
        if content.to_lowercase().contains(pattern) {
            insights.patterns.anti_patterns.push(anti.to_string());
        }
    }
}

fn detect_code_patterns(insights: &mut ProjectInsights) {
    if insights.metrics.complexity > 0.3 {
        insights.patterns.design_patterns.push("Consider Command Pattern".into());
    }
    if insights.metrics.lines_of_code > 5000 {
        insights.patterns.architecture_style = "Microservices Candidate".into();
    }
}

fn evaluate_quality(insights: &mut ProjectInsights) {
    if insights.metrics.complexity > 0.5 {
        insights.quality.best_practices.push("High complexity - consider refactoring".into());
    }
    if (insights.metrics.test_coverage / insights.metrics.files as f32) < 0.5 {
        insights.quality.best_practices.push("Low test coverage detected".into());
    }
}

fn generate_recommendations(insights: &mut ProjectInsights) {
    if insights.metrics.complexity > 0.3 {
        insights.recommendations.push(Recommendation {
            category: "Performance".into(),
            priority: 1,
            description: "High code complexity detected".into(),
            impact: "Affects runtime performance and maintainability".into(),
            effort: "Medium - Requires systematic refactoring".into(),
        });
    }
    
    if !insights.quality.performance_hotspots.is_empty() {
        insights.recommendations.push(Recommendation {
            category: "Performance".into(),
            priority: 2,
            description: "Performance hotspots found".into(),
            impact: "System performance degradation".into(),
            effort: "High - Requires optimization work".into(),
        });
    }

    if !insights.patterns.anti_patterns.is_empty() {
        insights.recommendations.push(Recommendation {
            category: "Architecture".into(),
            priority: 1,
            description: "Anti-patterns detected".into(),
            impact: "Code maintainability at risk".into(),
            effort: "Medium - Refactoring needed".into(),
        });
    }
}


fn display_insights(insights: &ProjectInsights) {
    println!("\n📊 Project Metrics");
    println!("================");
    println!("Lines of Code: {}", insights.metrics.lines_of_code);
    println!("Files: {}", insights.metrics.files);
    println!("Complexity: {:.2}", insights.metrics.complexity);
    println!("Test Coverage: {:.1}%", (insights.metrics.test_coverage / insights.metrics.files as f32) * 100.0);
    
    if !insights.tech_stack.languages.is_empty() {
        println!("\n🔍 Languages:");
        for (lang, count) in &insights.tech_stack.languages {
            println!("• {} ({} files)", lang, count);
        }
    }
    
    if !insights.quality.security_issues.is_empty() {
        println!("\n🛡️ Security Issues:");
        for issue in &insights.quality.security_issues {
            println!("{}", issue);
        }
    }

    if !insights.quality.performance_hotspots.is_empty() {
        println!("\n⚡ Performance Hotspots:");
        for hotspot in &insights.quality.performance_hotspots {
            println!("• {}", hotspot);
        }
    }

    if !insights.patterns.anti_patterns.is_empty() {
        println!("\n⚠️ Anti-Patterns Detected:");
        for pattern in &insights.patterns.anti_patterns {
            println!("• {}", pattern);
        }
    }

    if !insights.recommendations.is_empty() {
        println!("\n💡 Recommendations:");
        for rec in &insights.recommendations {
            println!("\n{} (Priority: {})", rec.category, rec.priority);
            println!("Description: {}", rec.description);
            println!("Impact: {}", rec.impact);
            println!("Effort: {}", rec.effort);
        }
    }
}
fn generate_detailed_report(insights: &ProjectInsights) {
    let report = format!(
        "DevFlow Analysis Report\n====================\n\
        Project Overview:\n\
        Files: {}\n\
        Lines: {}\n\
        Complexity: {:.2}\n\
        Test Coverage: {:.1}%\n\n\
        Performance Hotspots: {}\n\
        Security Issues: {}\n\
        Anti-Patterns: {}\n\
        Recommendations: {}\n\n\
        Priority Actions:\n\
        {}\n",
        insights.metrics.files,
        insights.metrics.lines_of_code,
        insights.metrics.complexity,
        (insights.metrics.test_coverage / insights.metrics.files as f32) * 100.0,
        insights.quality.performance_hotspots.len(),
        insights.quality.security_issues.len(),
        insights.patterns.anti_patterns.len(),
        insights.recommendations.len(),
        insights.recommendations.iter()
            .map(|r| format!("• {} (Priority: {}): {}", r.category, r.priority, r.description))
            .collect::<Vec<_>>()
            .join("\n")
    );
    let _ = fs::write("devflow-report.txt", report);
}

fn suggest_optimizations(insights: &ProjectInsights) {
    if insights.metrics.complexity > 0.4 {
        println!("\n⚡ Quick Wins");
        println!("===========");
        println!("• Break down complex functions (score: {:.2})", insights.metrics.complexity);
        println!("• Add performance monitoring");
        
        for hotspot in &insights.quality.performance_hotspots {
            println!("• Fix: {}", hotspot);
        }
        
        for pattern in &insights.patterns.anti_patterns {
            println!("• Refactor: {}", pattern);
        }
    }
}
