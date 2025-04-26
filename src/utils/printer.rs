use std::collections::HashSet;

pub struct MarkdownPrinter {
    base_url: String,
}

impl MarkdownPrinter {
    pub fn new(base_url: String) -> Self {
        MarkdownPrinter { base_url }
    }

    /// Render the full Markdown output.
    pub fn print(&self, fen: String, valid_moves: Vec<String>, selected: &str) -> String {
        // Borrow inputs for internal use
        let fen_str = fen.as_str();
        let moves_slice = valid_moves.as_slice();

        let mut out = String::with_capacity(2_048);
        out.push_str(HEADER);
        out.push_str(&self.render_board(fen_str, moves_slice, selected));
        out.push_str(FOOTER);
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

    /// Decide how to render a single square.
    fn render_square(
        &self,
        square: Option<char>,
        pos: &str,
        selected: &str,
        valid: &HashSet<&str>,
    ) -> String {
        // URL builders using self.base_url
        let select_url = |p: &str| format!("{}/select?square={}", self.base_url, p);
        let play_url = |mv: &str| format!("{}/play?move={}", self.base_url, mv);

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
                        md_link(&piece_md, PROFILE_URL)
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
                    md_link(&piece_md, PROFILE_URL)
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

/// Parse FEN into 8×8 board.
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

/// Bold for white, italic for black.
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

//——— constants ———//
const HEADER: &str = r#"# Readme Chess

Welcome to my GitHub profile! Here, you can play a game of chess with me, using my [readme-chess](https://github.com/grim-kalman/readme-chess) application.

## How to Play

- Click on any selectable piece ([**A**]()) to select it.
- Click on any destination square ([**_**]()) to move the selected piece.
- After each action, please wait for the page to refresh to see the updated game state.

## Chess Board
"#;

const FOOTER: &str = "\n[![New Game](https://img.shields.io/badge/New_Game-4CAF50)](https://readmechess.azurewebsites.net/new)";

const BOARD_HEADER: &str = "|     |  a  |  b  |  c  |  d  |  e  |  f  |  g  |  h  |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
";

const PROFILE_URL: &str = "https://github.com/grim-kalman";
