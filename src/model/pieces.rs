/// Trait representing a chess piece.
use crate::utils::constants::WHITE;

/// Trait representing a chess piece.
pub trait Piece {
    /// Returns the color of the piece (e.g., "w" or "b").
    fn color(&self) -> &str;
    /// Returns the symbol for the piece (e.g., 'P', 'p', 'K', etc.).
    fn symbol(&self) -> char;
}

macro_rules! impl_piece {
    ($name:ident, $sym_upper:expr, $sym_lower:expr) => {
        /// Chess piece type $name.
        pub struct $name {
            color: String,
        }

        impl $name {
            /// Creates a new piece of this type with the given color.
            pub fn new(color: &str) -> Self {
                Self {
                    color: color.to_string(),
                }
            }
        }

        impl Piece for $name {
            fn color(&self) -> &str {
                &self.color
            }
            fn symbol(&self) -> char {
                if self.color == WHITE {
                    $sym_upper
                } else {
                    $sym_lower
                }
            }
        }
    };
}

impl_piece!(Pawn, 'P', 'p');
impl_piece!(Rook, 'R', 'r');
impl_piece!(Knight, 'N', 'n');
impl_piece!(Bishop, 'B', 'b');
impl_piece!(Queen, 'Q', 'q');
impl_piece!(King, 'K', 'k');
