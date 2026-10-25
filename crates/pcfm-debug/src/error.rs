//! Errors for the `pcfm-debug` CLI.

use std::error::Error;
use std::fmt;

/// Errors that can occur in the CLI.
#[derive(Debug)]
pub enum CliError {
    /// The demo team is malformed. Unreachable in practice.
    DemoTeam(String),
    /// The JSON serialisation failed.
    Json(serde_json::Error),
    /// The HTML file could not be written.
    Io(std::io::Error),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DemoTeam(s) => write!(f, "demo team error: {s}"),
            Self::Json(e) => write!(f, "json error: {e}"),
            Self::Io(e) => write!(f, "io error: {e}"),
        }
    }
}

impl Error for CliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::DemoTeam(_) => None,
            Self::Json(e) => Some(e),
            Self::Io(e) => Some(e),
        }
    }
}

impl From<serde_json::Error> for CliError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
