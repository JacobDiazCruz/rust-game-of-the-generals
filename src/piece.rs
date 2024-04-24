use uuid::Uuid;

pub struct Piece {
    pub id: Uuid,
    pub rank: i32,
    pub name: String,
    pub player_id: Uuid,
    pub game_id: Uuid,
    pub square: String,
    pub eliminations: Vec<String>,
}

pub struct PieceBuilder {
    id: Uuid,
    rank: i32,
    name: String,
    player_id: Uuid,
    game_id: Uuid,
    square: String,
    eliminations: Vec<String>,
}

impl PieceBuilder {
    pub fn new(
        rank: i32,
        name: String,
        player_id: Uuid,
        game_id: Uuid,
        square: String,
        eliminations: Vec<String>
    ) -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            rank,
            name,
            eliminations,
            player_id,
            game_id,
            square,
        }
    }

    // logic for checking eliminations
    // 1. loop the eliminations field
    // 2. if enemy piece is has rank, compare ranks and remove the lower ranking piece.
    // 3. if enemy piece is == to "private", remove enemy piece.
    // 4. if enemy piece is == to "flag", remove enemy piece.
    // 5. if enemy piece is == to "spy", remove enemy piece.
    // 6. if enemy piece has rank, and elimiation === "all_officers", remove enemy piece.
    pub fn create_board_pieces(player_id: Uuid, game_id: Uuid) -> Vec<Piece> {
        let officer_eliminations = vec![
            "lower_rank_officers".to_string(),
            "private".to_string(),
            "flag".to_string()
        ];
        let private_eliminations = vec!["spy".to_string(), "flag".to_string()];
        let spy_eliminations = vec!["all_officers".to_string(), "flag".to_string()];
        let flag_eliminations = vec!["flag".to_string()];

        let flag = PieceBuilder::new(
            0,
            "Flag".to_string(),
            player_id,
            game_id,
            "11".to_string(),
            flag_eliminations
        );
        let spy: PieceBuilder = PieceBuilder::new(
            0,
            "Spy".to_string(),
            player_id,
            game_id,
            "12".to_string(),
            spy_eliminations
        );
        let private = PieceBuilder::new(
            0,
            "Private".to_string(),
            player_id,
            game_id,
            "13".to_string(),
            private_eliminations
        );

        let five_star_general = PieceBuilder::new(
            1,
            "Five Star General".to_string(),
            player_id,
            game_id,
            "13".to_string(),
            private_eliminations
        );
        let four_star_general = PieceBuilder::new(
            2,
            "Four Star General".to_string(),
            player_id,
            game_id,
            "13".to_string(),
            private_eliminations
        );
        let three_star_general = PieceBuilder::new(
            3,
            "Three Star General".to_string(),
            player_id,
            game_id,
            "13".to_string(),
            private_eliminations
        );
        let two_star_general = PieceBuilder::new(
            4,
            "Two Star General".to_string(),
            player_id,
            game_id,
            "13".to_string(),
            private_eliminations
        );
        let one_star_general = PieceBuilder::new(
            5,
            "One Star General".to_string(),
            player_id,
            game_id,
            "13".to_string(),
            private_eliminations
        );
        let colonel = PieceBuilder::new(
            6,
            "Colonel".to_string(),
            player_id,
            game_id,
            "13".to_string(),
            private_eliminations
        );
        let lt_colonel = PieceBuilder::new(
            7,
            "Lieutenant Colonel".to_string(),
            player_id,
            game_id,
            "13".to_string(),
            private_eliminations
        );
        let major = PieceBuilder::new(
            8,
            "Major".to_string(),
            player_id,
            game_id,
            "13".to_string(),
            private_eliminations
        );
        let captain = PieceBuilder::new(
            9,
            "Captain".to_string(),
            player_id,
            game_id,
            "13".to_string(),
            private_eliminations
        );
        let first_lt = PieceBuilder::new(
            10,
            "First Lieutenant".to_string(),
            player_id,
            game_id,
            "13".to_string(),
            private_eliminations
        );
        let second_lt = PieceBuilder::new(
            11,
            "Second Lieutenant".to_string(),
            player_id,
            game_id,
            "13".to_string(),
            private_eliminations
        );
        let sergeant = PieceBuilder::new(
            12,
            "Sergeant".to_string(),
            player_id,
            game_id,
            "13".to_string(),
            private_eliminations
        );

        let vec = vec![flag.build(), spy.build(), private.build()];
        vec
    }

    pub fn build(self) -> Piece {
        Piece {
            id: self.id,
            rank: self.rank,
            name: self.name,
            player_id: self.player_id,
            eliminations: self.eliminations,
            game_id: self.game_id,
            square: self.square,
        }
    }
}
