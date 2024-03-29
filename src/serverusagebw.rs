use actix_multiresponse::Payload;
use actix_web::web::{post, resource, ServiceConfig};
use prost_derive::Message;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Message, Clone)]
pub struct SrvPost {
    #[prost(float, tag = "1")]
    nblisteners: f32,
    #[prost(float, tag = "2")]
    bitrate: f32,
    #[prost(float, tag = "3")]
    nbdays: f32,
    #[prost(float, tag = "4")]
    nbhours: f32,
}

#[derive(Serialize, Deserialize, Message, Clone)]
pub struct Resp {
    #[prost(float, tag = "1")]
    result: f32,
}

pub async fn compute(data: Payload<SrvPost>) -> Payload<Resp> {
    Payload(Resp {
        result: 28125.0 * data.nbdays * data.nbhours * data.bitrate * data.nblisteners / 65536.0,
    })
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(resource("/serverusagebw").route(post().to(compute)));
}

#[cfg(test)]
mod tests {
    use actix_web::{
        dev::Service,
        http::StatusCode,
        test::{call_and_read_body_json, init_service, TestRequest},
        App,
    };
    use serde::Serialize;
    use serde_json::json;

    use super::{init_routes, Resp};

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

        let resp_expected = Resp { result: 164_794.92 };

        let app = init_service(App::new().configure(init_routes)).await;

        let req = TestRequest::post()
            .uri("/serverusagebw")
            .set_json(&req_body)
            .to_request();
        let resp: Resp = call_and_read_body_json(&app, req).await;
        assert_eq!(resp.result, resp_expected.result);
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
