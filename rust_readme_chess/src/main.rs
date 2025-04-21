use actix_web::{App, HttpServer, web};
use env_logger;

mod config;
mod controllers;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let config = config::Config::from_env().expect("Failed to load config");
    // Clone the server address before moving `config` into the closure
    let server_addr = config.server_addr.clone();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .configure(controllers::init_routes)
    })
    .bind(&server_addr)?
    .run()
    .await
}
