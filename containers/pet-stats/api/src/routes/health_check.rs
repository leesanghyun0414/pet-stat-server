use actix_web::{get, web, HttpResponse};
use tracing::info;

#[get("/healthz")]
async fn healthz() -> HttpResponse {
    info!("Hello?");
    HttpResponse::Ok().body("Ok")
}

pub(crate) fn health_check_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(healthz);
}
