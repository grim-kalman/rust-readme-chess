use std::env;
use rust_readme_chess::services::engine_service::EngineService;


#[tokio::test]
async fn test_start_and_get_position() {
    let path = env::var("ENGINE_PATH").expect("ENGINE_PATH not set");
    let mut engine = EngineService::start(&path).await.unwrap();
    let fen = engine.get_position().await.unwrap();
    assert!(
        fen.contains("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
        "Unexpected FEN: {}",
        fen
    );
    engine.stop().await.unwrap();
}

#[tokio::test]
async fn test_valid_moves_count() {
    let path = env::var("ENGINE_PATH").expect("ENGINE_PATH not set");
    let mut engine = EngineService::start(&path).await.unwrap();
    let moves = engine.get_valid_moves().await.unwrap();
    assert_eq!(moves.len(), 20);
    engine.stop().await.unwrap();
}

#[tokio::test]
async fn test_make_move_and_get_position() {
    let path = env::var("ENGINE_PATH").expect("ENGINE_PATH not set");
    let mut engine = EngineService::start(&path).await.unwrap();
    engine.make_move("e2e4").await.unwrap();
    let fen = engine.get_position().await.unwrap();
    assert!(fen.contains("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"));
    engine.stop().await.unwrap();
}

#[tokio::test]
async fn test_best_move_format() {
    let path = env::var("ENGINE_PATH").expect("ENGINE_PATH not set");
    let mut engine = EngineService::start(&path).await.unwrap();
    let mv = engine.best_move().await.unwrap();
    assert!(mv.len() == 4);
    let bytes = mv.as_bytes();
    assert!(b'a' <= bytes[0] && bytes[0] <= b'h');
    assert!(b'1' <= bytes[1] && bytes[1] <= b'8');
    assert!(b'a' <= bytes[2] && bytes[2] <= b'h');
    assert!(b'1' <= bytes[3] && bytes[3] <= b'8');
    engine.stop().await.unwrap();
}

#[tokio::test]
async fn test_new_game_resets_position() {
    let path = env::var("ENGINE_PATH").expect("ENGINE_PATH not set");
    let mut engine = EngineService::start(&path).await.unwrap();
    engine.make_move("e2e4").await.unwrap();
    engine.new_game().await.unwrap();
    let fen = engine.get_position().await.unwrap();
    assert!(fen.contains("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));
    engine.stop().await.unwrap();
}
