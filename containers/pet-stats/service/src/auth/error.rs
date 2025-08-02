use rand::rand_core;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidToken,
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Token expired")]
    TokenExpired,
    #[error("Initilizing error")]
    InitilizingError,

    #[error("Missiong Config")]
    Config(#[from] config::error::ConfigError),
    #[error("Cache erorr")]
    Cache(#[from] CacheError),

    #[error("OS-CSPRNG failure: {0}")]
    Rng(#[from] rand_core::OsError),
}

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Failed to retrieve key from cache")]
    RetrievalFailed,
    #[error("Failed to store key in cache")]
    StorageFailed,
    #[error("Cache entry has expired")]
    Expired,
    #[error("Key conversion failed: {0}")]
    KeyConversion(String),
    #[error("Cache connection failed: {0}")]
    ConnectionFailed(String),
}
