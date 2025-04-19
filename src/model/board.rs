use crate::model::pieces::Piece;
use crate::service::pieceservice::PieceService;
use crate::utils::validator::MoveValidator;
use crate::utils::constants::{WHITE, BLACK};

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
        let mut s = BoardState {
            active_color: String::new(),
            castling_availability: String::new(),
            en_passant_target: String::new(),
            half_move_clock: 0,
            full_move_number: 0,
        };
        s.reset();
        s
    }

    /// Resets to the starting board state.
    pub fn reset(&mut self) {
        // Initialize to starting position: white to move, full castling rights, no en passant, clocks reset
        self.active_color = WHITE.to_string();
        self.castling_availability = "KQkq".to_string();
        self.en_passant_target = "-".to_string();
        self.half_move_clock = 0;
        self.full_move_number = 1;
    }

    /// Updates the state after making a move.
    pub fn update(&mut self, from_square: &str, capture: bool) {
        // Flip active color and possibly advance full move counter
        self.update_active_color_and_full_move_number();
        // Clear en passant target after any move
        self.reset_en_passant_target();
        // Adjust castling rights if king or rook moved
        self.update_castling_rights(from_square);
        // Update half-move clock based on capture
        self.update_half_move_clock(capture);
    }

    /// Sets the en passant target based on a pawn's two-square move.
    pub fn handle_en_passant_target(&mut self, to_square: &str) {
        // Set en passant target one rank behind the pawn's two-square move
        let file = to_square.chars().next().unwrap();
        let rank = if self.active_color == BLACK { '3' } else { '6' };
        self.en_passant_target = format!("{}{}", file, rank);
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
    
    // -- internal helpers matching Java BoardState logic --
    fn update_active_color_and_full_move_number(&mut self) {
        if self.active_color == BLACK {
            self.active_color = WHITE.to_string();
        } else {
            self.active_color = BLACK.to_string();
            self.full_move_number += 1;
        }
    }

    fn reset_en_passant_target(&mut self) {
        self.en_passant_target = "-".to_string();
    }

    fn update_castling_rights(&mut self, from_square: &str) {
        let file: char = from_square.chars().nth(0);
        let rank: char = from_square.chars().nth(1);
        
        if matches!(file, 'e' | 'a' | 'h') && matches!(rank, '1' | '8') {
            self.castling_availability = self.castling_availability
                .replace(if file == 'a' { "Q" } else { "K" }, "")
                .replace(if rank == '1' { "Q" } else { "q" }, "");
        }
    }

    fn update_half_move_clock(&mut self, capture: bool) {
        self.half_move_clock = if capture { 0 } else { self.half_move_clock + 1 };
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
