use crate::services::chess_service::ChessService;
use crate::services::engine_service::EngineService;
use crate::services::game_actor;
use crate::services::github_service::{GithubConfig, GithubService};
use crate::utils::printer::MarkdownPrinter;
use actix_web::{App, HttpServer, web};
use env_logger::Env;
use std::sync::Arc;

mod config;
mod controllers;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Load configuration from environment variables
    let config = config::Config::from_env().expect("Failed to load config");
    let server_addr = config.server_addr.clone();

    // Initialize core services
    let engine = EngineService::start(&config.engine_path)
        .await
        .expect("Failed to start engine");
    let github_service = GithubService::new(Arc::new(GithubConfig {
        token: config.github_token.clone(),
        api_url: "https://api.github.com/repos".to_string(),
        owner_repo: config.github_owner_repo.clone(),
        readme_path: config.github_readme_path.clone(),
        branch: config.github_branch.clone(),
    }));
    let printer = MarkdownPrinter::new(config.base_url.clone(), config.github_owner_repo.clone());
    let game = game_actor::spawn(ChessService::new(engine), github_service, printer);

    // Start Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(game.clone()))
            .configure(controllers::init_routes)
    })
    .bind(&server_addr)?
    .run()
    .await
}
