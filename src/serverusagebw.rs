use actix_web::{
    web::{post, resource, Json, ServiceConfig},
    Result,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SrvPost {
    nblisteners: f32,
    bitrate: f32,
    nbdays: f32,
    nbhours: f32,
}

#[derive(Debug, Serialize)]
pub struct SrvResp {
    result: f32,
}

pub async fn compute(data: Json<SrvPost>) -> Result<Json<SrvResp>> {
    Ok(Json(SrvResp {
        result: 28125.0 * data.nbdays * data.nbhours * data.bitrate * data.nblisteners / 65536.0,
    }))
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(resource("/serverusagebw").route(post().to(compute)));
}

#[cfg(test)]
mod tests {
    use super::{super::trait_imp::BodyTest, compute, init_routes, Json, SrvPost, SrvResp};
    use actix_web::{
        body::to_bytes,
        dev::Service,
        http::StatusCode,
        test::{init_service, TestRequest},
        App,
    };
    use serde_json::json;

    #[actix_web::test]
    async fn test_function() {
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
            "result": 164794.92
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
