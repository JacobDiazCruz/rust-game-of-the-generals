use crate::player::Player;
use uuid::Uuid;

#[derive(Debug)]
pub struct Game {
    pub id: Option<Uuid>,
    pub white_player_id: Uuid,
    pub black_player_id: Uuid,
    pub turn_player_id: Uuid,
    pub winner_player_id: Option<Uuid>,
}

pub struct GameBuilder {
    id: Option<Uuid>,
    white_player_id: Uuid,
    black_player_id: Uuid,
    turn_player_id: Uuid,
    winner_player_id: Option<Uuid>,
}

impl GameBuilder {
    pub fn new(id: Uuid, player_one: Player, player_two: Player) -> Self {
        let player_one_id = player_one.id.clone().unwrap();

        Self {
            id: Some(id),
            white_player_id: player_one.id.unwrap(),
            black_player_id: player_two.id.unwrap(),
            turn_player_id: player_one_id,
            winner_player_id: None,
        }
    }

    pub fn build(self) -> Game {
        Game {
            id: self.id,
            white_player_id: self.white_player_id,
            black_player_id: self.black_player_id,
            turn_player_id: self.turn_player_id,
            winner_player_id: self.winner_player_id,
        }
    }
}
