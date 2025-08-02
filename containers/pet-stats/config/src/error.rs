use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing or invalid environment variables: {0}")]
    Envy(#[from] envy::Error),
}
