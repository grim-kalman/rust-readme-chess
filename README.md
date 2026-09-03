# Rust Readme Chess

A Rust port of [readme-chess](https://github.com/grim-kalman/readme-chess), the interactive GitHub chessboard. This app lets users play chess directly from a GitHub profile README. The board is rendered in Markdown, and moves are made by clicking links, which update the board using the GitHub API.

## Features
- Play chess from a GitHub profile README.
- Board state and move links rendered in Markdown.
- Uses Stockfish as the chess engine backend.
- Rust backend with Actix-web for async HTTP endpoints.
- GitHub API integration for updating the README.
- The move list is recorded in the README itself, so a restart resumes the game in progress.

## How It Works
1. The backend serves endpoints for making moves and selecting pieces.
2. Every link on the README board is a plain GET against one of them, so clicking a link calls the backend directly.
3. The backend validates the move, computes the engine’s reply, and commits the new board to the README (with the move list in an HTML comment).
4. The user is redirected back to the updated GitHub profile.
5. On start-up the backend reads the move list back from the README and replays it, so a redeploy or restart does not lose the game.

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
- `GITHUB_OWNER_REPO` - GitHub username whose profile README holds the board; the profile repo `<user>/<user>` is updated (default: `grim-kalman`)
- `GITHUB_BRANCH` - Branch to update (default: `main`)
- `GITHUB_README_PATH` - Path to README file (default: `README.md`)
- `BASE_URL` - Public URL for endpoint links (default: `https://rust-readme-chess.fly.dev`)

## Testing
Run all tests with:
```sh
cargo test
```
The tests drive the real Stockfish binary in `engine/`. One test commits to the live profile README and is ignored by default; run it deliberately with `cargo test --test github_service_tests -- --ignored`.

## Deploying
Hosted on [Fly.io](https://fly.io) (`fly.toml`). A push to `main` deploys through the GitHub Actions workflow; `GITHUB_TOKEN` is a Fly secret. Fly's `/health` check proves the engine answers, and the app exits non-zero if the engine dies or hangs so Fly restarts it.

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
