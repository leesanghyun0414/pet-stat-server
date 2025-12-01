use actix_web::{
    get,
    http::header::ContentType,
    post,
    web::{self},
    HttpMessage, HttpRequest, HttpResponse, Result,
};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use config::app_config::{Flavor, APP_CONFIG};
use tracing::instrument;

use crate::{
    context_data::AccessToken,
    gql::{mutations::Mutation, queries::Query},
};

#[instrument]
#[get("/graphql-pg")]
async fn playground() -> Result<HttpResponse> {
    let source = playground_source(
        GraphQLPlaygroundConfig::new("/graphql-pg").subscription_endpoint("/graphql-pg"),
    );
    Ok(HttpResponse::Ok()
        .insert_header(ContentType::html())
        .body(source))
}

#[instrument(skip(schema, gql_req,))]
#[post("/graphql")]
async fn graphql_handler(
    schema: web::Data<Schema<Query, Mutation, EmptySubscription>>,
    gql_req: GraphQLRequest,
    req: HttpRequest,
) -> GraphQLResponse {
    let mut request = gql_req.into_inner();
    if let Some(tok) = req.extensions().get::<AccessToken>().cloned() {
        request = request.data(tok)
    };

    schema.execute(request).await.into()
}

#[instrument(skip(cfg))]
pub(crate) fn graphql_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(graphql_handler);

    let app_config = &*APP_CONFIG;

    if matches!(app_config.flavor, Flavor::Dev) {
        cfg.service(playground);
    }
}
