#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

use actix_cors::Cors;
use actix_web::{
    middleware::{Compress, Logger},
    App, HttpServer,
};
use env_logger::Env;

use configuration::{app_ip, app_port};

mod configuration;

mod bwserver;
mod index;
mod serverusagebw;
mod util;

#[actix_web::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("https://schy.sycured.com")
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .wrap(cors)
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
