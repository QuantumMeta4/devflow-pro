use crate::ai_enhanced::AIAnalysisResult;
use tree_sitter::{Parser, Query, QueryCursor, Tree};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SemanticError {
    #[error("Failed to parse file: {0}")]
    ParseError(String),
    #[error("Failed to read file: {0}")]
    FileError(#[from] std::io::Error),
    #[error("Language not supported: {0}")]
    UnsupportedLanguage(String),
}

pub type Result<T> = std::result::Result<T, SemanticError>;

#[derive(Debug, Clone)]
pub struct SemanticContext {
    pub imports: Vec<String>,
    pub functions: Vec<FunctionInfo>,
    pub types: Vec<TypeInfo>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub parameters: Vec<String>,
    pub return_type: Option<String>,
    pub complexity: usize,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub name: String,
    pub kind: TypeKind,
    pub fields: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Struct,
    Enum,
    Trait,
    Interface,
}

pub struct SemanticAnalyzer {
    parsers: Mutex<HashMap<String, Box<Parser>>>,
    queries: HashMap<String, Query>,
}

impl std::fmt::Debug for SemanticAnalyzer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SemanticAnalyzer")
            .field("queries", &self.queries)
            .field("parsers", &"<Parser>")
            .finish()
    }
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            parsers: Mutex::new(HashMap::new()),
            queries: HashMap::new(),
        };
        analyzer.initialize_parsers();
        analyzer
    }

    fn initialize_parsers(&mut self) {
        let mut parser = Parser::new();

        if let Ok(()) = parser.set_language(tree_sitter_rust::language()) {
            if let Ok(query) = Query::new(
                tree_sitter_rust::language(),
                r#"
                (function_item
                    name: (identifier) @function.name
                    parameters: (parameters) @function.params
                    return_type: (_)? @function.return
                ) @function.def

                (struct_item
                    name: (type_identifier) @type.name
                    body: (field_declaration_list)? @type.fields
                ) @type.def

                (use_declaration
                    path: (scoped_identifier) @import.path
                ) @import
                "#,
            ) {
                if let Ok(mut parsers) = self.parsers.lock() {
                    parsers.insert("rust".to_string(), Box::new(parser));
                    self.queries.insert("rust".to_string(), query);
                }
            }
        }
    }

    pub fn analyze_file(&self, path: &PathBuf, content: &str) -> Result<SemanticContext> {
        let ext = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let lang = match ext.as_str() {
            "rs" => "rust",
            _ => return Err(SemanticError::UnsupportedLanguage(ext)),
        };

        let tree = {
            let mut parsers = self.parsers.lock()
                .map_err(|_| SemanticError::ParseError("Failed to acquire parser lock".to_string()))?;
            
            let parser = parsers.get_mut(lang)
                .ok_or_else(|| SemanticError::UnsupportedLanguage(lang.to_string()))?;

            parser.parse(content, None)
                .ok_or_else(|| SemanticError::ParseError("Failed to parse file".to_string()))?
        };

        self.extract_semantic_info(&tree, content, lang)
    }

    fn extract_semantic_info(&self, tree: &Tree, content: &str, lang: &str) -> Result<SemanticContext> {
        let mut context = SemanticContext {
            imports: Vec::new(),
            functions: Vec::new(),
            types: Vec::new(),
            dependencies: Vec::new(),
        };

        if let Some(query) = self.queries.get(lang) {
            let mut cursor = QueryCursor::new();
            let matches = cursor.matches(query, tree.root_node(), content.as_bytes());

            for match_ in matches {
                for capture in match_.captures {
                    let capture_name = query.capture_names()[capture.index as usize].as_str();
                    let node_text = capture.node.utf8_text(content.as_bytes()).unwrap_or("");

                    match capture_name {
                        "function.name" => {
                            context.functions.push(FunctionInfo {
                                name: node_text.to_string(),
                                parameters: Vec::new(),
                                return_type: None,
                                complexity: self.calculate_complexity(&capture.node),
                                dependencies: Vec::new(),
                            });
                        }
                        "type.name" => {
                            context.types.push(TypeInfo {
                                name: node_text.to_string(),
                                kind: TypeKind::Struct,
                                fields: Vec::new(),
                            });
                        }
                        "import.path" => {
                            context.imports.push(node_text.to_string());
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(context)
    }

    fn calculate_complexity(&self, node: &tree_sitter::Node) -> usize {
        let mut complexity = 1;
        let mut cursor = node.walk();
        
        for child in node.children(&mut cursor) {
            match child.kind() {
                "if_expression" | "match_expression" | "for_expression" | 
                "while_expression" | "loop_expression" => {
                    complexity += 1;
                }
                _ => {}
            }
        }
        
        complexity
    }

    pub fn merge_with_ai_analysis(&self, semantic: &SemanticContext, ai: &AIAnalysisResult) -> SemanticContext {
        let mut merged = semantic.clone();
        
        // Enhance function complexity with AI insights
        for function in &mut merged.functions {
            if let Some(_suggestion) = ai.optimization_suggestions.iter()
                .find(|s| s.description.contains(&function.name)) {
                function.complexity += 1;
            }
        }

        // Add any additional dependencies identified by AI
        merged.dependencies.extend(
            ai.security_recommendations.iter()
                .filter_map(|r| {
                    if r.description.contains("dependency") {
                        Some(r.description.clone())
                    } else {
                        None
                    }
                })
        );

        merged
    }
}
