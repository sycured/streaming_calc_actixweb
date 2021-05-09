use actix_web::Result;
use paperclip::actix::{
    api_v2_operation,
    web::{post, resource, Json, ServiceConfig},
    Apiv2Schema,
};
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

pub fn init_routes(cfg: &mut ServiceConfig<'_>) {
    cfg.service(resource("/serverusagebw").route(post().to(compute)));
}

#[cfg(test)]
mod tests {
    use super::{super::trait_imp::BodyTest, compute, init_routes, Json, SrvPost, SrvResp};
    use actix_web::{
        test::{init_service, TestRequest},
        App,
    };
    use paperclip::actix::OpenApiExt;
    use serde_json::json;

    #[actix_rt::test]
    async fn function() {
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

    #[actix_rt::test]
    async fn via_app() {
        let mut app = init_service(App::new().wrap_api().configure(init_routes).build()).await;

        let request_body = json!({
            "nblisteners": 250,
            "bitrate": 64,
            "nbdays": 1,
            "nbhours": 24
        });

        let bad_request_body = json!({
            "nblisteners": 250,
            "bitrate": 64,
            "nbdays": "a",
            "nbhours": 24
        });

        let resp_expected = json!({"result": 164794.92});

        let resp = TestRequest::get()
            .uri("/serverusagebw")
            .send_request(&mut app)
            .await;
        assert_eq!(resp.status().as_u16(), 405);

        let mut resp = TestRequest::post()
            .uri("/serverusagebw")
            .set_json(&request_body)
            .send_request(&mut app)
            .await;
        assert_eq!(resp.status().as_u16(), 200);
        assert_eq!(resp.take_body().as_str(), resp_expected.to_string());

        let resp = TestRequest::post()
            .uri("/serverusagebw")
            .set_json(&bad_request_body)
            .send_request(&mut app)
            .await;
        assert_eq!(resp.status().as_u16(), 400);

        let resp = TestRequest::delete()
            .uri("/serverusagebw")
            .send_request(&mut app)
            .await;
        assert_eq!(resp.status().as_u16(), 405);

        let resp = TestRequest::patch()
            .uri("/serverusagebw")
            .send_request(&mut app)
            .await;
        assert_eq!(resp.status().as_u16(), 405);

        let resp = TestRequest::put()
            .uri("/serverusagebw")
            .send_request(&mut app)
            .await;
        assert_eq!(resp.status().as_u16(), 405);
    }
}
