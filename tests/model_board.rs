use rust_readme_chess::model::board::Board;

#[test]
fn test_board_new_and_reset() {
    let mut board = Board::new();
    board.reset();
    // No square selected initially
    assert!(board.selected_square().is_none());
    // Board state should be reset
    assert_eq!(board.state().full_move_number(), 1);
}

// White pawn promotion: a2 to a1 should yield a white queen
#[test]
fn test_white_pawn_promotion_replaces_pawn_with_queen() {
    let mut board = Board::new();
    board.reset();
    // Simulate a pawn promotion move from a2 to a1
    board.make_move("a2a1");
    let pieces = board.pieces();
    let promoted = pieces.get("a1").expect("Piece should be at a1 after promotion");
    // White pawn promotion yields white queen ('Q') per Java logic
    assert_eq!(promoted.symbol(), 'Q');
}

// Queen-side castling for white: e1 to c1 and a1 to d1
#[test]
fn test_queen_side_castling_moves_rook_and_king_for_white() {
    let mut board = Board::new();
    board.reset();
    // Castling queen side for white: king e1->c1, rook a1->d1
    board.make_move("e1c1");
    let pieces = board.pieces();
    assert!(pieces.get("e1").is_none());
    assert_eq!(pieces.get("c1").unwrap().symbol(), 'K');
    assert!(pieces.get("a1").is_none());
    assert_eq!(pieces.get("d1").unwrap().symbol(), 'R');
}

// Queen-side castling for black: e8 to c8 and a8 to d8
#[test]
fn test_black_queen_side_castling_moves_rook_and_king() {
    let mut board = Board::new();
    board.reset();
    // Castling queen side for black: king e8->c8, rook a8->d8
    board.make_move("e8c8");
    let pieces = board.pieces();
    assert!(pieces.get("e8").is_none());
    assert_eq!(pieces.get("c8").unwrap().symbol(), 'q');
    assert!(pieces.get("a8").is_none());
    assert_eq!(pieces.get("d8").unwrap().symbol(), 'r');
}

// Invalid move should panic or error out
#[test]
#[should_panic]
fn test_invalid_move_panics() {
    let mut board = Board::new();
    board.reset();
    board.make_move("invalid");
}

#[test]
fn test_select_square_toggle() {
    let mut board = Board::new();
    board.select_square("e2");
    assert_eq!(board.selected_square().map(String::as_str), Some("e2"));
    board.select_square("e2");
    assert!(board.selected_square().is_none());
}

#[test]
fn test_simple_move_updates_pieces() {
    let mut board = Board::new();
    board.reset();
    // Move pawn from e2 to e4
    board.make_move("e2e4");
    let pieces = board.pieces();
    assert!(!pieces.contains_key("e2"));
    let p = pieces.get("e4").expect("Pawn should be at e4");
    assert_eq!(p.symbol(), 'P');
}

#[test]
fn test_capture_move() {
    let mut board = Board::new();
    board.reset();
    // White pawn captures black pawn on e7
    board.make_move("e2e7");
    let pieces = board.pieces();
    // Source cleared
    assert!(!pieces.contains_key("e2"));
    // White pawn now on e7
    let p = pieces.get("e7").expect("Pawn should be on e7 after capture");
    assert_eq!(p.symbol(), 'P');
}

#[test]
fn test_promotion_replaces_pawn_with_queen() {
    let mut board = Board::new();
    board.reset();
    // Simulate a pawn promotion move from a7 to a8
    board.make_move("a7a8");
    let pieces = board.pieces();
    let promoted = pieces.get("a8").expect("Piece should be at a8 after promotion");
    // Black pawn promotion from a7 -> a8 yields black queen ('q') per Java logic
    assert_eq!(promoted.symbol(), 'q');
}

#[test]
fn test_castling_moves_rook_and_king() {
    let mut board = Board::new();
    board.reset();
    // Castling king side for white: e1 to g1 and h1 to f1
    board.make_move("e1g1");
    let pieces = board.pieces();
    assert!(pieces.get("e1").is_none());
    assert_eq!(pieces.get("g1").unwrap().symbol(), 'K');
    assert!(pieces.get("h1").is_none());
    assert_eq!(pieces.get("f1").unwrap().symbol(), 'R');
}
// Board state propagation after moves

#[test]
fn test_board_state_after_simple_move() {
    let mut board = Board::new();
    board.reset();
    board.make_move("e2e4");
    let st = board.state();
    // Turn should flip to black, move number stays 1
    assert_eq!(st.active_color(), "b");
    assert_eq!(st.full_move_number(), 1);
    // Half-move clock increments on non-capture
    assert_eq!(st.half_move_clock(), 1);
    // Castling rights unchanged, en passant cleared after update
    assert_eq!(st.castling_availability(), "KQkq");
    assert_eq!(st.en_passant_target(), "-");
}

#[test]
fn test_board_state_after_two_pawn_moves() {
    let mut board = Board::new();
    board.reset();
    board.make_move("e2e4");
    board.make_move("e7e5");
    let st = board.state();
    // Turn back to white, full move number increments to 2
    assert_eq!(st.active_color(), "w");
    assert_eq!(st.full_move_number(), 2);
    // Half-move clock resets on each pawn move then increments => 1
    assert_eq!(st.half_move_clock(), 1);
    // Castling rights unchanged, en passant cleared
    assert_eq!(st.castling_availability(), "KQkq");
    assert_eq!(st.en_passant_target(), "-");
}