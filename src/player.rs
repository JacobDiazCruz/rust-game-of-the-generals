#[derive(Debug, Clone)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub player_color: PlayerColors,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlayerColors {
    White,
    Black,
}

#[derive(Debug, Clone)]
pub struct PlayerBuilder {
    id: String,
    name: String,
    player_color: PlayerColors,
}

impl PlayerBuilder {
    pub fn new(name: String, player_color: PlayerColors) -> Self {
        Self {
            id: String::from("default"),
            name,
            player_color,
        }
    }

    pub fn build(self) -> Player {
        Player {
            id: self.id,
            name: self.name,
            player_color: self.player_color,
        }
    }
}
