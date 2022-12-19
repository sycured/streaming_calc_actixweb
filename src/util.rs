use actix_web::{
    http::header::{HeaderName, HeaderValue},
    HttpRequest, HttpResponse,
};
use serde_json::Value;

pub fn get_x_request_id_header(req: &HttpRequest) -> Option<&str> {
    req.headers().get("x-request-id")?.to_str().ok()
}

pub fn return_response_json(payload: Value, x_request_id: Option<&str>) -> HttpResponse {
    let mut response = HttpResponse::Ok().json(payload);
    if let Some(x_request_id) = x_request_id {
        response.headers_mut().insert(
            HeaderName::from_static("x-request-id"),
            HeaderValue::from_str(x_request_id).unwrap(),
        );
    }
    response
}

pub fn return_response_plain(body: &'static str, x_request_id: Option<&str>) -> HttpResponse {
    let mut response = HttpResponse::Ok()
        .append_header(("content-type", "text/plain"))
        .body(body);
    if let Some(x_request_id) = x_request_id {
        response.headers_mut().insert(
            HeaderName::from_static("x-request-id"),
            HeaderValue::from_str(x_request_id).unwrap(),
        );
    }
    response
}

#[cfg(test)]
mod tests {
    use actix_web::{body::to_bytes, test::TestRequest};
    use serde_json::json;

    use super::{get_x_request_id_header, return_response_json, return_response_plain};

    #[actix_web::test]
    async fn test_get_x_request_id_header() {
        let x_request_id = "4D590945-64E8-45F8-B34F-737608FAB875";
        let req = TestRequest::default()
            .insert_header(("x-request-id", x_request_id))
            .to_http_request();
        assert_eq!(get_x_request_id_header(&req).unwrap(), x_request_id);
    }

    #[actix_web::test]
    async fn test_return_response_json() {
        let x_request_id = "4D590945-64E8-45F8-B34F-737608FAB875";
        let payload = json!({
            "code": 200,
            "success": true,
            "payload": {
                "features": [
                    "serde",
                    "json"
                ]
            }
        });

        let resp = return_response_json(payload.clone(), Some(x_request_id));

        assert_eq!(resp.headers().get("x-request-id").unwrap(), x_request_id);

        assert_eq!(
            to_bytes(resp.into_body()).await.unwrap(),
            payload.to_string().as_bytes()
        );
    }

    #[actix_web::test]
    async fn test_resturn_response_plain() {
        let x_request_id = "4D590945-64E8-45F8-B34F-737608FAB875";
        let body = &"It's a test";

        let resp = return_response_plain(body, Some(x_request_id));

        assert_eq!(resp.headers().get("x-request-id").unwrap(), x_request_id);
        assert_eq!(to_bytes(resp.into_body()).await.unwrap(), body);
    }
}
