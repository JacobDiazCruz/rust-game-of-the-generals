use crate::player::Player;

pub struct Board {
    white_player: Player,
    black_player: Player,
}

pub struct BoardBuilder {
    white_player: Player,
    black_player: Player,
}

impl BoardBuilder {
    pub fn new(white_player: Player, black_player: Player) -> Self {
        Self {
            white_player,
            black_player,
        }
    }

    pub fn build(self) -> Board {
        Board {
            white_player: self.white_player,
            black_player: self.black_player,
        }
    }
}
