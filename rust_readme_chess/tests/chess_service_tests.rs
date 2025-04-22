use std::env;
use rust_readme_chess::services::chess_service::ChessService;
use rust_readme_chess::services::engine_service::EngineService;

// Test constants
const SAMPLE_MOVE: &str = "e2e4";
const INVALID_MOVE: &str = "e2e5";
const SAMPLE_SQUARE: &str = "e2";
const PAWN_LINK_PATTERN: &str = "[**P**](https://readmechess.azurewebsites.net/select?square=a2)";

// Helper function to setup chess service for tests
async fn setup_chess_service() -> ChessService {
    let path = env::var("ENGINE_PATH").expect("ENGINE_PATH not set");
    let engine = EngineService::start(&path).await.unwrap();
    let svc = ChessService::new(engine);
    svc.new_game().await.unwrap();
    svc
}




#[tokio::test]
async fn test_new_game_resets_state() {
    let svc = setup_chess_service().await;
    
    // Make a move to change initial state
    svc.play("a2a4").await.unwrap();
    assert!(!svc.get_moves().await.is_empty());
    
    // Test new game reset
    svc.new_game().await.unwrap();
    
    // Verify state reset
    assert_eq!(
        svc.get_moves().await.len(),
        0,
        "move history should be cleared"
    );
    assert!(
        svc.get_selected().await.is_none(),
        "selection should be cleared"
    );
    
    // Verify board state
    let md = svc.print_board().await.unwrap();
    assert!(
        md.contains(PAWN_LINK_PATTERN),
        "initial board should show selectable white pawns"
    );
}




#[tokio::test]
async fn test_select_toggles_selection() {
    let svc = setup_chess_service().await;
    
    // Test initial state
    assert!(
        svc.get_selected().await.is_none(),
        "should have no initial selection"
    );
    
    // Test selection
    svc.select(SAMPLE_SQUARE).await.unwrap();
    assert_eq!(
        svc.get_selected().await.as_deref(),
        Some(SAMPLE_SQUARE),
        "should select square"
    );
    
    // Test deselection
    svc.select(SAMPLE_SQUARE).await.unwrap();
    assert!(
        svc.get_selected().await.is_none(),
        "should deselect on second click"
    );
}




#[tokio::test]
async fn test_play_appends_moves_and_clears_selection() {
    let svc = setup_chess_service().await;
    
    // Setup and execute move
    svc.select(SAMPLE_SQUARE).await.unwrap();
    svc.play(SAMPLE_MOVE).await.unwrap();
    
    // Verify selection cleared
    assert!(
        svc.get_selected().await.is_none(),
        "play should clear selection"
    );
    
    // Verify moves recorded
    let moves = svc.get_moves().await;
    assert_eq!(
        moves,
        vec![SAMPLE_MOVE.to_string(), "e7e5".to_string()],
        "should record player move and engine reply"
    );
}




#[tokio::test]
async fn test_play_invalid_move() {
    let svc = setup_chess_service().await;
    
    // Attempt invalid move
    let result = svc.play(INVALID_MOVE).await;
    
    // Verify error handling
    assert!(result.is_err(), "Expected error for invalid move");
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains(&format!("Invalid move: {}", INVALID_MOVE)),
        "Unexpected error message: {}", 
        err_msg
    );
}