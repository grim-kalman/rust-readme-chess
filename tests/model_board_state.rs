use rust_readme_chess::model::board::BoardState;
use rust_readme_chess::utils::constants::{WHITE, BLACK};

#[test]
fn test_reset_initial_state() {
    let mut state = BoardState::new();
    state.reset();
    assert_eq!(state.active_color(), WHITE);
    assert_eq!(state.castling_availability(), "KQkq");
    assert_eq!(state.en_passant_target(), "-");
    assert_eq!(state.half_move_clock(), 0);
    assert_eq!(state.full_move_number(), 1);
}

#[test]
fn test_update_and_full_move_increment() {
    let mut state = BoardState::new();
    state.reset();
    state.update("e2", false);
    // active color toggles to black
    assert_eq!(state.active_color(), BLACK);
    state.update("e7", false);
    // back to white and full move number increments
    assert_eq!(state.active_color(), WHITE);
    assert_eq!(state.full_move_number(), 2);
}

#[test]
fn test_handle_en_passant_target() {
    let mut state = BoardState::new();
    state.reset();
    // white two-step move sets en passant target to Java's "e6"
    state.handle_en_passant_target("e4");
    assert_eq!(state.en_passant_target(), "e6");

    // switch to black
    state.update("e2", false);
    state.handle_en_passant_target("d5");
    assert_eq!(state.en_passant_target(), "d3");
}

#[test]
fn test_half_move_clock_behavior() {
    let mut state = BoardState::new();
    state.reset();
    // Non-capture move increments half-move clock
    state.update("e2", false);
    assert_eq!(state.half_move_clock(), 1);
    // Capture move resets half-move clock
    state.update("e7", true);
    assert_eq!(state.half_move_clock(), 0);
}

#[test]
fn test_castling_rights_after_moves() {
    // King move on e1 removes both white castling rights
    let mut s1 = BoardState::new();
    s1.reset();
    s1.update("e1", false);
    assert_eq!(s1.castling_availability(), "kq");
    // Rook move on a1 removes white queen-side only
    let mut s2 = BoardState::new();
    s2.reset();
    s2.update("a1", false);
    assert_eq!(s2.castling_availability(), "Kkq");
    // King move on e8 removes both black castling rights
    let mut s3 = BoardState::new();
    s3.reset();
    s3.update("e8", false);
    assert_eq!(s3.castling_availability(), "KQ");
}

#[test]
fn test_en_passant_reset_on_update() {
    let mut state = BoardState::new();
    state.reset();
    state.handle_en_passant_target("c4");
    // After any move, en passant target should reset
    state.update("b2", false);
    assert_eq!(state.en_passant_target(), "-");
}

#[test]
fn test_castling_rights_h1_removes_white_castle() {
    let mut s = BoardState::new();
    s.reset();
    s.update("h1", false);
    // moving rook from h1 removes both white castling rights (matches Java logic)
    assert_eq!(s.castling_availability(), "kq");
}

#[test]
fn test_castling_rights_black_rooks() {
    // a8 removes black queen-side
    let mut sb = BoardState::new();
    sb.reset();
    sb.update("a8", false);
    assert_eq!(sb.castling_availability(), "Kk");
    // h8 removes black king-side
    let mut sb2 = BoardState::new();
    sb2.reset();
    sb2.update("h8", false);
    assert_eq!(sb2.castling_availability(), "Qk");
}