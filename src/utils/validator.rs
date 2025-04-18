use crate::service::engine::EngineService;

/// Validates chess moves against a list of moves from the engine.
pub struct MoveValidator {
    engine_service: EngineService,
}

impl MoveValidator {
    /// Creates a new MoveValidator wrapping the given engine service.
    pub fn new(engine_service: EngineService) -> Self {
        // TODO: implement
        unimplemented!()
    }

    /// Returns true if the move string exactly matches one of the engine's valid moves.
    pub fn is_valid(&self, mv: &str) -> bool {
        // TODO: implement
        unimplemented!()
    }

    /// Returns true if any valid move starts with the given position prefix.
    pub fn is_start_of_valid_move(&self, position: &str) -> bool {
        // TODO: implement
        unimplemented!()
    }
}
