use std::env;
use rust_readme_chess::services::engine_service::EngineService;


// Constants for commonly used FEN positions
const INITIAL_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const AFTER_E4_POSITION: &str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1";

// Helper function to setup engine for tests
async fn setup_engine() -> EngineService {
    let path = env::var("ENGINE_PATH").expect("ENGINE_PATH not set");
    EngineService::start(&path).await.unwrap()
}

// Helper function to validate move format
fn is_valid_move_format(move_str: &str) -> bool {
    if move_str.len() != 4 {
        return false;
    }
    
    let bytes = move_str.as_bytes();
    let valid_file = |b| b'a' <= b && b <= b'h';
    let valid_rank = |b| b'1' <= b && b <= b'8';
    
    valid_file(bytes[0]) && valid_rank(bytes[1]) &&
    valid_file(bytes[2]) && valid_rank(bytes[3])
}




#[tokio::test]
async fn test_start_and_get_position() {
    let mut engine = setup_engine().await;
    
    let fen = engine.get_position().await.unwrap();
    assert!(fen.contains(INITIAL_POSITION), "Unexpected FEN: {}", fen);
    
    engine.stop().await.unwrap();
}




#[tokio::test]
async fn test_valid_moves_count() {
    let mut engine = setup_engine().await;
    
    let moves = engine.get_valid_moves().await.unwrap();
    assert_eq!(moves.len(), 20, "Expected 20 valid moves in starting position");
    
    engine.stop().await.unwrap();
}




#[tokio::test]
async fn test_make_move_and_get_position() {
    let mut engine = setup_engine().await;
    
    engine.make_move("e2e4").await.unwrap();
    let fen = engine.get_position().await.unwrap();
    assert!(fen.contains(AFTER_E4_POSITION), "Position after e4 is incorrect");
    
    engine.stop().await.unwrap();
}




#[tokio::test]
async fn test_best_move_format() {
    let mut engine = setup_engine().await;
    
    let best_move = engine.best_move().await.unwrap();
    assert!(is_valid_move_format(&best_move), "Invalid move format: {}", best_move);
    
    engine.stop().await.unwrap();
}




#[tokio::test]
async fn test_new_game_resets_position() {
    let mut engine = setup_engine().await;
    
    engine.make_move("e2e4").await.unwrap();
    engine.new_game().await.unwrap();
    
    let fen = engine.get_position().await.unwrap();
    assert!(fen.contains(INITIAL_POSITION), "New game didn't reset to initial position");
    
    engine.stop().await.unwrap();
}