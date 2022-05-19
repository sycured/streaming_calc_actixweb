use actix_web::{HttpRequest, HttpResponse};
use serde_json::Value;

pub fn get_x_request_id_header(req: &HttpRequest) -> Option<&str> {
    req.headers().get("x-request-id")?.to_str().ok()
}

pub fn return_response_json(payload: Value, x_request_id: Option<&str>) -> HttpResponse {
    let mut response = HttpResponse::Ok();
    response.insert_header(("content-type", "application/json"));
    if let Some(x_request_id) = x_request_id {
        response.insert_header(("x-request-id", x_request_id));
    }
    response.json(payload)
}

pub fn return_response_plain(body: &'static str, x_request_id: Option<&str>) -> HttpResponse {
    let mut response = HttpResponse::Ok();
    response.insert_header(("content-type", "text/plain"));
    if let Some(x_request_id) = x_request_id {
        response.insert_header(("x-request-id", x_request_id));
    }
    response.body(body)
}
