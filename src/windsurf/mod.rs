pub mod config;
pub mod ide_main;
pub mod interface;
pub mod mock;

pub use config::Config;
pub use ide_main::Plugin;
pub use interface::{Integration, Interface, Position, Range};
pub use mock::IntegrationImpl;

use std::path::PathBuf;

/// Represents a file in the workspace.
#[derive(Debug, Clone)]
pub struct File {
    pub path: PathBuf,
    pub content: String,
}

/// Represents a diagnostic message.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub message: String,
    pub severity: DiagnosticSeverity,
}

/// Represents the severity of a diagnostic message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}
