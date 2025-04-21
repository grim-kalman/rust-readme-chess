use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PlayQuery {
    /// `move` is a reserved keyword, so we use `mv`.
    pub mv: String,
}

/// Apply the player's move and the engine's reply.
#[get("/play")]
async fn play(query: web::Query<PlayQuery>) -> impl Responder {
    // TODO: invoke ChessService.play
    HttpResponse::NotImplemented().finish()
}

#[derive(Deserialize)]
pub struct SelectQuery {
    pub square: String,
}

/// Highlight or deselect a square.
#[get("/select")]
async fn select(query: web::Query<SelectQuery>) -> impl Responder {
    // TODO: invoke ChessService.select
    HttpResponse::NotImplemented().finish()
}

/// Start a new game from the initial position.
#[get("/new")]
async fn new_game() -> impl Responder {
    // TODO: invoke ChessService.new_game
    HttpResponse::NotImplemented().finish()
}
