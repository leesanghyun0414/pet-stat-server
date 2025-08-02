use jsonwebtoken::jwk::Jwk;

use super::error::AuthError;
use async_trait::async_trait;

#[async_trait]
pub trait OAuthProvider {
    type Claims;
    async fn verify_token(&self, token: &str) -> Result<Self::Claims, AuthError>;
    async fn fetch_public_key(&self, kid: &str) -> Result<Jwk, AuthError>;
}
