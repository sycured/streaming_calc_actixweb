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
