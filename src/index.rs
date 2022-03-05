use actix_web::{
    web::{get, resource, ServiceConfig},
    HttpResponse,
};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("streaming_calc_actixweb")
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(resource("/").route(get().to(index)));
}

#[cfg(test)]
mod tests {
    use super::super::trait_imp::BodyTest;
    use super::*;
    use actix_web::{
        body::to_bytes,
        dev::Service,
        http::StatusCode,
        test::{init_service, TestRequest},
        App,
    };

    #[actix_web::test]
    async fn test_function() {
        assert_eq!(index().await.status(), StatusCode::OK);
        assert_eq!(
            to_bytes(index().await.into_body()).await.unwrap().as_str(),
            "streaming_calc_actixweb".to_string()
        );
    }

    #[actix_web::test]
    async fn test_get() {
        let app = init_service(App::new().configure(init_routes)).await;
        let req = TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_post() {
        let app = init_service(App::new().configure(init_routes)).await;
        let req = TestRequest::post().uri("/").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
    }
    #[actix_web::test]
    async fn test_delete() {
        let app = init_service(App::new().configure(init_routes)).await;
        let req = TestRequest::delete().uri("/").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
    }
    #[actix_web::test]
    async fn test_put() {
        let app = init_service(App::new().configure(init_routes)).await;
        let req = TestRequest::put().uri("/").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
    }
    #[actix_web::test]
    async fn test_patch() {
        let app = init_service(App::new().configure(init_routes)).await;
        let req = TestRequest::patch().uri("/").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
    }
}
