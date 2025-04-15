use thiserror::Error;

#[derive(Debug, Error)]
pub enum DlsiteError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("{0}")]
    ParseError(String),
    #[error("{0}")]
    ServerError(String),
}

pub(crate) type Result<T> = std::result::Result<T, DlsiteError>;
