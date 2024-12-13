use crate::ai_enhanced::AIAnalysisResult;
use dashmap::DashMap;
use std::path::PathBuf;
use syn::{self, parse_file, Block, Expr, ImplItem, Item, UseTree};
use thiserror::Error;

/// Error types for semantic analysis operations
#[derive(Debug, Error)]
pub enum AnalysisError {
    #[error("Failed to read file: {0}")]
    FileRead(#[from] std::io::Error),
    #[error("Failed to parse Rust code: {0}")]
    Parse(#[from] syn::Error),
}

pub type Result<T> = std::result::Result<T, AnalysisError>;

/// Context containing semantic analysis results
#[derive(Debug, Default, Clone)]
pub struct Context {
    pub complexity: usize,
    pub functions: Vec<String>,
    pub imports: Vec<String>,
}

/// Analyzer for Rust source code
#[derive(Debug)]
pub struct Analyzer {
    cache: DashMap<PathBuf, Context>,
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl Analyzer {
    /// Creates a new analyzer instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            cache: DashMap::new(),
        }
    }

    /// Analyzes a Rust source file and returns semantic context
    ///
    /// # Errors
    /// Returns an error if:
    /// - The file cannot be read
    /// - The file contains invalid Rust syntax
    /// - The analyzer fails to parse the syntax tree
    pub fn analyze_file(&mut self, path: &PathBuf) -> Result<Context> {
        if let Some(ctx) = self.cache.get(path) {
            return Ok(ctx.clone());
        }

        let source = std::fs::read_to_string(path)?;
        let syntax_tree = parse_file(&source)?;

        let mut analysis = Context::default();

        for item in syntax_tree.items {
            match item {
                Item::Use(item_use) => {
                    Self::process_use_tree(&item_use.tree, &mut analysis.imports);
                }
                Item::Fn(item_fn) => {
                    analysis.functions.push(item_fn.sig.ident.to_string());
                    analysis.complexity += Self::calculate_block_complexity(&item_fn.block);
                }
                Item::Impl(item_impl) => {
                    for impl_item in item_impl.items {
                        if let ImplItem::Fn(impl_fn) = impl_item {
                            analysis.functions.push(impl_fn.sig.ident.to_string());
                            analysis.complexity += Self::calculate_block_complexity(&impl_fn.block);
                        }
                    }
                }
                _ => {}
            }
        }

        self.cache.insert(path.clone(), analysis.clone());
        Ok(analysis)
    }

    fn calculate_block_complexity(block: &Block) -> usize {
        let mut complexity = 0;
        for stmt in &block.stmts {
            if let syn::Stmt::Expr(expr, _) = stmt {
                complexity += Self::calculate_complexity(expr);
            }
        }
        complexity
    }

    fn calculate_complexity(expr: &Expr) -> usize {
        match expr {
            Expr::If(expr_if) => {
                let mut complexity = 1;
                complexity += Self::calculate_complexity(&expr_if.cond);
                complexity += Self::calculate_block_complexity(&expr_if.then_branch);
                if let Some((_, else_branch)) = &expr_if.else_branch {
                    complexity += Self::calculate_complexity(else_branch);
                }
                complexity
            }
            Expr::While(expr_while) => {
                1 + Self::calculate_complexity(&expr_while.cond)
                    + Self::calculate_block_complexity(&expr_while.body)
            }
            Expr::ForLoop(expr_for) => 1 + Self::calculate_block_complexity(&expr_for.body),
            Expr::Match(expr_match) => {
                let mut complexity = expr_match.arms.len();
                for arm in &expr_match.arms {
                    complexity += Self::calculate_complexity(&arm.body);
                }
                complexity
            }
            Expr::Block(expr_block) => Self::calculate_block_complexity(&expr_block.block),
            Expr::Return(expr_return) => expr_return
                .expr
                .as_ref()
                .map_or(0, |expr| Self::calculate_complexity(expr)),
            _ => 0,
        }
    }

    fn process_use_tree(tree: &UseTree, imports: &mut Vec<String>) {
        match tree {
            UseTree::Path(use_path) => {
                Self::process_use_tree(&use_path.tree, imports);
            }
            UseTree::Name(use_name) => {
                imports.push(use_name.ident.to_string());
            }
            UseTree::Rename(_) => {
                // Skip renamed imports
            }
            UseTree::Glob(_) => {
                imports.push("*".to_string());
            }
            UseTree::Group(use_group) => {
                for tree in &use_group.items {
                    Self::process_use_tree(tree, imports);
                }
            }
        }
    }

    /// Merges semantic analysis with AI-enhanced analysis
    #[must_use]
    pub const fn merge_with_ai_analysis(
        &self,
        context: Context,
        _ai: &AIAnalysisResult,
    ) -> Context {
        context
    }
}
