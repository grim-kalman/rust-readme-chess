use crate::config::Config;
use crate::services::chess_service::ChessService;
use crate::services::github_service::GithubService;
use crate::utils::printer::MarkdownPrinter;
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

// Redirects the user to the GitHub profile with a nanosecond cachebuster to force refresh.
fn redirect_to_github(config: &Config) -> actix_web::HttpResponse {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let redirect_url = format!(
        "https://github.com/{}?cb={}",
        config.github_owner_repo, nanos
    );
    actix_web::HttpResponse::SeeOther()
        .append_header(("Location", redirect_url))
        .finish()
}

// Updates the README on GitHub, polls until updated, and redirects the user.
async fn update_and_redirect(
    board_md: String,
    github_service: &Arc<GithubService>,
    config: &Config,
) -> actix_web::HttpResponse {
    let _ = github_service.update_readme(&board_md).await;
    let _ = github_service.poll_readme_until_updated(&board_md, 10).await;
    redirect_to_github(config)
}

// Helper to get the current board state (FEN, valid moves, selected square)
async fn get_board_state(
    service: &mut ChessService,
) -> Result<(String, Vec<String>, String), actix_web::HttpResponse> {
    let fen = service.get_fen().await.map_err(|e| {
        actix_web::HttpResponse::InternalServerError().body(format!("FEN error: {}", e))
    })?;
    let valid_moves = service.get_valid_moves().await.map_err(|e| {
        actix_web::HttpResponse::InternalServerError().body(format!("Moves error: {}", e))
    })?;
    let selected = service.get_selected_square().unwrap_or("").to_string();
    Ok((fen, valid_moves, selected))
}

#[derive(Deserialize)]
/// Query for /play endpoint. `move` is reserved, so we use `mv`.
pub struct PlayQuery {
    pub mv: String,
}

// Handles a play (move) request.
pub async fn play(
    query: web::Query<PlayQuery>,
    chess_service: web::Data<Arc<Mutex<ChessService>>>,
    github_service: web::Data<Arc<GithubService>>,
    config: web::Data<Config>,
) -> impl Responder {
    let mut service = chess_service.lock().unwrap();
    if let Err(e) = service.play(&query.mv).await {
        return HttpResponse::BadRequest().body(format!("Invalid move: {}", e));
    }
    let (fen, valid_moves, _) = match get_board_state(&mut service).await {
        Ok(data) => data,
        Err(resp) => return resp,
    };
    let printer = MarkdownPrinter::new(config.base_url.clone(), config.github_owner_repo.clone());
    let board_md = printer.print(fen, valid_moves, "");
    update_and_redirect(board_md, &github_service, &config).await
}

#[derive(Deserialize)]
/// Query for /select endpoint.
pub struct SelectQuery {
    pub square: String,
}

// Handles a select (piece selection) request.
pub async fn select(
    query: web::Query<SelectQuery>,
    chess_service: web::Data<Arc<Mutex<ChessService>>>,
    github_service: web::Data<Arc<GithubService>>,
    config: web::Data<Config>,
) -> impl Responder {
    let mut service = chess_service.lock().unwrap();
    if let Err(e) = service.select(&query.square).await {
        return HttpResponse::BadRequest().body(format!("Select error: {}", e));
    }
    let (fen, valid_moves, selected) = match get_board_state(&mut service).await {
        Ok(data) => data,
        Err(resp) => return resp,
    };
    let printer = MarkdownPrinter::new(config.base_url.clone(), config.github_owner_repo.clone());
    let board_md = printer.print(fen, valid_moves, &selected);
    update_and_redirect(board_md, &github_service, &config).await
}

// Handles a new game request.
pub async fn new_game(
    chess_service: web::Data<Arc<Mutex<ChessService>>>,
    github_service: web::Data<Arc<GithubService>>,
    config: web::Data<Config>,
) -> impl Responder {
    let mut service = chess_service.lock().unwrap();
    if let Err(e) = service.new_game().await {
        return HttpResponse::InternalServerError().body(format!("New game error: {}", e));
    }
    let (fen, valid_moves, _) = match get_board_state(&mut service).await {
        Ok(data) => data,
        Err(resp) => return resp,
    };
    let printer = MarkdownPrinter::new(config.base_url.clone(), config.github_owner_repo.clone());
    let board_md = printer.print(fen, valid_moves, "");
    update_and_redirect(board_md, &github_service, &config).await
}
