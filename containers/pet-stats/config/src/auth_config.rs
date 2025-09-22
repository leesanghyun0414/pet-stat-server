use serde::Deserialize;

use crate::{base_config::Config, error::ConfigError, utils::load_config};

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    pub google_oauth_public_key_url: String,
    pub google_oauth_client_id: String,
    pub jwt_sign_secret: String,
    pub refresh_key_hashing_secret: String,
}

impl Config for AuthConfig {
    fn new() -> Result<Self, ConfigError> {
        load_config::<AuthConfig>()
    }
}
