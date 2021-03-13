use std::env;

use actix_web::{App, HttpServer};
use actix_web::middleware::Compress;
use actix_web::middleware::Logger;
use env_logger::Env;

mod bwserver;
mod serverusagebw;
mod index;
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let default_ip = "127.0.0.1".to_string();
    let default_port = "8080".to_string();
    let ip = env::var("APP_IP").unwrap_or(default_ip);
    let port = env::var("APP_PORT").unwrap_or(default_port);
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .configure(index::init_routes)
            .configure(bwserver::init_routes)
            .configure(serverusagebw::init_routes)
    })
        .bind(format!("{}:{}", ip, port))?
        .run()
        .await
}