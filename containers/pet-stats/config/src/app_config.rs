use std::sync::LazyLock;

use serde::Deserialize;
use tracing::{info, instrument};

use crate::{base_config::Config, error::ConfigError, utils::load_config};
#[derive(Debug, Deserialize)]
pub enum Flavor {
    #[serde(alias = "dev", alias = "DEV")]
    Dev,
    #[serde(alias = "stg", alias = "STG")]
    Stg,
    #[serde(alias = "prod", alias = "PROD")]
    Prod,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub flavor: Flavor,
    pub skip_middleware_operations: Vec<String>,
}

pub static APP_CONFIG: LazyLock<AppConfig> = LazyLock::new(|| {
    info!("Loading App configurations...");

    AppConfig::new().unwrap()
});

impl Config for AppConfig {
    #[instrument]
    fn new() -> Result<Self, ConfigError> {
        load_config::<AppConfig>()
    }
}
