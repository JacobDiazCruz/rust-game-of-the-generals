use crate::player::Player;

pub struct Game {
    white: GamePlayer,
    black: GamePlayer,
    turn: GamePlayer,
    winner: Option<GamePlayer>,
    status: Status,
}
pub struct GameBuilder {
    white: GamePlayer,
    black: GamePlayer,
    turn: GamePlayer,
    winner: Option<GamePlayer>,
    status: Status,
}

pub struct GamePlayer {
    user_id: String,
    pieces: Vec<Pieces>,
    removed_pieces: Vec<Pieces>,
}

pub struct Pieces {
    name: PiecesNames,
    size: String,
    code: String,
    cell: String,
}

enum Status {
    Ongoing,
    Done,
}

enum PiecesNames {
    Private,
    Spy,
    Major,
    Flag,
}

impl GameBuilder {
    fn new(player_one: Player, player_two: Player) -> Self {
        let white = GamePlayer {
            user_id: player_one.id,
            pieces: Vec::new(),
            removed_pieces: Vec::new(),
        };
        let black = GamePlayer {
            user_id: player_two.id,
            pieces: Vec::new(),
            removed_pieces: Vec::new(),
        };
        Self {
            white,
            black,
            turn: white,
            winner: None,
            status: Status::Ongoing,
        }
    }

    fn build(self) -> Game {
        Game {
            white: self.white,
            black: self.black,
            turn: self.turn,
            winner: self.winner,
            status: self.status,
        }
    }
}
