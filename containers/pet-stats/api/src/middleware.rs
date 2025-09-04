use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::{ErrorInternalServerError, ErrorUnauthorized},
    middleware::Next,
    Error, HttpMessage,
};
use config::{auth_config::AuthConfig, base_config::Config};
use service::jwt::validate_jwt;
use tracing::{debug, info, instrument};

/// Validate access token from request If exist Authorization header.
/// If not exist Authorization header is mean not guarded request (eg. Sign In)
#[instrument(skip(next), fields())]
pub(crate) async fn access_token_validator(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth_config = AuthConfig::new().map_err(|_| ErrorInternalServerError("Internal Error"))?;

    if let Some(header) = req
        .headers()
        .get("Authorization")
        .inspect(|h| info!("{:?}", h))
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
    {
        match validate_jwt(header, &auth_config.jwt_sign_secret) {
            Ok(user) => {
                req.extensions_mut().insert(user);
            }
            Err(e) => return Err(ErrorUnauthorized(e)),
        }
    }

    let res = next.call(req).await?;
    info!("{:?}", res.response());
    Ok(res)
}

/// Logging request and response.
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
