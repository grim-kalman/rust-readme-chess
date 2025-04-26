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

/// Manages a Stockfish engine subprocess via UCI.
pub struct EngineService {
    engine_path: String,
    child: Child,
    writer: ChildStdin,
    reader: BufReader<ChildStdout>,
    moves: Vec<String>,
    /// Cached legal moves for current position
    valid_moves: Vec<String>,
}

impl EngineService {
    /// Launch Stockfish and initialize with UCI handshake and starting position.
    pub async fn start<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let path_str = path.as_ref().to_string_lossy().into_owned();
        // Spawn the engine
        let mut child = Command::new(&path_str)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        let writer = child.stdin.take().ok_or("engine stdin unavailable")?;
        let stdout = child.stdout.take().ok_or("engine stdout unavailable")?;
        let mut svc = EngineService {
            engine_path: path_str,
            child,
            writer,
            reader: BufReader::new(stdout),
            moves: Vec::new(),
            valid_moves: Vec::new(),
        };

        // Handshake
        svc.send("uci\n").await?;
        svc.wait_for("uciok").await?;
        svc.send("isready\n").await?;
        svc.wait_for("readyok").await?;
        svc.send("position startpos\n").await?;

        // Prime legal moves cache
        svc.valid_moves = svc.get_valid_moves().await?;
        Ok(svc)
    }

    /// Quit the engine cleanly.
    pub async fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        self.send("quit\n").await?;
        let _ = self.child.wait().await?;
        Ok(())
    }

    /// Restart a fresh game (stop + start).
    pub async fn new_game(&mut self) -> Result<(), Box<dyn Error>> {
        let path = self.engine_path.clone();
        let _ = self.stop().await;
        *self = EngineService::start(path).await?;
        Ok(())
    }

    /// Find best move at fixed depth (16).
    pub async fn best_move(&mut self) -> Result<String, Box<dyn Error>> {
        self.send("go depth 16\n").await?;
        let mut line = String::new();
        loop {
            self.reader.read_line(&mut line).await?;
            if let Some(rest) = line.strip_prefix("bestmove ") {
                return Ok(rest
                    .split_whitespace()
                    .next()
                    .unwrap_or_default()
                    .to_string());
            }
            line.clear();
        }
    }

    /// Apply a UCI move (e.g., "e2e4") and update legal moves.
    pub async fn make_move(&mut self, mv: &str) -> Result<(), Box<dyn Error>> {
        self.moves.push(mv.to_string());
        let cmd = format!("position startpos moves {}\n", self.moves.join(" "));
        self.send(&cmd).await?;
        self.valid_moves = self.get_valid_moves().await?;
        Ok(())
    }

    /// Get current position FEN by issuing 'd'.
    pub async fn get_position(&mut self) -> Result<String, Box<dyn Error>> {
        self.send("d\n").await?;
        let mut line = String::new();
        loop {
            self.reader.read_line(&mut line).await?;
            if let Some(f) = line.strip_prefix("Fen: ") {
                return Ok(f.trim().to_string());
            }
            line.clear();
        }
    }

    /// List legal moves via perft(1).
    pub async fn get_valid_moves(&mut self) -> Result<Vec<String>, Box<dyn Error>> {
        self.send("go perft 1\n").await?;
        let mut moves = Vec::new();
        let mut line = String::new();
        loop {
            self.reader.read_line(&mut line).await?;
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
    async fn send(&mut self, cmd: &str) -> Result<(), Box<dyn Error>> {
        self.writer.write_all(cmd.as_bytes()).await?;
        self.writer.flush().await?;
        Ok(())
    }

    /// Read lines until one equals the expected keyword (trimmed).
    async fn wait_for(&mut self, expected: &str) -> Result<(), Box<dyn Error>> {
        let mut line = String::new();
        loop {
            self.reader.read_line(&mut line).await?;
            if line.trim() == expected {
                break;
            }
            line.clear();
        }
        Ok(())
    }
}
