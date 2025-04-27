use crate::services::chess_service::ChessService;
use crate::services::engine_service::EngineService;
use crate::services::github_service::{GithubConfig, GithubService};
use actix_web::{App, HttpServer, web};
use env_logger;
use std::sync::Arc;
use std::sync::Mutex;

mod config;
mod controllers;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up logging for debugging
    unsafe {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    // Load configuration from environment variables
    let config = config::Config::from_env().expect("Failed to load config");
    let server_addr = config.server_addr.clone();

    // Initialize core services
    let engine = EngineService::start(&config.engine_path)
        .await
        .expect("Failed to start engine");
    let chess_service = Arc::new(Mutex::new(ChessService::new(engine)));
    let github_config = Arc::new(GithubConfig {
        token: config.github_token.clone(),
        api_url: "https://api.github.com/repos".to_string(),
        owner_repo: config.github_owner_repo.clone(),
        readme_path: config.github_readme_path.clone(),
        branch: config.github_branch.clone(),
    });
    let github_service = Arc::new(GithubService::new(github_config));

    // Start Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(chess_service.clone()))
            .app_data(web::Data::new(github_service.clone()))
            .configure(controllers::init_routes)
    })
    .workers(1)
    .bind(&server_addr)?
    .run()
    .await
}
