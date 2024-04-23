use crate::player::Player;

pub struct Game {
    pub white_player_id: String,
    pub black_player_id: String,
    pub turn_player_id: String,
    pub winner_player_id: Option<String>,
    pub status: String,
}

pub struct GameBuilder {
    white_player_id: String,
    black_player_id: String,
    turn_player_id: String,
    winner_player_id: Option<String>,
    status: String,
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
        let player_one_id = player_one.id.clone().unwrap();

        Self {
            white_player_id: player_one.id.unwrap(),
            black_player_id: player_two.id.unwrap(),
            turn_player_id: player_one_id,
            winner_player_id: None,
            status: "Ongoing".to_string(),
        }
    }

    pub fn build(self) -> Game {
        Game {
            white_player_id: self.white_player_id,
            black_player_id: self.black_player_id,
            turn_player_id: self.turn_player_id,
            winner_player_id: self.winner_player_id,
            status: self.status,
        }
    }
}
