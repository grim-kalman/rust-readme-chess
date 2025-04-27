use rust_readme_chess::config::Config;
use rust_readme_chess::services::chess_service::ChessService;
use rust_readme_chess::services::engine_service::EngineService;
use rust_readme_chess::utils::printer::MarkdownPrinter;

// Helper to create a ChessService for tests
async fn setup_chess_service() -> ChessService {
    let config = Config::from_env().unwrap();
    let engine = EngineService::start(&config.engine_path)
        .await
        .expect("Failed to start engine");
    ChessService::new(engine)
}

/// Test: New game resets the board to the initial position.
#[tokio::test]
async fn test_new_game_resets_board() {
    // Setup
    let mut service = setup_chess_service().await;

    // Action
    service.play("e2e4").await.unwrap();
    service.new_game().await.unwrap();

    // Assert
    let fen = service.get_fen().await.unwrap();
    assert!(
        fen.starts_with("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"),
        "Board should be reset to initial position, got: {}",
        fen
    );
}

/// Test: Playing a move updates the board and markdown output.
#[tokio::test]
async fn test_play_and_print_board() {
    // Setup
    let mut service = setup_chess_service().await;

    // Action
    service.play("e2e4").await.unwrap();

    // Assert
    let fen = service.get_fen().await.unwrap();
    let valid_moves = service.get_valid_moves().await.unwrap();
    let config = Config::from_env().unwrap();
    let printer = MarkdownPrinter::new(config.base_url.clone(), config.github_owner_repo.clone());
    let board_md = printer.print(fen, valid_moves, "");
    assert!(
        board_md.contains("select?square=a2"),
        "Pawn a2 should be present in the board markdown and be selectable"
    );
}

/// Test: Selecting a square shows move links for valid moves.
#[tokio::test]
async fn test_select_square_shows_move_links() {
    // Setup
    let mut service = setup_chess_service().await;

    // Action
    service.select("e2").await.unwrap();

    // Assert
    let fen = service.get_fen().await.unwrap();
    let valid_moves = service.get_valid_moves().await.unwrap();
    let config = Config::from_env().unwrap();
    let printer = MarkdownPrinter::new(config.base_url.clone(), config.github_owner_repo.clone());
    let board_md = printer.print(fen, valid_moves, "e2");
    assert!(
        board_md.contains("play?mv=e2e3"),
        "Markdown should contain move link for e2e3"
    );
    assert!(
        board_md.contains("play?mv=e2e4"),
        "Markdown should contain move link for e2e4"
    );
}

/// Test: Toggling selection hides move links for that piece.
#[tokio::test]
async fn test_select_square_toggle_hides_move_links() {
    // Setup
    let mut service = setup_chess_service().await;

    // Action
    service.select("e2").await.unwrap();
    service.select("e2").await.unwrap();

    // Assert
    let fen = service.get_fen().await.unwrap();
    let valid_moves = service.get_valid_moves().await.unwrap();
    let config = Config::from_env().unwrap();
    let printer = MarkdownPrinter::new(config.base_url.clone(), config.github_owner_repo.clone());
    let board_md = printer.print(fen, valid_moves, "");
    assert!(
        !board_md.contains("play?mv=e2e3"),
        "Markdown should not contain move link for e2e3 after toggle"
    );
    assert!(
        !board_md.contains("play?mv=e2e4"),
        "Markdown should not contain move link for e2e4 after toggle"
    );
}

/// Test: Invalid move returns an error.
#[tokio::test]
async fn test_play_invalid_move_fails() {
    // Setup
    let mut service = setup_chess_service().await;

    // Action
    let result = service.play("e2e5").await;

    // Assert
    assert!(result.is_err(), "Invalid move should return an error");
}
