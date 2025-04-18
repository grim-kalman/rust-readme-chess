use rust_readme_chess::model::board::Board;
use rust_readme_chess::utils::printer::{BoardPrinter, FenBoardPrinter};

#[test]
fn test_initial_fen_format() {
    let board = Board::new();
    let printer = FenBoardPrinter::new(&board);
    let fen = printer.print();
    // Standard starting position FEN
    let expected = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    assert_eq!(fen, expected);
}