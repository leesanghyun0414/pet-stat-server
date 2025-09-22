// service/queries/auth.rs

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn validate_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    info!("Validate JWT started.");
    let validation = Validation::new(Algorithm::HS256);
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    ) {
        Ok(token_data) => Ok(token_data.claims),

        Err(err) => {
            error!("{:?}", err);
            Err(err)
        }
    }
}

pub fn generate_jwt(
    sub: &str,
    secret: &str,
    exp: usize,
) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        sub: sub.to_owned(),
        exp,
    };
    let header = Header::new(Algorithm::HS256);
    encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()))
}
