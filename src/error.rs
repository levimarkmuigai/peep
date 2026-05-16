#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("serde_json: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("not found: {0}")]
    ParseError(String),
}
