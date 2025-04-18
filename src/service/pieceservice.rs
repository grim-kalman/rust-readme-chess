use std::collections::HashMap;
use crate::model::pieces::{Piece, Pawn, Rook, Knight, Bishop, Queen, King};
use crate::utils::constants::{WHITE, BLACK};

/// Service responsible for setting up and resetting chess pieces.
pub struct PieceService {
    pieces: HashMap<String, Box<dyn Piece>>,
}

impl PieceService {
    /// Constructs a new PieceService.
    pub fn new() -> Self {
        // Initialize with starting pieces
        let mut service = PieceService { pieces: HashMap::new() };
        service.reset();
        service
    }

    /// Resets the internal piece map to the starting position.
    pub fn reset(&mut self) {
        // Rebuild the pieces map to starting position
        self.pieces = self.setup_pieces();
    }

    /// Returns the current piece map.
    pub fn pieces(&self) -> &HashMap<String, Box<dyn Piece>> {
        &self.pieces
    }

    /// Builds a map of starting pieces.
    pub fn setup_pieces(&self) -> HashMap<String, Box<dyn Piece>> {
        let mut m: HashMap<String, Box<dyn Piece>> = HashMap::new();
        // Pawns
        for col in b'a'..=b'h' {
            let file = (col as char).to_string();
            m.insert(format!("{}2", file), Box::new(Pawn::new(WHITE)));
            m.insert(format!("{}7", file), Box::new(Pawn::new(BLACK)));
        }
        // Rooks
        m.insert("a1".to_string(), Box::new(Rook::new(WHITE)));
        m.insert("h1".to_string(), Box::new(Rook::new(WHITE)));
        m.insert("a8".to_string(), Box::new(Rook::new(BLACK)));
        m.insert("h8".to_string(), Box::new(Rook::new(BLACK)));
        // Knights
        m.insert("b1".to_string(), Box::new(Knight::new(WHITE)));
        m.insert("g1".to_string(), Box::new(Knight::new(WHITE)));
        m.insert("b8".to_string(), Box::new(Knight::new(BLACK)));
        m.insert("g8".to_string(), Box::new(Knight::new(BLACK)));
        // Bishops
        m.insert("c1".to_string(), Box::new(Bishop::new(WHITE)));
        m.insert("f1".to_string(), Box::new(Bishop::new(WHITE)));
        m.insert("c8".to_string(), Box::new(Bishop::new(BLACK)));
        m.insert("f8".to_string(), Box::new(Bishop::new(BLACK)));
        // Queens and Kings
        m.insert("d1".to_string(), Box::new(Queen::new(WHITE)));
        m.insert("e1".to_string(), Box::new(King::new(WHITE)));
        m.insert("d8".to_string(), Box::new(Queen::new(BLACK)));
        m.insert("e8".to_string(), Box::new(King::new(BLACK)));
        m
    }
}