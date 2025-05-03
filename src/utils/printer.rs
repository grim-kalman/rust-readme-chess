use std::collections::HashSet;

/// MarkdownPrinter renders the chess board and controls as Markdown for the README.
pub struct MarkdownPrinter {
    base_url: String,
    owner_repo: String,
}

impl MarkdownPrinter {
    /// Create a new MarkdownPrinter with the given base URL and owner/repo.
    pub fn new(base_url: String, owner_repo: String) -> Self {
        MarkdownPrinter { base_url, owner_repo }
    }

    /// Render the full Markdown output (header, board, footer).
    pub fn print(&self, fen: String, valid_moves: Vec<String>, selected: &str) -> String {
        // Borrow inputs for internal use
        let fen_str = fen.as_str();
        let moves_slice = valid_moves.as_slice();

        let mut out = String::with_capacity(2_048);
        out.push_str(HEADER);
        out.push_str(&self.render_board(fen_str, moves_slice, selected));
        out.push_str(&self.footer());
        out
    }

    /// Build just the board section.
    fn render_board(&self, fen: &str, valid_moves: &[String], selected: &str) -> String {
        let board = parse_fen(fen);
        let valid: HashSet<&str> = valid_moves.iter().map(String::as_str).collect();

        let mut s = String::with_capacity(1_024);
        s.push_str(BOARD_HEADER);
        for (row_idx, row) in board.iter().enumerate() {
            let rank = 8 - row_idx;
            s.push_str(&format!("|  **{}**  |", rank));
            for (file_idx, &square) in row.iter().enumerate() {
                let file = (b'a' + file_idx as u8) as char;
                let pos = format!("{}{}", file, rank);
                s.push_str(&format!(
                    "  {}  |",
                    self.render_square(square, &pos, selected, &valid)
                ));
            }
            s.push('\n');
        }
        s
    }

    /// Build the footer section with a dynamic New Game link.
    fn footer(&self) -> String {
        format!(
            "\n[![New Game](https://img.shields.io/badge/New_Game-4CAF50)]({}/new)",
            self.base_url
        )
    }

    /// Decide how to render a single square (piece, empty, selectable, move target, etc).
    fn render_square(
        &self,
        square: Option<char>,
        pos: &str,
        selected: &str,
        valid: &HashSet<&str>,
    ) -> String {
        // URL builders using self.base_url
        let select_url = |p: &str| format!("{}/select?square={}", self.base_url, p);
        let play_url = |mv: &str| format!("{}/play?mv={}", self.base_url, mv);

        let owner_repo = &self.owner_repo;

        match square {
            Some(piece) => {
                let piece_md = format_piece(piece);
                let is_white = piece.is_uppercase();

                if !selected.is_empty() {
                    // If this is the selected piece -> keep it selected
                    if pos == selected {
                        return md_link(&piece_md, &select_url(pos));
                    }
                    // If this is a valid move destination -> show move link
                    let mv = format!("{}{}", selected, pos);
                    if valid.contains(mv.as_str()) {
                        return md_link("_", &play_url(&mv));
                    }
                    // Otherwise, allow re-selecting another white piece that has moves
                    if is_white && valid.iter().any(|m| m.starts_with(pos)) {
                        return md_link(&piece_md, &select_url(pos));
                    }
                    // Else, default render
                    return if is_white {
                        md_link(&piece_md, &get_profile_url(owner_repo))
                    } else {
                        piece_md
                    };
                }

                // No piece selected: allow selecting white pieces that have moves
                if is_white && valid.iter().any(|m| m.starts_with(pos)) {
                    return md_link(&piece_md, &select_url(pos));
                }
                // Otherwise, white pieces link to profile, blacks just render
                if is_white {
                    md_link(&piece_md, &get_profile_url(owner_repo))
                } else {
                    piece_md
                }
            }
            None => {
                // Empty square: if a piece is selected and this is a valid target
                if !selected.is_empty() {
                    let mv = format!("{}{}", selected, pos);
                    if valid.contains(mv.as_str()) {
                        return md_link("_", &play_url(&mv));
                    }
                }
                " ".into()
            }
        }
    }
}

/// Parse FEN into 8×8 board array.
fn parse_fen(fen: &str) -> Vec<[Option<char>; 8]> {
    let mut rows = Vec::with_capacity(8);
    let ranks = fen.split_whitespace().next().unwrap();
    for rank_str in ranks.split('/') {
        let mut row = [None; 8];
        let mut file = 0;
        for c in rank_str.chars() {
            if let Some(n) = c.to_digit(10) {
                file += n as usize;
            } else {
                row[file] = Some(c);
                file += 1;
            }
        }
        rows.push(row);
    }
    rows
}

/// Format a piece: bold for white, italic for black.
fn format_piece(piece: char) -> String {
    if piece.is_uppercase() {
        format!("**{}**", piece)
    } else {
        format!("_{}_", piece)
    }
}

/// Markdown link helper.
fn md_link(text: &str, url: &str) -> String {
    format!("[{}]({})", text, url)
}

/// Get the GitHub profile URL for the owner/repo.
fn get_profile_url(owner_repo: &str) -> String {
    format!("https://github.com/{}", owner_repo)
}

//——— constants ———//
const HEADER: &str = r#"# Readme Chess

Welcome to my GitHub profile! Here, you can play a game of chess with me, using my [readme-chess](https://github.com/grim-kalman/rust-readme-chess) application.

## How to Play

- Click on any selectable piece ([**A**]()) to select it.
- Click on any destination square ([**_**]()) to move the selected piece.
- After each action, please wait for the page to refresh to see the updated game state.

## Chess Board
"#;

const BOARD_HEADER: &str = "|     |  a  |  b  |  c  |  d  |  e  |  f  |  g  |  h  |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
";