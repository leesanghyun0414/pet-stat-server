use std::time::Duration;

use async_trait::async_trait;
use config::{auth_config::AuthConfig, base_config::Config};
use jsonwebtoken::{
    decode, decode_header,
    jwk::{Jwk, JwkSet},
    DecodingKey, Validation,
};
use rest::client::{HttpClient, HttpClientBuilder};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};

use super::{error::AuthError, oauth_provider::OAuthProvider};

static GOOGLE_ISSUERS: &[&str] = &["https://accounts.google.com", "accounts.google.com"];

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleClaims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: i64,
    pub iat: i64,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub name: Option<String>,
}

pub struct GoogleOAuth {
    client_id: String,
    http_client: HttpClient,
}

impl GoogleOAuth {
    pub fn new(client_id: String) -> Result<Self, AuthError> {
        let http_client = HttpClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|_| AuthError::InitilizingError)?;
        Ok(Self {
            client_id,
            http_client,
        })
    }
}

#[async_trait]
impl OAuthProvider for GoogleOAuth {
    type Claims = GoogleClaims;

    async fn verify_token(&self, token: &str) -> Result<Self::Claims, super::error::AuthError> {
        let header = decode_header(token).map_err(|_| AuthError::InvalidToken)?;
        let kid = header.kid.ok_or(AuthError::InvalidToken)?;

        let jwk = self.fetch_public_key(&kid).await?;
        let decoding_key =
            DecodingKey::from_jwk(&jwk).map_err(|e| AuthError::NetworkError(e.to_string()))?;
        info!("Decoding Key is good");

        let mut validation = Validation::new(header.alg);
        validation.set_audience(&[&self.client_id]);
        validation.set_issuer(GOOGLE_ISSUERS);

        let token_data = decode::<GoogleClaims>(token, &decoding_key, &validation)
            .inspect(|x| info!("Inspect !{:?}", x.header.alg))
            .map_err(|e| {
                error!("Token Error! {:?}", e.to_string());
                AuthError::NetworkError(e.to_string())
            })?;
        Ok(token_data.claims)
    }

    async fn fetch_public_key(&self, kid: &str) -> Result<Jwk, super::error::AuthError> {
        let auth_config = AuthConfig::new()?;

        let res = self
            .http_client
            .get(auth_config.google_oauth_public_key_url)
            .await
            .map_err(|e| AuthError::NetworkError(e.to_string()))?;

        let jwk_set: JwkSet = res.json().await.map_err(|_| AuthError::InvalidToken)?;
        let jwk: &Jwk = jwk_set.find(kid).ok_or(AuthError::InvalidToken)?;

        debug!("jwk algorithm: {:?}", jwk.algorithm);

        Ok(jwk.to_owned())
    }
}
