use rust_readme_chess::model::board::Board;
use rust_readme_chess::utils::printer::{BoardPrinter, MarkdownBoardPrinter};
use rust_readme_chess::service::engine::EngineService;
use rust_readme_chess::utils::validator::MoveValidator;

#[test]
fn test_markdown_contains_header_and_board() {
    let board = Board::new();
    let engine = EngineService::new();
    let validator = MoveValidator::new(engine);
    let printer = MarkdownBoardPrinter::new(&board, validator);
    let md = printer.print();
    // Should contain the markdown header
    assert!(md.contains("# Readme Chess"));
    // Should contain file labels a-h
    assert!(md.contains("|  a  |"));
}