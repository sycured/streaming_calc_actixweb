use actix_web::Result;
use paperclip::actix::web::{post, resource, Json, ServiceConfig};
use paperclip::actix::{api_v2_operation, Apiv2Schema};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Apiv2Schema)]
pub struct SrvPost {
    nblisteners: f32,
    bitrate: f32,
    nbdays: f32,
    nbhours: f32,
}

#[derive(Serialize, Apiv2Schema)]
pub struct SrvResp {
    result: f32,
}

#[api_v2_operation(
    description = "Determine the amount of bandwidth used for the streaming (GiB)",
    consumes = "application/json",
    produces = "application/json"
)]
pub async fn compute(data: Json<SrvPost>) -> Result<Json<SrvResp>> {
    Ok(Json(SrvResp {
        result: 28125.0 * data.nbdays * data.nbhours * data.bitrate * data.nblisteners / 65536.0,
    }))
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(resource("/serverusagebw").route(post().to(compute)));
}

#[cfg(test)]
#[actix_rt::test]
async fn serverusagebw() {
    assert_eq!(
        Json(SrvResp { result: 164794.92 }).result,
        compute(Json(SrvPost {
            nblisteners: 250.0,
            bitrate: 64.0,
            nbdays: 1.0,
            nbhours: 24.0
        }))
        .await
        .unwrap()
        .result
    )
}
