use actix_web::Result;
use paperclip::actix::web::{post, resource, Json, ServiceConfig};
use paperclip::actix::{api_v2_operation, Apiv2Schema};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Apiv2Schema)]
pub struct BwPost {
    nblisteners: f32,
    bitrate: f32,
}

#[derive(Serialize, Apiv2Schema)]
pub struct BwResp {
    result: f32,
}

#[api_v2_operation(
    description = "Determine necessary server bandwidth (MiB/s)",
    consumes = "application/json",
    produces = "application/json"
)]
pub async fn compute(data: Json<BwPost>) -> Result<Json<BwResp>> {
    Ok(Json(BwResp {
        result: 125.0 * data.nblisteners * data.bitrate / 128.0,
    }))
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(resource("/bwserver").route(post().to(compute)));
}
