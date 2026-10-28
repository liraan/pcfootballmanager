//! Error type for the demo CLI.

/// Errors that can occur while building the demo or rendering the result.
#[derive(Debug, thiserror::Error)]
pub enum DemoError {
    /// The demo teams are malformed (unreachable by construction).
    #[error("demo team is malformed: {0}")]
    DemoTeam(String),

    /// An I/O error occurred while writing the output file.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
