use crate::ai_enhanced::AIAnalysisResult;
use std::collections::HashMap;
use std::path::PathBuf;
use syn::{self, parse_file, Block, Expr, ImplItem, Item, UseTree};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SemanticError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    ParseError(#[from] syn::Error),
    #[error("Invalid path: {0}")]
    InvalidPath(String),
}

pub type Result<T> = std::result::Result<T, SemanticError>;

#[derive(Debug, Default, Clone)]
pub struct SemanticContext {
    pub imports: Vec<String>,
    pub functions: Vec<String>,
    pub complexity: usize,
}

#[derive(Debug, Default)]
pub struct SemanticAnalyzer {
    cache: HashMap<PathBuf, SemanticContext>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn analyze_file(&mut self, path: &PathBuf) -> Result<SemanticContext> {
        if let Some(context) = self.cache.get(path) {
            return Ok(context.clone());
        }

        let content = std::fs::read_to_string(path)?;
        let syntax_tree = parse_file(&content)?;

        let mut context = SemanticContext::default();

        for item in syntax_tree.items {
            match item {
                Item::Use(item_use) => {
                    Self::process_use_tree(&item_use.tree, &mut context.imports);
                }
                Item::Fn(item_fn) => {
                    context.functions.push(item_fn.sig.ident.to_string());
                    context.complexity += Self::calculate_block_complexity(&item_fn.block);
                }
                Item::Impl(item_impl) => {
                    for impl_item in item_impl.items {
                        if let ImplItem::Fn(impl_fn) = impl_item {
                            context.functions.push(impl_fn.sig.ident.to_string());
                            context.complexity += Self::calculate_block_complexity(&impl_fn.block);
                        }
                    }
                }
                _ => {}
            }
        }

        self.cache.insert(path.clone(), context.clone());
        Ok(context)
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
                let mut complexity = 1; // Base complexity for if
                complexity += Self::calculate_complexity(&expr_if.cond);
                complexity += Self::calculate_block_complexity(&expr_if.then_branch);
                if let Some((_, else_branch)) = &expr_if.else_branch {
                    complexity += Self::calculate_complexity(else_branch);
                }
                complexity
            }
            Expr::While(expr_while) => {
                1 + Self::calculate_complexity(&expr_while.cond) + 
                   Self::calculate_block_complexity(&expr_while.body)
            }
            Expr::ForLoop(expr_for) => {
                1 + Self::calculate_block_complexity(&expr_for.body)
            }
            Expr::Match(expr_match) => {
                let mut complexity = expr_match.arms.len();
                for arm in &expr_match.arms {
                    complexity += Self::calculate_complexity(&arm.body);
                }
                complexity
            }
            Expr::Block(expr_block) => Self::calculate_block_complexity(&expr_block.block),
            Expr::Return(expr_return) => {
                if let Some(expr) = &expr_return.expr {
                    Self::calculate_complexity(expr)
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    fn process_use_tree(tree: &UseTree, imports: &mut Vec<String>) {
        match tree {
            UseTree::Path(use_path) => {
                imports.push(use_path.ident.to_string());
                SemanticAnalyzer::process_use_tree(&use_path.tree, imports);
            }
            UseTree::Name(use_name) => {
                imports.push(use_name.ident.to_string());
            }
            UseTree::Rename(use_rename) => {
                imports.push(format!("{} as {}", &use_rename.ident, &use_rename.rename));
            }
            UseTree::Glob(_) => {
                imports.push("*".to_string());
            }
            UseTree::Group(use_group) => {
                for tree in &use_group.items {
                    SemanticAnalyzer::process_use_tree(tree, imports);
                }
            }
        }
    }

    #[must_use]
    pub fn merge_with_ai_analysis(
        &self,
        context: SemanticContext,
        _ai: &AIAnalysisResult,
    ) -> SemanticContext {
        // TODO: Enhance with AI insights
        context
    }
}
