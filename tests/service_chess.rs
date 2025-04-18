use rust_readme_chess::service::chess::ChessService;
use rust_readme_chess::model::board::Board;
use rust_readme_chess::service::engine::EngineService;
use rust_readme_chess::service::github::GithubService;

#[test]
#[ignore]
fn test_chess_service_methods() {
    // Instantiate with stub implementations
    let mut service = ChessService::new(Board::new(), EngineService::new(), GithubService::new());
    let _ = service.init();
    let _ = service.play("e2e4");
    let _ = service.select("e2");
    let _ = service.new_game();
    let _ = service.print_board();
}