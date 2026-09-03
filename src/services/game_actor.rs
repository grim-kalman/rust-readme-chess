use crate::services::chess_service::{Board, ChessError, ChessService};
use crate::services::github_service::GithubService;
use crate::utils::printer::MarkdownPrinter;
use std::fmt;
use std::time::Duration;
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::{mpsc, oneshot};
use tokio::time::timeout;

/// Requests that can wait in line for the engine before the caller is told to come back later.
const QUEUE_CAPACITY: usize = 8;
/// One engine action (validate + player move + depth-16 reply) takes ~1-3 s; well past that
/// Stockfish is stuck, and a stuck engine is treated the same as a dead one.
const ENGINE_TIMEOUT: Duration = Duration::from_secs(30);
/// Queue wait + engine + README publish; a request past this returns 504 and is skipped,
/// never applied late.
const REPLY_TIMEOUT: Duration = Duration::from_secs(90);

pub enum Command {
    Play(String),
    Select(String),
    NewGame,
    Ping,
}

#[derive(Debug)]
pub enum GameError {
    InvalidMove(String),
    Busy,
    Timeout,
    Unavailable(String),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::InvalidMove(mv) => write!(f, "Invalid move: {}", mv),
            GameError::Busy => write!(f, "Game is busy, try again in a moment"),
            GameError::Timeout => write!(f, "Timed out waiting for the engine"),
            GameError::Unavailable(why) => write!(f, "Engine unavailable: {}", why),
        }
    }
}

type Reply = Result<(), GameError>;
type Request = (Command, oneshot::Sender<Reply>);

/// The only way into the game: every request queues here and is answered by the single
/// owner task, so there is no lock to hold and no engine conversation a client can abort.
/// The task also publishes each new board to the README before answering, so commits land
/// in move order.
#[derive(Clone)]
pub struct GameHandle {
    tx: mpsc::Sender<Request>,
}

impl GameHandle {
    pub async fn send(&self, cmd: Command) -> Reply {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.tx.try_send((cmd, reply_tx)).map_err(|e| match e {
            TrySendError::Full(_) => GameError::Busy,
            TrySendError::Closed(_) => GameError::Unavailable("game task stopped".into()),
        })?;
        match timeout(REPLY_TIMEOUT, reply_rx).await {
            Ok(Ok(reply)) => reply,
            Ok(Err(_)) => Err(GameError::Unavailable("game task stopped".into())),
            Err(_) => Err(GameError::Timeout),
        }
    }
}

/// Spawn the owner task. If the engine breaks or hangs, the task ends and the process exits
/// non-zero so the platform restarts it — the engine's state is unknown past that point, and
/// a silently wedged game is exactly the failure this actor exists to prevent.
pub fn spawn(
    mut service: ChessService,
    github: GithubService,
    printer: MarkdownPrinter,
) -> GameHandle {
    let (tx, mut rx) = mpsc::channel::<Request>(QUEUE_CAPACITY);
    tokio::spawn(async move {
        while let Some((cmd, reply)) = rx.recv().await {
            if reply.is_closed() {
                continue;
            }
            let outcome = match timeout(ENGINE_TIMEOUT, handle(&mut service, cmd)).await {
                Ok(outcome) => outcome,
                Err(_) => Err(ChessError::Engine("engine timed out".into())),
            };
            let fatal = matches!(outcome, Err(ChessError::Engine(_)));
            let answer = match outcome {
                Ok(Some(board)) => {
                    publish(&github, &printer, board).await;
                    Ok(())
                }
                Ok(None) => Ok(()),
                Err(e) => Err(GameError::from(e)),
            };
            let _ = reply.send(answer);
            if fatal {
                break;
            }
        }
        log::error!("game task stopped; exiting so the platform restarts the process");
        std::process::exit(1);
    });
    GameHandle { tx }
}

/// Runs the command; a game action yields the board to publish, a ping yields nothing.
async fn handle(service: &mut ChessService, cmd: Command) -> Result<Option<Board>, ChessError> {
    match cmd {
        Command::Play(mv) => service.play(&mv).await?,
        Command::Select(square) => service.select(&square),
        Command::NewGame => service.new_game().await?,
        Command::Ping => return service.board().await.map(|_| None),
    }
    service.board().await.map(Some)
}

async fn publish(github: &GithubService, printer: &MarkdownPrinter, board: Board) {
    let board_md = printer.print(board.fen, board.valid_moves, &board.selected);
    if let Err(e) = github.update_readme(&board_md).await {
        log::error!("README update failed: {:#}", e);
    }
}

impl From<ChessError> for GameError {
    fn from(e: ChessError) -> Self {
        match e {
            ChessError::InvalidMove(mv) => GameError::InvalidMove(mv),
            ChessError::Engine(e) => GameError::Unavailable(e.to_string()),
        }
    }
}
