use rust_readme_chess::config::Config;
use rust_readme_chess::services::chess_service::{Board, ChessService, GameStatus};
use rust_readme_chess::services::engine_service::EngineService;
use rust_readme_chess::utils::printer::{parse_moves, MarkdownPrinter};

// Helper to start Stockfish for tests using the same config pattern as the main app
async fn setup_engine() -> EngineService {
    let config = Config::from_env().unwrap();
    EngineService::start(&config.engine_path)
        .await
        .expect("Failed to start engine")
}

fn setup_printer() -> (MarkdownPrinter, Config) {
    let config = Config::from_env().unwrap();
    let printer = MarkdownPrinter::new(config.base_url.clone(), config.github_owner_repo.clone());
    (printer, config)
}

fn board(fen: &str, valid_moves: &[&str], selected: &str, status: GameStatus) -> Board {
    Board {
        fen: fen.to_string(),
        valid_moves: valid_moves.iter().map(|m| m.to_string()).collect(),
        selected: selected.to_string(),
        status,
        moves: vec![],
    }
}

/// Test: The move list written into the README reads back unchanged; a README without
/// one reads as no moves.
#[test]
fn test_printer_round_trips_the_move_list() {
    let printer = MarkdownPrinter::new("http://x".into(), "owner".into());
    let mut played = board("k7/8/8/8/8/8/8/4K3 w - - 0 1", &["e1d1"], "", GameStatus::Playing);
    played.moves = vec!["e2e4".into(), "e7e5".into(), "g1f3".into()];
    let fresh = board("k7/8/8/8/8/8/8/4K3 w - - 0 1", &["e1d1"], "", GameStatus::Playing);

    assert_eq!(parse_moves(&printer.print(&played)), played.moves);
    assert!(parse_moves(&printer.print(&fresh)).is_empty());
    assert!(parse_moves("# Some other README").is_empty());
}

/// Test: A pawn on the seventh rank links its promotion square, promoting to a queen.
#[test]
fn test_printer_links_promotion_as_queen() {
    let printer = MarkdownPrinter::new("http://x".into(), "owner".into());
    let moves = ["e7e8q", "e7e8r", "e7e8b", "e7e8n", "e1d1", "e1f1"];
    let board = board("k7/4P3/8/8/8/8/8/4K3 w - - 0 1", &moves, "e7", GameStatus::Playing);

    let md = printer.print(&board);

    assert!(md.contains("[_](http://x/play?mv=e7e8q)"), "{}", md);
    assert!(!md.contains("e7e8r"), "{}", md);
}

/// Test: A finished game says so above the New Game link; a running one says nothing.
#[test]
fn test_printer_announces_the_outcome() {
    let printer = MarkdownPrinter::new("http://x".into(), "owner".into());
    let fen = "r1bqkb1r/pppp1Qpp/2n2n2/4p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4";

    let won = printer.print(&board(fen, &[], "", GameStatus::PlayerWon));
    let lost = printer.print(&board(fen, &[], "", GameStatus::EngineWon));
    let drawn = printer.print(&board(fen, &[], "", GameStatus::Draw));
    let playing = printer.print(&board(fen, &["e1e2"], "", GameStatus::Playing));

    assert!(won.contains("**Checkmate — you win!**"), "{}", won);
    assert!(lost.contains("**Checkmate — I win this one.**"), "{}", lost);
    assert!(drawn.contains("**Stalemate — a draw.**"), "{}", drawn);
    assert!(!playing.contains("**Checkmate"), "{}", playing);
    assert!(won.find("Checkmate") < won.find("http://x/new"), "{}", won);
}

/// Test: Initial board position renders correct markdown.
#[tokio::test]
async fn test_printer_initial_position() {
    // Arrange
    let mut service = ChessService::new(setup_engine().await);
    let (printer, config) = setup_printer();
    let base_url = &config.base_url;

    // Act
    let md = printer.print(&service.board().await.unwrap());

    // Assert
    let expected_md = format!(
        r#"# Readme Chess

Welcome to my GitHub profile! Here, you can play a game of chess with me, using my [readme-chess](https://github.com/grim-kalman/rust-readme-chess) application.

## How to Play

- Click on any selectable piece ([**A**]()) to select it.
- Click on any destination square ([**_**]()) to move the selected piece.
- After each action, please wait for the page to refresh to see the updated game state.

## Chess Board
|     |  a  |  b  |  c  |  d  |  e  |  f  |  g  |  h  |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
|  **8**  |  _r_  |  _n_  |  _b_  |  _q_  |  _k_  |  _b_  |  _n_  |  _r_  |
|  **7**  |  _p_  |  _p_  |  _p_  |  _p_  |  _p_  |  _p_  |  _p_  |  _p_  |
|  **6**  |     |     |     |     |     |     |     |     |
|  **5**  |     |     |     |     |     |     |     |     |
|  **4**  |     |     |     |     |     |     |     |     |
|  **3**  |     |     |     |     |     |     |     |     |
|  **2**  |  [**P**]({0}/select?square=a2)  |  [**P**]({0}/select?square=b2)  |  [**P**]({0}/select?square=c2)  |  [**P**]({0}/select?square=d2)  |  [**P**]({0}/select?square=e2)  |  [**P**]({0}/select?square=f2)  |  [**P**]({0}/select?square=g2)  |  [**P**]({0}/select?square=h2)  |
|  **1**  |  [**R**](https://github.com/{1})  |  [**N**]({0}/select?square=b1)  |  [**B**](https://github.com/{1})  |  [**Q**](https://github.com/{1})  |  [**K**](https://github.com/{1})  |  [**B**](https://github.com/{1})  |  [**N**]({0}/select?square=g1)  |  [**R**](https://github.com/{1})  |

[![New Game](https://img.shields.io/badge/New_Game-4CAF50)]({0}/new)
<!-- moves:{2} -->"#,
        base_url,
        config.github_owner_repo,
        ""
    );

    assert_eq!(
        md.trim(),
        expected_md.trim(),
        "Markdown output does not match the expected output"
    );
}

/// Test: Selecting a pawn highlights its valid moves.
#[tokio::test]
async fn test_printer_select_pawn_e2() {
    // Arrange
    let mut service = ChessService::new(setup_engine().await);
    let (printer, config) = setup_printer();
    let base_url = &config.base_url;

    // Act
    service.select("e2");
    let md = printer.print(&service.board().await.unwrap());

    // Assert
    let expected_md = format!(
        r#"# Readme Chess

Welcome to my GitHub profile! Here, you can play a game of chess with me, using my [readme-chess](https://github.com/grim-kalman/rust-readme-chess) application.

## How to Play

- Click on any selectable piece ([**A**]()) to select it.
- Click on any destination square ([**_**]()) to move the selected piece.
- After each action, please wait for the page to refresh to see the updated game state.

## Chess Board
|     |  a  |  b  |  c  |  d  |  e  |  f  |  g  |  h  |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
|  **8**  |  _r_  |  _n_  |  _b_  |  _q_  |  _k_  |  _b_  |  _n_  |  _r_  |
|  **7**  |  _p_  |  _p_  |  _p_  |  _p_  |  _p_  |  _p_  |  _p_  |  _p_  |
|  **6**  |     |     |     |     |     |     |     |     |
|  **5**  |     |     |     |     |     |     |     |     |
|  **4**  |     |     |     |     |  [_]({0}/play?mv=e2e4)  |     |     |     |
|  **3**  |     |     |     |     |  [_]({0}/play?mv=e2e3)  |     |     |     |
|  **2**  |  [**P**]({0}/select?square=a2)  |  [**P**]({0}/select?square=b2)  |  [**P**]({0}/select?square=c2)  |  [**P**]({0}/select?square=d2)  |  [**P**]({0}/select?square=e2)  |  [**P**]({0}/select?square=f2)  |  [**P**]({0}/select?square=g2)  |  [**P**]({0}/select?square=h2)  |
|  **1**  |  [**R**](https://github.com/{1})  |  [**N**]({0}/select?square=b1)  |  [**B**](https://github.com/{1})  |  [**Q**](https://github.com/{1})  |  [**K**](https://github.com/{1})  |  [**B**](https://github.com/{1})  |  [**N**]({0}/select?square=g1)  |  [**R**](https://github.com/{1})  |

[![New Game](https://img.shields.io/badge/New_Game-4CAF50)]({0}/new)
<!-- moves:{2} -->"#,
        base_url,
        config.github_owner_repo,
        ""
    );

    assert_eq!(
        md.trim(),
        expected_md.trim(),
        "Markdown output does not match the expected output"
    );
}

/// Test: After e2e4 and c7c5, board renders correctly.
#[tokio::test]
async fn test_printer_after_move_e2e4_c7c5() {
    // Arrange
    let mut engine = setup_engine().await;
    let (printer, config) = setup_printer();
    let base_url = &config.base_url;

    // Act
    engine.make_move("e2e4").await.unwrap();
    engine.make_move("c7c5").await.unwrap();
    let mut service = ChessService::new(engine);
    let md = printer.print(&service.board().await.unwrap());

    // Assert
    let expected_md = format!(
        r#"# Readme Chess

Welcome to my GitHub profile! Here, you can play a game of chess with me, using my [readme-chess](https://github.com/grim-kalman/rust-readme-chess) application.

## How to Play

- Click on any selectable piece ([**A**]()) to select it.
- Click on any destination square ([**_**]()) to move the selected piece.
- After each action, please wait for the page to refresh to see the updated game state.

## Chess Board
|     |  a  |  b  |  c  |  d  |  e  |  f  |  g  |  h  |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
|  **8**  |  _r_  |  _n_  |  _b_  |  _q_  |  _k_  |  _b_  |  _n_  |  _r_  |
|  **7**  |  _p_  |  _p_  |     |  _p_  |  _p_  |  _p_  |  _p_  |  _p_  |
|  **6**  |     |     |     |     |     |     |     |     |
|  **5**  |     |     |  _p_  |     |     |     |     |     |
|  **4**  |     |     |     |     |  [**P**]({0}/select?square=e4)  |     |     |     |
|  **3**  |     |     |     |     |     |     |     |     |
|  **2**  |  [**P**]({0}/select?square=a2)  |  [**P**]({0}/select?square=b2)  |  [**P**]({0}/select?square=c2)  |  [**P**]({0}/select?square=d2)  |     |  [**P**]({0}/select?square=f2)  |  [**P**]({0}/select?square=g2)  |  [**P**]({0}/select?square=h2)  |
|  **1**  |  [**R**](https://github.com/{1})  |  [**N**]({0}/select?square=b1)  |  [**B**](https://github.com/{1})  |  [**Q**]({0}/select?square=d1)  |  [**K**]({0}/select?square=e1)  |  [**B**]({0}/select?square=f1)  |  [**N**]({0}/select?square=g1)  |  [**R**](https://github.com/{1})  |

[![New Game](https://img.shields.io/badge/New_Game-4CAF50)]({0}/new)
<!-- moves:{2} -->"#,
        base_url,
        config.github_owner_repo,
        " e2e4 c7c5"
    );

    assert_eq!(
        md.trim(),
        expected_md.trim(),
        "Markdown output does not match the expected output after move e2e4 and engine reply c7c5"
    );
}

/// Test: After e2e4, c7c5, and selecting d1, the queen's moves are shown and the other
/// white pieces with moves stay selectable.
#[tokio::test]
async fn test_printer_after_move_e2e4_c7c5_and_select_d1() {
    // Arrange
    let mut engine = setup_engine().await;
    let (printer, config) = setup_printer();
    let base_url = &config.base_url;

    // Act
    engine.make_move("e2e4").await.unwrap();
    engine.make_move("c7c5").await.unwrap();
    let mut service = ChessService::new(engine);
    service.select("d1");
    let md = printer.print(&service.board().await.unwrap());

    // Assert
    let expected_md = format!(
        r#"# Readme Chess

Welcome to my GitHub profile! Here, you can play a game of chess with me, using my [readme-chess](https://github.com/grim-kalman/rust-readme-chess) application.

## How to Play

- Click on any selectable piece ([**A**]()) to select it.
- Click on any destination square ([**_**]()) to move the selected piece.
- After each action, please wait for the page to refresh to see the updated game state.

## Chess Board
|     |  a  |  b  |  c  |  d  |  e  |  f  |  g  |  h  |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
|  **8**  |  _r_  |  _n_  |  _b_  |  _q_  |  _k_  |  _b_  |  _n_  |  _r_  |
|  **7**  |  _p_  |  _p_  |     |  _p_  |  _p_  |  _p_  |  _p_  |  _p_  |
|  **6**  |     |     |     |     |     |     |     |     |
|  **5**  |     |     |  _p_  |     |     |     |     |  [_]({0}/play?mv=d1h5)  |
|  **4**  |     |     |     |     |  [**P**]({0}/select?square=e4)  |     |  [_]({0}/play?mv=d1g4)  |     |
|  **3**  |     |     |     |     |     |  [_]({0}/play?mv=d1f3)  |     |     |
|  **2**  |  [**P**]({0}/select?square=a2)  |  [**P**]({0}/select?square=b2)  |  [**P**]({0}/select?square=c2)  |  [**P**]({0}/select?square=d2)  |  [_]({0}/play?mv=d1e2)  |  [**P**]({0}/select?square=f2)  |  [**P**]({0}/select?square=g2)  |  [**P**]({0}/select?square=h2)  |
|  **1**  |  [**R**](https://github.com/{1})  |  [**N**]({0}/select?square=b1)  |  [**B**](https://github.com/{1})  |  [**Q**]({0}/select?square=d1)  |  [**K**]({0}/select?square=e1)  |  [**B**]({0}/select?square=f1)  |  [**N**]({0}/select?square=g1)  |  [**R**](https://github.com/{1})  |

[![New Game](https://img.shields.io/badge/New_Game-4CAF50)]({0}/new)
<!-- moves:{2} -->"#,
        base_url,
        config.github_owner_repo,
        " e2e4 c7c5"
    );

    assert_eq!(
        md.trim(),
        expected_md.trim(),
        "Markdown output does not match the expected output after move e2e4 and engine reply c7c5"
    );
}
