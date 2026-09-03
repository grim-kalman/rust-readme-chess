use std::error::Error;
use std::path::Path;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};

// Simple filter: UCI moves are 4 or 5 chars of file/rank notation, e.g. e2e4 or e7e8q
fn is_uci_move(s: &str) -> bool {
    let bytes = s.as_bytes();
    matches!(
        bytes,
        [
            _from_file @ b'a'..=b'h',
            _from_rank @ b'1'..=b'8',
            _to_file @ b'a'..=b'h',
            _to_rank @ b'1'..=b'8',
            ..,
        ]
    )
}

/// The engine's view of the current position.
#[derive(Debug, Clone)]
pub struct Position {
    pub fen: String,
    pub in_check: bool,
}

/// Manages a Stockfish engine subprocess via UCI.
pub struct EngineService {
    child: Child,
    writer: ChildStdin,
    reader: BufReader<ChildStdout>,
    moves: Vec<String>,
}

impl EngineService {
    /// Launch Stockfish and initialize with UCI handshake and starting position.
    pub async fn start<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error + Send + Sync>> {
        // Spawn the engine
        let mut child = Command::new(path.as_ref())
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        let writer = child.stdin.take().ok_or("engine stdin unavailable")?;
        let stdout = child.stdout.take().ok_or("engine stdout unavailable")?;
        let mut svc = EngineService {
            child,
            writer,
            reader: BufReader::new(stdout),
            moves: Vec::new(),
        };

        // Handshake
        svc.send("uci\n").await?;
        svc.wait_for("uciok").await?;
        svc.send("isready\n").await?;
        svc.wait_for("readyok").await?;
        svc.send("position startpos\n").await?;
        Ok(svc)
    }

    /// Quit the engine cleanly.
    pub async fn stop(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.send("quit\n").await?;
        let _ = self.child.wait().await?;
        Ok(())
    }

    /// Reset to the starting position in the running process; `ucinewgame` clears the
    /// engine's search tables so the new game is not steered by the old one.
    pub async fn new_game(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.moves.clear();
        self.send("ucinewgame\n").await?;
        self.ping().await?;
        self.send("position startpos\n").await
    }

    /// Find best move at fixed depth (16); `None` when the side to move has no legal move
    /// (Stockfish answers `bestmove (none)` on checkmate and stalemate).
    pub async fn best_move(&mut self) -> Result<Option<String>, Box<dyn Error + Send + Sync>> {
        self.send("go depth 16\n").await?;
        let mut line = String::new();
        loop {
            self.read_line(&mut line).await?;
            if let Some(rest) = line.strip_prefix("bestmove ") {
                let mv = rest.split_whitespace().next().unwrap_or_default();
                return Ok(Some(mv.to_string()).filter(|m| m != "(none)"));
            }
            line.clear();
        }
    }

    /// Every move played since the start position, in order.
    pub fn moves(&self) -> &[String] {
        &self.moves
    }

    /// Apply a UCI move (e.g., "e2e4").
    pub async fn make_move(&mut self, mv: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.moves.push(mv.to_string());
        let cmd = format!("position startpos moves {}\n", self.moves.join(" "));
        self.send(&cmd).await
    }

    /// The UCI liveness probe: the engine answers `readyok` once it has processed everything
    /// sent before it.
    pub async fn ping(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.send("isready\n").await?;
        self.wait_for("readyok").await
    }

    /// Get the current position by issuing 'd', which prints the FEN and then the squares
    /// of any pieces giving check.
    pub async fn get_position(&mut self) -> Result<Position, Box<dyn Error + Send + Sync>> {
        self.send("d\n").await?;
        let mut fen = String::new();
        let mut line = String::new();
        loop {
            self.read_line(&mut line).await?;
            if let Some(f) = line.strip_prefix("Fen: ") {
                fen = f.trim().to_string();
            }
            if let Some(checkers) = line.strip_prefix("Checkers:") {
                let in_check = !checkers.trim().is_empty();
                return Ok(Position { fen, in_check });
            }
            line.clear();
        }
    }

    /// List legal moves via perft(1).
    pub async fn get_valid_moves(&mut self) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
        self.send("go perft 1\n").await?;
        let mut moves = Vec::new();
        let mut line = String::new();
        loop {
            self.read_line(&mut line).await?;
            if line.starts_with("Nodes searched") {
                break;
            }
            if let Some((mv, _)) = line.split_once(':') {
                let mv = mv.trim();
                if is_uci_move(mv) {
                    moves.push(mv.to_string());
                }
            }
            line.clear();
        }
        Ok(moves)
    }

    //–– Internal helpers ––

    /// Send a command string to Stockfish.
    async fn send(&mut self, cmd: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.writer.write_all(cmd.as_bytes()).await?;
        self.writer.flush().await?;
        Ok(())
    }

    /// Read lines until one equals the expected keyword (trimmed).
    async fn wait_for(&mut self, expected: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut line = String::new();
        loop {
            self.read_line(&mut line).await?;
            if line.trim() == expected {
                break;
            }
            line.clear();
        }
        Ok(())
    }

    /// Read one line from Stockfish; EOF means the engine process is gone, which every
    /// read loop must treat as an error rather than spin on forever.
    async fn read_line(&mut self, line: &mut String) -> Result<(), Box<dyn Error + Send + Sync>> {
        if self.reader.read_line(line).await? == 0 {
            return Err("engine closed its output (process exited)".into());
        }
        Ok(())
    }
}
