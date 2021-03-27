use std::env;

use actix_web::middleware::Compress;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use env_logger::Env;
use paperclip::actix::OpenApiExt;

mod bwserver;
mod serverusagebw;
mod trait_imp;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let default_ip = "127.0.0.1".to_string();
    let default_port = "8080".to_string();
    let ip = env::var("APP_IP").unwrap_or(default_ip);
    let port = env::var("APP_PORT").unwrap_or(default_port);

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| {
        App::new()
            .wrap_api()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .configure(bwserver::init_routes)
            .configure(serverusagebw::init_routes)
            .with_json_spec_at("/")
            .build()
    })
    .bind(format!("{}:{}", ip, port))?
    .run()
    .await
}
