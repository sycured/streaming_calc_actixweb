use actix_web::Result;
use paperclip::actix::web::{post, resource, Json, ServiceConfig};
use paperclip::actix::{api_v2_operation, Apiv2Schema};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Apiv2Schema)]
pub struct BwPost {
    nblisteners: f32,
    bitrate: f32,
}

#[derive(Debug, Serialize, Apiv2Schema)]
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

#[cfg(test)]
mod tests {
    use super::super::trait_imp::BodyTest;
    use super::{compute, init_routes, BwPost, BwResp, Json};
    use actix_web::{
        test::{init_service, TestRequest},
        App,
    };
    use paperclip::actix::OpenApiExt;
    use serde_json::json;

    #[actix_rt::test]
    async fn function() {
        assert_eq!(
            Json(BwResp { result: 15625.0 }).result,
            compute(Json(BwPost {
                nblisteners: 250.0,
                bitrate: 64.0,
            }))
            .await
            .unwrap()
            .result
        )
    }

    #[actix_rt::test]
    async fn via_app() {
        let mut app = init_service(App::new().wrap_api().configure(init_routes).build()).await;

        let request_body = json!({
            "nblisteners": 250,
            "bitrate": 64
        });

        let bad_request_body = json!({
        "nblisteners": 250,
        "bitrate": "a"
        });

        let resp_expected = json!({"result": 15625.0 });

        let resp = TestRequest::get()
            .uri("/bwserver")
            .send_request(&mut app)
            .await;
        assert_eq!(resp.status().as_u16(), 405);

        let mut resp = TestRequest::post()
            .uri("/bwserver")
            .set_json(&request_body)
            .send_request(&mut app)
            .await;
        assert_eq!(resp.status().as_u16(), 200);
        assert_eq!(resp.take_body().as_str(), resp_expected.to_string());

        let resp = TestRequest::post()
            .uri("/bwserver")
            .set_json(&bad_request_body)
            .send_request(&mut app)
            .await;
        assert_eq!(resp.status().as_u16(), 400);

        let resp = TestRequest::delete()
            .uri("/bwserver")
            .send_request(&mut app)
            .await;
        assert_eq!(resp.status().as_u16(), 405);

        let resp = TestRequest::patch()
            .uri("/bwserver")
            .send_request(&mut app)
            .await;
        assert_eq!(resp.status().as_u16(), 405);

        let resp = TestRequest::put()
            .uri("/bwserver")
            .send_request(&mut app)
            .await;
        assert_eq!(resp.status().as_u16(), 405);
    }
}
