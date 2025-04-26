use rust_readme_chess::config::Config;
use rust_readme_chess::services::engine_service::EngineService;
use rust_readme_chess::utils::printer::MarkdownPrinter;

// Helper to start Stockfish for tests using the same config pattern as the main app
async fn setup_engine() -> EngineService {
    let config = Config::from_env().unwrap();
    EngineService::start(&config.engine_path)
        .await
        .expect("Failed to start engine")
}

#[tokio::test]
async fn test_printer_initial_position() {
    // Arrange
    let mut engine = setup_engine().await;
    let config = Config::from_env().unwrap();
    let base_url = &config.base_url;
    let printer = MarkdownPrinter::new(base_url.clone());

    // Act
    let fen = engine.get_position().await.unwrap();
    let valid_moves = engine.get_valid_moves().await.unwrap();
    let md = printer.print(fen, valid_moves, "");

    // Assert
    let expected_md = format!(
        r#"# Readme Chess

Welcome to my GitHub profile! Here, you can play a game of chess with me, using my [readme-chess](https://github.com/grim-kalman/readme-chess) application.

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
|  **1**  |  [**R**](https://github.com/grim-kalman)  |  [**N**]({0}/select?square=b1)  |  [**B**](https://github.com/grim-kalman)  |  [**Q**](https://github.com/grim-kalman)  |  [**K**](https://github.com/grim-kalman)  |  [**B**](https://github.com/grim-kalman)  |  [**N**]({0}/select?square=g1)  |  [**R**](https://github.com/grim-kalman)  |

[![New Game](https://img.shields.io/badge/New_Game-4CAF50)]({0}/new)"#,
        base_url
    );

    assert_eq!(
        md.trim(),
        expected_md.trim(),
        "Markdown output does not match the expected output"
    );
}

#[tokio::test]
async fn test_printer_select_pawn_e2() {
    // Arrange
    let mut engine = setup_engine().await;
    let config = Config::from_env().unwrap();
    let base_url = &config.base_url;
    let printer = MarkdownPrinter::new(base_url.clone());

    // Act
    let fen = engine.get_position().await.unwrap();
    let selected_square = "e2";
    let valid_moves = engine.get_valid_moves().await.unwrap();
    let md = printer.print(fen, valid_moves, selected_square);

    // Assert
    let expected_md = format!(
        r#"# Readme Chess

Welcome to my GitHub profile! Here, you can play a game of chess with me, using my [readme-chess](https://github.com/grim-kalman/readme-chess) application.

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
|  **5**  |     |     |     |     |  [_]({0}/play?move=e2e4)  |     |     |     |
|  **4**  |     |     |     |     |  [_]({0}/play?move=e2e3)  |     |     |     |
|  **2**  |  [**P**]({0}/select?square=a2)  |  [**P**]({0}/select?square=b2)  |  [**P**]({0}/select?square=c2)  |  [**P**]({0}/select?square=d2)  |  [**P**]({0}/select?square=e2)  |  [**P**]({0}/select?square=f2)  |  [**P**]({0}/select?square=g2)  |  [**P**]({0}/select?square=h2)  |
|  **1**  |  [**R**](https://github.com/grim-kalman)  |  [**N**]({0}/select?square=b1)  |  [**B**](https://github.com/grim-kalman)  |  [**Q**](https://github.com/grim-kalman)  |  [**K**](https://github.com/grim-kalman)  |  [**B**](https://github.com/grim-kalman)  |  [**N**]({0}/select?square=g1)  |  [**R**](https://github.com/grim-kalman)  |

[![New Game](https://img.shields.io/badge/New_Game-4CAF50)]({0}/new)"#,
        base_url
    );

    assert_eq!(
        md.trim(),
        expected_md.trim(),
        "Markdown output does not match the expected output"
    );
}

#[tokio::test]
async fn test_printer_after_move_e2e4_c7c5() {
    // Arrange
    let mut engine = setup_engine().await;
    let config = Config::from_env().unwrap();
    let base_url = &config.base_url;
    let printer = MarkdownPrinter::new(base_url.clone());

    // Act
    engine.make_move("e2e4").await.unwrap();
    engine.make_move("c7c5").await.unwrap();
    let fen = engine.get_position().await.unwrap();
    let valid_moves = engine.get_valid_moves().await.unwrap();
    let md = printer.print(fen, valid_moves, "");

    // Assert
    let expected_md = format!(
        r#"# Readme Chess

Welcome to my GitHub profile! Here, you can play a game of chess with me, using my [readme-chess](https://github.com/grim-kalman/readme-chess) application.

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
|  **1**  |  [**R**](https://github.com/grim-kalman)  |  [**N**]({0}/select?square=b1)  |  [**B**](https://github.com/grim-kalman)  |  [**Q**]({0}/select?square=d1)  |  [**K**]({0}/select?square=e1)  |  [**B**]({0}/select?square=f1)  |  [**N**]({0}/select?square=g1)  |  [**R**](https://github.com/grim-kalman)  |

[![New Game](https://img.shields.io/badge/New_Game-4CAF50)]({0}/new)"#,
        base_url
    );

    assert_eq!(
        md.trim(),
        expected_md.trim(),
        "Markdown output does not match the expected output after move e2e4 and engine reply c7c5"
    );
}

#[tokio::test]
async fn test_printer_after_move_e2e4_c7c5_and_select_d1() {
    // Arrange
    let mut engine = setup_engine().await;
    let config = Config::from_env().unwrap();
    let base_url = &config.base_url;
    let printer = MarkdownPrinter::new(base_url.clone());

    // Act
    engine.make_move("e2e4").await.unwrap();
    engine.make_move("c7c5").await.unwrap();
    let fen = engine.get_position().await.unwrap();
    let valid_moves = engine.get_valid_moves().await.unwrap();
    let md = printer.print(fen, valid_moves, "d1");

    // Assert
    let expected_md = format!(
        r#"# Readme Chess

Welcome to my GitHub profile! Here, you can play a game of chess with me, using my [readme-chess](https://github.com/grim-kalman/readme-chess) application.

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
|  **5**  |     |     |  _p_  |     |     |     |     |  [_]({0}/play?move=d1h5)  |
|  **4**  |     |     |     |     |  [**P**]({0}/select?square=e4)  |     |  [_]({0}/play?move=d1g4)  |     |
|  **3**  |     |     |     |     |     |  [_]({0}/play?move=d1f3)  |     |     |
|  **2**  |  [**P**]({0}/select?square=a2)  |  [**P**]({0}/select?square=b2)  |  [**P**]({0}/select?square=c2)  |  [**P**]({0}/select?square=d2)  |  [_]({0}/play?move=d1e2)  |  [**P**]({0}/select?square=f2)  |  [**P**]({0}/select?square=g2)  |  [**P**]({0}/select?square=h2)  |
|  **1**  |  [**R**](https://github.com/grim-kalman)  |  [**N**]({0}/select?square=b1)  |  [**B**](https://github.com/grim-kalman)  |  [**Q**](https://github.com/grim-kalman)  |  [**K**](https://github.com/grim-kalman)  |  [**B**](https://github.com/grim-kalman)  |  [**N**]({0}/select?square=g1)  |  [**R**](https://github.com/grim-kalman)  |

[![New Game](https://img.shields.io/badge/New_Game-4CAF50)]({0}/new)"#,
        base_url
    );

    assert_eq!(
        md.trim(),
        expected_md.trim(),
        "Markdown output does not match the expected output after move e2e4 and engine reply c7c5"
    );
}
