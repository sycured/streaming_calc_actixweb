use actix_files::NamedFile;
use actix_web::{
    web::{get, resource, HttpRequest, ServiceConfig},
    Result,
};
use std::path::PathBuf;

async fn serve_redoc(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./static/redoc.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(resource("/").route(get().to(serve_redoc)));
}

#[cfg(test)]
mod tests {
    use super::init_routes;
    use actix_web::{
        test::{init_service, TestRequest},
        App,
    };

    #[actix_rt::test]
    async fn via_app() {
        let mut app = init_service(App::new().configure(init_routes)).await;

        let resp = TestRequest::get().uri("/").send_request(&mut app).await;
        assert_eq!(resp.status().as_u16(), 200);

        let resp = TestRequest::post().uri("/").send_request(&mut app).await;
        assert_eq!(resp.status().as_u16(), 405);

        let resp = TestRequest::delete().uri("/").send_request(&mut app).await;
        assert_eq!(resp.status().as_u16(), 405);

        let resp = TestRequest::patch().uri("/").send_request(&mut app).await;
        assert_eq!(resp.status().as_u16(), 405);

        let resp = TestRequest::put().uri("/").send_request(&mut app).await;
        assert_eq!(resp.status().as_u16(), 405);
    }
}
