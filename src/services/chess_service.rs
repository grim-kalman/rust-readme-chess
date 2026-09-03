use crate::services::engine_service::EngineService;
use std::error::Error;
use std::fmt;

/// Why a chess action failed: a bad request from the player, or the engine itself
/// breaking (exited, pipe closed) — after which the game state can't be trusted.
#[derive(Debug)]
pub enum ChessError {
    InvalidMove(String),
    Engine(Box<dyn Error + Send + Sync>),
}

impl fmt::Display for ChessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChessError::InvalidMove(mv) => write!(f, "Invalid move: {}", mv),
            ChessError::Engine(e) => write!(f, "Engine error: {}", e),
        }
    }
}

impl From<Box<dyn Error + Send + Sync>> for ChessError {
    fn from(e: Box<dyn Error + Send + Sync>) -> Self {
        ChessError::Engine(e)
    }
}

/// The player is always white; the engine always black.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    Playing,
    PlayerWon,
    EngineWon,
    Draw,
}

/// Snapshot of the game as the printer needs it.
#[derive(Debug, Clone)]
pub struct Board {
    pub fen: String,
    pub valid_moves: Vec<String>,
    pub selected: String,
    pub status: GameStatus,
}

impl Board {
    fn status(fen: &str, valid_moves: &[String], in_check: bool) -> GameStatus {
        if !valid_moves.is_empty() {
            return GameStatus::Playing;
        }
        let player_to_move = fen.split_whitespace().nth(1) == Some("w");
        match (in_check, player_to_move) {
            (false, _) => GameStatus::Draw,
            (true, true) => GameStatus::EngineWon,
            (true, false) => GameStatus::PlayerWon,
        }
    }
}

/// Service for managing chess game state and player/engine moves.
pub struct ChessService {
    engine: EngineService,
    selected_square: Option<String>,
}

impl ChessService {
    /// Create a new ChessService with the given engine.
    pub fn new(engine: EngineService) -> Self {
        Self {
            engine,
            selected_square: None,
        }
    }

    /// Play a move as the player, then let the engine reply.
    pub async fn play(&mut self, mv: &str) -> Result<(), ChessError> {
        let valid_moves = self.engine.get_valid_moves().await?;
        if !valid_moves.contains(&mv.to_string()) {
            return Err(ChessError::InvalidMove(mv.to_string()));
        }
        self.engine.make_move(mv).await?;
        if let Some(engine_move) = self.engine.best_move().await? {
            self.engine.make_move(&engine_move).await?;
        }
        self.selected_square = None;
        Ok(())
    }

    /// Toggle selection of a square (for piece selection UI).
    pub fn select(&mut self, square: &str) {
        if self.selected_square.as_deref() == Some(square) {
            self.selected_square = None;
        } else {
            self.selected_square = Some(square.to_string());
        }
    }

    /// Start a new game (reset engine and selection).
    pub async fn new_game(&mut self) -> Result<(), ChessError> {
        self.engine.new_game().await?;
        self.selected_square = None;
        Ok(())
    }

    /// Current position, legal moves, selection and outcome, as the printer needs them.
    pub async fn board(&mut self) -> Result<Board, ChessError> {
        let position = self.engine.get_position().await?;
        let valid_moves = self.engine.get_valid_moves().await?;
        let status = Board::status(&position.fen, &valid_moves, position.in_check);
        Ok(Board {
            fen: position.fen,
            valid_moves,
            selected: self.selected_square.clone().unwrap_or_default(),
            status,
        })
    }
}
