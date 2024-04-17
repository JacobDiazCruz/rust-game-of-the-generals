#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    player_color: PlayerColors,
    pub pieces: Pieces,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlayerColors {
    White,
    Black,
}

#[derive(Debug, Clone)]
pub struct PlayerBuilder {
    name: String,
    player_color: PlayerColors,
    pieces: Pieces,
}

#[derive(Debug, Clone)]
pub struct Pieces {
    pub flag: (String, u32),
    pub spy: (String, u32),
    pub private: (String, u32),
}

impl PlayerBuilder {
    pub fn new(name: String, player_color: PlayerColors) -> Self {
        let pieces = Pieces {
            flag: (String::from("FLAG"), 14),
            spy: (String::from("SPY"), 15),
            private: (String::from("PRIVATE"), 14),
        };
        Self {
            name,
            player_color,
            pieces,
        }
    }

    pub fn build(self) -> Player {
        Player {
            name: self.name,
            player_color: self.player_color,
            pieces: self.pieces,
        }
    }
}
