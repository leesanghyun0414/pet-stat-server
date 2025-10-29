use crate::gql::guards::AuthGuard;
use crate::gql::objects::User;
use crate::gql::utils::verified_claims_from_ctx;
use crate::{context_data::AccessToken, db::Database};
use async_graphql::{Context, Object, Result};

use service::{jwt::Claims, queries::user::UserQuery as ServiceUserQuery};
use tracing::instrument;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    #[instrument(skip(self, ctx), fields(user_id = id))]
    async fn get_user_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<User> {
        let db = ctx.data::<Database>()?;
        let conn = db.get_connection();

        let user = ServiceUserQuery::user_by_id(conn, id).await?;
        Ok(User::from(user))
    }

    #[graphql(guard = "AuthGuard")]
    async fn get(&self, ctx: &Context<'_>, d: String) -> String {
        d.to_string()
    }

    #[graphql(guard = "AuthGuard")]
    #[instrument(skip(self, ctx))]
    async fn me(&self, ctx: &Context<'_>) -> Result<User> {
        let db = ctx.data::<Database>()?;
        let conn = db.get_connection();

        let claims = verified_claims_from_ctx(ctx)?;

        let id = claims.sub;
        let user = ServiceUserQuery::user_by_id(conn, id).await?;
        Ok(User::from(user))
    }
}
