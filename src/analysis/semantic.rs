use crate::ai_enhanced::AIAnalysisResult;
use quote::ToTokens;
use std::collections::HashSet;
use std::path::Path;
use syn::{self, parse_file, Block, Expr, FnArg, ImplItem, Item, ItemUse, UseTree};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SemanticError {
    #[error("Failed to parse file: {0}")]
    ParseError(String),
    #[error("Failed to read file: {0}")]
    FileError(#[from] std::io::Error),
    #[error("Language not supported: {0}")]
    UnsupportedLanguage(String),
    #[error("Invalid path: {0}")]
    InvalidPath(String),
}

pub type Result<T> = std::result::Result<T, SemanticError>;

#[derive(Debug, Clone, Default)]
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
    queries: HashSet<String>,
}

impl std::fmt::Debug for SemanticAnalyzer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SemanticAnalyzer")
            .field("queries", &self.queries)
            .finish()
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticAnalyzer {
    /// Creates a new semantic analyzer
    #[must_use]
    pub fn new() -> Self {
        Self {
            queries: HashSet::new(),
        }
    }

    fn calculate_complexity(block: &Block) -> usize {
        let mut complexity = 1;

        for stmt in &block.stmts {
            if let syn::Stmt::Expr(expr, _) = stmt {
                complexity += Self::expr_complexity(expr);
            }
        }

        complexity
    }

    fn expr_complexity(expr: &Expr) -> usize {
        match expr {
            Expr::If(_) => 1,
            Expr::Match(_) => 1,
            Expr::While(_) => 1,
            Expr::ForLoop(_) => 1,
            Expr::Loop(_) => 1,
            Expr::Block(block_expr) => Self::calculate_complexity(&block_expr.block),
            _ => 0,
        }
    }

    /// Performs basic Rust analysis
    fn analyze_rust(content: &str) -> SemanticContext {
        let mut context = SemanticContext {
            imports: Vec::new(),
            functions: Vec::new(),
            types: Vec::new(),
            dependencies: Vec::new(),
        };

        let syntax = match parse_file(content) {
            Ok(syntax) => syntax,
            Err(_) => return context,
        };

        for item in syntax.items {
            match item {
                Item::Use(item_use) => {
                    Self::extract_imports(&item_use, &mut context.imports);
                }
                Item::Fn(item_fn) => {
                    context.functions.push(FunctionInfo {
                        name: item_fn.sig.ident.to_string(),
                        parameters: item_fn
                            .sig
                            .inputs
                            .iter()
                            .map(|arg| match arg {
                                FnArg::Typed(pat_type) => pat_type.ty.to_token_stream().to_string(),
                                FnArg::Receiver(_) => "self".to_string(),
                            })
                            .collect(),
                        return_type: None,
                        complexity: Self::calculate_complexity(&item_fn.block),
                        dependencies: Vec::new(),
                    });
                }
                Item::Impl(item_impl) => {
                    for impl_item in item_impl.items {
                        if let ImplItem::Fn(impl_fn) = impl_item {
                            context.functions.push(FunctionInfo {
                                name: impl_fn.sig.ident.to_string(),
                                parameters: impl_fn
                                    .sig
                                    .inputs
                                    .iter()
                                    .map(|arg| match arg {
                                        FnArg::Typed(pat_type) => {
                                            pat_type.ty.to_token_stream().to_string()
                                        }
                                        FnArg::Receiver(_) => "self".to_string(),
                                    })
                                    .collect(),
                                return_type: None,
                                complexity: Self::calculate_complexity(&impl_fn.block),
                                dependencies: Vec::new(),
                            });
                        }
                    }
                }
                Item::Struct(item_struct) => {
                    context.types.push(TypeInfo {
                        name: item_struct.ident.to_string(),
                        kind: TypeKind::Struct,
                        fields: item_struct
                            .fields
                            .iter()
                            .filter_map(|f| f.ident.as_ref().map(|i| i.to_string()))
                            .collect(),
                    });
                }
                _ => {}
            }
        }

        context
    }

    fn extract_imports(item_use: &ItemUse, imports: &mut Vec<String>) {
        fn extract_tree(tree: &UseTree, imports: &mut Vec<String>) {
            match tree {
                UseTree::Path(path) => {
                    extract_tree(&path.tree, imports);
                }
                UseTree::Name(name) => {
                    imports.push(name.ident.to_string());
                }
                UseTree::Glob(_) => {
                    // Handle glob imports
                }
                UseTree::Group(group) => {
                    for tree in &group.items {
                        extract_tree(tree, imports);
                    }
                }
                UseTree::Rename(_) => {
                    // Handle renamed imports
                }
            }
        }

        extract_tree(&item_use.tree, imports);
    }

    /// Analyzes a file for semantic context
    ///
    /// # Errors
    /// Returns an error if file analysis fails
    pub fn analyze_file(&self, path: &Path, content: &str) -> Result<SemanticContext> {
        let extension = path.extension().and_then(|e| e.to_str()).ok_or_else(|| {
            SemanticError::InvalidPath("Failed to get file extension".to_string())
        })?;

        match extension {
            "rs" => Ok(Self::analyze_rust(content)),
            _ => Err(SemanticError::UnsupportedLanguage(extension.to_string())),
        }
    }

    /// Merges semantic context with AI analysis results
    #[must_use]
    pub fn merge_with_ai_analysis(
        &self,
        semantic: &SemanticContext,
        ai: &AIAnalysisResult,
    ) -> SemanticContext {
        let mut merged = semantic.clone();

        // Enhance function complexity with AI insights
        for function in &mut merged.functions {
            if let Some(_suggestion) = ai
                .optimization_suggestions
                .iter()
                .find(|s| s.description.contains(&function.name))
            {
                function.complexity += 1;
            }
        }

        // Add any additional dependencies identified by AI
        merged
            .dependencies
            .extend(ai.security_recommendations.iter().filter_map(|r| {
                if r.description.contains("dependency") {
                    Some(r.description.clone())
                } else {
                    None
                }
            }));

        merged
    }
}
