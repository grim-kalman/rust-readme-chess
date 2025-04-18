use crate::model::pieces::Piece;
use crate::service::pieceservice::PieceService;
use crate::utils::validator::MoveValidator;

/// Maintains the state of the board (active color, castling rights, en passant target, clocks).
pub struct BoardState {
    active_color: String,
    castling_availability: String,
    en_passant_target: String,
    half_move_clock: usize,
    full_move_number: usize,
}

impl BoardState {
    /// Constructs a new BoardState with default values.
    pub fn new() -> Self {
        // TODO: implement
        unimplemented!()
    }

    /// Resets to the starting board state.
    pub fn reset(&mut self) {
        // TODO: implement
        unimplemented!()
    }

    /// Updates the state after making a move.
    pub fn update(&mut self, from_square: &str, capture: bool) {
        // TODO: implement
        unimplemented!()
    }

    /// Sets the en passant target based on a pawn's two-square move.
    pub fn handle_en_passant_target(&mut self, to_square: &str) {
        // TODO: implement
        unimplemented!()
    }

    /// Getter for active_color.
    pub fn active_color(&self) -> &str {
        &self.active_color
    }

    /// Getter for castling_availability.
    pub fn castling_availability(&self) -> &str {
        &self.castling_availability
    }

    /// Getter for en_passant_target.
    pub fn en_passant_target(&self) -> &str {
        &self.en_passant_target
    }

    /// Getter for half_move_clock.
    pub fn half_move_clock(&self) -> usize {
        self.half_move_clock
    }

    /// Getter for full_move_number.
    pub fn full_move_number(&self) -> usize {
        self.full_move_number
    }
}

/// Represents a chess board and its pieces.
pub struct Board {
    selected_square: Option<String>,
    board_state: BoardState,
    piece_service: PieceService,
    move_validator: MoveValidator,
}

impl Board {
    /// Creates a new Board with fresh state.
    pub fn new() -> Self {
        // TODO: implement
        unimplemented!()
    }

    /// Resets the board and state.
    pub fn reset(&mut self) {
        // TODO: implement
        unimplemented!()
    }

    /// Returns the currently selected square, if any.
    pub fn selected_square(&self) -> Option<&String> {
        self.selected_square.as_ref()
    }

    /// Selects or deselects a square.
    pub fn select_square(&mut self, square: &str) {
        // TODO: implement
        unimplemented!()
    }

    /// Makes a move on the board, updating pieces and state.
    pub fn make_move(&mut self, mv: &str) {
        // TODO: implement
        unimplemented!()
    }

    /// Getter for the underlying board state.
    pub fn state(&self) -> &BoardState {
        &self.board_state
    }

    /// Getter for the piece map.
    pub fn pieces(&self) -> &std::collections::HashMap<String, Box<dyn Piece>> {
        &self.piece_service.pieces()
    }
}
