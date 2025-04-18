use rust_readme_chess::model::pieces::{Pawn, Rook, Knight, Bishop, Queen, King, Piece};

#[test]
fn test_pawn_symbol_and_color() {
    let p_w = Pawn::new("w");
    assert_eq!(p_w.color(), "w");
    assert_eq!(p_w.symbol(), 'P');
    let p_b = Pawn::new("b");
    assert_eq!(p_b.color(), "b");
    assert_eq!(p_b.symbol(), 'p');
}

#[test]
fn test_rook_symbol_and_color() {
    let r_w = Rook::new("w");
    assert_eq!(r_w.color(), "w");
    assert_eq!(r_w.symbol(), 'R');
}

#[test]
fn test_knight_symbol_and_color() {
    let n_b = Knight::new("b");
    assert_eq!(n_b.color(), "b");
    assert_eq!(n_b.symbol(), 'n');
}

#[test]
fn test_bishop_and_queen_and_king() {
    let b = Bishop::new("w");
    assert_eq!(b.symbol(), 'B');
    let q = Queen::new("b");
    assert_eq!(q.symbol(), 'q');
    let k = King::new("w");
    assert_eq!(k.symbol(), 'K');
}