use std::sync::LazyLock;

use crate::{base_config::Config, error::ConfigError, utils::load_config};
use serde::Deserialize;
use tracing::{info, instrument, Level};

#[derive(Deserialize, Debug)]
pub struct LoggingConfig {
    #[serde(deserialize_with = "deserialize_level")]
    pub log_level: Level,
}

fn deserialize_level<'de, D>(deserializer: D) -> Result<Level, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let level_str: String = String::deserialize(deserializer)?;
    match level_str.parse::<Level>() {
        Ok(level) => Ok(level),
        Err(_) => Err(serde::de::Error::custom(format!(
            "Invalid log level: {}",
            level_str
        ))),
    }
}

pub static LOGGING_CONFIG: LazyLock<LoggingConfig> = LazyLock::new(|| {
    info!("Loading Logging configurations...");

    LoggingConfig::new().unwrap()
});
impl Config for LoggingConfig {
    #[instrument]
    fn new() -> Result<Self, ConfigError> {
        load_config::<LoggingConfig>()
    }
}
