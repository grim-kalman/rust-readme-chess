use crate::config::Config;
use crate::services::game_actor::{Command, GameError, GameHandle};
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

// Redirects the user to the GitHub profile with a nanosecond cachebuster to force refresh.
fn redirect_to_github(config: &Config) -> HttpResponse {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let redirect_url = format!(
        "https://github.com/{}?cb={}",
        config.github_owner_repo, nanos
    );
    HttpResponse::SeeOther()
        .append_header(("Location", redirect_url))
        .finish()
}

fn error_response(e: GameError) -> HttpResponse {
    match e {
        GameError::InvalidMove(_) => HttpResponse::BadRequest().body(e.to_string()),
        GameError::Busy | GameError::Unavailable(_) => {
            HttpResponse::ServiceUnavailable().body(e.to_string())
        }
        GameError::Timeout => HttpResponse::GatewayTimeout().body(e.to_string()),
    }
}

// Runs one game command (the game task also publishes the new board), then redirects.
async fn act_and_redirect(cmd: Command, game: &GameHandle, config: &Config) -> HttpResponse {
    match game.send(cmd).await {
        Ok(()) => redirect_to_github(config),
        Err(e) => error_response(e),
    }
}

#[derive(Deserialize)]
/// Query for /play endpoint. `move` is reserved, so we use `mv`.
pub struct PlayQuery {
    pub mv: String,
}

// Handles a play (move) request.
pub async fn play(
    query: web::Query<PlayQuery>,
    game: web::Data<GameHandle>,
    config: web::Data<Config>,
) -> impl Responder {
    act_and_redirect(Command::Play(query.mv.clone()), &game, &config).await
}

#[derive(Deserialize)]
/// Query for /select endpoint.
pub struct SelectQuery {
    pub square: String,
}

// Handles a select (piece selection) request.
pub async fn select(
    query: web::Query<SelectQuery>,
    game: web::Data<GameHandle>,
    config: web::Data<Config>,
) -> impl Responder {
    act_and_redirect(Command::Select(query.square.clone()), &game, &config).await
}

// Handles a new game request.
pub async fn new_game(game: web::Data<GameHandle>, config: web::Data<Config>) -> impl Responder {
    act_and_redirect(Command::NewGame, &game, &config).await
}

// Health check: proves the engine answers, not just that the HTTP thread is alive.
pub async fn health(game: web::Data<GameHandle>) -> impl Responder {
    match game.send(Command::Ping).await {
        Ok(()) => HttpResponse::Ok().body("OK"),
        Err(e) => error_response(e),
    }
}
