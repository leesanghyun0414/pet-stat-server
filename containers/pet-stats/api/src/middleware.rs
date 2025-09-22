use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::{ErrorInternalServerError, ErrorUnauthorized, InternalError},
    middleware::Next,
    Error, HttpMessage, HttpResponse,
};
use async_graphql::ErrorExtensions;
use config::{auth_config::AuthConfig, base_config::Config};
use jwt::{verify_jwt, JwtAuthError};
use serde_json::json;
use tracing::{debug, error, info, instrument, warn};

use crate::context_data::AccessToken;

fn extract_bearer_token(req: &ServiceRequest) -> Option<String> {
    req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|s| s.to_string())
}

/// Validate access token from request If exist Authorization header.
/// If not exist Authorization header is mean not guarded request (eg. Sign In)
#[instrument(skip(next), fields())]
pub(crate) async fn access_token_validator(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth_config = AuthConfig::new().map_err(|_| ErrorInternalServerError("Internal Error"))?;
    error!("{:?}", req.headers());
    if let Some(header) = req
        .headers()
        .get("Authorization")
        .inspect(|h| info!("Header Value: {:?}", h))
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
    {
        warn!("HHHHHHHHHHHHHHHA! {:?}", header);
        req.extensions_mut().insert(AccessToken(header.to_string()));
        // match verify_jwt(header, auth_config.jwt_sign_secret) {
        //     Ok(user) => {
        //         req.extensions_mut().insert(user);
        //     }
        //     Err(JwtAuthError::Expired) => {
        //         error!("EXPIRED!!!!");
        //         let resp = HttpResponse::Unauthorized()
        //             .content_type("application/json")
        //             .json(json!({
        //                 "errors": [
        //                     { "message": "Expired token", "code": "TOKEN_EXPIRED" }
        //                 ]
        //             }));
        //
        //         return Err(InternalError::from_response("Expired token", resp).into());
        //     }
        //
        //     Err(_) => {
        //         return Err(ErrorUnauthorized("Invalid Token"));
        //     }
        // }
    }

    let res = next.call(req).await?;
    Ok(res)
}

/// Logging request and response.
///
/// # Errors
///
///  Actix web error when calling next request.
#[instrument(skip(next, req), fields())]
pub(crate) async fn logging_transaction(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    debug!("Received request: {} {}", req.method(), req.path());

    let res = next.call(req).await?;
    debug!("Responsed with: {}", res.response().status());
    Ok(res)
}
