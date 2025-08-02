use crate::db::Database;
use crate::gql::objects::{Input, OauthPayload};
use async_graphql::{Context, Error, Object, Result};
use chrono::TimeDelta;
use config::auth_config::{self, AuthConfig};
use entity::entities::sea_orm_active_enums::ProviderType;
use jwt::create_jwt;
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
    pub async fn sign(&self, ctx: &Context<'_>, input: Input) -> Result<OauthPayload> {
        let auth_config = ctx.data::<AuthConfig>()?;
        let db = ctx.data::<Database>()?;
        let conn = db.get_connection();

        let google_oauth =
            service::auth::google::GoogleOAuth::new(auth_config.google_oauth_client_id.to_owned())?;
        let claim = google_oauth.verify_token(&input.id_token).await?;

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

        let jwt_token = create_jwt(
            user.id.to_string(),
            user.email.to_owned(),
            auth_config.jwt_sign_secret.to_owned(),
            TimeDelta::minutes(30),
        )?;

        let token = RefreshToken::generate()?;
        let token_hash = token.hash(auth_config.refresh_key_hashing_secret.as_bytes());

        ServiceUserMutation::store_refresh_token(conn, user.id, &token_hash).await?;

        debug!("{}, {}", jwt_token, token.0);
        Ok(OauthPayload {
            access_token: jwt_token,
            refresh_token: token.0,
        })
    }

    #[instrument(skip(self, ctx))]
    pub async fn sign_out(&self, ctx: &Context<'_>) -> Result<String> {
        let auth_config = ctx.data::<AuthConfig>()?;

        let db = ctx.data::<Database>()?;
        let conn = db.get_connection();

        todo!()
    }
}
