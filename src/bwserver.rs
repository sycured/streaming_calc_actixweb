use actix_web::{
    HttpRequest,
    HttpResponse, web::{Json, post, resource, ServiceConfig},
};
use serde::Deserialize;
use serde_json::{json, Value};

use super::util::{get_x_request_id_header, return_response_json};

#[derive(Deserialize)]
pub struct BwPost {
    nblisteners: f32,
    bitrate: f32,
}

pub async fn compute(req: HttpRequest, data: Json<BwPost>) -> HttpResponse {
    let payload: Value = json!({
        "result": 125.0 * data.nblisteners * data.bitrate / 128.0,
    });
    return_response_json(payload, get_x_request_id_header(&req))
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(resource("/bwserver").route(post().to(compute)));
}

#[cfg(test)]
mod tests {
    use actix_web::{
        App,
        body::to_bytes,
        dev::Service,
        http::StatusCode,
        test::{init_service, TestRequest},
    };
    use serde_json::json;

    use super::{BwPost, BwResp, compute, init_routes, Json, super::trait_imp::BodyTest};

    #[actix_web::test]
    async fn test_function() {
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

    #[actix_web::test]
    async fn test_get() {
        let app = init_service(App::new().configure(init_routes)).await;
        let req = TestRequest::get().uri("/bwserver").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[actix_web::test]
    async fn test_delete() {
        let app = init_service(App::new().configure(init_routes)).await;
        let req = TestRequest::delete().uri("/bwserver").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[actix_web::test]
    async fn test_patch() {
        let app = init_service(App::new().configure(init_routes)).await;
        let req = TestRequest::patch().uri("/bwserver").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[actix_web::test]
    async fn test_put() {
        let app = init_service(App::new().configure(init_routes)).await;
        let req = TestRequest::put().uri("/bwserver").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[actix_web::test]
    async fn test_post_1() {
        let req_body = json!({
            "nblisteners": 250,
            "bitrate": 64
        });

        let resp_expected = json!({
            "result": 15625.0
        });

        let app = init_service(App::new().configure(init_routes)).await;

        let req = TestRequest::post()
            .uri("/bwserver")
            .set_json(&req_body)
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(
            to_bytes(resp.into_body()).await.unwrap().as_str(),
            resp_expected.to_string()
        );
    }

    #[actix_web::test]
    async fn test_post_2() {
        let req_body = json!({
            "nblisteners": 250,
            "bitrate": "a"
        });

        let app = init_service(App::new().configure(init_routes)).await;

        let req = TestRequest::post()
            .uri("/bwserver")
            .set_json(&req_body)
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
