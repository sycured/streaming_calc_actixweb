use actix_multiresponse::Payload;
use actix_web::web::{post, resource, ServiceConfig};
use prost_derive::Message;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Message, Clone)]
pub struct BwPost {
    #[prost(float, tag = "1")]
    nblisteners: f32,
    #[prost(float, tag = "2")]
    bitrate: f32,
}

#[derive(Serialize, Deserialize, Clone, Message)]
pub struct Resp {
    #[prost(float, tag = "1")]
    result: f32,
}

pub async fn compute(data: Payload<BwPost>) -> Payload<Resp> {
    Payload(Resp {
        result: 125.0 * data.nblisteners * data.bitrate / 128.0,
    })
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(resource("/bwserver").route(post().to(compute)));
}

#[cfg(test)]
mod tests {
    use actix_web::{
        dev::Service,
        http::StatusCode,
        test::{call_and_read_body_json, init_service, TestRequest},
        App,
    };
    use serde_json::json;

    use super::{init_routes, Resp};

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

        let resp_expected = Resp { result: 15625.0 };

        let app = init_service(App::new().configure(init_routes)).await;

        let req = TestRequest::post()
            .uri("/bwserver")
            .set_json(&req_body)
            .to_request();
        let resp: Resp = call_and_read_body_json(&app, req).await;
        assert_eq!(resp.result, resp_expected.result);
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
