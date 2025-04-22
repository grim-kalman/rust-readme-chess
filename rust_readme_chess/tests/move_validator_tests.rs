use std::env;
use rust_readme_chess::services::engine_service::EngineService;
use rust_readme_chess::utils::validator::MoveValidator;


#[tokio::test]
async fn test_valid_moves_from_initial_position() {
    // Setup
    let path = env::var("ENGINE_PATH").unwrap();
    let mut engine = EngineService::start(&path).await.unwrap();
    let validator = MoveValidator::new(&mut engine).await.unwrap();

    // Test pawn moves
    assert!(validator.is_valid("e2e4"), "Pawn should move two squares forward");
    assert!(!validator.is_valid("e2e5"), "Pawn cannot move three squares");
    
    // Test partial move validation
    assert!(validator.is_start_of_valid_move("e2"), "White pawn should be moveable");
    assert!(!validator.is_start_of_valid_move("e7"), "Black pawn should not be moveable");
    
    // Test knight moves
    assert!(validator.is_valid("b1c3"), "Knight should move in L-shape");
    assert!(validator.is_start_of_valid_move("b1"), "Knight should be moveable");

    // Teardown
    engine.stop().await.unwrap();
}