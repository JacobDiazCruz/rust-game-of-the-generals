#[derive(Debug, Clone)]
pub struct Player {
    pub id: Option<String>,
    pub player_name: String,
    pub player_color: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlayerColors {
    White,
    Black,
}

#[derive(Debug, Clone)]
pub struct PlayerBuilder {
    id: String,
    player_name: String,
    player_color: String,
}

impl PlayerBuilder {
    pub fn new(player_name: String, player_color: String) -> Self {
        Self {
            id: String::from("default"),
            player_name,
            player_color,
        }
    }

    pub fn build(self) -> Player {
        Player {
            id: Some(self.id),
            player_name: self.player_name,
            player_color: self.player_color,
        }
    }
}
