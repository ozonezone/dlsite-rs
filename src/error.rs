use thiserror::Error;

/// Errors that can occur while using the Dlsite API
#[derive(Debug, Error)]
pub enum DlsiteError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error("{0}")]
    Parse(String),
    #[error("{0}")]
    Server(String),
}

pub(crate) type Result<T> = std::result::Result<T, DlsiteError>;
