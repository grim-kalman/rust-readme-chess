# Rust Readme Chess

A GitHub-integrated chess engine that lets users play chess via a repository's README file. The board state is rendered as Markdown, and moves are made by clicking links in the README, which update the board using GitHub Actions and the GitHub API.

## Features
- Play chess directly from a GitHub profile README.
- Board state and move links are rendered in Markdown.
- Uses Stockfish as the chess engine backend.
- Rust backend with Actix-web for HTTPS endpoints.
- GitHub API integration for updating the README.
- Tests for all core logic and rendering.

## How It Works
1. The backend serves endpoints for making moves and selecting pieces.
2. When a user clicks a move link in the README, a GitHub Action or webhook triggers the backend.
3. The backend updates the board state, computes the engine's reply, and pushes the new board to the README.

## Project Structure
- `src/` - Rust backend source code
  - `main.rs` - Application entry point
  - `controllers/` - HTTPS route handlers
  - `services/` - Chess, engine, and GitHub integration logic
  - `utils/` - Markdown rendering utilities
- `engine/` - Stockfish binary (required)
- `tests/` - Integration and rendering tests
- `Cargo.toml` - Rust project manifest

## Running Locally
1. Install Rust and Cargo.
2. Download Stockfish and place the binary in the `engine/` directory.
3. Set required environment variables (see below).
4. Run the server:
   ```sh
   cargo run
   ```

## Required Environment Variables
- `ENGINE_PATH` - Path to Stockfish binary (default: `engine/stockfish`)
- `SERVER_ADDR` - Address to bind the server (default: `0.0.0.0:8080`)
- `GITHUB_TOKEN` - GitHub personal access token (required)
- `GITHUB_OWNER_REPO` - GitHub repo in `owner/repo` format (default: `grim-kalman`)
- `GITHUB_BRANCH` - Branch to update (default: `main`)
- `GITHUB_README_PATH` - Path to README file (default: `README.md`)
- `BASE_URL` - Public URL for endpoint links (default: `https://rust-readme-chess.duckdns.org`)

## Testing
Run all tests with:
```sh
cargo test
```

## License
MIT
