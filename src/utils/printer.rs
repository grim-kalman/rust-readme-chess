use crate::model::board::Board;
use crate::utils::validator::MoveValidator;

/// Trait for printing a chess board in various formats.
pub trait BoardPrinter {
    /// Renders the board to a string.
    fn print(&self) -> String;
}

/// FEN (Forsyth–Edwards Notation) board printer.
pub struct FenBoardPrinter<'a> {
    board: &'a Board,
}

impl<'a> FenBoardPrinter<'a> {
    /// Creates a new FEN printer for the given board.
    pub fn new(board: &'a Board) -> Self {
        // TODO: implement
        unimplemented!()
    }
}

impl<'a> BoardPrinter for FenBoardPrinter<'a> {
    fn print(&self) -> String {
        // TODO: implement
        unimplemented!()
    }
}

/// Markdown board printer.
pub struct MarkdownBoardPrinter<'a> {
    board: &'a Board,
    move_validator: MoveValidator,
}

impl<'a> MarkdownBoardPrinter<'a> {
    /// Creates a new Markdown printer for the board, with a move validator.
    pub fn new(board: &'a Board, move_validator: MoveValidator) -> Self {
        // TODO: implement
        unimplemented!()
    }
}

impl<'a> BoardPrinter for MarkdownBoardPrinter<'a> {
    fn print(&self) -> String {
        // TODO: implement
        unimplemented!()
    }
}
