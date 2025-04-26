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
    env_logger::init();
    let config = config::Config::from_env().expect("Failed to load config");
    let server_addr = config.server_addr.clone();

    // Initialize services
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

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::from(chess_service.clone()))
            .app_data(web::Data::from(github_service.clone()))
            .configure(controllers::init_routes)
    })
    .bind(&server_addr)?
    .run()
    .await
}
