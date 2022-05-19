use std::str::from_utf8;

use actix_web::web::Bytes;

pub trait BodyTest {
    fn as_str(&self) -> &str;
}

impl BodyTest for Bytes {
    fn as_str(&self) -> &str {
        from_utf8(self).unwrap()
    }
}
