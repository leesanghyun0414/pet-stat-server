use async_graphql::Guard;
use jwt::Claims;

pub(crate) struct AuthGuard;

impl Guard for AuthGuard {
    async fn check(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<()> {
        if ctx.data_opt::<Claims>().is_some() {
            Ok(())
        } else {
            Err("Unauthorized".into())
        }
    }
}
