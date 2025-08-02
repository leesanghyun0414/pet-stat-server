use serde::Deserialize;

use crate::{error::ConfigError, utils::load_config};

#[derive(Debug, Deserialize, Clone)]
pub struct SecretConfig {
    pub jwt_secret: String,
}

impl SecretConfig {
    pub fn new() -> Result<Self, ConfigError> {
        load_config::<SecretConfig>()
    }
}
