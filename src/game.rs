use crate::player::Player;
use std::sync::Arc;

pub struct Game {
    white: Arc<GamePlayer>,
    black: Arc<GamePlayer>,
    turn: Arc<GamePlayer>,
    winner: Option<Arc<GamePlayer>>,
    status: Status,
}

pub struct GameBuilder {
    white: Arc<GamePlayer>,
    black: Arc<GamePlayer>,
    turn: Arc<GamePlayer>,
    winner: Option<Arc<GamePlayer>>,
    status: Status,
}

pub struct GamePlayer {
    user_id: String,
    pieces: Vec<Pieces>,
    removed_pieces: Vec<Pieces>,
}
enum Status {
    Ongoing,
    Done,
}

#[derive(Debug, Clone)]
pub enum Pieces {
    Flag(String, u32),
    Spy(String, u32),
    Private(String, u32),
}

impl GameBuilder {
    pub fn new(player_one: Player, player_two: Player) -> Self {
        let flag = Pieces::Flag(String::from("FLAG"), 14);
        let spy = Pieces::Spy(String::from("SPY"), 15);
        let private = Pieces::Private(String::from("PRIVATE"), 14);
        let pieces = vec![flag.clone(), spy.clone(), private.clone()];

        let white = Arc::new(GamePlayer {
            user_id: player_one.id.unwrap(),
            pieces: pieces.clone(),
            removed_pieces: Vec::new(),
        });
        let black = Arc::new(GamePlayer {
            user_id: player_two.id.unwrap(),
            pieces: pieces.clone(),
            removed_pieces: Vec::new(),
        });
        Self {
            white: white.clone(),
            black: black.clone(),
            turn: white.clone(),
            winner: None,
            status: Status::Ongoing,
        }
    }

    pub fn build(self) -> Game {
        Game {
            white: self.white,
            black: self.black,
            turn: self.turn,
            winner: self.winner,
            status: self.status,
        }
    }
}
