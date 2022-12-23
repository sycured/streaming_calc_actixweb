#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

use actix_request_identifier::RequestIdentifier;
use actix_web::{
    middleware::{Compress, Logger},
    App, HttpServer,
};
use env_logger::Env;

use configuration::{app_ip, app_port, cors};

mod configuration;

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
            .wrap(cors())
            .wrap(RequestIdentifier::with_uuid())
            .wrap(Logger::new(
                r#" %a %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" "%{x-request-id}o" %T"#,
            ))
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
