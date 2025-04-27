use rust_readme_chess::config::Config;
use rust_readme_chess::services::engine_service::EngineService;

// Constants for commonly used FEN positions
const INITIAL_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const AFTER_E4_POSITION: &str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1";

// Helper function to setup engine for tests
async fn setup_engine() -> EngineService {
    let config = Config::from_env().unwrap();
    EngineService::start(&config.engine_path)
        .await
        .expect("Failed to start engine")
}

// Helper function to validate move format
fn is_valid_move_format(move_str: &str) -> bool {
    if move_str.len() != 4 {
        return false;
    }
    let bytes = move_str.as_bytes();
    let valid_file = |b| b'a' <= b && b <= b'h';
    let valid_rank = |b| b'1' <= b && b <= b'8';
    valid_file(bytes[0]) && valid_rank(bytes[1]) && valid_file(bytes[2]) && valid_rank(bytes[3])
}

/// Test: Engine starts and returns the initial position FEN.
#[tokio::test]
async fn test_start_and_get_position() {
    // Arrange
    let mut engine = setup_engine().await;

    // Act
    let fen = engine.get_position().await.unwrap();

    // Assert
    assert!(fen.contains(INITIAL_POSITION), "Unexpected FEN: {}", fen);

    engine.stop().await.unwrap();
}

/// Test: Valid moves count is correct in the starting position.
#[tokio::test]
async fn test_valid_moves_count() {
    // Arrange
    let mut engine = setup_engine().await;

    // Act
    let moves = engine.get_valid_moves().await.unwrap();

    // Assert
    assert_eq!(
        moves.len(),
        20,
        "Expected 20 valid moves in starting position"
    );

    engine.stop().await.unwrap();
}

/// Test: Making a move updates the position FEN.
#[tokio::test]
async fn test_make_move_and_get_position() {
    // Arrange
    let mut engine = setup_engine().await;

    // Act
    engine.make_move("e2e4").await.unwrap();
    let fen = engine.get_position().await.unwrap();

    // Assert
    assert!(
        fen.contains(AFTER_E4_POSITION),
        "Position after e4 is incorrect"
    );

    engine.stop().await.unwrap();
}

/// Test: Engine returns a valid best move in UCI format.
#[tokio::test]
async fn test_best_move_format() {
    // Arrange
    let mut engine = setup_engine().await;

    // Act
    let best_move = engine.best_move().await.unwrap();

    // Assert
    assert!(
        is_valid_move_format(&best_move),
        "Invalid move format: {}",
        best_move
    );

    engine.stop().await.unwrap();
}

/// Test: New game resets the position to the initial FEN.
#[tokio::test]
async fn test_new_game_resets_position() {
    // Arrange
    let mut engine = setup_engine().await;

    // Act
    engine.make_move("e2e4").await.unwrap();
    engine.new_game().await.unwrap();
    let fen = engine.get_position().await.unwrap();

    // Assert
    assert!(
        fen.contains(INITIAL_POSITION),
        "New game didn't reset to initial position"
    );

    engine.stop().await.unwrap();
}
