use crate::config::Config;
use crate::services::chess_service::ChessService;
use crate::services::github_service::GithubService;
use crate::utils::printer::MarkdownPrinter;
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize)]
pub struct PlayQuery {
    /// `move` is a reserved keyword, so we use `mv`.
    pub mv: String,
}

/// Apply the player's move and the engine's reply.
pub async fn play(
    query: web::Query<PlayQuery>,
    chess_service: web::Data<Arc<Mutex<ChessService>>>,
    github_service: web::Data<Arc<GithubService>>,
    config: web::Data<Config>,
) -> impl Responder {
    let mut service = chess_service.lock().await;
    if let Err(e) = service.play(&query.mv).await {
        return HttpResponse::BadRequest().body(format!("Invalid move: {}", e));
    }
    let fen = match service.get_fen().await {
        Ok(f) => f,
        Err(e) => return HttpResponse::InternalServerError().body(format!("FEN error: {}", e)),
    };
    let valid_moves = match service.get_valid_moves().await {
        Ok(m) => m,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Moves error: {}", e)),
    };
    // Use config.base_url for endpoint links
    let printer = MarkdownPrinter::new(config.base_url.clone());
    let board_md = printer.print(fen, valid_moves, "");
    // Update GitHub README
    let _ = github_service.update_readme(&board_md).await;
    HttpResponse::Ok().body(board_md)
}

#[derive(Deserialize)]
pub struct SelectQuery {
    pub square: String,
}

/// Highlight or deselect a square.
pub async fn select(
    query: web::Query<SelectQuery>,
    chess_service: web::Data<Arc<Mutex<ChessService>>>,
    github_service: web::Data<Arc<GithubService>>,
    config: web::Data<Config>,
) -> impl Responder {
    let mut service = chess_service.lock().await;
    if let Err(e) = service.select(&query.square).await {
        return HttpResponse::BadRequest().body(format!("Select error: {}", e));
    }
    let fen = match service.get_fen().await {
        Ok(f) => f,
        Err(e) => return HttpResponse::InternalServerError().body(format!("FEN error: {}", e)),
    };
    let valid_moves = match service.get_valid_moves().await {
        Ok(m) => m,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Moves error: {}", e)),
    };
    let selected = service.get_selected_square().unwrap_or("");
    // Use config.base_url for endpoint links
    let printer = MarkdownPrinter::new(config.base_url.clone());
    let board_md = printer.print(fen, valid_moves, selected);
    // Update GitHub README
    let _ = github_service.update_readme(&board_md).await;
    HttpResponse::Ok().body(board_md)
}

/// Start a new game from the initial position.
pub async fn new_game(
    chess_service: web::Data<Arc<Mutex<ChessService>>>,
    github_service: web::Data<Arc<GithubService>>,
    config: web::Data<Config>,
) -> impl Responder {
    let mut service = chess_service.lock().await;
    if let Err(e) = service.new_game().await {
        return HttpResponse::InternalServerError().body(format!("New game error: {}", e));
    }
    let fen = match service.get_fen().await {
        Ok(f) => f,
        Err(e) => return HttpResponse::InternalServerError().body(format!("FEN error: {}", e)),
    };
    let valid_moves = match service.get_valid_moves().await {
        Ok(m) => m,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Moves error: {}", e)),
    };
    // Use config.base_url for endpoint links
    let printer = MarkdownPrinter::new(config.base_url.clone());
    let board_md = printer.print(fen, valid_moves, "");
    // Update GitHub README
    let _ = github_service.update_readme(&board_md).await;
    HttpResponse::Ok().body(board_md)
}
