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

// Test FEN after a simple pawn move e2 to e4
#[test]
fn test_fen_after_simple_move() {
    let mut board = Board::new();
    board.reset();
    // Make a pawn move from e2 to e4
    board.make_move("e2e4");
    let printer = FenBoardPrinter::new(&board);
    let fen = printer.print();
    // After e2e4: pawn on e4, active_color=b, half_move_clock=1, full_move_number=1, no en passant target
    let expected = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 1 1";
    assert_eq!(fen, expected);
}