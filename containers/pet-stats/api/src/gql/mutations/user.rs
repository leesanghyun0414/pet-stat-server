use crate::db::Database;
use crate::gql::guards::AuthGuard;
use crate::gql::objects::{OauthPayload, OauthSignInInput, SignOutPayload};
use async_graphql::{Context, EmptyMutation, Error, ErrorExtensions, Object, Result};
use chrono::TimeDelta;
use config::auth_config::AuthConfig;
use entity::entities::sea_orm_active_enums::ProviderType;
use jwt::{create_jwt, Claims};
use sea_orm::DbErr;
use service::auth::oauth_provider::OAuthProvider;
use service::auth::refresh_token::RefreshToken;
use service::{
    mutations::user::UserMutation as ServiceUserMutation,
    queries::user::UserQuery as ServiceUserQuery,
};
use tracing::{debug, error, info, instrument};

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
            TimeDelta::minutes(30),
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
    #[instrument(skip(self, ctx))]
    pub async fn sign_out(
        &self,
        ctx: &Context<'_>,
        refresh_token: String,
    ) -> Result<SignOutPayload> {
        info!("Starting Sign-Out process.");

        info!("Getting user claims from data.");
        let user_id = ctx.data::<Claims>()?.sub;
        info!("User claims successfully getting user_id: {:?}.", user_id);
        let db = ctx.data::<Database>()?;
        let conn = db.get_connection();

        info!("Disable user refresh token.");
        let auth_config = ctx.data::<AuthConfig>()?;
        let refresh_token_hash =
            RefreshToken(refresh_token).hash(auth_config.refresh_key_hashing_secret.as_bytes());

        ServiceUserMutation::revoke_refresh_token(conn, &refresh_token_hash, user_id)
            .await
            .map_err(|err| {
                error!("Failed to revoke refresh token: {:?}", err);

                Error::new("Failed to sign out")
                    .extend_with(|_e, ext| ext.set("code", "SIGN_OUT_FAILED"))
            })?;

        info!("Disable user refresh token has successfully ended.");

        info!("Sign-Out process ended successfully.");

        Ok(SignOutPayload {
            success: true,
            message: "successfully signed out.".to_string(),
        })
    }
}
