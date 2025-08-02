use actix_web::web;

mod graphql;
mod health_check;
pub(crate) mod utils;
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    graphql::graphql_routes(cfg);
    health_check::health_check_routes(cfg);
}
