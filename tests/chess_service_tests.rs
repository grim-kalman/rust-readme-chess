use rust_readme_chess::config::Config;
use rust_readme_chess::services::chess_service::{ChessService, GameStatus};
use rust_readme_chess::services::engine_service::EngineService;
use rust_readme_chess::utils::printer::MarkdownPrinter;

const INITIAL_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

async fn setup_engine() -> EngineService {
    let config = Config::from_env().unwrap();
    EngineService::start(&config.engine_path)
        .await
        .expect("Failed to start engine")
}

// Helper to create a ChessService for tests
async fn setup_chess_service() -> ChessService {
    ChessService::new(setup_engine().await)
}

fn setup_printer() -> MarkdownPrinter {
    let config = Config::from_env().unwrap();
    MarkdownPrinter::new(config.base_url.clone(), config.github_owner_repo.clone())
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
    let board = service.board().await.unwrap();
    assert!(
        board.fen.starts_with(INITIAL_POSITION),
        "Board should be reset to initial position, got: {}",
        board.fen
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
    let board = service.board().await.unwrap();
    let board_md = setup_printer().print(&board);
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
    service.select("e2");

    // Assert
    let board = service.board().await.unwrap();
    let board_md = setup_printer().print(&board);
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
    service.select("e2");
    service.select("e2");

    // Assert
    let board = service.board().await.unwrap();
    let board_md = setup_printer().print(&board);
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

/// Test: A mating move ends the game with no engine reply and the player's win recorded.
#[tokio::test]
async fn test_play_checkmate_ends_the_game() {
    // Setup: the scholar's mate position, white to deliver h5f7
    let mut engine = setup_engine().await;
    for mv in ["e2e4", "e7e5", "d1h5", "b8c6", "f1c4", "g8f6"] {
        engine.make_move(mv).await.unwrap();
    }
    let mut service = ChessService::new(engine);

    // Action
    service.play("h5f7").await.unwrap();

    // Assert
    let board = service.board().await.unwrap();
    assert_eq!(board.status, GameStatus::PlayerWon);
    assert!(board.valid_moves.is_empty());
    assert!(board.fen.contains(" b "), "engine is still to move: {}", board.fen);
}

/// Test: A mated player sees the engine's win.
#[tokio::test]
async fn test_board_reports_the_engine_win() {
    // Setup: the fool's mate, white mated
    let mut engine = setup_engine().await;
    for mv in ["f2f3", "e7e5", "g2g4", "d8h4"] {
        engine.make_move(mv).await.unwrap();
    }
    let mut service = ChessService::new(engine);

    // Assert
    let board = service.board().await.unwrap();
    assert_eq!(board.status, GameStatus::EngineWon);
}
