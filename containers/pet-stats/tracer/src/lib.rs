use config::{
    app_config::{Flavor, APP_CONFIG},
    error::ConfigError,
    logging_config::LOGGING_CONFIG,
};
use thiserror::Error;
use tracing::{info, subscriber::SetGlobalDefaultError};
use tracing_subscriber::fmt;

#[derive(Debug, Error)]
pub enum TracingError {
    #[error("Missing or invalid environment variables: {0}")]
    Config(#[from] ConfigError),

    #[error("Failed to Initialize setting global default logger: {0}")]
    Init(#[from] SetGlobalDefaultError),
}

/// Initializing tracing configurations.
pub fn init_tracing() -> Result<(), TracingError> {
    let _subscriber_guard = tracing::subscriber::set_default(fmt().pretty().finish());
    let app_config = &*APP_CONFIG;
    let logging_config = &*LOGGING_CONFIG;
    match app_config.flavor {
        Flavor::Dev => tracing::subscriber::set_global_default(
            fmt()
                .with_max_level(logging_config.log_level)
                .with_target(true)
                .pretty()
                .finish(),
        )?,
        Flavor::Stg | Flavor::Prod => tracing::subscriber::set_global_default(
            fmt()
                .with_max_level(logging_config.log_level)
                .with_target(true)
                .finish(),
        )?,
    }
    // Use a more compact, abbreviated format.

    info!("Init logging level - {:?}", logging_config.log_level);
    Ok(())
}
