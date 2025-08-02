use crate::db::Database;
use crate::gql::guards::AuthGuard;
use crate::gql::objects::User;
use async_graphql::{Context, Object, Result};
use service::{jwt::Claims, queries::user::UserQuery as ServiceUserQuery};
use tracing::{instrument, warn};

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

    // #[graphql(guard = "AuthGuard")]
    async fn get(&self, ctx: &Context<'_>, d: String) -> String {
        let claims = ctx.data::<Claims>().unwrap();
        d.to_string()
    }

    #[instrument(skip(self, ctx))]
    async fn me(&self, ctx: &Context<'_>) -> Result<User> {
        let db = ctx.data::<Database>()?;
        let conn = db.get_connection();
        let id = ctx.data::<Claims>()?.sub.parse::<i32>().unwrap();
        let user = ServiceUserQuery::user_by_id(conn, id).await?;
        Ok(User::from(user))
    }
}
