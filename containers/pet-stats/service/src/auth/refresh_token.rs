use rand::TryRngCore;

use super::error::AuthError;

#[derive(Debug, Clone)]
pub struct RefreshToken(pub String);

impl RefreshToken {
    pub fn generate() -> Result<Self, AuthError> {
        use base64::{engine::general_purpose, Engine};
        use rand::rngs::OsRng;

        let mut buf = [0u8; 32];
        OsRng.try_fill_bytes(&mut buf)?;

        Ok(Self(general_purpose::URL_SAFE_NO_PAD.encode(buf)))
    }

    pub fn hash(&self, secret: &[u8]) -> [u8; 32] {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        type HmacSha256 = Hmac<Sha256>;

        let mut mac = HmacSha256::new_from_slice(secret).unwrap();
        mac.update(self.0.as_bytes());
        mac.finalize().into_bytes().into()
    }
}
