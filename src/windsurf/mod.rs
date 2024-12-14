pub mod ide_main;
pub mod interface;
pub mod mock;

use std::path::PathBuf;

/// Position in a text document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

/// Range in a text document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

/// Configuration for Windsurf functionality.
#[derive(Debug, Clone, Default)]
pub struct WindsurfConfig {
    pub real_time_enabled: bool,
}

/// Plugin for Windsurf functionality.
#[derive(Debug, Clone)]
pub struct WindsurfPlugin {
    pub name: String,
    pub version: String,
}

impl Default for WindsurfPlugin {
    fn default() -> Self {
        Self {
            name: "Windsurf".to_string(),
            version: "0.1.0".to_string(),
        }
    }
}

/// Represents a file in the workspace.
#[derive(Debug, Clone)]
pub struct WorkspaceFile {
    pub path: PathBuf,
    pub content: String,
}

/// Represents a change in a text document.
#[derive(Debug, Clone)]
pub struct TextChange {
    pub range: Range,
    pub text: String,
}

/// Represents a command in the IDE.
#[derive(Debug, Clone)]
pub struct CommandInfo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
}

/// Represents a diagnostic message.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub range: Range,
    pub message: String,
    pub severity: DiagnosticSeverity,
}

/// Severity levels for diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

pub use interface::{IDEInterface, WindsurfIntegration};
pub use mock::MockWindsurfIntegration;
