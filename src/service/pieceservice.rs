use std::collections::HashMap;
use crate::model::pieces::Piece;

/// Service responsible for setting up and resetting chess pieces.
pub struct PieceService {
    pieces: HashMap<String, Box<dyn Piece>>,
}

impl PieceService {
    /// Constructs a new PieceService.
    pub fn new() -> Self {
        // TODO: implement
        unimplemented!()
    }

    /// Resets the internal piece map to the starting position.
    pub fn reset(&mut self) {
        // TODO: implement
        unimplemented!()
    }

    /// Returns the current piece map.
    pub fn pieces(&self) -> &HashMap<String, Box<dyn Piece>> {
        &self.pieces
    }

    /// Builds a map of starting pieces.
    pub fn setup_pieces(&self) -> HashMap<String, Box<dyn Piece>> {
        // TODO: implement
        unimplemented!()
    }
}