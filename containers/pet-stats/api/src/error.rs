use std::io;

use config::error::ConfigError;
use sea_orm::DbErr;
use service::auth::error::AuthError;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Failed to create schema: {0}")]
    Schema(#[from] DbErr),

    #[error("Signal handling failed: {0}")]
    Signal(String),

    #[error("Configuration Error: {0}")]
    Config(#[from] ConfigError),
    #[error("HTTP server encountered an error: {0}")]
    ServerError(#[from] io::Error),

    #[error("Auth Error")]
    Auth(#[from] AuthError),
}
