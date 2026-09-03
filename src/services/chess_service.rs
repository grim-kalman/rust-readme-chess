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

/// Snapshot of the game as the printer needs it.
#[derive(Debug, Clone)]
pub struct Board {
    pub fen: String,
    pub valid_moves: Vec<String>,
    pub selected: String,
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
        let engine_move = self.engine.best_move().await?;
        if !engine_move.is_empty() {
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

    /// Current FEN, legal moves and selection in one round-trip to the engine.
    pub async fn board(&mut self) -> Result<Board, ChessError> {
        Ok(Board {
            fen: self.engine.get_position().await?,
            valid_moves: self.engine.get_valid_moves().await?,
            selected: self.selected_square.clone().unwrap_or_default(),
        })
    }

    /// Get the current FEN string for the board.
    pub async fn get_fen(&mut self) -> Result<String, ChessError> {
        Ok(self.engine.get_position().await?)
    }

    /// Get the list of valid moves in the current position.
    pub async fn get_valid_moves(&mut self) -> Result<Vec<String>, ChessError> {
        Ok(self.engine.get_valid_moves().await?)
    }

    /// Get the currently selected square, if any.
    pub fn get_selected_square(&self) -> Option<&str> {
        self.selected_square.as_deref()
    }
}
