/// Holds configuration values (GitHub URLs, tokens, engine path).
use std::env;

/// Application configuration: static endpoints and environment secrets.
pub struct AppConfig {
    token: String,
    engine_path: String,
}

impl AppConfig {
    /// Constructs a new AppConfig (e.g., from environment or file).
    /// Constructs AppConfig, pulling the GitHub token and engine path from environment.
    pub fn new() -> Self {
        let token = env::var("GITHUB_TOKEN")
            .expect("`GITHUB_TOKEN` environment variable must be set");
        let engine_path = env::var("CHESS_ENGINE_PATH")
            .unwrap_or_else(|_| String::from("/usr/bin/stockfish"));
        AppConfig { token, engine_path }
    }

    /// Returns the GitHub URL for redirects.
    /// The public URL for the chess web app.
    pub fn github_url(&self) -> &str {
        "https://github.com/grim-kalman"
    }

    /// Returns the GitHub API base URL.
    /// The base GitHub API URL.
    pub fn github_api_url(&self) -> &str {
        "https://api.github.com/repos/"
    }

    /// Returns the path to the README file.
    /// The path to the README file in the repo.
    pub fn readme_path(&self) -> &str {
        "README.md"
    }

    /// Returns the branch name.
    /// The branch to update in the repo.
    pub fn branch(&self) -> &str {
        "main"
    }

    /// Returns the owner/repo identifier.
    /// The owner/repo string for GitHub API calls.
    pub fn owner_repo(&self) -> &str {
        "grim-kalman"
    }

    /// Returns the GitHub token.
    /// The GitHub API token (from env).
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Returns the path to the chess engine binary.
    /// The path to the UCI engine binary (from env or default).
    pub fn engine_path(&self) -> &str {
        &self.engine_path
    }
}
