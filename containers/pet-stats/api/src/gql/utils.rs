use async_graphql::{Context, Error, ErrorExtensions};
use config::auth_config::AuthConfig;
use jwt::{verify_jwt, Claims, JwtAuthError};
use sea_orm::DbErr;
use tracing::{error, info};

use crate::context_data::AccessToken;

#[inline]
fn gql_err(code: &'static str, msg: impl Into<String>) -> Error {
    Error::new(msg).extend_with(|_, e| e.set("code", code))
}

pub fn db_err_to_gql(err: DbErr) -> Error {
    match err {
        DbErr::Custom(s) if s == "TOKEN_EXPIRED" => {
            gql_err("TOKEN_EXPIRED", "Expired refresh token")
        }
        other => gql_err("OTHER_ERROR", other.to_string()),
    }
}

pub fn verified_claims_from_ctx(ctx: &Context<'_>) -> Result<Claims, Error> {
    let auth_config = ctx.data::<AuthConfig>()?;
    let token = ctx.data::<AccessToken>()?;

    let claims = verify_jwt(token.0.as_str(), auth_config.jwt_sign_secret.to_owned()).map_err(
        |e| match e {
            JwtAuthError::Expired => {
                error!("Access Token Expired");
                gql_err("ACCESS_TOKEN_EXPIRED", "Access Token Expired")
            }
            other => other.into(),
        },
    )?;

    info!("Succeed validation.");

    Ok(claims)
}
