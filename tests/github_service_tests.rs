use rust_readme_chess::config::Config;
use rust_readme_chess::services::chess_service::ChessService;
use rust_readme_chess::services::engine_service::EngineService;
use rust_readme_chess::services::github_service::{GithubConfig, GithubService};
use rust_readme_chess::utils::printer::MarkdownPrinter;
use std::sync::Arc;

/// Test: Updates the README on the real GitHub profile repository, then reads it back.
/// Ignored by default because it commits to the live profile; run it deliberately with
/// `cargo test --test github_service_tests -- --ignored`.
#[tokio::test]
#[ignore]
async fn test_update_readme_real_github() {
    // Setup
    let config = Config::from_env().unwrap();
    let github_config = Arc::new(GithubConfig {
        token: config.github_token.clone(),
        api_url: "https://api.github.com/repos".to_string(),
        owner_repo: config.github_owner_repo.clone(),
        readme_path: config.github_readme_path.clone(),
        branch: config.github_branch.clone(),
    });
    let service = GithubService::new(github_config);

    // Get current board state and valid moves
    let engine = EngineService::start(&config.engine_path).await.unwrap();
    let board = ChessService::new(engine).board().await.unwrap();
    let printer = MarkdownPrinter::new(config.base_url.clone(), config.github_owner_repo.clone());
    let board_markdown = printer.print(&board);

    // Act
    let result = service.update_readme(&board_markdown).await;

    // Assert
    assert!(
        result.is_ok(),
        "Failed to update README: {:?}",
        result.err()
    );
    let readme = service.fetch_readme().await.unwrap();
    assert_eq!(readme.trim(), board_markdown.trim());
}
