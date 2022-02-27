use actix_web::{
    middleware::{Compress, Logger},
    App, HttpServer,
};

use env_logger::Env;

mod configuration;
use configuration::{app_ip, app_port};

mod bwserver;
mod index;
mod serverusagebw;

#[actix_web::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .configure(index::init_routes)
            .configure(bwserver::init_routes)
            .configure(serverusagebw::init_routes)
    })
    .bind(format!(
        "{ip}:{port}",
        ip = app_ip().await,
        port = app_port().await
    ))?
    .run()
    .await
}

#[cfg(test)]
mod trait_imp;
