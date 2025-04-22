/// Responsible for converting board state into Markdown.
/// Converts a FEN string into a Markdown chess board for README.
use crate::utils::validator::MoveValidator;
pub struct MarkdownPrinter;

impl MarkdownPrinter {
    pub fn new() -> Self {
        MarkdownPrinter
    }

    /// Render the given FEN into a Markdown board with no selection.
    pub fn print(&self, fen: &str) -> String {
        let board = Self::parse_board(fen);
        let validator = MoveValidator::from_board(&board);
        let mut md = String::new();
        md.push_str(&Self::intro());
        md.push_str(&Self::header());
        md.push_str(&self.rows(&board, &validator, None));
        md.push_str(&Self::new_game_button());
        md
    }
    
    /// Render the given FEN into a Markdown board with a selected square.
    pub fn print_with_selection(&self, fen: &str, selected: &str) -> String {
        let board = Self::parse_board(fen);
        let validator = MoveValidator::from_board(&board);
        let mut md = String::new();
        md.push_str(&Self::intro());
        md.push_str(&Self::header());
        md.push_str(&self.rows(&board, &validator, Some(selected)));
        md.push_str(&Self::new_game_button());
        md
    }
    /// Parse a FEN string into an 8×8 board representation of Option<char>.
    pub fn parse_board(fen: &str) -> Vec<Vec<Option<char>>> {
        let board_part = fen.split_whitespace().next().unwrap_or("");
        board_part
            .split('/')
            .map(|rank_fen| {
                let mut row = Vec::new();
                for ch in rank_fen.chars() {
                    if let Some(count) = ch.to_digit(10) {
                        for _ in 0..count {
                            row.push(None);
                        }
                    } else {
                        row.push(Some(ch));
                    }
                }
                row
            })
            .collect()
    }

    fn intro() -> String {
        let mut s = String::new();
        s.push_str("# Readme Chess\n\n");
        s.push_str("Welcome to my GitHub profile! Here, you can play a game of chess with me, using my [readme-chess](https://github.com/grim-kalman/readme-chess) application.\n\n");
        s.push_str("## How to Play\n\n");
        s.push_str("- Click on any selectable piece ([**A**]()) to select it.\n");
        s.push_str("- Click on any destination square ([**_**]()) to move the selected piece.\n");
        s.push_str("- After each action, please wait for the page to refresh to see the updated game state.\n\n");
        s.push_str("## Chess Board\n");
        s
    }

    fn header() -> String {
        let mut s = String::new();
        s.push_str("|     |  a  |  b  |  c  |  d  |  e  |  f  |  g  |  h  |\n");
        s.push_str("|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|\n");
        s
    }

    fn rows(
        &self,
        board: &Vec<Vec<Option<char>>>,
        validator: &MoveValidator,
        selected: Option<&str>,
    ) -> String {
        let profile_url = "https://github.com/grim-kalman";
        let select_base = "https://readmechess.azurewebsites.net/select?square=";
        let play_base = "https://readmechess.azurewebsites.net/play?move=";
        let mut s = String::new();
        for (i, row) in board.iter().enumerate() {
            let rank = 8 - i;
            s.push_str(&format!("|  **{}**  |", rank));
            for (j, cell) in row.iter().enumerate() {
                let file = (b'a' + j as u8) as char;
                let pos = format!("{}{}", file, rank);
                // Base symbol for the square
                let square_symbol = match cell {
                    None => "_".to_string(),
                    Some(piece) if piece.is_lowercase() => format!("_{}_", piece),
                    Some(piece) => format!("**{}**", piece),
                };
                // Determine the Markdown for this cell
                let cell_md = if let Some(sel) = selected {
                    // Play link for valid moves from selection
                    let mv = format!("{}{}", sel, pos);
                    if validator.is_valid(&mv) {
                        format!("[_]({}{})", play_base, mv)
                    } else if let Some(piece) = cell {
                        // Select link for other pieces
                        if piece.is_uppercase() && validator.is_start_of_valid_move(&pos) {
                            format!("[{}]({}{})", square_symbol, select_base, pos)
                        } else if piece.is_uppercase() {
                            format!("[{}]({})", square_symbol, profile_url)
                        } else {
                            square_symbol
                        }
                    } else {
                        " ".to_string()
                    }
                } else {
                    // No selection: only select links for piece starts
                    if let Some(piece) = cell {
                        if piece.is_uppercase() && validator.is_start_of_valid_move(&pos) {
                            format!("[{}]({}{})", square_symbol, select_base, pos)
                        } else if piece.is_uppercase() {
                            format!("[{}]({})", square_symbol, profile_url)
                        } else {
                            square_symbol
                        }
                    } else {
                        " ".to_string()
                    }
                };
                s.push_str(&format!("  {}  |", cell_md));
            }
            s.push('\n');
        }
        s
    }

    fn new_game_button() -> String {
        let new_game = "https://readmechess.azurewebsites.net/new";
        format!("\n[![New Game](https://img.shields.io/badge/New_Game-4CAF50)]({})\n", new_game)
    }
}
