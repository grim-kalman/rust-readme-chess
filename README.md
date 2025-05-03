# Rust Readme Chess

A Rust port of [readme-chess](https://github.com/grim-kalman/readme-chess), the interactive GitHub chessboard. This app lets users play chess directly from a GitHub profile README. The board is rendered in Markdown, and moves are made by clicking links, which update the board using the GitHub API.

## Features
- Play chess from a GitHub profile README.
- Board state and move links rendered in Markdown.
- Uses Stockfish as the chess engine backend.
- Rust backend with Actix-web for async HTTP endpoints.
- GitHub API integration for updating the README.
- Comprehensive integration and rendering tests.

## How It Works
1. The backend serves endpoints for making moves and selecting pieces.
2. When a user clicks a move link in the README, a webhook or GitHub Action triggers the backend.
3. The backend updates the board state, computes the engine’s reply, and pushes the new board to the README.
4. The user is redirected to the updated GitHub profile.

## Project Structure
- `src/` - Rust backend source code
  - `main.rs` - Application entry point
  - `controllers/` - HTTP route handlers
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

## Comparison: Rust vs Java Version

- **Original Java version:**  
  - Built with Spring Boot and Maven, deployed on Azure.
  - Used Java’s concurrency and web stack.
  - Maintained game state in the application layer.
  - See: [grim-kalman/readme-chess (Java)](https://github.com/grim-kalman/readme-chess)

- **Rust version (this repo):**
  - Uses Actix-web for async HTTP server.
  - Async/await for engine and GitHub API operations.
  - Strong type safety and error handling.
  - Modular, testable design.
  - Improved performance and lower resource usage.
  - **Relies on Stockfish’s internal game state** instead of duplicating state in the backend, reducing complexity and potential for desync.

## Contributing

Contributions, issues, and feature requests are welcome!  
Feel free to open an issue or pull request.

## License

MIT
