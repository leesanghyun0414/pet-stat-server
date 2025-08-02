use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpClientError {
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("JSON serialization failed: {0}")]
    JsonError(String),

    #[error("HTTP error {status}: {message}")]
    HttpError { status: StatusCode, message: String },

    #[error("Timeout occurred")]
    Timeout,

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Initilizing error")]
    InitilizingError,
}
