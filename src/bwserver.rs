use actix_web::{get, post, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Get {
    variables: [String; 2],
    result: String,
    method: String,
    description: String,
}

#[derive(Deserialize)]
pub struct PostJson {
    nblisteners: f32,
    bitrate: f32,
}

#[derive(Serialize)]
pub struct JsonResp {
    result: f32,
}

#[get("/bwserver")]
pub async fn get() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(Get {
        variables: ["nblisteners".to_string(), "bitrate (kb/s)".to_string()],
        result: "Server bandwidth (Mib/s)".to_string(),
        method: "POST a json to this endpoint".to_string(),
        description: "Determine necessary server bandwidth".to_string(),
    }))
}

#[post("/bwserver")]
pub async fn post(data: web::Json<PostJson>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(JsonResp {
        result: 125.0 * data.nblisteners * data.bitrate / 128.0,
    }))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
    cfg.service(post);
}
