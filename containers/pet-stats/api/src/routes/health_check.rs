use actix_web::{get, web, HttpResponse};

#[get("/healthz")]
async fn healthz() -> HttpResponse {
    HttpResponse::Ok().body("Ok")
}

pub(crate) fn health_check_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(healthz);
}
