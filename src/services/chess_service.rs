use crate::services::engine_service::EngineService;

pub struct ChessService {
    engine: EngineService,
    selected_square: Option<String>,
}

impl ChessService {
    pub fn new(engine: EngineService) -> Self {
        Self {
            engine,
            selected_square: None,
        }
    }

    pub async fn play(&mut self, mv: &str) -> Result<(), String> {
        // Validate move
        let valid_moves = self
            .engine
            .get_valid_moves()
            .await
            .map_err(|e| e.to_string())?;
        if !valid_moves.contains(&mv.to_string()) {
            return Err(format!("Invalid move: {}", mv));
        }
        // Player move
        self.engine.make_move(mv).await.map_err(|e| e.to_string())?;
        // Engine reply
        let engine_move = self.engine.best_move().await.map_err(|e| e.to_string())?;
        if !engine_move.is_empty() {
            self.engine
                .make_move(&engine_move)
                .await
                .map_err(|e| e.to_string())?;
        }
        // Clear selection after move
        self.selected_square = None;
        Ok(())
    }

    pub async fn select(&mut self, square: &str) -> Result<(), String> {
        // Toggle selection
        if self.selected_square.as_deref() == Some(square) {
            self.selected_square = None;
        } else {
            self.selected_square = Some(square.to_string());
        }
        Ok(())
    }

    pub async fn new_game(&mut self) -> Result<(), String> {
        self.engine.new_game().await.map_err(|e| e.to_string())?;
        self.selected_square = None;
        Ok(())
    }

    pub async fn get_fen(&mut self) -> Result<String, String> {
        self.engine.get_position().await.map_err(|e| e.to_string())
    }

    pub async fn get_valid_moves(&mut self) -> Result<Vec<String>, String> {
        self.engine
            .get_valid_moves()
            .await
            .map_err(|e| e.to_string())
    }

    pub fn get_selected_square(&self) -> Option<&str> {
        self.selected_square.as_deref()
    }
}
