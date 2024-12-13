use devflow_pro::analysis::SemanticAnalyzer;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_semantic_analysis() {
    let mut analyzer = SemanticAnalyzer::new();

    let test_code = r#"
        use std::collections::HashMap;
        
        pub struct TestStruct {
            field1: String,
            field2: i32,
        }
        
        impl TestStruct {
            pub fn new() -> Self {
                Self {
                    field1: String::new(),
                    field2: 0,
                }
            }
            
            pub fn process(&self, value: i32) -> String {
                if value > 0 {
                    "positive".to_string()
                } else if value < 0 {
                    "negative".to_string()
                } else {
                    "zero".to_string()
                }
            }
        }
    "#;

    let path = PathBuf::from("test.rs");
    fs::write(&path, test_code).unwrap();
    let result = analyzer.analyze_file(&path).unwrap();
    fs::remove_file(&path).unwrap();

    // Verify imports
    assert!(result.imports.iter().any(|i| i.contains("HashMap")));

    // Verify function detection
    assert!(result.functions.contains(&"new".to_string()));
    assert!(result.functions.contains(&"process".to_string()));

    // Debug output
    println!("Complexity: {}", result.complexity);
    println!("Functions: {:?}", result.functions);
    println!("Imports: {:?}", result.imports);

    // Verify complexity calculation
    assert!(
        result.complexity > 1,
        "Code should have complexity > 1 due to if/else branches"
    );
}
