use crate::context_data::AccessToken;
use crate::db::Database;
use crate::gql::guards::AuthGuard;
use crate::gql::objects::{OauthPayload, OauthSignInInput, SignOutPayload, TokenRotationPayload};
use crate::gql::utils::db_err_to_gql;
use async_graphql::{Context, Error, Object, Result};
use config::auth_config::AuthConfig;
use entity::entities::sea_orm_active_enums::ProviderType;
use jwt::{create_jwt, verify_jwt, JwtAuthError, DEFAULT_EXP};
use sea_orm::DbErr;
use service::auth::oauth_provider::OAuthProvider;
use service::auth::refresh_token::RefreshToken;
use service::{
    mutations::user::UserMutation as ServiceUserMutation,
    queries::user::UserQuery as ServiceUserQuery,
};
use tracing::{error, info, instrument, warn};

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    #[instrument(skip(self, input, ctx))]
    pub async fn sign(&self, ctx: &Context<'_>, input: OauthSignInInput) -> Result<OauthPayload> {
        info!(
            "Starting OAuth sign-in process for provider: {:?}",
            input.provider_type
        );

        let auth_config = ctx.data::<AuthConfig>()?;
        let db = ctx.data::<Database>()?;
        let conn = db.get_connection();

        // OPTIMIZE: Matching another provider oauth traits after subscript apple developer.
        let google_oauth =
            service::auth::google::GoogleOAuth::new(auth_config.google_oauth_client_id.to_owned())?;

        info!("Verifying OAuth token");
        let claim = google_oauth.verify_token(&input.id_token).await?;
        info!(
            "OAuth token verified successfully for user_id: {:?}",
            claim.sub
        );

        let user = match ServiceUserQuery::user_by_provider_user_id(conn, claim.sub.clone()).await {
            Ok(user) => user,
            Err(DbErr::RecordNotFound(e)) => {
                info!("Record Not Found {:?}", e);
                ServiceUserMutation::create_oauth_user(
                    conn,
                    claim.email,
                    ProviderType::Google,
                    claim.sub,
                )
                .await?
            }
            Err(err) => {
                error!("{:?}", err.to_string());
                return Err(Error::new(err.to_string()));
            }
        };

        info!("Generating JWT for user_id: {:?}", user.id);
        let jwt_token = create_jwt(
            user.id,
            user.email.to_owned(),
            auth_config.jwt_sign_secret.to_owned(),
            DEFAULT_EXP,
        )?;
        info!("JWT generated successfully for user_id: {:?}", user.id);

        info!("Generating refresh token for user_id: {}", user.id);
        let token = RefreshToken::generate()?;
        info!(
            "Refresh token generated and stored for user_id: {}",
            user.id
        );
        let token_hash = token.hash(auth_config.refresh_key_hashing_secret.as_bytes());

        info!("Storing refresh token for user_id: {}", user.id);
        ServiceUserMutation::store_refresh_token(conn, user.id, &token_hash).await?;
        info!("Refresh token stored successfully for user_id: {}", user.id);

        info!(
            "OAuth sign-in completed successfully for user_id: {}",
            user.id
        );
        Ok(OauthPayload {
            access_token: jwt_token,
            refresh_token: token.0,
        })
    }

    /// Disable last refresh token.
    #[graphql(guard = "AuthGuard")]
    #[instrument(skip(self, ctx, refresh_token))]
    pub async fn sign_out(
        &self,
        ctx: &Context<'_>,
        refresh_token: String,
    ) -> Result<SignOutPayload> {
        info!("Starting Sign-Out process.");

        info!("Getting user claims from data.");
        let auth_config = ctx.data::<AuthConfig>()?;
        let token = ctx.data::<AccessToken>()?;
        match verify_jwt(token.0.as_str(), auth_config.jwt_sign_secret.clone()) {
            Ok(_claims) => {
                info!("Access token verified on Sign Out workflow.");
            }
            Err(JwtAuthError::Expired) => {
                warn!("Token expired: {:?}; treating as success", token.0);
            }
            Err(e) => {
                return Err(e.into());
            }
        };

        let db = ctx.data::<Database>()?;
        let conn = db.get_connection();

        info!("Disable user refresh token.");
        let refresh_token_hash =
            RefreshToken(refresh_token).hash(auth_config.refresh_key_hashing_secret.as_bytes());

        // Revoke but throw error that known db error.
        // When sign out is not important token expired, and record search because revoked token
        // record wouldn't using anymore.
        match ServiceUserMutation::revoke_refresh_token(conn, &refresh_token_hash).await {
            Ok(_) => {
                info!("Refresh token revoked successfully.");
            }

            Err(DbErr::RecordNotFound(_)) => {
                warn!("Refresh token not found; treating as success");
            }

            Err(DbErr::Custom(msg)) if msg == "TOKEN_EXPIRED" => {
                warn!("Refresh token already expired; treating as success.");
            }

            Err(e) => {
                error!("Faild to revoke refresh token: {:?}", e);
                db_err_to_gql(e);
            }
        }

        info!("Disable user refresh token has successfully ended.");

        info!("Sign-Out process ended successfully.");

        Ok(SignOutPayload {
            success: true,
            message: "successfully signed out.".to_string(),
        })
    }

    /// Generate new Access token and Refresh token.
    /// This mutations called when access token expired.
    ///
    /// 1. Verifying refresh token. If expired or not found token, response invalid token error
    ///    without detail error.
    /// 2. Revoking old refresh token.
    /// 3. Generate new access token and refresh token.
    /// 4. Storing hashed refresh token to datastore.
    /// 5. Return non-hashed refresh token and access token.
    #[instrument(skip(self, ctx))]
    pub async fn rotate_token(
        &self,
        ctx: &Context<'_>,
        refresh_token: String,
    ) -> Result<TokenRotationPayload> {
        info!("Starting rotating tokens");
        let db = ctx.data::<Database>()?;

        let auth_config = ctx.data::<AuthConfig>()?;
        let conn = db.get_connection();
        let old_refresh_token_hash =
            RefreshToken(refresh_token).hash(auth_config.refresh_key_hashing_secret.as_bytes());

        let new_refresh_token = RefreshToken::generate()?;

        let new_refresh_token_hash =
            new_refresh_token.hash(auth_config.refresh_key_hashing_secret.as_bytes());

        let user_token = ServiceUserMutation::rotate_refresh_token(
            conn,
            &old_refresh_token_hash,
            &new_refresh_token_hash,
        )
        .await
        .map_err(|e| {
            error!("DB Error: {:?}", e);
            db_err_to_gql(e)
        })?;

        let user = ServiceUserQuery::user_by_id(conn, user_token.user_id).await?;

        let access_token = create_jwt(
            user_token.user_id,
            user.email,
            auth_config.jwt_sign_secret.to_owned(),
            DEFAULT_EXP,
        )?;
        Ok(TokenRotationPayload {
            access_token,
            refresh_token: new_refresh_token.0,
        })
    }
}
