use serde::Deserialize;

use crate::{base_config::Config, error::ConfigError, utils::load_config};

// JWT_SIGN_SECRET=dcc31703b0fc1b08b130cf716f4450e35c0d2b45776910b26131f289b906c912
// REFRESH_KEY_HASHING_SECRET=0e6985caff19ed72e0cd98c69a94411a17bff1f8f1dc2d1c82923c2f6f7d90e8
#[derive(Debug, Deserialize)]
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
