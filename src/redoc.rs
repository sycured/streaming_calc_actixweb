use actix_files::NamedFile;
use actix_web::web::{get, resource, HttpRequest, ServiceConfig};
use actix_web::Result;
use std::path::PathBuf;

async fn serve_redoc(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./static/redoc.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(resource("/").route(get().to(serve_redoc)));
}
