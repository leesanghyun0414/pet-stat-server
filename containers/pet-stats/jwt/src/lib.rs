use chrono::{DateTime, TimeDelta, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{error, info, instrument};

pub const DEFAULT_EXP: TimeDelta = TimeDelta::seconds(1);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnixTimestamp(pub i64);

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub email: Option<String>,
    pub iat: UnixTimestamp,
    pub exp: UnixTimestamp,
}

impl From<DateTime<Utc>> for UnixTimestamp {
    fn from(dt: DateTime<Utc>) -> Self {
        UnixTimestamp(dt.timestamp())
    }
}

impl TryFrom<UnixTimestamp> for DateTime<Utc> {
    type Error = JwtAuthError;
    fn try_from(ts: UnixTimestamp) -> Result<Self, Self::Error> {
        DateTime::from_timestamp(ts.0, 0)
            .inspect(|t| info!("{:?}", t))
            .ok_or(JwtAuthError::TimestampConversionError(ts.0))
    }
}

#[derive(Error, Debug)]
pub enum JwtAuthError {
    #[error("JWT failed: {0}")]
    JWTError(#[from] jsonwebtoken::errors::Error),

    #[error("JWT expired")]
    Expired,
    #[error("Invalid JWT")]
    Invalid,

    #[error("Timestamp conversion failed for value: {0}")]
    TimestampConversionError(i64),
}

#[instrument(skip(token, secret))]
pub fn verify_jwt(token: &str, secret: String) -> Result<Claims, JwtAuthError> {
    info!("Verificating JWT.");
    let validation = Validation::default();
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    ) {
        Ok(token_data) => Ok(token_data.claims),
        Err(err) => {
            error!("{:?}", err);
            match err.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => Err(JwtAuthError::Expired),
                jsonwebtoken::errors::ErrorKind::InvalidToken => Err(JwtAuthError::Invalid),
                _ => Err(JwtAuthError::JWTError(err)),
            }
        }
    }
}

#[instrument(skip(email, secret))]
pub fn create_jwt(
    sub: i32,
    email: Option<String>,
    secret: String,
    exp: TimeDelta,
) -> Result<String, JwtAuthError> {
    info!("Creating new JWT");

    let now = Utc::now();
    let iat = UnixTimestamp::from(now);
    let exp = UnixTimestamp::from(now + exp);
    info!("EXP : {:?}", exp);
    let claims = Claims {
        sub,
        email: email.map(|e| e.to_owned()),
        exp: exp.clone(),
        iat,
    };
    let header = Header {
        alg: jsonwebtoken::Algorithm::HS256,
        ..Default::default()
    };

    let generated_token = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    info!("JWT creation successfully ended.");

    Ok(generated_token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use chrono::Utc;
    use jsonwebtoken::encode;
    use jsonwebtoken::errors::ErrorKind;
    use serde_json::json;
    use std::convert::TryInto;
    use tracing_test::traced_test;

    const SECRET: &str = "secret_key";

    #[test]

    fn test_create_and_verify_jwt() {
        let sub = 123123;
        let email = Some("test@example.com".to_string());
        let time_delta = TimeDelta::hours(1);
        // Create the token
        let token = create_jwt(sub, email.to_owned(), SECRET.to_string(), time_delta)
            .expect("Token creation failed");

        // Verify the token and extract claims
        let claims = verify_jwt(&token, SECRET.to_string()).expect("Token verification failed");
        assert_eq!(claims.sub, sub);

        // Test conversion from UnixTimestamp to DateTime<Utc>
        let iat: DateTime<Utc> = claims.iat.try_into().expect("Failed to convert iat");
        let exp: DateTime<Utc> = claims.exp.try_into().expect("Failed to convert exp");

        assert!(
            exp > iat,
            "Expiration time must be later than issued at time"
        );
        let diff = exp - iat;
        assert!(
            diff.num_minutes() >= 59 && diff.num_minutes() <= 61,
            "Expiration should be approximately 1 hour after issuance"
        );
    }

    #[test]
    fn test_verify_jwt_with_wrong_secret() {
        let wrong_secret = "wrong_key";
        let sub = 123123;
        let email = None;
        let time_delta = TimeDelta::hours(1);
        // Create the token with the correct secret
        let token =
            create_jwt(sub, email, SECRET.to_string(), time_delta).expect("Token creation failed");

        // Verification should fail when using an incorrect secret
        let result = verify_jwt(&token, wrong_secret.to_string());
        assert!(
            result.is_err(),
            "Verification should fail with an incorrect secret"
        );
    }

    #[test]
    fn test_unix_timestamp_conversion() {
        let now = Utc::now();
        let ts = UnixTimestamp::from(now);
        let converted: DateTime<Utc> = ts.try_into().expect("Conversion failed");

        // Ensure the converted time is nearly equal to the current time (within 1 second)
        let diff = (converted.timestamp() - now.timestamp()).abs();
        assert!(
            diff <= 1,
            "Converted time should be nearly equal to the current time"
        );
    }

    // Boundary condition: test for expired token verification
    #[test]
    #[traced_test]
    fn test_verify_expired_jwt() {
        info!("Testing Started!");
        let sub = 123123;
        let email = Some("expired@example.com");

        let now = Utc::now();
        // Create expired token: issued 2 hours ago and expired 1 hour ago
        let expired_iat = UnixTimestamp::from(now - Duration::hours(2));
        let expired_exp = UnixTimestamp::from(now - Duration::hours(1));
        let claims = Claims {
            sub,
            email: email.map(String::from),
            iat: expired_iat,
            exp: expired_exp,
        };

        // Encode the token using the claims with expired times
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(SECRET.as_bytes()),
        )
        .expect("Token encoding failed");

        let result = verify_jwt(&token, SECRET.to_string());
        info!("{:?}", result);
        let is_expired = result.is_err_and(|e| {
            if let JwtAuthError::JWTError(jwt_err) = e {
                matches!(jwt_err.kind(), ErrorKind::ExpiredSignature)
            } else {
                false
            }
        });
        assert!(is_expired, "Verification should fail for an expired token");
    }

    // Test for invalid token format
    #[test]
    fn test_verify_invalid_token_format() {
        let invalid_token = "this.is.not.a.valid.token";
        let result = verify_jwt(invalid_token, SECRET.to_string());
        assert!(
            result.is_err(),
            "Verification should fail for an invalid token format"
        );
    }

    // Test for token with missing required fields
    #[test]
    fn test_verify_token_with_missing_fields() {
        // Create JSON claims without the 'sub' field
        let incomplete_claims = json!({
            "email": "missing@example.com",
            "iat": (Utc::now() - Duration::hours(1)).timestamp(),
            "exp": (Utc::now() + Duration::hours(1)).timestamp()
        });
        let token = encode(
            &Header::default(),
            &incomplete_claims,
            &EncodingKey::from_secret(SECRET.as_bytes()),
        )
        .expect("Token encoding failed");

        let result = verify_jwt(&token, SECRET.to_string());
        assert!(
            result.is_err(),
            "Verification should fail for a token with missing fields"
        );
    }
}
