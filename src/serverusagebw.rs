use actix_web::{
    web::{post, resource, Json, ServiceConfig},
    HttpRequest, HttpResponse,
};
use serde::Deserialize;
use serde_json::{json, Value};

use super::util::{get_x_request_id_header, return_response_json};

#[derive(Deserialize)]
pub struct SrvPost {
    nblisteners: f32,
    bitrate: f32,
    nbdays: f32,
    nbhours: f32,
}

pub async fn compute(req: HttpRequest, data: Json<SrvPost>) -> HttpResponse {
    let payload: Value = json!({
        "result": 28125.0 * data.nbdays * data.nbhours * data.bitrate * data.nblisteners / 65536.0,
    });
    return_response_json(payload, get_x_request_id_header(&req))
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(resource("/serverusagebw").route(post().to(compute)));
}

#[cfg(test)]
mod tests {
    use actix_web::{
        body::to_bytes,
        dev::Service,
        http::StatusCode,
        test::{init_service, TestRequest},
        App,
    };
    use serde::Serialize;
    use serde_json::json;

    use super::{super::trait_imp::BodyTest, init_routes};

    #[derive(Debug, Serialize)]
    pub struct SrvResp {
        result: f32,
    }

    #[actix_web::test]
    async fn test_get() {
        let app = init_service(App::new().configure(init_routes)).await;
        let req = TestRequest::get().uri("/serverusagebw").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[actix_web::test]
    async fn test_delete() {
        let app = init_service(App::new().configure(init_routes)).await;
        let req = TestRequest::delete().uri("/serverusagebw").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[actix_web::test]
    async fn test_patch() {
        let app = init_service(App::new().configure(init_routes)).await;
        let req = TestRequest::patch().uri("/serverusagebw").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[actix_web::test]
    async fn test_put() {
        let app = init_service(App::new().configure(init_routes)).await;
        let req = TestRequest::put().uri("/serverusagebw").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[actix_web::test]
    async fn test_post_1() {
        let req_body = json!({
            "nblisteners": 250,
            "bitrate": 64,
            "nbdays": 1,
            "nbhours": 24
        });

        let resp_expected = json!({
            "result": 164794.921875
        });

        let app = init_service(App::new().configure(init_routes)).await;

        let req = TestRequest::post()
            .uri("/serverusagebw")
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
            "bitrate": 64,
            "nbdays": "a",
            "nbhours": 24
        });

        let app = init_service(App::new().configure(init_routes)).await;

        let req = TestRequest::post()
            .uri("/serverusagebw")
            .set_json(&req_body)
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
