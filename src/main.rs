use actix_web::{App, HttpServer, middleware, web};
use env_logger::Env;
use rust_readme_chess::config::Config;
use rust_readme_chess::controllers;
use rust_readme_chess::services::chess_service::ChessService;
use rust_readme_chess::services::engine_service::EngineService;
use rust_readme_chess::services::game_actor;
use rust_readme_chess::services::github_service::{GithubConfig, GithubService};
use rust_readme_chess::utils::printer::{self, MarkdownPrinter};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Load configuration from environment variables
    let config = Config::from_env().expect("Failed to load config");
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
    let mut service = ChessService::new(engine);
    restore_game(&mut service, &github_service).await;
    let game = game_actor::spawn(service, github_service, printer);

    // Start Actix web server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::DefaultHeaders::new().add(("Cache-Control", "no-store")))
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(game.clone()))
            .configure(controllers::init_routes)
    })
    .bind(&server_addr)?
    .run()
    .await
}

// The README is the only record of the game that survives a restart; a README we can't
// read or replay means a fresh game, never a refusal to start.
async fn restore_game(service: &mut ChessService, github: &GithubService) {
    let readme = match github.fetch_readme().await {
        Ok(readme) => readme,
        Err(e) => {
            log::warn!("could not read the README ({:#}); starting a new game", e);
            return;
        }
    };
    let moves = printer::parse_moves(&readme);
    match service.restore(&moves).await {
        Ok(()) => log::info!("restored a game of {} moves from the README", moves.len()),
        Err(e) => {
            log::warn!("README move list did not replay ({}); starting a new game", e);
            service.new_game().await.expect("engine failed to reset");
        }
    }
}
