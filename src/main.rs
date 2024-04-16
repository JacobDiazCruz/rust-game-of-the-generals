pub mod player;
pub mod board;

use crate::player::{ PlayerBuilder, PlayerColors };
use crate::board::BoardBuilder;
// create a model for a player with their pieces
// instantiate the model by defining: let white_player = Player::new()
// create a model for the board
// create the board with its users
// Board::new(white_player, black_player)
// add the logic in the board model
// create an API handler where it accepts 2 params
// 1. ULDR
// 2. the player who moves
// 3. piece
// features:
// - move the piece to its desired direction
fn main() {
    println!("Hello, world!");
    let white_player = PlayerBuilder::new("CJ".to_string(), PlayerColors::White).build();
    let black_player = PlayerBuilder::new("CJ".to_string(), PlayerColors::Black).build();
    let board = BoardBuilder::new(white_player, black_player);
}
