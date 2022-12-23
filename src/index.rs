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
    use actix_web::{
        dev::Service,
        http::StatusCode,
        test::{call_and_read_body, init_service, TestRequest},
        web::Bytes,
        App,
    };

    use super::*;

    #[actix_web::test]
    async fn test_get() {
        let app = init_service(App::new().configure(init_routes)).await;
        let req = TestRequest::get().uri("/").to_request();
        let resp = call_and_read_body(&app, req).await;
        assert_eq!(resp, Bytes::from_static(b"streaming_calc_actixweb"));
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
