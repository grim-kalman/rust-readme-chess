use std::env;

#[derive(Clone, Debug)]
/// Centralized application configuration loaded from environment variables.
pub struct Config {
    /// Path to the chess engine executable (e.g., Stockfish)
    pub engine_path: String,
    /// Address and port to bind the Actix web server (e.g., "127.0.0.1:8080")
    pub server_addr: String,
    /// GitHub personal access token for API operations
    pub github_token: String,
    /// GitHub repository in the format "owner" (e.g., "grim-kalman")
    pub github_owner_repo: String,
    /// Branch to update (e.g., "main")
    pub github_branch: String,
    /// Path to the README file in the repository
    pub github_readme_path: String,
    /// Base URL for endpoint links (e.g., "https://your.domain.com")
    pub base_url: String,
}

impl Config {
    /// Load configuration from environment variables. Panics if required variables are missing.
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            engine_path: env::var("ENGINE_PATH").unwrap_or_else(|_| "engine/stockfish".to_string()),
            server_addr: env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string()),
            github_token: env::var("GITHUB_TOKEN")?,
            github_owner_repo: env::var("GITHUB_OWNER_REPO")
                .unwrap_or_else(|_| "grim-kalman".to_string()),
            github_branch: env::var("GITHUB_BRANCH").unwrap_or_else(|_| "main".to_string()),
            github_readme_path: env::var("GITHUB_README_PATH")
                .unwrap_or_else(|_| "README.md".to_string()),
            base_url: env::var("BASE_URL")
                .unwrap_or_else(|_| "https://rust-readme-chess.duckdns.org".to_string()),
        })
    }
}
