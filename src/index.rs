use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct IndexJson {
    status: String,
    app: String,
    method: String,
    endpoints: [String; 2],
    developer: String,
}

#[get("/")]
pub async fn get() -> impl Responder {
    HttpResponse::Ok().json(IndexJson {
        status: "ready".to_string(),
        app: "streaming_calc_actixweb".to_string(),
        method: "GET".to_string(),
        endpoints: ["/bwserver".to_string(), "/serverusagebw".to_string()],
        developer: "sycured".to_string(),
    })
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
}
