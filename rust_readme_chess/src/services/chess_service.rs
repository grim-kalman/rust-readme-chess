use crate::utils::printer::MarkdownPrinter;
use crate::services::engine_service::EngineService;
use tokio::sync::Mutex;
use std::error::Error;

/// Coordinates moves between the Stockfish engine and the Markdown printer.
pub struct ChessService {
    engine: Mutex<EngineService>,
    printer: MarkdownPrinter,
    moves: Mutex<Vec<String>>,
    selected: Mutex<Option<String>>,
}




impl ChessService {
    /// Create a new ChessService wrapping a started EngineService.
    pub fn new(engine: EngineService) -> Self {
        ChessService {
            engine: Mutex::new(engine),
            printer: MarkdownPrinter::new(),
            moves: Mutex::new(Vec::new()),
            selected: Mutex::new(None),
        }
    }




    /// Start a new game: clear history and selection, reset engine.
    pub async fn new_game(&self) -> Result<(), Box<dyn Error>> {
        let mut eng = self.engine.lock().await;
        let mut mvlist = self.moves.lock().await;
        let mut sel = self.selected.lock().await;
        mvlist.clear();
        *sel = None;
        eng.new_game().await?;
        Ok(())
    }




    /// Toggle selection of a square (e.g. "e2").
    pub async fn select(&self, square: &str) -> Result<(), Box<dyn Error>> {
        let mut sel = self.selected.lock().await;
        if sel.as_deref() == Some(square) {
            *sel = None;
        } else {
            *sel = Some(square.to_string());
        }
        Ok(())
    }




    /// Play a move (user move + engine reply), then clear selection.
    pub async fn play(&self, mv: &str) -> Result<(), Box<dyn Error>> {
        // validate against engine's legal moves
        {
            let mut eng = self.engine.lock().await;
            let valid = eng.get_valid_moves().await?;
            if !valid.contains(&mv.to_string()) {
                return Err(format!("Invalid move: {}", mv).into());
            }
            // apply user move
            eng.make_move(mv).await?;
            // get engine reply
            let reply = eng.best_move().await?;
            // record both moves
            let mut mvlist = self.moves.lock().await;
            mvlist.push(mv.to_string());
            mvlist.push(reply.clone());
            // apply engine move
            eng.make_move(&reply).await?;
        }
        // clear selection
        *self.selected.lock().await = None;
        Ok(())
    }




    /// Render the current board as Markdown (with selection highlighting).
    pub async fn print_board(&self) -> Result<String, Box<dyn Error>> {
        let mut eng = self.engine.lock().await;
        let fen = eng.get_position().await?;
        let sel = self.selected.lock().await.clone();
        let md = if let Some(ref s) = sel {
            self.printer.print_with_selection(&fen, s)
        } else {
            self.printer.print(&fen)
        };
        Ok(md)
    }




    /// Get move history (for testing).
    pub async fn get_moves(&self) -> Vec<String> {
        self.moves.lock().await.clone()
    }



    
    /// Get current selection (for testing).
    pub async fn get_selected(&self) -> Option<String> {
        self.selected.lock().await.clone()
    }
}