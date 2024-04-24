use uuid::Uuid;

pub struct Piece {
    pub id: Uuid,
    pub name: String,
    pub player_id: Uuid,
    pub game_id: Uuid,
    pub square: String,
}

pub struct PieceBuilder {
    pub id: Uuid,
    pub name: String,
    pub player_id: Uuid,
    pub game_id: Uuid,
    pub square: String,
}

impl PieceBuilder {
    pub fn new(name: String, player_id: Uuid, game_id: Uuid, square: String) -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            name,
            player_id,
            game_id,
            square
        }
    }

    pub fn create_board_pieces(player_id: Uuid, game_id: Uuid) -> Vec<Piece> {
        let flag = PieceBuilder::new("Flag".to_string(), player_id, game_id, "11".to_string());
        let spy: PieceBuilder = PieceBuilder::new("Spy".to_string(), player_id, game_id, "12".to_string());
        let private = PieceBuilder::new("Private".to_string(), player_id, game_id, "13".to_string());

        let vec = vec![
            flag.build(),
            spy.build(),
            private.build()
        ];
        vec
    }

    pub fn build(self) -> Piece {
        Piece {
            id: self.id,
            name: self.name,
            player_id: self.player_id,
            game_id: self.game_id,
            square: self.square
        }
    }
}