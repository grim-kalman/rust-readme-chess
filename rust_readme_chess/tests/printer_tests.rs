use rust_readme_chess::utils::printer::MarkdownPrinter;

/// Test that initial position renders pieces at their correct files.
#[test]
fn test_printer_initial_position() {
    let printer = MarkdownPrinter::new();
    // Standard initial position FEN
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let md = printer.print(fen);

    // Expected Markdown output
    let expected_md = r#"
# Readme Chess

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
|  **2**  |  [**P**](https://readmechess.azurewebsites.net/select?square=a2)  |  [**P**](https://readmechess.azurewebsites.net/select?square=b2)  |  [**P**](https://readmechess.azurewebsites.net/select?square=c2)  |  [**P**](https://readmechess.azurewebsites.net/select?square=d2)  |  [**P**](https://readmechess.azurewebsites.net/select?square=e2)  |  [**P**](https://readmechess.azurewebsites.net/select?square=f2)  |  [**P**](https://readmechess.azurewebsites.net/select?square=g2)  |  [**P**](https://readmechess.azurewebsites.net/select?square=h2)  |
|  **1**  |  [**R**](https://github.com/grim-kalman)  |  [**N**](https://readmechess.azurewebsites.net/select?square=b1)  |  [**B**](https://github.com/grim-kalman)  |  [**Q**](https://github.com/grim-kalman)  |  [**K**](https://github.com/grim-kalman)  |  [**B**](https://github.com/grim-kalman)  |  [**N**](https://readmechess.azurewebsites.net/select?square=g1)  |  [**R**](https://github.com/grim-kalman)  |

[![New Game](https://img.shields.io/badge/New_Game-4CAF50)](https://readmechess.azurewebsites.net/new)
"#;

    // Compare the generated Markdown with the expected output
    assert_eq!(md.trim(), expected_md.trim(), "Markdown output does not match the expected output");
}