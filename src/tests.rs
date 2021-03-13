#[cfg(test)]
mod tests {
    use actix_web::{test::{self, TestRequest}, App, body::{Body, ResponseBody}};
    use super::super::*;

    trait BodyTest {
        fn as_str(&self) -> &str;
    }

    impl BodyTest for ResponseBody<Body> {
        fn as_str(&self) -> &str {
            match self {
                ResponseBody::Body(ref b) => match b {
                    Body::Bytes(ref by) => std::str::from_utf8(&by).unwrap(),
                    _ => panic!(),
                },
                ResponseBody::Other(ref b) => match b {
                    Body::Bytes(ref by) => std::str::from_utf8(&by).unwrap(),
                    _ => panic!(),
                },
            }
        }
    }

    #[actix_rt::test]
    async fn index(){
        let mut app = test::init_service(App::new().configure(index::init_routes)).await;

        let resp = TestRequest::get().uri("/").send_request(&mut app).await;
        assert_eq!(resp.status().is_success(), true);

        let resp = TestRequest::post().uri("/").send_request(&mut app).await;
        assert_eq!(resp.status().is_success(), false);

        let resp = TestRequest::delete().uri("/").send_request(&mut app).await;
        assert_eq!(resp.status().is_success(), false);

        let resp = TestRequest::patch().uri("/").send_request(&mut app).await;
        assert_eq!(resp.status().is_success(), false);

        let resp = TestRequest::put().uri("/").send_request(&mut app).await;
        assert_eq!(resp.status().is_success(), false);
    }

    #[actix_rt::test]
    async fn bwserver(){
        let mut app = test::init_service(App::new().configure(bwserver::init_routes)).await;

        let request_body = serde_json::json!({
            "nblisteners": 250,
            "bitrate": 64
        });

        let mut resp = TestRequest::post().uri("/bwserver").set_json(&request_body).send_request(&mut app).await;
        assert!(resp.status().is_success(), "Failed to post");
        assert!(resp.take_body().as_str().contains(":15625.0"), true);

        let resp = TestRequest::delete().uri("/bwserver").send_request(&mut app).await;
        assert_eq!(resp.status().is_success(), false);

        let resp = TestRequest::patch().uri("/bwserver").send_request(&mut app).await;
        assert_eq!(resp.status().is_success(), false);

        let resp = TestRequest::put().uri("/bwserver").send_request(&mut app).await;
        assert_eq!(resp.status().is_success(), false);
    }

    #[actix_rt::test]
    async fn serverusagebw(){
        let mut app = test::init_service(App::new().configure(serverusagebw::init_routes)).await;

        let request_body = serde_json::json!({
            "nblisteners": 250,
            "bitrate": 64,
            "nbdays": 1,
            "nbhours": 24
        });

        let mut resp = TestRequest::post().uri("/serverusagebw").set_json(&request_body).send_request(&mut app).await;
        assert!(resp.status().is_success(), "Failed to post");
        assert!(resp.take_body().as_str().contains(":164794.92"), true);

        let resp = TestRequest::delete().uri("/serverusagebw").send_request(&mut app).await;
        assert_eq!(resp.status().is_success(), false);

        let resp = TestRequest::patch().uri("/serverusagebw").send_request(&mut app).await;
        assert_eq!(resp.status().is_success(), false);

        let resp = TestRequest::put().uri("/serverusagebw").send_request(&mut app).await;
        assert_eq!(resp.status().is_success(), false);
    }
}