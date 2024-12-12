use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::Path;
use tempfile::TempDir;
use std::fs::{self, File};
use std::io::Write;

fn create_test_project() -> TempDir {
    let dir = TempDir::new().unwrap();
    
    // Create some test files
    let files = vec![
        ("main.rs", "fn main() {\n    println!(\"Hello\");\n}"),
        ("lib.rs", "pub fn add(a: i32, b: i32) -> i32 {\n    a + b\n}"),
        ("test.rs", "#[test]\nfn test_add() {\n    assert_eq!(add(2, 2), 4);\n}"),
    ];
    
    for (name, content) in files {
        let path = dir.path().join(name);
        let mut file = File::create(path).unwrap();
        writeln!(file, "{}", content).unwrap();
    }
    
    dir
}

fn benchmark_small_project(c: &mut Criterion) {
    let test_dir = create_test_project();
    
    c.bench_function("analyze_small_project", |b| {
        b.iter(|| {
            let config = devflow_pro::AppConfig::default();
            devflow_pro::analyze_codebase(black_box(test_dir.path()), &config)
        });
    });
}

fn benchmark_file_analysis(c: &mut Criterion) {
    let test_dir = create_test_project();
    let test_file = test_dir.path().join("main.rs");
    
    c.bench_function("analyze_single_file", |b| {
        b.iter(|| {
            let mut insights = devflow_pro::ProjectInsights::default();
            let config = devflow_pro::AppConfig::default();
            devflow_pro::analyze_file(black_box(&test_file), &mut insights, &config)
        });
    });
}

criterion_group!(benches, benchmark_small_project, benchmark_file_analysis);
criterion_main!(benches);
