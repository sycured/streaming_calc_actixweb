use actix_web::body::{Body, ResponseBody};

pub trait BodyTest {
    fn as_str(&self) -> &str;
}

impl BodyTest for ResponseBody<Body> {
    fn as_str(&self) -> &str {
        match self {
            ResponseBody::Body(ref b) => match b {
                Body::Bytes(ref by) => std::str::from_utf8(by).unwrap(),
                _ => panic!(),
            },
            ResponseBody::Other(ref b) => match b {
                Body::Bytes(ref by) => std::str::from_utf8(by).unwrap(),
                _ => panic!(),
            },
        }
    }
}
