use crate::player::{ Player, PlayerColors };
use std::collections::HashMap;

pub struct Board {
    white_player: Player,
    black_player: Player,
}

pub struct BoardBuilder {
    white_player: Player,
    black_player: Player,
}

pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Pieces {
    pieces_map: HashMap<String, (String, u32)>,
}

impl Pieces {
    pub fn new() -> Self {
        let mut pieces_map = HashMap::new();
        pieces_map.insert(String::from("FLAG"), (String::from("FLAG"), 14));
        pieces_map.insert(String::from("SPY"), (String::from("SPY"), 15));
        pieces_map.insert(String::from("PRIVATE"), (String::from("PRIVATE"), 14));
        Self { pieces_map }
    }

    pub fn update_cell_number(&mut self, piece_name: &str, new_cell_number: u32) {
        if let Some(piece) = self.pieces_map.get_mut(piece_name) {
            piece.1 = new_cell_number;
        }
    }
}

impl BoardBuilder {
    pub fn new(white_player: Player, black_player: Player) -> Self {
        Self {
            white_player,
            black_player,
        }
    }

    pub fn move_piece(
        &mut self,
        player_color: PlayerColors,
        direction: Directions,
        piece_name: &str
    ) {
        let player = if player_color == PlayerColors::White {
            &mut self.white_player
        } else {
            &mut self.black_player
        };

        match direction {
            Directions::Up => {
                if let Some((_, cell_number)) = player.pieces.pieces_map.get_mut(piece_name) {
                    if *cell_number == 16 {
                        *cell_number += 1;
                    }
                }
            }
            Directions::Down => {}
            Directions::Left => {}
            Directions::Right => {}
        }
    }

    pub fn build(self) -> Board {
        Board {
            white_player: self.white_player,
            black_player: self.black_player,
        }
    }
}
