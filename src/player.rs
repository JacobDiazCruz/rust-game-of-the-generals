use uuid::Uuid;
#[derive(Debug, Clone)]
pub struct Player {
    pub id: Option<Uuid>,
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
    pub id: Option<Uuid>,
    player_name: String,
    player_color: String,
}

impl PlayerBuilder {
    pub fn new(player_name: String, player_color: String) -> Self {
        Self {
            id: Some(Uuid::new_v4()),
            player_name,
            player_color,
        }
    }

    pub fn build(self) -> Player {
        Player {
            id: Some(self.id.unwrap()),
            player_name: self.player_name,
            player_color: self.player_color,
        }
    }
}
