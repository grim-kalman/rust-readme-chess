use crate::services::engine_service::EngineService;

/// MoveValidator holds the current legal moves as reported by the engine,
/// and can check if a full move is legal or if a square is the start of any legal move.
pub struct MoveValidator {
    valid_moves: Vec<String>,
}

impl MoveValidator {
    /// Creates a new MoveValidator by querying the engine for perft(1) legal moves.
    pub async fn new(engine: &mut EngineService) -> Result<Self, Box<dyn std::error::Error>> {
        let moves = engine.get_valid_moves().await?;
        Ok(MoveValidator { valid_moves: moves })
    }

    /// Returns true if the given UCI move (e.g. "e2e4") is legal in the current position.
    pub fn is_valid(&self, mv: &str) -> bool {
        self.valid_moves.contains(&mv.to_string())
    }

    /// Returns true if any legal move starts from the given square (e.g. "e2").
    pub fn is_start_of_valid_move(&self, pos: &str) -> bool {
        self.valid_moves.iter().any(|m| m.starts_with(pos))
    }
    
    /// Creates a new MoveValidator from a board representation.
    /// Currently only generates moves for white Pawns and Knights.
    pub fn from_board(board: &Vec<Vec<Option<char>>>) -> Self {
        let mut valid_moves = Vec::new();
        for (row_idx, row) in board.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if let Some(piece) = cell {
                    if piece.is_uppercase() {
                        let file = (b'a' + col_idx as u8) as char;
                        let rank = (8 - row_idx) as u8;
                        let from = format!("{}{}", file, rank);
                        match piece {
                            'P' => {
                                // one-step
                                if row_idx >= 1 && board[row_idx - 1][col_idx].is_none() {
                                    let to_rank = rank + 1;
                                    let to = format!("{}{}", file, to_rank);
                                    valid_moves.push(format!("{}{}", from, to));
                                    // two-step from starting rank 2
                                    if row_idx == 6 && board[row_idx - 2][col_idx].is_none() {
                                        let to2 = format!("{}{}", file, rank + 2);
                                        valid_moves.push(format!("{}{}", from, to2));
                                    }
                                }
                            }
                            'N' => {
                                const OFFSETS: &[(i8, i8)] = &[
                                    (2, 1), (2, -1), (-2, 1), (-2, -1),
                                    (1, 2), (1, -2), (-1, 2), (-1, -2),
                                ];
                                for &(dr, dc) in OFFSETS {
                                    let r = row_idx as i8 - dr;
                                    let c = col_idx as i8 + dc;
                                    if (0..8).contains(&r) && (0..8).contains(&c) {
                                        let to_file = (b'a' + c as u8) as char;
                                        let to_rank = (8 - r as usize) as u8;
                                        let to = format!("{}{}", to_file, to_rank);
                                        valid_moves.push(format!("{}{}", from, to));
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        MoveValidator { valid_moves }
    }
}
