use actix_web::{
    get,
    http::header::ContentType,
    post,
    web::{self, Data},
    HttpRequest, HttpResponse, Result,
};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use config::{
    app_config::{Flavor, APP_CONFIG},
    secret_config::SecretConfig,
};
use service::jwt::validate_jwt;
use tracing::{error, instrument};

use crate::gql::{mutations::Mutation, queries::Query};

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

#[instrument(skip(schema, gql_req, secret_config))]
#[post("/graphql")]
async fn graphql_handler(
    schema: web::Data<Schema<Query, Mutation, EmptySubscription>>,
    gql_req: GraphQLRequest,
    req: HttpRequest,
    secret_config: Data<SecretConfig>,
) -> GraphQLResponse {
    let request = gql_req.into_inner();
    // if let Some(auth_header) = req
    //     .headers()
    //     .get("Authorization")
    //     .and_then(|h| h.to_str().ok())
    // {
    //     if let Some(token) = auth_header.strip_prefix("Bearer ") {
    //         match validate_jwt(token, &secret_config.jwt_secret) {
    //             Ok(auth_context) => {
    //                 request = request.data(auth_context);
    //             }
    //             Err(err) => {
    //                 error!("{:?}", err);
    //                 let error_response = async_graphql::Response::from_errors(vec![
    //                     async_graphql::ServerError::new("Unauthorized", None),
    //                 ]);
    //
    //                 return error_response.into();
    //             }
    //         }
    //     }
    // }

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
