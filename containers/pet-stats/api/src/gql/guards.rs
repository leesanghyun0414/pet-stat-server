use async_graphql::Guard;

use crate::context_data::AccessToken;

pub(crate) struct AuthGuard;

impl Guard for AuthGuard {
    async fn check(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<()> {
        if ctx.data_opt::<AccessToken>().is_some() {
            Ok(())
        } else {
            Err("Unauthorized".into())
        }
    }
}
