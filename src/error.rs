#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("serde_json: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("not found: {0}")]
    ParseError(String),

    #[error("command error: {0}")]
    Command(#[from] std::io::Error),

    #[error("stdout error")]
    StdoutUnavailable,
}
