use std::sync::Arc;

use async_graphql::{
    extensions::{Extension, ExtensionContext, ExtensionFactory, NextPrepareRequest, NextRequest},
    Request, Response, ServerResult,
};
use async_trait::async_trait;
use config::app_config::APP_CONFIG;
use tracing::info;

pub(crate) struct AuthExtension;

#[async_trait]
impl Extension for AuthExtension {
    async fn prepare_request(
        &self,
        ctx: &ExtensionContext<'_>,
        request: Request,
        next: NextPrepareRequest<'_>,
    ) -> ServerResult<Request> {
        let operation_name = request.operation_name.clone().unwrap_or_default();
        info!("Your Query - {:?}", operation_name);
        info!(
            "Skip operation targets - {:?}",
            &*APP_CONFIG.skip_middleware_operations
        );

        if APP_CONFIG
            .skip_middleware_operations
            .contains(&operation_name)
        {
            return next.run(ctx, request).await;
        }

        // The code here will be run before the prepare_request is executed, just after the request lifecycle hook.
        let result = next.run(ctx, request).await;

        // The code here will be run just after the prepare_request
        result
    }
}

impl ExtensionFactory for AuthExtension {
    fn create(&self) -> std::sync::Arc<dyn Extension> {
        Arc::new(AuthExtension {}) as Arc<dyn Extension>
    }
}
