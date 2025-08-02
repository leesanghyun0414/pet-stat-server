use std::sync::LazyLock;

use envy::prefixed;
use serde::Deserialize;
use tracing::{info, instrument};

use crate::base_config::Config;

#[derive(Debug, Deserialize)]
pub struct PetStatCentralDbEnv {
    password: String,
    name: String,
    port: u16,
    host: String,
    user: String,
}

#[derive(Debug)]
pub struct PetStatCentralDbConfig {
    pub url: String,
}

impl PetStatCentralDbConfig {
    pub fn from_env() -> Result<Self, crate::error::ConfigError> {
        let env: PetStatCentralDbEnv = prefixed("PET_STAT_CENTRAL_DB_").from_env()?;
        let url = format!(
            "postgresql://{}:{}@{}:{}/{}",
            env.user, env.password, env.host, env.port, env.name,
        );
        Ok(Self { url })
    }
}
pub static PET_STAT_CENTRAL_DB_CONFIG: LazyLock<PetStatCentralDbConfig> = LazyLock::new(|| {
    info!("Loading Database configurations...");

    PetStatCentralDbConfig::new().unwrap()
});
impl Config for PetStatCentralDbConfig {
    #[instrument]
    fn new() -> Result<Self, crate::error::ConfigError> {
        Self::from_env()
    }
}
