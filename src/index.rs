use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

#[get("/")]
pub async fn get() -> impl Responder {
    let index_json = json!({
        "status": "ready",
            "app": "streaming_calc_actixweb",
            "method": "GET",
            "endpoints": ["/bwserver", "/serverusagebw"],
            "developer": "sycured"
    });
    HttpResponse::Ok().json(index_json)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
}
