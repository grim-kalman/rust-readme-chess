use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};

// Simple filter to detect UCI moves: 4 or 5 chars, file/rank notation e.g. e2e4 or e7e8q
fn is_uci_move(s: &str) -> bool {
    matches!(s.as_bytes(), 
        [from_file @ b'a'..=b'h', from_rank @ b'1'..=b'8', 
         to_file @ b'a'..=b'h', to_rank @ b'1'..=b'8', ..])
}

/// Manages a Stockfish engine subprocess via UCI, and tracks the move history.
pub struct EngineService {
    engine_path: String,
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    moves: Vec<String>,
}




impl EngineService {
    /// Launches Stockfish, performs UCI initialization, and sets the start position.
    pub async fn start(engine_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Spawn engine process
        let mut child = Command::new(engine_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        let stdin = child.stdin.take().ok_or("Failed to open engine stdin")?;
        let stdout = child.stdout.take().ok_or("Failed to open engine stdout")?;
        let mut reader = BufReader::new(stdout);
        let mut writer = stdin;
        let mut line = String::new();

        // UCI handshake
        writer.write_all(b"uci\n").await?;
        writer.flush().await?;
        // Wait for 'uciok'
        loop {
            line.clear();
            reader.read_line(&mut line).await?;
            if line.trim() == "uciok" { break; }
        }

        // Ensure engine is ready
        writer.write_all(b"isready\n").await?;
        writer.flush().await?;
        loop {
            line.clear();
            reader.read_line(&mut line).await?;
            if line.trim() == "readyok" { break; }
        }

        // Start new game at the standard initial position
        writer.write_all(b"position startpos\n").await?;
        writer.flush().await?;

        Ok(Self {
            engine_path: engine_path.to_string(),
            child,
            stdin: writer,
            stdout: reader,
            moves: Vec::new(),
        })
    }




    /// Quit the engine process cleanly.
    pub async fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.stdin.write_all(b"quit\n").await?;
        self.stdin.flush().await?;
        // Wait for child to exit
        let _ = self.child.wait().await?;
        Ok(())
    }




    /// Reset to a fresh game by stopping and restarting Stockfish.
    pub async fn new_game(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let path = self.engine_path.clone();
        let _ = self.stop().await;
        let fresh = EngineService::start(&path).await?;
        *self = fresh;
        Ok(())
    }




    /// Instructs Stockfish to search for the best move (fixed depth 16).
    pub async fn best_move(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        self.stdin.write_all(b"go depth 16\n").await?;
        self.stdin.flush().await?;
        let mut line = String::new();
        loop {
            line.clear();
            self.stdout.read_line(&mut line).await?;
            if line.starts_with("bestmove ") {
                let mv = line[9..].split_whitespace().next().unwrap_or("");
                return Ok(mv.to_string());
            }
        }
    }




    /// Applies a player's move in UCI notation (e.g., "e2e4").
    pub async fn make_move(&mut self, mv: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.moves.push(mv.to_string());
        let cmd = format!("position startpos moves {}\n", self.moves.join(" "));
        self.stdin.write_all(cmd.as_bytes()).await?;
        self.stdin.flush().await?;
        Ok(())
    }




    /// Requests the current position's FEN string by issuing the 'd' command.
    pub async fn get_position(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        self.stdin.write_all(b"d\n").await?;
        self.stdin.flush().await?;
        let mut fen = None;
        let mut line = String::new();
        loop {
            line.clear();
            self.stdout.read_line(&mut line).await?;
            if let Some(rest) = line.strip_prefix("Fen: ") {
                fen = Some(rest.trim().to_string());
                break;
            }
        }
        fen.ok_or_else(|| "Failed to parse FEN".into())
    }




    /// Returns the list of legal moves by using perft(1).
    pub async fn get_valid_moves(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        self.stdin.write_all(b"go perft 1\n").await?;
        self.stdin.flush().await?;
        let mut moves = Vec::new();
        let mut line = String::new();
        loop {
            let bytes = self.stdout.read_line(&mut line).await?;
            if line.starts_with("Nodes searched") {
                break;
            }
            if let Some((mv_part, _)) = line.split_once(':') {
                let mv = mv_part.trim();
                if is_uci_move(mv) {
                    moves.push(mv.to_string());
                }
            }
            line.clear();
        }
        Ok(moves)
    }
}