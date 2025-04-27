use rust_readme_chess::config::Config;
use rust_readme_chess::services::engine_service::EngineService;
use rust_readme_chess::services::github_service::{GithubConfig, GithubService};
use rust_readme_chess::utils::printer::MarkdownPrinter;
use std::sync::Arc;

/// Test: Updates the README on a real GitHub repository with the current board state.
#[tokio::test]
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
    let mut engine = EngineService::start(&config.engine_path).await.unwrap();
    let fen = engine.get_position().await.unwrap();
    let valid_moves = engine.get_valid_moves().await.unwrap();
    let printer = MarkdownPrinter::new(config.base_url.clone(), config.github_owner_repo.clone());
    let board_markdown = printer.print(fen, valid_moves, "");

    // Act
    let result = service.update_readme(&board_markdown).await;

    // Assert
    assert!(
        result.is_ok(),
        "Failed to update README: {:?}",
        result.err()
    );
}
