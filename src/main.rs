use actix_web::{
    middleware::{Compress, Logger},
    App, HttpServer,
};

use env_logger::Env;
use paperclip::actix::OpenApiExt;

mod configuration;
use configuration::{app_ip, app_port};

mod bwserver;
mod redoc;
mod serverusagebw;
mod trait_imp;

#[actix_web::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| {
        App::new()
            .wrap_api()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .configure(bwserver::init_routes)
            .configure(serverusagebw::init_routes)
            .with_json_spec_at("/openapi.json")
            .build()
            .configure(redoc::init_routes)
    })
    .bind(format!(
        "{ip}:{port}",
        ip = app_ip().await,
        port = app_port().await
    ))?
    .run()
    .await
}
