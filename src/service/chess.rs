use crate::model::board::Board;
use crate::utils::printer::{FenBoardPrinter, MarkdownBoardPrinter};
use crate::service::engine::EngineService;
use crate::service::github::GithubService;

/// High-level service coordinating play, selection, and game reset.
pub struct ChessService {
    board: Board,
    engine: EngineService,
    github: GithubService,
}

impl ChessService {
    /// Constructs a new ChessService from its dependencies.
    pub fn new(board: Board, engine: EngineService, github: GithubService) -> Self {
        // TODO: implement
        unimplemented!()
    }

    /// Starts the service (e.g., engine, initial README update).
    pub fn init(&mut self) -> Result<(), String> {
        // TODO: implement
        unimplemented!()
    }

    /// Plays a move from the user and engine.
    pub fn play(&mut self, mv: &str) -> Result<(), String> {
        // TODO: implement
        unimplemented!()
    }

    /// Selects a square.
    pub fn select(&mut self, square: &str) -> Result<(), String> {
        // TODO: implement
        unimplemented!()
    }

    /// Starts a new game.
    pub fn new_game(&mut self) -> Result<(), String> {
        // TODO: implement
        unimplemented!()
    }

    /// Renders the board as Markdown.
    pub fn print_board(&self) -> String {
        // TODO: implement
        unimplemented!()
    }
}
