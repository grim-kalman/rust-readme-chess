/// Handles HTTP-style endpoints for play, select, and new game.
pub struct Controller {
    // dependencies (config, chess service)
}

impl Controller {
    /// Constructs a new Controller.
    pub fn new(/* config: AppConfig, chess_service: ChessService */) -> Self {
        // TODO: implement
        unimplemented!()
    }

    /// Handles a /play request.
    pub fn play(&self, mv: &str) -> Result<String, String> {
        // TODO: implement
        unimplemented!()
    }

    /// Handles a /select request.
    pub fn select(&self, square: &str) -> Result<String, String> {
        // TODO: implement
        unimplemented!()
    }

    /// Handles a /new request.
    pub fn new_game(&self) -> Result<String, String> {
        // TODO: implement
        unimplemented!()
    }
}
