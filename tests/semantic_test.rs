use devflow_pro::analysis::SemanticAnalyzer;
use std::path::PathBuf;

#[test]
fn test_semantic_analysis() {
    let analyzer = SemanticAnalyzer::new();
    
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
    let result = analyzer.analyze_file(&path, test_code).unwrap();
    
    // Verify imports
    assert!(result.imports.iter().any(|i| i.contains("HashMap")));
    
    // Verify struct detection
    assert!(result.types.iter().any(|t| t.name == "TestStruct"));
    
    // Verify function detection
    let functions: Vec<_> = result.functions.iter()
        .map(|f| f.name.as_str())
        .collect();
    assert!(functions.contains(&"new"));
    assert!(functions.contains(&"process"));
    
    // Verify complexity calculation
    let process_fn = result.functions.iter()
        .find(|f| f.name == "process")
        .unwrap();
    assert!(process_fn.complexity > 1, "process function should have complexity > 1 due to if/else branches");
}
