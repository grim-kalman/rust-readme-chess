/// Responsible for converting board state into Markdown.
/// Converts a FEN string into a Markdown chess board for README.
pub struct MarkdownPrinter;

impl MarkdownPrinter {
    pub fn new() -> Self {
        MarkdownPrinter
    }

    /// Render the given FEN into a Markdown board matching the Java implementation.
    pub fn print(&self, fen: &str) -> String {
        let board = Self::parse_board(fen);
        let mut md = String::new();
        md.push_str(&Self::intro());
        md.push_str(&Self::header());
        md.push_str(&Self::rows(&board));
        md.push_str(&Self::new_game_button());
        md
    }
    fn parse_board(fen: &str) -> Vec<Vec<Option<char>>> {
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

    fn rows(board: &Vec<Vec<Option<char>>>) -> String {
        let profile_url = "https://github.com/grim-kalman";
        let select_base = "https://readmechess.azurewebsites.net/select?square=";
        let mut s = String::new();
        for (i, row) in board.iter().enumerate() {
            let rank = 8 - i;
            s.push_str(&format!("|  **{}**  |", rank));
            for (j, cell) in row.iter().enumerate() {
                let file = (b'a' + j as u8) as char;
                let cell_str = match cell {
                    Some(piece) if piece.is_uppercase() => {
                        let pos = format!("{}{}", file, rank);
                        let symbol = format!("**{}**", piece);
                        let is_selectable = (*piece == 'P' && rank == 2)
                            || (*piece == 'N' && rank == 1 && (file == 'b' || file == 'g'));
                        if is_selectable {
                            format!("[{}]({}{})", symbol, select_base, pos)
                        } else {
                            format!("[{}]({})", symbol, profile_url)
                        }
                    }
                    Some(piece) => format!("_{}_", piece),
                    None => " ".to_string(),
                };
                s.push_str(&format!("  {}  |", cell_str));
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
