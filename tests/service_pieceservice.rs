use rust_readme_chess::service::pieceservice::PieceService;

#[test]
fn test_setup_pieces_count() {
    let service = PieceService::new();
    let pieces = service.setup_pieces();
    // Expect 32 pieces in starting position
    assert_eq!(pieces.len(), 32);
}

#[test]
fn test_pawn_positions() {
    let pieces = PieceService::new().setup_pieces();
    // a2 should be a white pawn 'P'
    let p = pieces.get("a2").expect("Missing a2");
    assert_eq!(p.symbol(), 'P');
    // h7 should be a black pawn 'p'
    let p_b = pieces.get("h7").expect("Missing h7");
    assert_eq!(p_b.symbol(), 'p');
}

#[test]
fn test_other_piece_positions() {
    let pieces = PieceService::new().setup_pieces();
    // White rooks
    assert_eq!(pieces.get("a1").unwrap().symbol(), 'R');
    assert_eq!(pieces.get("h1").unwrap().symbol(), 'R');
    // Black rooks
    assert_eq!(pieces.get("a8").unwrap().symbol(), 'r');
    assert_eq!(pieces.get("h8").unwrap().symbol(), 'r');
    // Knights
    assert_eq!(pieces.get("b1").unwrap().symbol(), 'N');
    assert_eq!(pieces.get("g8").unwrap().symbol(), 'n');
    // Bishops
    assert_eq!(pieces.get("c1").unwrap().symbol(), 'B');
    assert_eq!(pieces.get("f8").unwrap().symbol(), 'b');
    // Queens and Kings
    assert_eq!(pieces.get("d1").unwrap().symbol(), 'Q');
    assert_eq!(pieces.get("e1").unwrap().symbol(), 'K');
    assert_eq!(pieces.get("d8").unwrap().symbol(), 'q');
    assert_eq!(pieces.get("e8").unwrap().symbol(), 'k');
}