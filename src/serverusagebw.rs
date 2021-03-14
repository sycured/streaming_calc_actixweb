use actix_web::{get, post, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Get {
    variables: [String; 4],
    result: String,
    method: String,
    description: String,
}

#[derive(Deserialize)]
pub struct PostJson {
    nblisteners: f32,
    bitrate: f32,
    nbdays: f32,
    nbhours: f32,
}

#[derive(Serialize)]
pub struct JsonResp {
    result: f32,
}

#[get("/serverusagebw")]
pub async fn get() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(Get {
        variables: [
            "nblisteners".to_string(),
            "bitrate (kb/s)".to_string(),
            "nbdays".to_string(),
            "nbhours".to_string(),
        ],
        result: "Bandwidth used (GiB)".to_string(),
        method: "POST a json to this endpoint".to_string(),
        description: "Determine the amount of data used for the streaming".to_string(),
    }))
}

#[post("/serverusagebw")]
pub async fn post(data: web::Json<PostJson>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(JsonResp {
        result: 28125.0 * data.nbdays * data.nbhours * data.bitrate * data.nblisteners / 65536.0,
    }))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
    cfg.service(post);
}
